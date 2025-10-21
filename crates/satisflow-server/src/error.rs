// crates/satisflow-server/src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    InternalError(#[from] anyhow::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Engine error: {0}")]
    EngineError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::InternalError(ref e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                // Don't expose internal error details in production
                if cfg!(debug_assertions) {
                    e.to_string()
                } else {
                    "An internal error occurred".to_string()
                }
            ),
            AppError::SerializationError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to serialize/deserialize data".to_string()
            ),
            AppError::EngineError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;