//! Factory lifecycle integration tests covering the agreed backend checklist:
//! - Create: minimal payload defaults, metadata handling, whitespace trimming.
//! - Validate: reject blank names with contract-compliant errors.
//! - Update: ensure notes trimming applies during edits.
//! - Delete: verify logistics lines cascade when factories are removed.

mod common;

use common::{
    assertions::{assert_json_response, assert_no_content},
    create_test_client, create_test_server,
    test_data::{
        create_factory_request, factory_with_notes_request, minimal_factory_request,
        truck_logistics_request, update_factory_notes_request,
    },
};
use serde_json::Value;
use uuid::Uuid;

/// Rejects blank factory names and asserts the contract `{error,status}` shape.
#[tokio::test]
async fn factory_create_rejects_blank_name() {
    let server = create_test_server().await;
    let client = create_test_client();

    let response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&minimal_factory_request(""))
        .send()
        .await
        .expect("Failed to create factory with blank name");

    assert_eq!(response.status().as_u16(), 400);
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["status"], 400);
    assert_eq!(body["error"], "Factory name cannot be empty");
}

/// Confirms the minimal create payload defaults optional fields to `null`.
#[tokio::test]
async fn factory_create_defaults_optional_fields() {
    let server = create_test_server().await;
    let client = create_test_client();

    let response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&minimal_factory_request("Compact Factory"))
        .send()
        .await
        .expect("Failed to create minimal factory");

    assert_eq!(response.status().as_u16(), 201);
    let factory: Value = response.json().await.unwrap();
    assert_eq!(factory["name"], "Compact Factory");
    assert!(factory["description"].is_null());
    assert!(factory["notes"].is_null());
}

/// Ensures whitespace-only notes collapse to `None`, documenting the trimming
/// rule on creation.
#[tokio::test]
async fn factory_create_trims_whitespace_notes() {
    let server = create_test_server().await;
    let client = create_test_client();

    let response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&factory_with_notes_request("Whitespace Factory", "    "))
        .send()
        .await
        .expect("Failed to create factory with whitespace notes");

    assert_eq!(response.status().as_u16(), 201);
    let factory: Value = response.json().await.unwrap();
    assert!(factory["notes"].is_null());
}

/// Demonstrates the same notes trimming behaviour on update, validating the
/// idempotent PUT path.
#[tokio::test]
async fn factory_update_clears_notes_when_empty() {
    let server = create_test_server().await;
    let client = create_test_client();

    let create_response = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&create_factory_request())
        .send()
        .await
        .expect("Failed to seed factory");

    let created: Value = create_response.json().await.unwrap();
    let factory_id = created["id"]
        .as_str()
        .and_then(|value| Uuid::parse_str(value).ok())
        .expect("Factory id missing from create response");

    let update_response = client
        .put(format!("{}/api/factories/{}", server.base_url, factory_id))
        .json(&update_factory_notes_request("   "))
        .send()
        .await
        .expect("Failed to update factory notes");

    assert_eq!(update_response.status().as_u16(), 200);
    let updated: Value = update_response.json().await.unwrap();
    assert!(updated["notes"].is_null());
}

/// Verifies logistics cascades when deleting a factory, covering the cross-domain
/// regression scenario.
#[tokio::test]
async fn factory_delete_cascades_logistics() {
    let server = create_test_server().await;
    let client = create_test_client();

    // Seed two factories to connect via logistics.
    let factory_a = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&minimal_factory_request("Alpha"))
        .send()
        .await
        .expect("Failed to create factory Alpha")
        .json::<Value>()
        .await
        .unwrap();

    let factory_b = client
        .post(format!("{}/api/factories", server.base_url))
        .json(&minimal_factory_request("Beta"))
        .send()
        .await
        .expect("Failed to create factory Beta")
        .json::<Value>()
        .await
        .unwrap();

    let factory_a_id = factory_a["id"]
        .as_str()
        .and_then(|id| Uuid::parse_str(id).ok())
        .unwrap();
    let factory_b_id = factory_b["id"]
        .as_str()
        .and_then(|id| Uuid::parse_str(id).ok())
        .unwrap();

    // Create a logistics line between the factories.
    let logistics_response = client
        .post(format!("{}/api/logistics", server.base_url))
        .json(&truck_logistics_request(
            factory_a_id,
            factory_b_id,
            "IronOre",
            60.0,
        ))
        .send()
        .await
        .expect("Failed to create logistics line");

    assert_eq!(logistics_response.status().as_u16(), 201);
    let logistics: Value = logistics_response.json().await.unwrap();
    let logistics_id = logistics["id"]
        .as_str()
        .and_then(|id| Uuid::parse_str(id).ok())
        .unwrap();

    // Delete factory A and ensure the logistics line is removed as part of the cascade.
    let delete_response = client
        .delete(format!(
            "{}/api/factories/{}",
            server.base_url, factory_a_id
        ))
        .send()
        .await
        .expect("Failed to delete factory");
    assert_no_content(delete_response).await;

    let get_response = client
        .get(format!(
            "{}/api/logistics/{}",
            server.base_url, logistics_id
        ))
        .send()
        .await
        .expect("Failed to fetch logistics after cascade");

    assert_eq!(get_response.status().as_u16(), 404);
    let missing_body: Value = get_response.json().await.unwrap();
    assert_eq!(missing_body["status"], 404);
    assert!(
        missing_body["error"]
            .as_str()
            .unwrap_or_default()
            .contains("not found"),
        "Expected logistics fetch error message to reference missing line"
    );

    // Confirm logistics collection is empty to guard against orphaned entries.
    let list_response = client
        .get(format!("{}/api/logistics", server.base_url))
        .send()
        .await
        .expect("Failed to list logistics after cascade");
    let list: Value = assert_json_response(list_response).await;
    assert!(list.as_array().unwrap().is_empty());
}
