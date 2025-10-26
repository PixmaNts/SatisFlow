//! Save/Load API handlers
//!
//! Provides endpoints for saving and loading the entire engine state.

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, state::AppState};
use satisflow_engine::{SaveFile, SaveFileSummary, SatisflowEngine};

/// Request body for loading a save file
#[derive(Debug, Deserialize)]
pub struct LoadRequest {
    /// JSON string of the save file
    pub save_data: String,
}

/// Response for save endpoint
#[derive(Debug, Serialize)]
pub struct SaveResponse {
    /// The save file as JSON string
    pub save_data: String,
    /// Summary information about the save
    pub summary: SaveFileSummary,
}

/// Response for successful load
#[derive(Debug, Serialize)]
pub struct LoadResponse {
    /// Success message
    pub message: String,
    /// Summary of loaded save file
    pub summary: SaveFileSummary,
}

/// Response for reset endpoint
#[derive(Debug, Serialize)]
pub struct ResetResponse {
    /// Success message
    pub message: String,
}

/// GET /api/save
///
/// Saves the current engine state and returns it as JSON
///
/// # Returns
///
/// - `200 OK` with save data and summary
/// - `500 Internal Server Error` if save fails
pub async fn save_engine(State(state): State<AppState>) -> Result<Json<SaveResponse>, AppError> {
    let engine = state.engine.read().await;

    // Save to JSON string
    let save_json = engine
        .save_to_json()
        .map_err(|e| AppError::EngineError(e.to_string()))?;

    // Parse to get summary
    let save_file: SaveFile = serde_json::from_str(&save_json)
        .map_err(|e| AppError::SerializationError(e))?;

    let summary = save_file.summary();

    Ok(Json(SaveResponse {
        save_data: save_json,
        summary,
    }))
}

/// POST /api/load
///
/// Loads a save file and replaces the current engine state
///
/// # Request Body
///
/// ```json
/// {
///   "save_data": "{ ... save file JSON ... }"
/// }
/// ```
///
/// # Returns
///
/// - `200 OK` with success message and summary
/// - `400 Bad Request` if JSON is invalid or version incompatible
/// - `500 Internal Server Error` if load fails
pub async fn load_engine(
    State(state): State<AppState>,
    Json(request): Json<LoadRequest>,
) -> Result<Json<LoadResponse>, AppError> {
    // Attempt to load the engine from JSON
    let new_engine = SatisflowEngine::load_from_json(&request.save_data)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Get summary before replacing
    let save_file: SaveFile = serde_json::from_str(&request.save_data)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let summary = save_file.summary();

    // Replace the engine state
    let mut engine = state.engine.write().await;
    *engine = new_engine;

    Ok(Json(LoadResponse {
        message: format!(
            "Successfully loaded save file (version {}, {} factories, {} logistics lines)",
            summary.version, summary.factory_count, summary.logistics_count
        ),
        summary,
    }))
}

/// POST /api/reset
///
/// Resets the engine to an empty state (clears all factories and logistics lines)
///
/// # Returns
///
/// - `200 OK` with success message
/// - `500 Internal Server Error` if reset fails
pub async fn reset_engine(
    State(state): State<AppState>,
) -> Result<Json<ResetResponse>, AppError> {
    // Reset the engine
    let mut engine = state.engine.write().await;
    engine
        .reset()
        .map_err(|e| AppError::EngineError(e.to_string()))?;

    Ok(Json(ResetResponse {
        message: "Engine reset successfully - all factories and logistics lines have been cleared"
            .to_string(),
    }))
}

