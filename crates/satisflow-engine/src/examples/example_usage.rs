//! Example usage of the Satisflow engine with the factory example
//!
//! This module demonstrates how to use the factory example to create
//! a complete factory setup and analyze the results.

use crate::{examples::factory_example::create_sample_factory_setup, models::logistics::Transport};

/// Run a demonstration of the Satisflow engine with the factory example
pub fn run_factory_demo() {
    println!("=== Satisflow Engine Factory Demo ===\n");

    // Create the factory setup
    let mut engine = create_sample_factory_setup();

    // Display factory information
    println!("Created {} factories:", engine.factories.len());
    for (id, factory) in &engine.factories {
        println!(
            "  Factory {}: {} ({} production lines, {} power generators)",
            id,
            factory.name,
            factory.production_lines.len(),
            factory.power_generators.len()
        );
    }
    println!();

    // Display logistics information
    println!(
        "Created {} logistics connections:",
        engine.logistics_lines.len()
    );
    for (id, logistics) in &engine.logistics_lines {
        println!(
            "  Logistics {}: {} -> {} ({})",
            id,
            logistics.from_factory,
            logistics.to_factory,
            logistics.transport_type.get_transport_type_name()
        );
    }
    println!();

    // Update and calculate item balances
    println!("Calculating item balances...");
    let global_items = engine.update();

    // Display some key items
    let key_items = [
        crate::models::Item::IronIngot,
        crate::models::Item::CopperIngot,
        crate::models::Item::SteelIngot,
        crate::models::Item::Plastic,
        crate::models::Item::Computer,
        crate::models::Item::CircuitBoard,
    ];

    println!("Key item balances:");
    for item in &key_items {
        if let Some(quantity) = global_items.get(item) {
            let status = if *quantity > 0.0 {
                "Surplus"
            } else if *quantity < 0.0 {
                "Deficit"
            } else {
                "Balanced"
            };
            println!("  {}: {:.2}/min ({})", item, quantity, status);
        }
    }
    println!();

    // Display power statistics
    println!("Power statistics:");
    let power_stats = engine.global_power_stats();
    println!("  Total Generation: {:.1} MW", power_stats.total_generation);
    println!(
        "  Total Consumption: {:.1} MW",
        power_stats.total_consumption
    );
    println!("  Power Balance: {:.1} MW", power_stats.power_balance);

    println!("\nFactory power details:");
    for factory_stat in &power_stats.factory_stats {
        let balance_status = if factory_stat.balance > 0.0 {
            "Surplus"
        } else if factory_stat.balance < 0.0 {
            "Deficit"
        } else {
            "Balanced"
        };
        println!(
            "  {}: {:.1} MW gen, {:.1} MW cons, {:.1} MW ({})",
            factory_stat.factory_name,
            factory_stat.generation,
            factory_stat.consumption,
            factory_stat.balance,
            balance_status
        );
    }

    println!("\n=== Demo Complete ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_demo_runs() {
        // This test just verifies the demo runs without panicking
        run_factory_demo();
    }
}
