// crates/satisflow-server/src/handlers/game_data.rs
use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;

use crate::{error::Result, state::AppState};
use satisflow_engine::models::game_data::MachineType;
use satisflow_engine::models::{all_items, all_recipes};

#[derive(Serialize)]
pub struct RecipeInfo {
    pub name: String,
    pub machine: String,
    pub inputs: Vec<ItemQuantity>,
    pub outputs: Vec<ItemQuantity>,
}

#[derive(Serialize)]
pub struct ItemQuantity {
    pub item: String,
    pub quantity: f32,
}

#[derive(Serialize)]
pub struct MachineInfo {
    pub name: String,
    pub base_power: f32,
    pub max_somersloop: u8,
}

pub async fn get_recipes(State(_state): State<AppState>) -> Result<Json<Vec<RecipeInfo>>> {
    let recipes: Vec<RecipeInfo> = all_recipes()
        .iter()
        .map(|details| RecipeInfo {
            name: details.name.to_string(),
            machine: format!("{:?}", details.machine),
            inputs: details
                .inputs
                .iter()
                .map(|(item, qty)| ItemQuantity {
                    item: format!("{:?}", item),
                    quantity: *qty,
                })
                .collect(),
            outputs: details
                .outputs
                .iter()
                .map(|(item, qty)| ItemQuantity {
                    item: format!("{:?}", item),
                    quantity: *qty,
                })
                .collect(),
        })
        .collect();

    Ok(Json(recipes))
}

pub async fn get_items(State(_state): State<AppState>) -> Result<Json<Vec<String>>> {
    let items: Vec<String> = all_items()
        .iter()
        .map(|(item, _)| format!("{:?}", item)) // Serialize enum variant name, not display name
        .collect();

    Ok(Json(items))
}

pub async fn get_machines(State(_state): State<AppState>) -> Result<Json<Vec<MachineInfo>>> {
    let machines: Vec<MachineInfo> = [
        MachineType::Constructor,
        MachineType::Smelter,
        MachineType::Assembler,
        MachineType::Manufacturer,
        MachineType::Refinery,
        MachineType::Blender,
        MachineType::Packager,
        MachineType::ParticleAccelerator,
        MachineType::Foundry,
    ]
    .iter()
    .map(|machine| MachineInfo {
        name: format!("{:?}", machine),
        base_power: machine.base_power_mw(),
        max_somersloop: machine.max_somersloop(),
    })
    .collect();

    Ok(Json(machines))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/recipes", get(get_recipes))
        .route("/items", get(get_items))
        .route("/machines", get(get_machines))
}
