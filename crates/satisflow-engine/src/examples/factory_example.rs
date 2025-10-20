//! Comprehensive factory example for the Satisflow engine
//! 
//! This module creates a complete factory setup with 5 specialized factories:
//! 1. Northern Forest - Smelting hub with iron/copper processing
//! 2. Central Assembly - Advanced manufacturing hub
//! 3. Oil Refinery - Petroleum processing and plastics
//! 4. Steel Mill - Heavy industry with steel and concrete
//! 5. Electronics Lab - High-tech components and computers
//! 
//! Each factory includes production lines, raw inputs, power generators,
//! and logistics connections between factories.

use crate::{
    SatisflowEngine,
    models::{
        logistics::{Bus, Conveyor, ConveyorSpeed, Pipeline, PipelineCapacity, Train, TransportType, TruckTransport, Wagon, WagonType},
        power_generator::{GeneratorGroup, GeneratorType, PowerGenerator},
        production_line::{MachineGroup, ProductionLine, ProductionLineRecipe},
        raw_input::{ExtractorType, Purity, RawInput},
        Item, Recipe,
    },
};

/// Create a comprehensive factory setup for testing and demonstration
/// Creates 5 specialized factories with diverse production chains and logistics network
pub fn create_sample_factory_setup() -> SatisflowEngine {
    let mut engine = SatisflowEngine::new();

    // Create all 5 factories
    let northern_forest_id = engine.create_factory(
        "Northern Forest - Smelting Hub".to_string(),
        Some("Northern Forest - Smelting Hub for iron and copper processing".to_string()),
    );

    let central_assembly_id = engine.create_factory(
        "Central Assembly - Manufacturing Hub".to_string(),
        Some("Central Assembly - Manufacturing Hub for complex components".to_string()),
    );

    let oil_refinery_id = engine.create_factory(
        "Oil Refinery - Petroleum Hub".to_string(),
        Some("Oil Refinery - Petroleum Hub for fuel and plastics".to_string()),
    );

    let steel_mill_id = engine.create_factory(
        "Steel Mill - Heavy Industry".to_string(),
        Some("Steel Mill - Heavy Industry for construction materials".to_string()),
    );

    let electronics_lab_id = engine.create_factory(
        "Electronics Lab - High-Tech Hub".to_string(),
        Some("Electronics Lab - High-Tech Hub for computers and circuits".to_string()),
    );

    // Set up each factory with production lines, raw inputs, and power
    setup_northern_forest_factory(&mut engine, northern_forest_id);
    setup_central_assembly_factory(&mut engine, central_assembly_id);
    setup_oil_refinery_factory(&mut engine, oil_refinery_id);
    setup_steel_mill_factory(&mut engine, steel_mill_id);
    setup_electronics_lab_factory(&mut engine, electronics_lab_id);

    // Set up logistics connections between factories
    setup_inter_factory_logistics(&mut engine, northern_forest_id, central_assembly_id, oil_refinery_id, steel_mill_id, electronics_lab_id);

    engine
}

