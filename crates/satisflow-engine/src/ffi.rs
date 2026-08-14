//! FFI-safe wrapper functions for the Satisflow engine.
//!
//! These functions provide a C-compatible interface for Flutter FFI bindings.
//! All UUIDs are passed as Strings across the FFI boundary.
//! All returned data is cloned to ensure ownership transfer.
//!
//! # FFI Signature Notes
//!
//! - Mutation operations take `&mut SatisflowEngine` (required for HashMap inserts)
//! - Read operations take `&SatisflowEngine`
//! - All UUID parameters are passed as Strings and parsed internally
//! - All returned collections clone data for ownership

use std::collections::HashMap;

use crate::models::{
    logistics::{LogisticsFlux, TransportType},
    production_line::{EngineError, ProductionLineBlueprint},
    Factory, FactoryId, LogisticsId, PowerStats, ProductionLineId,
};
use crate::SatisflowEngine;

/// Result type alias for FFI operations
pub type FfiResult<T> = Result<T, crate::SatisflowError>;

// ============================================================================
// Factory Operations
// ============================================================================

/// Create a new factory and return its ID as a String.
pub fn ffi_create_factory(
    engine: &mut SatisflowEngine,
    name: String,
    description: Option<String>,
) -> FfiResult<String> {
    let factory_id = engine.create_factory(name, description);
    Ok(factory_id.to_string())
}

/// Get a factory by ID, returning a cloned Factory if found.
pub fn ffi_get_factory(engine: &SatisflowEngine, id: String) -> Option<Factory> {
    let factory_id = FactoryId::parse_str(&id).ok()?;
    engine.get_factory(factory_id).cloned()
}

/// Get all factories as a HashMap with String keys (UUIDs converted).
pub fn ffi_get_all_factories(engine: &SatisflowEngine) -> HashMap<String, Factory> {
    engine
        .get_all_factories()
        .iter()
        .map(|(id, factory)| (id.to_string(), factory.clone()))
        .collect()
}

/// Delete a factory by ID.
pub fn ffi_delete_factory(engine: &mut SatisflowEngine, id: String) -> FfiResult<()> {
    let factory_id =
        FactoryId::parse_str(&id).map_err(|_| crate::SatisflowError::InvalidInput {
            message: format!("Invalid factory ID format: {}", id),
        })?;
    engine.delete_factory(factory_id)
}

// ============================================================================
// Logistics Operations
// ============================================================================

/// Create a logistics line between two factories and return the new line's ID as a String.
pub fn ffi_create_logistics_line(
    engine: &mut SatisflowEngine,
    from_id: String,
    to_id: String,
    transport_type: TransportType,
    transport_detail: String,
) -> FfiResult<String> {
    let from = FactoryId::parse_str(&from_id).map_err(|_| crate::SatisflowError::InvalidInput {
        message: format!("Invalid from_factory ID format: {}", from_id),
    })?;
    let to = FactoryId::parse_str(&to_id).map_err(|_| crate::SatisflowError::InvalidInput {
        message: format!("Invalid to_factory ID format: {}", to_id),
    })?;

    let logistics_id = engine.create_logistics_line(from, to, transport_type, transport_detail)?;
    Ok(logistics_id.to_string())
}

/// Update an existing logistics line.
pub fn ffi_update_logistics_line(
    engine: &mut SatisflowEngine,
    id: String,
    from_id: String,
    to_id: String,
    transport_type: TransportType,
    transport_detail: String,
) -> FfiResult<()> {
    let logistics_id =
        LogisticsId::parse_str(&id).map_err(|_| crate::SatisflowError::InvalidInput {
            message: format!("Invalid logistics ID format: {}", id),
        })?;
    let from = FactoryId::parse_str(&from_id).map_err(|_| crate::SatisflowError::InvalidInput {
        message: format!("Invalid from_factory ID format: {}", from_id),
    })?;
    let to = FactoryId::parse_str(&to_id).map_err(|_| crate::SatisflowError::InvalidInput {
        message: format!("Invalid to_factory ID format: {}", to_id),
    })?;

    engine.update_logistics_line(logistics_id, from, to, transport_type, transport_detail)
}

/// Get a logistics line by ID, returning a cloned LogisticsFlux if found.
pub fn ffi_get_logistics_line(engine: &SatisflowEngine, id: String) -> Option<LogisticsFlux> {
    let logistics_id = LogisticsId::parse_str(&id).ok()?;
    engine.get_logistics_line(logistics_id).cloned()
}

/// Get all logistics lines as a HashMap with String keys (UUIDs converted).
pub fn ffi_get_all_logistics(engine: &SatisflowEngine) -> HashMap<String, LogisticsFlux> {
    engine
        .get_all_logistics()
        .iter()
        .map(|(id, flux)| (id.to_string(), flux.clone()))
        .collect()
}

