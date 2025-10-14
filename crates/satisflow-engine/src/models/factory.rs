use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::models::{logistics::LogisticsFlux, production_line::ProductionLine, Item};

pub struct Factory {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub production_lines: HashMap<u64,Box<dyn ProductionLine>>,

    pub logistics_output: HashMap<u64, Arc<Mutex<LogisticsFlux>>>,
    pub logistics_input: HashMap<u64, Arc<Mutex<LogisticsFlux>>>,

    pub raw_inputs: HashMap<Item, f32>, // Raw inputs available to the factory
    pub items: HashMap<Item, f32>, // Inventory of items in the factory
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
        // Add all raw inputs
        for (item, qty) in &self.raw_inputs {
            *self.items.entry(*item).or_insert(0.0) += qty;
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