/// Set up the Northern Forest factory - Primary smelting hub for iron/copper processing
fn setup_northern_forest_factory(engine: &mut SatisflowEngine, factory_id: u64) {
    // Add raw inputs for smelting hub
    let iron_ore = RawInput::new(1, ExtractorType::MinerMk2, Item::IronOre, Some(Purity::Normal))
        .expect("Should create valid iron ore input");
    
    let copper_ore = RawInput::new(2, ExtractorType::MinerMk2, Item::CopperOre, Some(Purity::Normal))
        .expect("Should create valid copper ore input");
    
    let limestone = RawInput::new(3, ExtractorType::MinerMk1, Item::Limestone, Some(Purity::Normal))
        .expect("Should create valid limestone input");

    if let Some(factory) = engine.get_factory_mut(factory_id) {
        factory.add_raw_input(iron_ore).expect("Should add iron ore input");
        factory.add_raw_input(copper_ore).expect("Should add copper ore input");
        factory.add_raw_input(limestone).expect("Should add limestone input");
    }

    // Add production lines for smelting operations
    add_production_line(engine, factory_id, 1, "Iron Ingot Smelting", Recipe::IronIngot, vec![
        MachineGroup::new(4, 100.0, 0), // 4 Constructors at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 2, "Copper Ingot Smelting", Recipe::CopperIngot, vec![
        MachineGroup::new(3, 100.0, 0), // 3 Constructors at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 3, "Iron Rod Production", Recipe::IronRod, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Constructors at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 4, "Iron Plate Production", Recipe::IronPlate, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Constructors at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 5, "Wire Production", Recipe::Wire, vec![
        MachineGroup::new(3, 100.0, 0), // 3 Constructors at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 6, "Cable Production", Recipe::Cable, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Constructors at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 7, "Screw Production", Recipe::Screw, vec![
        MachineGroup::new(2, 150.0, 0), // 2 Constructors at 150% clock speed
    ]);

    add_production_line(engine, factory_id, 8, "Reinforced Iron Plate", Recipe::ReinforcedIronPlate, vec![
        MachineGroup::new(1, 100.0, 1), // 1 Assembler with 1 Somersloop
    ]);

    add_production_line(engine, factory_id, 9, "Rotor Production", Recipe::Rotor, vec![
        MachineGroup::new(1, 100.0, 0), // 1 Assembler at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 10, "Modular Frame Production", Recipe::ModularFrame, vec![
        MachineGroup::new(1, 100.0, 0), // 1 Assembler at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 11, "Concrete Production", Recipe::Concrete, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Constructors at 100% clock speed
    ]);

    // Add power generators
    add_coal_power_generator(engine, factory_id, 1, 4, 100.0); // 4 coal generators at 100%
}

/// Set up the Central Assembly factory - Advanced manufacturing hub
fn setup_central_assembly_factory(engine: &mut SatisflowEngine, factory_id: u64) {
    // Add production lines for advanced assembly
    add_production_line(engine, factory_id, 1, "Smart Plating", Recipe::SmartPlating, vec![
        MachineGroup::new(2, 100.0, 1), // 2 Assemblers with 1 Somersloop each
    ]);

    add_production_line(engine, factory_id, 2, "Versatile Framework", Recipe::VersatileFramework, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Manufacturers at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 3, "Heavy Modular Frame", Recipe::HeavyModularFrame, vec![
        MachineGroup::new(1, 120.0, 2), // 1 Manufacturer with 2 Somersloops at 120% clock speed
    ]);

    add_production_line(engine, factory_id, 4, "Motor Production", Recipe::Motor, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Assemblers at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 5, "Stator Production", Recipe::Stator, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Assemblers at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 6, "Encased Industrial Beam", Recipe::EncasedIndustrialBeam, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Assemblers at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 7, "Automated Wiring", Recipe::AutomatedWiring, vec![
        MachineGroup::new(1, 100.0, 0), // 1 Assembler at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 8, "Circuit Board", Recipe::CircuitBoard, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Assemblers at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 9, "AI Limiter", Recipe::AILimiter, vec![
        MachineGroup::new(1, 100.0, 1), // 1 Assembler with 1 Somersloop
    ]);

    add_production_line(engine, factory_id, 10, "Crystal Oscillator", Recipe::CrystalOscillator, vec![
        MachineGroup::new(1, 80.0, 0), // 1 Manufacturer at 80% clock speed
    ]);

    add_production_line(engine, factory_id, 11, "Supercomputer", Recipe::Supercomputer, vec![
        MachineGroup::new(1, 100.0, 2), // 1 Manufacturer with 2 Somersloops
    ]);

    // Add power generators
    add_fuel_power_generator(engine, factory_id, 1, 3, 100.0); // 3 fuel generators at 100%
}

