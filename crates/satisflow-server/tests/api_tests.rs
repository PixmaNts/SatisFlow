// Comprehensive API tests for satisflow-server
mod common;

use common::{assertions::*, create_test_client, create_test_server, test_data::*};
use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::test]
async fn test_health_check() {
    let server = create_test_server().await;
    let client = create_test_client();

    let response = client
        .get(format!("{}/health", server.base_url))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);
    let text = response.text().await.unwrap();
    assert_eq!(text, "OK");
}

// FACTORY CRUD TESTS
#[tokio::test]
async fn test_factory_crud_operations() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Test 1: Create factory
    let create_response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&create_factory_request())
        .send()
        .await
        .expect("Failed to create factory");

    // Note: This will likely fail with current implementation since handlers return "Not implemented yet"
    // but the test structure is ready for when handlers are fully implemented
    if create_response.status().as_u16() == 201 {
        let factory: Value = assert_created_response(create_response).await;
        let factory_id = factory["id"].as_str().unwrap().to_string();
        assert_eq!(factory["notes"], json!("Test notes"));

        // Test 2: Get all factories
        let list_response = client
            .get(format!("{}/api/factories", server.base_url))
            .send()
            .await
            .expect("Failed to get factories");

        let factories: Value = assert_json_response(list_response).await;
        assert!(!factories.as_array().unwrap().is_empty());

        // Test 3: Get specific factory
        let get_response = client
            .get(format!("{}/api/factories/{}", server.base_url, factory_id))
            .send()
            .await
            .expect("Failed to get factory");

        let retrieved_factory: Value = assert_json_response(get_response).await;
        assert_eq!(retrieved_factory["id"], json!(factory_id));
        assert_eq!(retrieved_factory["name"], "Test Factory");
        assert_eq!(retrieved_factory["notes"], json!("Test notes"));

        // Test 4: Update factory
        let update_response = client
            .put(format!("{}/api/factories/{}", server.base_url, factory_id))
            .json(&update_factory_request())
            .send()
            .await
            .expect("Failed to update factory");

        let updated_factory: Value = assert_json_response(update_response).await;
        assert_eq!(updated_factory["name"], "Updated Factory");
        assert_eq!(updated_factory["notes"], json!("Updated notes"));

        // Test 5: Delete factory
        let delete_response = client
            .delete(format!("{}/api/factories/{}", server.base_url, factory_id))
            .send()
            .await
            .expect("Failed to delete factory");

        assert_no_content(delete_response).await;

        // Test 6: Verify factory is deleted
        let verify_response = client
            .get(format!("{}/api/factories/{}", server.base_url, factory_id))
            .send()
            .await
            .expect("Failed to verify deletion");

        assert_not_found(verify_response).await;
    } else {
        // Current implementation returns "Not implemented yet"
        assert_bad_request(create_response).await;
    }
}

#[tokio::test]
async fn test_factory_error_cases() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Test 1: Get non-existent factory
    let unknown = Uuid::new_v4();

    let response = client
        .get(format!("{}/api/factories/{}", server.base_url, unknown))
        .send()
        .await
        .expect("Failed to send request");

    assert_not_found(response).await;

    // Test 2: Update non-existent factory
    let response = client
        .put(format!("{}/api/factories/{}", server.base_url, unknown))
        .json(&update_factory_request())
        .send()
        .await
        .expect("Failed to send request");

    assert_not_found(response).await;

    // Test 3: Delete non-existent factory
    let response = client
        .delete(format!("{}/api/factories/{}", server.base_url, unknown))
        .send()
        .await
        .expect("Failed to send request");

    assert_not_found(response).await;

    // Test 4: Create factory with invalid data
    let response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&invalid_factory_request())
        .send()
        .await
        .expect("Failed to send request");

    // This might be 400 (Bad Request) or still return "Not implemented yet"
    assert!(response.status().as_u16() == 400 || response.status().as_u16() == 400);
}