/// Delete a logistics line by ID.
pub fn ffi_delete_logistics_line(engine: &mut SatisflowEngine, id: String) -> FfiResult<()> {
    let logistics_id =
        LogisticsId::parse_str(&id).map_err(|_| crate::SatisflowError::InvalidInput {
            message: format!("Invalid logistics ID format: {}", id),
        })?;
    engine.delete_logistics_line(logistics_id)
}

// ============================================================================
// Engine State Operations
// ============================================================================

/// Reset the engine to an empty state (clear all factories and logistics).
pub fn ffi_reset(engine: &mut SatisflowEngine) -> FfiResult<()> {
    engine.reset()
}

/// Get global power statistics for all factories.
pub fn ffi_global_power_stats(engine: &SatisflowEngine) -> PowerStats {
    engine.global_power_stats()
}

/// Run one update cycle and return global item balances.
/// Returns a HashMap with item names as String keys.
pub fn ffi_update(engine: &mut SatisflowEngine) -> HashMap<String, f32> {
    engine
        .update()
        .into_iter()
        .map(|(item, qty)| (item.to_string(), qty))
        .collect()
}

// ============================================================================
// Blueprint Template Operations
// ============================================================================

/// Add a blueprint template and return its ID as a String.
pub fn ffi_add_blueprint_template(
    engine: &mut SatisflowEngine,
    blueprint: ProductionLineBlueprint,
) -> String {
    engine.add_blueprint_template(blueprint).to_string()
}

/// Get a blueprint template by ID, returning a cloned template if found.
pub fn ffi_get_blueprint_template(
    engine: &SatisflowEngine,
    id: String,
) -> Option<ProductionLineBlueprint> {
    let blueprint_id = ProductionLineId::parse_str(&id).ok()?;
    engine.get_blueprint_template(blueprint_id).cloned()
}

/// Get all blueprint templates as a HashMap with String keys (UUIDs converted).
pub fn ffi_get_all_blueprint_templates(
    engine: &SatisflowEngine,
) -> HashMap<String, ProductionLineBlueprint> {
    engine
        .get_all_blueprint_templates()
        .iter()
        .map(|(id, blueprint)| (id.to_string(), blueprint.clone()))
        .collect()
}

/// Remove a blueprint template by ID.
pub fn ffi_remove_blueprint_template(engine: &mut SatisflowEngine, id: String) -> FfiResult<()> {
    let blueprint_id =
        ProductionLineId::parse_str(&id).map_err(|_| crate::SatisflowError::InvalidInput {
            message: format!("Invalid blueprint ID format: {}", id),
        })?;
    engine.remove_blueprint_template(blueprint_id)
}

/// Update a blueprint template's name and description.
pub fn ffi_update_blueprint_template(
    engine: &mut SatisflowEngine,
    id: String,
    name: String,
    description: String,
) -> FfiResult<()> {
    let blueprint_id =
        ProductionLineId::parse_str(&id).map_err(|_| crate::SatisflowError::InvalidInput {
            message: format!("Invalid blueprint ID format: {}", id),
        })?;
    engine
        .update_blueprint_template(blueprint_id, name, description)
        .map_err(|e| match e {
            EngineError::EmptyName => crate::SatisflowError::InvalidInput {
                message: "Blueprint name cannot be empty".to_string(),
            },
            EngineError::TemplateNotFound { id } => {
                crate::SatisflowError::BlueprintNotFound { id: id.to_string() }
            }
            EngineError::IndexOutOfBounds { index, length } => {
                crate::SatisflowError::InvalidInput {
                    message: format!("Index {} out of bounds for {} elements", index, length),
                }
            }
        })
}

/// Instantiate a blueprint template into a factory.
/// Returns a tuple of (instance_id, instance_name) as Strings.
pub fn ffi_instantiate_blueprint_into_factory(
    engine: &mut SatisflowEngine,
    factory_id_str: String,
    blueprint_id_str: String,
    custom_name: Option<String>,
) -> FfiResult<(String, String)> {
    let factory_id =
        FactoryId::parse_str(&factory_id_str).map_err(|_| crate::SatisflowError::InvalidInput {
            message: format!("Invalid factory ID format: {}", factory_id_str),
        })?;
    let blueprint_id = ProductionLineId::parse_str(&blueprint_id_str).map_err(|_| {
        crate::SatisflowError::InvalidInput {
            message: format!("Invalid blueprint ID format: {}", blueprint_id_str),
        }
    })?;

    let (instance_id, instance_name) =
        engine.instantiate_blueprint_into_factory(factory_id, blueprint_id, custom_name)?;

    Ok((instance_id.to_string(), instance_name))
}

// ============================================================================
// Save/Load Operations
// ============================================================================