/// Set up the Oil Refinery factory - Petroleum processing and plastics
fn setup_oil_refinery_factory(engine: &mut SatisflowEngine, factory_id: u64) {
    // Add raw inputs for oil processing
    let crude_oil = RawInput::new(1, ExtractorType::OilExtractor, Item::CrudeOil, Some(Purity::Normal))
        .expect("Should create valid crude oil input");
    
    let water = RawInput::new(2, ExtractorType::WaterExtractor, Item::Water, None)
        .expect("Should create valid water input");

    if let Some(factory) = engine.get_factory_mut(factory_id) {
        factory.add_raw_input(crude_oil).expect("Should add crude oil input");
        factory.add_raw_input(water).expect("Should add water input");
    }

    // Add production lines for oil processing
    add_production_line(engine, factory_id, 1, "Plastic Production", Recipe::Plastic, vec![
        MachineGroup::new(3, 100.0, 0), // 3 Refineries at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 2, "Rubber Production", Recipe::Rubber, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Refineries at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 3, "Fuel Production", Recipe::Fuel, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Refineries at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 4, "Computer Production", Recipe::Computer, vec![
        MachineGroup::new(2, 100.0, 1), // 2 Manufacturers with 1 Somersloop each
    ]);

    add_production_line(engine, factory_id, 5, "High-Speed Connector", Recipe::HighSpeedConnector, vec![
        MachineGroup::new(1, 100.0, 1), // 1 Manufacturer with 1 Somersloop
    ]);

    add_production_line(engine, factory_id, 6, "Beacon", Recipe::Beacon, vec![
        MachineGroup::new(1, 100.0, 0), // 1 Manufacturer at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 7, "Empty Canister", Recipe::EmptyCanister, vec![
        MachineGroup::new(1, 100.0, 0), // 1 Assembler at 100% clock speed
    ]);

    // Add power generators
    add_fuel_power_generator(engine, factory_id, 1, 4, 125.0); // 4 fuel generators at 125%
}

/// Set up the Steel Mill factory - Heavy industry with steel and concrete
fn setup_steel_mill_factory(engine: &mut SatisflowEngine, factory_id: u64) {
    // Add raw inputs for heavy industry
    let iron_ore = RawInput::new(1, ExtractorType::MinerMk3, Item::IronOre, Some(Purity::Pure))
        .expect("Should create valid iron ore input");
    
    let coal = RawInput::new(2, ExtractorType::MinerMk2, Item::Coal, Some(Purity::Normal))
        .expect("Should create valid coal input");
    
    let limestone = RawInput::new(3, ExtractorType::MinerMk3, Item::Limestone, Some(Purity::Pure))
        .expect("Should create valid limestone input");
    
    let bauxite = RawInput::new(4, ExtractorType::MinerMk2, Item::Bauxite, Some(Purity::Normal))
        .expect("Should create valid bauxite input");

    if let Some(factory) = engine.get_factory_mut(factory_id) {
        factory.add_raw_input(iron_ore).expect("Should add iron ore input");
        factory.add_raw_input(coal).expect("Should add coal input");
        factory.add_raw_input(limestone).expect("Should add limestone input");
        factory.add_raw_input(bauxite).expect("Should add bauxite input");
    }

    // Add production lines for steel and construction
    add_production_line(engine, factory_id, 1, "Steel Ingot Production", Recipe::SteelIngot, vec![
        MachineGroup::new(4, 100.0, 0), // 4 Foundries at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 2, "Steel Beam Production", Recipe::SteelBeam, vec![
        MachineGroup::new(3, 100.0, 0), // 3 Constructors at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 3, "Steel Pipe Production", Recipe::SteelPipe, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Constructors at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 4, "Concrete Production", Recipe::Concrete, vec![
        MachineGroup::new(4, 100.0, 0), // 4 Constructors at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 5, "Encased Industrial Beam", Recipe::EncasedIndustrialBeam, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Assemblers at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 6, "Heavy Modular Frame", Recipe::HeavyModularFrame, vec![
        MachineGroup::new(2, 100.0, 1), // 2 Manufacturers with 1 Somersloop each
    ]);

    add_production_line(engine, factory_id, 7, "Aluminum Ingot Production", Recipe::AluminumIngot, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Foundries at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 8, "Aluminum Casing", Recipe::AluminumCasing, vec![
        MachineGroup::new(1, 100.0, 0), // 1 Constructor at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 9, "Alclad Aluminum Sheet", Recipe::AlcladAluminumSheet, vec![
        MachineGroup::new(1, 100.0, 0), // 1 Manufacturer at 100% clock speed
    ]);

    // Add power generators
    add_coal_power_generator(engine, factory_id, 1, 6, 100.0); // 6 coal generators at 100%
}

