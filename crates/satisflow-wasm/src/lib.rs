use satisflow_engine::{Factory, ProductionTracker};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Setup panic hook for better error messages in WASM
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// WASM wrapper for the ProductionTracker
#[wasm_bindgen]
pub struct WebTracker {
    inner: ProductionTracker,
}

#[wasm_bindgen]
impl WebTracker {
    /// Create a new WebTracker instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebTracker {
        WebTracker {
            inner: ProductionTracker::new(),
        }
    }

    /// Load static game data (items and recipes)
    #[wasm_bindgen]
    pub fn load_static_data(&mut self) {
        self.inner.load_static_data();
    }

    /// Load a sample factory setup (from init.rs) for demo/testing
    #[wasm_bindgen]
    pub fn load_sample_data(&mut self) {
        self.inner = satisflow_engine::init::create_sample_factory_setup();
    }

    /// Get production overview as JavaScript value
    #[wasm_bindgen]
    pub fn get_overview(&self) -> Result<JsValue, JsValue> {
        let overview = self.inner.calculate_overview();
        to_value(&overview).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Add a factory from JavaScript data
    #[wasm_bindgen]
    pub fn add_factory(&mut self, factory_data: &JsValue) -> Result<(), JsValue> {
        let factory: Factory =
            from_value(factory_data.clone()).map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.inner.add_factory(factory);
        Ok(())
    }

    /// Export tracker data as JSON string
    #[wasm_bindgen]
    pub fn export_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.inner).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Import tracker data from JSON string
    #[wasm_bindgen]
    pub fn import_json(&mut self, json_data: &str) -> Result<(), JsValue> {
        let tracker: ProductionTracker = serde_json::from_str(json_data)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;

        self.inner = tracker;
        Ok(())
    }

    /// Get factory count
    #[wasm_bindgen]
    pub fn get_factory_count(&self) -> u32 {
        self.inner.factories.len() as u32
    }

    /// Get total production line count across all factories
    #[wasm_bindgen]
    pub fn get_production_line_count(&self) -> u32 {
        self.inner
            .factories
            .values()
            .map(|f| f.production_lines.len() as u32)
            .sum()
    }

    /// Get the number of game items loaded
    #[wasm_bindgen]
    pub fn get_item_count(&self) -> u32 {
        self.inner.items.len() as u32
    }

    /// Get factories list as JavaScript value
    #[wasm_bindgen]
    pub fn get_factories(&self) -> Result<JsValue, JsValue> {
        // Convert the factories HashMap to a Vec for easier JS consumption
        let factories_vec: Vec<&Factory> = self.inner.factories.values().collect();
        to_value(&factories_vec).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get available recipes for dropdowns
    #[wasm_bindgen]
    pub fn get_recipes(&self) -> Result<JsValue, JsValue> {
        if let Some(index) = &self.inner.index {
            let recipes: Vec<_> = index.recipes_by_id.values().collect();
            to_value(&recipes).map_err(|e| JsValue::from_str(&e.to_string()))
        } else {
            // Fallback to old recipes if index not available
            let recipes: Vec<_> = self.inner.recipes.values().collect();
            to_value(&recipes).map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }

    /// Get available items for dropdowns
    #[wasm_bindgen]
    pub fn get_items(&self) -> Result<JsValue, JsValue> {
        if let Some(index) = &self.inner.index {
            let items: Vec<_> = index.items_by_id.values().collect();
            to_value(&items).map_err(|e| JsValue::from_str(&e.to_string()))
        } else {
            // Fallback to old items if index not available
            let items: Vec<_> = self.inner.items.values().collect();
            to_value(&items).map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }

    /// Create a new factory with given name and return the generated factory data
    #[wasm_bindgen]
    pub fn create_factory(&mut self, name: &str) -> Result<JsValue, JsValue> {
        use satisflow_engine::{Factory, FactoryId};
        
        if self.inner.factory_name_exists(name) {
            return Err(JsValue::from_str(&format!("Factory name '{}' already exists", name)));
        }

        let factory_id = self.inner.generate_factory_id(name);
        let factory = Factory {
            id: factory_id,
            name: name.to_string(),
            raw_inputs: vec![],
            logistics_inputs: vec![],
            production_lines: vec![],
        };

        let factory_js = to_value(&factory).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.inner.add_factory(factory);
        Ok(factory_js)
    }

    /// Add a production line to a factory
    #[wasm_bindgen]
    pub fn add_production_line(&mut self, line_data: &JsValue) -> Result<(), JsValue> {
        use satisflow_engine::*;
        use serde_json::Value;
        
        // First deserialize to a generic JSON value
        let mut json: Value = from_value(line_data.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
            
        // Convert string fields to proper newtype wrappers
        if let Value::Object(ref mut obj) = json {
            if let Some(Value::String(id)) = obj.get("id").cloned() {
                obj["id"] = serde_json::json!(ProductionLineId(id));
            }
            if let Some(Value::String(factory_id)) = obj.get("factory_id").cloned() {
                obj["factory_id"] = serde_json::json!(FactoryId(factory_id));
            }
            if let Some(Value::String(recipe_id)) = obj.get("recipe_id").cloned() {
                obj["recipe_id"] = serde_json::json!(RecipeId(recipe_id));
            }
        }
        
        let line: ProductionLine = serde_json::from_value(json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.inner.add_production_line(line)
            .map_err(|e| JsValue::from_str(&e))
    }

    /// Add a logistics flux connection
    #[wasm_bindgen]
    pub fn add_logistics_flux(&mut self, flux_data: &JsValue) -> Result<(), JsValue> {
        use satisflow_engine::*;
        use serde_json::Value;
        
        // First deserialize to a generic JSON value
        let mut json: Value = from_value(flux_data.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
            
        // Convert string fields to proper newtype wrappers
        if let Value::Object(ref mut obj) = json {
            if let Some(Value::String(id)) = obj.get("id").cloned() {
                obj["id"] = serde_json::json!(LogisticsFluxId(id));
            }
            if let Some(Value::String(from_factory)) = obj.get("from_factory").cloned() {
                obj["from_factory"] = serde_json::json!(FactoryId(from_factory));
            }
            if let Some(Value::String(to_factory)) = obj.get("to_factory").cloned() {
                obj["to_factory"] = serde_json::json!(FactoryId(to_factory));
            }
            if let Some(Value::String(item)) = obj.get("item").cloned() {
                obj["item"] = serde_json::json!(ItemId(item));
            }
        }
        
        let flux: LogisticsFlux = serde_json::from_value(json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.inner.add_logistics_flux(flux)
            .map_err(|e| JsValue::from_str(&e))
    }

    /// Generate a unique production line ID for a factory
    #[wasm_bindgen]
    pub fn generate_line_id(&self, factory_id: &str, recipe_name: &str) -> String {
        use satisflow_engine::FactoryId;
        let fid = FactoryId(factory_id.to_string());
        self.inner.generate_line_id(&fid, recipe_name).0
    }

    /// Get all logistics flux connections
    #[wasm_bindgen]
    pub fn get_logistics_fluxes(&self) -> Result<JsValue, JsValue> {
        let fluxes: Vec<_> = self.inner.logistics_fluxes.values().collect();
        to_value(&fluxes).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Update a logistics flux connection
    #[wasm_bindgen]
    pub fn update_logistics_flux(&mut self, flux_id: &str, flux_data: &JsValue) -> Result<(), JsValue> {
        use satisflow_engine::*;
        use serde_json::Value;
        
        // First deserialize to a generic JSON value
        let mut json: Value = from_value(flux_data.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
            
        // Convert string fields to proper newtype wrappers
        if let Value::Object(ref mut obj) = json {
            if let Some(Value::String(id)) = obj.get("id").cloned() {
                obj["id"] = serde_json::json!(LogisticsFluxId(id));
            }
            if let Some(Value::String(from_factory)) = obj.get("from_factory").cloned() {
                obj["from_factory"] = serde_json::json!(FactoryId(from_factory));
            }
            if let Some(Value::String(to_factory)) = obj.get("to_factory").cloned() {
                obj["to_factory"] = serde_json::json!(FactoryId(to_factory));
            }
            if let Some(Value::String(item)) = obj.get("item").cloned() {
                obj["item"] = serde_json::json!(ItemId(item));
            }
        }
        
        let flux: LogisticsFlux = serde_json::from_value(json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let flux_id_typed = LogisticsFluxId(flux_id.to_string());
        self.inner.update_logistics_flux(&flux_id_typed, flux)
            .map_err(|e| JsValue::from_str(&e))
    }

    /// Remove a logistics flux connection
    #[wasm_bindgen]
    pub fn remove_logistics_flux(&mut self, flux_id: &str) -> Result<(), JsValue> {
        use satisflow_engine::*;
        
        let flux_id_typed = LogisticsFluxId(flux_id.to_string());
        self.inner.remove_logistics_flux(&flux_id_typed)
            .map_err(|e| JsValue::from_str(&e))
    }

    /// Generate a unique logistics ID
    #[wasm_bindgen]
    pub fn generate_logistics_id(&self, transport_type: &str) -> String {
        use satisflow_engine::TransportType;
        let t_type = match transport_type {
            "Conveyor" => TransportType::Conveyor,
            "Train" => TransportType::Train,
            "Truck" => TransportType::Truck,
            "Drone" => TransportType::Drone,
            _ => TransportType::Conveyor, // default
        };
        self.inner.generate_logistics_id(&t_type).0
    }

    /// Add a raw input to a factory
    #[wasm_bindgen]
    pub fn add_raw_input(&mut self, factory_id: &str, raw_input_data: &JsValue) -> Result<(), JsValue> {
        use satisflow_engine::*;
        use serde_json::Value;
        
        // First deserialize to a generic JSON value
        let mut json: Value = from_value(raw_input_data.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
            
        // Convert string fields to proper newtype wrappers
        if let Value::Object(ref mut obj) = json {
            if let Some(Value::String(item)) = obj.get("item").cloned() {
                obj["item"] = serde_json::json!(ItemId(item));
            }
        }
        
        let raw_input: RawInput = serde_json::from_value(json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let factory_id_typed = FactoryId(factory_id.to_string());
        self.inner.add_raw_input_to_factory(&factory_id_typed, raw_input)
            .map_err(|e| JsValue::from_str(&e))
    }

    /// Remove a raw input from a factory
    #[wasm_bindgen]
    pub fn remove_raw_input(&mut self, factory_id: &str, item_id: &str) -> Result<(), JsValue> {
        use satisflow_engine::*;
        
        let factory_id_typed = FactoryId(factory_id.to_string());
        let item_id_typed = ItemId(item_id.to_string());
        
        self.inner.remove_raw_input_from_factory(&factory_id_typed, &item_id_typed)
            .map_err(|e| JsValue::from_str(&e))
    }

    /// Update a production line in a factory
    #[wasm_bindgen]
    pub fn update_production_line(&mut self, line_id: &str, production_line_data: &JsValue) -> Result<(), JsValue> {
        use satisflow_engine::*;
        use serde_json::Value;
        
        // First deserialize to a generic JSON value
        let mut json: Value = from_value(production_line_data.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
            
        // Convert string fields to proper newtype wrappers
        if let Value::Object(ref mut obj) = json {
            if let Some(Value::String(id)) = obj.get("id").cloned() {
                obj["id"] = serde_json::json!(ProductionLineId(id));
            }
            if let Some(Value::String(factory_id)) = obj.get("factory_id").cloned() {
                obj["factory_id"] = serde_json::json!(FactoryId(factory_id));
            }
            if let Some(Value::String(recipe_id)) = obj.get("recipe_id").cloned() {
                obj["recipe_id"] = serde_json::json!(RecipeId(recipe_id));
            }
        }
        
        let line: ProductionLine = serde_json::from_value(json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let line_id_typed = ProductionLineId(line_id.to_string());
        self.inner.update_production_line(&line_id_typed, line)
            .map_err(|e| JsValue::from_str(&e))
    }

    /// Update a raw input in a factory
    #[wasm_bindgen]
    pub fn update_raw_input(&mut self, factory_id: &str, item_id: &str, raw_input_data: &JsValue) -> Result<(), JsValue> {
        use satisflow_engine::*;
        use serde_json::Value;
        
        // First deserialize to a generic JSON value
        let mut json: Value = from_value(raw_input_data.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
            
        // Convert string fields to proper newtype wrappers
        if let Value::Object(ref mut obj) = json {
            if let Some(Value::String(item)) = obj.get("item").cloned() {
                obj["item"] = serde_json::json!(ItemId(item));
            }
        }
        
        let raw_input: RawInput = serde_json::from_value(json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let factory_id_typed = FactoryId(factory_id.to_string());
        let item_id_typed = ItemId(item_id.to_string());
        
        self.inner.update_raw_input_in_factory(&factory_id_typed, &item_id_typed, raw_input)
            .map_err(|e| JsValue::from_str(&e))
    }
}

/// Utility functions for the frontend
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn console_log(message: &str) {
    log(message);
}
