use std::{
    collections::HashMap
};

use serde::{Deserialize, Serialize};

use crate::models::{
    logistics::LogisticsFlux, power_generator::PowerGenerator, production_line::ProductionLine,
    raw_input::RawInput, Item,
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factory {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub production_lines: HashMap<u64, ProductionLine>,
    pub raw_inputs: HashMap<u64, RawInput>, // Raw resource extraction sources
    pub power_generators: HashMap<u64, PowerGenerator>, // Power generation systems
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
            raw_inputs: HashMap::new(),
            power_generators: HashMap::new(),
        }
    }

    pub fn add_production_line(&mut self, line: ProductionLine) {
        self.production_lines.insert(line.id(), line);
    }

    /// Add a raw input to this factory
    pub fn add_raw_input(&mut self, raw_input: RawInput) -> Result<(), String> {
        // Validate the raw input before adding
        raw_input.validate().map_err(|e| e.to_string())?;

        self.raw_inputs.insert(raw_input.id, raw_input);

        Ok(())
    }

    /// Remove a raw input from this factory
    pub fn remove_raw_input(&mut self, id: u64) -> Option<RawInput> {
        let removed = self.raw_inputs.remove(&id);
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

    /// Add a power generator to this factory
    pub fn add_power_generator(&mut self, generator: PowerGenerator) -> Result<(), String> {
        // Validate the generator before adding
        generator.validate().map_err(|e| e.to_string())?;

        self.power_generators.insert(generator.id, generator);
        Ok(())
    }

    /// Remove a power generator from this factory
    pub fn remove_power_generator(&mut self, id: u64) -> Option<PowerGenerator> {
        let removed = self.power_generators.remove(&id);
        removed
    }

    /// Get a reference to a power generator by ID
    pub fn get_power_generator(&self, id: u64) -> Option<&PowerGenerator> {
        self.power_generators.get(&id)
    }

    /// Get a mutable reference to a power generator by ID
    pub fn get_power_generator_mut(&mut self, id: u64) -> Option<&mut PowerGenerator> {
        self.power_generators.get_mut(&id)
    }

    /// Calculate total power generation from all power generators
    pub fn total_power_generation(&self) -> f32 {
        self.power_generators
            .values()
            .map(|generator| generator.total_power_generation())
            .sum()
    }

    /// Calculate power balance (generation - consumption)
    pub fn power_balance(&self) -> f32 {
        self.total_power_generation() - self.total_power_consumption()
    }

    pub fn total_power_consumption(&self) -> f32 {
        let production_power = self
            .production_lines
            .iter()
            .map(|line| line.1.total_power_consumption())
            .sum::<f32>();

        let raw_input_power = self
            .raw_inputs
            .values()
            .map(|raw_input| raw_input.power_consumption())
            .sum::<f32>();

        production_power + raw_input_power
    }

    pub fn calculate_item(&mut self, logistics_lines: &HashMap<u64, LogisticsFlux>) {
        self.items.clear();
        // Add all inputs from logistics input lines
        for line in logistics_lines.iter().filter(|(_k,v)| v.to_factory == self.id as u64) {
            for itemflow in &line.1.get_items() {
                *self.items.entry(itemflow.item).or_insert(0.0) += itemflow.quantity_per_min;
            }
        }
        // Subtract all outputs to logistics output lines
        for line in logistics_lines.iter().filter(|(_k,v)| v.from_factory == self.id as u64) {
            for itemflow in &line.1.get_items() {
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
        // Subtract fuel consumption from power generators and add waste production
        for generator in self.power_generators.values() {
            // Subtract fuel consumption
            let fuel_consumption = generator.total_fuel_consumption();
            if fuel_consumption > 0.0 {
                *self.items.entry(generator.fuel_type).or_insert(0.0) -= fuel_consumption;
            }
            // Add waste production (if any)
            if let Some(waste_product) = generator.waste_product() {
                let waste_rate = generator.waste_production_rate();
                if waste_rate > 0.0 {
                    *self.items.entry(waste_product).or_insert(0.0) += waste_rate;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{logistics, ExtractorType, GeneratorGroup, GeneratorType, Purity};

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

        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
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

        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
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

        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
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

        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);

        assert_eq!(factory.items.get(&Item::IronOre), Some(&120.0));

        let removed = factory.remove_raw_input(1);
        assert_eq!(removed, Some(raw_input));
        assert_eq!(factory.raw_inputs.len(), 0);
        factory.calculate_item(&logistics_lines);
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
        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
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
        
        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
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
        
        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
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
        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);

        assert_eq!(factory.raw_inputs.len(), 4);
        assert_eq!(factory.items.get(&Item::IronOre), Some(&60.0));
        assert_eq!(factory.items.get(&Item::Water), Some(&120.0));
        assert_eq!(factory.items.get(&Item::CrudeOil), Some(&120.0));
        assert_eq!(factory.items.get(&Item::NitrogenGas), Some(&60.0));
    }

    // ===== Power Generator Tests =====

    #[test]
    fn test_add_coal_power_generator() {
        let mut factory = Factory::new(1, "Power Factory".into(), None);

        let mut generator = PowerGenerator::new(1, GeneratorType::Coal, Item::Coal)
            .expect("Should create valid coal generator");
        let group = GeneratorGroup::new(4, 100.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        factory
            .add_power_generator(generator)
            .expect("Should add power generator");

        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);

        assert_eq!(factory.power_generators.len(), 1);
        assert_eq!(factory.total_power_generation(), 300.0); // 4 * 75MW
        assert_eq!(factory.power_balance(), 300.0); // No consumption yet

        // Check that fuel consumption is calculated
        assert_eq!(factory.items.get(&Item::Coal), Some(&-60.0)); // 4 * 15 items/min
    }

    #[test]
    fn test_add_fuel_power_generator_with_turbofuel() {
        let mut factory = Factory::new(1, "Advanced Power Factory".into(), None);

        let mut generator = PowerGenerator::new(1, GeneratorType::Fuel, Item::Turbofuel)
            .expect("Should create valid fuel generator");
        let group = GeneratorGroup::new(2, 150.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        factory
            .add_power_generator(generator)
            .expect("Should add power generator");

        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
        assert_eq!(factory.total_power_generation(), 450.0); // 2 * 150MW * 1.5
        assert_eq!(factory.items.get(&Item::Turbofuel), Some(&-3.375)); // 2 * 4.5 * 0.25 * 1.5
    }

    #[test]
    fn test_add_geothermal_power_generator() {
        let mut factory = Factory::new(1, "Geothermal Factory".into(), None);

        let mut generator = PowerGenerator::new_geothermal(1);
        let group = GeneratorGroup::new(3, 100.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        factory
            .add_power_generator(generator)
            .expect("Should add power generator");

        assert_eq!(factory.total_power_generation(), 600.0); // 3 * 200MW
        assert_eq!(factory.power_balance(), 600.0);

        // Geothermal doesn't consume fuel
        assert_eq!(factory.items.get(&Item::Water), None);
    }

    #[test]
    fn test_add_nuclear_power_generator() {
        let mut factory = Factory::new(1, "Nuclear Factory".into(), None);

        let mut generator = PowerGenerator::new(1, GeneratorType::Nuclear, Item::UraniumFuelRod)
            .expect("Should create valid nuclear generator");
        let group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        factory
            .add_power_generator(generator)
            .expect("Should add power generator");

        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
        assert_eq!(factory.total_power_generation(), 2500.0); // 1 * 2500MW
        assert_eq!(factory.power_balance(), 2500.0);

        // Check fuel consumption and waste production
        assert_eq!(factory.items.get(&Item::UraniumFuelRod), Some(&-0.025)); // 1 * 0.025 rods/min
        assert_eq!(factory.items.get(&Item::UraniumWaste), Some(&0.025)); // 1 * 0.025 waste/min
    }

    #[test]
    fn test_remove_power_generator() {
        let mut factory = Factory::new(1, "Power Factory".into(), None);

        let mut generator = PowerGenerator::new(1, GeneratorType::Biomass, Item::Biomass)
            .expect("Should create valid biomass generator");
        let group = GeneratorGroup::new(2, 100.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        factory
            .add_power_generator(generator.clone())
            .expect("Should add power generator");

        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
        assert_eq!(factory.power_generators.len(), 1);
        assert_eq!(factory.total_power_generation(), 60.0);
        assert_eq!(factory.items.get(&Item::Biomass), Some(&-9.0)); // 2 * 4.5

        let removed = factory.remove_power_generator(1);
        assert_eq!(removed, Some(generator));
        assert_eq!(factory.power_generators.len(), 0);
        assert_eq!(factory.total_power_generation(), 0.0);
        factory.calculate_item(&logistics_lines);
        // Items should be recalculated
        assert_eq!(factory.items.get(&Item::Biomass), None);
    }

    #[test]
    fn test_remove_nonexistent_power_generator() {
        let mut factory = Factory::new(1, "Power Factory".into(), None);

        let removed = factory.remove_power_generator(999);
        assert_eq!(removed, None);
    }

    #[test]
    fn test_get_power_generator() {
        let mut factory = Factory::new(1, "Power Factory".into(), None);

        let mut generator = PowerGenerator::new(1, GeneratorType::Coal, Item::Coal)
            .expect("Should create valid coal generator");
        let group = GeneratorGroup::new(3, 120.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        factory
            .add_power_generator(generator.clone())
            .expect("Should add power generator");

        let retrieved = factory.get_power_generator(1);
        assert_eq!(retrieved, Some(&generator));
    }

    #[test]
    fn test_get_power_generator_mut() {
        let mut factory = Factory::new(1, "Power Factory".into(), None);

        let mut generator = PowerGenerator::new(1, GeneratorType::Fuel, Item::Fuel)
            .expect("Should create valid fuel generator");
        let group = GeneratorGroup::new(2, 100.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        factory
            .add_power_generator(generator)
            .expect("Should add power generator");

        // Get mutable reference and modify
        if let Some(generator) = factory.get_power_generator_mut(1) {
            if let Some(group) = generator.get_group_mut(0) {
                group
                    .set_clock_speed(200.0)
                    .expect("Should update clock speed");
            }
        }
        // Recalculate items
        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
        assert_eq!(factory.total_power_generation(), 600.0); // 2 * 150MW * 2.0
        assert_eq!(factory.items.get(&Item::Fuel), Some(&-18.0)); // 2 * 4.5 * 2.0
    }

    #[test]
    fn test_multiple_power_generators() {
        let mut factory = Factory::new(1, "Mixed Power Factory".into(), None);

        // Add coal generators
        let mut coal_gen = PowerGenerator::new(1, GeneratorType::Coal, Item::Coal)
            .expect("Should create valid coal generator");
        let coal_group = GeneratorGroup::new(4, 100.0).expect("Should create valid group");
        coal_gen.add_group(coal_group).expect("Should add group");

        // Add fuel generators
        let mut fuel_gen = PowerGenerator::new(2, GeneratorType::Fuel, Item::Fuel)
            .expect("Should create valid fuel generator");
        let fuel_group = GeneratorGroup::new(2, 100.0).expect("Should create valid group");
        fuel_gen.add_group(fuel_group).expect("Should add group");

        factory
            .add_power_generator(coal_gen)
            .expect("Should add coal generator");
        factory
            .add_power_generator(fuel_gen)
            .expect("Should add fuel generator");

        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
        assert_eq!(factory.power_generators.len(), 2);
        assert_eq!(factory.total_power_generation(), 600.0); // 300MW (coal) + 300MW (fuel)
        assert_eq!(factory.items.get(&Item::Coal), Some(&-60.0)); // 4 * 15
        assert_eq!(factory.items.get(&Item::Fuel), Some(&-9.0)); // 2 * 4.5
    }

    #[test]
    fn test_power_balance_with_consumption() {
        let mut factory = Factory::new(1, "Balanced Factory".into(), None);

        // Add power generators
        let mut coal_gen = PowerGenerator::new(1, GeneratorType::Coal, Item::Coal)
            .expect("Should create valid coal generator");
        let coal_group = GeneratorGroup::new(2, 100.0).expect("Should create valid group");
        coal_gen.add_group(coal_group).expect("Should add group");

        factory
            .add_power_generator(coal_gen)
            .expect("Should add coal generator");

        // Add power consumption (raw input extractor)
        let iron_input = RawInput::new(
            1,
            ExtractorType::MinerMk2,
            Item::IronOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid iron input");

        factory
            .add_raw_input(iron_input)
            .expect("Should add iron input");

        assert_eq!(factory.total_power_generation(), 150.0); // 2 * 75MW
        assert_eq!(factory.total_power_consumption(), 15.0); // Miner Mk2
        assert_eq!(factory.power_balance(), 135.0); // 150 - 15
    }

    #[test]
    fn test_power_balance_deficit() {
        let mut factory = Factory::new(1, "Deficit Factory".into(), None);

        // Add only power consumption (no generation)
        let iron_input = RawInput::new(
            1,
            ExtractorType::MinerMk3,
            Item::IronOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid iron input");

        factory
            .add_raw_input(iron_input)
            .expect("Should add iron input");

        assert_eq!(factory.total_power_generation(), 0.0);
        assert_eq!(factory.total_power_consumption(), 45.0); // Miner Mk3
        assert_eq!(factory.power_balance(), -45.0); // 0 - 45 (deficit)
    }

    #[test]
    fn test_nuclear_waste_accumulation() {
        let mut factory = Factory::new(1, "Nuclear Factory".into(), None);

        let mut generator = PowerGenerator::new(1, GeneratorType::Nuclear, Item::UraniumFuelRod)
            .expect("Should create valid nuclear generator");

        // Add multiple groups to test waste accumulation
        let group1 = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        let group2 = GeneratorGroup::new(2, 200.0).expect("Should create valid group");
        generator.add_group(group1).expect("Should add group");
        generator.add_group(group2).expect("Should add group");

        factory
            .add_power_generator(generator)
            .expect("Should add nuclear generator");

        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
        // Calculate waste:
        // Group 1: 1 * 0.025 * 1.0 = 0.025
        // Group 2: 2 * 0.025 * 2.0 = 0.1
        // Total: 0.125 waste rods/min
        assert_eq!(factory.items.get(&Item::UraniumWaste), Some(&0.125));

        // Fuel consumption:
        // Group 1: 1 * 0.025 * 1.0 = 0.025
        // Group 2: 2 * 0.025 * 2.0 = 0.1
        // Total: 0.125 fuel rods/min
        assert_eq!(factory.items.get(&Item::UraniumFuelRod), Some(&-0.125));
    }

    #[test]
    fn test_factory_with_all_power_generator_types() {
        let mut factory = Factory::new(1, "All Power Types Factory".into(), None);

        // Biomass generator
        let mut biomass_gen = PowerGenerator::new(1, GeneratorType::Biomass, Item::Biomass)
            .expect("Should create valid biomass generator");
        let biomass_group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        biomass_gen
            .add_group(biomass_group)
            .expect("Should add group");

        // Coal generator
        let mut coal_gen = PowerGenerator::new(2, GeneratorType::Coal, Item::Coal)
            .expect("Should create valid coal generator");
        let coal_group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        coal_gen.add_group(coal_group).expect("Should add group");

        // Fuel generator
        let mut fuel_gen = PowerGenerator::new(3, GeneratorType::Fuel, Item::Fuel)
            .expect("Should create valid fuel generator");
        let fuel_group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        fuel_gen.add_group(fuel_group).expect("Should add group");

        // Nuclear generator
        let mut nuclear_gen = PowerGenerator::new(4, GeneratorType::Nuclear, Item::UraniumFuelRod)
            .expect("Should create valid nuclear generator");
        let nuclear_group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        nuclear_gen
            .add_group(nuclear_group)
            .expect("Should add group");

        // Geothermal generator
        let mut geo_gen = PowerGenerator::new_geothermal(5);
        let geo_group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        geo_gen.add_group(geo_group).expect("Should add group");

        // Add all generators
        factory
            .add_power_generator(biomass_gen)
            .expect("Should add biomass generator");
        factory
            .add_power_generator(coal_gen)
            .expect("Should add coal generator");
        factory
            .add_power_generator(fuel_gen)
            .expect("Should add fuel generator");
        factory
            .add_power_generator(nuclear_gen)
            .expect("Should add nuclear generator");
        factory
            .add_power_generator(geo_gen)
            .expect("Should add geothermal generator");

        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
        // Check total power generation
        let expected_power = 30.0 + 75.0 + 150.0 + 2500.0 + 200.0; // Sum of all generators
        assert_eq!(factory.total_power_generation(), expected_power);
        assert_eq!(factory.power_generators.len(), 5);

        // Check fuel consumption
        assert_eq!(factory.items.get(&Item::Biomass), Some(&-4.5));
        assert_eq!(factory.items.get(&Item::Coal), Some(&-15.0));
        assert_eq!(factory.items.get(&Item::Fuel), Some(&-4.5));
        assert_eq!(factory.items.get(&Item::UraniumFuelRod), Some(&-0.025));

        // Check waste production
        assert_eq!(factory.items.get(&Item::UraniumWaste), Some(&0.025));

        // Geothermal shouldn't consume any fuel
        assert_eq!(factory.items.get(&Item::Water), None);
    }

    #[test]
    fn test_add_invalid_power_generator() {
        let mut factory = Factory::new(1, "Test Factory".into(), None);

        // Try to add a generator with no groups (invalid)
        let generator = PowerGenerator::new(1, GeneratorType::Coal, Item::Coal)
            .expect("Should create valid coal generator");

        let result = factory.add_power_generator(generator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must have at least one group"));
    }

    #[test]
    fn test_add_power_generator_triggers_recalculation() {
        let mut factory = Factory::new(1, "Test Factory".into(), None);

        // Add some raw input first
        let iron_input = RawInput::new(
            1,
            ExtractorType::MinerMk1,
            Item::IronOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid iron input");

        factory
            .add_raw_input(iron_input)
            .expect("Should add iron input");

        let logistics_lines = HashMap::new();
        factory.calculate_item(&logistics_lines);
        assert_eq!(factory.items.get(&Item::IronOre), Some(&60.0));

        // Add power generator (should trigger recalculation)
        let mut generator = PowerGenerator::new(1, GeneratorType::Coal, Item::Coal)
            .expect("Should create valid coal generator");
        let group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        factory
            .add_power_generator(generator)
            .expect("Should add power generator");
        factory.calculate_item(&logistics_lines);
        // Both iron ore and coal should be in items
        assert_eq!(factory.items.get(&Item::IronOre), Some(&60.0));
        assert_eq!(factory.items.get(&Item::Coal), Some(&-15.0));
    }
}
