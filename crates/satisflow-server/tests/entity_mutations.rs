//! Integration tests covering entity lifecycle mutations:
//! - Factory update lifecycle (create → update name → update description → delete)
//! - Production line machine group mutations (add, update, remove)
//! - Logistics child entity mutations (Bus conveyors, Train wagons, Truck/Drone fields)
//! - Blueprint template in-place update (verify ID preservation)

mod common;

use common::{
    assertions::{
        assert_created_response, assert_json_response, assert_no_content, assert_not_found,
    },
    create_test_client, create_test_server,
    test_data::{
        create_blueprint_template_request, create_factory_request, minimal_factory_request,
    },
};
use reqwest::Client;
use serde_json::{json, Value};
use uuid::Uuid;

/// Create a factory and return its ID
async fn create_factory(client: &Client, base_url: &str, name: &str) -> Uuid {
    client
        .post(format!("{}/api/factories", base_url))
        .json(&minimal_factory_request(name))
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap()["id"]
        .as_str()
        .and_then(|raw| Uuid::parse_str(raw).ok())
        .expect("Factory id missing or invalid")
}

/// Create a production line and return its ID
async fn create_production_line(
    client: &Client,
    base_url: &str,
    factory_id: Uuid,
) -> Uuid {
    let request = json!({
        "name": "Test Production Line",
        "description": "Test line for mutations",
        "type": "recipe",
        "recipe": "Iron Ingot",
        "machine_groups": [
            {
                "number_of_machine": 2,
                "oc_value": 100.0,
                "somersloop": 0
            }
        ]
    });

    let response = client
        .post(format!(
            "{}/api/factories/{}/production-lines",
            base_url, factory_id
        ))
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 201);

    let factory: Value = response.json().await.unwrap();
    factory["production_lines"]
        .as_array()
        .and_then(|lines| lines.last())
        .and_then(|line| line["ProductionLineRecipe"]["id"].as_str())
        .and_then(|id| Uuid::parse_str(id).ok())
        .expect("Production line ID missing")
}

/// Create a logistics line and return the logistics ID
async fn create_logistics_line(
    client: &Client,
    base_url: &str,
    from_factory: Uuid,
    to_factory: Uuid,
) -> Uuid {
    let request = json!({
        "from_factory": from_factory,
        "to_factory": to_factory,
        "transport_type": "Truck",
        "item": "IronOre",
        "quantity_per_min": 60.0
    });

    let response = client
        .post(format!("{}/api/logistics", base_url))
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 201);

    let logistics: Value = response.json().await.unwrap();
    logistics["id"]
        .as_str()
        .and_then(|id| Uuid::parse_str(id).ok())
        .expect("Logistics ID missing")
}

/// Create a bus logistics line and return the logistics ID
async fn create_bus_logistics(
    client: &Client,
    base_url: &str,
    from_factory: Uuid,
    to_factory: Uuid,
) -> Uuid {
    let request = json!({
        "from_factory": from_factory,
        "to_factory": to_factory,
        "transport_type": "Bus",
        "bus_name": "Test Bus Route",
        "conveyors": [
            {
                "line_id": "1",
                "conveyor_type": "Mk3",
                "item": "IronPlate",
                "quantity_per_min": 270.0
            }
        ]
    });

    let response = client
        .post(format!("{}/api/logistics", base_url))
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 201);

    let logistics: Value = response.json().await.unwrap();
    logistics["id"]
        .as_str()
        .and_then(|id| Uuid::parse_str(id).ok())
        .expect("Logistics ID missing")
}

