//! Factory Demo Binary
//!
//! This is a standalone binary program that demonstrates the Satisflow engine
//! using the comprehensive factory example. It shows how to use the engine
//! in a real application.

use satisflow_engine::examples::{run_factory_demo, run_test_program};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "demo" => {
                println!("Running factory demo...\n");
                run_factory_demo();
            }
            "test" => {
                println!("Running comprehensive test program...\n");
                run_test_program();
            }
            "both" => {
                println!("Running both demo and test program...\n");
                run_factory_demo();
                println!("\n{}\n", "=".repeat(60));
                run_test_program();
            }
            _ => {
                print_usage();
            }
        }
    } else {
        print_usage();
    }
}

fn print_usage() {
    println!("Satisflow Engine Factory Demo");
    println!("Usage: factory_demo <command>");
    println!();
    println!("Commands:");
    println!("  demo  - Run the factory demonstration (shows calculated results)");
    println!("  test  - Run comprehensive test program (verifies all features)");
    println!("  both  - Run both demo and test program");
    println!();
    println!("Example:");
    println!("  factory_demo demo");
    println!("  factory_demo test");
}
