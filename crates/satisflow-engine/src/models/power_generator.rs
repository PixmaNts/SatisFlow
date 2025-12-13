use serde::{Deserialize, Serialize};
use std::fmt;

use crate::models::{FactoryId, Item, PowerGeneratorId};

/// Types of power generators available in Satisfactory
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum GeneratorType {
    Biomass,
    Coal,
    Fuel,
    Nuclear,
    Geothermal,
}

impl GeneratorType {
    /// Get the base power output for this generator type in MW at 100% clock speed
    pub fn base_power_output(&self) -> f32 {
        match self {
            GeneratorType::Biomass => 30.0,     // Biomass Burner
            GeneratorType::Coal => 75.0,        // Coal Generator
            GeneratorType::Fuel => 150.0,       // Fuel Generator
            GeneratorType::Nuclear => 2500.0,   // Nuclear Power Plant
            GeneratorType::Geothermal => 200.0, // Geothermal Generator
        }
    }

    /// Get the base fuel consumption for this generator type at 100% clock speed
    /// Returns items/min for solid fuels or m³/min for liquid fuels
    pub fn base_fuel_consumption(&self) -> f32 {
        match self {
            GeneratorType::Biomass => 4.5,    // Biomass items/min
            GeneratorType::Coal => 15.0,      // Coal items/min
            GeneratorType::Fuel => 4.5,       // Fuel m³/min
            GeneratorType::Nuclear => 0.025,  // Uranium Fuel Rods/min (1 every 40 minutes)
            GeneratorType::Geothermal => 0.0, // No fuel consumption
        }
    }

    /// Check if this generator type is compatible with the given fuel type
    pub fn is_compatible_with(&self, fuel: &Item) -> bool {
        match self {
            GeneratorType::Biomass => {
                matches!(
                    fuel,
                    Item::Biomass
                        | Item::Leaves
                        | Item::Wood
                        | Item::Mycelia
                        | Item::FlowerPetals
                        | Item::BaconAgaric
                        | Item::Paleberry
                )
            }
            GeneratorType::Coal => {
                matches!(fuel, Item::Coal | Item::CompactedCoal | Item::PetroleumCoke)
            }
            GeneratorType::Fuel => {
                matches!(fuel, Item::Fuel | Item::Turbofuel | Item::LiquidBiofuel)
            }
            GeneratorType::Nuclear => matches!(fuel, Item::UraniumFuelRod),
            GeneratorType::Geothermal => false, // Geothermal doesn't use fuel
        }
    }

    /// Get the fuel consumption multiplier for this generator type
    /// This allows different fuels to have different efficiency rates
    pub fn fuel_consumption_multiplier(&self, fuel: &Item) -> f32 {
        match self {
            GeneratorType::Biomass => 1.0, // All biomass types have same efficiency
            GeneratorType::Coal => match fuel {
                Item::Coal => 1.0,
                Item::CompactedCoal => 0.8, // More efficient
                Item::PetroleumCoke => 1.2, // Less efficient
                _ => 1.0,
            },
            GeneratorType::Fuel => match fuel {
                Item::Fuel => 1.0,
                Item::Turbofuel => 0.25,     // 4x more efficient
                Item::LiquidBiofuel => 1.33, // Less efficient
                _ => 1.0,
            },
            GeneratorType::Nuclear => 1.0, // Nuclear fuel rods have fixed efficiency
            GeneratorType::Geothermal => 0.0, // No fuel consumption
        }
    }

    /// Check if this generator type produces waste products
    pub fn produces_waste(&self) -> bool {
        matches!(self, GeneratorType::Nuclear)
    }

    /// Get the waste product type for this generator (if any)
    pub fn waste_product(&self) -> Option<Item> {
        match self {
            GeneratorType::Nuclear => Some(Item::UraniumWaste),
            _ => None,
        }
    }

    /// Get the waste production rate at 100% clock speed (items/min)
    pub fn base_waste_production(&self) -> f32 {
        match self {
            GeneratorType::Nuclear => 0.025, // 1 waste rod every 40 minutes
            _ => 0.0,
        }
    }
}

/// A group of identical power generators with the same clock speed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratorGroup {
    pub number_of_generators: u32,
    pub clock_speed: f32, // 0.000 to 250.000
}

impl GeneratorGroup {
    /// Create a new generator group with validation
    pub fn new(number_of_generators: u32, clock_speed: f32) -> Result<Self, PowerGeneratorError> {
        if number_of_generators == 0 {
            return Err(PowerGeneratorError::InvalidGeneratorCount {
                count: number_of_generators,
            });
        }

        if !(0.0..=250.0).contains(&clock_speed) {
            return Err(PowerGeneratorError::InvalidClockSpeed { clock_speed });
        }

        Ok(Self {
            number_of_generators,
            clock_speed,
        })
    }