/// Set up the Electronics Lab factory - High-tech components and computers
fn setup_electronics_lab_factory(engine: &mut SatisflowEngine, factory_id: u64) {
    // Add raw inputs for high-tech manufacturing
    let caterium_ore = RawInput::new(1, ExtractorType::MinerMk2, Item::CateriumOre, Some(Purity::Normal))
        .expect("Should create valid caterium ore input");
    
    let raw_quartz = RawInput::new(2, ExtractorType::MinerMk2, Item::RawQuartz, Some(Purity::Normal))
        .expect("Should create valid quartz input");
    
    let sulfur = RawInput::new(3, ExtractorType::MinerMk1, Item::Sulfur, Some(Purity::Impure))
        .expect("Should create valid sulfur input");

    if let Some(factory) = engine.get_factory_mut(factory_id) {
        factory.add_raw_input(caterium_ore).expect("Should add caterium ore input");
        factory.add_raw_input(raw_quartz).expect("Should add quartz input");
        factory.add_raw_input(sulfur).expect("Should add sulfur input");
    }

    // Add production lines for electronics
    add_production_line(engine, factory_id, 1, "Circuit Board", Recipe::CircuitBoard, vec![
        MachineGroup::new(3, 100.0, 0), // 3 Assemblers at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 2, "AI Limiter", Recipe::AILimiter, vec![
        MachineGroup::new(2, 100.0, 1), // 2 Assemblers with 1 Somersloop each
    ]);

    add_production_line(engine, factory_id, 3, "High-Speed Connector", Recipe::HighSpeedConnector, vec![
        MachineGroup::new(2, 100.0, 1), // 2 Manufacturers with 1 Somersloop each
    ]);

    add_production_line(engine, factory_id, 4, "Computer", Recipe::Computer, vec![
        MachineGroup::new(3, 100.0, 2), // 3 Manufacturers with 2 Somersloops each
    ]);

    add_production_line(engine, factory_id, 5, "Supercomputer", Recipe::Supercomputer, vec![
        MachineGroup::new(2, 100.0, 2), // 2 Manufacturers with 2 Somersloops each
    ]);

    add_production_line(engine, factory_id, 6, "Crystal Oscillator", Recipe::CrystalOscillator, vec![
        MachineGroup::new(2, 100.0, 0), // 2 Manufacturers at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 7, "Pressure Conversion Cube", Recipe::PressureConversionCube, vec![
        MachineGroup::new(1, 100.0, 0), // 1 Manufacturer at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 8, "Radio Control Unit", Recipe::RadioControlUnit, vec![
        MachineGroup::new(1, 100.0, 1), // 1 Manufacturer with 1 Somersloop
    ]);

    add_production_line(engine, factory_id, 9, "Beacon", Recipe::Beacon, vec![
        MachineGroup::new(1, 100.0, 0), // 1 Manufacturer at 100% clock speed
    ]);

    add_production_line(engine, factory_id, 10, "Adaptive Control Unit", Recipe::AdaptiveControlUnit, vec![
        MachineGroup::new(1, 100.0, 2), // 1 Manufacturer with 2 Somersloops
    ]);

    add_production_line(engine, factory_id, 11, "Assembly Director System", Recipe::AssemblyDirectorSystem, vec![
        MachineGroup::new(1, 50.0, 2), // 1 Manufacturer with 2 Somersloops at 50% clock speed
    ]);

    // Add power generators
    add_fuel_power_generator(engine, factory_id, 1, 2, 150.0); // 2 fuel generators at 150%
}

