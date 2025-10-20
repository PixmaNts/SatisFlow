# Satisflow Engine Examples

This directory contains example implementations and demonstrations of how to use the Satisflow engine.

## Factory Example

The `factory_example.rs` module creates a comprehensive factory setup with 5 specialized factories:

1. **Northern Forest - Smelting Hub**: Primary smelting hub for iron and copper processing
   - Raw inputs: Iron ore, Copper ore, Limestone
   - Production lines: Iron ingot, Copper ingot, Iron rod, Iron plate, Wire, Cable, Screw, Reinforced iron plate, Rotor, Modular frame, Concrete
   - Power: Coal generators

2. **Central Assembly - Manufacturing Hub**: Advanced manufacturing hub for complex components
   - Production lines: Smart plating, Versatile framework, Heavy modular frame, Motor, Stator, Encased industrial beam, Automated wiring, Circuit board, AI limiter, Crystal oscillator, Supercomputer
   - Power: Fuel generators

3. **Oil Refinery - Petroleum Hub**: Petroleum processing and plastics production
   - Raw inputs: Crude oil, Water
   - Production lines: Plastic, Rubber, Fuel, Computer, High-speed connector, Beacon, Empty canister
   - Power: Fuel generators

4. **Steel Mill - Heavy Industry**: Heavy industry with steel and concrete
   - Raw inputs: Iron ore, Coal, Limestone, Bauxite
   - Production lines: Steel ingot, Steel beam, Steel pipe, Concrete, Encased industrial beam, Heavy modular frame, Aluminum ingot, Aluminum casing, Alclad aluminum sheet
   - Power: Coal generators

5. **Electronics Lab - High-Tech Hub**: High-tech components and computers
   - Raw inputs: Caterium ore, Raw quartz, Sulfur
   - Production lines: Circuit board, AI limiter, High-speed connector, Computer, Supercomputer, Crystal oscillator, Pressure conversion cube, Radio control unit, Beacon, Adaptive control unit, Assembly director system
   - Power: Fuel generators

## Logistics Network

The factories are connected through a comprehensive logistics network:

- **Bus connections**: Conveyor buses for high-volume item transport
- **Train connections**: Heavy material transport between major factories
- **Truck connections**: Specialized item transport for specific needs

## Usage

### Creating a Factory Setup

```rust
use satisflow_engine::examples::create_sample_factory_setup;

// Create a complete factory setup
let mut engine = create_sample_factory_setup();

// Update to calculate item balances and power consumption
let global_items = engine.update();

// Get power statistics
let power_stats = engine.global_power_stats();
```

### Running the Demo

```rust
use satisflow_engine::examples::run_factory_demo;

// Run a demonstration that prints factory information
run_factory_demo();
```

## Testing

The examples include comprehensive tests that verify:

- All factories are created correctly
- Production lines have proper configurations
- Logistics connections are properly established
- The overall system calculates item balances correctly
- Power generation and consumption are calculated correctly

Run the tests with:

```bash
cargo test examples
```

## Implementation Details

The factory example demonstrates all major features of the Satisflow engine:

- **Factory management**: Creating factories with production lines, raw inputs, and power generators
- **Production lines**: Recipe-based production with machine groups, overclocking, and somersloops
- **Raw inputs**: Resource extraction with different extractor types and purity levels
- **Power generation**: Various generator types with fuel consumption and waste production
- **Logistics**: Transport connections between factories using buses, trains, and trucks
- **Item calculation**: Global item balance calculation across all factories
- **Power calculation**: Power generation and consumption statistics

This example serves as a comprehensive reference for how to use the Satisflow engine to model complex factory setups in Satisfactory.