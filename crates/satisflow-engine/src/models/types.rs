use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Strongly typed identifiers (newtype pattern)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecipeId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FactoryId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProductionLineId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LogisticsFluxId(pub String);

// Explicit enums for stable game concepts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MachineType {
    Constructor,
    Assembler,
    Manufacturer,
    Smelter,
    Foundry,
    Refinery,
    Blender,
    Packager,
    ParticleAccelerator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameEvent {
    Ficsmas,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Availability {
    Always,
    RequiresEvent(GameEvent),
}

impl MachineType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "constructor" => Some(MachineType::Constructor),
            "assembler" => Some(MachineType::Assembler),
            "manufacturer" => Some(MachineType::Manufacturer),
            "smelter" => Some(MachineType::Smelter),
            "foundry" => Some(MachineType::Foundry),
            "refinery" => Some(MachineType::Refinery),
            "blender" => Some(MachineType::Blender),
            "packager" => Some(MachineType::Packager),
            "particle accelerator" => Some(MachineType::ParticleAccelerator),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemCategory {
    RawMaterial,
    Fluid,
    Ingot,
    Part,
    AdvancedPart,
    ProcessedMaterial,
    Item,
}

impl ItemCategory {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "raw material" => Some(ItemCategory::RawMaterial),
            "fluid" => Some(ItemCategory::Fluid),
            "ingot" => Some(ItemCategory::Ingot),
            "part" => Some(ItemCategory::Part),
            "advanced part" => Some(ItemCategory::AdvancedPart),
            "processed material" => Some(ItemCategory::ProcessedMaterial),
            "item" => Some(ItemCategory::Item),
            _ => None,
        }
    }
}

// Utility: create stable slugs from human names
pub fn slugify(name: &str) -> String {
    let mut s = name.trim().to_lowercase();
    s = s
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect::<String>();
    while s.contains("  ") {
        s = s.replace("  ", " ");
    }
    s.replace(' ', "-")
}

// Internal domain models (typed, stable IDs)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainItem {
    pub id: ItemId,
    pub name: String,
    pub category: ItemCategory,
    pub description: String,
    pub availability: Availability,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainRecipeIngredient {
    pub item_id: ItemId,
    pub quantity_per_min: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainRecipe {
    pub id: RecipeId,
    pub name: String,
    pub machine_type: MachineType,
    pub base_duration: f32,
    pub inputs: Vec<DomainRecipeIngredient>,
    pub outputs: Vec<DomainRecipeIngredient>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GameIndex {
    pub items_by_id: HashMap<ItemId, DomainItem>,
    pub item_id_by_name: HashMap<String, ItemId>,
    pub item_name_by_id: HashMap<ItemId, String>,

    pub recipes_by_id: HashMap<RecipeId, DomainRecipe>,
    pub recipe_id_by_name: HashMap<String, RecipeId>,
    pub recipe_name_by_id: HashMap<RecipeId, String>,
}
