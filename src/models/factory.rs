use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProductionOutput {
    ToProductionLine(String),
    ToLogistics(String),
    Stored,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductionLine {
    pub id: String,
    pub factory_id: String,
    pub recipe_name: String,
    pub machine_count: u32,
    pub clock_ratio: f32,
    pub group_name: Option<String>,
    pub output_routing: HashMap<String, ProductionOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RawInput {
    pub item: String,
    pub quantity_per_min: f32,
    pub source_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Factory {
    pub id: String,
    pub name: String,
    pub raw_inputs: Vec<RawInput>,
    pub logistics_inputs: Vec<String>,
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
    pub item_name: String,
    pub total_produced_per_min: f32,
    pub total_consumed_per_min: f32,
    pub available_per_min: f32,
    pub status: ProductionStatus,
}