    /// Calculate power generation for this group
    /// Power generation scales linearly with clock speed
    pub fn power_generation(&self, base_power: f32) -> f32 {
        base_power * (self.clock_speed / 100.0) * self.number_of_generators as f32
    }

    /// Calculate fuel consumption for this group
    /// Fuel consumption scales linearly with clock speed (constant efficiency)
    pub fn fuel_consumption(&self, base_consumption: f32, fuel_multiplier: f32) -> f32 {
        base_consumption
            * fuel_multiplier
            * (self.clock_speed / 100.0)
            * self.number_of_generators as f32
    }

    /// Calculate waste production for this group (if applicable)
    /// Waste production scales linearly with clock speed
    pub fn waste_production(&self, base_waste: f32) -> f32 {
        base_waste * (self.clock_speed / 100.0) * self.number_of_generators as f32
    }

    /// Set the clock speed of this generator group
    pub fn set_clock_speed(&mut self, clock_speed: f32) -> Result<(), PowerGeneratorError> {
        if !(0.0..=250.0).contains(&clock_speed) {
            return Err(PowerGeneratorError::InvalidClockSpeed { clock_speed });
        }
        self.clock_speed = clock_speed;
        Ok(())
    }

    /// Set the number of generators in this group
    pub fn set_number_of_generators(&mut self, count: u32) -> Result<(), PowerGeneratorError> {
        if count == 0 {
            return Err(PowerGeneratorError::InvalidGeneratorCount { count });
        }
        self.number_of_generators = count;
        Ok(())
    }
}

/// Represents a power generator system in a factory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PowerGenerator {
    pub id: PowerGeneratorId,
    pub generator_type: GeneratorType,
    pub fuel_type: Item,
    pub groups: Vec<GeneratorGroup>,
}

impl PowerGenerator {
    /// Create a new power generator with validation
    pub fn new(
        id: PowerGeneratorId,
        generator_type: GeneratorType,
        fuel_type: Item,
    ) -> Result<Self, PowerGeneratorError> {
        // Validate fuel compatibility
        if !generator_type.is_compatible_with(&fuel_type) {
            return Err(PowerGeneratorError::IncompatibleFuel {
                generator: generator_type,
                fuel: fuel_type,
            });
        }

        // Geothermal generators don't use fuel
        if generator_type == GeneratorType::Geothermal {
            return Err(PowerGeneratorError::GeothermalUsesFuel);
        }

        Ok(Self {
            id,
            generator_type,
            fuel_type,
            groups: Vec::new(),
        })
    }

    /// Create a new geothermal generator (special case that doesn't use fuel)
    pub fn new_geothermal(id: PowerGeneratorId) -> Self {
        Self {
            id,
            generator_type: GeneratorType::Geothermal,
            fuel_type: Item::Water, // Placeholder, not actually used
            groups: Vec::new(),
        }
    }

    /// Add a generator group to this power generator
    pub fn add_group(&mut self, group: GeneratorGroup) -> Result<(), PowerGeneratorError> {
        self.groups.push(group);
        Ok(())
    }

    /// Remove a generator group by index
    pub fn remove_group(&mut self, index: usize) -> Result<GeneratorGroup, PowerGeneratorError> {
        if index >= self.groups.len() {
            return Err(PowerGeneratorError::GroupNotFound { index });
        }
        Ok(self.groups.remove(index))
    }

    /// Get a reference to a generator group by index
    pub fn get_group(&self, index: usize) -> Option<&GeneratorGroup> {
        self.groups.get(index)
    }

    /// Get a mutable reference to a generator group by index
    pub fn get_group_mut(&mut self, index: usize) -> Option<&mut GeneratorGroup> {
        self.groups.get_mut(index)
    }

    /// Calculate total power generation from all groups
    pub fn total_power_generation(&self) -> f32 {
        let base_power = self.generator_type.base_power_output();
        self.groups
            .iter()
            .map(|group| group.power_generation(base_power))
            .sum()
    }

    /// Calculate total fuel consumption from all groups
    pub fn total_fuel_consumption(&self) -> f32 {
        if self.generator_type == GeneratorType::Geothermal {
            return 0.0;
        }

        let base_consumption = self.generator_type.base_fuel_consumption();
        let fuel_multiplier = self
            .generator_type
            .fuel_consumption_multiplier(&self.fuel_type);
        self.groups
            .iter()
            .map(|group| group.fuel_consumption(base_consumption, fuel_multiplier))
            .sum()
    }

    /// Calculate total waste production from all groups (if applicable)
    pub fn waste_production_rate(&self) -> f32 {
        if !self.generator_type.produces_waste() {
            return 0.0;
        }

        let base_waste = self.generator_type.base_waste_production();
        self.groups
            .iter()
            .map(|group| group.waste_production(base_waste))
            .sum()
    }

