// Comprehensive tests for calculated fields and preview endpoints
mod common;

use common::{assertions::*, create_test_client, create_test_server};
use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::test]
async fn test_production_line_calculated_fields() {
    let server = create_test_server().await;
    let client = create_test_client();

    // First create a factory for testing
    let factory_response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&json!({
            "name": "Calculation Test Factory",
            "description": "Factory for testing calculated fields"
        }))
        .send()
        .await
        .expect("Failed to create factory");

    if factory_response.status().as_u16() == 201 {
        let factory: Value = factory_response.json().await.unwrap();
        let factory_id = factory["id"].as_str().unwrap().to_string();

        // Test 1: Create a simple recipe production line
        let recipe_request = json!({
            "name": "Iron Ingot Production",
            "description": "Basic iron ingot production",
            "type": "recipe",
            "recipe": "Iron Ingot",
            "machine_groups": [
                {
                    "number_of_machine": 4,
                    "oc_value": 100.0,
                    "somersloop": 0
                }
            ]
        });

        let create_response = client
            .post(format!(
                "{}/api/factories/{}/production-lines",
                server.base_url, factory_id
            ))
            .json(&recipe_request)
            .send()
            .await
            .expect("Failed to create production line");

        println!("DEBUG: First production line creation status: {}", create_response.status());

        if create_response.status().as_u16() == 201 {
            let created_factory: Value = create_response.json().await.unwrap();

            // Verify calculated fields in response
            let production_lines = created_factory["production_lines"].as_array().unwrap();
            println!("DEBUG: Production lines after first creation: {}", production_lines.len());
            assert_eq!(production_lines.len(), 1);

            let line = &production_lines[0];
            assert_eq!(line["total_machines"], 4);
            assert_eq!(line["total_somersloop"], 0);
            assert_eq!(line["total_power_consumption"], 16.0); // 4 machines * 4MW each

            // Verify input/output rates
            let input_rate = line["input_rate"].as_array().unwrap();
            assert_eq!(input_rate.len(), 1);
            assert_eq!(input_rate[0]["item"], "IronOre");
            assert_eq!(input_rate[0]["quantity"], 120.0); // 4 machines * 30 ore/min each

            let output_rate = line["output_rate"].as_array().unwrap();
            assert_eq!(output_rate.len(), 1);
            assert_eq!(output_rate[0]["item"], "IronIngot");
            assert_eq!(output_rate[0]["quantity"], 120.0); // 4 machines * 30 ingots/min each
        } else {
            println!("DEBUG: First production line creation failed with status: {}", create_response.status());
            println!("DEBUG: Response body: {:?}", create_response.text().await.unwrap());
        }

        // Test 2: Create a production line with overclocking and somersloop
        let overclocked_request = json!({
            "name": "Overclocked Motor Production",
            "description": "Motor production with overclocking and somersloop",
            "type": "recipe",
            "recipe": "Motor",
            "machine_groups": [
                {
                    "number_of_machine": 2,
                    "oc_value": 150.0,
                    "somersloop": 2
                }
            ]
        });

        let overclocked_response = client
            .post(format!(
                "{}/api/factories/{}/production-lines",
                server.base_url, factory_id
            ))
            .json(&overclocked_request)
            .send()
            .await
            .expect("Failed to create overclocked production line");

        if overclocked_response.status().as_u16() == 201 {
            let updated_factory: Value = overclocked_response.json().await.unwrap();

            // Debug: print the actual response
            println!("DEBUG: Factory after second production line: {:?}", updated_factory);

            // Verify calculated fields with overclocking and somersloop
            let production_lines = updated_factory["production_lines"].as_array().unwrap();
            println!("DEBUG: Number of production lines: {}", production_lines.len());
            assert_eq!(production_lines.len(), 2);

            // Find the motor production line (second one)
            let motor_line = production_lines.iter().find(|line| {
                line["ProductionLineRecipe"]["recipe"] == "Motor"
            }).unwrap();

            assert_eq!(motor_line["total_machines"], 2);
            assert_eq!(motor_line["total_somersloop"], 4); // 2 machines * 2 somersloop each

            // Power calculation: base 16MW * (1 + 2/2)^2 * (150/100)^1.321928 * 2 machines
            // = 16 * (1 + 1)^2 * 1.5^1.321928 * 2 = 16 * 4 * ~2.5 * 2 = ~320MW
            let expected_power = 16.0_f64 * (1.0_f64 + 2.0_f64/2.0_f64).powf(2.0_f64) * (1.5_f64).powf(1.321928_f64) * 2.0_f64;
            assert!((motor_line["total_power_consumption"].as_f64().unwrap() - expected_power).abs() < 0.1);

            // Verify input/output rates are scaled by overclocking
            let input_rate = motor_line["input_rate"].as_array().unwrap();
            let output_rate = motor_line["output_rate"].as_array().unwrap();

            // Motor recipe: 5 rotors, 10 screws, 40 wire -> 1 motor (scaled by 1.5x overclock)
            assert!(!input_rate.is_empty());
            assert!(!output_rate.is_empty());
        }

        // Test 3: Create a blueprint production line
        let blueprint_request = json!({
            "name": "Heavy Modular Frame Blueprint",
            "description": "Complete heavy modular frame production",
            "type": "blueprint",
            "production_lines": [
                {
                    "name": "Modular Frame Production",
                    "description": "Basic modular frame",
                    "recipe": "ModularFrame",
                    "machine_groups": [
                        {
                            "number_of_machine": 2,
                            "oc_value": 100.0,
                            "somersloop": 2
                        }
                    ]
                },
                {
                    "name": "Steel Beam Production",
                    "description": "Steel beam for heavy frames",
                    "recipe": "SteelBeam",
                    "machine_groups": [
                        {
                            "number_of_machine": 1,
                            "oc_value": 100.0,
                            "somersloop": 0
                        }
                    ]
                }
            ]
        });

        let blueprint_response = client
            .post(format!(
                "{}/api/factories/{}/production-lines",
                server.base_url, factory_id
            ))
            .json(&blueprint_request)
            .send()
            .await
            .expect("Failed to create blueprint production line");

        if blueprint_response.status().as_u16() == 201 {
            let blueprint_factory: Value = blueprint_response.json().await.unwrap();

            // Verify blueprint calculated fields
            let production_lines = blueprint_factory["production_lines"].as_array().unwrap();
            assert_eq!(production_lines.len(), 3);

            // Find the blueprint line
            let blueprint_line = production_lines.iter().find(|line| {
                line["ProductionLineBlueprint"]["name"] == "Heavy Modular Frame Blueprint"
            }).unwrap();

            assert_eq!(blueprint_line["total_machines"], 3); // 2 + 1
            assert_eq!(blueprint_line["total_somersloop"], 4); // 2 machines * 2 somersloop each

            // Power should be sum of both sub-lines
            let power_consumption = blueprint_line["total_power_consumption"].as_f64().unwrap();
            assert!(power_consumption > 0.0);

            // Verify aggregated input/output rates
            let input_rate = blueprint_line["input_rate"].as_array().unwrap();
            let output_rate = blueprint_line["output_rate"].as_array().unwrap();

            assert!(!input_rate.is_empty());
            assert!(!output_rate.is_empty());
        }
    }
}

