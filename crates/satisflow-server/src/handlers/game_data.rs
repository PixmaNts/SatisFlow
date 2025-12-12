// crates/satisflow-server/src/handlers/game_data.rs
use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;

use crate::{error::Result, state::AppState};
use satisflow_engine::models::game_data::MachineType;
use satisflow_engine::models::raw_input::ExtractorType;
use satisflow_engine::models::{all_items, all_recipes, Item};

#[derive(Serialize)]
pub struct RecipeInfo {
    pub name: String,
    pub machine: MachineType,
    pub inputs: Vec<ItemQuantity>,
    pub outputs: Vec<ItemQuantity>,
}

#[derive(Serialize)]
pub struct ItemQuantity {
    pub item: Item,
    pub quantity: f32,
}

#[derive(Serialize)]
pub struct MachineInfo {
    pub name: MachineType,
    pub base_power: f32,
    pub max_somersloop: u8,
}

pub async fn get_recipes(State(_state): State<AppState>) -> Result<Json<Vec<RecipeInfo>>> {
    let recipes: Vec<RecipeInfo> = all_recipes()
        .iter()
        .map(|details| RecipeInfo {
            name: details.name.to_string(),
            machine: details.machine,
            inputs: details
                .inputs
                .iter()
                .map(|(item, qty)| ItemQuantity {
                    item: *item,
                    quantity: *qty,
                })
                .collect(),
            outputs: details
                .outputs
                .iter()
                .map(|(item, qty)| ItemQuantity {
                    item: *item,
                    quantity: *qty,
                })
                .collect(),
        })
        .collect();

    Ok(Json(recipes))
}

pub async fn get_items(State(_state): State<AppState>) -> Result<Json<Vec<Item>>> {
    let items: Vec<Item> = all_items().iter().map(|(item, _)| *item).collect();

    Ok(Json(items))
}

pub async fn get_machines(State(_state): State<AppState>) -> Result<Json<Vec<MachineInfo>>> {
    let machines: Vec<MachineInfo> = [
        MachineType::Constructor,
        MachineType::Assembler,
        MachineType::Manufacturer,
        MachineType::Smelter,
        MachineType::Foundry,
        MachineType::Refinery,
        MachineType::Blender,
        MachineType::Packager,
        MachineType::ParticleAccelerator,
        MachineType::Manual,
    ]
    .iter()
    .map(|machine| MachineInfo {
        name: *machine,
        base_power: machine.base_power_mw(),
        max_somersloop: machine.max_somersloop(),
    })
    .collect();

    Ok(Json(machines))
}

#[derive(Serialize)]
pub struct ExtractorCompatibleItemsResponse {
    pub extractor_type: ExtractorType,
    pub compatible_items: Vec<Item>,
}

pub async fn get_extractor_compatible_items(
    State(_state): State<AppState>,
) -> Result<Json<Vec<ExtractorCompatibleItemsResponse>>> {
    let extractor_types = vec![
        ExtractorType::MinerMk1,
        ExtractorType::MinerMk2,
        ExtractorType::MinerMk3,
        ExtractorType::WaterExtractor,
        ExtractorType::OilExtractor,
        ExtractorType::ResourceWellExtractor,
    ];

    let responses: Vec<ExtractorCompatibleItemsResponse> = extractor_types
        .iter()
        .map(|extractor_type| ExtractorCompatibleItemsResponse {
            extractor_type: *extractor_type,
            compatible_items: extractor_type.compatible_items(),
        })
        .collect();

    Ok(Json(responses))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/recipes", get(get_recipes))
        .route("/items", get(get_items))
        .route("/machines", get(get_machines))
        .route("/extractor-compatible-items", get(get_extractor_compatible_items))
}
