//! Logistics integration tests covering the transport checklist for trucks, buses,
//! and trains: identifier defaults, throughput validation, payload aggregation,
//! and error handling for malformed requests. Each case doubles as executable
//! documentation of the expected behaviour.

mod common;

use common::{
    create_test_client, create_test_server,
    test_data::{
        bus_with_whitespace_name_request, bus_with_zero_pipeline_request, minimal_factory_request,
        mixed_bus_logistics_request, train_empty_wagons_request, train_logistics_request,
        truck_logistics_request, truck_logistics_with_id_request,
    },
};
use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

/// Convenience helper to create a factory and return its identifier for wiring
/// up logistics lines within integration tests.
async fn create_factory(client: &Client, base_url: &str, name: &str) -> Uuid {
    client
        .post(&format!("{}/api/factories", base_url))
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

/// Truck happy path ensuring transport identifiers auto-increment and payload
/// totals line up with the submitted request.
#[tokio::test]
async fn logistics_truck_assigns_default_identifier() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_id = create_factory(&client, &server.base_url, "Source").await;
    let to_id = create_factory(&client, &server.base_url, "Sink").await;

    let response = client
        .post(&format!("{}/api/logistics", server.base_url))
        .json(&truck_logistics_request(from_id, to_id, "IronPlate", 180.0))
        .send()
        .await
        .expect("Failed to create truck logistics");

    assert_eq!(response.status().as_u16(), 201);
    let payload: Value = response.json().await.unwrap();
    assert_eq!(payload["transport_type"], "Truck");
    assert_eq!(payload["transport_id"], "TRK-1");

    let items = payload["items"].as_array().expect("Truck items missing");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["item"], "IronPlate");
    assert_eq!(items[0]["quantity_per_min"], 180.0);
    assert_eq!(payload["total_quantity_per_min"], 180.0);

    let details = payload["transport_details"]
        .as_str()
        .expect("Missing transport details");
    let details_json: Value = serde_json::from_str(details).expect("Transport details not JSON");
    assert_eq!(details_json["truck_id"], "TRK-001");
    assert_eq!(details_json["item"], "IronPlate");
    assert_eq!(details_json["quantity_per_min"], 180.0);
}

/// Truck validation case confirming non-positive throughput is rejected with a
/// descriptive error message.
#[tokio::test]
async fn logistics_truck_rejects_zero_quantity() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_id = create_factory(&client, &server.base_url, "Source").await;
    let to_id = create_factory(&client, &server.base_url, "Sink").await;

    let response = client
        .post(&format!("{}/api/logistics", server.base_url))
        .json(&truck_logistics_request(from_id, to_id, "IronOre", 0.0))
        .send()
        .await
        .expect("Failed request for zero throughput truck");

    assert_eq!(response.status().as_u16(), 400);
    let error_body: Value = response.json().await.unwrap();
    assert_eq!(error_body["status"], 400);
    assert!(
        error_body["error"]
            .as_str()
            .unwrap_or_default()
            .contains("must be greater than zero"),
        "Error message should clarify throughput constraint"
    );
}

/// Truck customisation path showing that user-provided identifiers are trimmed
/// and propagated through transport details.
#[tokio::test]
async fn logistics_truck_respects_custom_identifier() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_id = create_factory(&client, &server.base_url, "Source").await;
    let to_id = create_factory(&client, &server.base_url, "Sink").await;

    let response = client
        .post(&format!("{}/api/logistics", server.base_url))
        .json(&truck_logistics_with_id_request(
            from_id,
            to_id,
            "Wire",
            90.0,
            "  TRK-777  ",
        ))
        .send()
        .await
        .expect("Failed to create truck with custom identifier");

    assert_eq!(response.status().as_u16(), 201);
    let payload: Value = response.json().await.unwrap();
    assert_eq!(payload["transport_id"], "TRK-777");

    let details = payload["transport_details"]
        .as_str()
        .expect("Missing transport details");
    let details_json: Value = serde_json::from_str(details).expect("Transport details not JSON");
    assert_eq!(details_json["truck_id"], "TRK-777");
    assert_eq!(details_json["quantity_per_min"], 90.0);
}

/// Mixed bus scenario verifying totals across conveyors and pipelines.
#[tokio::test]
async fn logistics_bus_mixed_payload_totals_items() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_id = create_factory(&client, &server.base_url, "Bus Hub").await;
    let to_id = create_factory(&client, &server.base_url, "Bus Drop").await;

    let response = client
        .post(&format!("{}/api/logistics", server.base_url))
        .json(&mixed_bus_logistics_request(from_id, to_id))
        .send()
        .await
        .expect("Failed to create mixed bus logistics");

    assert_eq!(response.status().as_u16(), 201);
    let payload: Value = response.json().await.unwrap();
    assert_eq!(payload["transport_type"], "Bus");
    assert_eq!(payload["transport_id"], "BUS-1");
    assert_eq!(payload["transport_name"], "Hybrid Bus Route");
    assert_eq!(payload["total_quantity_per_min"], 660.0);

    let items = payload["items"].as_array().expect("Bus items missing");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["item"], "IronPlate");
    assert_eq!(items[0]["quantity_per_min"], 180.0);
    assert_eq!(items[1]["item"], "Water");
    assert_eq!(items[1]["quantity_per_min"], 480.0);
}