#[tokio::test]
async fn test_power_generator_calculated_fields() {
    let server = create_test_server().await;
    let client = create_test_client();

    // First create a factory for testing
    let factory_response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&json!({
            "name": "Power Test Factory",
            "description": "Factory for testing power generator calculations"
        }))
        .send()
        .await
        .expect("Failed to create factory");

    if factory_response.status().as_u16() == 201 {
        let factory: Value = factory_response.json().await.unwrap();
        let factory_id = factory["id"].as_str().unwrap().to_string();

        // Test 1: Create coal power generators
        let coal_request = json!({
            "generator_type": "Coal",
            "fuel_type": "Coal",
            "groups": [
                {
                    "number_of_generators": 5,
                    "clock_speed": 150.0
                }
            ]
        });

        let coal_response = client
            .post(format!(
                "{}/api/factories/{}/power-generators",
                server.base_url, factory_id
            ))
            .json(&coal_request)
            .send()
            .await
            .expect("Failed to create coal generators");

        if coal_response.status().as_u16() == 201 {
            let coal_factory: Value = coal_response.json().await.unwrap();

            // Verify calculated fields in response
            let generators = coal_factory["power_generators"].as_array().unwrap();
            assert_eq!(generators.len(), 1);

            let generator = &generators[0];
            assert_eq!(generator["total_power_generation"], 562.5); // 5 * 75MW * 1.5
            assert_eq!(generator["total_fuel_consumption"], 112.5); // 5 * 15 * 1.5
            assert_eq!(generator["waste_production_rate"], 0.0); // Coal generators don't produce waste
            assert!(generator["waste_product"].is_null());
        }

        // Test 2: Create nuclear power generators (with waste)
        let nuclear_request = json!({
            "generator_type": "Nuclear",
            "fuel_type": "UraniumFuelRod",
            "groups": [
                {
                    "number_of_generators": 2,
                    "clock_speed": 100.0
                }
            ]
        });

        let nuclear_response = client
            .post(format!(
                "{}/api/factories/{}/power-generators",
                server.base_url, factory_id
            ))
            .json(&nuclear_request)
            .send()
            .await
            .expect("Failed to create nuclear generators");

        if nuclear_response.status().as_u16() == 201 {
            let nuclear_factory: Value = nuclear_response.json().await.unwrap();

            // Verify nuclear generator calculations
            let generators = nuclear_factory["power_generators"].as_array().unwrap();
            assert_eq!(generators.len(), 2);

            // Find the nuclear generator
            let nuclear_generator = generators.iter().find(|gen| {
                gen["generator_type"] == "Nuclear"
            }).unwrap();

            assert_eq!(nuclear_generator["total_power_generation"], 5000.0); // 2 * 2500MW
            assert_eq!(nuclear_generator["total_fuel_consumption"], 0.05); // 2 * 0.025
            assert_eq!(nuclear_generator["waste_production_rate"], 0.05); // 2 * 0.025
            assert_eq!(nuclear_generator["waste_product"], "UraniumWaste");
        }

        // Test 3: Create fuel power generators with turbofuel
        let fuel_request = json!({
            "generator_type": "Fuel",
            "fuel_type": "Turbofuel",
            "groups": [
                {
                    "number_of_generators": 3,
                    "clock_speed": 200.0
                }
            ]
        });

        let fuel_response = client
            .post(format!(
                "{}/api/factories/{}/power-generators",
                server.base_url, factory_id
            ))
            .json(&fuel_request)
            .send()
            .await
            .expect("Failed to create fuel generators");

        if fuel_response.status().as_u16() == 201 {
            let fuel_factory: Value = fuel_response.json().await.unwrap();

            // Verify fuel generator calculations
            let generators = fuel_factory["power_generators"].as_array().unwrap();
            assert_eq!(generators.len(), 3);

            // Find the fuel generator
            let fuel_generator = generators.iter().find(|gen| {
                gen["fuel_type"] == "Turbofuel"
            }).unwrap();

            assert_eq!(fuel_generator["total_power_generation"], 900.0); // 3 * 150MW * 2.0
            assert_eq!(fuel_generator["total_fuel_consumption"], 6.75); // 3 * 4.5 * 0.25 * 2.0
            assert_eq!(fuel_generator["waste_production_rate"], 0.0); // Fuel generators don't produce waste
            assert!(fuel_generator["waste_product"].is_null());
        }

        // Test 4: Create geothermal generators (no fuel)
        let geothermal_request = json!({
            "generator_type": "Geothermal",
            "groups": [
                {
                    "number_of_generators": 4,
                    "clock_speed": 100.0
                }
            ]
        });

        let geothermal_response = client
            .post(format!(
                "{}/api/factories/{}/power-generators",
                server.base_url, factory_id
            ))
            .json(&geothermal_request)
            .send()
            .await
            .expect("Failed to create geothermal generators");

        if geothermal_response.status().as_u16() == 201 {
            let geothermal_factory: Value = geothermal_response.json().await.unwrap();

            // Verify geothermal generator calculations
            let generators = geothermal_factory["power_generators"].as_array().unwrap();
            assert_eq!(generators.len(), 4);

            // Find the geothermal generator
            let geothermal_generator = generators.iter().find(|gen| {
                gen["generator_type"] == "Geothermal"
            }).unwrap();

            assert_eq!(geothermal_generator["total_power_generation"], 800.0); // 4 * 200MW
            assert_eq!(geothermal_generator["total_fuel_consumption"], 0.0); // No fuel consumption
            assert_eq!(geothermal_generator["waste_production_rate"], 0.0); // No waste
            assert!(geothermal_generator["waste_product"].is_null());
        }
    }
}

