use crate::engine::ProductionTracker;
use serde::{Deserialize, Serialize};

// Versioned save envelope per design
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrackerSave {
    pub version: String,
    pub tracker: ProductionTracker,
}

const CURRENT_SAVE_VERSION: &str = "1";

impl TrackerSave {
    pub fn new(tracker: ProductionTracker) -> Self {
        Self {
            version: CURRENT_SAVE_VERSION.to_string(),
            tracker,
        }
    }
}

pub fn save_tracker(tracker: &ProductionTracker) -> Result<(), Box<dyn std::error::Error>> {
    let save = TrackerSave::new(tracker.clone());
    let json = serde_json::to_string_pretty(&save)?;
    std::fs::write("satisflow_data.json", json)?;
    Ok(())
}

pub fn load_tracker() -> Result<ProductionTracker, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("satisflow_data.json")?;
    // Try versioned first, fallback to legacy for backward compatibility
    if let Ok(save) = serde_json::from_str::<TrackerSave>(&content) {
        Ok(save.tracker)
    } else {
        let tracker: ProductionTracker = serde_json::from_str(&content)?;
        Ok(tracker)
    }
}

// Helpers for tests/consumers who want to serialize without IO
pub fn to_save_json(tracker: &ProductionTracker) -> Result<String, Box<dyn std::error::Error>> {
    let save = TrackerSave::new(tracker.clone());
    Ok(serde_json::to_string_pretty(&save)?)
}

pub fn from_save_json(s: &str) -> Result<ProductionTracker, Box<dyn std::error::Error>> {
    if let Ok(save) = serde_json::from_str::<TrackerSave>(s) {
        Ok(save.tracker)
    } else {
        let tracker: ProductionTracker = serde_json::from_str(s)?;
        Ok(tracker)
    }
}