/// Create a train logistics line and return the logistics ID
async fn create_train_logistics(
    client: &Client,
    base_url: &str,
    from_factory: Uuid,
    to_factory: Uuid,
) -> Uuid {
    let request = json!({
        "from_factory": from_factory,
        "to_factory": to_factory,
        "transport_type": "Train",
        "train_name": "Test Train Route",
        "wagons": [
            {
                "wagon_id": "1",
                "wagon_type": "Cargo",
                "item": "IronPlate",
                "quantity_per_min": 120.0
            }
        ]
    });

    let response = client
        .post(format!("{}/api/logistics", base_url))
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 201);

    let logistics: Value = response.json().await.unwrap();
    logistics["id"]
        .as_str()
        .and_then(|id| Uuid::parse_str(id).ok())
        .expect("Logistics ID missing")
}

/// Create a blueprint template and return the template ID
async fn create_blueprint_template(client: &Client, base_url: &str) -> Uuid {
    let request = json!({
        "name": "Test Blueprint Template",
        "description": "A test blueprint for integration testing",
        "production_lines": [
            {
                "name": "Iron Ingot Production",
                "description": "Basic iron ingot production",
                "recipe": "Iron Ingot",
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

    let response = client
        .post(format!("{}/api/blueprints/templates", base_url))
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 201);

    let template: Value = response.json().await.unwrap();
    template["id"]
        .as_str()
        .and_then(|id| Uuid::parse_str(id).ok())
        .expect("Template ID missing")
}

// =============================================================================
// FACTORY UPDATE LIFECYCLE TESTS
// =============================================================================

#[tokio::test]
async fn factory_update_lifecycle_complete() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Step 1: Create factory
    let create_response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&create_factory_request())
        .send()
        .await
        .expect("Failed to create factory");

    assert_eq!(create_response.status().as_u16(), 201);
    let factory: Value = create_response.json().await.unwrap();
    let factory_id = factory["id"]
        .as_str()
        .and_then(|id| Uuid::parse_str(id).ok())
        .expect("Factory ID missing");

    assert_eq!(factory["name"], "Test Factory");
    assert_eq!(factory["description"], "A test factory for unit testing");
    assert_eq!(factory["notes"], "Test notes");

    // Step 2: Update name only
    let update_name_request = json!({ "name": "Updated Factory Name" });

    let update_name_response = client
        .put(format!("{}/api/factories/{}", server.base_url, factory_id))
        .json(&update_name_request)
        .send()
        .await
        .expect("Failed to update factory name");

    assert_eq!(update_name_response.status().as_u16(), 200);
    let updated_factory: Value = update_name_response.json().await.unwrap();

    assert_eq!(updated_factory["name"], "Updated Factory Name");
    assert_eq!(updated_factory["description"], "A test factory for unit testing");
    assert_eq!(updated_factory["notes"], "Test notes");

    // Step 3: Update description only
    let update_description_request = json!({ "description": "Updated description for testing" });

    let update_description_response = client
        .put(format!("{}/api/factories/{}", server.base_url, factory_id))
        .json(&update_description_request)
        .send()
        .await
        .expect("Failed to update factory description");

    assert_eq!(update_description_response.status().as_u16(), 200);
    let updated_factory2: Value = update_description_response.json().await.unwrap();

    assert_eq!(updated_factory2["name"], "Updated Factory Name");
    assert_eq!(updated_factory2["description"], "Updated description for testing");
    assert_eq!(updated_factory2["notes"], "Test notes");

    // Step 4: Delete factory
    let delete_response = client
        .delete(format!("{}/api/factories/{}", server.base_url, factory_id))
        .send()
        .await
        .expect("Failed to delete factory");

    assert_no_content(delete_response).await;

    // Step 5: Verify factory is deleted
    let get_response = client
        .get(format!("{}/api/factories/{}", server.base_url, factory_id))
        .send()
        .await
        .expect("Failed to get factory after delete");

    assert_not_found(get_response).await;
}

#[tokio::test]
async fn factory_partial_updates() {
    let server = create_test_server().await;
    let client = create_test_client();

    let factory_id = create_factory(&client, &server.base_url, "Partial Update Factory").await;

    // Update only name
    let update1 = client
        .put(format!("{}/api/factories/{}", server.base_url, factory_id))
        .json(&json!({ "name": "New Name" }))
        .send()
        .await
        .unwrap();

    assert_eq!(update1.status().as_u16(), 200);
    let factory1: Value = update1.json().await.unwrap();
    assert_eq!(factory1["name"], "New Name");
    assert!(factory1["description"].is_null());

    // Update only description
    let update2 = client
        .put(format!("{}/api/factories/{}", server.base_url, factory_id))
        .json(&json!({ "description": "A description" }))
        .send()
        .await
        .unwrap();

    assert_eq!(update2.status().as_u16(), 200);
    let factory2: Value = update2.json().await.unwrap();
    assert_eq!(factory2["description"], "A description");

    // Update only notes
    let update3 = client
        .put(format!("{}/api/factories/{}", server.base_url, factory_id))
        .json(&json!({ "notes": "Important notes here" }))
        .send()
        .await
        .unwrap();

    assert_eq!(update3.status().as_u16(), 200);
    let factory3: Value = update3.json().await.unwrap();
    assert_eq!(factory3["notes"], "Important notes here");

    // Clear notes with whitespace
    let update4 = client
        .put(format!("{}/api/factories/{}", server.base_url, factory_id))
        .json(&json!({ "notes": "   " }))
        .send()
        .await
        .unwrap();

    assert_eq!(update4.status().as_u16(), 200);
    let factory4: Value = update4.json().await.unwrap();
    assert!(factory4["notes"].is_null());
}

// =============================================================================
// PRODUCTION LINE MACHINE GROUP MUTATIONS
// =============================================================================

#[tokio::test]
async fn production_line_add_machine_groups() {
    let server = create_test_server().await;
    let client = create_test_client();

    let factory_id = create_factory(&client, &server.base_url, "Machine Group Test Factory").await;
    let line_id = create_production_line(&client, &server.base_url, factory_id).await;

    // Get initial state
    let initial_response = client
        .get(format!("{}/api/factories/{}", server.base_url, factory_id))
        .send()
        .await
        .unwrap();

    let initial_factory: Value = initial_response.json().await.unwrap();
    let initial_line = initial_factory["production_lines"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["ProductionLineRecipe"]["id"] == line_id.to_string())
        .cloned()
        .expect("Production line not found");

    let initial_groups = initial_line["ProductionLineRecipe"]["machine_groups"]
        .as_array()
        .unwrap();
    assert_eq!(initial_groups.len(), 1);
    assert_eq!(initial_groups[0]["number_of_machine"], 2);

    // Update with additional machine group
    let update_request = json!({
        "name": "Test Production Line",
        "type": "recipe",
        "recipe": "Iron Ingot",
        "machine_groups": [
            {
                "number_of_machine": 2,
                "oc_value": 100.0,
                "somersloop": 0
            },
            {
                "number_of_machine": 3,
                "oc_value": 150.0,
                "somersloop": 0
            }
        ]
    });

    let update_response = client
        .put(format!(
            "{}/api/factories/{}/production-lines/{}",
            server.base_url, factory_id, line_id
        ))
        .json(&update_request)
        .send()
        .await
        .unwrap();

    assert_eq!(update_response.status().as_u16(), 200);
    let updated_factory: Value = update_response.json().await.unwrap();
    let updated_line = updated_factory["production_lines"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["ProductionLineRecipe"]["id"] == line_id.to_string())
        .cloned()
        .expect("Production line not found");

    let updated_groups = updated_line["ProductionLineRecipe"]["machine_groups"]
        .as_array()
        .unwrap();
    assert_eq!(updated_groups.len(), 2);
    assert_eq!(updated_groups[0]["number_of_machine"], 2);
    assert_eq!(updated_groups[1]["number_of_machine"], 3);

    // Verify totals updated
    assert_eq!(updated_line["total_machines"], 5);
}

#[tokio::test]
async fn production_line_update_machine_group_params() {
    let server = create_test_server().await;
    let client = create_test_client();

    let factory_id = create_factory(&client, &server.base_url, "Machine Group Update Factory").await;
    let line_id = create_production_line(&client, &server.base_url, factory_id).await;

    // Update machine group with new OC and somersloop
    let update_request = json!({
        "name": "Test Production Line",
        "type": "recipe",
        "recipe": "Iron Ingot",
        "machine_groups": [
            {
                "number_of_machine": 2,
                "oc_value": 200.0,
                "somersloop": 1
            }
        ]
    });

    let update_response = client
        .put(format!(
            "{}/api/factories/{}/production-lines/{}",
            server.base_url, factory_id, line_id
        ))
        .json(&update_request)
        .send()
        .await
        .unwrap();

    assert_eq!(update_response.status().as_u16(), 200);
    let updated_factory: Value = update_response.json().await.unwrap();
    let updated_line = updated_factory["production_lines"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["ProductionLineRecipe"]["id"] == line_id.to_string())
        .cloned()
        .expect("Production line not found");

    let updated_groups = updated_line["ProductionLineRecipe"]["machine_groups"]
        .as_array()
        .unwrap();
    assert_eq!(updated_groups.len(), 1);
    assert_eq!(updated_groups[0]["oc_value"], 200.0);
    assert_eq!(updated_groups[0]["somersloop"], 1);

    // Verify totals reflect somersloop
    assert_eq!(updated_line["total_somersloop"], 2);
}

#[tokio::test]
async fn production_line_remove_machine_groups() {
    let server = create_test_server().await;
    let client = create_test_client();

    let factory_id = create_factory(&client, &server.base_url, "Machine Group Remove Factory").await;
    let line_id = create_production_line(&client, &server.base_url, factory_id).await;

    // First add a second machine group
    let add_request = json!({
        "name": "Test Production Line",
        "type": "recipe",
        "recipe": "Iron Ingot",
        "machine_groups": [
            {
                "number_of_machine": 2,
                "oc_value": 100.0,
                "somersloop": 0
            },
            {
                "number_of_machine": 3,
                "oc_value": 150.0,
                "somersloop": 0
            }
        ]
    });

    let _ = client
        .put(format!(
            "{}/api/factories/{}/production-lines/{}",
            server.base_url, factory_id, line_id
        ))
        .json(&add_request)
        .send()
        .await
        .unwrap();

    // Now remove the second group
    let remove_request = json!({
        "name": "Test Production Line",
        "type": "recipe",
        "recipe": "Iron Ingot",
        "machine_groups": [
            {
                "number_of_machine": 2,
                "oc_value": 100.0,
                "somersloop": 0
            }
        ]
    });

    let remove_response = client
        .put(format!(
            "{}/api/factories/{}/production-lines/{}",
            server.base_url, factory_id, line_id
        ))
        .json(&remove_request)
        .send()
        .await
        .unwrap();

    assert_eq!(remove_response.status().as_u16(), 200);
    let updated_factory: Value = remove_response.json().await.unwrap();
    let updated_line = updated_factory["production_lines"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["ProductionLineRecipe"]["id"] == line_id.to_string())
        .cloned()
        .expect("Production line not found");

    let updated_groups = updated_line["ProductionLineRecipe"]["machine_groups"]
        .as_array()
        .unwrap();
    assert_eq!(updated_groups.len(), 1);
    assert_eq!(updated_groups[0]["number_of_machine"], 2);
    assert_eq!(updated_line["total_machines"], 2);
}

// =============================================================================
// LOGISTICS CHILD ENTITY MUTATIONS
// =============================================================================

#[tokio::test]
async fn bus_add_conveyor() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_factory = create_factory(&client, &server.base_url, "Bus Source").await;
    let to_factory = create_factory(&client, &server.base_url, "Bus Destination").await;
    let logistics_id = create_bus_logistics(&client, &server.base_url, from_factory, to_factory).await;

    // Add a conveyor to the bus
    let add_request = json!({
        "line_id": "2",
        "conveyor_type": "Mk4",
        "item": "Wire",
        "quantity_per_min": 480.0
    });

    let add_response = client
        .post(format!(
            "{}/api/logistics/{}/conveyors",
            server.base_url, logistics_id
        ))
        .json(&add_request)
        .send()
        .await
        .unwrap();

    assert_eq!(add_response.status().as_u16(), 201);
    let updated_logistics: Value = add_response.json().await.unwrap();
    assert_eq!(updated_logistics["total_quantity_per_min"], 750.0); // 270 + 480
}