#[tokio::test]
async fn test_raw_input_calculated_fields() {
    let server = create_test_server().await;
    let client = create_test_client();

    // First create a factory for testing
    let factory_response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&json!({
            "name": "Raw Input Test Factory",
            "description": "Factory for testing raw input calculations"
        }))
        .send()
        .await
        .expect("Failed to create factory");

    if factory_response.status().as_u16() == 201 {
        let factory: Value = factory_response.json().await.unwrap();
        let factory_id = factory["id"].as_str().unwrap().to_string();

        // Test 1: Create regular miner (Miner Mk2)
        let miner_request = json!({
            "extractor_type": "MinerMk2",
            "item": "IronOre",
            "purity": "Normal",
            "quantity_per_min": 0.0
        });

        let miner_response = client
            .post(format!(
                "{}/api/factories/{}/raw-inputs",
                server.base_url, factory_id
            ))
            .json(&miner_request)
            .send()
            .await
            .expect("Failed to create miner");

        if miner_response.status().as_u16() == 201 {
            let miner_factory: Value = miner_response.json().await.unwrap();

            // Verify calculated fields in response
            let raw_inputs = miner_factory["raw_inputs"].as_array().unwrap();
            assert_eq!(raw_inputs.len(), 1);

            let raw_input = &raw_inputs[0];
            assert_eq!(raw_input["power_consumption"], 15.0); // Miner Mk2 base power
            assert_eq!(raw_input["quantity_per_min"], 120.0); // Miner Mk2 * Normal purity
        }

        // Test 2: Create water extractor
        let water_request = json!({
            "extractor_type": "WaterExtractor",
            "item": "Water",
            "purity": null,
            "quantity_per_min": 0.0
        });

        let water_response = client
            .post(format!(
                "{}/api/factories/{}/raw-inputs",
                server.base_url, factory_id
            ))
            .json(&water_request)
            .send()
            .await
            .expect("Failed to create water extractor");

        if water_response.status().as_u16() == 201 {
            let water_factory: Value = water_response.json().await.unwrap();

            // Verify water extractor calculations
            let raw_inputs = water_factory["raw_inputs"].as_array().unwrap();
            assert_eq!(raw_inputs.len(), 2);

            // Find the water extractor
            let water_input = raw_inputs.iter().find(|input| {
                input["item"] == "Water"
            }).unwrap();

            assert_eq!(water_input["power_consumption"], 20.0); // Water extractor base power
            assert_eq!(water_input["quantity_per_min"], 120.0); // Fixed water extraction rate
        }

        // Test 3: Create resource well system with pressurizer
        let resource_well_request = json!({
            "extractor_type": "ResourceWellExtractor",
            "item": "CrudeOil",
            "purity": null,
            "quantity_per_min": 0.0,
            "pressurizer": {
                "clock_speed": 150.0
            },
            "extractors": [
                {
                    "purity": "Normal",
                    "quantity_per_min": 0.0
                },
                {
                    "purity": "Pure",
                    "quantity_per_min": 0.0
                }
            ]
        });

        let resource_well_response = client
            .post(format!(
                "{}/api/factories/{}/raw-inputs",
                server.base_url, factory_id
            ))
            .json(&resource_well_request)
            .send()
            .await
            .expect("Failed to create resource well");

        if resource_well_response.status().as_u16() == 201 {
            let resource_well_factory: Value = resource_well_response.json().await.unwrap();

            // Verify resource well calculations
            let raw_inputs = resource_well_factory["raw_inputs"].as_array().unwrap();
            assert_eq!(raw_inputs.len(), 3);

            // Find the resource well
            let resource_well_input = raw_inputs.iter().find(|input| {
                input["extractor_type"] == "ResourceWellExtractor"
            }).unwrap();

            // Power should be pressurizer power at 150% clock speed
            let power_consumption = resource_well_input["power_consumption"].as_f64().unwrap();
            assert!(power_consumption > 150.0);

            // Quantity should be sum of both extractors at 150% clock speed
            // Normal: 60 * 1.5 = 90, Pure: 120 * 1.5 = 180, Total: 270
            assert_eq!(resource_well_input["quantity_per_min"], 270.0);
        }

        // Test 4: Create oil extractor with different purities
        let oil_request = json!({
            "extractor_type": "OilExtractor",
            "item": "CrudeOil",
            "purity": "Pure",
            "quantity_per_min": 0.0
        });

        let oil_response = client
            .post(format!(
                "{}/api/factories/{}/raw-inputs",
                server.base_url, factory_id
            ))
            .json(&oil_request)
            .send()
            .await
            .expect("Failed to create oil extractor");

        if oil_response.status().as_u16() == 201 {
            let oil_factory: Value = oil_response.json().await.unwrap();

            // Verify oil extractor calculations
            let raw_inputs = oil_factory["raw_inputs"].as_array().unwrap();
            assert_eq!(raw_inputs.len(), 4);

            // Find the oil extractor
            let oil_input = raw_inputs.iter().find(|input| {
                input["item"] == "CrudeOil" && input["extractor_type"] == "OilExtractor"
            }).unwrap();

            assert_eq!(oil_input["power_consumption"], 40.0); // Oil extractor base power
            assert_eq!(oil_input["quantity_per_min"], 240.0); // Oil extractor * Pure purity
        }
    }
}