/// Bus validation confirming that at least one conveyor or pipeline segment is
/// required.
#[tokio::test]
async fn logistics_bus_requires_transport_segments() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_id = create_factory(&client, &server.base_url, "Bus Hub").await;
    let to_id = create_factory(&client, &server.base_url, "Bus Drop").await;

    let empty_bus = common::test_data::empty_bus_logistics_request(from_id, to_id);
    let response = client
        .post(&format!("{}/api/logistics", server.base_url))
        .json(&empty_bus)
        .send()
        .await
        .expect("Failed request for empty bus payload");

    assert_eq!(response.status().as_u16(), 400);
    let error_body: Value = response.json().await.unwrap();
    assert_eq!(error_body["status"], 400);
    assert!(
        error_body["error"]
            .as_str()
            .unwrap_or_default()
            .contains("requires at least one conveyor or pipeline"),
        "Expected bus validation error about required segments"
    );
}

/// Bus naming edge case where whitespace names fall back to generated labels.
#[tokio::test]
async fn logistics_bus_whitespace_name_defaults() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_id = create_factory(&client, &server.base_url, "Bus Hub").await;
    let to_id = create_factory(&client, &server.base_url, "Bus Drop").await;

    let response = client
        .post(&format!("{}/api/logistics", server.base_url))
        .json(&bus_with_whitespace_name_request(from_id, to_id))
        .send()
        .await
        .expect("Failed to create bus with whitespace name");

    assert_eq!(response.status().as_u16(), 201);
    let payload: Value = response.json().await.unwrap();
    assert_eq!(payload["transport_name"], "Bus 1");
}

/// Bus pipeline validation ensuring zero throughput is rejected.
#[tokio::test]
async fn logistics_bus_rejects_zero_pipeline_throughput() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_id = create_factory(&client, &server.base_url, "Bus Hub").await;
    let to_id = create_factory(&client, &server.base_url, "Bus Drop").await;

    let response = client
        .post(&format!("{}/api/logistics", server.base_url))
        .json(&bus_with_zero_pipeline_request(from_id, to_id))
        .send()
        .await
        .expect("Failed request for zero pipeline throughput");

    assert_eq!(response.status().as_u16(), 400);
    let error_body: Value = response.json().await.unwrap();
    assert!(
        error_body["error"]
            .as_str()
            .unwrap_or_default()
            .contains("Bus pipeline quantity_per_min must be greater than zero"),
        "Expected pipeline throughput validation error"
    );
}

/// Train validation covering the "unknown wagon type" negative path.
#[tokio::test]
async fn logistics_train_rejects_invalid_wagon_type() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_id = create_factory(&client, &server.base_url, "Train Loader").await;
    let to_id = create_factory(&client, &server.base_url, "Train Unloader").await;

    let response = client
        .post(&format!("{}/api/logistics", server.base_url))
        .json(&train_logistics_request(
            from_id,
            to_id,
            "Hover",
            "IronPlate",
            120.0,
        ))
        .send()
        .await
        .expect("Failed request for invalid train wagon type");

    assert_eq!(response.status().as_u16(), 400);
    let error_body: Value = response.json().await.unwrap();
    assert_eq!(error_body["status"], 400);
    assert!(
        error_body["error"]
            .as_str()
            .unwrap_or_default()
            .contains("Unknown wagon type"),
        "Expected wagon type validation error"
    );
}

/// Train validation asserting that at least one wagon must be present.
#[tokio::test]
async fn logistics_train_requires_wagon_entry() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_id = create_factory(&client, &server.base_url, "Train Loader").await;
    let to_id = create_factory(&client, &server.base_url, "Train Unloader").await;

    let response = client
        .post(&format!("{}/api/logistics", server.base_url))
        .json(&train_empty_wagons_request(from_id, to_id))
        .send()
        .await
        .expect("Failed request for empty wagon list");

    assert_eq!(response.status().as_u16(), 400);
    let error_body: Value = response.json().await.unwrap();
    assert!(
        error_body["error"]
            .as_str()
            .unwrap_or_default()
            .contains("Train transport requires at least one wagon"),
        "Expected wagon presence validation error"
    );
}

/// Shared validation verifying unknown item names produce clear error messages.
#[tokio::test]
async fn logistics_rejects_unknown_item() {
    let server = create_test_server().await;
    let client = create_test_client();

    let from_id = create_factory(&client, &server.base_url, "Item Source").await;
    let to_id = create_factory(&client, &server.base_url, "Item Sink").await;

    let response = client
        .post(&format!("{}/api/logistics", server.base_url))
        .json(&truck_logistics_request(
            from_id,
            to_id,
            "AlienProtein",
            30.0,
        ))
        .send()
        .await
        .expect("Failed request for unknown item");

    assert_eq!(response.status().as_u16(), 400);
    let error_body: Value = response.json().await.unwrap();
    assert_eq!(error_body["status"], 400);
    assert!(
        error_body["error"]
            .as_str()
            .unwrap_or_default()
            .contains("Unknown item type"),
        "Expected error describing unknown item"
    );
}
