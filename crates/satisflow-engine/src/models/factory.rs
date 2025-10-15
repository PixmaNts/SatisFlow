use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::models::{
    logistics::LogisticsFlux, production_line::ProductionLine, raw_input::RawInput, Item,
};

pub struct Factory {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub production_lines: HashMap<u64, Box<dyn ProductionLine>>,

    pub logistics_output: HashMap<u64, Arc<Mutex<LogisticsFlux>>>,
    pub logistics_input: HashMap<u64, Arc<Mutex<LogisticsFlux>>>,

    pub raw_inputs: HashMap<u64, RawInput>, // Raw resource extraction sources
    pub items: HashMap<Item, f32>,          // Inventory of items in the factory
}

impl Factory {
    pub fn new(id: u32, name: String, description: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
            production_lines: HashMap::new(),
            items: HashMap::new(),
            logistics_output: HashMap::new(),
            logistics_input: HashMap::new(),
            raw_inputs: HashMap::new(),
        }
    }

    pub fn add_production_line(&mut self, line: Box<dyn ProductionLine>) {
        self.production_lines.insert(line.id(), line);
        self.calculate_item();
    }

    /// Add a raw input to this factory
    pub fn add_raw_input(&mut self, raw_input: RawInput) -> Result<(), String> {
        // Validate the raw input before adding
        raw_input.validate().map_err(|e| e.to_string())?;

        self.raw_inputs.insert(raw_input.id, raw_input);
        self.calculate_item();
        Ok(())
    }

    /// Remove a raw input from this factory
    pub fn remove_raw_input(&mut self, id: u64) -> Option<RawInput> {
        let removed = self.raw_inputs.remove(&id);
        if removed.is_some() {
            self.calculate_item();
        }
        removed
    }

    /// Get a reference to a raw input by ID
    pub fn get_raw_input(&self, id: u64) -> Option<&RawInput> {
        self.raw_inputs.get(&id)
    }

    /// Get a mutable reference to a raw input by ID
    pub fn get_raw_input_mut(&mut self, id: u64) -> Option<&mut RawInput> {
        self.raw_inputs.get_mut(&id)
    }

    pub fn total_power_consumption(&self) -> f32 {
        self.production_lines
            .iter()
            .map(|line| line.1.total_power_consumption())
            .sum()
    }

    pub fn calculate_item(&mut self) {
        self.items.clear();
        // Add all inputs from logistics input lines
        for line in self.logistics_input.values() {
            let line = line.lock().unwrap();
            for itemflow in &line.get_items() {
                *self.items.entry(itemflow.item).or_insert(0.0) += itemflow.quantity_per_min;
            }
        }
        // Subtract all outputs to logistics output lines
        for line in self.logistics_output.values() {
            let line = line.lock().unwrap();
            for itemflow in &line.get_items() {
                *self.items.entry(itemflow.item).or_insert(0.0) -= itemflow.quantity_per_min;
            }
        }
        // Add all raw inputs from extraction sources
        for raw_input in self.raw_inputs.values() {
            *self.items.entry(raw_input.item).or_insert(0.0) += raw_input.quantity_per_min;
        }
        // Add all production line outputs and subtract inputs
        for line in self.production_lines.values() {
            for (item, qty) in line.output_rate() {
                *self.items.entry(item).or_insert(0.0) += qty;
            }
            for (item, qty) in line.input_rate() {
                *self.items.entry(item).or_insert(0.0) -= qty;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ExtractorType, Purity};

    #[test]
    fn test_add_raw_input() {
        let mut factory = Factory::new(1, "Test Factory".into(), None);

        let raw_input = RawInput::new(
            1,
            ExtractorType::MinerMk2,
            Item::IronOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid raw input");

        factory
            .add_raw_input(raw_input.clone())
            .expect("Should add raw input");

        assert_eq!(factory.raw_inputs.len(), 1);
        assert_eq!(factory.get_raw_input(1), Some(&raw_input));

        // Check that items were calculated
        assert_eq!(factory.items.get(&Item::IronOre), Some(&120.0));
    }

    #[test]
    fn test_add_multiple_raw_inputs() {
        let mut factory = Factory::new(1, "Test Factory".into(), None);

        let iron_input = RawInput::new(
            1,
            ExtractorType::MinerMk3,
            Item::IronOre,
            Some(Purity::Pure),
        )
        .expect("Should create valid raw input");

        let copper_input = RawInput::new(
            2,
            ExtractorType::MinerMk2,
            Item::CopperOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid raw input");

        factory
            .add_raw_input(iron_input)
            .expect("Should add iron input");
        factory
            .add_raw_input(copper_input)
            .expect("Should add copper input");

        assert_eq!(factory.raw_inputs.len(), 2);
        assert_eq!(factory.items.get(&Item::IronOre), Some(&480.0)); // Mk3 Pure = 240 * 2.0
        assert_eq!(factory.items.get(&Item::CopperOre), Some(&120.0)); // Mk2 Normal = 120 * 1.0
    }

    #[test]
    fn test_add_water_extractor() {
        let mut factory = Factory::new(1, "Test Factory".into(), None);

        let water_input = RawInput::new(1, ExtractorType::WaterExtractor, Item::Water, None)
            .expect("Should create valid water input");

        factory
            .add_raw_input(water_input)
            .expect("Should add water input");

        assert_eq!(factory.items.get(&Item::Water), Some(&120.0));
    }

    #[test]
    fn test_remove_raw_input() {
        let mut factory = Factory::new(1, "Test Factory".into(), None);

        let raw_input = RawInput::new(
            1,
            ExtractorType::MinerMk2,
            Item::IronOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid raw input");

        factory
            .add_raw_input(raw_input.clone())
            .expect("Should add raw input");
        assert_eq!(factory.items.get(&Item::IronOre), Some(&120.0));

        let removed = factory.remove_raw_input(1);
        assert_eq!(removed, Some(raw_input));
        assert_eq!(factory.raw_inputs.len(), 0);

        // Items should be recalculated and iron should be gone
        assert_eq!(factory.items.get(&Item::IronOre), None);
    }

    #[test]
    fn test_remove_nonexistent_raw_input() {
        let mut factory = Factory::new(1, "Test Factory".into(), None);

        let removed = factory.remove_raw_input(999);
        assert_eq!(removed, None);
    }

    #[test]
    fn test_get_raw_input() {
        let mut factory = Factory::new(1, "Test Factory".into(), None);

        let raw_input = RawInput::new(1, ExtractorType::MinerMk3, Item::Coal, Some(Purity::Impure))
            .expect("Should create valid raw input");

        factory
            .add_raw_input(raw_input.clone())
            .expect("Should add raw input");

        let retrieved = factory.get_raw_input(1);
        assert_eq!(retrieved, Some(&raw_input));
    }

    #[test]
    fn test_get_raw_input_mut() {
        let mut factory = Factory::new(1, "Test Factory".into(), None);

        let raw_input = RawInput::new(
            1,
            ExtractorType::MinerMk2,
            Item::IronOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid raw input");

        factory
            .add_raw_input(raw_input)
            .expect("Should add raw input");

        // Get mutable reference and modify
        if let Some(input) = factory.get_raw_input_mut(1) {
            input.quantity_per_min = 150.0; // Manual override for testing
        }

        factory.calculate_item();
        assert_eq!(factory.items.get(&Item::IronOre), Some(&150.0));
    }

    #[test]
    fn test_calculate_item_with_raw_inputs() {
        let mut factory = Factory::new(1, "Test Factory".into(), None);

        // Add multiple raw inputs
        factory
            .add_raw_input(
                RawInput::new(
                    1,
                    ExtractorType::MinerMk1,
                    Item::IronOre,
                    Some(Purity::Normal),
                )
                .expect("valid"),
            )
            .expect("Should add");

        factory
            .add_raw_input(
                RawInput::new(
                    2,
                    ExtractorType::MinerMk1,
                    Item::IronOre,
                    Some(Purity::Pure),
                )
                .expect("valid"),
            )
            .expect("Should add");

        // Should have 60 + 120 = 180 iron ore per minute
        assert_eq!(factory.items.get(&Item::IronOre), Some(&180.0));
    }

    #[test]
    fn test_oil_extractor_with_different_purities() {
        let mut factory = Factory::new(1, "Oil Factory".into(), None);

        factory
            .add_raw_input(
                RawInput::new(
                    1,
                    ExtractorType::OilExtractor,
                    Item::CrudeOil,
                    Some(Purity::Impure),
                )
                .expect("valid"),
            )
            .expect("Should add");

        factory
            .add_raw_input(
                RawInput::new(
                    2,
                    ExtractorType::OilExtractor,
                    Item::CrudeOil,
                    Some(Purity::Pure),
                )
                .expect("valid"),
            )
            .expect("Should add");

        // Impure: 60, Pure: 240, Total: 300
        assert_eq!(factory.items.get(&Item::CrudeOil), Some(&300.0));
    }

    #[test]
    fn test_all_extractor_types() {
        let mut factory = Factory::new(1, "Test Factory".into(), None);

        // Add one of each extractor type
        factory
            .add_raw_input(
                RawInput::new(
                    1,
                    ExtractorType::MinerMk1,
                    Item::IronOre,
                    Some(Purity::Normal),
                )
                .expect("valid"),
            )
            .expect("Should add");

        factory
            .add_raw_input(
                RawInput::new(2, ExtractorType::WaterExtractor, Item::Water, None).expect("valid"),
            )
            .expect("Should add");

        factory
            .add_raw_input(
                RawInput::new(
                    3,
                    ExtractorType::OilExtractor,
                    Item::CrudeOil,
                    Some(Purity::Normal),
                )
                .expect("valid"),
            )
            .expect("Should add");

        // Resource Well Extractors need a pressurizer and extractors
        use crate::models::raw_input::{ResourceWellExtractor, ResourceWellPressurizer};
        let pressurizer =
            ResourceWellPressurizer::new(10, 100.0).expect("Should create pressurizer");
        let extractors = vec![ResourceWellExtractor::new(11, Purity::Normal)];
        let resource_well =
            RawInput::new_resource_well(4, Item::NitrogenGas, pressurizer, extractors)
                .expect("Should create valid resource well system");

        factory.add_raw_input(resource_well).expect("Should add");

        assert_eq!(factory.raw_inputs.len(), 4);
        assert_eq!(factory.items.get(&Item::IronOre), Some(&60.0));
        assert_eq!(factory.items.get(&Item::Water), Some(&120.0));
        assert_eq!(factory.items.get(&Item::CrudeOil), Some(&120.0));
        assert_eq!(factory.items.get(&Item::NitrogenGas), Some(&60.0));
    }
}
