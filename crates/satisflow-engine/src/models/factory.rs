use super::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProductionOutput {
    ToProductionLine(ProductionLineId),
    ToLogistics(LogisticsFluxId),
    Stored,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductionLine {
    pub id: ProductionLineId,
    pub factory_id: FactoryId,
    pub recipe_id: RecipeId,
    pub machine_count: u32,
    pub clock_ratio: f32,
    pub group_name: Option<String>,
    pub output_routing: HashMap<ItemId, ProductionOutput>,
    // Optional Strange Matter boosters: number of machines in this line with active booster
    // Each boosted machine doubles its output (inputs unaffected)
    #[serde(default)]
    pub strange_matter_boosted: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RawInput {
    pub id: RawInputId,
    pub item: ItemId,
    pub quantity_per_min: f32,
    pub source_type: String,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogisticsInput {
    pub item: ItemId,
    pub quantity_per_min: f32,
    pub from_factory: Option<FactoryId>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Factory {
    pub id: FactoryId,
    pub name: String,
    pub raw_inputs: Vec<RawInput>,
    pub logistics_inputs: Vec<LogisticsInput>,
    pub production_lines: Vec<ProductionLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProductionStatus {
    Balanced,
    Overflow,
    Underflow,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemSummary {
    pub item_id: ItemId,
    pub item_name: String,
    pub total_produced_per_min: f32,
    pub total_consumed_per_min: f32,
    pub available_per_min: f32,
    pub status: ProductionStatus,
}
