// Test runner utility for satisflow-server
use std::process::Command;
use std::env;

fn main() {
    println!("Running Satisflow Server API Tests");
    println!("==================================");
    
    let args: Vec<String> = env::args().collect();
    
    // Run tests with different options based on arguments
    let mut cmd = Command::new("cargo");
    cmd.arg("test");
    cmd.arg("--package");
    cmd.arg("satisflow-server");
    
    if args.len() > 1 {
        match args[1].as_str() {
            "verbose" => {
                cmd.arg("--");
                cmd.arg("--nocapture");
            }
            "single" => {
                if args.len() > 2 {
                    cmd.arg("--");
                    cmd.arg("--exact");
                    cmd.arg(&args[2]);
                }
            }
            "dashboard" => {
                cmd.arg("--");
                cmd.arg("--exact");
                cmd.arg("test_dashboard_endpoints");
            }
            "factory" => {
                cmd.arg("--");
                cmd.arg("--exact");
                cmd.arg("test_factory_crud_operations");
            }
            "logistics" => {
                cmd.arg("--");
                cmd.arg("--exact");
                cmd.arg("test_logistics_crud_operations");
            }
            "game_data" => {
                cmd.arg("--");
                cmd.arg("--exact");
                cmd.arg("test_game_data_endpoints");
            }
            "cors" => {
                cmd.arg("--");
                cmd.arg("--exact");
                cmd.arg("test_cors_headers");
            }
            "errors" => {
                cmd.arg("--");
                cmd.arg("--exact");
                cmd.arg("test_error_response_format");
            }
            _ => {
                print_usage();
                return;
            }
        }
    }
    
    let status = cmd.status().expect("Failed to execute cargo test");
    
    if status.success() {
        println!("\n✅ All tests passed!");
    } else {
        println!("\n❌ Some tests failed!");
        std::process::exit(1);
    }
}

fn print_usage() {
    println!("Usage: cargo run --bin test_runner [OPTION]");
    println!();
    println!("Options:");
    println!("  verbose    Run tests with verbose output");
    println!("  single <test_name>  Run a specific test");
    println!("  dashboard  Run dashboard endpoint tests");
    println!("  factory    Run factory CRUD tests");
    println!("  logistics  Run logistics endpoint tests");
    println!("  game_data  Run game data endpoint tests");
    println!("  cors       Run CORS functionality tests");
    println!("  errors     Run error handling tests");
    println!();
    println!("Examples:");
    println!("  cargo run --bin test_runner verbose");
    println!("  cargo run --bin test_runner factory");
    println!("  cargo run --bin test_runner single test_health_check");
}