#[tokio::test]
async fn bus_update_conveyor() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_factory = create_factory(&client, &server.base_url, "Bus Source").await;
    let to_factory = create_factory(&client, &server.base_url, "Bus Destination").await;
    let logistics_id = create_bus_logistics(&client, &server.base_url, from_factory, to_factory).await;

    // Update the conveyor
    let update_request = json!({
        "conveyor_type": "Mk4",
        "item": "IronRod",
        "quantity_per_min": 180.0
    });

    let update_response = client
        .put(format!(
            "{}/api/logistics/{}/conveyors/1",
            server.base_url, logistics_id
        ))
        .json(&update_request)
        .send()
        .await
        .unwrap();

    assert_eq!(update_response.status().as_u16(), 200);
    let updated_logistics: Value = update_response.json().await.unwrap();
    assert_eq!(updated_logistics["total_quantity_per_min"], 180.0);

    let items = updated_logistics["items"].as_array().unwrap();
    assert_eq!(items[0]["item"], "IronRod");
    assert_eq!(items[0]["quantity_per_min"], 180.0);
}

#[tokio::test]
async fn bus_remove_conveyor() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_factory = create_factory(&client, &server.base_url, "Bus Source").await;
    let to_factory = create_factory(&client, &server.base_url, "Bus Destination").await;

    // Create bus with two conveyors
    let request = json!({
        "from_factory": from_factory,
        "to_factory": to_factory,
        "transport_type": "Bus",
        "bus_name": "Test Bus",
        "conveyors": [
            {
                "line_id": "1",
                "conveyor_type": "Mk3",
                "item": "IronPlate",
                "quantity_per_min": 270.0
            },
            {
                "line_id": "2",
                "conveyor_type": "Mk4",
                "item": "Wire",
                "quantity_per_min": 480.0
            }
        ]
    });

    let create_response = client
        .post(format!("{}/api/logistics", server.base_url))
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(create_response.status().as_u16(), 201);
    let logistics: Value = create_response.json().await.unwrap();
    let logistics_id = logistics["id"].as_str().unwrap();

    // Remove one conveyor
    let remove_response = client
        .delete(format!(
            "{}/api/logistics/{}/conveyors/1",
            server.base_url, logistics_id
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(remove_response.status().as_u16(), 204);
}

#[tokio::test]
async fn train_add_wagon() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_factory = create_factory(&client, &server.base_url, "Train Source").await;
    let to_factory = create_factory(&client, &server.base_url, "Train Destination").await;
    let logistics_id = create_train_logistics(&client, &server.base_url, from_factory, to_factory).await;

    // Add a wagon to the train
    let add_request = json!({
        "wagon_id": "2",
        "wagon_type": "Fluid",
        "item": "Water",
        "quantity_per_min": 240.0
    });

    let add_response = client
        .post(format!(
            "{}/api/logistics/{}/wagons",
            server.base_url, logistics_id
        ))
        .json(&add_request)
        .send()
        .await
        .unwrap();

    assert_eq!(add_response.status().as_u16(), 201);
    let updated_logistics: Value = add_response.json().await.unwrap();
    assert_eq!(updated_logistics["total_quantity_per_min"], 360.0); // 120 + 240
}

