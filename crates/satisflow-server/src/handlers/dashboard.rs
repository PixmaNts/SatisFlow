// crates/satisflow-server/src/handlers/dashboard.rs
use axum::{extract::State, routing::get, Json, Router};
use satisflow_engine::models::{power_generator::GeneratorType, Item};
use serde::Serialize;
use uuid::Uuid;

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
    pub item: Item,
    pub balance: f32,
    pub state: String,
}

#[derive(Serialize)]
pub struct FactoryPowerStatsResponse {
    pub factory_id: Uuid,
    pub factory_name: String,
    pub generation: f32,
    pub consumption: f32,
    pub balance: f32,
    pub generator_count: u32,
    pub generator_types: Vec<GeneratorType>,
}

#[derive(Serialize)]
pub struct PowerStatisticsResponse {
    pub total_generation: f32,
    pub total_consumption: f32,
    pub power_balance: f32,
    pub has_surplus: bool,
    pub has_deficit: bool,
    pub is_balanced: bool,
    pub factory_stats: Vec<FactoryPowerStatsResponse>,
}

pub async fn get_summary(State(state): State<AppState>) -> Result<Json<DashboardSummary>> {
    let mut engine = state.engine.write().await;

    // Update all factories to get current calculations
    let _global_items = engine.update();

    let factories = engine.get_all_factories();
    let logistics_lines = engine.get_all_logistics();

    // Calculate totals
    let total_factories = factories.len();
    let total_logistics_lines = logistics_lines.len();

    let mut total_production_lines = 0;
    let mut total_power_consumption = 0.0;
    let mut total_power_generation = 0.0;

    for factory in factories.values() {
        total_production_lines += factory.production_lines.len();
        total_power_consumption += factory.total_power_consumption();
        total_power_generation += factory.total_power_generation();
    }

    let net_power = total_power_generation - total_power_consumption;

    Ok(Json(DashboardSummary {
        total_factories,
        total_production_lines,
        total_logistics_lines,
        total_power_consumption,
        total_power_generation,
        net_power,
    }))
}

pub async fn get_item_balances(State(state): State<AppState>) -> Result<Json<Vec<ItemBalance>>> {
    let mut engine = state.engine.write().await;

    // Update all factories to get current calculations
    let global_items = engine.update();

    let mut item_balances = Vec::new();

    for (item, balance) in global_items {
        let state = if balance > 0.0 {
            "overflow".to_string()
        } else if balance < 0.0 {
            "underflow".to_string()
        } else {
            "balanced".to_string()
        };

        item_balances.push(ItemBalance {
            item,
            balance,
            state,
        });
    }

    // Sort by item name for consistent ordering
    item_balances.sort_by(|a, b| format!("{:?}", a.item).cmp(&format!("{:?}", b.item)));

    Ok(Json(item_balances))
}

pub async fn get_power_statistics(
    State(state): State<AppState>,
) -> Result<Json<PowerStatisticsResponse>> {
    let engine = state.engine.read().await;

    // Get power statistics from the engine
    let power_stats = engine.global_power_stats();

    // Convert factory stats to response format
    let factory_stats: Vec<FactoryPowerStatsResponse> = power_stats
        .factory_stats
        .iter()
        .map(|stat| FactoryPowerStatsResponse {
            factory_id: stat.factory_id,
            factory_name: stat.factory_name.clone(),
            generation: stat.generation,
            consumption: stat.consumption,
            balance: stat.balance,
            generator_count: stat.generator_count,
            generator_types: stat.generator_types.clone(),
        })
        .collect();

    let response = PowerStatisticsResponse {
        total_generation: power_stats.total_generation,
        total_consumption: power_stats.total_consumption,
        power_balance: power_stats.power_balance,
        has_surplus: power_stats.has_surplus(),
        has_deficit: power_stats.has_deficit(),
        is_balanced: power_stats.is_balanced(),
        factory_stats,
    };

    Ok(Json(response))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/summary", get(get_summary))
        .route("/items", get(get_item_balances))
        .route("/power", get(get_power_statistics))
}
