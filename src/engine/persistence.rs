use crate::engine::ProductionTracker;

#[cfg(target_arch = "wasm32")]
use web_sys::{window, Storage};

#[cfg(target_arch = "wasm32")]
const STORAGE_KEY: &str = "satisflow_data";

#[cfg(target_arch = "wasm32")]
fn get_local_storage() -> Result<Storage, Box<dyn std::error::Error>> {
    let window = window().ok_or("No global `window` exists")?;
    let storage = window
        .local_storage()
        .map_err(|_| "Failed to get localStorage")?
        .ok_or("localStorage is not available")?;
    Ok(storage)
}

#[cfg(target_arch = "wasm32")]
pub fn save_tracker(tracker: &ProductionTracker) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(tracker)?;
    let storage = get_local_storage()?;
    storage
        .set_item(STORAGE_KEY, &json)
        .map_err(|_| "Failed to save to localStorage")?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn load_tracker() -> Result<ProductionTracker, Box<dyn std::error::Error>> {
    let storage = get_local_storage()?;
    let json = storage
        .get_item(STORAGE_KEY)
        .map_err(|_| "Failed to read from localStorage")?
        .ok_or("No saved data found")?;
    let tracker: ProductionTracker = serde_json::from_str(&json)?;
    Ok(tracker)
}

// Desktop/non-WASM fallback
#[cfg(not(target_arch = "wasm32"))]
pub fn save_tracker(tracker: &ProductionTracker) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(tracker)?;
    std::fs::write("satisflow_data.json", json)?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_tracker() -> Result<ProductionTracker, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("satisflow_data.json")?;
    let tracker: ProductionTracker = serde_json::from_str(&content)?;
    Ok(tracker)
}
