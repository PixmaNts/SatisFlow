use flutter_rust_bridge::frb;
use std::collections::HashMap;

// Re-export types from satisflow-engine
pub use satisflow_engine::models::{
    logistics::{Bus, DroneTransport, LogisticsFlux, Train, TransportType, TruckTransport},
    production_line::{EngineError, ProductionLineBlueprint},
    Factory, Item, PowerStats,
};
// Re-export SatisflowEngine for FRB code generation
pub use satisflow_engine::SatisflowEngine;

// Re-export the FfiResult type
pub type FfiResult<T> = Result<T, satisflow_engine::SatisflowError>;

// ============================================================================
// Engine Lifecycle
// ============================================================================

/// Create a new SatisflowEngine instance.
/// This is the entry point for the Flutter app to get an engine handle.
#[frb]
pub fn new_engine() -> SatisflowEngine {
    SatisflowEngine::new()
}

/// Get engine version string for debugging purposes.
#[frb]
pub fn ffi_engine_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ============================================================================
// Factory Operations
// ============================================================================

/// Create a new factory and return its ID as a String.
#[frb]
pub fn ffi_create_factory(
    engine: &mut satisflow_engine::SatisflowEngine,
    name: String,
    description: Option<String>,
) -> FfiResult<String> {
    let factory_id = engine.create_factory(name, description);
    Ok(factory_id.to_string())
}

/// Get a factory by ID, returning a cloned Factory if found.
#[frb]
pub fn ffi_get_factory(engine: &satisflow_engine::SatisflowEngine, id: String) -> Option<Factory> {
    let factory_id = satisflow_engine::models::FactoryId::parse_str(&id).ok()?;
    engine.get_factory(factory_id).cloned()
}

/// Get all factories as a HashMap with String keys (UUIDs converted).
#[frb]
pub fn ffi_get_all_factories(
    engine: &satisflow_engine::SatisflowEngine,
) -> HashMap<String, Factory> {
    engine
        .get_all_factories()
        .iter()
        .map(|(id, factory)| (id.to_string(), factory.clone()))
        .collect()
}

/// Delete a factory by ID.
#[frb]
pub fn ffi_delete_factory(
    engine: &mut satisflow_engine::SatisflowEngine,
    id: String,
) -> FfiResult<()> {
    let factory_id = satisflow_engine::models::FactoryId::parse_str(&id).map_err(|_| {
        satisflow_engine::SatisflowError::InvalidInput {
            message: format!("Invalid factory ID format: {}", id),
        }
    })?;
    engine.delete_factory(factory_id)
}

// ============================================================================
// Logistics Operations
// ============================================================================

/// Create a logistics line between two factories and return the new line's ID.
#[frb]
pub fn ffi_create_logistics_line(
    engine: &mut satisflow_engine::SatisflowEngine,
    from_id: String,
    to_id: String,
    transport_type: TransportType,
    transport_detail: String,
) -> FfiResult<String> {
    let from = satisflow_engine::models::FactoryId::parse_str(&from_id).map_err(|_| {
        satisflow_engine::SatisflowError::InvalidInput {
            message: format!("Invalid from_factory ID format: {}", from_id),
        }
    })?;
    let to = satisflow_engine::models::FactoryId::parse_str(&to_id).map_err(|_| {
        satisflow_engine::SatisflowError::InvalidInput {
            message: format!("Invalid to_factory ID format: {}", to_id),
        }
    })?;

    let logistics_id = engine.create_logistics_line(from, to, transport_type, transport_detail)?;
    Ok(logistics_id.to_string())
}

/// Update an existing logistics line.
#[frb]
pub fn ffi_update_logistics_line(
    engine: &mut satisflow_engine::SatisflowEngine,
    id: String,
    from_id: String,
    to_id: String,
    transport_type: TransportType,
    transport_detail: String,
) -> FfiResult<()> {
    let logistics_id = satisflow_engine::models::LogisticsId::parse_str(&id).map_err(|_| {
        satisflow_engine::SatisflowError::InvalidInput {
            message: format!("Invalid logistics ID format: {}", id),
        }
    })?;
    let from = satisflow_engine::models::FactoryId::parse_str(&from_id).map_err(|_| {
        satisflow_engine::SatisflowError::InvalidInput {
            message: format!("Invalid from_factory ID format: {}", from_id),
        }
    })?;
    let to = satisflow_engine::models::FactoryId::parse_str(&to_id).map_err(|_| {
        satisflow_engine::SatisflowError::InvalidInput {
            message: format!("Invalid to_factory ID format: {}", to_id),
        }
    })?;

    engine.update_logistics_line(logistics_id, from, to, transport_type, transport_detail)
}

