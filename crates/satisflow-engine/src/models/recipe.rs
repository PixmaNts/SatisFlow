use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub name: String,
    pub category: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecipeIngredient {
    pub item: String,
    pub quantity_per_min: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub machine_type: String,
    pub base_duration: f32,
    pub inputs: Vec<RecipeIngredient>,
    pub outputs: Vec<RecipeIngredient>,
}
