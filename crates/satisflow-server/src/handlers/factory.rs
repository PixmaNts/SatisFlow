// crates/satisflow-server/src/handlers/factory.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    state::AppState,
};
use satisflow_engine::models::recipes::recipe_by_name;
use satisflow_engine::models::{
    factory::Factory,
    logistics::LogisticsFlux,
    power_generator::{GeneratorGroup as EngineGeneratorGroup, GeneratorType, PowerGenerator},
    production_line::{
        MachineGroup as EngineMachineGroup, ProductionLine, ProductionLineBlueprint,
        ProductionLineRecipe,
    },
    raw_input::{ExtractorType, Purity, RawInput, ResourceWellExtractor, ResourceWellPressurizer},
    Item,
};

// DTOs for API requests/responses
#[derive(Serialize, Deserialize)]
pub struct CreateFactoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateFactoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ProductionLineType {
    Recipe,
    Blueprint,
}

#[derive(Deserialize, Clone)]
pub struct MachineGroupPayload {
    pub number_of_machine: u32,
    pub oc_value: f32,
    pub somersloop: u8,
}

#[derive(Deserialize, Clone)]
pub struct BlueprintSubLinePayload {
    pub name: String,
    pub description: Option<String>,
    pub recipe: String,
    #[serde(default)]
    pub machine_groups: Vec<MachineGroupPayload>,
}

#[derive(Deserialize, Clone)]
pub struct ProductionLinePayload {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub line_type: ProductionLineType,
    pub recipe: Option<String>,
    #[serde(default)]
    pub machine_groups: Vec<MachineGroupPayload>,
    #[serde(default)]
    pub production_lines: Vec<BlueprintSubLinePayload>,
}

#[derive(Deserialize, Clone)]
pub struct RawInputPressurizerPayload {
    #[serde(default)]
    pub id: Option<u64>,
    pub clock_speed: f32,
}

#[derive(Deserialize, Clone)]
pub struct RawInputExtractorPayload {
    #[serde(default)]
    pub id: Option<u64>,
    pub purity: Purity,
    #[serde(default)]
    pub _item: Option<Item>,
    #[serde(default)]
    pub _quantity_per_min: Option<f32>,
}

#[derive(Deserialize, Clone)]
pub struct RawInputPayload {
    pub extractor_type: ExtractorType,
    pub item: Item,
    pub purity: Option<Purity>,
    #[serde(default)]
    pub quantity_per_min: f32,
    #[serde(default)]
    pub pressurizer: Option<RawInputPressurizerPayload>,
    #[serde(default)]
    pub extractors: Vec<RawInputExtractorPayload>,
}

#[derive(Deserialize, Clone)]
pub struct GeneratorGroupPayload {
    pub number_of_generators: u32,
    pub clock_speed: f32,
}

#[derive(Deserialize, Clone)]
pub struct PowerGeneratorPayload {
    pub generator_type: GeneratorType,
    pub fuel_type: Option<Item>,
    #[serde(default)]
    pub groups: Vec<GeneratorGroupPayload>,
}

#[derive(Serialize)]
pub struct ItemBalanceResponse {
    pub item: Item,
    pub quantity: f32,
}

#[derive(Serialize)]
pub struct ProductionLineResponse {
    #[serde(flatten)]
    pub production_line: ProductionLine,
}

#[derive(Serialize)]
pub struct RawInputResponse {
    #[serde(flatten)]
    pub raw_input: RawInput,
}

#[derive(Serialize)]
pub struct PowerGeneratorResponse {
    #[serde(flatten)]
    pub power_generator: PowerGenerator,
}

#[derive(Serialize)]
pub struct FactoryResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub production_lines: Vec<ProductionLineResponse>,
    pub raw_inputs: Vec<RawInputResponse>,
    pub power_generators: Vec<PowerGeneratorResponse>,
    pub items: Vec<ItemBalanceResponse>,
    pub total_power_consumption: f32,
    pub total_power_generation: f32,
    pub power_balance: f32,
}

// Helper function to convert HashMap<Item, f32> to Vec<ItemBalanceResponse>
fn convert_items_to_response(items: &HashMap<Item, f32>) -> Vec<ItemBalanceResponse> {
    items
        .iter()
        .map(|(item, quantity)| ItemBalanceResponse {
            item: *item,
            quantity: *quantity,
        })
        .collect()
}

