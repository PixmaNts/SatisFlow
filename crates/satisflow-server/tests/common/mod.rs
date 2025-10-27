//! Shared integration-test harness for the Satisflow backend.
//!
//! Builders and assertions in this module encapsulate the factory lifecycle and
//! logistics scenarios listed in the backend testing checklist: blank-name and
//! whitespace validation for factories, cascade behaviour across logistics, and
//! payload builders for each transport type (truck, bus, train) so tests can
//! focus on behaviour instead of JSON boilerplate.
use axum::Router;
use satisflow_server::{
    handlers::{
        blueprint, blueprint_templates, dashboard, factory, game_data, logistics, save_load,
    },
    state::AppState,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

/// Minimal wrapper around the spawned Axum server.
pub struct TestServer {
    pub addr: SocketAddr,
    pub base_url: String,
}

/// Create a test server with the full routing tree used by integration tests.
pub async fn create_test_server() -> TestServer {
    // Create application state
    let state = AppState::new();

    // Build the application router (same as main.rs)
    let app = Router::new()
        // API routes
        .nest("/api/factories", factory::routes())
        .nest("/api/logistics", logistics::routes())
        .nest("/api/dashboard", dashboard::routes())
        .nest("/api/game-data", game_data::routes())
        .nest("/api", save_load::routes())
        .nest("/api", blueprint::routes())
        .nest("/api", blueprint_templates::routes())
        // Health check
        .route("/health", axum::routing::get(|| async { "OK" }))
        // Global middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([
                            axum::http::Method::GET,
                            axum::http::Method::POST,
                            axum::http::Method::PUT,
                            axum::http::Method::DELETE,
                        ])
                        .allow_headers(Any),
                ),
        )
        .with_state(state);

    // Bind to random port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base_url = format!("http://{}", addr);

    // Start server in background
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    TestServer { addr, base_url }
}

/// Construct a reqwest client configured for integration testing.
pub fn create_test_client() -> reqwest::Client {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
}

/// Common test data
pub mod test_data {
    use serde_json::json;
    use uuid::Uuid;

    /// Fully populated factory creation payload for metadata-based flows.
    pub fn create_factory_request() -> serde_json::Value {
        json!({
            "name": "Test Factory",
            "description": "A test factory for unit testing",
            "notes": "Test notes"
        })
    }

    /// Minimal factory payload exercising the "name only" happy path.
    pub fn minimal_factory_request(name: &str) -> serde_json::Value {
        json!({ "name": name })
    }

    /// Standard factory update payload including optional fields.
    pub fn update_factory_request() -> serde_json::Value {
        json!({
            "name": "Updated Factory",
            "description": Some("Updated description"),
            "notes": Some("Updated notes")
        })
    }

    /// Update payload dedicated to manipulating the notes field.
    pub fn update_factory_notes_request(notes: &str) -> serde_json::Value {
        json!({ "notes": notes })
    }

    /// Negative factory payload that forces blank-name validation.
    pub fn invalid_factory_request() -> serde_json::Value {
        json!({
            "name": "", // Empty name should be invalid
            "description": "Invalid factory"
        })
    }

    /// Factory payload with user-supplied notes to test trimming behaviour.
    pub fn factory_with_notes_request(name: &str, notes: &str) -> serde_json::Value {
        json!({
            "name": name,
            "notes": notes
        })
    }

    /// Truck payload using generated identifiers to validate defaults.
    pub fn truck_logistics_request(
        from_factory: Uuid,
        to_factory: Uuid,
        item: &str,
        quantity_per_min: f32,
    ) -> serde_json::Value {
        json!({
            "from_factory": from_factory,
            "to_factory": to_factory,
            "transport_type": "Truck",
            "item": item,
            "quantity_per_min": quantity_per_min
        })
    }

    /// Truck payload providing an explicit identifier to validate parsing.
    pub fn truck_logistics_with_id_request(
        from_factory: Uuid,
        to_factory: Uuid,
        item: &str,
        quantity_per_min: f32,
        truck_id: &str,
    ) -> serde_json::Value {
        json!({
            "from_factory": from_factory,
            "to_factory": to_factory,
            "transport_type": "Truck",
            "item": item,
            "quantity_per_min": quantity_per_min,
            "truck_id": truck_id
        })
    }

    /// Bus payload with no segments, used for validation failure.
    pub fn empty_bus_logistics_request(from_factory: Uuid, to_factory: Uuid) -> serde_json::Value {
        json!({
            "from_factory": from_factory,
            "to_factory": to_factory,
            "transport_type": "Bus",
            "conveyors": [],
            "pipelines": []
        })
    }

