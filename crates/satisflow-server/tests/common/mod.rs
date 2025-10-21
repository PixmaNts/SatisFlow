// Test utilities for satisflow-server
use axum::Router;
use satisflow_server::{
    handlers::{dashboard, factory, game_data, logistics},
    state::AppState,
};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

/// Test server configuration
pub struct TestServer {
    pub addr: SocketAddr,
    pub base_url: String,
}

/// Create a test server with all routes
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

/// Create a reqwest client for testing
pub fn create_test_client() -> reqwest::Client {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
}

/// Common test data
pub mod test_data {
    use serde_json::json;

    pub fn create_factory_request() -> serde_json::Value {
        json!({
            "name": "Test Factory",
            "description": "A test factory for unit testing",
            "notes": "Test notes"
        })
    }

    pub fn update_factory_request() -> serde_json::Value {
        json!({
            "name": "Updated Factory",
            "description": Some("Updated description"),
            "notes": Some("Updated notes")
        })
    }

    pub fn create_logistics_request() -> serde_json::Value {
        json!({
            "from_factory": 1,
            "to_factory": 2,
            "transport_type": "truck",
            "transport_details": "Test truck transport"
        })
    }

    pub fn invalid_factory_request() -> serde_json::Value {
        json!({
            "name": "", // Empty name should be invalid
            "description": "Invalid factory"
        })
    }

    pub fn invalid_logistics_request() -> serde_json::Value {
        json!({
            "from_factory": 999, // Non-existent factory
            "to_factory": 1000,
            "transport_type": "invalid",
            "transport_details": ""
        })
    }
}

/// Assertion helpers
pub mod assertions {
    use reqwest::Response;
    use serde_json::Value;

    pub async fn assert_status(response: Response, expected_status: u16) {
        assert_eq!(
            response.status().as_u16(),
            expected_status,
            "Expected status {}, got {}",
            expected_status,
            response.status().as_u16()
        );
    }

    pub async fn assert_json_response(response: Response) -> Value {
        let status = response.status();
        assert_eq!(status.as_u16(), 200);
        response.json().await.unwrap()
    }

    pub async fn assert_created_response(response: Response) -> Value {
        let status = response.status();
        assert_eq!(status.as_u16(), 201);
        response.json().await.unwrap()
    }

    pub async fn assert_no_content(response: Response) {
        assert_status(response, 204).await;
    }

    pub async fn assert_not_found(response: Response) {
        assert_status(response, 404).await;
    }

    pub async fn assert_bad_request(response: Response) {
        assert_status(response, 400).await;
    }

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