// Helper function to convert HashMap to Vec for nested data
fn convert_production_lines_to_response(
    production_lines: &HashMap<Uuid, ProductionLine>,
) -> Vec<ProductionLineResponse> {
    production_lines
        .values()
        .cloned()
        .map(|pl| ProductionLineResponse {
            production_line: pl,
        })
        .collect()
}

fn convert_raw_inputs_to_response(raw_inputs: &HashMap<Uuid, RawInput>) -> Vec<RawInputResponse> {
    raw_inputs
        .values()
        .cloned()
        .map(|ri| RawInputResponse { raw_input: ri })
        .collect()
}

fn convert_power_generators_to_response(
    power_generators: &HashMap<Uuid, PowerGenerator>,
) -> Vec<PowerGeneratorResponse> {
    power_generators
        .values()
        .cloned()
        .map(|pg| PowerGeneratorResponse {
            power_generator: pg,
        })
        .collect()
}

fn build_factory_response(
    factory: &Factory,
    logistics: &HashMap<Uuid, LogisticsFlux>,
) -> FactoryResponse {
    let mut temp_factory = factory.clone();
    temp_factory.calculate_item(logistics);

    FactoryResponse {
        id: factory.id,
        name: factory.name.clone(),
        description: factory.description.clone(),
        notes: factory.notes.clone(),
        production_lines: convert_production_lines_to_response(&factory.production_lines),
        raw_inputs: convert_raw_inputs_to_response(&factory.raw_inputs),
        power_generators: convert_power_generators_to_response(&factory.power_generators),
        items: convert_items_to_response(&temp_factory.items),
        total_power_consumption: temp_factory.total_power_consumption(),
        total_power_generation: temp_factory.total_power_generation(),
        power_balance: temp_factory.power_balance(),
    }
}

fn build_recipe_line_from_payload(
    payload: &ProductionLinePayload,
    line_id: Uuid,
) -> Result<ProductionLineRecipe> {
    let recipe_name = payload.recipe.as_ref().ok_or_else(|| {
        AppError::BadRequest("Recipe name is required for recipe lines".to_string())
    })?;

    let recipe = recipe_by_name(recipe_name)
        .ok_or_else(|| AppError::BadRequest(format!("Unknown recipe: {}", recipe_name)))?;

    if payload.machine_groups.is_empty() {
        return Err(AppError::BadRequest(
            "At least one machine group is required".to_string(),
        ));
    }

    let mut line = ProductionLineRecipe::new(
        line_id,
        payload.name.clone(),
        payload.description.clone(),
        recipe,
    );

    for group in &payload.machine_groups {
        let machine_group =
            EngineMachineGroup::new(group.number_of_machine, group.oc_value, group.somersloop);
        line.add_machine_group(machine_group)
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
    }

    Ok(line)
}

fn build_blueprint_line_from_payload(
    payload: &ProductionLinePayload,
    line_id: Uuid,
) -> Result<ProductionLine> {
    if payload.production_lines.is_empty() {
        return Err(AppError::BadRequest(
            "Blueprint production line requires at least one recipe".to_string(),
        ));
    }

    let mut blueprint =
        ProductionLineBlueprint::new(line_id, payload.name.clone(), payload.description.clone());

    for sub_line in &payload.production_lines {
        let recipe = recipe_by_name(&sub_line.recipe)
            .ok_or_else(|| AppError::BadRequest(format!("Unknown recipe: {}", sub_line.recipe)))?;

        if sub_line.machine_groups.is_empty() {
            return Err(AppError::BadRequest(format!(
                "Blueprint recipe {} requires at least one machine group",
                sub_line.name
            )));
        }

        let mut blueprint_line = ProductionLineRecipe::new(
            Uuid::new_v4(),
            sub_line.name.clone(),
            sub_line.description.clone(),
            recipe,
        );

        for group in &sub_line.machine_groups {
            let machine_group =
                EngineMachineGroup::new(group.number_of_machine, group.oc_value, group.somersloop);
            blueprint_line
                .add_machine_group(machine_group)
                .map_err(|e| AppError::ValidationError(e.to_string()))?;
        }

        blueprint.add_production_line(blueprint_line);
    }

    Ok(ProductionLine::ProductionLineBlueprint(blueprint))
}