/// Save the engine state to a JSON string.
pub fn ffi_save_to_json(engine: &SatisflowEngine) -> FfiResult<String> {
    engine.save_to_json()
}

/// Load the engine state from a JSON string.
/// This will modify the engine's internal state.
pub fn ffi_load_from_json(engine: &mut SatisflowEngine, json: String) -> FfiResult<()> {
    let loaded_engine = SatisflowEngine::load_from_json(&json)?;
    *engine = loaded_engine;
    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::logistics::{TransportType, TruckTransport};
    use crate::models::production_line::{ProductionLineBlueprint, ProductionLineRecipe};
    use crate::models::recipes::Recipe;
    use crate::Item;
    use uuid::Uuid;

    fn create_test_engine() -> SatisflowEngine {
        SatisflowEngine::new()
    }

    #[test]
    fn test_ffi_create_factory() {
        let mut engine = create_test_engine();
        let result = ffi_create_factory(&mut engine, "Test Factory".to_string(), None);
        assert!(result.is_ok());
        let id = result.unwrap();
        assert!(!id.is_empty());
    }

    #[test]
    fn test_ffi_create_factory_with_description() {
        let mut engine = create_test_engine();
        let result = ffi_create_factory(
            &mut engine,
            "Test Factory".to_string(),
            Some("A test factory".to_string()),
        );
        assert!(result.is_ok());

        let factory = ffi_get_factory(&engine, result.unwrap()).unwrap();
        assert_eq!(factory.name, "Test Factory");
        assert_eq!(factory.description, Some("A test factory".to_string()));
    }

    #[test]
    fn test_ffi_get_all_factories() {
        let mut engine = create_test_engine();
        ffi_create_factory(&mut engine, "Factory 1".to_string(), None).unwrap();
        ffi_create_factory(&mut engine, "Factory 2".to_string(), None).unwrap();

        let factories = ffi_get_all_factories(&engine);
        assert_eq!(factories.len(), 2);
    }

    #[test]
    fn test_ffi_delete_factory() {
        let mut engine = create_test_engine();
        let id = ffi_create_factory(&mut engine, "Test Factory".to_string(), None).unwrap();

        let result = ffi_delete_factory(&mut engine, id.clone());
        assert!(result.is_ok());

        let factories = ffi_get_all_factories(&engine);
        assert_eq!(factories.len(), 0);
    }

    #[test]
    fn test_ffi_create_logistics_line() {
        let mut engine = create_test_engine();
        let factory1 = ffi_create_factory(&mut engine, "Factory 1".to_string(), None).unwrap();
        let factory2 = ffi_create_factory(&mut engine, "Factory 2".to_string(), None).unwrap();

        let transport = TransportType::Truck(TruckTransport::new(1, Item::IronOre, 60.0));
        let result = ffi_create_logistics_line(
            &mut engine,
            factory1,
            factory2,
            transport,
            "Test truck".to_string(),
        );
        assert!(result.is_ok());
        let logistics_id = result.unwrap();

        let logistics = ffi_get_logistics_line(&engine, logistics_id).unwrap();
        assert_eq!(logistics.transport_details, "Test truck");
    }

    #[test]
    fn test_ffi_get_all_logistics() {
        let mut engine = create_test_engine();
        let factory1 = ffi_create_factory(&mut engine, "Factory 1".to_string(), None).unwrap();
        let factory2 = ffi_create_factory(&mut engine, "Factory 2".to_string(), None).unwrap();

        let transport = TransportType::Truck(TruckTransport::new(1, Item::IronOre, 60.0));
        ffi_create_logistics_line(
            &mut engine,
            factory1.clone(),
            factory2.clone(),
            transport,
            "Truck 1".to_string(),
        )
        .unwrap();

        let transport2 = TransportType::Truck(TruckTransport::new(2, Item::CopperOre, 120.0));
        ffi_create_logistics_line(
            &mut engine,
            factory2,
            factory1,
            transport2,
            "Truck 2".to_string(),
        )
        .unwrap();

        let logistics = ffi_get_all_logistics(&engine);
        assert_eq!(logistics.len(), 2);
    }

    #[test]
    fn test_ffi_delete_logistics_line() {
        let mut engine = create_test_engine();
        let factory1 = ffi_create_factory(&mut engine, "Factory 1".to_string(), None).unwrap();
        let factory2 = ffi_create_factory(&mut engine, "Factory 2".to_string(), None).unwrap();

        let transport = TransportType::Truck(TruckTransport::new(1, Item::IronOre, 60.0));
        let logistics_id = ffi_create_logistics_line(
            &mut engine,
            factory1,
            factory2,
            transport,
            "Test truck".to_string(),
        )
        .unwrap();

        let result = ffi_delete_logistics_line(&mut engine, logistics_id.clone());
        assert!(result.is_ok());

        let logistics = ffi_get_all_logistics(&engine);
        assert_eq!(logistics.len(), 0);
    }

    #[test]
    fn test_ffi_reset() {
        let mut engine = create_test_engine();
        ffi_create_factory(&mut engine, "Factory 1".to_string(), None).unwrap();

        let result = ffi_reset(&mut engine);
        assert!(result.is_ok());

        let factories = ffi_get_all_factories(&engine);
        assert_eq!(factories.len(), 0);
    }

    #[test]
    fn test_ffi_global_power_stats() {
        let engine = create_test_engine();
        let stats = ffi_global_power_stats(&engine);
        assert_eq!(stats.total_generation, 0.0);
        assert_eq!(stats.total_consumption, 0.0);
    }

    #[test]
    fn test_ffi_update() {
        let mut engine = create_test_engine();
        let result = ffi_update(&mut engine);
        assert!(result.is_empty());
    }

    #[test]
    fn test_ffi_blueprint_crud() {
        let mut engine = create_test_engine();

        // Create a blueprint
        let blueprint = ProductionLineBlueprint::new(
            Uuid::new_v4(),
            "Test Blueprint".to_string(),
            Some("A test blueprint".to_string()),
        );

        // Add blueprint
        let blueprint_id = ffi_add_blueprint_template(&mut engine, blueprint.clone());
        assert!(!blueprint_id.is_empty());

        // Get blueprint
        let retrieved = ffi_get_blueprint_template(&engine, blueprint_id.clone());
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Blueprint");

        // Get all blueprints
        let all_blueprints = ffi_get_all_blueprint_templates(&engine);
        assert_eq!(all_blueprints.len(), 1);

        // Update blueprint
        let update_result = ffi_update_blueprint_template(
            &mut engine,
            blueprint_id.clone(),
            "Updated Blueprint".to_string(),
            "Updated description".to_string(),
        );
        assert!(update_result.is_ok());

        let updated = ffi_get_blueprint_template(&engine, blueprint_id.clone()).unwrap();
        assert_eq!(updated.name, "Updated Blueprint");

        // Remove blueprint
        let remove_result = ffi_remove_blueprint_template(&mut engine, blueprint_id);
        assert!(remove_result.is_ok());

        let all_blueprints = ffi_get_all_blueprint_templates(&engine);
        assert_eq!(all_blueprints.len(), 0);
    }

    #[test]
    fn test_ffi_instantiate_blueprint() {
        let mut engine = create_test_engine();

        // Create a factory
        let factory_id = ffi_create_factory(&mut engine, "Test Factory".to_string(), None).unwrap();

        // Create a blueprint with a production line
        let mut blueprint =
            ProductionLineBlueprint::new(Uuid::new_v4(), "Test Blueprint".to_string(), None);

        let line = ProductionLineRecipe::new(
            Uuid::new_v4(),
            "Iron Line".to_string(),
            None,
            Recipe::IronIngot,
        );
        blueprint.add_production_line(line);

        let blueprint_id = ffi_add_blueprint_template(&mut engine, blueprint);

        // Instantiate
        let result = ffi_instantiate_blueprint_into_factory(
            &mut engine,
            factory_id,
            blueprint_id,
            Some("Custom Name".to_string()),
        );
        assert!(result.is_ok());

        let (instance_id, instance_name) = result.unwrap();
        assert_eq!(instance_name, "Custom Name");
        assert!(!instance_id.is_empty());
    }

    #[test]
    fn test_ffi_save_load_json() {
        let mut engine = create_test_engine();
        ffi_create_factory(&mut engine, "Test Factory".to_string(), None).unwrap();

        // Save to JSON
        let json = ffi_save_to_json(&engine).unwrap();
        assert!(json.contains("Test Factory"));

        // Create new engine and load from JSON
        let mut new_engine = create_test_engine();
        let result = ffi_load_from_json(&mut new_engine, json);
        assert!(result.is_ok());

        let factories = ffi_get_all_factories(&new_engine);
        assert_eq!(factories.len(), 1);
    }

    #[test]
    fn test_ffi_invalid_factory_id() {
        let mut engine = create_test_engine();
        let result = ffi_delete_factory(&mut engine, "invalid-uuid".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_ffi_invalid_logistics_id() {
        let mut engine = create_test_engine();
        let result = ffi_delete_logistics_line(&mut engine, "invalid-uuid".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_ffi_nonexistent_factory_delete() {
        let mut engine = create_test_engine();
        let fake_id = Uuid::new_v4().to_string();
        let result = ffi_delete_factory(&mut engine, fake_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_ffi_nonexistent_logistics_delete() {
        let mut engine = create_test_engine();
        let fake_id = Uuid::new_v4().to_string();
        let result = ffi_delete_logistics_line(&mut engine, fake_id);
        assert!(result.is_err());
    }
}