    /// Get the waste product type (if any)
    pub fn waste_product(&self) -> Option<Item> {
        self.generator_type.waste_product()
    }

    /// Validate that this power generator configuration is correct
    pub fn validate(&self) -> Result<(), PowerGeneratorError> {
        // Check fuel compatibility (except for geothermal)
        if self.generator_type != GeneratorType::Geothermal
            && !self.generator_type.is_compatible_with(&self.fuel_type)
        {
            return Err(PowerGeneratorError::IncompatibleFuel {
                generator: self.generator_type,
                fuel: self.fuel_type,
            });
        }

        // Check that there's at least one group
        if self.groups.is_empty() {
            return Err(PowerGeneratorError::NoGroups);
        }

        // Validate each group
        for group in self.groups.iter() {
            if group.number_of_generators == 0 {
                return Err(PowerGeneratorError::InvalidGeneratorCount {
                    count: group.number_of_generators,
                });
            }
            if !(0.0..=250.0).contains(&group.clock_speed) {
                return Err(PowerGeneratorError::InvalidClockSpeed {
                    clock_speed: group.clock_speed,
                });
            }
        }

        Ok(())
    }
}

/// Errors that can occur when working with power generators
#[derive(Debug, Clone, PartialEq)]
pub enum PowerGeneratorError {
    IncompatibleFuel {
        generator: GeneratorType,
        fuel: Item,
    },
    InvalidClockSpeed {
        clock_speed: f32,
    },
    InvalidGeneratorCount {
        count: u32,
    },
    NoGroups,
    GroupNotFound {
        index: usize,
    },
    GeothermalUsesFuel,
}

impl fmt::Display for PowerGeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PowerGeneratorError::IncompatibleFuel { generator, fuel } => {
                write!(f, "Generator {:?} cannot use fuel {:?}", generator, fuel)
            }
            PowerGeneratorError::InvalidClockSpeed { clock_speed } => {
                write!(
                    f,
                    "Clock speed {} is invalid. Must be between 0.000 and 250.000",
                    clock_speed
                )
            }
            PowerGeneratorError::InvalidGeneratorCount { count } => {
                write!(
                    f,
                    "Generator count {} is invalid. Must be greater than 0",
                    count
                )
            }
            PowerGeneratorError::NoGroups => {
                write!(f, "Power generator must have at least one group")
            }
            PowerGeneratorError::GroupNotFound { index } => {
                write!(f, "Generator group at index {} not found", index)
            }
            PowerGeneratorError::GeothermalUsesFuel => {
                write!(f, "Geothermal generators do not use fuel")
            }
        }
    }
}

impl std::error::Error for PowerGeneratorError {}

/// Global power statistics for the entire Satisflow system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PowerStats {
    pub total_generation: f32,
    pub total_consumption: f32,
    pub power_balance: f32,
    pub factory_stats: Vec<FactoryPowerStats>,
}

/// Power statistics for a single factory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactoryPowerStats {
    pub factory_id: FactoryId,
    pub factory_name: String,
    pub generation: f32,
    pub consumption: f32,
    pub balance: f32,
    pub generator_count: u32,
    pub generator_types: Vec<GeneratorType>,
}

impl PowerStats {
    /// Create new power statistics
    pub fn new(
        total_generation: f32,
        total_consumption: f32,
        factory_stats: Vec<FactoryPowerStats>,
    ) -> Self {
        let power_balance = total_generation - total_consumption;
        Self {
            total_generation,
            total_consumption,
            power_balance,
            factory_stats,
        }
    }

    /// Check if the system has a power surplus
    pub fn has_surplus(&self) -> bool {
        self.power_balance > 0.0
    }

    /// Check if the system has a power deficit
    pub fn has_deficit(&self) -> bool {
        self.power_balance < 0.0
    }

    /// Check if the system is power balanced
    pub fn is_balanced(&self) -> bool {
        (self.power_balance - 0.0).abs() < f32::EPSILON
    }
}

impl FactoryPowerStats {
    /// Create new factory power statistics
    pub fn new(
        factory_id: FactoryId,
        factory_name: String,
        generation: f32,
        consumption: f32,
        generator_count: u32,
        generator_types: Vec<GeneratorType>,
    ) -> Self {
        let balance = generation - consumption;
        Self {
            factory_id,
            factory_name,
            generation,
            consumption,
            balance,
            generator_count,
            generator_types,
        }
    }

    /// Check if this factory has a power surplus
    pub fn has_surplus(&self) -> bool {
        self.balance > 0.0
    }

    /// Check if this factory has a power deficit
    pub fn has_deficit(&self) -> bool {
        self.balance < 0.0
    }

