//! Blueprint Template Library API handlers
//!
//! Provides endpoints for managing blueprint templates in the central library.
//! Templates are reusable blueprints that can be instantiated into factories.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::AppError, state::AppState};
use satisflow_engine::models::{
    production_line::{
        MachineGroup, ProductionLine, ProductionLineBlueprint, ProductionLineRecipe,
    },
    recipes::recipe_by_name,
    Item, ProductionLineId,
};

/// Response for a single blueprint template
#[derive(Debug, Serialize)]
pub struct BlueprintTemplateResponse {
    pub id: ProductionLineId,
    pub name: String,
    pub description: Option<String>,
    pub production_lines: Vec<ProductionLineRecipeInfo>,
    pub total_machines: u32,
    pub total_power: f32,
    pub input_items: Vec<(Item, f32)>,
    pub output_items: Vec<(Item, f32)>,
}

/// Information about a production line within a blueprint
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductionLineRecipeInfo {
    pub id: ProductionLineId,
    pub name: String,
    pub description: Option<String>,
    pub recipe: String,
    pub machine_groups: Vec<MachineGroupInfo>,
}

/// Machine group information
#[derive(Debug, Serialize, Deserialize)]
pub struct MachineGroupInfo {
    pub number_of_machine: u32,
    pub oc_value: f32,
    pub somersloop: u8,
}

/// Request for creating a new blueprint template
#[derive(Debug, Deserialize)]
pub struct CreateBlueprintTemplateRequest {
    pub name: String,
    pub description: Option<String>,
    pub production_lines: Vec<CreateProductionLineRequest>,
}

/// Request for creating a production line within a blueprint
#[derive(Debug, Deserialize)]
pub struct CreateProductionLineRequest {
    pub name: String,
    pub description: Option<String>,
    pub recipe: String,
    pub machine_groups: Vec<MachineGroupInfo>,
}

/// Request for importing a blueprint to the library
#[derive(Debug, Deserialize)]
pub struct ImportTemplateRequest {
    pub blueprint_json: String,
    pub name: Option<String>,
}

/// Response for blueprint export
#[derive(Debug, Serialize)]
pub struct ExportTemplateResponse {
    pub blueprint_json: String,
    pub metadata: TemplateMetadata,
}

/// Metadata for template export
#[derive(Debug, Serialize)]
pub struct TemplateMetadata {
    pub name: String,
    pub description: Option<String>,
    pub total_machines: u32,
    pub total_power: f32,
    pub input_items: Vec<(Item, f32)>,
    pub output_items: Vec<(Item, f32)>,
    pub exported_at: String,
}

/// Request for creating an instance from a template
#[derive(Debug, Deserialize)]
pub struct CreateFromTemplateRequest {
    pub name: Option<String>,
}

/// Response for creating instance from template
#[derive(Debug, Serialize)]
pub struct CreateFromTemplateResponse {
    pub message: String,
    pub blueprint_id: ProductionLineId,
    pub factory_id: Uuid,
}

impl From<&ProductionLineBlueprint> for BlueprintTemplateResponse {
    fn from(blueprint: &ProductionLineBlueprint) -> Self {
        let production_line = ProductionLine::ProductionLineBlueprint(blueprint.clone());

        Self {
            id: blueprint.id,
            name: blueprint.name.clone(),
            description: blueprint.description.clone(),
            production_lines: blueprint
                .production_lines
                .iter()
                .map(|line| ProductionLineRecipeInfo {
                    id: line.id,
                    name: line.name.clone(),
                    description: line.description.clone(),
                    recipe: format!("{:?}", line.recipe),
                    machine_groups: line
                        .machine_groups
                        .iter()
                        .map(|mg| MachineGroupInfo {
                            number_of_machine: mg.number_of_machine,
                            oc_value: mg.oc_value,
                            somersloop: mg.somersloop,
                        })
                        .collect(),
                })
                .collect(),
            total_machines: production_line.total_machines(),
            total_power: production_line.total_power_consumption(),
            input_items: production_line.input_rate(),
            output_items: production_line.output_rate(),
        }
    }
}

/// GET /api/blueprints/templates
///
/// Get all blueprint templates from the library
///
/// # Returns
///
/// - `200 OK` with list of templates
pub async fn get_all_templates(
    State(state): State<AppState>,
) -> Result<Json<Vec<BlueprintTemplateResponse>>, AppError> {
    let engine = state.engine.read().await;

    let templates: Vec<BlueprintTemplateResponse> = engine
        .get_all_blueprint_templates()
        .values()
        .map(|template| template.into())
        .collect();

    Ok(Json(templates))
}

