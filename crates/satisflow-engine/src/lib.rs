use std::collections::HashMap;
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod examples;
pub mod models;
pub mod version;

use models::{
    factory::Factory,
    logistics::{LogisticsFlux, TransportType},
    FactoryId, Item, LogisticsId, PowerStats,
};

pub use version::{SaveVersion, VersionError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatisflowEngine {
    factories: HashMap<FactoryId, Factory>,
    logistics_lines: HashMap<LogisticsId, LogisticsFlux>,
}

/// Wrapper struct for save files with versioning and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveFile {
    /// Save file format version (using engine crate version)
    pub version: String,
    /// When the save file was first created
    pub created_at: DateTime<Utc>,
    /// When the save file was last modified
    pub last_modified: DateTime<Utc>,
    /// Optional game version tracking
    pub game_version: Option<String>,
    /// The actual engine state
    pub engine: SatisflowEngine,
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

    pub fn create_factory(&mut self, name: String, description: Option<String>) -> FactoryId {
        let id = Uuid::new_v4();
        let factory = Factory::new(id, name, description);
        self.factories.insert(id, factory);
        id
    }

    pub fn get_factory(&self, id: FactoryId) -> Option<&Factory> {
        self.factories.get(&id)
    }

    pub fn get_factory_mut(&mut self, id: FactoryId) -> Option<&mut Factory> {
        self.factories.get_mut(&id)
    }

    pub fn create_logistics_line(
        &mut self,
        from: FactoryId,
        to: FactoryId,
        transport_type: TransportType,
        transport_detail: String,
    ) -> Result<LogisticsId, Box<dyn std::error::Error>> {
        let id = Uuid::new_v4();
        let line = LogisticsFlux {
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

    pub fn update_logistics_line(
        &mut self,
        id: LogisticsId,
        from: FactoryId,
        to: FactoryId,
        transport_type: TransportType,
        transport_detail: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.factories.contains_key(&from) {
            return Err(format!("Factory with id {} does not exist", from).into());
        }
        if !self.factories.contains_key(&to) {
            return Err(format!("Factory with id {} does not exist", to).into());
        }

        let logistics = self
            .logistics_lines
            .get_mut(&id)
            .ok_or_else(|| format!("Logistics line with id {} not found", id))?;

        logistics.from_factory = from;
        logistics.to_factory = to;
        logistics.transport_type = transport_type;
        logistics.transport_details = transport_detail;

        Ok(())
    }

    pub fn get_logistics_line(&self, id: LogisticsId) -> Option<&LogisticsFlux> {
        self.logistics_lines.get(&id)
    }

    pub fn update(&mut self) -> HashMap<Item, f32> {
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
    pub fn global_power_stats(&self) -> PowerStats {
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

        PowerStats::new(total_generation, total_consumption, factory_stats)
    }

    /// Get all factories
    pub fn get_all_factories(&self) -> &HashMap<FactoryId, Factory> {
        &self.factories
    }

    /// Get all logistics lines
    pub fn get_all_logistics(&self) -> &HashMap<LogisticsId, LogisticsFlux> {
        &self.logistics_lines
    }

    /// Delete a factory and its connected logistics lines
    pub fn delete_factory(&mut self, id: FactoryId) -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn delete_logistics_line(
        &mut self,
        id: LogisticsId,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check if logistics line exists
        if !self.logistics_lines.contains_key(&id) {
            return Err(format!("Logistics line with id {} does not exist", id).into());
        }

        self.logistics_lines
            .remove(&id)
            .ok_or("Logistics line not found")?;

        Ok(())
    }

    /// Reset the engine to an empty state (clear all factories and logistics)
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    ///
    /// # Example
    ///
    /// ```
    /// use satisflow_engine::SatisflowEngine;
    ///
    /// let mut engine = SatisflowEngine::new();
    /// engine.create_factory("Test Factory".to_string(), None);
    /// assert_eq!(engine.get_all_factories().len(), 1);
    ///
    /// engine.reset().unwrap();
    /// assert_eq!(engine.get_all_factories().len(), 0);
    /// ```
    pub fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.factories.clear();
        self.logistics_lines.clear();
        Ok(())
    }

    /// Save the engine state to a JSON file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the save file
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    ///
    /// # Example
    ///
    /// ```no_run
    /// use satisflow_engine::SatisflowEngine;
    /// use std::path::Path;
    ///
    /// let engine = SatisflowEngine::new();
    /// engine.save_to_file(Path::new("my_factory.json")).unwrap();
    /// ```
    pub fn save_to_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let save_file = SaveFile::new(self.clone());
        let json = serde_json::to_string_pretty(&save_file)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load the engine state from a JSON file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the save file
    ///
    /// # Returns
    ///
    /// Result containing the loaded engine or an error
    ///
    /// # Example
    ///
    /// ```no_run
    /// use satisflow_engine::SatisflowEngine;
    /// use std::path::Path;
    ///
    /// let engine = SatisflowEngine::load_from_file(Path::new("my_factory.json")).unwrap();
    /// ```
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        Self::load_from_json_with_version_check(&json)
    }

    /// Save to a JSON string (for API usage)
    ///
    /// # Returns
    ///
    /// Result containing the JSON string or an error
    pub fn save_to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        let save_file = SaveFile::new(self.clone());
        let json = serde_json::to_string_pretty(&save_file)?;
        Ok(json)
    }

    /// Load from a JSON string (for API usage)
    ///
    /// # Arguments
    ///
    /// * `json` - JSON string containing the save file
    ///
    /// # Returns
    ///
    /// Result containing the loaded engine or an error
    pub fn load_from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_from_json_with_version_check(json)
    }

    /// Internal method to load with version checking
    fn load_from_json_with_version_check(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // First, parse just to get the version
        let value: serde_json::Value = serde_json::from_str(json)?;

        let file_version_str = value["version"]
            .as_str()
            .ok_or("Missing version field in save file")?;

        let file_version = SaveVersion::parse(file_version_str)?;
        let engine_version = SaveVersion::current();

        // Check version compatibility
        if file_version == engine_version {
            // Exact match - load directly
            let save_file: SaveFile = serde_json::from_str(json)?;
            return Ok(save_file.engine);
        }

        if !file_version.is_compatible_with(&engine_version) {
            // Incompatible major version
            return Err(Box::new(VersionError::Incompatible {
                save_version: file_version.to_string(),
                engine_version: engine_version.to_string(),
            }));
        }

        if file_version.is_newer_than(&engine_version) {
            // Save is from a newer version
            return Err(Box::new(VersionError::SaveTooNew {
                save_version: file_version.to_string(),
                engine_version: engine_version.to_string(),
            }));
        }

        // Save is older but compatible - for now, try to load it
        // In the future, this is where we'll apply migrations
        if file_version.is_older_than(&engine_version) {
            // For now, we'll try to load it and let serde handle defaults
            // TODO: Add proper migration system here when needed
            println!(
                "INFO: Loading save file from older version {} (current: {})",
                file_version, engine_version
            );
            println!("INFO: Migration system not yet implemented - using default values for new fields");
        }

        let save_file: SaveFile = serde_json::from_str(json)?;
        Ok(save_file.engine)
    }
}