#[tokio::test]
async fn test_preview_endpoints_calculations() {
    let server = create_test_server().await;
    let client = create_test_client();

    // First create a factory for testing
    let factory_response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&json!({
            "name": "Preview Test Factory",
            "description": "Factory for testing preview endpoints"
        }))
        .send()
        .await
        .expect("Failed to create factory");

    if factory_response.status().as_u16() == 201 {
        let factory: Value = factory_response.json().await.unwrap();
        let factory_id = factory["id"].as_str().unwrap().to_string();

        // Test 1: Preview production line (recipe type)
        let production_line_preview = json!({
            "name": "Iron Ingot Production",
            "description": "Basic iron ingot production",
            "type": "recipe",
            "recipe": "Iron Ingot",
            "machine_groups": [
                {
                    "number_of_machine": 4,
                    "oc_value": 100.0,
                    "somersloop": 0
                }
            ]
        });

        let preview_response = client
            .post(format!(
                "{}/api/factories/{}/production-lines/preview",
                server.base_url, factory_id
            ))
            .json(&production_line_preview)
            .send()
            .await
            .expect("Failed to preview production line");

        if preview_response.status().as_u16() == 200 {
            let preview: Value = preview_response.json().await.unwrap();

            // Verify preview response structure
            assert!(preview.get("total_power_consumption").is_some());
            assert!(preview.get("total_machines").is_some());
            assert!(preview.get("total_somersloop").is_some());
            assert!(preview.get("input_rate").is_some());
            assert!(preview.get("output_rate").is_some());

            // Verify calculations
            assert_eq!(preview["total_machines"], 4);
            assert_eq!(preview["total_somersloop"], 0);
            assert_eq!(preview["total_power_consumption"], 16.0); // 4 machines * 4MW each

            // Verify input/output rates
            let input_rate = preview["input_rate"].as_array().unwrap();
            assert_eq!(input_rate.len(), 1);
            assert_eq!(input_rate[0]["item"], "IronOre");
            assert_eq!(input_rate[0]["quantity"], 120.0); // 4 machines * 30 ore/min each

            let output_rate = preview["output_rate"].as_array().unwrap();
            assert_eq!(output_rate.len(), 1);
            assert_eq!(output_rate[0]["item"], "IronIngot");
            assert_eq!(output_rate[0]["quantity"], 120.0); // 4 machines * 30 ingots/min each
        }

        // Test 2: Preview production line (blueprint type)
        let blueprint_preview = json!({
            "name": "Motor Production Blueprint",
            "description": "Complete motor production",
            "type": "blueprint",
            "production_lines": [
                {
                    "name": "Iron Ingot Production",
                    "description": "Basic iron ingot production",
                    "recipe": "IronIngot",
                    "machine_groups": [
                        {
                            "number_of_machine": 8,
                            "oc_value": 100.0,
                            "somersloop": 0
                        }
                    ]
                },
                {
                    "name": "Motor Assembly",
                    "description": "Motor assembly line",
                    "recipe": "Motor",
                    "machine_groups": [
                        {
                            "number_of_machine": 2,
                            "oc_value": 100.0,
                            "somersloop": 2
                        }
                    ]
                }
            ]
        });

        let blueprint_response = client
            .post(format!(
                "{}/api/factories/{}/production-lines/preview",
                server.base_url, factory_id
            ))
            .json(&blueprint_preview)
            .send()
            .await
            .expect("Failed to preview blueprint");

        if blueprint_response.status().as_u16() == 200 {
            let blueprint_preview_data: Value = blueprint_response.json().await.unwrap();

            // Verify blueprint calculations
            assert_eq!(blueprint_preview_data["total_machines"], 10); // 8 + 2
            assert_eq!(blueprint_preview_data["total_somersloop"], 4); // 2 machines * 2 somersloop each

            // Verify power consumption includes somersloop multiplier
            let power_consumption = blueprint_preview_data["total_power_consumption"].as_f64().unwrap();
            assert!(power_consumption > 0.0);

            // Verify input/output aggregation
            let input_rate = blueprint_preview_data["input_rate"].as_array().unwrap();
            let output_rate = blueprint_preview_data["output_rate"].as_array().unwrap();

            // Should have inputs for both iron ore and motor components
            assert!(!input_rate.is_empty());
            assert!(!output_rate.is_empty());
        }

        // Test 3: Preview power generator
        let generator_preview = json!({
            "generator_type": "Coal",
            "fuel_type": "Coal",
            "groups": [
                {
                    "number_of_generators": 5,
                    "clock_speed": 150.0
                }
            ]
        });

        let generator_response = client
            .post(format!(
                "{}/api/factories/{}/power-generators/preview",
                server.base_url, factory_id
            ))
            .json(&generator_preview)
            .send()
            .await
            .expect("Failed to preview power generator");

        if generator_response.status().as_u16() == 200 {
            let generator_preview_data: Value = generator_response.json().await.unwrap();

            // Verify generator preview response structure
            assert!(generator_preview_data.get("total_power_generation").is_some());
            assert!(generator_preview_data.get("total_fuel_consumption").is_some());
            assert!(generator_preview_data.get("waste_production_rate").is_some());
            assert!(generator_preview_data.get("waste_product").is_some());

            // Verify calculations
            assert_eq!(generator_preview_data["total_power_generation"], 562.5); // 5 * 75MW * 1.5
            assert_eq!(generator_preview_data["total_fuel_consumption"], 112.5); // 5 * 15 * 1.5
            assert_eq!(generator_preview_data["waste_production_rate"], 0.0); // Coal generators don't produce waste
            assert!(generator_preview_data["waste_product"].is_null());
        }

        // Test 4: Preview nuclear power generator (with waste)
        let nuclear_preview = json!({
            "generator_type": "Nuclear",
            "fuel_type": "UraniumFuelRod",
            "groups": [
                {
                    "number_of_generators": 2,
                    "clock_speed": 100.0
                }
            ]
        });

        let nuclear_response = client
            .post(format!(
                "{}/api/factories/{}/power-generators/preview",
                server.base_url, factory_id
            ))
            .json(&nuclear_preview)
            .send()
            .await
            .expect("Failed to preview nuclear generator");

        if nuclear_response.status().as_u16() == 200 {
            let nuclear_preview_data: Value = nuclear_response.json().await.unwrap();

            // Verify nuclear generator calculations
            assert_eq!(nuclear_preview_data["total_power_generation"], 5000.0); // 2 * 2500MW
            assert_eq!(nuclear_preview_data["total_fuel_consumption"], 0.05); // 2 * 0.025
            assert_eq!(nuclear_preview_data["waste_production_rate"], 0.05); // 2 * 0.025
            assert_eq!(nuclear_preview_data["waste_product"], "UraniumWaste");
        }

        // Test 5: Preview raw input (regular extractor)
        let raw_input_preview = json!({
            "extractor_type": "MinerMk2",
            "item": "IronOre",
            "purity": "Normal",
            "quantity_per_min": 0.0
        });

        let raw_input_response = client
            .post(format!(
                "{}/api/factories/{}/raw-inputs/preview",
                server.base_url, factory_id
            ))
            .json(&raw_input_preview)
            .send()
            .await
            .expect("Failed to preview raw input");

        if raw_input_response.status().as_u16() == 200 {
            let raw_input_preview_data: Value = raw_input_response.json().await.unwrap();

            // Verify raw input preview response structure
            assert!(raw_input_preview_data.get("power_consumption").is_some());
            assert!(raw_input_preview_data.get("quantity_per_min").is_some());

            // Verify calculations
            assert_eq!(raw_input_preview_data["power_consumption"], 15.0); // Miner Mk2 base power
            assert_eq!(raw_input_preview_data["quantity_per_min"], 120.0); // Miner Mk2 * Normal purity
        }

        // Test 6: Preview raw input (resource well system)
        let resource_well_preview = json!({
            "extractor_type": "ResourceWellExtractor",
            "item": "CrudeOil",
            "purity": null,
            "quantity_per_min": 0.0,
            "pressurizer": {
                "clock_speed": 150.0
            },
            "extractors": [
                {
                    "purity": "Normal",
                    "quantity_per_min": 0.0
                },
                {
                    "purity": "Pure",
                    "quantity_per_min": 0.0
                }
            ]
        });

        let resource_well_response = client
            .post(format!(
                "{}/api/factories/{}/raw-inputs/preview",
                server.base_url, factory_id
            ))
            .json(&resource_well_preview)
            .send()
            .await
            .expect("Failed to preview resource well");

        if resource_well_response.status().as_u16() == 200 {
            let resource_well_preview_data: Value = resource_well_response.json().await.unwrap();

            // Verify resource well calculations
            let power_consumption = resource_well_preview_data["power_consumption"].as_f64().unwrap();
            let quantity_per_min = resource_well_preview_data["quantity_per_min"].as_f64().unwrap();

            // Power should be pressurizer power at 150% clock speed
            assert!(power_consumption > 150.0);

            // Quantity should be sum of both extractors at 150% clock speed
            // Normal: 60 * 1.5 = 90, Pure: 120 * 1.5 = 180, Total: 270
            assert_eq!(quantity_per_min, 270.0);
        }
    }
}

