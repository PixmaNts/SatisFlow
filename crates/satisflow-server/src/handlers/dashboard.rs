// crates/satisflow-server/src/handlers/dashboard.rs
use axum::{
    extract::State,
    Json,
    Router,
    routing::get,
};
use serde::Serialize;

use crate::{error::Result, state::AppState};

#[derive(Serialize)]
pub struct DashboardSummary {
    pub total_factories: usize,
    pub total_production_lines: usize,
    pub total_logistics_lines: usize,
    pub total_power_consumption: f32,
    pub total_power_generation: f32,
    pub net_power: f32,
}

#[derive(Serialize)]
pub struct ItemBalance {
    pub item: String,
    pub balance: f32,
    pub state: String,
}

pub async fn get_summary(
    State(_state): State<AppState>,
) -> Result<Json<DashboardSummary>> {
    // TODO: Implement dashboard summary
    Ok(Json(DashboardSummary {
        total_factories: 0,
        total_production_lines: 0,
        total_logistics_lines: 0,
        total_power_consumption: 0.0,
        total_power_generation: 0.0,
        net_power: 0.0,
    }))
}

pub async fn get_item_balances(
    State(_state): State<AppState>,
) -> Result<Json<Vec<ItemBalance>>> {
    // TODO: Implement item balances
    Ok(Json(vec![]))
}

pub async fn get_power_statistics(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>> {
    // TODO: Implement power statistics
    Ok(Json(serde_json::json!({})))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/summary", get(get_summary))
        .route("/items", get(get_item_balances))
        .route("/power", get(get_power_statistics))
}