/// Set up comprehensive logistics network between all 5 factories
fn setup_inter_factory_logistics(
    engine: &mut SatisflowEngine,
    northern_forest_id: u64,
    central_assembly_id: u64,
    oil_refinery_id: u64,
    steel_mill_id: u64,
    electronics_lab_id: u64,
) {
    // BUS 1: Northern Forest -> Central Assembly (Basic materials via conveyor bus)
    let bus1 = Bus::new(1, "Forest to Assembly Bus")
        .with_conveyor(Conveyor::new(1, ConveyorSpeed::Mk3, Item::IronIngot, 200.0))
        .with_conveyor(Conveyor::new(2, ConveyorSpeed::Mk3, Item::CopperIngot, 120.0))
        .with_conveyor(Conveyor::new(3, ConveyorSpeed::Mk2, Item::IronRod, 60.0))
        .with_conveyor(Conveyor::new(4, ConveyorSpeed::Mk2, Item::Wire, 180.0));

    engine.create_logistics_line(
        northern_forest_id,
        central_assembly_id,
        TransportType::Bus(bus1),
        "Primary iron and copper supply for assembly operations".to_string(),
    ).expect("Should create forest->assembly bus");

    // TRAIN 1: Steel Mill -> Central Assembly (Heavy materials via train)
    let train1 = Train::new(1, "Steel Mainline")
        .with_wagon(Wagon::new(1, WagonType::Cargo, Item::SteelIngot, 400.0))
        .with_wagon(Wagon::new(2, WagonType::Cargo, Item::SteelBeam, 300.0))
        .with_wagon(Wagon::new(3, WagonType::Cargo, Item::SteelPipe, 200.0))
        .with_wagon(Wagon::new(4, WagonType::Cargo, Item::Concrete, 600.0));

    engine.create_logistics_line(
        steel_mill_id,
        central_assembly_id,
        TransportType::Train(train1),
        "Heavy steel delivery - Main industrial train".to_string(),
    ).expect("Should create steel->assembly train");

    // TRUCK 1: Oil Refinery -> Electronics Lab (Plastic via truck)
    let truck1 = TruckTransport::new(1, Item::Plastic, 180.0);

    engine.create_logistics_line(
        oil_refinery_id,
        electronics_lab_id,
        TransportType::Truck(truck1),
        "Plastic delivery for circuit board production".to_string(),
    ).expect("Should create oil->electronics plastic truck");

    // TRUCK 2: Oil Refinery -> Electronics Lab (Rubber via truck)
    let truck2 = TruckTransport::new(2, Item::Rubber, 120.0);

    engine.create_logistics_line(
        oil_refinery_id,
        electronics_lab_id,
        TransportType::Truck(truck2),
        "Rubber supply for advanced components".to_string(),
    ).expect("Should create oil->electronics rubber truck");

    // BUS 2: Electronics Lab -> Central Assembly (High-tech components)
    let bus2 = Bus::new(2, "Electronics to Assembly Bus")
        .with_conveyor(Conveyor::new(1, ConveyorSpeed::Mk4, Item::CircuitBoard, 90.0))
        .with_conveyor(Conveyor::new(2, ConveyorSpeed::Mk4, Item::AILimiter, 15.0))
        .with_conveyor(Conveyor::new(3, ConveyorSpeed::Mk4, Item::Computer, 30.0))
        .with_conveyor(Conveyor::new(4, ConveyorSpeed::Mk3, Item::HighSpeedConnector, 45.0));

    engine.create_logistics_line(
        electronics_lab_id,
        central_assembly_id,
        TransportType::Bus(bus2),
        "High-tech electronics components for advanced assembly".to_string(),
    ).expect("Should create electronics->assembly bus");

    // TRAIN 2: Steel Mill -> Oil Refinery (Materials for oil processing)
    let train2 = Train::new(2, "Steel to Oil Shuttle")
        .with_wagon(Wagon::new(1, WagonType::Cargo, Item::SteelPipe, 120.0))
        .with_wagon(Wagon::new(2, WagonType::Cargo, Item::HeavyModularFrame, 40.0));

    engine.create_logistics_line(
        steel_mill_id,
        oil_refinery_id,
        TransportType::Train(train2),
        "Steel pipes and frames for oil refinery infrastructure".to_string(),
    ).expect("Should create steel->oil train");

    // BUS 3: Oil Refinery -> Central Assembly (Fuel and chemicals)
    let bus3 = Bus::new(3, "Oil to Assembly Bus")
        .with_pipeline(Pipeline::new(1, PipelineCapacity::Mk2, Item::Fuel, 60.0))
        .with_conveyor(Conveyor::new(1, ConveyorSpeed::Mk2, Item::Plastic, 100.0));

    engine.create_logistics_line(
        oil_refinery_id,
        central_assembly_id,
        TransportType::Bus(bus3),
        "Packaged fuel and plastic for vehicle and generator operations".to_string(),
    ).expect("Should create oil->assembly bus");

    // BUS 4: Northern Forest -> Steel Mill (Copper for advanced steel processing)
    let bus4 = Bus::new(4, "Forest Support Shuttle")
        .with_conveyor(Conveyor::new(1, ConveyorSpeed::Mk3, Item::CopperIngot, 80.0))
        .with_conveyor(Conveyor::new(2, ConveyorSpeed::Mk2, Item::Screw, 240.0));

    engine.create_logistics_line(
        northern_forest_id,
        steel_mill_id,
        TransportType::Bus(bus4),
        "Copper supply for aluminum processing at steel mill".to_string(),
    ).expect("Should create forest->steel bus");

    // TRUCK 3: Steel Mill -> Electronics Lab (Aluminum materials)
    let truck3 = TruckTransport::new(3, Item::AlcladAluminumSheet, 20.0);

    engine.create_logistics_line(
        steel_mill_id,
        electronics_lab_id,
        TransportType::Truck(truck3),
        "Specialized aluminum components for high-tech manufacturing".to_string(),
    ).expect("Should create steel->electronics truck");
}