/// Get a logistics line by ID, returning a cloned LogisticsFlux if found.
#[frb]
pub fn ffi_get_logistics_line(
    engine: &satisflow_engine::SatisflowEngine,
    id: String,
) -> Option<LogisticsFlux> {
    let logistics_id = satisflow_engine::models::LogisticsId::parse_str(&id).ok()?;
    engine.get_logistics_line(logistics_id).cloned()
}

/// Get all logistics lines as a HashMap with String keys.
#[frb]
pub fn ffi_get_all_logistics(
    engine: &satisflow_engine::SatisflowEngine,
) -> HashMap<String, LogisticsFlux> {
    engine
        .get_all_logistics()
        .iter()
        .map(|(id, flux)| (id.to_string(), flux.clone()))
        .collect()
}

/// Get transport type as a string for a given logistics line.
#[frb]
pub fn ffi_get_logistics_transport_type(flux: &LogisticsFlux) -> String {
    flux.transport_type.to_string()
}

/// Get all logistics transport types as a HashMap with String keys.
#[frb]
pub fn ffi_get_all_logistics_transport_types(
    engine: &satisflow_engine::SatisflowEngine,
) -> HashMap<String, String> {
    engine
        .get_all_logistics()
        .iter()
        .map(|(id, flux)| (id.to_string(), flux.transport_type.to_string()))
        .collect()
}

/// Delete a logistics line by ID.
#[frb]
pub fn ffi_delete_logistics_line(
    engine: &mut satisflow_engine::SatisflowEngine,
    id: String,
) -> FfiResult<()> {
    let logistics_id = satisflow_engine::models::LogisticsId::parse_str(&id).map_err(|_| {
        satisflow_engine::SatisflowError::InvalidInput {
            message: format!("Invalid logistics ID format: {}", id),
        }
    })?;
    engine.delete_logistics_line(logistics_id)
}

/// Create a TransportType from a type name string.
/// This is needed because TransportType is opaque in Flutter and cannot be constructed directly.
#[frb]
pub fn ffi_create_transport_type(type_name: String) -> TransportType {
    match type_name.as_str() {
        "Bus" => TransportType::Bus(Bus::new(0, "Default Bus")),
        "Train" => TransportType::Train(Train::new(0, "Default Train")),
        "Truck" => TransportType::Truck(TruckTransport::new(0, Item::IronOre, 60.0)),
        "Drone" => TransportType::Drone(DroneTransport::new(0, Item::IronOre, 60.0)),
        _ => panic!("Unknown transport type: {}", type_name),
    }
}

// ============================================================================
// Engine State Operations
// ============================================================================

/// Reset the engine to an empty state.
#[frb]
pub fn ffi_reset(engine: &mut satisflow_engine::SatisflowEngine) -> FfiResult<()> {
    engine.reset()
}

/// Get global power statistics for all factories.
#[frb]
pub fn ffi_global_power_stats(engine: &satisflow_engine::SatisflowEngine) -> PowerStats {
    engine.global_power_stats()
}