    /// Check if this factory is power balanced
    pub fn is_balanced(&self) -> bool {
        (self.balance - 0.0).abs() < f32::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn uuid_from_u64(value: u64) -> Uuid {
        Uuid::from_u128(value as u128)
    }

    // ===== GeneratorType Tests =====

    #[test]
    fn test_generator_base_power_output() {
        assert_eq!(GeneratorType::Biomass.base_power_output(), 30.0);
        assert_eq!(GeneratorType::Coal.base_power_output(), 75.0);
        assert_eq!(GeneratorType::Fuel.base_power_output(), 150.0);
        assert_eq!(GeneratorType::Nuclear.base_power_output(), 2500.0);
        assert_eq!(GeneratorType::Geothermal.base_power_output(), 200.0);
    }

    #[test]
    fn test_generator_base_fuel_consumption() {
        assert_eq!(GeneratorType::Biomass.base_fuel_consumption(), 4.5);
        assert_eq!(GeneratorType::Coal.base_fuel_consumption(), 15.0);
        assert_eq!(GeneratorType::Fuel.base_fuel_consumption(), 4.5);
        assert_eq!(GeneratorType::Nuclear.base_fuel_consumption(), 0.025);
        assert_eq!(GeneratorType::Geothermal.base_fuel_consumption(), 0.0);
    }

    #[test]
    fn test_biomass_fuel_compatibility() {
        assert!(GeneratorType::Biomass.is_compatible_with(&Item::Biomass));
        assert!(GeneratorType::Biomass.is_compatible_with(&Item::Leaves));
        assert!(GeneratorType::Biomass.is_compatible_with(&Item::Wood));
        assert!(GeneratorType::Biomass.is_compatible_with(&Item::Mycelia));
        assert!(GeneratorType::Biomass.is_compatible_with(&Item::FlowerPetals));
        assert!(GeneratorType::Biomass.is_compatible_with(&Item::BaconAgaric));
        assert!(GeneratorType::Biomass.is_compatible_with(&Item::Paleberry));
        assert!(!GeneratorType::Biomass.is_compatible_with(&Item::Coal));
        assert!(!GeneratorType::Biomass.is_compatible_with(&Item::Fuel));
    }

    #[test]
    fn test_coal_fuel_compatibility() {
        assert!(GeneratorType::Coal.is_compatible_with(&Item::Coal));
        assert!(GeneratorType::Coal.is_compatible_with(&Item::CompactedCoal));
        assert!(GeneratorType::Coal.is_compatible_with(&Item::PetroleumCoke));
        assert!(!GeneratorType::Coal.is_compatible_with(&Item::Biomass));
        assert!(!GeneratorType::Coal.is_compatible_with(&Item::Fuel));
    }

    #[test]
    fn test_fuel_fuel_compatibility() {
        assert!(GeneratorType::Fuel.is_compatible_with(&Item::Fuel));
        assert!(GeneratorType::Fuel.is_compatible_with(&Item::Turbofuel));
        assert!(GeneratorType::Fuel.is_compatible_with(&Item::LiquidBiofuel));
        assert!(!GeneratorType::Fuel.is_compatible_with(&Item::Coal));
        assert!(!GeneratorType::Fuel.is_compatible_with(&Item::Biomass));
    }

    #[test]
    fn test_nuclear_fuel_compatibility() {
        assert!(GeneratorType::Nuclear.is_compatible_with(&Item::UraniumFuelRod));
        assert!(!GeneratorType::Nuclear.is_compatible_with(&Item::Coal));
        assert!(!GeneratorType::Nuclear.is_compatible_with(&Item::Fuel));
    }

    #[test]
    fn test_geothermal_fuel_compatibility() {
        // Geothermal doesn't use any fuel
        assert!(!GeneratorType::Geothermal.is_compatible_with(&Item::Coal));
        assert!(!GeneratorType::Geothermal.is_compatible_with(&Item::Fuel));
        assert!(!GeneratorType::Geothermal.is_compatible_with(&Item::Biomass));
    }

    #[test]
    fn test_fuel_consumption_multipliers() {
        // Coal generators
        assert_eq!(
            GeneratorType::Coal.fuel_consumption_multiplier(&Item::Coal),
            1.0
        );
        assert_eq!(
            GeneratorType::Coal.fuel_consumption_multiplier(&Item::CompactedCoal),
            0.8
        );
        assert_eq!(
            GeneratorType::Coal.fuel_consumption_multiplier(&Item::PetroleumCoke),
            1.2
        );

        // Fuel generators
        assert_eq!(
            GeneratorType::Fuel.fuel_consumption_multiplier(&Item::Fuel),
            1.0
        );
        assert_eq!(
            GeneratorType::Fuel.fuel_consumption_multiplier(&Item::Turbofuel),
            0.25
        );
        assert_eq!(
            GeneratorType::Fuel.fuel_consumption_multiplier(&Item::LiquidBiofuel),
            1.33
        );

        // Biomass generators (all same efficiency)
        assert_eq!(
            GeneratorType::Biomass.fuel_consumption_multiplier(&Item::Biomass),
            1.0
        );
        assert_eq!(
            GeneratorType::Biomass.fuel_consumption_multiplier(&Item::Wood),
            1.0
        );
    }

    #[test]
    fn test_waste_production() {
        assert!(GeneratorType::Nuclear.produces_waste());
        assert!(!GeneratorType::Biomass.produces_waste());
        assert!(!GeneratorType::Coal.produces_waste());
        assert!(!GeneratorType::Fuel.produces_waste());
        assert!(!GeneratorType::Geothermal.produces_waste());

        assert_eq!(
            GeneratorType::Nuclear.waste_product(),
            Some(Item::UraniumWaste)
        );
        assert_eq!(GeneratorType::Biomass.waste_product(), None);
        assert_eq!(GeneratorType::Nuclear.base_waste_production(), 0.025);
    }

    // ===== GeneratorGroup Tests =====

    #[test]
    fn test_generator_group_creation() {
        let group = GeneratorGroup::new(5, 150.0).expect("Should create valid group");
        assert_eq!(group.number_of_generators, 5);
        assert_eq!(group.clock_speed, 150.0);
    }

    #[test]
    fn test_generator_group_invalid_count() {
        let result = GeneratorGroup::new(0, 100.0);
        assert!(result.is_err());
        match result {
            Err(PowerGeneratorError::InvalidGeneratorCount { count }) => {
                assert_eq!(count, 0);
            }
            _ => panic!("Expected InvalidGeneratorCount error"),
        }
    }

    #[test]
    fn test_generator_group_invalid_clock_speed_too_low() {
        let result = GeneratorGroup::new(5, -10.0);
        assert!(result.is_err());
        match result {
            Err(PowerGeneratorError::InvalidClockSpeed { clock_speed }) => {
                assert_eq!(clock_speed, -10.0);
            }
            _ => panic!("Expected InvalidClockSpeed error"),
        }
    }

    #[test]
    fn test_generator_group_invalid_clock_speed_too_high() {
        let result = GeneratorGroup::new(5, 300.0);
        assert!(result.is_err());
        match result {
            Err(PowerGeneratorError::InvalidClockSpeed { clock_speed }) => {
                assert_eq!(clock_speed, 300.0);
            }
            _ => panic!("Expected InvalidClockSpeed error"),
        }
    }

    #[test]
    fn test_generator_group_power_generation() {
        let group = GeneratorGroup::new(4, 100.0).expect("Should create valid group");
        // At 100% clock speed: 30MW * 1.0 * 4 generators = 120MW
        assert_eq!(group.power_generation(30.0), 120.0);
    }

    #[test]
    fn test_generator_group_power_generation_with_overclock() {
        let group = GeneratorGroup::new(2, 200.0).expect("Should create valid group");
        // At 200% clock speed: 75MW * 2.0 * 2 generators = 300MW
        assert_eq!(group.power_generation(75.0), 300.0);
    }

    #[test]
    fn test_generator_group_fuel_consumption() {
        let group = GeneratorGroup::new(3, 100.0).expect("Should create valid group");
        // At 100% clock speed: 15 * 1.0 * 1.0 * 3 generators = 45 items/min
        assert_eq!(group.fuel_consumption(15.0, 1.0), 45.0);
    }

    #[test]
    fn test_generator_group_fuel_consumption_with_overclock() {
        let group = GeneratorGroup::new(2, 50.0).expect("Should create valid group");
        // At 50% clock speed: 4.5 * 0.25 * 0.5 * 2 generators = 1.125 m³/min
        assert_eq!(group.fuel_consumption(4.5, 0.25), 1.125);
    }

    #[test]
    fn test_generator_group_waste_production() {
        let group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        // At 100% clock speed: 0.025 * 1.0 * 1 generator = 0.025 items/min
        assert_eq!(group.waste_production(0.025), 0.025);
    }

    #[test]
    fn test_generator_group_waste_production_with_overclock() {
        let group = GeneratorGroup::new(2, 200.0).expect("Should create valid group");
        // At 200% clock speed: 0.025 * 2.0 * 2 generators = 0.1 items/min
        assert_eq!(group.waste_production(0.025), 0.1);
    }

    #[test]
    fn test_generator_group_set_clock_speed() {
        let mut group = GeneratorGroup::new(5, 100.0).expect("Should create valid group");
        group
            .set_clock_speed(175.0)
            .expect("Should update clock speed");
        assert_eq!(group.clock_speed, 175.0);
    }

    #[test]
    fn test_generator_group_set_invalid_clock_speed() {
        let mut group = GeneratorGroup::new(5, 100.0).expect("Should create valid group");
        let result = group.set_clock_speed(300.0);
        assert!(result.is_err());
        assert_eq!(group.clock_speed, 100.0); // Should remain unchanged
    }

    #[test]
    fn test_generator_group_set_number_of_generators() {
        let mut group = GeneratorGroup::new(5, 100.0).expect("Should create valid group");
        group
            .set_number_of_generators(10)
            .expect("Should update count");
        assert_eq!(group.number_of_generators, 10);
    }

    #[test]
    fn test_generator_group_set_invalid_number_of_generators() {
        let mut group = GeneratorGroup::new(5, 100.0).expect("Should create valid group");
        let result = group.set_number_of_generators(0);
        assert!(result.is_err());
        assert_eq!(group.number_of_generators, 5); // Should remain unchanged
    }

    // ===== PowerGenerator Tests =====

    #[test]
    fn test_create_valid_coal_generator() {
        let generator = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Coal, Item::Coal)
            .expect("Should create valid coal generator");

        assert_eq!(generator.id, uuid_from_u64(1));
        assert_eq!(generator.generator_type, GeneratorType::Coal);
        assert_eq!(generator.fuel_type, Item::Coal);
        assert_eq!(generator.groups.len(), 0);
    }

