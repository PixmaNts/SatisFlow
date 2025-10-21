// crates/satisflow-server/src/handlers/factory.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    error::{AppError, Result},
    state::AppState,
};
use satisflow_engine::{
    models::{production_line::ProductionLine, Item, PowerGenerator, RawInput}
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

#[derive(Serialize)]
pub struct ItemBalanceResponse {
    pub item: String,
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
    pub id: u64,
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
            item: format!("{:?}", item),
            quantity: *quantity,
        })
        .collect()
}

// Helper function to convert HashMap to Vec for nested data
fn convert_production_lines_to_response(
    production_lines: &HashMap<u64, ProductionLine>,
) -> Vec<ProductionLineResponse> {
    production_lines
        .values()
        .cloned()
        .map(|pl| ProductionLineResponse {
            production_line: pl,
        })
        .collect()
}

fn convert_raw_inputs_to_response(raw_inputs: &HashMap<u64, RawInput>) -> Vec<RawInputResponse> {
    raw_inputs
        .values()
        .cloned()
        .map(|ri| RawInputResponse { raw_input: ri })
        .collect()
}

fn convert_power_generators_to_response(
    power_generators: &HashMap<u64, PowerGenerator>,
) -> Vec<PowerGeneratorResponse> {
    power_generators
        .values()
        .cloned()
        .map(|pg| PowerGeneratorResponse {
            power_generator: pg,
        })
        .collect()
}

// API handlers
pub async fn get_factories(State(state): State<AppState>) -> Result<Json<Vec<FactoryResponse>>> {
    let engine = state.engine.read().await;
    let factories = engine.get_all_factories();

    let mut factory_responses = Vec::new();

    for (id, factory) in factories {
        // We need to get the current state of logistics lines
        let logistics_lines = engine.get_all_logistics();

        // Create a temporary factory with the same state to calculate items
        let mut temp_factory = factory.clone();
        temp_factory.calculate_item(logistics_lines);

        let response = FactoryResponse {
            id: *id,
            name: factory.name.clone(),
            description: factory.description.clone(),
            notes: None, // Factory model doesn't have notes field yet
            production_lines: convert_production_lines_to_response(&factory.production_lines),
            raw_inputs: convert_raw_inputs_to_response(&factory.raw_inputs),
            power_generators: convert_power_generators_to_response(&factory.power_generators),
            items: convert_items_to_response(&temp_factory.items),
            total_power_consumption: temp_factory.total_power_consumption(),
            total_power_generation: temp_factory.total_power_generation(),
            power_balance: temp_factory.power_balance(),
        };

        factory_responses.push(response);
    }

    Ok(Json(factory_responses))
}

pub async fn get_factory(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<FactoryResponse>> {
    let engine = state.engine.read().await;

    let factory = engine
        .get_factory(id)
        .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", id)))?;

    // Calculate items for this factory
    let logistics_lines = engine.get_all_logistics();
    let mut temp_factory = factory.clone();
    temp_factory.calculate_item(logistics_lines);

    let response = FactoryResponse {
        id,
        name: factory.name.clone(),
        description: factory.description.clone(),
        notes: None, // Factory model doesn't have notes field yet
        production_lines: convert_production_lines_to_response(&factory.production_lines),
        raw_inputs: convert_raw_inputs_to_response(&factory.raw_inputs),
        power_generators: convert_power_generators_to_response(&factory.power_generators),
        items: convert_items_to_response(&temp_factory.items),
        total_power_consumption: temp_factory.total_power_consumption(),
        total_power_generation: temp_factory.total_power_generation(),
        power_balance: temp_factory.power_balance(),
    };

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

    let factory = engine.get_factory(factory_id).unwrap(); // We just created it

    let response = FactoryResponse {
        id: factory_id,
        name: factory.name.clone(),
        description: factory.description.clone(),
        notes: None,
        production_lines: convert_production_lines_to_response(&factory.production_lines),
        raw_inputs: convert_raw_inputs_to_response(&factory.raw_inputs),
        power_generators: convert_power_generators_to_response(&factory.power_generators),
        items: convert_items_to_response(&factory.items),
        total_power_consumption: factory.total_power_consumption(),
        total_power_generation: factory.total_power_generation(),
        power_balance: factory.power_balance(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn update_factory(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(request): Json<UpdateFactoryRequest>,
) -> Result<Json<FactoryResponse>> {
    let mut engine = state.engine.write().await;

    let factory = engine
        .get_factory_mut(id)
        .ok_or_else(|| AppError::NotFound(format!("Factory with id {} not found", id)))?;

    // Update fields if provided
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

    // Notes field is not in the Factory model yet, so we skip it for now

    // Get the updated factory
    let updated_factory = engine.get_factory(id).unwrap();
    let logistics_lines = engine.get_all_logistics();
    let mut temp_factory = updated_factory.clone();
    temp_factory.calculate_item(logistics_lines);

    let response = FactoryResponse {
        id,
        name: updated_factory.name.clone(),
        description: updated_factory.description.clone(),
        notes: None,
        production_lines: convert_production_lines_to_response(&updated_factory.production_lines),
        raw_inputs: convert_raw_inputs_to_response(&updated_factory.raw_inputs),
        power_generators: convert_power_generators_to_response(&updated_factory.power_generators),
        items: convert_items_to_response(&temp_factory.items),
        total_power_consumption: temp_factory.total_power_consumption(),
        total_power_generation: temp_factory.total_power_generation(),
        power_balance: temp_factory.power_balance(),
    };

    Ok(Json(response))
}

pub async fn delete_factory(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode> {
    let mut engine = state.engine.write().await;

    engine
        .delete_factory(id)
        .map_err(|_| AppError::NotFound(format!("Factory with id {} not found", id)))?;

    Ok(StatusCode::NO_CONTENT)
}

// Route configuration
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_factories).post(create_factory))
        .route(
            "/:id",
            get(get_factory).put(update_factory).delete(delete_factory),
        )
}