// Route configuration
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/save", get(save_engine))
        .route("/load", post(load_engine))
        .route("/reset", post(reset_engine))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    fn create_test_state() -> AppState {
        AppState {
            engine: Arc::new(RwLock::new(SatisflowEngine::new())),
        }
    }

    #[tokio::test]
    async fn test_save_empty_engine() {
        let state = create_test_state();

        let result = save_engine(State(state)).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response.0.save_data.is_empty());
        assert_eq!(response.0.summary.factory_count, 0);
        assert_eq!(response.0.summary.logistics_count, 0);
    }

    #[tokio::test]
    async fn test_save_with_factories() {
        let state = create_test_state();

        // Add some factories
        {
            let mut engine = state.engine.write().await;
            engine.create_factory("Factory 1".to_string(), None);
            engine.create_factory("Factory 2".to_string(), None);
        }

        let result = save_engine(State(state)).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.0.summary.factory_count, 2);
        assert!(response.0.save_data.contains("Factory 1"));
        assert!(response.0.save_data.contains("Factory 2"));
    }

    #[tokio::test]
    async fn test_roundtrip_save_load() {
        let state = create_test_state();

        // Add data to engine
        {
            let mut engine = state.engine.write().await;
            engine.create_factory("Test Factory".to_string(), Some("Description".to_string()));
        }

        // Save
        let save_result = save_engine(State(state.clone())).await;
        assert!(save_result.is_ok());

        let save_data = save_result.unwrap().0.save_data;

        // Clear engine
        {
            let mut engine = state.engine.write().await;
            *engine = SatisflowEngine::new();
        }

        // Verify empty
        {
            let engine = state.engine.read().await;
            assert_eq!(engine.get_all_factories().len(), 0);
        }

        // Load
        let load_request = LoadRequest { save_data };
        let load_result = load_engine(State(state.clone()), Json(load_request)).await;
        assert!(load_result.is_ok());

        let response = load_result.unwrap();
        assert_eq!(response.0.summary.factory_count, 1);

        // Verify loaded
        {
            let engine = state.engine.read().await;
            assert_eq!(engine.get_all_factories().len(), 1);
            let factory = engine.get_all_factories().values().next().unwrap();
            assert_eq!(factory.name, "Test Factory");
            assert_eq!(factory.description.as_ref().unwrap(), "Description");
        }
    }

    #[tokio::test]
    async fn test_load_invalid_json() {
        let state = create_test_state();

        let request = LoadRequest {
            save_data: "{ invalid json }".to_string(),
        };

        let result = load_engine(State(state), Json(request)).await;
        assert!(result.is_err());

        let err = result.unwrap_err();
        match err {
            AppError::BadRequest(_) => {} // Expected
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_load_future_version() {
        let state = create_test_state();

        let request = LoadRequest {
            save_data: r#"{
                "version": "999.0.0",
                "created_at": "2025-10-25T12:00:00Z",
                "last_modified": "2025-10-25T12:00:00Z",
                "game_version": null,
                "engine": {
                    "factories": {},
                    "logistics_lines": {}
                }
            }"#
            .to_string(),
        };

        let result = load_engine(State(state), Json(request)).await;
        assert!(result.is_err());

        let err = result.unwrap_err();
        match err {
            AppError::BadRequest(msg) => {
                assert!(
                    msg.contains("incompatible") || msg.contains("too new"),
                    "Expected version error, got: {}",
                    msg
                );
            }
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_load_missing_version() {
        let state = create_test_state();

        let request = LoadRequest {
            save_data: r#"{
                "created_at": "2025-10-25T12:00:00Z",
                "last_modified": "2025-10-25T12:00:00Z",
                "game_version": null,
                "engine": {
                    "factories": {},
                    "logistics_lines": {}
                }
            }"#
            .to_string(),
        };

        let result = load_engine(State(state), Json(request)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reset_empty_engine() {
        let state = create_test_state();

        let result = reset_engine(State(state.clone())).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.0.message.contains("reset successfully"));

        // Verify engine is empty
        let engine = state.engine.read().await;
        assert_eq!(engine.get_all_factories().len(), 0);
        assert_eq!(engine.get_all_logistics().len(), 0);
    }

    #[tokio::test]
    async fn test_reset_with_data() {
        let state = create_test_state();

        // Add some data
        {
            let mut engine = state.engine.write().await;
            engine.create_factory("Factory 1".to_string(), None);
            engine.create_factory("Factory 2".to_string(), None);
        }

        // Verify data exists
        {
            let engine = state.engine.read().await;
            assert_eq!(engine.get_all_factories().len(), 2);
        }

        // Reset
        let result = reset_engine(State(state.clone())).await;
        assert!(result.is_ok());

        // Verify data is cleared
        {
            let engine = state.engine.read().await;
            assert_eq!(engine.get_all_factories().len(), 0);
            assert_eq!(engine.get_all_logistics().len(), 0);
        }
    }

    #[tokio::test]
    async fn test_reset_clears_logistics() {
        use satisflow_engine::models::logistics::{TransportType, TruckTransport};
        use satisflow_engine::models::Item;

        let state = create_test_state();

        // Add factories and logistics
        {
            let mut engine = state.engine.write().await;
            let factory1 = engine.create_factory("Factory 1".to_string(), None);
            let factory2 = engine.create_factory("Factory 2".to_string(), None);

            let transport = TransportType::Truck(TruckTransport::new(1, Item::IronOre, 60.0));
            engine
                .create_logistics_line(factory1, factory2, transport, "Test".to_string())
                .unwrap();
        }

        // Verify data exists
        {
            let engine = state.engine.read().await;
            assert_eq!(engine.get_all_factories().len(), 2);
            assert_eq!(engine.get_all_logistics().len(), 1);
        }

        // Reset
        let result = reset_engine(State(state.clone())).await;
        assert!(result.is_ok());

        // Verify everything is cleared
        {
            let engine = state.engine.read().await;
            assert_eq!(engine.get_all_factories().len(), 0);
            assert_eq!(engine.get_all_logistics().len(), 0);
        }
    }
}