fn build_production_line_from_payload(
    payload: &ProductionLinePayload,
    id: Option<Uuid>,
) -> Result<ProductionLine> {
    let line_id = id.unwrap_or_else(Uuid::new_v4);

    match payload.line_type {
        ProductionLineType::Recipe => {
            let recipe_line = build_recipe_line_from_payload(payload, line_id)?;
            Ok(ProductionLine::ProductionLineRecipe(recipe_line))
        }
        ProductionLineType::Blueprint => build_blueprint_line_from_payload(payload, line_id),
    }
}

fn build_raw_input_from_payload(payload: &RawInputPayload, id: Option<Uuid>) -> Result<RawInput> {
    let raw_input_id = id.unwrap_or_else(Uuid::new_v4);

    let mut raw_input = if payload.extractor_type == ExtractorType::ResourceWellExtractor {
        let pressurizer_payload = payload.pressurizer.as_ref().ok_or_else(|| {
            AppError::BadRequest("Resource well extractors require a pressurizer".to_string())
        })?;

        if payload.extractors.is_empty() {
            return Err(AppError::BadRequest(
                "Resource well extractors require at least one extractor".to_string(),
            ));
        }

        let pressurizer_id = pressurizer_payload.id.unwrap_or(1);
        let pressurizer =
            ResourceWellPressurizer::new(pressurizer_id, pressurizer_payload.clock_speed)
                .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let extractors: Vec<ResourceWellExtractor> = payload
            .extractors
            .iter()
            .enumerate()
            .map(|(index, extractor)| {
                let extractor_id = extractor.id.unwrap_or((index + 1) as u64);
                ResourceWellExtractor::new(extractor_id, extractor.purity)
            })
            .collect();

        RawInput::new_resource_well(raw_input_id, payload.item, pressurizer, extractors)
            .map_err(|e| AppError::ValidationError(e.to_string()))?
    } else {
        RawInput::new(
            raw_input_id,
            payload.extractor_type,
            payload.item,
            payload.purity,
        )
        .map_err(|e| AppError::ValidationError(e.to_string()))?
    };

    if payload.quantity_per_min > 0.0 {
        raw_input.quantity_per_min = payload.quantity_per_min;
    }

    Ok(raw_input)
}

fn build_power_generator_from_payload(
    payload: &PowerGeneratorPayload,
    id: Option<Uuid>,
) -> Result<PowerGenerator> {
    let generator_id = id.unwrap_or_else(Uuid::new_v4);

    let mut generator = match payload.generator_type {
        GeneratorType::Geothermal => PowerGenerator::new_geothermal(generator_id),
        _ => {
            let fuel = payload.fuel_type.ok_or_else(|| {
                AppError::BadRequest("Fuel type is required for this generator".to_string())
            })?;
            PowerGenerator::new(generator_id, payload.generator_type, fuel)
                .map_err(|e| AppError::ValidationError(e.to_string()))?
        }
    };

    if payload.groups.is_empty() {
        return Err(AppError::BadRequest(
            "At least one generator group is required".to_string(),
        ));
    }

    for group in &payload.groups {
        let generator_group =
            EngineGeneratorGroup::new(group.number_of_generators, group.clock_speed)
                .map_err(|e| AppError::ValidationError(e.to_string()))?;
        generator
            .add_group(generator_group)
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
    }

    Ok(generator)
}

// API handlers
pub async fn get_factories(State(state): State<AppState>) -> Result<Json<Vec<FactoryResponse>>> {
    let engine = state.engine.read().await;
    let factories = engine.get_all_factories();
    let logistics_lines = engine.get_all_logistics();

    let responses = factories
        .values()
        .map(|factory| build_factory_response(factory, logistics_lines))
        .collect();

    Ok(Json(responses))
}