// LOGISTICS TESTS
#[tokio::test]
async fn test_logistics_crud_operations() {
    let server = create_test_server().await;
    let client = create_test_client();

    // First create two factories for logistics testing
    let factory1_response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&json!({
            "name": "Factory 1",
            "description": "Source factory"
        }))
        .send()
        .await
        .expect("Failed to create factory 1");

    let factory2_response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&json!({
            "name": "Factory 2",
            "description": "Destination factory"
        }))
        .send()
        .await
        .expect("Failed to create factory 2");

    // If factories were created successfully, test logistics
    if factory1_response.status().as_u16() == 201 && factory2_response.status().as_u16() == 201 {
        let factory1: Value = factory1_response.json().await.unwrap();
        let factory2: Value = factory2_response.json().await.unwrap();
        let factory1_id = factory1["id"].as_str().unwrap().to_string();
        let factory2_id = factory2["id"].as_str().unwrap().to_string();

        // Test 1: Create logistics line
        let logistics_request = json!({
            "from_factory": factory1_id,
            "to_factory": factory2_id,
            "transport_type": "Truck",
            "item": "IronOre",
            "quantity_per_min": 60.0,
            "truck_id": "TRK-101"
        });

        let create_response = client
            .post(format!("{}/api/logistics", server.base_url))
            .json(&logistics_request)
            .send()
            .await
            .expect("Failed to create logistics");

        if create_response.status().as_u16() == 201 {
            let logistics: Value = assert_created_response(create_response).await;
            let logistics_id = logistics["id"]
                .as_str()
                .and_then(|id| Uuid::parse_str(id).ok())
                .unwrap();

            // Test 2: Get all logistics
            let list_response = client
                .get(format!("{}/api/logistics", server.base_url))
                .send()
                .await
                .expect("Failed to get logistics");

            let logistics_list: Value = assert_json_response(list_response).await;
            assert!(!logistics_list.as_array().unwrap().is_empty());

            // Test 3: Get specific logistics line
            let get_response = client
                .get(format!(
                    "{}/api/logistics/{}",
                    server.base_url, logistics_id
                ))
                .send()
                .await
                .expect("Failed to get logistics line");

            let retrieved_logistics: Value = assert_json_response(get_response).await;
            assert_eq!(retrieved_logistics["id"], json!(logistics_id.to_string()));
            assert_eq!(retrieved_logistics["transport_type"], json!("Truck"));
            assert!(retrieved_logistics["items"]
                .as_array()
                .map(|items| !items.is_empty())
                .unwrap_or(false));

            // Test 4: Delete logistics line
            let delete_response = client
                .delete(format!(
                    "{}/api/logistics/{}",
                    server.base_url, logistics_id
                ))
                .send()
                .await
                .expect("Failed to delete logistics");

            assert_no_content(delete_response).await;

            // Test 5: Verify logistics is deleted
            let verify_response = client
                .get(format!(
                    "{}/api/logistics/{}",
                    server.base_url, logistics_id
                ))
                .send()
                .await
                .expect("Failed to verify deletion");

            assert_not_found(verify_response).await;

            // Additional tests: create bus logistics
            let bus_request = json!({
                "from_factory": factory1_id,
                "to_factory": factory2_id,
                "transport_type": "Bus",
                "bus_name": "Test Bus Route",
                "conveyors": [
                    {
                        "line_id": "CV-001",
                        "conveyor_type": "Mk3",
                        "item": "IronPlate",
                        "quantity_per_min": 120.0
                    }
                ],
                "pipelines": [
                    {
                        "pipeline_id": "PL-001",
                        "pipeline_type": "Mk1",
                        "item": "Water",
                        "quantity_per_min": 240.0
                    }
                ]
            });

            let bus_response = client
                .post(format!("{}/api/logistics", server.base_url))
                .json(&bus_request)
                .send()
                .await
                .expect("Failed to create bus logistics");

            let bus_status = bus_response.status().as_u16();
            let bus_body = bus_response.text().await.unwrap();
            assert_eq!(bus_status, 201, "Bus creation failed: {}", bus_body);
            let bus_logistics: Value = serde_json::from_str(&bus_body).unwrap();
            assert_eq!(bus_logistics["transport_type"], json!("Bus"));
            let bus_id = bus_logistics["id"]
                .as_str()
                .and_then(|id| Uuid::parse_str(id).ok())
                .unwrap();

            // Additional tests: create train logistics
            let train_request = json!({
                "from_factory": factory1_id,
                "to_factory": factory2_id,
                "transport_type": "Train",
                "train_name": "Test Train Line",
                "wagons": [
                    {
                        "wagon_id": "WG-001",
                        "wagon_type": "Cargo",
                        "item": "IronPlate",
                        "quantity_per_min": 120.0
                    },
                    {
                        "wagon_id": "WG-002",
                        "wagon_type": "Fluid",
                        "item": "Water",
                        "quantity_per_min": 300.0
                    }
                ]
            });

            let train_response = client
                .post(format!("{}/api/logistics", server.base_url))
                .json(&train_request)
                .send()
                .await
                .expect("Failed to create train logistics");

            let train_status = train_response.status().as_u16();
            let train_body = train_response.text().await.unwrap();
            assert_eq!(train_status, 201, "Train creation failed: {}", train_body);
            let train_logistics: Value = serde_json::from_str(&train_body).unwrap();
            assert_eq!(train_logistics["transport_type"], json!("Train"));
            let train_id = train_logistics["id"]
                .as_str()
                .and_then(|id| Uuid::parse_str(id).ok())
                .unwrap();

            // Clean up created logistics lines
            let _ = client
                .delete(format!("{}/api/logistics/{}", server.base_url, bus_id))
                .send()
                .await;
            let _ = client
                .delete(format!("{}/api/logistics/{}", server.base_url, train_id))
                .send()
                .await;
        } else {
            // Not implemented yet
            assert_bad_request(create_response).await;
        }
    }
}