#[tokio::test]
async fn train_update_wagon() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_factory = create_factory(&client, &server.base_url, "Train Source").await;
    let to_factory = create_factory(&client, &server.base_url, "Train Destination").await;
    let logistics_id = create_train_logistics(&client, &server.base_url, from_factory, to_factory).await;

    // Update the wagon
    let update_request = json!({
        "wagon_type": "Cargo",
        "item": "CopperSheet",
        "quantity_per_min": 200.0
    });

    let update_response = client
        .put(format!(
            "{}/api/logistics/{}/wagons/1",
            server.base_url, logistics_id
        ))
        .json(&update_request)
        .send()
        .await
        .unwrap();

    assert_eq!(update_response.status().as_u16(), 200);
    let updated_logistics: Value = update_response.json().await.unwrap();
    assert_eq!(updated_logistics["total_quantity_per_min"], 200.0);

    let items = updated_logistics["items"].as_array().unwrap();
    assert_eq!(items[0]["item"], "CopperSheet");
}

#[tokio::test]
async fn train_remove_wagon() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_factory = create_factory(&client, &server.base_url, "Train Source").await;
    let to_factory = create_factory(&client, &server.base_url, "Train Destination").await;

    // Create train with two wagons
    let request = json!({
        "from_factory": from_factory,
        "to_factory": to_factory,
        "transport_type": "Train",
        "train_name": "Test Train",
        "wagons": [
            {
                "wagon_id": "1",
                "wagon_type": "Cargo",
                "item": "IronPlate",
                "quantity_per_min": 120.0
            },
            {
                "wagon_id": "2",
                "wagon_type": "Cargo",
                "item": "CopperSheet",
                "quantity_per_min": 100.0
            }
        ]
    });

    let create_response = client
        .post(format!("{}/api/logistics", server.base_url))
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(create_response.status().as_u16(), 201);
    let logistics: Value = create_response.json().await.unwrap();
    let logistics_id = logistics["id"].as_str().unwrap();

    // Remove one wagon
    let remove_response = client
        .delete(format!(
            "{}/api/logistics/{}/wagons/1",
            server.base_url, logistics_id
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(remove_response.status().as_u16(), 204);
}

#[tokio::test]
async fn truck_field_updates() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_factory = create_factory(&client, &server.base_url, "Truck Source").await;
    let to_factory = create_factory(&client, &server.base_url, "Truck Destination").await;
    let logistics_id = create_logistics_line(&client, &server.base_url, from_factory, to_factory).await;

    // Update truck fields
    let update_request = json!({
        "item": "CopperOre",
        "quantity_per_min": 120.0
    });

    let update_response = client
        .put(format!(
            "{}/api/logistics/{}/truck",
            server.base_url, logistics_id
        ))
        .json(&update_request)
        .send()
        .await
        .unwrap();

    assert_eq!(update_response.status().as_u16(), 200);
    let updated_logistics: Value = update_response.json().await.unwrap();
    assert_eq!(updated_logistics["total_quantity_per_min"], 120.0);

    let items = updated_logistics["items"].as_array().unwrap();
    assert_eq!(items[0]["item"], "CopperOre");
}

