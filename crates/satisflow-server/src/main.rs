use axum::{
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Serialize)]
struct Health {
    status: String,
    version: String,
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
    password: String,
}

async fn health() -> Json<Health> {
    Json(Health {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn register(Json(payload): Json<CreateUser>) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Implement user registration
    info!("Registration attempt for user: {}", payload.username);

    Ok(Json(serde_json::json!({
        "message": "Registration endpoint - not yet implemented",
        "username": payload.username
    })))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/auth/register", post(register))
        .layer(
            ServiceBuilder::new().layer(CorsLayer::permissive()), // TODO: Restrict in production
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!(
        "SatisFlow server starting on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