    #[test]
    fn test_create_valid_fuel_generator_with_turbofuel() {
        let generator = PowerGenerator::new(uuid_from_u64(2), GeneratorType::Fuel, Item::Turbofuel)
            .expect("Should create valid fuel generator");

        assert_eq!(generator.generator_type, GeneratorType::Fuel);
        assert_eq!(generator.fuel_type, Item::Turbofuel);
    }

    #[test]
    fn test_create_invalid_fuel_combination() {
        let result = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Coal, Item::Fuel);
        assert!(result.is_err());
        match result {
            Err(PowerGeneratorError::IncompatibleFuel { generator, fuel }) => {
                assert_eq!(generator, GeneratorType::Coal);
                assert_eq!(fuel, Item::Fuel);
            }
            _ => panic!("Expected IncompatibleFuel error"),
        }
    }

    #[test]
    fn test_create_geothermal_generator() {
        let generator = PowerGenerator::new_geothermal(uuid_from_u64(3));
        assert_eq!(generator.id, uuid_from_u64(3));
        assert_eq!(generator.generator_type, GeneratorType::Geothermal);
        assert_eq!(generator.groups.len(), 0);
    }

    #[test]
    fn test_add_generator_group() {
        let mut generator =
            PowerGenerator::new(uuid_from_u64(1), GeneratorType::Biomass, Item::Biomass)
                .expect("Should create valid generator");

        let group = GeneratorGroup::new(3, 150.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        assert_eq!(generator.groups.len(), 1);
        assert_eq!(generator.groups[0].number_of_generators, 3);
        assert_eq!(generator.groups[0].clock_speed, 150.0);
    }

    #[test]
    fn test_remove_generator_group() {
        let mut generator = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Coal, Item::Coal)
            .expect("Should create valid generator");

        let group1 = GeneratorGroup::new(2, 100.0).expect("Should create valid group");
        let group2 = GeneratorGroup::new(3, 150.0).expect("Should create valid group");
        generator.add_group(group1).expect("Should add group");
        generator.add_group(group2).expect("Should add group");

        assert_eq!(generator.groups.len(), 2);

        let removed = generator.remove_group(0).expect("Should remove group");
        assert_eq!(removed.number_of_generators, 2);
        assert_eq!(generator.groups.len(), 1);
        assert_eq!(generator.groups[0].number_of_generators, 3);
    }

    #[test]
    fn test_remove_nonexistent_generator_group() {
        let mut generator = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Fuel, Item::Fuel)
            .expect("Should create valid generator");

        let result = generator.remove_group(0);
        assert!(result.is_err());
        match result {
            Err(PowerGeneratorError::GroupNotFound { index }) => {
                assert_eq!(index, 0);
            }
            _ => panic!("Expected GroupNotFound error"),
        }
    }

    #[test]
    fn test_get_generator_group() {
        let mut generator = PowerGenerator::new(
            uuid_from_u64(1),
            GeneratorType::Nuclear,
            Item::UraniumFuelRod,
        )
        .expect("Should create valid generator");

        let group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        let retrieved = generator.get_group(0);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().number_of_generators, 1);
    }

    #[test]
    fn test_get_generator_group_mut() {
        let mut generator =
            PowerGenerator::new(uuid_from_u64(1), GeneratorType::Biomass, Item::Biomass)
                .expect("Should create valid generator");

        let group = GeneratorGroup::new(2, 100.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        if let Some(group_mut) = generator.get_group_mut(0) {
            group_mut
                .set_clock_speed(200.0)
                .expect("Should update clock speed");
        }

        assert_eq!(generator.get_group(0).unwrap().clock_speed, 200.0);
    }

    // ===== Calculation Tests =====

    #[test]
    fn test_total_power_generation_single_group() {
        let mut generator = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Coal, Item::Coal)
            .expect("Should create valid generator");

        let group = GeneratorGroup::new(4, 100.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        // 4 coal generators at 100%: 4 * 75MW = 300MW
        assert_eq!(generator.total_power_generation(), 300.0);
    }

    #[test]
    fn test_total_power_generation_multiple_groups() {
        let mut generator = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Fuel, Item::Fuel)
            .expect("Should create valid generator");

        let group1 = GeneratorGroup::new(2, 100.0).expect("Should create valid group");
        let group2 = GeneratorGroup::new(3, 200.0).expect("Should create valid group");
        generator.add_group(group1).expect("Should add group");
        generator.add_group(group2).expect("Should add group");

        // Group 1: 2 * 150MW * 1.0 = 300MW
        // Group 2: 3 * 150MW * 2.0 = 900MW
        // Total: 1200MW
        assert_eq!(generator.total_power_generation(), 1200.0);
    }

    #[test]
    fn test_total_fuel_consumption_coal() {
        let mut generator = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Coal, Item::Coal)
            .expect("Should create valid generator");

        let group = GeneratorGroup::new(2, 150.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        // 2 coal generators at 150%: 2 * 15 * 1.0 * 1.5 = 45 items/min
        assert_eq!(generator.total_fuel_consumption(), 45.0);
    }

    #[test]
    fn test_total_fuel_consumption_turbofuel() {
        let mut generator =
            PowerGenerator::new(uuid_from_u64(1), GeneratorType::Fuel, Item::Turbofuel)
                .expect("Should create valid generator");

        let group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        // 1 fuel generator with turbofuel: 1 * 4.5 * 0.25 * 1.0 = 1.125 m³/min
        assert_eq!(generator.total_fuel_consumption(), 1.125);
    }

    #[test]
    fn test_total_fuel_consumption_compacted_coal() {
        let mut generator =
            PowerGenerator::new(uuid_from_u64(1), GeneratorType::Coal, Item::CompactedCoal)
                .expect("Should create valid generator");

        let group = GeneratorGroup::new(3, 100.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        // 3 coal generators with compacted coal: 3 * 15 * 0.8 * 1.0 = 36 items/min
        assert_eq!(generator.total_fuel_consumption(), 36.0);
    }

    #[test]
    fn test_geothermal_no_fuel_consumption() {
        let mut generator = PowerGenerator::new_geothermal(uuid_from_u64(1));
        let group = GeneratorGroup::new(5, 200.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        assert_eq!(generator.total_fuel_consumption(), 0.0);
        // But still generates power: 5 * 200MW * 2.0 = 2000MW
        assert_eq!(generator.total_power_generation(), 2000.0);
    }

    #[test]
    fn test_nuclear_waste_production() {
        let mut generator = PowerGenerator::new(
            uuid_from_u64(1),
            GeneratorType::Nuclear,
            Item::UraniumFuelRod,
        )
        .expect("Should create valid generator");

        let group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        assert_eq!(generator.waste_production_rate(), 0.025);
        assert_eq!(generator.waste_product(), Some(Item::UraniumWaste));
    }

    #[test]
    fn test_nuclear_waste_production_with_overclock() {
        let mut generator = PowerGenerator::new(
            uuid_from_u64(1),
            GeneratorType::Nuclear,
            Item::UraniumFuelRod,
        )
        .expect("Should create valid generator");

        let group = GeneratorGroup::new(2, 200.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        // 2 nuclear plants at 200%: 2 * 0.025 * 2.0 = 0.1 waste rods/min
        assert_eq!(generator.waste_production_rate(), 0.1);
    }

    #[test]
    fn test_non_nuclear_no_waste_production() {
        let mut generator = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Coal, Item::Coal)
            .expect("Should create valid generator");

        let group = GeneratorGroup::new(5, 200.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        assert_eq!(generator.waste_production_rate(), 0.0);
        assert_eq!(generator.waste_product(), None);
    }

    // ===== Validation Tests =====

    #[test]
    fn test_validate_valid_generator() {
        let mut generator =
            PowerGenerator::new(uuid_from_u64(1), GeneratorType::Biomass, Item::Biomass)
                .expect("Should create valid generator");

        let group = GeneratorGroup::new(3, 150.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        assert!(generator.validate().is_ok());
    }

    #[test]
    fn test_validate_generator_no_groups() {
        let generator = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Coal, Item::Coal)
            .expect("Should create valid generator");

        let result = generator.validate();
        assert!(result.is_err());
        match result {
            Err(PowerGeneratorError::NoGroups) => {
                // Expected error
            }
            _ => panic!("Expected NoGroups error"),
        }
    }

    #[test]
    fn test_validate_invalid_fuel_combination() {
        let generator = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Fuel, Item::Coal)
            .expect_err("Should fail to create generator");

        match generator {
            PowerGeneratorError::IncompatibleFuel { generator, fuel } => {
                assert_eq!(generator, GeneratorType::Fuel);
                assert_eq!(fuel, Item::Coal);
            }
            _ => panic!("Expected IncompatibleFuel error"),
        }
    }

    // ===== Edge Case Tests =====

    #[test]
    fn test_all_generator_types_with_compatible_fuels() {
        let test_cases = vec![
            (GeneratorType::Biomass, Item::Biomass),
            (GeneratorType::Coal, Item::Coal),
            (GeneratorType::Fuel, Item::Fuel),
            (GeneratorType::Nuclear, Item::UraniumFuelRod),
        ];

        for (gen_type, fuel) in test_cases {
            let generator =
                PowerGenerator::new(uuid_from_u64(1), gen_type, fuel).unwrap_or_else(|_| {
                    panic!(
                        "Should create valid {:?} generator with {:?}",
                        gen_type, fuel
                    )
                });
            assert_eq!(generator.generator_type, gen_type);
            assert_eq!(generator.fuel_type, fuel);
        }
    }

    #[test]
    fn test_all_biomass_fuel_types() {
        let biomass_fuels = vec![
            Item::Biomass,
            Item::Leaves,
            Item::Wood,
            Item::Mycelia,
            Item::FlowerPetals,
            Item::BaconAgaric,
            Item::Paleberry,
        ];

        for fuel in biomass_fuels {
            let generator = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Biomass, fuel)
                .unwrap_or_else(|_| {
                    panic!("Should create valid biomass generator with {:?}", fuel)
                });
            assert_eq!(generator.fuel_type, fuel);
        }
    }

    #[test]
    fn test_fuel_efficiency_comparison() {
        let mut coal_gen = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Coal, Item::Coal)
            .expect("Should create valid coal generator");
        let mut compacted_gen =
            PowerGenerator::new(uuid_from_u64(2), GeneratorType::Coal, Item::CompactedCoal)
                .expect("Should create valid compacted coal generator");

        let group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        coal_gen.add_group(group.clone()).expect("Should add group");
        compacted_gen.add_group(group).expect("Should add group");

        // Both generate same power (75MW)
        assert_eq!(
            coal_gen.total_power_generation(),
            compacted_gen.total_power_generation()
        );

        // But compacted coal uses less fuel (15 * 0.8 = 12 vs 15)
        assert!(coal_gen.total_fuel_consumption() > compacted_gen.total_fuel_consumption());
    }

    #[test]
    fn test_turbofuel_efficiency() {
        let mut fuel_gen = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Fuel, Item::Fuel)
            .expect("Should create valid fuel generator");
        let mut turbo_gen =
            PowerGenerator::new(uuid_from_u64(2), GeneratorType::Fuel, Item::Turbofuel)
                .expect("Should create valid turbofuel generator");

        let group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        fuel_gen.add_group(group.clone()).expect("Should add group");
        turbo_gen.add_group(group).expect("Should add group");

        // Both generate same power (150MW)
        assert_eq!(
            fuel_gen.total_power_generation(),
            turbo_gen.total_power_generation()
        );

        // But turbofuel uses much less fuel (4.5 * 0.25 = 1.125 vs 4.5)
        assert!(fuel_gen.total_fuel_consumption() > turbo_gen.total_fuel_consumption());
    }

    #[test]
    fn test_overclock_scaling_linearity() {
        let mut generator = PowerGenerator::new(uuid_from_u64(1), GeneratorType::Coal, Item::Coal)
            .expect("Should create valid generator");

        let group = GeneratorGroup::new(1, 100.0).expect("Should create valid group");
        generator.add_group(group).expect("Should add group");

        let base_power = generator.total_power_generation();
        let base_fuel = generator.total_fuel_consumption();

        // Test linear scaling: at 200% should be exactly 2x
        if let Some(group) = generator.get_group_mut(0) {
            group
                .set_clock_speed(200.0)
                .expect("Should set clock speed");
        }

        assert_eq!(generator.total_power_generation(), base_power * 2.0);
        assert_eq!(generator.total_fuel_consumption(), base_fuel * 2.0);

        // Test linear scaling: at 50% should be exactly 0.5x
        if let Some(group) = generator.get_group_mut(0) {
            group.set_clock_speed(50.0).expect("Should set clock speed");
        }

        assert_eq!(generator.total_power_generation(), base_power * 0.5);
        assert_eq!(generator.total_fuel_consumption(), base_fuel * 0.5);
    }
}