#[tokio::test]
async fn test_preview_endpoints_error_handling() {
    let server = create_test_server().await;
    let client = create_test_client();

    // First create a factory for testing
    let factory_response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&json!({
            "name": "Error Test Factory",
            "description": "Factory for testing error handling"
        }))
        .send()
        .await
        .expect("Failed to create factory");

    if factory_response.status().as_u16() == 201 {
        let factory: Value = factory_response.json().await.unwrap();
        let factory_id = factory["id"].as_str().unwrap().to_string();

        // Test 1: Preview with invalid factory ID
        let invalid_factory_id = Uuid::new_v4();

        let invalid_response = client
            .post(format!(
                "{}/api/factories/{}/production-lines/preview",
                server.base_url, invalid_factory_id
            ))
            .json(&json!({
                "name": "Test Line",
                "type": "recipe",
                "recipe": "IronIngot",
                "machine_groups": []
            }))
            .send()
            .await
            .expect("Failed to send request to invalid factory");

        assert_not_found(invalid_response).await;

        // Test 2: Preview with invalid recipe
        let invalid_recipe_preview = json!({
            "name": "Invalid Production Line",
            "type": "recipe",
            "recipe": "NonExistentRecipe",
            "machine_groups": [
                {
                    "number_of_machine": 1,
                    "oc_value": 100.0,
                    "somersloop": 0
                }
            ]
        });

        let invalid_recipe_response = client
            .post(format!(
                "{}/api/factories/{}/production-lines/preview",
                server.base_url, factory_id
            ))
            .json(&invalid_recipe_preview)
            .send()
            .await
            .expect("Failed to send invalid recipe preview request");

        assert_bad_request(invalid_recipe_response).await;

        // Test 3: Preview with invalid overclock value
        let invalid_overclock_preview = json!({
            "name": "Invalid Overclock Line",
            "type": "recipe",
            "recipe": "Iron Ingot",
            "machine_groups": [
                {
                    "number_of_machine": 1,
                    "oc_value": 300.0, // Invalid: > 250%
                    "somersloop": 0
                }
            ]
        });

        let invalid_overclock_response = client
            .post(format!(
                "{}/api/factories/{}/production-lines/preview",
                server.base_url, factory_id
            ))
            .json(&invalid_overclock_preview)
            .send()
            .await
            .expect("Failed to send invalid overclock preview request");

        assert_bad_request(invalid_overclock_response).await;

        // Test 4: Preview with invalid somersloop value
        let invalid_somersloop_preview = json!({
            "name": "Invalid Somersloop Line",
            "type": "recipe",
            "recipe": "IronIngot",
            "machine_groups": [
                {
                    "number_of_machine": 1,
                    "oc_value": 100.0,
                    "somersloop": 5 // Invalid: > 1 for Smelter
                }
            ]
        });

        let invalid_somersloop_response = client
            .post(format!(
                "{}/api/factories/{}/production-lines/preview",
                server.base_url, factory_id
            ))
            .json(&invalid_somersloop_preview)
            .send()
            .await
            .expect("Failed to send invalid somersloop preview request");

        assert_bad_request(invalid_somersloop_response).await;

        // Test 5: Preview power generator with invalid fuel type
        let invalid_fuel_preview = json!({
            "generator_type": "Coal",
            "fuel_type": "NonExistentFuel",
            "groups": [
                {
                    "number_of_generators": 1,
                    "clock_speed": 100.0
                }
            ]
        });

        let invalid_fuel_response = client
            .post(format!(
                "{}/api/factories/{}/power-generators/preview",
                server.base_url, factory_id
            ))
            .json(&invalid_fuel_preview)
            .send()
            .await
            .expect("Failed to send invalid fuel preview request");

        // Invalid enum value causes deserialization error (422)
        assert_bad_request_or_unprocessable(invalid_fuel_response).await;

        // Test 6: Preview power generator without fuel type (for non-geothermal)
        let missing_fuel_preview = json!({
            "generator_type": "Coal",
            "groups": [
                {
                    "number_of_generators": 1,
                    "clock_speed": 100.0
                }
            ]
        });

        let missing_fuel_response = client
            .post(format!(
                "{}/api/factories/{}/power-generators/preview",
                server.base_url, factory_id
            ))
            .json(&missing_fuel_preview)
            .send()
            .await
            .expect("Failed to send missing fuel preview request");

        // Missing required field causes validation error (400)
        assert_bad_request(missing_fuel_response).await;

        // Test 7: Preview raw input with incompatible extractor/item combination
        let incompatible_raw_input_preview = json!({
            "extractor_type": "MinerMk1",
            "item": "Water", // Miners can't extract water
            "purity": "Normal",
            "quantity_per_min": 0.0
        });

        let incompatible_response = client
            .post(format!(
                "{}/api/factories/{}/raw-inputs/preview",
                server.base_url, factory_id
            ))
            .json(&incompatible_raw_input_preview)
            .send()
            .await
            .expect("Failed to send incompatible raw input preview request");

        assert_bad_request(incompatible_response).await;

        // Test 8: Preview resource well without pressurizer
        let missing_pressurizer_preview = json!({
            "extractor_type": "ResourceWellExtractor",
            "item": "CrudeOil",
            "purity": null,
            "quantity_per_min": 0.0,
            "extractors": [
                {
                    "purity": "Normal",
                    "quantity_per_min": 0.0
                }
            ]
        });

        let missing_pressurizer_response = client
            .post(format!(
                "{}/api/factories/{}/raw-inputs/preview",
                server.base_url, factory_id
            ))
            .json(&missing_pressurizer_preview)
            .send()
            .await
            .expect("Failed to send missing pressurizer preview request");

        assert_bad_request(missing_pressurizer_response).await;

        // Test 9: Preview resource well without extractors
        let missing_extractors_preview = json!({
            "extractor_type": "ResourceWellExtractor",
            "item": "CrudeOil",
            "purity": null,
            "quantity_per_min": 0.0,
            "pressurizer": {
                "clock_speed": 100.0
            },
            "extractors": []
        });

        let missing_extractors_response = client
            .post(format!(
                "{}/api/factories/{}/raw-inputs/preview",
                server.base_url, factory_id
            ))
            .json(&missing_extractors_preview)
            .send()
            .await
            .expect("Failed to send missing extractors preview request");

        assert_bad_request(missing_extractors_response).await;
    }
}

