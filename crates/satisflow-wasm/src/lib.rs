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