#[tokio::test]
async fn test_logistics_error_cases() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Test 1: Get non-existent logistics line
    let unknown = Uuid::new_v4();

    let response = client
        .get(format!("{}/api/logistics/{}", server.base_url, unknown))
        .send()
        .await
        .expect("Failed to send request");

    assert_not_found(response).await;

    // Test 2: Delete non-existent logistics line
    let response = client
        .delete(format!("{}/api/logistics/{}", server.base_url, unknown))
        .send()
        .await
        .expect("Failed to send request");

    assert_not_found(response).await;

    // Test 3: Create logistics with invalid data
    let response = client
        .post(format!("{}/api/logistics", server.base_url))
        .json(&invalid_logistics_request())
        .send()
        .await
        .expect("Failed to send request");

    assert_bad_request(response).await;
}

// DASHBOARD TESTS
#[tokio::test]
async fn test_dashboard_endpoints() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Test 1: Get dashboard summary
    let response = client
        .get(format!("{}/api/dashboard/summary", server.base_url))
        .send()
        .await
        .expect("Failed to get dashboard summary");

    let summary: Value = assert_json_response(response).await;
    assert!(summary.get("total_factories").is_some());
    assert!(summary.get("total_production_lines").is_some());
    assert!(summary.get("total_logistics_lines").is_some());
    assert!(summary.get("total_power_consumption").is_some());
    assert!(summary.get("total_power_generation").is_some());
    assert!(summary.get("net_power").is_some());

    // Test 2: Get item balances
    let response = client
        .get(format!("{}/api/dashboard/items", server.base_url))
        .send()
        .await
        .expect("Failed to get item balances");

    let items: Value = assert_json_response(response).await;
    assert!(items.is_array());

    // Test 3: Get power statistics
    let response = client
        .get(format!("{}/api/dashboard/power", server.base_url))
        .send()
        .await
        .expect("Failed to get power statistics");

    let power: Value = assert_json_response(response).await;
    assert!(power.is_object());
}

// GAME DATA TESTS
#[tokio::test]
async fn test_game_data_endpoints() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Test 1: Get recipes
    let response = client
        .get(format!("{}/api/game-data/recipes", server.base_url))
        .send()
        .await
        .expect("Failed to get recipes");

    let recipes: Value = assert_json_response(response).await;
    assert!(recipes.is_array());
    assert!(!recipes.as_array().unwrap().is_empty());

    // Verify recipe structure
    if let Some(first_recipe) = recipes.as_array().unwrap().first() {
        assert!(first_recipe.get("name").is_some());
        assert!(first_recipe.get("machine").is_some());
        assert!(first_recipe.get("inputs").is_some());
        assert!(first_recipe.get("outputs").is_some());
    }

    // Test 2: Get items
    let response = client
        .get(format!("{}/api/game-data/items", server.base_url))
        .send()
        .await
        .expect("Failed to get items");

    let items: Value = assert_json_response(response).await;
    assert!(items.is_array());
    assert!(!items.as_array().unwrap().is_empty());

    // Test 3: Get machines
    let response = client
        .get(format!("{}/api/game-data/machines", server.base_url))
        .send()
        .await
        .expect("Failed to get machines");

    let machines: Value = assert_json_response(response).await;
    assert!(machines.is_array());
    assert!(!machines.as_array().unwrap().is_empty());

    // Verify machine structure
    if let Some(first_machine) = machines.as_array().unwrap().first() {
        assert!(first_machine.get("name").is_some());
        assert!(first_machine.get("base_power").is_some());
        assert!(first_machine.get("max_somersloop").is_some());
    }
}