    /// Mixed conveyor/pipeline bus payload for aggregation assertions.
    pub fn mixed_bus_logistics_request(from_factory: Uuid, to_factory: Uuid) -> serde_json::Value {
        json!({
            "from_factory": from_factory,
            "to_factory": to_factory,
            "transport_type": "Bus",
            "bus_name": "Hybrid Bus Route",
            "conveyors": [
                {
                    "line_id": "CV-101",
                    "conveyor_type": "Mk4",
                    "item": "IronPlate",
                    "quantity_per_min": 180.0
                }
            ],
            "pipelines": [
                {
                    "pipeline_id": "PL-501",
                    "pipeline_type": "Mk2",
                    "item": "Water",
                    "quantity_per_min": 480.0
                }
            ]
        })
    }

    /// Bus payload containing whitespace-only name to trigger defaulting.
    pub fn bus_with_whitespace_name_request(
        from_factory: Uuid,
        to_factory: Uuid,
    ) -> serde_json::Value {
        json!({
            "from_factory": from_factory,
            "to_factory": to_factory,
            "transport_type": "Bus",
            "bus_name": "   ",
            "conveyors": [
                {
                    "conveyor_type": "Mk2",
                    "item": "Wire",
                    "quantity_per_min": 120.0
                }
            ]
        })
    }

    /// Bus payload containing zero-throughput pipeline to trigger validation.
    pub fn bus_with_zero_pipeline_request(
        from_factory: Uuid,
        to_factory: Uuid,
    ) -> serde_json::Value {
        json!({
            "from_factory": from_factory,
            "to_factory": to_factory,
            "transport_type": "Bus",
            "pipelines": [
                {
                    "pipeline_type": "Mk1",
                    "item": "Water",
                    "quantity_per_min": 0.0
                }
            ]
        })
    }

    /// Train payload helper for type-specific wagon construction.
    pub fn train_logistics_request(
        from_factory: Uuid,
        to_factory: Uuid,
        wagon_type: &str,
        item: &str,
        quantity_per_min: f32,
    ) -> serde_json::Value {
        json!({
            "from_factory": from_factory,
            "to_factory": to_factory,
            "transport_type": "Train",
            "train_name": "Test Train",
            "wagons": [
                {
                    "wagon_id": "WG-900",
                    "wagon_type": wagon_type,
                    "item": item,
                    "quantity_per_min": quantity_per_min
                }
            ]
        })
    }

    /// Train payload with no wagons used to assert minimum elements.
    pub fn train_empty_wagons_request(from_factory: Uuid, to_factory: Uuid) -> serde_json::Value {
        json!({
            "from_factory": from_factory,
            "to_factory": to_factory,
            "transport_type": "Train",
            "train_name": "Edge Train",
            "wagons": []
        })
    }

    /// Negative logistics payload pointing to unknown factories and zero flow.
    pub fn invalid_logistics_request() -> serde_json::Value {
        json!({
            "from_factory": Uuid::new_v4(), // Non-existent factory
            "to_factory": Uuid::new_v4(),
            "transport_type": "Truck",
            "item": "IronOre",
            "quantity_per_min": 0.0
        })
    }
}

/// Assertion helpers
pub mod assertions {
    use reqwest::Response;
    use serde_json::Value;

    /// Assert that a response carries the expected status code and include an
    /// informative panic message when it does not.
    pub async fn assert_status(response: Response, expected_status: u16) {
        assert_eq!(
            response.status().as_u16(),
            expected_status,
            "Expected status {}, got {}",
            expected_status,
            response.status().as_u16()
        );
    }

    /// Obtain a 200 OK JSON payload, panicking otherwise.
    pub async fn assert_json_response(response: Response) -> Value {
        let status = response.status();
        assert_eq!(status.as_u16(), 200);
        response.json().await.unwrap()
    }

    /// Obtain a 201 Created JSON payload, panicking otherwise.
    pub async fn assert_created_response(response: Response) -> Value {
        let status = response.status();
        assert_eq!(status.as_u16(), 201);
        response.json().await.unwrap()
    }

    /// Convenience helper asserting a 204 No Content status.
    pub async fn assert_no_content(response: Response) {
        assert_status(response, 204).await;
    }

    /// Convenience helper asserting a 404 Not Found status.
    pub async fn assert_not_found(response: Response) {
        assert_status(response, 404).await;
    }

    /// Convenience helper asserting a 400 Bad Request status.
    pub async fn assert_bad_request(response: Response) {
        assert_status(response, 400).await;
    }

    /// Convenience helper asserting either 400 Bad Request or 422 Unprocessable Entity.
    /// Use this for validation errors that may be caught at different layers.
    pub async fn assert_bad_request_or_unprocessable(response: Response) {
        let status = response.status().as_u16();
        assert!(
            status == 400 || status == 422,
            "Expected status 400 or 422, got {}",
            status
        );
    }

    /// Assert that a JSON error payload contains a descriptive substring.
    pub fn assert_contains_error(json: &Value, expected_error: &str) {
        let error = json.get("error").and_then(|e| e.as_str()).unwrap_or("");
        assert!(
            error.contains(expected_error),
            "Expected error to contain '{}', got '{}'",
            expected_error,
            error
        );
    }
}