/// GET /api/blueprints/templates/:id
///
/// Get a specific blueprint template by ID
///
/// # Returns
///
/// - `200 OK` with template
/// - `404 Not Found` if template doesn't exist
pub async fn get_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BlueprintTemplateResponse>, AppError> {
    let engine = state.engine.read().await;

    let template = engine
        .get_blueprint_template(id)
        .ok_or_else(|| AppError::NotFound(format!("Blueprint template {} not found", id)))?;

    Ok(Json(template.into()))
}

/// POST /api/blueprints/templates
///
/// Create a new blueprint template
///
/// # Returns
///
/// - `201 Created` with created template
/// - `400 Bad Request` if validation fails
pub async fn create_template(
    State(state): State<AppState>,
    Json(request): Json<CreateBlueprintTemplateRequest>,
) -> Result<(StatusCode, Json<BlueprintTemplateResponse>), AppError> {
    // Validate name is not empty
    if request.name.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Blueprint name cannot be empty".to_string(),
        ));
    }

    let mut blueprint =
        ProductionLineBlueprint::new(Uuid::new_v4(), request.name, request.description);

    // Convert request production lines to actual ProductionLineRecipe instances
    for line_request in request.production_lines {
        // Parse recipe string
        let recipe = recipe_by_name(&line_request.recipe).ok_or_else(|| {
            AppError::BadRequest(format!("Invalid recipe: {}", line_request.recipe))
        })?;

        let mut line = ProductionLineRecipe::new(
            Uuid::new_v4(),
            line_request.name,
            line_request.description,
            recipe,
        );

        // Add machine groups
        for mg in line_request.machine_groups {
            line.add_machine_group(MachineGroup::new(
                mg.number_of_machine,
                mg.oc_value,
                mg.somersloop,
            ))
            .map_err(|e| AppError::BadRequest(e.to_string()))?;
        }

        blueprint.add_production_line(line);
    }

    // Validate the blueprint
    validate_template(&blueprint)?;

    // Add to engine
    let mut engine = state.engine.write().await;
    engine.add_blueprint_template(blueprint.clone());

    Ok((StatusCode::CREATED, Json((&blueprint).into())))
}

/// PUT /api/blueprints/templates/:id
///
/// Update a template (creates new version with new ID)
///
/// # Returns
///
/// - `200 OK` with new template
/// - `404 Not Found` if original template doesn't exist
/// - `400 Bad Request` if validation fails
pub async fn update_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<CreateBlueprintTemplateRequest>,
) -> Result<Json<BlueprintTemplateResponse>, AppError> {
    // Verify original exists
    {
        let engine = state.engine.read().await;
        engine
            .get_blueprint_template(id)
            .ok_or_else(|| AppError::NotFound(format!("Blueprint template {} not found", id)))?;
    }

    // Create new template with new ID (versioning behavior)
    let mut new_blueprint =
        ProductionLineBlueprint::new(Uuid::new_v4(), request.name, request.description);

    // Convert request production lines
    for line_request in request.production_lines {
        let recipe = recipe_by_name(&line_request.recipe).ok_or_else(|| {
            AppError::BadRequest(format!("Invalid recipe: {}", line_request.recipe))
        })?;

        let mut line = ProductionLineRecipe::new(
            Uuid::new_v4(),
            line_request.name,
            line_request.description,
            recipe,
        );

        for mg in line_request.machine_groups {
            line.add_machine_group(MachineGroup::new(
                mg.number_of_machine,
                mg.oc_value,
                mg.somersloop,
            ))
            .map_err(|e| AppError::BadRequest(e.to_string()))?;
        }

        new_blueprint.add_production_line(line);
    }

    validate_template(&new_blueprint)?;

    // Add new version to library
    let mut engine = state.engine.write().await;
    engine.add_blueprint_template(new_blueprint.clone());

    Ok(Json((&new_blueprint).into()))
}

