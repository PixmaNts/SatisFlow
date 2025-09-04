use crate::models::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionTracker {
    pub factories: HashMap<String, Factory>,
    pub logistics_fluxes: HashMap<String, LogisticsFlux>,
    pub recipes: HashMap<String, Recipe>,
    pub items: HashMap<String, Item>,
}

impl ProductionTracker {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
            logistics_fluxes: HashMap::new(),
            recipes: HashMap::new(),
            items: HashMap::new(),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn load_game_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.items = crate::data::load_items().await?;
        self.recipes = crate::data::load_recipes().await?;
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_game_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.items = crate::data::load_items()?;
        self.recipes = crate::data::load_recipes()?;
        Ok(())
    }

    pub fn calculate_overview(&self) -> Vec<ItemSummary> {
        let mut item_production: HashMap<String, f32> = HashMap::new();
        let mut item_consumption: HashMap<String, f32> = HashMap::new();
        
        for factory in self.factories.values() {
            for raw_input in &factory.raw_inputs {
                *item_production.entry(raw_input.item.clone()).or_insert(0.0) += raw_input.quantity_per_min;
            }
            
            for production_line in &factory.production_lines {
                if let Some(recipe) = self.recipes.get(&production_line.recipe_name) {
                    let production_multiplier = production_line.machine_count as f32 * production_line.clock_ratio;
                    
                    for output in &recipe.outputs {
                        let actual_quantity = output.quantity_per_min * production_multiplier;
                        *item_production.entry(output.item.clone()).or_insert(0.0) += actual_quantity;
                    }
                    
                    for input in &recipe.inputs {
                        let actual_quantity = input.quantity_per_min * production_multiplier;
                        *item_consumption.entry(input.item.clone()).or_insert(0.0) += actual_quantity;
                    }
                }
            }
        }
        
        let mut summaries = Vec::new();
        let mut all_items: std::collections::HashSet<String> = std::collections::HashSet::new();
        all_items.extend(item_production.keys().cloned());
        all_items.extend(item_consumption.keys().cloned());
        
        for item_name in all_items {
            let produced = item_production.get(&item_name).copied().unwrap_or(0.0);
            let consumed = item_consumption.get(&item_name).copied().unwrap_or(0.0);
            let available = produced - consumed;
            
            let status = if available.abs() < 0.1 {
                ProductionStatus::Balanced
            } else if available > 0.0 {
                ProductionStatus::Overflow
            } else {
                ProductionStatus::Underflow
            };
            
            summaries.push(ItemSummary {
                item_name,
                total_produced_per_min: produced,
                total_consumed_per_min: consumed,
                available_per_min: available,
                status,
            });
        }
        
        summaries.sort_by(|a, b| a.item_name.cmp(&b.item_name));
        summaries
    }

    pub fn add_factory(&mut self, factory: Factory) {
        self.factories.insert(factory.id.clone(), factory);
    }

    pub fn add_logistics_flux(&mut self, flux: LogisticsFlux) {
        self.logistics_fluxes.insert(flux.id.clone(), flux);
    }
}