/// Helper function to add a production line to a factory
fn add_production_line(
    engine: &mut SatisflowEngine,
    factory_id: u64,
    line_id: u64,
    name: &str,
    recipe: Recipe,
    machine_groups: Vec<MachineGroup>,
) {
    let mut production_line = ProductionLineRecipe::new(
        line_id,
        name.to_string(),
        Some(format!("Production line for {}", name)),
        recipe,
    );

    for group in machine_groups {
        production_line
            .add_machine_group(group)
            .expect("Should add machine group");
    }

    if let Some(factory) = engine.get_factory_mut(factory_id) {
        factory.add_production_line(ProductionLine::ProductionLineRecipe(production_line));
    }
}

/// Helper function to add coal power generators to a factory
fn add_coal_power_generator(
    engine: &mut SatisflowEngine,
    factory_id: u64,
    generator_id: u64,
    num_generators: u32,
    clock_speed: f32,
) {
    let mut generator = PowerGenerator::new(generator_id, GeneratorType::Coal, Item::Coal)
        .expect("Should create valid coal generator");

    let group = GeneratorGroup::new(num_generators, clock_speed)
        .expect("Should create valid generator group");
    generator.add_group(group).expect("Should add generator group");

    if let Some(factory) = engine.get_factory_mut(factory_id) {
        factory.add_power_generator(generator).expect("Should add power generator");
    }
}