#[tokio::test]
async fn drone_field_updates() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_factory = create_factory(&client, &server.base_url, "Drone Source").await;
    let to_factory = create_factory(&client, &server.base_url, "Drone Destination").await;

    // Create drone logistics
    let request = json!({
        "from_factory": from_factory,
        "to_factory": to_factory,
        "transport_type": "Drone",
        "item": "IronOre",
        "quantity_per_min": 60.0
    });

    let create_response = client
        .post(format!("{}/api/logistics", server.base_url))
        .json(&request)
        .send()
        .await
        .unwrap();

    let logistics: Value = create_response.json().await.unwrap();
    let logistics_id = logistics["id"].as_str().unwrap();

    // Update drone fields
    let update_request = json!({
        "item": "CompactedCoal",
        "quantity_per_min": 45.0
    });

    let update_response = client
        .put(format!(
            "{}/api/logistics/{}/drone",
            server.base_url, logistics_id
        ))
        .json(&update_request)
        .send()
        .await
        .unwrap();

    assert_eq!(update_response.status().as_u16(), 200);
    let updated_logistics: Value = update_response.json().await.unwrap();
    assert_eq!(updated_logistics["total_quantity_per_min"], 45.0);

    let items = updated_logistics["items"].as_array().unwrap();
    assert_eq!(items[0]["item"], "CompactedCoal");
}

