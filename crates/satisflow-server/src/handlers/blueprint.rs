//! Blueprint Import/Export API handlers
//!
//! Provides endpoints for exporting and importing ProductionLineBlueprint as JSON files.

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::AppError, state::AppState};
use satisflow_engine::models::{
    production_line::{ProductionLine, ProductionLineBlueprint},
    Item, ProductionLineId,
};

/// Metadata about a blueprint export
#[derive(Debug, Serialize, Deserialize)]
pub struct BlueprintMetadata {
    pub name: String,
    pub description: Option<String>,
    pub total_machines: u32,
    pub total_power: f32,
    pub input_items: Vec<(Item, f32)>,
    pub output_items: Vec<(Item, f32)>,
    pub exported_at: String,
}

/// Response for blueprint export
#[derive(Debug, Serialize)]
pub struct BlueprintExportResponse {
    /// The serialized blueprint as JSON string
    pub blueprint_json: String,
    /// Metadata about the blueprint
    pub metadata: BlueprintMetadata,
}

/// Request body for blueprint import
#[derive(Debug, Deserialize)]
pub struct BlueprintImportRequest {
    /// JSON string of the blueprint to import
    pub blueprint_json: String,
    /// Optional name override for the imported blueprint
    pub name: Option<String>,
}

/// Response for blueprint import (returns updated factory)
#[derive(Debug, Serialize)]
pub struct BlueprintImportResponse {
    pub message: String,
    pub blueprint_id: ProductionLineId,
    pub factory_id: Uuid,
}