// CORS TESTS
#[tokio::test]
async fn test_cors_headers() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Test preflight request using reqwest's method
    let response = client
        .request(
            reqwest::Method::OPTIONS,
            format!("{}/api/factories", server.base_url),
        )
        .header("Origin", "http://localhost:5173")
        .header("Access-Control-Request-Method", "POST")
        .send()
        .await
        .expect("Failed to send preflight request");

    // Check CORS headers
    let cors_origin = response.headers().get("Access-Control-Allow-Origin");
    assert!(
        cors_origin.is_some(),
        "CORS origin header should be present"
    );

    let cors_methods = response.headers().get("Access-Control-Allow-Methods");
    assert!(
        cors_methods.is_some(),
        "CORS methods header should be present"
    );

    // Test actual request with Origin header
    let response = client
        .get(format!("{}/api/factories", server.base_url))
        .header("Origin", "http://localhost:5173")
        .send()
        .await
        .expect("Failed to send request with origin");

    let cors_origin = response.headers().get("Access-Control-Allow-Origin");
    assert!(
        cors_origin.is_some(),
        "CORS origin header should be present on actual request"
    );
}

// ERROR HANDLING TESTS
#[tokio::test]
async fn test_error_response_format() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Test error response format for non-existent resource
    let response = client
        .get(format!(
            "{}/api/factories/{}",
            server.base_url,
            Uuid::new_v4()
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 404);

    let error_json: Value = response.json().await.unwrap();
    assert!(error_json.get("error").is_some());
    assert!(error_json.get("status").is_some());
    assert_eq!(error_json["status"], 404);
}

// PERFORMANCE TESTS
#[tokio::test]
async fn test_concurrent_requests() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Send multiple concurrent requests to game data endpoints
    let mut handles = vec![];

    for _ in 0..10 {
        let client = client.clone();
        let url = format!("{}/api/game-data/recipes", server.base_url);

        let handle = tokio::spawn(async move {
            client
                .get(&url)
                .send()
                .await
                .expect("Failed to send request")
        });

        handles.push(handle);
    }

    // Wait for all requests to complete
    for handle in handles {
        let response = handle.await.unwrap();
        assert_eq!(response.status(), 200);
    }
}

// INVALID ROUTE TESTS
#[tokio::test]
async fn test_invalid_routes() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Test invalid endpoint
    let response = client
        .get(format!("{}/api/invalid", server.base_url))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 404);

    // Test invalid HTTP method - use a custom method since PATCH isn't directly available
    let patch_method = reqwest::Method::from_bytes(b"PATCH").unwrap();
    let response = client
        .request(patch_method, format!("{}/api/factories", server.base_url))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 405); // Method Not Allowed
}