pub async fn get_factory(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<FactoryResponse>> {
    let engine = state.engine.read().await;

    let factory = engine
        .get_factory(id)
        .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", id)))?;

    let response = build_factory_response(factory, engine.get_all_logistics());

    Ok(Json(response))
}

pub async fn create_factory(
    State(state): State<AppState>,
    Json(request): Json<CreateFactoryRequest>,
) -> Result<(StatusCode, Json<FactoryResponse>)> {
    let mut engine = state.engine.write().await;

    // Validate factory name is not empty
    if request.name.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Factory name cannot be empty".to_string(),
        ));
    }

    let factory_id = engine.create_factory(request.name.clone(), request.description.clone());

    if let Some(factory) = engine.get_factory_mut(factory_id) {
        factory.notes = match request.notes.clone() {
            Some(notes) if notes.trim().is_empty() => None,
            other => other,
        };
    }

    let factory = engine.get_factory(factory_id).unwrap(); // We just created it

    let response = build_factory_response(factory, engine.get_all_logistics());

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn update_factory(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateFactoryRequest>,
) -> Result<Json<FactoryResponse>> {
    let mut engine = state.engine.write().await;

    {
        let factory = engine
            .get_factory_mut(id)
            .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", id)))?;

        if let Some(name) = request.name {
            if name.trim().is_empty() {
                return Err(AppError::BadRequest(
                    "Factory name cannot be empty".to_string(),
                ));
            }
            factory.name = name;
        }

        if let Some(description) = request.description {
            factory.description = Some(description);
        }

        if let Some(notes) = request.notes {
            if notes.trim().is_empty() {
                factory.notes = None;
            } else {
                factory.notes = Some(notes);
            }
        }
    }

    let updated_factory = engine
        .get_factory(id)
        .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", id)))?;

    let response = build_factory_response(updated_factory, engine.get_all_logistics());

    Ok(Json(response))
}