// =============================================================================
// BLUEPRINT TEMPLATE IN-PLACE UPDATE
// =============================================================================

#[tokio::test]
async fn blueprint_template_in_place_update_preserves_id() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Step 1: Create blueprint template
    let template_id = create_blueprint_template(&client, &server.base_url).await;

    // Get initial template
    let get_response = client
        .get(format!(
            "{}/api/blueprints/templates/{}",
            server.base_url, template_id
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(get_response.status().as_u16(), 200);
    let initial_template: Value = get_response.json().await.unwrap();

    assert_eq!(initial_template["name"], "Test Blueprint Template");
    assert_eq!(
        initial_template["production_lines"]
            .as_array()
            .unwrap()
            .len(),
        1
    );

    // Step 2: Update template in-place
    let update_request = json!({
        "name": "Updated Blueprint Template",
        "description": "Updated description for testing",
        "production_lines": [
            {
                "name": "Updated Iron Ingot Production",
                "description": "Updated production line",
                "recipe": "Iron Ingot",
                "machine_groups": [
                    {
                        "number_of_machine": 6,
                        "oc_value": 150.0,
                        "somersloop": 1
                    },
                    {
                        "number_of_machine": 4,
                        "oc_value": 200.0,
                        "somersloop": 0
                    }
                ]
            },
            {
                "name": "Copper Ingot Production",
                "description": "New production line",
                "recipe": "Copper Ingot",
                "machine_groups": [
                    {
                        "number_of_machine": 3,
                        "oc_value": 100.0,
                        "somersloop": 0
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
        .unwrap();

    assert_eq!(update_response.status().as_u16(), 200);
    let updated_template: Value = update_response.json().await.unwrap();

    // Step 3: Verify ID is preserved
    let updated_id = updated_template["id"]
        .as_str()
        .and_then(|id| Uuid::parse_str(id).ok())
        .expect("Updated template ID missing");
    assert_eq!(updated_id, template_id, "Template ID should be preserved during in-place update");

    // Step 4: Verify updates applied
    assert_eq!(updated_template["name"], "Updated Blueprint Template");
    assert_eq!(
        updated_template["production_lines"]
            .as_array()
            .unwrap()
            .len(),
        2
    );

    // Verify total_machines updated
    assert_eq!(updated_template["total_machines"], 13); // 6 + 4 + 3

    // Step 5: Verify changes persisted by fetching again
    let get_response2 = client
        .get(format!(
            "{}/api/blueprints/templates/{}",
            server.base_url, template_id
        ))
        .send()
        .await
        .unwrap();

    let fetched_template: Value = get_response2.json().await.unwrap();
    assert_eq!(fetched_template["name"], "Updated Blueprint Template");
    assert_eq!(fetched_template["total_machines"], 13);
}

#[tokio::test]
async fn blueprint_template_update_partial() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Create blueprint template
    let template_id = create_blueprint_template(&client, &server.base_url).await;

    // Update only name and description
    let update_request = json!({
        "name": "Renamed Template",
        "description": "New description",
        "production_lines": [
            {
                "name": "Iron Ingot Production",
                "description": "Basic iron ingot production",
                "recipe": "Iron Ingot",
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

    let update_response = client
        .put(format!(
            "{}/api/blueprints/templates/{}",
            server.base_url, template_id
        ))
        .json(&update_request)
        .send()
        .await
        .unwrap();

    assert_eq!(update_response.status().as_u16(), 200);
    let updated_template: Value = update_response.json().await.unwrap();

    assert_eq!(updated_template["name"], "Renamed Template");
    assert_eq!(updated_template["id"], template_id.to_string());
}
