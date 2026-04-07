use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::models::{recipe_info, Item, ProductionLineId, Recipe};

/// Error type for ProductionLineBlueprint and SatisflowEngine operations
#[derive(Debug, Clone, PartialEq)]
pub enum EngineError {
    IndexOutOfBounds { index: usize, length: usize },
    TemplateNotFound { id: ProductionLineId },
    EmptyName,
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::IndexOutOfBounds { index, length } => {
                write!(
                    f,
                    "Index {} out of bounds for production line with {} elements",
                    index, length
                )
            }
            EngineError::TemplateNotFound { id } => {
                write!(f, "Blueprint template with id {} not found", id)
            }
            EngineError::EmptyName => {
                write!(f, "Name cannot be empty")
            }
        }
    }
}

impl std::error::Error for EngineError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductionLine {
    ProductionLineRecipe(ProductionLineRecipe),
    ProductionLineBlueprint(ProductionLineBlueprint),
}
impl ProductionLine {
    pub fn id(&self) -> ProductionLineId {
        match self {
            ProductionLine::ProductionLineRecipe(line) => line.id(),
            ProductionLine::ProductionLineBlueprint(blueprint) => blueprint.id(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ProductionLine::ProductionLineRecipe(line) => line.name(),
            ProductionLine::ProductionLineBlueprint(blueprint) => blueprint.name(),
        }
    }

    pub fn total_machines(&self) -> u32 {
        match self {
            ProductionLine::ProductionLineRecipe(line) => line.total_machines(),
            ProductionLine::ProductionLineBlueprint(blueprint) => blueprint.total_machines(),
        }
    }

    pub fn total_somersloop(&self) -> u32 {
        match self {
            ProductionLine::ProductionLineRecipe(line) => line.total_somersloop(),
            ProductionLine::ProductionLineBlueprint(blueprint) => blueprint.total_somersloop(),
        }
    }

    pub fn output_rate(&self) -> Vec<(Item, f32)> {
        match self {
            ProductionLine::ProductionLineRecipe(line) => line.output_rate(),
            ProductionLine::ProductionLineBlueprint(blueprint) => blueprint.output_rate(),
        }
    }

    pub fn input_rate(&self) -> Vec<(Item, f32)> {
        match self {
            ProductionLine::ProductionLineRecipe(line) => line.input_rate(),
            ProductionLine::ProductionLineBlueprint(blueprint) => blueprint.input_rate(),
        }
    }

    pub fn total_power_consumption(&self) -> f32 {
        match self {
            ProductionLine::ProductionLineRecipe(line) => line.total_power_consumption(),
            ProductionLine::ProductionLineBlueprint(blueprint) => {
                blueprint.total_power_consumption()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionLineRecipe {
    pub id: ProductionLineId,
    pub name: String,
    pub description: Option<String>,
    pub recipe: Recipe,
    pub machine_groups: Vec<MachineGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionLineBlueprint {
    pub id: ProductionLineId,
    pub name: String,
    pub description: Option<String>,
    pub production_lines: Vec<ProductionLineRecipe>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineGroup {
    pub number_of_machine: u32, // number of machine in the groupe
    pub oc_value: f32,          // overclock value
    pub somersloop: u8,         // number of somersloop per machine
}

impl ProductionLineRecipe {
    /// Create a new production line with no machine groups
    pub fn new(
        id: ProductionLineId,
        name: String,
        description: Option<String>,
        recipe: Recipe,
    ) -> Self {
        Self {
            id,
            name,
            description,
            recipe,
            machine_groups: Vec::new(),
        }
    }

    /// Add a machine group to the production line
    /// Returns an error if the machine group is invalid
    pub fn add_machine_group(
        &mut self,
        group: MachineGroup,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if group.somersloop > recipe_info(self.recipe).machine.max_somersloop() {
            return Err(format!("Cannot add machine group with more somersloop than the machine type allows {} > {}", group.somersloop, recipe_info(self.recipe).machine.max_somersloop()).into());
        }
        if group.oc_value < 0.0 || group.oc_value > 250.0 {
            return Err("Overclock value must be between 0.000 and 250.000".into());
        }
        self.machine_groups.push(group);
        Ok(())
    }

    fn id(&self) -> ProductionLineId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn total_machines(&self) -> u32 {
        self.machine_groups
            .iter()
            .map(|group| group.number_of_machine)
            .sum()
    }

    fn total_somersloop(&self) -> u32 {
        self.machine_groups
            .iter()
            .map(|group| group.number_of_machine * group.somersloop as u32)
            .sum()
    }

    fn output_rate(&self) -> Vec<(Item, f32)> {
        let recipe_info = recipe_info(self.recipe);
        let mut result = vec![];
        for (item, rate) in recipe_info.outputs.iter() {
            for group in &self.machine_groups {
                let machine_output =
                    rate * (group.oc_value / 100.0) * group.number_of_machine as f32;
                if group.somersloop > 0 {
                    // Somersloop multiply the production rate depending on the number of somersloop and the machine type
                    let somersloop_multiplier = 1.0
                        + (group.somersloop as f32 / recipe_info.machine.max_somersloop() as f32);
                    result.push((*item, machine_output * somersloop_multiplier));
                } else {
                    result.push((*item, machine_output));
                }
            }
        }
        result
    }

    fn input_rate(&self) -> Vec<(Item, f32)> {
        let recipe_info = recipe_info(self.recipe);
        let mut result = vec![];
        for (item, rate) in recipe_info.inputs.iter() {
            for group in &self.machine_groups {
                let machine_input =
                    rate * (group.oc_value / 100.0) * group.number_of_machine as f32;
                result.push((*item, machine_input));
            }
        }
        result
    }

    /// Power multiplier = (1 + somersloop / max_somersloop)²
    /// Power usage = Base power usage × Power multiplier × (Clock speed100)^1.321928
    fn total_power_consumption(&self) -> f32 {
        let recipe_info = recipe_info(self.recipe);
        let base_power = recipe_info.machine.base_power_mw();
        let mut total_power = 0.0;
        for group in &self.machine_groups {
            let somersloop_multiplier = if group.somersloop > 0 {
                1.0 + (group.somersloop as f32 / recipe_info.machine.max_somersloop() as f32)
            } else {
                1.0
            };
            let power_multiplier = somersloop_multiplier * somersloop_multiplier;
            let machine_power =
                base_power * power_multiplier * (group.oc_value / 100.0).powf(1.321928);
            total_power += machine_power * group.number_of_machine as f32;
        }
        total_power
    }

    /// Get a machine group by index
    pub fn get_machine_group(&self, index: usize) -> Option<&MachineGroup> {
        self.machine_groups.get(index)
    }

    /// Remove a machine group at the given index
    /// Returns the removed machine group if successful
    pub fn remove_machine_group(&mut self, index: usize) -> Option<MachineGroup> {
        if index < self.machine_groups.len() {
            Some(self.machine_groups.remove(index))
        } else {
            None
        }
    }

    /// Update a machine group at the given index with new values
    /// Returns an error if the index is out of bounds or if any value is invalid
    pub fn update_machine_group(
        &mut self,
        index: usize,
        oc: f32,
        somersloop: u8,
        count: u32,
    ) -> Result<(), MachineGroupError> {
        if index >= self.machine_groups.len() {
            return Err(MachineGroupError::IndexOutOfBounds {
                index,
                length: self.machine_groups.len(),
            });
        }

        // Validate all values before making any changes
        if !(0.0..=250.0).contains(&oc) {
            return Err(MachineGroupError::InvalidOverclockValue { oc_value: oc });
        }

        let max_somersloop = recipe_info(self.recipe).machine.max_somersloop();
        if somersloop > max_somersloop {
            return Err(MachineGroupError::InvalidSomersloopCount {
                somersloop,
                max_somersloop,
            });
        }

        if count == 0 {
            return Err(MachineGroupError::InvalidMachineCount { count });
        }

        // All validation passed, update the machine group
        let group = &mut self.machine_groups[index];
        group.oc_value = oc;
        group.somersloop = somersloop;
        group.number_of_machine = count;

        Ok(())
    }

    /// Set the name of the production line
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Set the description of the production line
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }
}

impl ProductionLineBlueprint {
    pub fn new(id: ProductionLineId, name: String, description: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
            production_lines: Vec::new(),
        }
    }

    pub fn add_production_line(&mut self, line: ProductionLineRecipe) {
        self.production_lines.push(line);
    }

    /// Remove a production line at the given index.
    /// Returns the removed line if successful, or None if index is out of bounds.
    pub fn remove_production_line(&mut self, index: usize) -> Option<ProductionLineRecipe> {
        if index < self.production_lines.len() {
            Some(self.production_lines.remove(index))
        } else {
            None
        }
    }

    /// Update a production line at the given index.
    /// Returns Ok(()) if successful, or EngineError if index is out of bounds.
    pub fn update_production_line(
        &mut self,
        index: usize,
        line: ProductionLineRecipe,
    ) -> Result<(), EngineError> {
        if index >= self.production_lines.len() {
            return Err(EngineError::IndexOutOfBounds {
                index,
                length: self.production_lines.len(),
            });
        }
        self.production_lines[index] = line;
        Ok(())
    }

    /// Get a production line at the given index.
    pub fn get_production_line(&self, index: usize) -> Option<&ProductionLineRecipe> {
        self.production_lines.get(index)
    }

    /// Set the blueprint name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Set the blueprint description.
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    fn id(&self) -> ProductionLineId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn total_machines(&self) -> u32 {
        self.production_lines
            .iter()
            .map(|line| line.total_machines())
            .sum()
    }

    fn total_somersloop(&self) -> u32 {
        self.production_lines
            .iter()
            .map(|line| line.total_somersloop())
            .sum()
    }

    fn output_rate(&self) -> Vec<(Item, f32)> {
        let mut result = vec![];
        for line in &self.production_lines {
            for (item, rate) in line.output_rate() {
                if let Some(existing) = result.iter_mut().find(|(i, _)| *i == item) {
                    existing.1 += rate;
                } else {
                    result.push((item, rate));
                }
            }
        }
        result
    }

    fn input_rate(&self) -> Vec<(Item, f32)> {
        let mut result = vec![];
        for line in &self.production_lines {
            for (item, rate) in line.input_rate() {
                if let Some(existing) = result.iter_mut().find(|(i, _)| *i == item) {
                    existing.1 += rate;
                } else {
                    result.push((item, rate));
                }
            }
        }
        result
    }

    fn total_power_consumption(&self) -> f32 {
        self.production_lines
            .iter()
            .map(|line| line.total_power_consumption())
            .sum()
    }
}

/// Errors that can occur when working with machine groups
#[derive(Debug, Clone, PartialEq)]
pub enum MachineGroupError {
    InvalidOverclockValue { oc_value: f32 },
    InvalidMachineCount { count: u32 },
    InvalidSomersloopCount { somersloop: u8, max_somersloop: u8 },
    IndexOutOfBounds { index: usize, length: usize },
}

impl std::fmt::Display for MachineGroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineGroupError::InvalidOverclockValue { oc_value } => {
                write!(
                    f,
                    "Overclock value {} is invalid. Must be between 0.000 and 250.000",
                    oc_value
                )
            }
            MachineGroupError::InvalidMachineCount { count } => {
                write!(
                    f,
                    "Machine count {} is invalid. Must be greater than 0",
                    count
                )
            }
            MachineGroupError::InvalidSomersloopCount {
                somersloop,
                max_somersloop,
            } => {
                write!(
                    f,
                    "Somersloop count {} is invalid. Must be between 0 and {}",
                    somersloop, max_somersloop
                )
            }
            MachineGroupError::IndexOutOfBounds { index, length } => {
                write!(
                    f,
                    "Index {} is out of bounds for machine groups of length {}",
                    index, length
                )
            }
        }
    }
}

impl std::error::Error for MachineGroupError {}

impl MachineGroup {
    pub fn new(number_of_machines: u32, overclock: f32, somersloop_per_machine: u8) -> Self {
        Self {
            number_of_machine: number_of_machines,
            oc_value: overclock,
            somersloop: somersloop_per_machine,
        }
    }

    /// Set the overclock value for this machine group
    pub fn set_oc_value(&mut self, oc: f32) -> Result<(), MachineGroupError> {
        if !(0.0..=250.0).contains(&oc) {
            return Err(MachineGroupError::InvalidOverclockValue { oc_value: oc });
        }
        self.oc_value = oc;
        Ok(())
    }

    /// Set the somersloop count per machine
    /// Note: Does not validate against max_somersloop - validation should be done
    /// by the caller (ProductionLineRecipe::update_machine_group) when recipe context is available
    pub fn set_somersloop(&mut self, s: u8) -> Result<(), MachineGroupError> {
        self.somersloop = s;
        Ok(())
    }

    /// Set the number of machines in this group
    pub fn set_number_of_machines(&mut self, n: u32) -> Result<(), MachineGroupError> {
        if n == 0 {
            return Err(MachineGroupError::InvalidMachineCount { count: n });
        }
        self.number_of_machine = n;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Recipe;
    use uuid::Uuid;

    fn uuid_from_u64(value: u64) -> Uuid {
        Uuid::from_u128(value as u128)
    }

    #[test]
    fn test_production_line_creation_empty() {
        let production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test Line".to_string(),
            None,
            Recipe::AILimiter,
        );
        assert_eq!(production_line.id, uuid_from_u64(1));
        assert_eq!(production_line.name, "Test Line");
        assert!(production_line.description.is_none());
        assert_eq!(production_line.machine_groups.len(), 0);
    }

    #[test]
    fn test_add_machine_group() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test Line".to_string(),
            None,
            Recipe::AILimiter,
        );
        let machine_group = MachineGroup::new(5, 150.0, 2);
        production_line
            .add_machine_group(machine_group)
            .expect("Invalide group");
        assert_eq!(production_line.machine_groups.len(), 1);
        assert_eq!(production_line.machine_groups[0].number_of_machine, 5);
        assert_eq!(production_line.machine_groups[0].oc_value, 150.0);
        assert_eq!(production_line.machine_groups[0].somersloop, 2);
    }