pub async fn delete_factory(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    let mut engine = state.engine.write().await;

    engine
        .delete_factory(id)
        .map_err(|_| AppError::NotFound(format!("Factory with id {} not found", id)))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_production_line(
    State(state): State<AppState>,
    Path(factory_id): Path<Uuid>,
    Json(payload): Json<ProductionLinePayload>,
) -> Result<(StatusCode, Json<FactoryResponse>)> {
    let mut engine = state.engine.write().await;

    let production_line = build_production_line_from_payload(&payload, None)?;

    {
        let factory = engine.get_factory_mut(factory_id).ok_or_else(|| {
            AppError::NotFound(format!("Factory with id {} not found", factory_id))
        })?;
        factory.add_production_line(production_line);
    }

    let factory = engine
        .get_factory(factory_id)
        .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", factory_id)))?;

    let response = build_factory_response(factory, engine.get_all_logistics());

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn update_production_line(
    State(state): State<AppState>,
    Path((factory_id, line_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<ProductionLinePayload>,
) -> Result<Json<FactoryResponse>> {
    let mut engine = state.engine.write().await;

    let production_line = build_production_line_from_payload(&payload, Some(line_id))?;

    {
        let factory = engine.get_factory_mut(factory_id).ok_or_else(|| {
            AppError::NotFound(format!("Factory with id {} not found", factory_id))
        })?;

        if factory
            .production_lines
            .insert(line_id, production_line)
            .is_none()
        {
            return Err(AppError::NotFound(format!(
                "Production line with id {} not found",
                line_id
            )));
        }
    }

    let factory = engine
        .get_factory(factory_id)
        .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", factory_id)))?;

    let response = build_factory_response(factory, engine.get_all_logistics());

    Ok(Json(response))
}

pub async fn delete_production_line(
    State(state): State<AppState>,
    Path((factory_id, line_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<FactoryResponse>> {
    let mut engine = state.engine.write().await;

    {
        let factory = engine.get_factory_mut(factory_id).ok_or_else(|| {
            AppError::NotFound(format!("Factory with id {} not found", factory_id))
        })?;

        if factory.production_lines.remove(&line_id).is_none() {
            return Err(AppError::NotFound(format!(
                "Production line with id {} not found",
                line_id
            )));
        }
    }

    let factory = engine
        .get_factory(factory_id)
        .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", factory_id)))?;

    let response = build_factory_response(factory, engine.get_all_logistics());

    Ok(Json(response))
}

pub async fn create_raw_input(
    State(state): State<AppState>,
    Path(factory_id): Path<Uuid>,
    Json(payload): Json<RawInputPayload>,
) -> Result<(StatusCode, Json<FactoryResponse>)> {
    let mut engine = state.engine.write().await;

    let raw_input = build_raw_input_from_payload(&payload, None)?;

    {
        let factory = engine.get_factory_mut(factory_id).ok_or_else(|| {
            AppError::NotFound(format!("Factory with id {} not found", factory_id))
        })?;
        factory
            .add_raw_input(raw_input)
            .map_err(|e| AppError::ValidationError(e))?;
    }

    let factory = engine
        .get_factory(factory_id)
        .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", factory_id)))?;

    let response = build_factory_response(factory, engine.get_all_logistics());

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn update_raw_input(
    State(state): State<AppState>,
    Path((factory_id, raw_input_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<RawInputPayload>,
) -> Result<Json<FactoryResponse>> {
    let mut engine = state.engine.write().await;

    let raw_input = build_raw_input_from_payload(&payload, Some(raw_input_id))?;

    {
        let factory = engine.get_factory_mut(factory_id).ok_or_else(|| {
            AppError::NotFound(format!("Factory with id {} not found", factory_id))
        })?;

        if factory.raw_inputs.insert(raw_input_id, raw_input).is_none() {
            return Err(AppError::NotFound(format!(
                "Raw input with id {} not found",
                raw_input_id
            )));
        }
    }

    let factory = engine
        .get_factory(factory_id)
        .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", factory_id)))?;

    let response = build_factory_response(factory, engine.get_all_logistics());

    Ok(Json(response))
}

pub async fn delete_raw_input(
    State(state): State<AppState>,
    Path((factory_id, raw_input_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<FactoryResponse>> {
    let mut engine = state.engine.write().await;

    {
        let factory = engine.get_factory_mut(factory_id).ok_or_else(|| {
            AppError::NotFound(format!("Factory with id {} not found", factory_id))
        })?;

        if factory.remove_raw_input(raw_input_id).is_none() {
            return Err(AppError::NotFound(format!(
                "Raw input with id {} not found",
                raw_input_id
            )));
        }
    }

    let factory = engine
        .get_factory(factory_id)
        .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", factory_id)))?;

    let response = build_factory_response(factory, engine.get_all_logistics());

    Ok(Json(response))
}

pub async fn create_power_generator(
    State(state): State<AppState>,
    Path(factory_id): Path<Uuid>,
    Json(payload): Json<PowerGeneratorPayload>,
) -> Result<(StatusCode, Json<FactoryResponse>)> {
    let mut engine = state.engine.write().await;

    let generator = build_power_generator_from_payload(&payload, None)?;

    {
        let factory = engine.get_factory_mut(factory_id).ok_or_else(|| {
            AppError::NotFound(format!("Factory with id {} not found", factory_id))
        })?;
        factory
            .add_power_generator(generator)
            .map_err(|e| AppError::ValidationError(e))?;
    }

    let factory = engine
        .get_factory(factory_id)
        .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", factory_id)))?;

    let response = build_factory_response(factory, engine.get_all_logistics());

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn update_power_generator(
    State(state): State<AppState>,
    Path((factory_id, generator_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<PowerGeneratorPayload>,
) -> Result<Json<FactoryResponse>> {
    let mut engine = state.engine.write().await;

    let generator = build_power_generator_from_payload(&payload, Some(generator_id))?;

    {
        let factory = engine.get_factory_mut(factory_id).ok_or_else(|| {
            AppError::NotFound(format!("Factory with id {} not found", factory_id))
        })?;

        if factory
            .power_generators
            .insert(generator_id, generator)
            .is_none()
        {
            return Err(AppError::NotFound(format!(
                "Power generator with id {} not found",
                generator_id
            )));
        }
    }

    let factory = engine
        .get_factory(factory_id)
        .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", factory_id)))?;

    let response = build_factory_response(factory, engine.get_all_logistics());

    Ok(Json(response))
}

pub async fn delete_power_generator(
    State(state): State<AppState>,
    Path((factory_id, generator_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<FactoryResponse>> {
    let mut engine = state.engine.write().await;

    {
        let factory = engine.get_factory_mut(factory_id).ok_or_else(|| {
            AppError::NotFound(format!("Factory with id {} not found", factory_id))
        })?;

        if factory.remove_power_generator(generator_id).is_none() {
            return Err(AppError::NotFound(format!(
                "Power generator with id {} not found",
                generator_id
            )));
        }
    }

    let factory = engine
        .get_factory(factory_id)
        .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", factory_id)))?;

    let response = build_factory_response(factory, engine.get_all_logistics());

    Ok(Json(response))
}

// Route configuration
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_factories).post(create_factory))
        .route(
            "/:id",
            get(get_factory).put(update_factory).delete(delete_factory),
        )
        .route("/:id/production-lines", post(create_production_line))
        .route(
            "/:id/production-lines/:line_id",
            put(update_production_line).delete(delete_production_line),
        )
        .route("/:id/raw-inputs", post(create_raw_input))
        .route(
            "/:id/raw-inputs/:raw_input_id",
            put(update_raw_input).delete(delete_raw_input),
        )
        .route("/:id/power-generators", post(create_power_generator))
        .route(
            "/:id/power-generators/:generator_id",
            put(update_power_generator).delete(delete_power_generator),
        )
}