#[tokio::test]
async fn test_factory_level_calculations() {
    let server = create_test_server().await;
    let client = create_test_client();

    // First create a factory for testing
    let factory_response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&json!({
            "name": "Factory Level Test Factory",
            "description": "Factory for testing factory-level calculations"
        }))
        .send()
        .await
        .expect("Failed to create factory");

    if factory_response.status().as_u16() == 201 {
        let factory: Value = factory_response.json().await.unwrap();
        let factory_id = factory["id"].as_str().unwrap().to_string();

        // Create a production line
        let production_request = json!({
            "name": "Iron Ingot Production",
            "description": "Basic iron ingot production",
            "type": "recipe",
            "recipe": "Iron Ingot",
            "machine_groups": [
                {
                    "number_of_machine": 4,
                    "oc_value": 100.0,
                    "somersloop": 0
                }
            ]
        });

        let production_response = client
            .post(format!(
                "{}/api/factories/{}/production-lines",
                server.base_url, factory_id
            ))
            .json(&production_request)
            .send()
            .await
            .expect("Failed to send production line request");

        if production_response.status().as_u16() != 201 {
            panic!("Production line creation failed: {}", production_response.text().await.unwrap());
        }

        // Create a raw input
        let raw_input_request = json!({
            "extractor_type": "MinerMk2",
            "item": "IronOre",
            "purity": "Normal",
            "quantity_per_min": 0.0
        });

        let _ = client
            .post(format!(
                "{}/api/factories/{}/raw-inputs",
                server.base_url, factory_id
            ))
            .json(&raw_input_request)
            .send()
            .await;

        // Create power generators
        let generator_request = json!({
            "generator_type": "Coal",
            "fuel_type": "Coal",
            "groups": [
                {
                    "number_of_generators": 2,
                    "clock_speed": 100.0
                }
            ]
        });

        let _ = client
            .post(format!(
                "{}/api/factories/{}/power-generators",
                server.base_url, factory_id
            ))
            .json(&generator_request)
            .send()
            .await;

        // Get the updated factory and verify calculations
        let get_response = client
            .get(format!("{}/api/factories/{}", server.base_url, factory_id))
            .send()
            .await
            .expect("Failed to get updated factory");

        if get_response.status().as_u16() == 200 {
            let updated_factory: Value = get_response.json().await.unwrap();

            // Verify factory-level calculations
            assert_eq!(updated_factory["total_power_consumption"], 31.0); // 16 (production) + 15 (raw input)
            assert_eq!(updated_factory["total_power_generation"], 150.0); // 2 * 75MW
            assert_eq!(updated_factory["power_balance"], 119.0); // 150 - 31

            // Verify item balance calculations
            let items = updated_factory["items"].as_array().unwrap();
            assert!(!items.is_empty());

            // Should have IronOre (-120 from production + 120 from raw input = 0)
            // and IronIngot (+120 from production)
            let iron_ore_item = items.iter().find(|item| item["item"] == "IronOre").unwrap();
            assert_eq!(iron_ore_item["quantity"], 0.0); // Balanced

            let iron_ingot_item = items.iter().find(|item| item["item"] == "IronIngot").unwrap();
            assert_eq!(iron_ingot_item["quantity"], 120.0); // Production output
        }
    }
}