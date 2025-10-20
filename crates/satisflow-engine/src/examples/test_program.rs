//! Test program that demonstrates using the factory example
//! 
//! This program creates a sample factory setup and performs various
//! operations to verify the engine works correctly in practice.

use crate::examples::create_sample_factory_setup;
use crate::models::{Item, logistics::Transport};

/// Main test function that runs through all engine features
pub fn run_test_program() {
    println!("=== Satisflow Engine Test Program ===\n");
    
    // Create the factory setup
    println!("1. Creating sample factory setup...");
    let mut engine = create_sample_factory_setup();
    
    // Verify factories were created
    println!("   Created {} factories", engine.factories.len());
    assert_eq!(engine.factories.len(), 5, "Should have exactly 5 factories");
    println!("   ✓ Factory creation successful\n");
    
    // Verify logistics connections
    println!("2. Verifying logistics connections...");
    println!("   Created {} logistics connections", engine.logistics_lines.len());
    assert!(!engine.logistics_lines.is_empty(), "Should have logistics connections");
    println!("   ✓ Logistics connections created\n");
    
    // Test item balance calculation
    println!("3. Testing item balance calculation...");
    let global_items = engine.update();
    println!("   Calculated balances for {} different items", global_items.len());
    assert!(!global_items.is_empty(), "Should have calculated items");
    
    // Check for specific key items
    let key_items = [
        Item::IronIngot,
        Item::CopperIngot,
        Item::SteelIngot,
        Item::Plastic,
        Item::Computer,
    ];
    
    for item in &key_items {
        assert!(global_items.contains_key(item), "Should track {:?}", item);
    }
    println!("   ✓ Item balance calculation working\n");
    
    // Test power statistics
    println!("4. Testing power statistics...");
    let power_stats = engine.global_power_stats();
    println!("   Total Generation: {:.1} MW", power_stats.total_generation);
    println!("   Total Consumption: {:.1} MW", power_stats.total_consumption);
    
    assert!(power_stats.total_generation > 0.0, "Should generate power");
    assert!(power_stats.total_consumption > 0.0, "Should consume power");
    assert_eq!(power_stats.factory_stats.len(), 5, "Should have stats for all factories");
    println!("   ✓ Power statistics calculation working\n");
    
    // Test factory details
    println!("5. Testing factory details...");
    for (id, factory) in &engine.factories {
        println!("   Factory {}: {}", id, factory.name);
        println!("     Production lines: {}", factory.production_lines.len());
        println!("     Raw inputs: {}", factory.raw_inputs.len());
        println!("     Power generators: {}", factory.power_generators.len());
        
        assert!(!factory.production_lines.is_empty(), "Factory should have production lines");
        assert!(!factory.power_generators.is_empty(), "Factory should have power generators");
        
        // Test power consumption calculation
        let power_consumption = factory.total_power_consumption();
        assert!(power_consumption > 0.0, "Factory should consume power");
        println!("     Power consumption: {:.1} MW", power_consumption);
    }
    println!("   ✓ Factory details verified\n");
    
    // Test logistics details
    println!("6. Testing logistics details...");
    for (id, logistics) in &engine.logistics_lines {
        let items = logistics.transport_type.get_items();
        println!("   Logistics {}: {} -> {} ({} items)", 
            id, logistics.from_factory, logistics.to_factory, items.len());
        
        assert!(!items.is_empty(), "Logistics should transport items");
    }
    println!("   ✓ Logistics details verified\n");
    
    // Test production line details
    println!("7. Testing production line details...");
    let mut total_production_lines = 0;
    for factory in engine.factories.values() {
        total_production_lines += factory.production_lines.len();
        
        for (line_id, line) in &factory.production_lines {
            let inputs = line.input_rate();
            let outputs = line.output_rate();
            let power = line.total_power_consumption();
            
            println!("     Line {}: {} inputs, {} outputs, {:.1} MW power", 
                line_id, inputs.len(), outputs.len(), power);
            
            assert!(!inputs.is_empty() || !outputs.is_empty(), 
                "Production line should have inputs or outputs");
            assert!(power >= 0.0, "Power consumption should be non-negative");
        }
    }
    println!("   Total production lines across all factories: {}", total_production_lines);
    println!("   ✓ Production line details verified\n");
    
    // Test raw input details
    println!("8. Testing raw input details...");
    let mut total_raw_inputs = 0;
    for factory in engine.factories.values() {
        total_raw_inputs += factory.raw_inputs.len();
        
        for (input_id, raw_input) in &factory.raw_inputs {
            println!("     Input {}: {:?} at {:.1}/min", 
                input_id, raw_input.item, raw_input.quantity_per_min);
            
            assert!(raw_input.quantity_per_min > 0.0, 
                "Raw input should have positive extraction rate");
        }
    }
    println!("   Total raw inputs across all factories: {}", total_raw_inputs);
    println!("   ✓ Raw input details verified\n");
    
    println!("=== All Tests Passed Successfully! ===");
    println!("The factory example demonstrates all major features of the Satisflow engine:");
    println!("✓ Factory creation and management");
    println!("✓ Production lines with recipes and machine groups");
    println!("✓ Raw inputs with extraction rates");
    println!("✓ Power generation and consumption calculation");
    println!("✓ Logistics connections between factories");
    println!("✓ Global item balance calculation");
    println!("✓ Comprehensive statistics and reporting");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_program_runs() {
        run_test_program();
    }
}