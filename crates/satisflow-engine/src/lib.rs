use std::collections::HashMap;

pub mod examples;
pub mod models;

pub struct SatisflowEngine {
    factories: HashMap<u64, models::factory::Factory>,
    logistics_lines: HashMap<u64, models::logistics::LogisticsFlux>,
}

impl Default for SatisflowEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SatisflowEngine {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
            logistics_lines: HashMap::new(),
        }
    }

    pub fn create_factory(&mut self, name: String, description: Option<String>) -> u64 {
        let id = self.factories.len() as u64 + 1;
        let factory = models::factory::Factory::new(id as u32, name, description);
        self.factories.insert(id, factory);
        id
    }

    pub fn get_factory(&self, id: u64) -> Option<&models::factory::Factory> {
        self.factories.get(&id)
    }

    pub fn get_factory_mut(&mut self, id: u64) -> Option<&mut models::factory::Factory> {
        self.factories.get_mut(&id)
    }

    pub fn create_logistics_line(
        &mut self,
        from: u64,
        to: u64,
        transport_type: models::logistics::TransportType,
        transport_detail: String,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let id = self.logistics_lines.len() as u64 + 1;
        let line = models::logistics::LogisticsFlux {
            id,
            from_factory: from,
            to_factory: to,
            transport_type,
            transport_details: transport_detail,
        };

        //check that from and to factories exist
        if !self.factories.contains_key(&from) {
            return Err(format!("Factory with id {} does not exist", from).into());
        }
        if !self.factories.contains_key(&to) {
            return Err(format!("Factory with id {} does not exist", to).into());
        }

        self.logistics_lines.insert(id, line);
        Ok(id)
    }

    pub fn get_logistics_line(&self, id: u64) -> Option<&models::logistics::LogisticsFlux> {
        self.logistics_lines.get(&id)
    }

    pub fn update(&mut self) -> HashMap<models::Item, f32> {
        let mut global_items = HashMap::new();
        self.factories.iter_mut().for_each(|(_id, factory)| {
            // Update each factory
            factory.calculate_item(&self.logistics_lines);
            // Aggregate items
            factory.items.iter().for_each(|(item, qty)| {
                *global_items.entry(*item).or_insert(0.0) += qty;
            });
        });
        global_items
    }

    /// Get global power statistics for all factories
    pub fn global_power_stats(&self) -> models::PowerStats {
        let mut total_generation = 0.0;
        let mut total_consumption = 0.0;
        let mut factory_stats = Vec::new();

        for (factory_id, factory) in &self.factories {
            let generation = factory.total_power_generation();
            let consumption = factory.total_power_consumption();
            let generator_count = factory.power_generators.len() as u32;

            // Collect unique generator types
            let mut generator_types = std::collections::HashSet::new();
            for generator in factory.power_generators.values() {
                generator_types.insert(generator.generator_type);
            }
            let generator_types: Vec<_> = generator_types.into_iter().collect();

            let factory_stat = models::FactoryPowerStats::new(
                *factory_id,
                factory.name.clone(),
                generation,
                consumption,
                generator_count,
                generator_types,
            );

            total_generation += generation;
            total_consumption += consumption;
            factory_stats.push(factory_stat);
        }

        models::PowerStats::new(total_generation, total_consumption, factory_stats)
    }

    /// Get all factories
    pub fn get_all_factories(&self) -> &HashMap<u64, models::factory::Factory> {
        &self.factories
    }

    /// Get all logistics lines
    pub fn get_all_logistics(&self) -> &HashMap<u64, models::logistics::LogisticsFlux> {
        &self.logistics_lines
    }

    /// Delete a factory and its connected logistics lines
    pub fn delete_factory(&mut self, id: u64) -> Result<(), Box<dyn std::error::Error>> {
        // Check if factory exists
        if !self.factories.contains_key(&id) {
            return Err(format!("Factory with id {} does not exist", id).into());
        }

        // Remove all logistics lines connected to this factory
        self.logistics_lines
            .retain(|_, logistics| logistics.from_factory != id && logistics.to_factory != id);

        // Remove the factory
        self.factories.remove(&id).ok_or("Factory not found")?;

        Ok(())
    }

    /// Delete a logistics line
    pub fn delete_logistics_line(&mut self, id: u64) -> Result<(), Box<dyn std::error::Error>> {
        // Check if logistics line exists
        if !self.logistics_lines.contains_key(&id) {
            return Err(format!("Logistics line with id {} does not exist", id).into());
        }

        self.logistics_lines
            .remove(&id)
            .ok_or("Logistics line not found")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        logistics::{LogisticsFlux, TransportType, TruckTransport},
        production_line::{
            MachineGroup, ProductionLine, ProductionLineBlueprint, ProductionLineRecipe,
        },
        Item, Recipe,
    };

    #[test]
    fn test_get_all_factories() {
        let mut engine = SatisflowEngine::new();

        // Create some factories
        let factory1_id = engine.create_factory("Factory 1".to_string(), None);
        let factory2_id =
            engine.create_factory("Factory 2".to_string(), Some("Test factory".to_string()));

        // Get all factories
        let all_factories = engine.get_all_factories();

        // Verify all factories are returned
        assert_eq!(all_factories.len(), 2);
        assert!(all_factories.contains_key(&factory1_id));
        assert!(all_factories.contains_key(&factory2_id));

        // Verify factory details
        let factory1 = all_factories.get(&factory1_id).unwrap();
        assert_eq!(factory1.name, "Factory 1");
        assert!(factory1.description.is_none());

        let factory2 = all_factories.get(&factory2_id).unwrap();
        assert_eq!(factory2.name, "Factory 2");
        assert_eq!(factory2.description.as_ref().unwrap(), "Test factory");
    }

    #[test]
    fn test_get_all_logistics() {
        let mut engine = SatisflowEngine::new();

        // Create factories first
        let factory1_id = engine.create_factory("Factory 1".to_string(), None);
        let factory2_id = engine.create_factory("Factory 2".to_string(), None);

        // Create some logistics lines
        let transport1 = TransportType::Truck(TruckTransport::new(1, Item::IronOre, 60.0));
        let logistics1_id = engine
            .create_logistics_line(
                factory1_id,
                factory2_id,
                transport1,
                "Test truck".to_string(),
            )
            .unwrap();

        let transport2 = TransportType::Truck(TruckTransport::new(2, Item::CopperOre, 120.0));
        let logistics2_id = engine
            .create_logistics_line(
                factory2_id,
                factory1_id,
                transport2,
                "Test truck 2".to_string(),
            )
            .unwrap();

        // Get all logistics
        let all_logistics = engine.get_all_logistics();

        // Verify all logistics lines are returned
        assert_eq!(all_logistics.len(), 2);
        assert!(all_logistics.contains_key(&logistics1_id));
        assert!(all_logistics.contains_key(&logistics2_id));

        // Verify logistics details
        let logistics1 = all_logistics.get(&logistics1_id).unwrap();
        assert_eq!(logistics1.from_factory, factory1_id);
        assert_eq!(logistics1.to_factory, factory2_id);
        assert_eq!(logistics1.transport_details, "Test truck");
    }

    #[test]
    fn test_delete_factory() {
        let mut engine = SatisflowEngine::new();

        // Create factories
        let factory1_id = engine.create_factory("Factory 1".to_string(), None);
        let factory2_id = engine.create_factory("Factory 2".to_string(), None);

        // Create logistics line between factories
        let transport = TransportType::Truck(TruckTransport::new(1, Item::IronOre, 60.0));
        let logistics_id = engine
            .create_logistics_line(
                factory1_id,
                factory2_id,
                transport,
                "Test truck".to_string(),
            )
            .unwrap();

        // Verify initial state
        assert_eq!(engine.get_all_factories().len(), 2);
        assert_eq!(engine.get_all_logistics().len(), 1);

        // Delete factory
        engine.delete_factory(factory1_id).unwrap();

        // Verify factory is deleted
        assert_eq!(engine.get_all_factories().len(), 1);
        assert!(!engine.get_all_factories().contains_key(&factory1_id));
        assert!(engine.get_all_factories().contains_key(&factory2_id));

        // Verify connected logistics lines are also deleted
        assert_eq!(engine.get_all_logistics().len(), 0);
        assert!(!engine.get_all_logistics().contains_key(&logistics_id));
    }

    #[test]
    fn test_delete_factory_not_found() {
        let mut engine = SatisflowEngine::new();

        // Try to delete non-existent factory
        let result = engine.delete_factory(999);

        // Verify error
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Factory with id 999 does not exist"));
    }

    #[test]
    fn test_delete_logistics_line() {
        let mut engine = SatisflowEngine::new();

        // Create factories
        let factory1_id = engine.create_factory("Factory 1".to_string(), None);
        let factory2_id = engine.create_factory("Factory 2".to_string(), None);

        // Create logistics line
        let transport = TransportType::Truck(TruckTransport::new(1, Item::IronOre, 60.0));
        let logistics_id = engine
            .create_logistics_line(
                factory1_id,
                factory2_id,
                transport,
                "Test truck".to_string(),
            )
            .unwrap();

        // Verify initial state
        assert_eq!(engine.get_all_logistics().len(), 1);

        // Delete logistics line
        engine.delete_logistics_line(logistics_id).unwrap();

        // Verify logistics line is deleted
        assert_eq!(engine.get_all_logistics().len(), 0);
        assert!(!engine.get_all_logistics().contains_key(&logistics_id));

        // Verify factories are still there
        assert_eq!(engine.get_all_factories().len(), 2);
    }

    #[test]
    fn test_delete_logistics_line_not_found() {
        let mut engine = SatisflowEngine::new();

        // Try to delete non-existent logistics line
        let result = engine.delete_logistics_line(999);

        // Verify error
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Logistics line with id 999 does not exist"));
    }

    #[test]
    fn test_production_line_name_method() {
        // Test ProductionLineRecipe
        let recipe_line = ProductionLineRecipe::new(
            1,
            "Test Recipe Line".to_string(),
            Some("Test description".to_string()),
            Recipe::IronIngot,
        );
        let production_line = ProductionLine::ProductionLineRecipe(recipe_line);
        assert_eq!(production_line.name(), "Test Recipe Line");

        // Test ProductionLineBlueprint
        let blueprint_line = ProductionLineBlueprint::new(
            2,
            "Test Blueprint".to_string(),
            Some("Blueprint description".to_string()),
        );
        let production_line = ProductionLine::ProductionLineBlueprint(blueprint_line);
        assert_eq!(production_line.name(), "Test Blueprint");
    }
}
