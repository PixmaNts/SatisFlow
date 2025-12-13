//! Save/Load Demo Binary
//!
//! This binary demonstrates the save and load functionality of the Satisflow engine.
//! It can create sample data, save it to a file, load it back, and verify the data.

use satisflow_engine::models::{
    logistics::{Bus, Conveyor, ConveyorSpeed, TransportType, TruckTransport},
    power_generator::{GeneratorGroup, GeneratorType, PowerGenerator},
    production_line::{MachineGroup, ProductionLine, ProductionLineRecipe},
    raw_input::{ExtractorType, Purity, RawInput},
    Item, Recipe,
};
use satisflow_engine::SatisflowEngine;
use std::env;
use std::path::Path;
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "demo" => run_demo(),
        "save" => {
            if args.len() < 3 {
                println!("Error: Please provide a file path for save");
                print_usage();
                return;
            }
            create_and_save(&args[2]);
        }
        "load" => {
            if args.len() < 3 {
                println!("Error: Please provide a file path to load");
                print_usage();
                return;
            }
            load_and_display(&args[2]);
        }
        "roundtrip" => {
            if args.len() < 3 {
                println!("Error: Please provide a file path for roundtrip test");
                print_usage();
                return;
            }
            test_roundtrip(&args[2]);
        }
        _ => {
            println!("Error: Unknown command '{}'", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Satisflow Engine Save/Load Demo");
    println!("Usage: save_load_demo <command> [file_path]");
    println!();
    println!("Commands:");
    println!("  demo             - Run full demo (save → load → verify)");
    println!("  save <file>      - Create sample data and save to file");
    println!("  load <file>      - Load and display data from file");
    println!("  roundtrip <file> - Test save/load roundtrip with verification");
    println!();
    println!("Examples:");
    println!("  save_load_demo demo");
    println!("  save_load_demo save my_factory.json");
    println!("  save_load_demo load my_factory.json");
    println!("  save_load_demo roundtrip test.json");
}

fn run_demo() {
    println!("{}", "=".repeat(60));
    println!("Satisflow Engine Save/Load Demo");
    println!("{}", "=".repeat(60));
    println!();

    let save_path = "demo_save.json";

    println!("Step 1: Creating sample factory data...");
    let engine = create_sample_engine();
    print_engine_summary(&engine);

    println!("\nStep 2: Saving to '{}'...", save_path);
    engine
        .save_to_file(Path::new(save_path))
        .expect("Failed to save");
    println!("✓ Saved successfully!");

    println!("\nStep 3: Loading from '{}'...", save_path);
    let loaded_engine =
        SatisflowEngine::load_from_file(Path::new(save_path)).expect("Failed to load");
    println!("✓ Loaded successfully!");

    println!("\nStep 4: Verifying loaded data...");
    print_engine_summary(&loaded_engine);

    println!("\nStep 5: Comparing original and loaded data...");
    verify_engines_match(&engine, &loaded_engine);

    println!("\n{}", "=".repeat(60));
    println!("Demo completed successfully!");
    println!("Save file: {}", save_path);
    println!("{}", "=".repeat(60));
}

fn create_and_save(file_path: &str) {
    println!("Creating sample factory data...");
    let engine = create_sample_engine();

    println!("Saving to '{}'...", file_path);
    engine
        .save_to_file(Path::new(file_path))
        .expect("Failed to save");

    println!("✓ Saved successfully!");
    print_engine_summary(&engine);
}

fn load_and_display(file_path: &str) {
    println!("Loading from '{}'...", file_path);
    let engine = SatisflowEngine::load_from_file(Path::new(file_path)).expect("Failed to load");

    println!("✓ Loaded successfully!");
    print_engine_summary(&engine);
}

fn test_roundtrip(file_path: &str) {
    println!("Running roundtrip test...");
    println!();

    println!("Creating sample data...");
    let original_engine = create_sample_engine();

    println!("Saving to '{}'...", file_path);
    original_engine
        .save_to_file(Path::new(file_path))
        .expect("Failed to save");

    println!("Loading from '{}'...", file_path);
    let loaded_engine =
        SatisflowEngine::load_from_file(Path::new(file_path)).expect("Failed to load");

    println!("Verifying data integrity...");
    verify_engines_match(&original_engine, &loaded_engine);

    println!("\n✓ Roundtrip test passed! Data integrity verified.");
}

fn create_sample_engine() -> SatisflowEngine {
    let mut engine = SatisflowEngine::new();

    // Create iron factory
    let iron_factory_id = engine.create_factory(
        "Iron Processing Plant".to_string(),
        Some("Main iron smelting facility".to_string()),
    );

    if let Some(factory) = engine.get_factory_mut(iron_factory_id) {
        // Add raw input - Iron Ore mining
        let iron_miner = RawInput::new(
            Uuid::new_v4(),
            ExtractorType::MinerMk2,
            Item::IronOre,
            Some(Purity::Pure),
            100.0,
            1,
        )
        .expect("Failed to create miner");
        factory
            .add_raw_input(iron_miner)
            .expect("Fail to add raw input iron");

        // Add production line - Iron Ingot smelting
        let mut iron_ingot_line = ProductionLineRecipe::new(
            Uuid::new_v4(),
            "Iron Ingot Production".to_string(),
            Some("Smelting iron ore".to_string()),
            Recipe::IronIngot,
        );
        iron_ingot_line
            .add_machine_group(MachineGroup {
                number_of_machine: 10,
                oc_value: 100.0,
                somersloop: 0,
            })
            .expect("Failed to add machine group");
        factory.add_production_line(ProductionLine::ProductionLineRecipe(iron_ingot_line));

        // Add power generator - Coal
        let mut coal_gen = PowerGenerator::new(Uuid::new_v4(), GeneratorType::Coal, Item::Coal)
            .expect("Failed to create generator");
        coal_gen
            .add_group(GeneratorGroup {
                number_of_generators: 8,
                clock_speed: 100.0,
            })
            .expect("Failed to add generator group");
        factory
            .add_power_generator(coal_gen)
            .expect("Failed to add generator to engine");
    }

    // Create steel factory
    let steel_factory_id = engine.create_factory(
        "Steel Processing Plant".to_string(),
        Some("Advanced steel production".to_string()),
    );

    if let Some(factory) = engine.get_factory_mut(steel_factory_id) {
        // Add coal input
        let coal_miner = RawInput::new(
            Uuid::new_v4(),
            ExtractorType::MinerMk1,
            Item::Coal,
            Some(Purity::Normal),
            100.0,
            1,
        )
        .expect("Failed to create miner");
        factory
            .add_raw_input(coal_miner)
            .expect("Fail to add raw input coal");

        // Add steel ingot production
        let mut steel_ingot_line = ProductionLineRecipe::new(
            Uuid::new_v4(),
            "Steel Ingot Production".to_string(),
            Some("Making steel from iron and coal".to_string()),
            Recipe::SteelIngot,
        );
        steel_ingot_line
            .add_machine_group(MachineGroup {
                number_of_machine: 5,
                oc_value: 100.0,
                somersloop: 0,
            })
            .expect("Failed to add machine group");
        factory.add_production_line(ProductionLine::ProductionLineRecipe(steel_ingot_line));

        // Add fuel generator
        let mut fuel_gen = PowerGenerator::new(Uuid::new_v4(), GeneratorType::Fuel, Item::Fuel)
            .expect("Failed to create generator");
        fuel_gen
            .add_group(GeneratorGroup {
                number_of_generators: 4,
                clock_speed: 100.0,
            })
            .expect("Failed to add generator group");
        factory
            .add_power_generator(fuel_gen)
            .expect("Fail to add power generator");
    }

    // Create logistics line - Iron transport
    let bus_transport =
        TransportType::Bus(Bus::new(1, "Iron Ingot Bus").with_conveyor(Conveyor::new(
            1,
            ConveyorSpeed::Mk3,
            Item::IronIngot,
            270.0,
        )));

    engine
        .create_logistics_line(
            iron_factory_id,
            steel_factory_id,
            bus_transport,
            "Iron Ingot Bus".to_string(),
        )
        .expect("Failed to create logistics");

    // Create logistics line - Coal transport
    let truck_transport = TransportType::Truck(TruckTransport::new(2, Item::Coal, 60.0));

    engine
        .create_logistics_line(
            steel_factory_id,
            iron_factory_id,
            truck_transport,
            "Coal Truck Route".to_string(),
        )
        .expect("Failed to create logistics");

    engine
}

fn print_engine_summary(engine: &SatisflowEngine) {
    println!();
    println!("Engine Summary:");
    println!("  Factories: {}", engine.get_all_factories().len());
    println!("  Logistics Lines: {}", engine.get_all_logistics().len());

    let power_stats = engine.global_power_stats();
    println!("  Power Generation: {:.2} MW", power_stats.total_generation);
    println!(
        "  Power Consumption: {:.2} MW",
        power_stats.total_consumption
    );
    println!(
        "  Power Balance: {:.2} MW",
        power_stats.total_generation - power_stats.total_consumption
    );

    println!("\nFactories:");
    for (id, factory) in engine.get_all_factories() {
        println!("  - {} (ID: {})", factory.name, id);
        println!("    Raw Inputs: {}", factory.raw_inputs.len());
        println!("    Production Lines: {}", factory.production_lines.len());
        println!("    Power Generators: {}", factory.power_generators.len());
    }

    println!("\nLogistics Lines:");
    for logistics in engine.get_all_logistics().values() {
        let from_factory = engine
            .get_factory(logistics.from_factory)
            .map(|f| f.name.as_str())
            .unwrap_or("Unknown");
        let to_factory = engine
            .get_factory(logistics.to_factory)
            .map(|f| f.name.as_str())
            .unwrap_or("Unknown");
        println!(
            "  - {} → {} ({})",
            from_factory, to_factory, logistics.transport_details
        );
    }
}

fn verify_engines_match(original: &SatisflowEngine, loaded: &SatisflowEngine) {
    let original_factories = original.get_all_factories();
    let loaded_factories = loaded.get_all_factories();

    assert_eq!(
        original_factories.len(),
        loaded_factories.len(),
        "Factory count mismatch"
    );
    println!("  ✓ Factory count matches: {}", original_factories.len());

    let original_logistics = original.get_all_logistics();
    let loaded_logistics = loaded.get_all_logistics();

    assert_eq!(
        original_logistics.len(),
        loaded_logistics.len(),
        "Logistics count mismatch"
    );
    println!("  ✓ Logistics count matches: {}", original_logistics.len());

    for (id, original_factory) in original_factories {
        let loaded_factory = loaded_factories
            .get(id)
            .unwrap_or_else(|| panic!("Factory {} not found in loaded data", id));

        assert_eq!(
            original_factory.name, loaded_factory.name,
            "Factory name mismatch"
        );
        assert_eq!(
            original_factory.raw_inputs.len(),
            loaded_factory.raw_inputs.len(),
            "Raw input count mismatch for factory {}",
            original_factory.name
        );
        assert_eq!(
            original_factory.production_lines.len(),
            loaded_factory.production_lines.len(),
            "Production line count mismatch for factory {}",
            original_factory.name
        );
        assert_eq!(
            original_factory.power_generators.len(),
            loaded_factory.power_generators.len(),
            "Power generator count mismatch for factory {}",
            original_factory.name
        );
    }

    println!("  ✓ All factory details match");
    println!("  ✓ Data integrity verified!");
}