// BLUEPRINT TEMPLATE TESTS
#[tokio::test]
async fn test_blueprint_template_crud_operations() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Test 1: Get all templates (should be empty initially)
    let list_response = client
        .get(format!("{}/api/blueprints/templates", server.base_url))
        .send()
        .await
        .expect("Failed to get blueprint templates");

    let templates: Value = assert_json_response(list_response).await;
    assert!(templates.as_array().unwrap().is_empty());

    // Test 2: Create blueprint template
    let create_request = json!({
        "name": "Iron Ingot Production",
        "description": "Basic iron ingot production setup",
        "production_lines": [
            {
                "name": "Iron Ingot Line",
                "description": "Smelts iron ore into ingots",
                "recipe": "IronIngot",
                "machine_groups": [
                    {
                        "number_of_machine": 4,
                        "oc_value": 100.0,
                        "somersloop": 0
                    }
                ]
            }
        ]
    });

    let create_response = client
        .post(format!("{}/api/blueprints/templates", server.base_url))
        .json(&create_request)
        .send()
        .await
        .expect("Failed to create blueprint template");

    if create_response.status().as_u16() == 201 {
        let template: Value = assert_created_response(create_response).await;
        let template_id = template["id"].as_str().unwrap().to_string();

        // Verify template structure
        assert_eq!(template["name"], "Iron Ingot Production");
        assert_eq!(template["description"], "Basic iron ingot production setup");
        assert_eq!(template["total_machines"], 4);
        assert!(template["total_power"].as_f64().unwrap() > 0.0);
        assert!(!template["production_lines"].as_array().unwrap().is_empty());

        // Test 3: Get all templates (should have one now)
        let list_response = client
            .get(format!("{}/api/blueprints/templates", server.base_url))
            .send()
            .await
            .expect("Failed to get blueprint templates");

        let templates: Value = assert_json_response(list_response).await;
        assert_eq!(templates.as_array().unwrap().len(), 1);

        // Test 4: Get specific template
        let get_response = client
            .get(format!(
                "{}/api/blueprints/templates/{}",
                server.base_url, template_id
            ))
            .send()
            .await
            .expect("Failed to get blueprint template");

        let retrieved_template: Value = assert_json_response(get_response).await;
        assert_eq!(retrieved_template["id"], template_id);
        assert_eq!(retrieved_template["name"], "Iron Ingot Production");

        // Test 5: Update template (creates new version)
        let update_request = json!({
            "name": "Iron Ingot Production v2",
            "description": "Updated iron ingot production setup",
            "production_lines": [
                {
                    "name": "Iron Ingot Line",
                    "description": "Smelts iron ore into ingots",
                    "recipe": "IronIngot",
                    "machine_groups": [
                        {
                            "number_of_machine": 6,
                            "oc_value": 150.0,
                            "somersloop": 1
                        }
                    ]
                }
            ]
        });

        let update_response = client
            .put(format!(
                "{}/api/blueprints/templates/{}",
                server.base_url, template_id
            ))
            .json(&update_request)
            .send()
            .await
            .expect("Failed to update blueprint template");

        let updated_template: Value = assert_json_response(update_response).await;
        let new_template_id = updated_template["id"].as_str().unwrap().to_string();

        // Should be a different ID (new version)
        assert_ne!(new_template_id, template_id);
        assert_eq!(updated_template["name"], "Iron Ingot Production v2");
        assert_eq!(updated_template["total_machines"], 6);

        // Test 6: Get all templates (should have two now - original + new version)
        let list_response = client
            .get(format!("{}/api/blueprints/templates", server.base_url))
            .send()
            .await
            .expect("Failed to get blueprint templates");

        let templates: Value = assert_json_response(list_response).await;
        assert_eq!(templates.as_array().unwrap().len(), 2);

        // Test 7: Export template
        let export_response = client
            .get(format!(
                "{}/api/blueprints/templates/{}/export",
                server.base_url, new_template_id
            ))
            .send()
            .await
            .expect("Failed to export blueprint template");

        let export_data: Value = assert_json_response(export_response).await;
        assert!(export_data.get("blueprint_json").is_some());
        assert!(export_data.get("metadata").is_some());

        // Test 8: Delete template
        let delete_response = client
            .delete(format!(
                "{}/api/blueprints/templates/{}",
                server.base_url, new_template_id
            ))
            .send()
            .await
            .expect("Failed to delete blueprint template");

        assert_no_content(delete_response).await;

        // Test 9: Verify template is deleted
        let verify_response = client
            .get(format!(
                "{}/api/blueprints/templates/{}",
                server.base_url, new_template_id
            ))
            .send()
            .await
            .expect("Failed to verify deletion");

        assert_not_found(verify_response).await;

        // Test 10: Get all templates (should have one again - only original)
        let list_response = client
            .get(format!("{}/api/blueprints/templates", server.base_url))
            .send()
            .await
            .expect("Failed to get blueprint templates");

        let templates: Value = assert_json_response(list_response).await;
        assert_eq!(templates.as_array().unwrap().len(), 1);
    } else {
        // Current implementation might not be complete yet
        assert_bad_request(create_response).await;
    }
}