/// Helper function to add fuel power generators to a factory
fn add_fuel_power_generator(
    engine: &mut SatisflowEngine,
    factory_id: u64,
    generator_id: u64,
    num_generators: u32,
    clock_speed: f32,
) {
    let mut generator = PowerGenerator::new(generator_id, GeneratorType::Fuel, Item::Fuel)
        .expect("Should create valid fuel generator");

    let group = GeneratorGroup::new(num_generators, clock_speed)
        .expect("Should create valid generator group");
    generator.add_group(group).expect("Should add generator group");

    if let Some(factory) = engine.get_factory_mut(factory_id) {
        factory.add_power_generator(generator).expect("Should add power generator");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_factory_creation() {
        let engine = create_sample_factory_setup();

        // Verify all 5 factories exist
        assert_eq!(engine.factories.len(), 5);
        
        // Check factory names
        let factory_names: Vec<String> = engine.factories.values()
            .map(|f| f.name.clone())
            .collect();
        
        assert!(factory_names.iter().any(|name| name.contains("Northern Forest")));
        assert!(factory_names.iter().any(|name| name.contains("Central Assembly")));
        assert!(factory_names.iter().any(|name| name.contains("Oil Refinery")));
        assert!(factory_names.iter().any(|name| name.contains("Steel Mill")));
        assert!(factory_names.iter().any(|name| name.contains("Electronics Lab")));

        // Verify production lines exist in each factory
        for factory in engine.factories.values() {
            assert!(!factory.production_lines.is_empty(), 
                "Factory {} should have production lines", factory.name);
            assert!(!factory.power_generators.is_empty(), 
                "Factory {} should have power generators", factory.name);
        }

        // Verify logistics connections exist
        assert!(!engine.logistics_lines.is_empty(), "Should have logistics connections");
        assert!(engine.logistics_lines.len() >= 8, "Should have at least 8 logistics lines");
    }

    #[test]
    fn test_northern_forest_factory_setup() {
        let engine = create_sample_factory_setup();
        
        // Find Northern Forest factory
        let northern_forest = engine.factories.values()
            .find(|f| f.name.contains("Northern Forest"))
            .expect("Should find Northern Forest factory");

        // Check raw inputs
        assert_eq!(northern_forest.raw_inputs.len(), 3, "Should have 3 raw inputs");
        
        // Check production lines
        assert_eq!(northern_forest.production_lines.len(), 11, "Should have 11 production lines");
        
        // Check power generators
        assert_eq!(northern_forest.power_generators.len(), 1, "Should have 1 power generator");

        // Verify specific production lines exist
        let line_names: Vec<String> = northern_forest.production_lines.values()
            .map(|line| match line {
                ProductionLine::ProductionLineRecipe(recipe) => recipe.name.clone(),
                ProductionLine::ProductionLineBlueprint(_) => "Blueprint".to_string(),
            })
            .collect();

        assert!(line_names.iter().any(|name| name.contains("Iron Ingot")));
        assert!(line_names.iter().any(|name| name.contains("Copper Ingot")));
        assert!(line_names.iter().any(|name| name.contains("Concrete")));
    }

    #[test]
    fn test_oil_refinery_factory_setup() {
        let engine = create_sample_factory_setup();
        
        // Find Oil Refinery factory
        let oil_refinery = engine.factories.values()
            .find(|f| f.name.contains("Oil Refinery"))
            .expect("Should find Oil Refinery factory");

        // Check raw inputs (crude oil and water)
        assert_eq!(oil_refinery.raw_inputs.len(), 2, "Should have 2 raw inputs");
        
        // Check production lines
        assert_eq!(oil_refinery.production_lines.len(), 7, "Should have 7 production lines");
        
        // Check power generators
        assert_eq!(oil_refinery.power_generators.len(), 1, "Should have 1 power generator");

        // Verify specific production lines exist
        let line_names: Vec<String> = oil_refinery.production_lines.values()
            .map(|line| match line {
                ProductionLine::ProductionLineRecipe(recipe) => recipe.name.clone(),
                ProductionLine::ProductionLineBlueprint(_) => "Blueprint".to_string(),
            })
            .collect();

        assert!(line_names.iter().any(|name| name.contains("Plastic")));
        assert!(line_names.iter().any(|name| name.contains("Rubber")));
        assert!(line_names.iter().any(|name| name.contains("Fuel")));
    }

    #[test]
    fn test_logistics_connections() {
        let engine = create_sample_factory_setup();
        
        // Verify we have multiple types of logistics
        let mut bus_count = 0;
        let mut train_count = 0;
        let mut truck_count = 0;

        for logistics in engine.logistics_lines.values() {
            match &logistics.transport_type {
                TransportType::Bus(_) => bus_count += 1,
                TransportType::Train(_) => train_count += 1,
                TransportType::Truck(_) => truck_count += 1,
                TransportType::Drone(_) => {} // Not used in this example
            }
        }

        assert!(bus_count >= 4, "Should have at least 4 bus connections");
        assert!(train_count >= 2, "Should have at least 2 train connections");
        assert!(truck_count >= 3, "Should have at least 3 truck connections");
    }

    #[test]
    fn test_power_balance_calculation() {
        let mut engine = create_sample_factory_setup();
        
        // Update to calculate power consumption and generation
        let global_items = engine.update();
        
        // Check that we have some items calculated
        assert!(!global_items.is_empty(), "Should have calculated global items");
        
        // Check power statistics
        let power_stats = engine.global_power_stats();
        assert!(power_stats.total_generation > 0.0, "Should have power generation");
        assert!(power_stats.total_consumption > 0.0, "Should have power consumption");
        
        // Most factories should be power-positive or close to balanced
        for factory_stat in &power_stats.factory_stats {
            assert!(factory_stat.generation > 0.0, 
                "Factory {} should generate power", factory_stat.factory_name);
            assert!(factory_stat.consumption > 0.0, 
                "Factory {} should consume power", factory_stat.factory_name);
        }
    }

    #[test]
    fn test_item_balance_calculation() {
        let mut engine = create_sample_factory_setup();
        
        // Update to calculate item balances
        let global_items = engine.update();
        
        // Should have many different items tracked
        assert!(global_items.len() >= 20, "Should track at least 20 different items");
        
        // Should have basic materials
        assert!(global_items.contains_key(&Item::IronIngot), "Should track Iron Ingot");
        assert!(global_items.contains_key(&Item::CopperIngot), "Should track Copper Ingot");
        assert!(global_items.contains_key(&Item::SteelIngot), "Should track Steel Ingot");
        
        // Should have advanced materials
        assert!(global_items.contains_key(&Item::Plastic), "Should track Plastic");
        assert!(global_items.contains_key(&Item::Computer), "Should track Computer");
        assert!(global_items.contains_key(&Item::CircuitBoard), "Should track Circuit Board");
    }

    #[test]
    fn test_production_line_power_consumption() {
        let engine = create_sample_factory_setup();
        
        for factory in engine.factories.values() {
            let total_consumption = factory.total_power_consumption();
            assert!(total_consumption > 0.0, 
                "Factory {} should consume power", factory.name);
            
            // Check that production lines contribute to power consumption
            let production_power = factory.production_lines.values()
                .map(|line| line.total_power_consumption())
                .sum::<f32>();
            
            assert!(production_power > 0.0, 
                "Factory {} should have production power consumption", factory.name);
        }
    }

    #[test]
    fn test_raw_input_extraction_rates() {
        let engine = create_sample_factory_setup();
        
        // Northern Forest should have iron, copper, and limestone
        let northern_forest = engine.factories.values()
            .find(|f| f.name.contains("Northern Forest"))
            .expect("Should find Northern Forest factory");
        
        let mut has_iron = false;
        let mut has_copper = false;
        let mut has_limestone = false;
        
        for raw_input in northern_forest.raw_inputs.values() {
            match raw_input.item {
                Item::IronOre => has_iron = true,
                Item::CopperOre => has_copper = true,
                Item::Limestone => has_limestone = true,
                _ => {}
            }
        }
        
        assert!(has_iron, "Northern Forest should have iron ore input");
        assert!(has_copper, "Northern Forest should have copper ore input");
        assert!(has_limestone, "Northern Forest should have limestone input");
    }

    #[test]
    fn test_factory_interdependencies() {
        let engine = create_sample_factory_setup();
        
        // Check that factories have both inputs and outputs through logistics
        let mut input_factories = std::collections::HashSet::new();
        let mut output_factories = std::collections::HashSet::new();
        
        for logistics in engine.logistics_lines.values() {
            input_factories.insert(logistics.to_factory);
            output_factories.insert(logistics.from_factory);
        }
        
        // Most factories should both send and receive materials
        assert!(input_factories.len() >= 4, "Most factories should receive materials");
        assert!(output_factories.len() >= 4, "Most factories should send materials");
    }
}