    #[test]
    fn test_total_machines() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test Line".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(4, 100.0, 0))
            .expect("Invalid group");
        assert!(production_line.total_machines() == 4);
        assert_eq!(
            production_line.output_rate(),
            vec![(Item::IronIngot, 120.0)]
        );
        assert_eq!(production_line.input_rate(), vec![(Item::IronOre, 120.0)]);
        assert_eq!(production_line.total_power_consumption(), 16.0); // 4 machines * 4 MW each at 100% clock speed
    }

    #[test]
    #[should_panic(
        expected = "Cannot add machine group with more somersloop than the machine type allows"
    )]
    fn test_add_machine_group_invalid_somersloop() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test Line".to_string(),
            None,
            Recipe::IronIngot,
        );
        let machine_group = MachineGroup::new(5, 100.0, 3); // Iron Ingot recipe uses Constructor which allows only 1 somersloop
        production_line.add_machine_group(machine_group).unwrap(); // This should panic
    }

    #[test]
    #[should_panic(expected = "Overclock value must be between 0.000 and 250.000")]
    fn test_add_machine_group_invalid_overclock() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test Line".to_string(),
            None,
            Recipe::IronIngot,
        );
        let machine_group = MachineGroup::new(5, 300.0, 1); // Invalid overclock value
        production_line.add_machine_group(machine_group).unwrap(); // This should panic
    }

    #[test]
    fn test_half_somersloop_for_power() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test Line".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(2, 100.0, 1))
            .expect("Invalid group");
        // Each machine has 1 somersloop, max is 1 for Constructor, so power multiplier is (1 + 1/1)² = 4
        // Each machine consumes 4 MW at base power, so total power = 2 machines * 4 MW * 4 = 32 MW
        assert_eq!(production_line.total_power_consumption(), 32.0);
    }

    // =========================================================================
    // Power Calculation Regression Tests
    // =========================================================================
    // Bug: total_power *= group.number_of_machine compounded across groups
    // instead of being part of each group's calculation.
    //
    // BUGGY: for 2 groups of 2 machines each at 100% OC:
    //   Group1: total = 4.0, total *= 2 → 8.0
    //   Group2: total = 8.0 + 4.0 = 12.0, total *= 2 → 24.0 (WRONG)
    //
    // CORRECT: for 2 groups of 2 machines each at 100% OC:
    //   Group1: 2 * 4.0 = 8.0
    //   Group2: 2 * 4.0 = 8.0
    //   Total: 8.0 + 8.0 = 16.0

    #[test]
    fn test_power_single_machine_100_oc() {
        // Single machine at 100% OC = 4.0 MW (base power)
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(1, 100.0, 0))
            .expect("Invalid group");
        // 1 machine * 4.0 MW * (100/100)^1.321928 = 4.0 MW
        assert_eq!(production_line.total_power_consumption(), 4.0);
    }

    #[test]
    fn test_power_two_machines_100_oc() {
        // 2 machines at 100% OC = 8.0 MW
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(2, 100.0, 0))
            .expect("Invalid group");
        // 2 machines * 4.0 MW * (100/100)^1.321928 = 8.0 MW
        assert_eq!(production_line.total_power_consumption(), 8.0);
    }

    #[test]
    fn test_power_multiple_groups_no_compounding() {
        // Two groups of 2 machines each at 100% OC = 16.0 MW
        // This is the key regression test - with the bug, this would be 24.0 MW
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(2, 100.0, 0))
            .expect("Invalid group");
        production_line
            .add_machine_group(MachineGroup::new(2, 100.0, 0))
            .expect("Invalid group");
        // Group1: 2 * 4.0 = 8.0 MW
        // Group2: 2 * 4.0 = 8.0 MW
        // Total: 8.0 + 8.0 = 16.0 MW (NOT compounding)
        assert_eq!(production_line.total_power_consumption(), 16.0);
    }

    #[test]
    fn test_power_multiple_groups_different_oc() {
        // One group at 100% OC, one at 200% OC
        // 100% OC: 1 * 4.0 = 4.0 MW
        // 200% OC: 1 * 4.0 * (200/100)^1.321928 = 4.0 * 2^1.321928 ≈ 4.0 * 2.5 = 10.0 MW
        // Total: 4.0 + 10.0 = 14.0 MW
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(1, 100.0, 0))
            .expect("Invalid group");
        production_line
            .add_machine_group(MachineGroup::new(1, 200.0, 0))
            .expect("Invalid group");
        let total_power = production_line.total_power_consumption();
        // Allow small floating point tolerance
        assert!((total_power - 14.0).abs() < 0.01);
    }

    #[test]
    fn test_power_zero_oc() {
        // 0% OC means machine is off, 0 MW power consumption
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(4, 0.0, 0))
            .expect("Invalid group");
        // 0% clock speed = machine off = 0 MW
        assert_eq!(production_line.total_power_consumption(), 0.0);
    }

    #[test]
    fn test_power_max_oc_250() {
        // 250% OC = 2.5x clock speed, power = base * 2.5^1.321928 = base * 2.5^log2(2.5)
        // = base * 2.5^1.321928 = base * 2.5^1.321928 ≈ base * 2.5^1.321928
        // For 1 machine at 250% OC: 4.0 * 2.5^1.321928 ≈ 4.0 * 2.5^1.321928
        // 2.5^1.321928 = e^(1.321928 * ln(2.5)) = e^(1.321928 * 0.9163) = e^(1.211) ≈ 3.36
        // So 4.0 * 3.36 ≈ 13.44 MW per machine
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(1, 250.0, 0))
            .expect("Invalid group");
        let total_power = production_line.total_power_consumption();
        // 4.0 * (250/100)^1.321928 = 4.0 * 2.5^1.321928 ≈ 4.0 * 3.36 = 13.44
        assert!((total_power - 13.44).abs() < 0.1);
    }

    // ============== MachineGroup Mutation Tests ==============

    #[test]
    fn test_machine_group_set_oc_value_valid() {
        let mut group = MachineGroup::new(4, 100.0, 0);
        group.set_oc_value(150.0).expect("Should set OC value");
        assert_eq!(group.oc_value, 150.0);
    }

    #[test]
    fn test_machine_group_set_oc_value_min() {
        let mut group = MachineGroup::new(4, 100.0, 0);
        group.set_oc_value(0.0).expect("Should set OC value to min");
        assert_eq!(group.oc_value, 0.0);
    }

    #[test]
    fn test_machine_group_set_oc_value_max() {
        let mut group = MachineGroup::new(4, 100.0, 0);
        group
            .set_oc_value(250.0)
            .expect("Should set OC value to max");
        assert_eq!(group.oc_value, 250.0);
    }

    #[test]
    fn test_machine_group_set_oc_value_invalid_negative() {
        let mut group = MachineGroup::new(4, 100.0, 0);
        let result = group.set_oc_value(-0.001);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MachineGroupError::InvalidOverclockValue { oc_value: -0.001 }
        );
    }

    #[test]
    fn test_machine_group_set_oc_value_invalid_over_max() {
        let mut group = MachineGroup::new(4, 100.0, 0);
        let result = group.set_oc_value(250.001);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MachineGroupError::InvalidOverclockValue { oc_value: 250.001 }
        );
    }

    #[test]
    fn test_machine_group_set_number_of_machines_valid() {
        let mut group = MachineGroup::new(4, 100.0, 0);
        group
            .set_number_of_machines(10)
            .expect("Should set machine count");
        assert_eq!(group.number_of_machine, 10);
    }

    #[test]
    fn test_machine_group_set_number_of_machines_invalid_zero() {
        let mut group = MachineGroup::new(4, 100.0, 0);
        let result = group.set_number_of_machines(0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MachineGroupError::InvalidMachineCount { count: 0 }
        );
    }

    #[test]
    fn test_machine_group_set_somersloop_valid() {
        let mut group = MachineGroup::new(4, 100.0, 0);
        group.set_somersloop(2).expect("Should set somersloop");
        assert_eq!(group.somersloop, 2);
    }

    // ============== ProductionLineRecipe Mutation Tests ==============

    #[test]
    fn test_get_machine_group() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(4, 100.0, 0))
            .expect("Invalid group");
        production_line
            .add_machine_group(MachineGroup::new(2, 150.0, 1))
            .expect("Invalid group");

        let group0 = production_line.get_machine_group(0);
        assert!(group0.is_some());
        assert_eq!(group0.unwrap().number_of_machine, 4);

        let group1 = production_line.get_machine_group(1);
        assert!(group1.is_some());
        assert_eq!(group1.unwrap().number_of_machine, 2);

        let group2 = production_line.get_machine_group(2);
        assert!(group2.is_none());
    }

    #[test]
    fn test_remove_machine_group() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(4, 100.0, 0))
            .expect("Invalid group");
        production_line
            .add_machine_group(MachineGroup::new(2, 150.0, 1))
            .expect("Invalid group");

        assert_eq!(production_line.machine_groups.len(), 2);

        let removed = production_line.remove_machine_group(0);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().number_of_machine, 4);
        assert_eq!(production_line.machine_groups.len(), 1);
        assert_eq!(production_line.machine_groups[0].number_of_machine, 2);

        // Remove out of bounds
        let removed = production_line.remove_machine_group(5);
        assert!(removed.is_none());
    }

    #[test]
    fn test_update_machine_group_valid() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(4, 100.0, 0))
            .expect("Invalid group");

        production_line
            .update_machine_group(0, 150.0, 1, 8)
            .expect("Should update machine group");

        let group = &production_line.machine_groups[0];
        assert_eq!(group.oc_value, 150.0);
        assert_eq!(group.somersloop, 1);
        assert_eq!(group.number_of_machine, 8);
    }

    #[test]
    fn test_update_machine_group_invalid_oc() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(4, 100.0, 0))
            .expect("Invalid group");

        let result = production_line.update_machine_group(0, 300.0, 0, 4);
        assert!(result.is_err());
        // Original group should be unchanged
        assert_eq!(production_line.machine_groups[0].oc_value, 100.0);
    }

    #[test]
    fn test_update_machine_group_invalid_somersloop() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(4, 100.0, 0))
            .expect("Invalid group");

        // IronIngot machine (Constructor) has max_somersloop of 1
        let result = production_line.update_machine_group(0, 100.0, 2, 4);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MachineGroupError::InvalidSomersloopCount {
                somersloop: 2,
                max_somersloop: 1
            }
        );
    }

    #[test]
    fn test_update_machine_group_invalid_count() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(4, 100.0, 0))
            .expect("Invalid group");

        let result = production_line.update_machine_group(0, 100.0, 0, 0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MachineGroupError::InvalidMachineCount { count: 0 }
        );
    }

    #[test]
    fn test_update_machine_group_index_out_of_bounds() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line
            .add_machine_group(MachineGroup::new(4, 100.0, 0))
            .expect("Invalid group");

        let result = production_line.update_machine_group(5, 100.0, 0, 4);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MachineGroupError::IndexOutOfBounds {
                index: 5,
                length: 1
            }
        );
    }

    #[test]
    fn test_set_name() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Original".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line.set_name("New Name".to_string());
        assert_eq!(production_line.name, "New Name");
    }

    #[test]
    fn test_set_description_some() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            None,
            Recipe::IronIngot,
        );
        production_line.set_description(Some("A description".to_string()));
        assert_eq!(
            production_line.description,
            Some("A description".to_string())
        );
    }

    #[test]
    fn test_set_description_none() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test".to_string(),
            Some("Has description".to_string()),
            Recipe::IronIngot,
        );
        production_line.set_description(None);
        assert!(production_line.description.is_none());
    }
}
