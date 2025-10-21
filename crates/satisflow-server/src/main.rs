// crates/satisflow-server/src/main.rs
use axum::{
    http::Method,
    routing::get,
    Router,
};
use serde_json::json;
use std::env;
use std::net::SocketAddr;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod error;
mod handlers;
mod state;

use error::Result;
use handlers::{dashboard, factory, game_data, logistics};
use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Get configuration from environment variables
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid PORT format"))?;

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

    // Configure logging based on environment
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| match environment.as_str() {
        "production" => "info".to_string(),
        _ => "debug".to_string(),
    });

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&log_level));

    // Initialize tracing with JSON format for production
    if environment == "production" {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_current_span(false)
                    .with_span_list(false),
            )
            .init();
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    // Create application state
    let state = AppState::new();

    // Configure CORS based on environment variables
    let cors_origins = env::var("CORS_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:5173".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

    let cors = if environment == "production" {
        // Convert string origins to proper headers
        let origins: Vec<_> = cors_origins
            .iter()
            .map(|s| {
                s.parse().unwrap_or_else(|_| {
                    axum::http::HeaderValue::from_static("http://localhost:5173")
                })
            })
            .collect();

        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers(Any)
    } else {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers(Any)
    };

    // Build the application router
    let app = Router::new()
        // API routes
        .nest("/api/factories", factory::routes())
        .nest("/api/logistics", logistics::routes())
        .nest("/api/dashboard", dashboard::routes())
        .nest("/api/game-data", game_data::routes())
        // Health check
        .route("/health", get(health_check))
        // Global middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors),
        )
        .with_state(state);

    // Run the server
    let addr_str = format!("{}:{}", host, port);
    let addr: SocketAddr = addr_str
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid address: {}", e))?;

    info!(
        "Satisflow server listening on {} in {} mode",
        addr, environment
    );

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind to address: {}", e))?;

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    info!("Server shutdown complete");
    Ok(())
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "satisflow-server"
    }))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Received shutdown signal, shutting down gracefully");
}