/// DELETE /api/blueprints/templates/:id
///
/// Delete a template from the library
///
/// # Returns
///
/// - `204 No Content` on success
/// - `404 Not Found` if template doesn't exist
pub async fn delete_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let mut engine = state.engine.write().await;
    engine
        .remove_blueprint_template(id)
        .map_err(|e| AppError::NotFound(e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/blueprints/templates/import
///
/// Import a blueprint JSON to the library
///
/// # Returns
///
/// - `201 Created` with imported template
/// - `400 Bad Request` if JSON is invalid
pub async fn import_template(
    State(state): State<AppState>,
    Json(request): Json<ImportTemplateRequest>,
) -> Result<(StatusCode, Json<BlueprintTemplateResponse>), AppError> {
    // Deserialize blueprint
    let mut blueprint: ProductionLineBlueprint = serde_json::from_str(&request.blueprint_json)
        .map_err(|e| AppError::BadRequest(format!("Invalid blueprint JSON: {}", e)))?;

    // Generate new UUIDs
    blueprint.id = Uuid::new_v4();
    for line in &mut blueprint.production_lines {
        line.id = Uuid::new_v4();
    }

    // Override name if provided
    if let Some(name) = request.name {
        blueprint.name = name;
    }

    validate_template(&blueprint)?;

    // Add to library
    let mut engine = state.engine.write().await;
    engine.add_blueprint_template(blueprint.clone());

    Ok((StatusCode::CREATED, Json((&blueprint).into())))
}

/// GET /api/blueprints/templates/:id/export
///
/// Export a template as JSON
///
/// # Returns
///
/// - `200 OK` with blueprint JSON
/// - `404 Not Found` if template doesn't exist
pub async fn export_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ExportTemplateResponse>, AppError> {
    let engine = state.engine.read().await;

    let template = engine
        .get_blueprint_template(id)
        .ok_or_else(|| AppError::NotFound(format!("Blueprint template {} not found", id)))?;

    let blueprint_json =
        serde_json::to_string_pretty(template).map_err(AppError::SerializationError)?;

    let production_line = ProductionLine::ProductionLineBlueprint(template.clone());

    let metadata = TemplateMetadata {
        name: template.name.clone(),
        description: template.description.clone(),
        total_machines: production_line.total_machines(),
        total_power: production_line.total_power_consumption(),
        input_items: production_line.input_rate(),
        output_items: production_line.output_rate(),
        exported_at: Utc::now().to_rfc3339(),
    };

    Ok(Json(ExportTemplateResponse {
        blueprint_json,
        metadata,
    }))
}

/// POST /api/factories/:factory_id/production-lines/from-template/:template_id
///
/// Create a production line instance from a template in a factory
///
/// # Returns
///
/// - `201 Created` with created instance
/// - `404 Not Found` if factory or template doesn't exist
pub async fn create_from_template(
    State(state): State<AppState>,
    Path((factory_id, template_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<CreateFromTemplateRequest>,
) -> Result<(StatusCode, Json<CreateFromTemplateResponse>), AppError> {
    let mut engine = state.engine.write().await;

    // Get template
    let template = engine
        .get_blueprint_template(template_id)
        .ok_or_else(|| AppError::NotFound(format!("Blueprint template {} not found", template_id)))?
        .clone();

    // Deep clone and regenerate UUIDs
    let mut blueprint = template.clone();
    blueprint.id = Uuid::new_v4();
    for line in &mut blueprint.production_lines {
        line.id = Uuid::new_v4();
    }

    // Override name if provided
    if let Some(name) = request.name {
        blueprint.name = name;
    }

    // Get factory and add blueprint
    let factory = engine
        .get_factory_mut(factory_id)
        .ok_or_else(|| AppError::NotFound(format!("Factory {} not found", factory_id)))?;

    let blueprint_id = blueprint.id;
    factory.production_lines.insert(
        blueprint_id,
        ProductionLine::ProductionLineBlueprint(blueprint),
    );

    Ok((
        StatusCode::CREATED,
        Json(CreateFromTemplateResponse {
            message: format!("Blueprint instance created in factory {}", factory_id),
            blueprint_id,
            factory_id,
        }),
    ))
}

/// Validates a blueprint template
fn validate_template(blueprint: &ProductionLineBlueprint) -> Result<(), AppError> {
    for line in &blueprint.production_lines {
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

// Route configuration
pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/blueprints/templates",
            get(get_all_templates).post(create_template),
        )
        .route(
            "/blueprints/templates/:id",
            get(get_template)
                .put(update_template)
                .delete(delete_template),
        )
        .route("/blueprints/templates/import", post(import_template))
        .route("/blueprints/templates/:id/export", get(export_template))
        .route(
            "/factories/:factory_id/production-lines/from-template/:template_id",
            post(create_from_template),
        )
}