/// GET /api/factories/:factory_id/production-lines/:line_id/export
///
/// Exports a production line blueprint as JSON
///
/// # Returns
///
/// - `200 OK` with blueprint JSON and metadata
/// - `404 Not Found` if factory or production line doesn't exist
/// - `400 Bad Request` if production line is not a blueprint
/// - `500 Internal Server Error` if serialization fails
pub async fn export_blueprint(
    State(state): State<AppState>,
    Path((factory_id, line_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<BlueprintExportResponse>, AppError> {
    let engine = state.engine.read().await;

    // Find the factory
    let factory = engine
        .get_factory(factory_id)
        .ok_or_else(|| AppError::NotFound(format!("Factory {} not found", factory_id)))?;

    // Find the production line
    let production_line = factory
        .production_lines
        .get(&line_id)
        .ok_or_else(|| AppError::NotFound(format!("Production line {} not found", line_id)))?;

    // Verify it's a blueprint
    let blueprint = match production_line {
        ProductionLine::ProductionLineBlueprint(bp) => bp,
        ProductionLine::ProductionLineRecipe(_) => {
            return Err(AppError::BadRequest(
                "Production line is not a blueprint (it's a recipe)".to_string(),
            ));
        }
    };

    // Serialize blueprint to JSON
    let blueprint_json =
        serde_json::to_string_pretty(blueprint).map_err(AppError::SerializationError)?;

    // Build metadata using the ProductionLine wrapper methods
    let metadata = BlueprintMetadata {
        name: blueprint.name.clone(),
        description: blueprint.description.clone(),
        total_machines: production_line.total_machines(),
        total_power: production_line.total_power_consumption(),
        input_items: production_line.input_rate(),
        output_items: production_line.output_rate(),
        exported_at: Utc::now().to_rfc3339(),
    };

    Ok(Json(BlueprintExportResponse {
        blueprint_json,
        metadata,
    }))
}

/// POST /api/factories/:factory_id/production-lines/import
///
/// Imports a blueprint JSON into a factory
///
/// # Request Body
///
/// ```json
/// {
///   "blueprint_json": "{ ... blueprint JSON ... }",
///   "name": "Optional Name Override"
/// }
/// ```
///
/// # Returns
///
/// - `200 OK` with success message and blueprint ID
/// - `404 Not Found` if factory doesn't exist
/// - `400 Bad Request` if JSON is invalid or validation fails
/// - `500 Internal Server Error` if import fails
pub async fn import_blueprint(
    State(state): State<AppState>,
    Path(factory_id): Path<Uuid>,
    Json(request): Json<BlueprintImportRequest>,
) -> Result<Json<BlueprintImportResponse>, AppError> {
    // Deserialize the blueprint JSON
    let mut blueprint: ProductionLineBlueprint = serde_json::from_str(&request.blueprint_json)
        .map_err(|e| AppError::BadRequest(format!("Invalid blueprint JSON: {}", e)))?;

    // Validate the blueprint structure
    validate_blueprint(&blueprint)?;

    // Generate new UUIDs to avoid conflicts
    blueprint.id = Uuid::new_v4();
    for line in &mut blueprint.production_lines {
        line.id = Uuid::new_v4();
    }

    // Override name if provided
    if let Some(name) = request.name {
        blueprint.name = name;
    }

    // Add blueprint to factory
    let mut engine = state.engine.write().await;
    let factory = engine
        .get_factory_mut(factory_id)
        .ok_or_else(|| AppError::NotFound(format!("Factory {} not found", factory_id)))?;

    let blueprint_id = blueprint.id;
    factory.production_lines.insert(
        blueprint_id,
        ProductionLine::ProductionLineBlueprint(blueprint),
    );

    Ok(Json(BlueprintImportResponse {
        message: format!(
            "Blueprint imported successfully into factory {}",
            factory_id
        ),
        blueprint_id,
        factory_id,
    }))
}

/// Validates a blueprint structure
///
/// Checks:
/// - Machine groups have valid overclock values (0-250%)
/// - Machine groups exist for recipes
fn validate_blueprint(blueprint: &ProductionLineBlueprint) -> Result<(), AppError> {
    // Validate each nested production line
    for line in &blueprint.production_lines {
        // Validate machine groups
        for group in &line.machine_groups {
            // Check overclock range
            if group.oc_value < 0.0 || group.oc_value > 250.0 {
                return Err(AppError::BadRequest(format!(
                    "Invalid overclock value {} in production line '{}'. Must be between 0 and 250",
                    group.oc_value, line.name
                )));
            }

            // Check number of machines is positive
            if group.number_of_machine == 0 {
                return Err(AppError::BadRequest(format!(
                    "Invalid machine count in production line '{}'. Must be greater than 0",
                    line.name
                )));
            }
        }
    }

    Ok(())
}

/// POST /api/blueprints/preview
///
/// Preview a blueprint JSON without importing it
/// Returns calculated metadata (power, machines, items) for preview
///
/// # Request Body
///
/// ```json
/// {
///   "blueprint_json": "{ ... blueprint JSON ... }"
/// }
/// ```
///
/// # Returns
///
/// - `200 OK` with blueprint metadata
/// - `400 Bad Request` if JSON is invalid
pub async fn preview_blueprint(
    Json(request): Json<BlueprintImportRequest>,
) -> Result<Json<BlueprintMetadata>, AppError> {
    // Deserialize the blueprint JSON
    let blueprint: ProductionLineBlueprint = serde_json::from_str(&request.blueprint_json)
        .map_err(|e| AppError::BadRequest(format!("Invalid Blueprint JSON: {}", e)))?;

    // Validate blueprint structure
    validate_blueprint(&blueprint)?;

    // Create a temporary ProductionLine wrapper to calculate metadata
    let production_line = ProductionLine::ProductionLineBlueprint(blueprint.clone());

    // Build metadata using engine methods
    let metadata = BlueprintMetadata {
        name: blueprint.name.clone(),
        description: blueprint.description.clone(),
        total_machines: production_line.total_machines(),
        total_power: production_line.total_power_consumption(),
        input_items: production_line.input_rate(),
        output_items: production_line.output_rate(),
        exported_at: Utc::now().to_rfc3339(),
    };

    Ok(Json(metadata))
}

// Route configuration
pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/factories/:factory_id/production-lines/:line_id/export",
            get(export_blueprint),
        )
        .route(
            "/factories/:factory_id/production-lines/import",
            post(import_blueprint),
        )
        .route("/blueprints/preview", post(preview_blueprint))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;
    use satisflow_engine::{
        models::{
            production_line::{
                MachineGroup, ProductionLine, ProductionLineBlueprint, ProductionLineRecipe,
            },
            Recipe,
        },
        SatisflowEngine,
    };
    use std::sync::Arc;
    use tokio::sync::RwLock;

    fn create_test_state() -> AppState {
        AppState {
            engine: Arc::new(RwLock::new(SatisflowEngine::new())),
        }
    }

    fn create_test_blueprint() -> ProductionLineBlueprint {
        let mut blueprint = ProductionLineBlueprint::new(
            Uuid::new_v4(),
            "Test Blueprint".to_string(),
            Some("A test blueprint for reinforced plates".to_string()),
        );

        // Add a production line for iron plates
        let mut iron_plate_line = ProductionLineRecipe::new(
            Uuid::new_v4(),
            "Iron Plate Line".to_string(),
            None,
            Recipe::IronPlate,
        );
        iron_plate_line
            .add_machine_group(MachineGroup::new(5, 100.0, 0))
            .unwrap();
        blueprint.add_production_line(iron_plate_line);

        // Add a production line for screws
        let mut screw_line = ProductionLineRecipe::new(
            Uuid::new_v4(),
            "Screw Line".to_string(),
            None,
            Recipe::Screw,
        );
        screw_line
            .add_machine_group(MachineGroup::new(2, 100.0, 0))
            .unwrap();
        blueprint.add_production_line(screw_line);

        blueprint
    }

    #[tokio::test]
    async fn test_export_blueprint_success() {
        let state = create_test_state();
        let blueprint = create_test_blueprint();
        let blueprint_id = blueprint.id;

        // Add factory with blueprint
        let factory_id = {
            let mut engine = state.engine.write().await;
            let factory_id = engine.create_factory("Test Factory".to_string(), None);
            let factory = engine.get_factory_mut(factory_id).unwrap();
            factory.production_lines.insert(
                blueprint_id,
                ProductionLine::ProductionLineBlueprint(blueprint),
            );
            factory_id
        };

        // Export blueprint
        let result = export_blueprint(State(state), Path((factory_id, blueprint_id))).await;

        assert!(result.is_ok());
        let response = result.unwrap();

        // Verify response contains valid JSON
        assert!(!response.0.blueprint_json.is_empty());
        assert!(response.0.blueprint_json.contains("Test Blueprint"));

        // Verify metadata
        assert_eq!(response.0.metadata.name, "Test Blueprint");
        assert_eq!(response.0.metadata.total_machines, 7); // 5 + 2
        assert!(response.0.metadata.total_power > 0.0);
    }

    #[tokio::test]
    async fn test_export_blueprint_not_found_factory() {
        let state = create_test_state();
        let fake_factory_id = Uuid::new_v4();
        let fake_line_id = Uuid::new_v4();

        let result = export_blueprint(State(state), Path((fake_factory_id, fake_line_id))).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(msg) => assert!(msg.contains("Factory")),
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_export_blueprint_not_found_line() {
        let state = create_test_state();

        // Create factory without blueprint
        let factory_id = {
            let mut engine = state.engine.write().await;
            engine.create_factory("Test Factory".to_string(), None)
        };

        let fake_line_id = Uuid::new_v4();

        let result = export_blueprint(State(state), Path((factory_id, fake_line_id))).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(msg) => assert!(msg.contains("Production line")),
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_export_non_blueprint_fails() {
        let state = create_test_state();

        // Add factory with regular recipe (not blueprint)
        let (factory_id, line_id) = {
            let mut engine = state.engine.write().await;
            let factory_id = engine.create_factory("Test Factory".to_string(), None);
            let factory = engine.get_factory_mut(factory_id).unwrap();

            let recipe_line = ProductionLineRecipe::new(
                Uuid::new_v4(),
                "Regular Recipe".to_string(),
                None,
                Recipe::IronPlate,
            );
            let line_id = recipe_line.id;
            factory
                .production_lines
                .insert(line_id, ProductionLine::ProductionLineRecipe(recipe_line));
            (factory_id, line_id)
        };

        let result = export_blueprint(State(state), Path((factory_id, line_id))).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => assert!(msg.contains("not a blueprint")),
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_import_blueprint_success() {
        let state = create_test_state();

        // Create factory
        let factory_id = {
            let mut engine = state.engine.write().await;
            engine.create_factory("Test Factory".to_string(), None)
        };

        // Serialize blueprint
        let blueprint = create_test_blueprint();
        let blueprint_json = serde_json::to_string(&blueprint).unwrap();

        let request = BlueprintImportRequest {
            blueprint_json,
            name: None,
        };

        let result = import_blueprint(State(state.clone()), Path(factory_id), Json(request)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.0.factory_id, factory_id);
        assert!(response.0.message.contains("imported successfully"));

        // Verify blueprint was added to factory
        {
            let engine = state.engine.read().await;
            let factory = engine.get_factory(factory_id).unwrap();
            assert_eq!(factory.production_lines.len(), 1);

            let (_, production_line) = factory.production_lines.iter().next().unwrap();
            match production_line {
                ProductionLine::ProductionLineBlueprint(bp) => {
                    assert_eq!(bp.name, "Test Blueprint");
                    assert_eq!(bp.production_lines.len(), 2);
                }
                _ => panic!("Expected blueprint"),
            }
        }
    }

    #[tokio::test]
    async fn test_import_blueprint_with_name_override() {
        let state = create_test_state();

        let factory_id = {
            let mut engine = state.engine.write().await;
            engine.create_factory("Test Factory".to_string(), None)
        };

        let blueprint = create_test_blueprint();
        let blueprint_json = serde_json::to_string(&blueprint).unwrap();

        let request = BlueprintImportRequest {
            blueprint_json,
            name: Some("Custom Name".to_string()),
        };

        let result = import_blueprint(State(state.clone()), Path(factory_id), Json(request)).await;

        assert!(result.is_ok());

        // Verify name was overridden
        {
            let engine = state.engine.read().await;
            let factory = engine.get_factory(factory_id).unwrap();

            let (_, production_line) = factory.production_lines.iter().next().unwrap();
            match production_line {
                ProductionLine::ProductionLineBlueprint(bp) => {
                    assert_eq!(bp.name, "Custom Name");
                }
                _ => panic!("Expected blueprint"),
            }
        }
    }

    #[tokio::test]
    async fn test_import_blueprint_generates_new_uuids() {
        let state = create_test_state();

        let factory_id = {
            let mut engine = state.engine.write().await;
            engine.create_factory("Test Factory".to_string(), None)
        };

        let blueprint = create_test_blueprint();
        let original_id = blueprint.id;
        let original_line_ids: Vec<_> = blueprint.production_lines.iter().map(|l| l.id).collect();

        let blueprint_json = serde_json::to_string(&blueprint).unwrap();

        let request = BlueprintImportRequest {
            blueprint_json,
            name: None,
        };

        let result = import_blueprint(State(state.clone()), Path(factory_id), Json(request)).await;

        assert!(result.is_ok());

        // Verify new UUIDs were generated
        {
            let engine = state.engine.read().await;
            let factory = engine.get_factory(factory_id).unwrap();

            let (_, production_line) = factory.production_lines.iter().next().unwrap();
            match production_line {
                ProductionLine::ProductionLineBlueprint(bp) => {
                    // Blueprint ID should be different
                    assert_ne!(bp.id, original_id);

                    // All nested line IDs should be different
                    for (i, line) in bp.production_lines.iter().enumerate() {
                        assert_ne!(line.id, original_line_ids[i]);
                    }
                }
                _ => panic!("Expected blueprint"),
            }
        }
    }

    #[tokio::test]
    async fn test_import_invalid_json() {
        let state = create_test_state();

        let factory_id = {
            let mut engine = state.engine.write().await;
            engine.create_factory("Test Factory".to_string(), None)
        };

        let request = BlueprintImportRequest {
            blueprint_json: "{ invalid json }".to_string(),
            name: None,
        };

        let result = import_blueprint(State(state), Path(factory_id), Json(request)).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => assert!(msg.contains("Invalid blueprint JSON")),
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_import_blueprint_factory_not_found() {
        let state = create_test_state();
        let fake_factory_id = Uuid::new_v4();

        let blueprint = create_test_blueprint();
        let blueprint_json = serde_json::to_string(&blueprint).unwrap();

        let request = BlueprintImportRequest {
            blueprint_json,
            name: None,
        };

        let result = import_blueprint(State(state), Path(fake_factory_id), Json(request)).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(msg) => assert!(msg.contains("Factory")),
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_validate_blueprint_invalid_overclock() {
        let mut blueprint = ProductionLineBlueprint::new(Uuid::new_v4(), "Test".to_string(), None);

        let mut line =
            ProductionLineRecipe::new(Uuid::new_v4(), "Line".to_string(), None, Recipe::IronPlate);

        // Add invalid machine group (overclock > 250%)
        line.machine_groups.push(MachineGroup::new(1, 300.0, 0));
        blueprint.add_production_line(line);

        let result = validate_blueprint(&blueprint);
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => assert!(msg.contains("overclock")),
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_validate_blueprint_zero_machines() {
        let mut blueprint = ProductionLineBlueprint::new(Uuid::new_v4(), "Test".to_string(), None);

        let mut line =
            ProductionLineRecipe::new(Uuid::new_v4(), "Line".to_string(), None, Recipe::IronPlate);

        // Add invalid machine group (0 machines)
        line.machine_groups.push(MachineGroup::new(0, 100.0, 0));
        blueprint.add_production_line(line);

        let result = validate_blueprint(&blueprint);
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => assert!(msg.contains("machine count")),
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_roundtrip_export_import() {
        let state = create_test_state();

        // Create factory with blueprint
        let (factory_id, blueprint_id) = {
            let mut engine = state.engine.write().await;
            let factory_id = engine.create_factory("Test Factory".to_string(), None);
            let factory = engine.get_factory_mut(factory_id).unwrap();

            let blueprint = create_test_blueprint();
            let blueprint_id = blueprint.id;
            factory.production_lines.insert(
                blueprint_id,
                ProductionLine::ProductionLineBlueprint(blueprint),
            );
            (factory_id, blueprint_id)
        };

        // Export
        let export_result =
            export_blueprint(State(state.clone()), Path((factory_id, blueprint_id))).await;

        assert!(export_result.is_ok());
        let export_response = export_result.unwrap();

        // Create second factory
        let factory2_id = {
            let mut engine = state.engine.write().await;
            engine.create_factory("Factory 2".to_string(), None)
        };

        // Import into second factory
        let import_request = BlueprintImportRequest {
            blueprint_json: export_response.0.blueprint_json,
            name: None,
        };

        let import_result = import_blueprint(
            State(state.clone()),
            Path(factory2_id),
            Json(import_request),
        )
        .await;

        assert!(import_result.is_ok());

        // Verify both factories have blueprints with same content (but different IDs)
        {
            let engine = state.engine.read().await;
            let factory1 = engine.get_factory(factory_id).unwrap();
            let factory2 = engine.get_factory(factory2_id).unwrap();

            let (_, line1) = factory1.production_lines.iter().next().unwrap();
            let (_, line2) = factory2.production_lines.iter().next().unwrap();

            match (line1, line2) {
                (
                    ProductionLine::ProductionLineBlueprint(bp1),
                    ProductionLine::ProductionLineBlueprint(bp2),
                ) => {
                    assert_ne!(bp1.id, bp2.id); // IDs should be different
                    assert_eq!(bp1.name, bp2.name); // Content should be same
                    assert_eq!(bp1.production_lines.len(), bp2.production_lines.len());
                }
                _ => panic!("Expected blueprints in both factories"),
            }
        }
    }

    #[tokio::test]
    async fn test_preview_blueprint() {
        let blueprint = create_test_blueprint();
        let blueprint_json = serde_json::to_string(&blueprint).unwrap();

        let request = BlueprintImportRequest {
            blueprint_json,
            name: None,
        };

        let result = preview_blueprint(Json(request)).await;

        assert!(result.is_ok());
        let metadata = result.unwrap().0;

        // Verify metadata
        assert_eq!(metadata.name, "Test Blueprint");
        assert_eq!(metadata.total_machines, 7); // 5 + 2
        assert!(metadata.total_power > 0.0);
    }

    #[tokio::test]
    async fn test_preview_blueprint_invalid_json() {
        let request = BlueprintImportRequest {
            blueprint_json: "{ invalid json }".to_string(),
            name: None,
        };

        let result = preview_blueprint(Json(request)).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => assert!(msg.contains("Invalid Blueprint JSON")),
            _ => panic!("Expected BadRequest error"),
        }
    }
}
