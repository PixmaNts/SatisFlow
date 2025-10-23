use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::models::{recipe_info, Item, ProductionLineId, Recipe};

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

    pub fn total_sommersloop(&self) -> u32 {
        match self {
            ProductionLine::ProductionLineRecipe(line) => line.total_sommersloop(),
            ProductionLine::ProductionLineBlueprint(blueprint) => blueprint.total_sommersloop(),
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
    pub somersloop: u8,         // number of sommersloop per machine
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
        if group.somersloop > recipe_info(self.recipe).machine.max_sommersloop() {
            return Err(format!("Cannot add machine group with more sommersloop than the machine type allows {} > {}", group.somersloop, recipe_info(self.recipe).machine.max_sommersloop()).into());
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

    fn total_sommersloop(&self) -> u32 {
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
                    // Sommersloop multiply the production rate depending on the number of sommersloop and the machine type
                    let sommersloop_multiplier = 1.0
                        + (group.somersloop as f32 / recipe_info.machine.max_sommersloop() as f32);
                    result.push((*item, machine_output * sommersloop_multiplier));
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

    /// Power multiplier = (1 + somersloop / max_sommersloop)²
    /// Power usage = Base power usage × Power multiplier × (Clock speed100)^1.321928
    fn total_power_consumption(&self) -> f32 {
        let recipe_info = recipe_info(self.recipe);
        let base_power = recipe_info.machine.base_power_mw();
        let mut total_power = 0.0;
        for group in &self.machine_groups {
            let sommersloop_multiplier = if group.somersloop > 0 {
                1.0 + (group.somersloop as f32 / recipe_info.machine.max_sommersloop() as f32)
            } else {
                1.0
            };
            let power_multiplier = sommersloop_multiplier * sommersloop_multiplier;
            total_power += base_power * power_multiplier * (group.oc_value / 100.0).powf(1.321928);
            total_power *= group.number_of_machine as f32;
        }
        total_power
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

    fn total_sommersloop(&self) -> u32 {
        self.production_lines
            .iter()
            .map(|line| line.total_sommersloop())
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

impl MachineGroup {
    pub fn new(number_of_machines: u32, overclock: f32, sommersloop_per_machine: u8) -> Self {
        Self {
            number_of_machine: number_of_machines,
            oc_value: overclock,
            somersloop: sommersloop_per_machine,
        }
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
        expected = "Cannot add machine group with more sommersloop than the machine type allows"
    )]
    fn test_add_machine_group_invalid_sommersloop() {
        let mut production_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test Line".to_string(),
            None,
            Recipe::IronIngot,
        );
        let machine_group = MachineGroup::new(5, 100.0, 3); // Iron Ingot recipe uses Constructor which allows only 1 sommersloop
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
        // Each machine has 1 sommersloop, max is 1 for Constructor, so power multiplier is (1 + 1/1)² = 4
        // Each machine consumes 4 MW at base power, so total power = 2 machines * 4 MW * 4 = 32 MW
        assert_eq!(production_line.total_power_consumption(), 32.0);
    }
}