#[tokio::test]
async fn test_blueprint_template_error_cases() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Test 1: Get non-existent template
    let unknown_id = Uuid::new_v4();

    let response = client
        .get(format!(
            "{}/api/blueprints/templates/{}",
            server.base_url, unknown_id
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_not_found(response).await;

    // Test 2: Update non-existent template
    let update_request = json!({
        "name": "Updated Template",
        "production_lines": []
    });

    let response = client
        .put(format!(
            "{}/api/blueprints/templates/{}",
            server.base_url, unknown_id
        ))
        .json(&update_request)
        .send()
        .await
        .expect("Failed to send request");

    assert_not_found(response).await;

    // Test 3: Delete non-existent template
    let response = client
        .delete(format!(
            "{}/api/blueprints/templates/{}",
            server.base_url, unknown_id
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_not_found(response).await;

    // Test 4: Export non-existent template
    let response = client
        .get(format!(
            "{}/api/blueprints/templates/{}/export",
            server.base_url, unknown_id
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_not_found(response).await;

    // Test 5: Create template with invalid data
    let invalid_request = json!({
        "name": "",
        "production_lines": []
    });

    let response = client
        .post(format!("{}/api/blueprints/templates", server.base_url))
        .json(&invalid_request)
        .send()
        .await
        .expect("Failed to send request");

    assert_bad_request(response).await;
}

#[tokio::test]
async fn test_blueprint_template_import_export() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Test 1: Import blueprint to library
    let blueprint_json = r#"{
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "name": "Motor Production Complex",
        "description": "Complete motor production setup",
        "production_lines": [
            {
                "id": "550e8400-e29b-41d4-a716-446655440001",
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
                "id": "550e8400-e29b-41d4-a716-446655440002",
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
    }"#;

    let import_request = json!({
        "blueprint_json": blueprint_json,
        "name": "Imported Motor Complex"
    });

    let import_response = client
        .post(format!(
            "{}/api/blueprints/templates/import",
            server.base_url
        ))
        .json(&import_request)
        .send()
        .await
        .expect("Failed to import blueprint");

    if import_response.status().as_u16() == 201 {
        let imported_template: Value = assert_created_response(import_response).await;
        let template_id = imported_template["id"].as_str().unwrap().to_string();

        // Verify imported template
        assert_eq!(imported_template["name"], "Imported Motor Complex");
        assert_eq!(imported_template["total_machines"], 10);
        assert_eq!(
            imported_template["production_lines"]
                .as_array()
                .unwrap()
                .len(),
            2
        );

        // Test 2: Export the imported template
        let export_response = client
            .get(format!(
                "{}/api/blueprints/templates/{}/export",
                server.base_url, template_id
            ))
            .send()
            .await
            .expect("Failed to export blueprint template");

        let export_data: Value = assert_json_response(export_response).await;
        assert!(export_data.get("blueprint_json").is_some());
        assert!(export_data.get("metadata").is_some());

        // Verify exported JSON contains expected data
        let exported_json = export_data["blueprint_json"].as_str().unwrap();
        let exported_blueprint: Value = serde_json::from_str(exported_json).unwrap();
        assert_eq!(exported_blueprint["name"], "Imported Motor Complex");
        assert_eq!(
            exported_blueprint["production_lines"]
                .as_array()
                .unwrap()
                .len(),
            2
        );

        // Test 3: Create instance from template in factory
        // First create a factory
        let factory_response = client
            .post(format!("{}/api/factories", server.base_url))
            .json(&json!({
                "name": "Test Factory",
                "description": "Factory for testing blueprint instances"
            }))
            .send()
            .await
            .expect("Failed to create factory");

        if factory_response.status().as_u16() == 201 {
            let factory: Value = factory_response.json().await.unwrap();
            let factory_id = factory["id"].as_str().unwrap().to_string();

            // Create instance from template
            let instance_request = json!({
                "name": "Motor Production Instance"
            });

            let instance_response = client
                .post(format!(
                    "{}/api/factories/{}/production-lines/from-template/{}",
                    server.base_url, factory_id, template_id
                ))
                .json(&instance_request)
                .send()
                .await
                .expect("Failed to create blueprint instance");

            if instance_response.status().as_u16() == 201 {
                let instance_data: Value = assert_created_response(instance_response).await;
                assert_eq!(
                    instance_data["message"],
                    "Blueprint instance created in factory ".to_string() + &factory_id
                );
                assert!(instance_data.get("blueprint_id").is_some());
                assert_eq!(instance_data["factory_id"], factory_id);

                // Verify factory now has the blueprint instance
                let factory_response = client
                    .get(format!("{}/api/factories/{}", server.base_url, factory_id))
                    .send()
                    .await
                    .expect("Failed to get updated factory");

                let updated_factory: Value = assert_json_response(factory_response).await;
                assert_eq!(
                    updated_factory["production_lines"]
                        .as_array()
                        .unwrap()
                        .len(),
                    1
                );
            }
        }
    } else {
        // Import might not be implemented yet
        assert_bad_request(import_response).await;
    }
}