impl SaveFile {
    /// Create a new save file from an engine instance
    pub fn new(engine: SatisflowEngine) -> Self {
        let now = Utc::now();
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: now,
            last_modified: now,
            game_version: None,
            engine,
        }
    }

    /// Update the last_modified timestamp
    pub fn update_timestamp(&mut self) {
        self.last_modified = Utc::now();
    }

    /// Get a summary of the save file contents
    pub fn summary(&self) -> SaveFileSummary {
        SaveFileSummary {
            version: self.version.clone(),
            created_at: self.created_at,
            last_modified: self.last_modified,
            factory_count: self.engine.factories.len(),
            logistics_count: self.engine.logistics_lines.len(),
        }
    }
}

/// Summary information about a save file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveFileSummary {
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub factory_count: usize,
    pub logistics_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        logistics::{DroneTransport, TransportType, TruckTransport},
        production_line::{ProductionLine, ProductionLineBlueprint, ProductionLineRecipe},
        Item, Recipe,
    };
    use uuid::Uuid;

    fn uuid_from_u64(value: u64) -> Uuid {
        Uuid::from_u128(value as u128)
    }

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
        let missing_id = uuid_from_u64(999);
        let result = engine.delete_factory(missing_id);

        // Verify error
        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains(&missing_id.to_string()),
            "expected error to reference missing factory id, got: {error}"
        );
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
        let missing_id = uuid_from_u64(9999);
        let result = engine.delete_logistics_line(missing_id);

        // Verify error
        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains(&missing_id.to_string()),
            "expected error to reference missing logistics id, got: {error}"
        );
    }

    #[test]
    fn test_update_logistics_line() {
        let mut engine = SatisflowEngine::new();

        let factory_a = engine.create_factory("Factory A".into(), None);
        let factory_b = engine.create_factory("Factory B".into(), None);
        let factory_c = engine.create_factory("Factory C".into(), None);

        let original_transport = TransportType::Truck(TruckTransport::new(1, Item::IronOre, 60.0));
        let logistics_id = engine
            .create_logistics_line(
                factory_a,
                factory_b,
                original_transport,
                "Initial truck route".into(),
            )
            .expect("should create logistics line");

        let updated_transport =
            TransportType::Drone(DroneTransport::new(5, Item::CircuitBoard, 45.0));

        engine
            .update_logistics_line(
                logistics_id,
                factory_b,
                factory_c,
                updated_transport.clone(),
                "Updated drone route".into(),
            )
            .expect("should update logistics line");

        let updated_line = engine
            .get_logistics_line(logistics_id)
            .expect("logistics line should exist");
        assert_eq!(updated_line.from_factory, factory_b);
        assert_eq!(updated_line.to_factory, factory_c);
        assert_eq!(updated_line.transport_type, updated_transport);
        assert_eq!(updated_line.transport_details, "Updated drone route");
    }

    #[test]
    fn test_production_line_name_method() {
        // Test ProductionLineRecipe
        let recipe_line = ProductionLineRecipe::new(
            uuid_from_u64(1),
            "Test Recipe Line".to_string(),
            Some("Test description".to_string()),
            Recipe::IronIngot,
        );
        let production_line = ProductionLine::ProductionLineRecipe(recipe_line);
        assert_eq!(production_line.name(), "Test Recipe Line");

        // Test ProductionLineBlueprint
        let blueprint_line = ProductionLineBlueprint::new(
            uuid_from_u64(2),
            "Test Blueprint".to_string(),
            Some("Blueprint description".to_string()),
        );
        let production_line = ProductionLine::ProductionLineBlueprint(blueprint_line);
        assert_eq!(production_line.name(), "Test Blueprint");
    }

    #[test]
    fn test_save_load_empty_engine() {
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let save_path = temp_dir.path().join("empty_engine.json");

        // Create and save empty engine
        let engine = SatisflowEngine::new();
        engine.save_to_file(&save_path).unwrap();

        // Verify file exists
        assert!(save_path.exists());

        // Load the engine back
        let loaded_engine = SatisflowEngine::load_from_file(&save_path).unwrap();

        // Verify it's empty
        assert_eq!(loaded_engine.get_all_factories().len(), 0);
        assert_eq!(loaded_engine.get_all_logistics().len(), 0);
    }

    #[test]
    fn test_save_load_with_factories() {
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let save_path = temp_dir.path().join("factories.json");

        // Create engine with factories
        let mut engine = SatisflowEngine::new();
        let factory1_id = engine.create_factory("Factory 1".to_string(), None);
        let factory2_id =
            engine.create_factory("Factory 2".to_string(), Some("Test factory".to_string()));

        // Save to file
        engine.save_to_file(&save_path).unwrap();

        // Load back
        let loaded_engine = SatisflowEngine::load_from_file(&save_path).unwrap();

        // Verify factories were loaded
        assert_eq!(loaded_engine.get_all_factories().len(), 2);
        let loaded_factory1 = loaded_engine.get_factory(factory1_id).unwrap();
        assert_eq!(loaded_factory1.name, "Factory 1");
        assert!(loaded_factory1.description.is_none());

        let loaded_factory2 = loaded_engine.get_factory(factory2_id).unwrap();
        assert_eq!(loaded_factory2.name, "Factory 2");
        assert_eq!(loaded_factory2.description.as_ref().unwrap(), "Test factory");
    }

    #[test]
    fn test_save_load_with_logistics() {
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let save_path = temp_dir.path().join("logistics.json");

        // Create engine with factories and logistics
        let mut engine = SatisflowEngine::new();
        let factory1_id = engine.create_factory("Factory 1".to_string(), None);
        let factory2_id = engine.create_factory("Factory 2".to_string(), None);

        let transport = TransportType::Truck(TruckTransport::new(1, Item::IronOre, 60.0));
        let logistics_id = engine
            .create_logistics_line(
                factory1_id,
                factory2_id,
                transport,
                "Test truck".to_string(),
            )
            .unwrap();

        // Save to file
        engine.save_to_file(&save_path).unwrap();

        // Load back
        let loaded_engine = SatisflowEngine::load_from_file(&save_path).unwrap();

        // Verify logistics were loaded
        assert_eq!(loaded_engine.get_all_logistics().len(), 1);
        let loaded_logistics = loaded_engine.get_logistics_line(logistics_id).unwrap();
        assert_eq!(loaded_logistics.from_factory, factory1_id);
        assert_eq!(loaded_logistics.to_factory, factory2_id);
        assert_eq!(loaded_logistics.transport_details, "Test truck");
    }

    #[test]
    fn test_save_to_json_string() {
        let mut engine = SatisflowEngine::new();
        engine.create_factory("Test Factory".to_string(), None);

        // Save to JSON string
        let json = engine.save_to_json().unwrap();

        // Verify it's valid JSON and contains expected fields
        assert!(json.contains("\"version\""));
        assert!(json.contains("\"created_at\""));
        assert!(json.contains("\"last_modified\""));
        assert!(json.contains("\"engine\""));
        assert!(json.contains("Test Factory"));
    }

    #[test]
    fn test_load_from_json_string() {
        let mut engine = SatisflowEngine::new();
        let factory_id = engine.create_factory("Test Factory".to_string(), None);

        // Save to JSON string
        let json = engine.save_to_json().unwrap();

        // Load from JSON string
        let loaded_engine = SatisflowEngine::load_from_json(&json).unwrap();

        // Verify factory was loaded
        assert_eq!(loaded_engine.get_all_factories().len(), 1);
        let loaded_factory = loaded_engine.get_factory(factory_id).unwrap();
        assert_eq!(loaded_factory.name, "Test Factory");
    }

    #[test]
    fn test_save_file_metadata() {
        let engine = SatisflowEngine::new();
        let save_file = SaveFile::new(engine);

        // Verify metadata
        assert_eq!(save_file.version, env!("CARGO_PKG_VERSION"));
        assert!(save_file.created_at <= Utc::now());
        assert!(save_file.last_modified <= Utc::now());
        assert!(save_file.game_version.is_none());
    }

    #[test]
    fn test_save_file_summary() {
        let mut engine = SatisflowEngine::new();
        engine.create_factory("Factory 1".to_string(), None);
        let factory2_id = engine.create_factory("Factory 2".to_string(), None);
        let factory3_id = engine.create_factory("Factory 3".to_string(), None);

        let transport = TransportType::Truck(TruckTransport::new(1, Item::IronOre, 60.0));
        engine
            .create_logistics_line(
                factory2_id,
                factory3_id,
                transport,
                "Test".to_string(),
            )
            .unwrap();

        let save_file = SaveFile::new(engine);
        let summary = save_file.summary();

        assert_eq!(summary.factory_count, 3);
        assert_eq!(summary.logistics_count, 1);
        assert_eq!(summary.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_save_load_file_not_found() {
        use std::path::PathBuf;

        let missing_path = PathBuf::from("nonexistent_file.json");
        let result = SatisflowEngine::load_from_file(&missing_path);

        assert!(result.is_err());
    }

    #[test]
    fn test_load_invalid_json() {
        let invalid_json = "{ this is not valid json }";
        let result = SatisflowEngine::load_from_json(invalid_json);

        assert!(result.is_err());
    }

    #[test]
    fn test_load_with_same_version() {
        let mut engine = SatisflowEngine::new();
        engine.create_factory("Test".to_string(), None);

        let json = engine.save_to_json().unwrap();
        let loaded = SatisflowEngine::load_from_json(&json).unwrap();

        assert_eq!(loaded.get_all_factories().len(), 1);
    }

    #[test]
    fn test_load_with_future_version() {
        // Create a save file with a future version
        let json = r#"{
            "version": "999.0.0",
            "created_at": "2025-10-25T12:00:00Z",
            "last_modified": "2025-10-25T12:00:00Z",
            "game_version": null,
            "engine": {
                "factories": {},
                "logistics_lines": {}
            }
        }"#;

        let result = SatisflowEngine::load_from_json(json);
        assert!(result.is_err());

        let err = result.unwrap_err();
        let err_msg = err.to_string();
        // Check for either "too new" or "incompatible" (both are correct for future versions)
        assert!(
            err_msg.contains("too new") || err_msg.contains("incompatible"),
            "Expected error about version mismatch, got: {}",
            err_msg
        );
    }

    #[test]
    fn test_load_with_incompatible_major_version() {
        // Create a save file with incompatible major version
        let current_version = SaveVersion::current();
        let incompatible_version = format!("{}.0.0", current_version.major + 1);

        let json = format!(
            r#"{{
            "version": "{}",
            "created_at": "2025-10-25T12:00:00Z",
            "last_modified": "2025-10-25T12:00:00Z",
            "game_version": null,
            "engine": {{
                "factories": {{}},
                "logistics_lines": {{}}
            }}
        }}"#,
            incompatible_version
        );

        let result = SatisflowEngine::load_from_json(&json);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.to_string().contains("incompatible"));
    }

    #[test]
    fn test_load_missing_version() {
        let json = r#"{
            "created_at": "2025-10-25T12:00:00Z",
            "last_modified": "2025-10-25T12:00:00Z",
            "game_version": null,
            "engine": {
                "factories": {},
                "logistics_lines": {}
            }
        }"#;

        let result = SatisflowEngine::load_from_json(json);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.to_string().contains("Missing version"));
    }

    #[test]
    fn test_load_invalid_version_format() {
        let json = r#"{
            "version": "not-a-version",
            "created_at": "2025-10-25T12:00:00Z",
            "last_modified": "2025-10-25T12:00:00Z",
            "game_version": null,
            "engine": {
                "factories": {},
                "logistics_lines": {}
            }
        }"#;

        let result = SatisflowEngine::load_from_json(json);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.to_string().contains("Invalid version format"));
    }

    #[test]
    fn test_reset_engine() {
        let mut engine = SatisflowEngine::new();

        // Create factories and logistics
        let factory1_id = engine.create_factory("Factory 1".to_string(), None);
        let factory2_id = engine.create_factory("Factory 2".to_string(), None);

        let transport = TransportType::Truck(TruckTransport::new(1, Item::IronOre, 60.0));
        engine
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

        // Reset the engine
        engine.reset().unwrap();

        // Verify engine is empty
        assert_eq!(engine.get_all_factories().len(), 0);
        assert_eq!(engine.get_all_logistics().len(), 0);
    }

    #[test]
    fn test_reset_empty_engine() {
        let mut engine = SatisflowEngine::new();

        // Reset empty engine (should not fail)
        let result = engine.reset();
        assert!(result.is_ok());

        // Verify still empty
        assert_eq!(engine.get_all_factories().len(), 0);
        assert_eq!(engine.get_all_logistics().len(), 0);
    }
}
