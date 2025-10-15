use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub mod models;

pub struct SatisflowEngine {
    factories: HashMap<u64, models::factory::Factory>,
    logistics_lines: HashMap<u64, Arc<Mutex<models::logistics::LogisticsFlux>>>,
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
        let line = Arc::new(Mutex::new(line));
        self.logistics_lines.insert(id, line.clone());
        self.factories
            .get_mut(&from)
            .unwrap()
            .logistics_output
            .insert(id, line.clone()); // Safe to unwrap because we checked above
        self.factories
            .get_mut(&to)
            .unwrap()
            .logistics_input
            .insert(id, line.clone()); // Safe to unwrap because we checked above
        Ok(id)
    }

    pub fn get_logistics_line(
        &self,
        id: u64,
    ) -> Option<Arc<Mutex<models::logistics::LogisticsFlux>>> {
        self.logistics_lines.get(&id).cloned()
    }

    pub fn update(&mut self) -> HashMap<models::Item, f32> {
        let mut global_items = HashMap::new();
        self.factories.iter_mut().for_each(|(_id, factory)| {
            // Update each factory
            factory.calculate_item();
            // Aggregate items
            factory.items.iter().for_each(|(item, qty)| {
                *global_items.entry(*item).or_insert(0.0) += qty;
            });
        });
        global_items
    }
}