/// Run one update cycle and return global item balances.
#[frb]
pub fn ffi_update(engine: &mut satisflow_engine::SatisflowEngine) -> HashMap<String, f32> {
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
#[frb]
pub fn ffi_add_blueprint_template(
    engine: &mut satisflow_engine::SatisflowEngine,
    blueprint: ProductionLineBlueprint,
) -> String {
    engine.add_blueprint_template(blueprint).to_string()
}

/// Get a blueprint template by ID.
#[frb]
pub fn ffi_get_blueprint_template(
    engine: &satisflow_engine::SatisflowEngine,
    id: String,
) -> Option<ProductionLineBlueprint> {
    let blueprint_id = satisflow_engine::models::ProductionLineId::parse_str(&id).ok()?;
    engine.get_blueprint_template(blueprint_id).cloned()
}

/// Get all blueprint templates as a HashMap with String keys.
#[frb]
pub fn ffi_get_all_blueprint_templates(
    engine: &satisflow_engine::SatisflowEngine,
) -> HashMap<String, ProductionLineBlueprint> {
    engine
        .get_all_blueprint_templates()
        .iter()
        .map(|(id, blueprint)| (id.to_string(), blueprint.clone()))
        .collect()
}

/// Remove a blueprint template by ID.
#[frb]
pub fn ffi_remove_blueprint_template(
    engine: &mut satisflow_engine::SatisflowEngine,
    id: String,
) -> FfiResult<()> {
    let blueprint_id =
        satisflow_engine::models::ProductionLineId::parse_str(&id).map_err(|_| {
            satisflow_engine::SatisflowError::InvalidInput {
                message: format!("Invalid blueprint ID format: {}", id),
            }
        })?;
    engine.remove_blueprint_template(blueprint_id)
}

/// Update a blueprint template's name and description.
#[frb]
pub fn ffi_update_blueprint_template(
    engine: &mut satisflow_engine::SatisflowEngine,
    id: String,
    name: String,
    description: String,
) -> FfiResult<()> {
    let blueprint_id =
        satisflow_engine::models::ProductionLineId::parse_str(&id).map_err(|_| {
            satisflow_engine::SatisflowError::InvalidInput {
                message: format!("Invalid blueprint ID format: {}", id),
            }
        })?;
    engine
        .update_blueprint_template(blueprint_id, name, description)
        .map_err(|e| match e {
            EngineError::EmptyName => satisflow_engine::SatisflowError::InvalidInput {
                message: "Blueprint name cannot be empty".to_string(),
            },
            EngineError::TemplateNotFound { id } => {
                satisflow_engine::SatisflowError::BlueprintNotFound { id: id.to_string() }
            }
            EngineError::IndexOutOfBounds { index, length } => {
                satisflow_engine::SatisflowError::InvalidInput {
                    message: format!("Index {} out of bounds for {} elements", index, length),
                }
            }
        })
}

/// Instantiate a blueprint template into a factory.
#[frb]
pub fn ffi_instantiate_blueprint_into_factory(
    engine: &mut satisflow_engine::SatisflowEngine,
    factory_id_str: String,
    blueprint_id_str: String,
    custom_name: Option<String>,
) -> FfiResult<(String, String)> {
    let factory_id =
        satisflow_engine::models::FactoryId::parse_str(&factory_id_str).map_err(|_| {
            satisflow_engine::SatisflowError::InvalidInput {
                message: format!("Invalid factory ID format: {}", factory_id_str),
            }
        })?;
    let blueprint_id = satisflow_engine::models::ProductionLineId::parse_str(&blueprint_id_str)
        .map_err(|_| satisflow_engine::SatisflowError::InvalidInput {
            message: format!("Invalid blueprint ID format: {}", blueprint_id_str),
        })?;

    let (instance_id, instance_name) =
        engine.instantiate_blueprint_into_factory(factory_id, blueprint_id, custom_name)?;

    Ok((instance_id.to_string(), instance_name))
}

// ============================================================================
// Save/Load Operations
// ============================================================================

/// Save the engine state to a JSON string.
/// Throws an exception on error (no opaque wrapper).
#[frb]
pub fn ffi_save_to_json(engine: &satisflow_engine::SatisflowEngine) -> String {
    engine
        .save_to_json()
        .expect("Failed to save engine state to JSON")
}

/// Load the engine state from a JSON string.
/// Throws an exception on error.
#[frb]
pub fn ffi_load_from_json(engine: &mut satisflow_engine::SatisflowEngine, json: String) {
    let loaded_engine = satisflow_engine::SatisflowEngine::load_from_json(&json)
        .expect("Failed to load engine state from JSON");
    *engine = loaded_engine;
}

// ============================================================================
// Blueprint JSON Serialization
// ============================================================================

/// Serialize a blueprint template to a JSON string for export.
/// Throws an exception on error.
#[frb]
pub fn ffi_blueprint_to_json(blueprint: ProductionLineBlueprint) -> String {
    serde_json::to_string(&blueprint).expect("Failed to serialize blueprint to JSON")
}

/// Deserialize a blueprint template from a JSON string for import.
/// Throws an exception on error.
#[frb]
pub fn ffi_blueprint_from_json(json: String) -> ProductionLineBlueprint {
    serde_json::from_str(&json).expect("Failed to deserialize blueprint from JSON")
}
