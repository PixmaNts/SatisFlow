# Satisfflow Development Guide

## Project Setup

### Prerequisites

- **Rust**: 1.70+ (stable)
- **Node.js**: 18+ (for future UI development)
- **Git**: For version control

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/PixmaNts/SatisFlow.git
cd SatisFlow

# Build the engine
cargo build --package satisflow-engine

# Run tests
cargo test --package satisflow-engine

# Build in release mode
cargo build --package satisflow-engine --release
```

## Project Structure

```
Satisflow/
├── .kilocode/                    # Kilo Code configuration
│   └── rules/
│       └── memory-bank/          # Project documentation
│           ├── brief.md          # Project overview and requirements
│           ├── architecture.md   # Technical architecture details
│           ├── implementation-status.md  # Current progress tracking
│           └── development-guide.md      # This file
├── crates/
│   └── satisflow-engine/         # Core Rust engine
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs            # SatisflowEngine main API
│           └── models/
│               ├── mod.rs
│               ├── factory.rs
│               ├── logistics.rs
│               ├── production_line.rs
│               ├── game_data.rs
│               ├── items.rs
│               ├── recipes.rs
│               ├── items_data.inc
│               └── recipes_data.inc
├── satisflow-engine-old/         # Legacy code (for reference)
└── Cargo.toml                    # Workspace configuration
```

## Development Workflow

### 1. Feature Development (TDD Approach)

The project follows **Test-Driven Development**:

```bash
# 1. Write failing tests first
# Edit: crates/satisflow-engine/src/models/<module>.rs
# Add test in #[cfg(test)] mod tests { ... }

# 2. Run tests (should fail)
cargo test --package satisflow-engine <test_name>

# 3. Implement feature
# Edit the actual implementation

# 4. Run tests again (should pass)
cargo test --package satisflow-engine <test_name>

# 5. Run all tests
cargo test --package satisflow-engine
```

### 2. Code Style

**Rust Conventions**:
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy -- -D warnings`
- Document public APIs with `///` comments

**Naming Conventions**:
- Types: `PascalCase` (e.g., `ProductionLine`)
- Functions: `snake_case` (e.g., `create_factory`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MK1_SPEED`)
- Enums: `PascalCase` variants (e.g., `TransportType::Bus`)

### 3. Testing Guidelines

**Test Organization**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        // Arrange
        let value = create_test_value();
        
        // Act
        let result = value.process();
        
        // Assert
        assert_eq!(result, expected);
    }
    
    #[test]
    #[should_panic(expected = "error message")]
    fn test_error_condition() {
        // Test that should panic
    }
}
```

**Test Coverage Goals**:
- Unit tests for all public functions
- Edge cases for validation logic
- Error paths for Result types
- Integration tests for SatisflowEngine workflows

### 4. Adding New Features

#### Example: Adding a New Transport Type

```rust
// 1. Define the type in models/logistics.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewTransport {
    pub transport_id: u64,
    pub item: Item,
    pub quantity_per_min: f32,
}

// 2. Add to TransportType enum
pub enum TransportType {
    Bus(Bus),
    Train(Train),
    Truck(TruckTransport),
    Drone(DroneTransport),
    NewTransport(NewTransport), // Add here
}

// 3. Implement Transport trait
impl Transport for NewTransport {
    fn get_items(&self) -> Vec<ItemFlow> {
        vec![ItemFlow {
            item: self.item.clone(),
            quantity_per_min: self.quantity_per_min,
        }]
    }
    
    fn get_transport_id(&self) -> String {
        format!("NEW-{}", self.transport_id)
    }
    
    fn get_transport_name(&self) -> Option<String> {
        None
    }
    
    fn get_transport_type_name(&self) -> &'static str {
        "NewTransport"
    }
}

// 4. Update TransportType trait delegation
impl Transport for TransportType {
    fn get_items(&self) -> Vec<ItemFlow> {
        match self {
            // ... existing matches
            TransportType::NewTransport(t) => t.get_items(),
        }
    }
    // ... update other methods
}

// 5. Write tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_new_transport() {
        let transport = NewTransport {
            transport_id: 1,
            item: Item::IronOre,
            quantity_per_min: 60.0,
        };
        
        let items = transport.get_items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].item, Item::IronOre);
    }
}
```

## Common Tasks

### Adding a New Recipe

Recipes are defined in `models/recipes_data.inc` using a macro format:

```rust
// In recipes_data.inc
IronIngot => {
    name: "Iron Ingot",
    machine: Smelter,
    inputs: [(IronOre, 30.0)],
    outputs: [(IronIngot, 30.0)],
}
```

Then add the variant to the `Recipe` enum in `models/recipes.rs`.

### Adding a New Item

Similar to recipes, items are defined in `models/items_data.inc`:

```rust
// In items_data.inc
IronOre => "Iron Ore",
```

### Running Specific Tests

```bash
# Run all tests
cargo test --package satisflow-engine

# Run tests in a specific module
cargo test --package satisfflow-engine logistics

# Run a specific test
cargo test --package satisflow-engine test_conveyor_speed_constants

# Run with output
cargo test --package satisflow-engine -- --nocapture
```

### Building for Different Targets

```bash
# Debug build (fast compilation, slower runtime)
cargo build --package satisflow-engine

# Release build (optimized)
cargo build --package satisflow-engine --release

# Check without building
cargo check --package satisfflow-engine

# Build documentation
cargo doc --package satisflow-engine --open
```

## Git Workflow

### Branch Naming

- `feature/<name>` - New features
- `fix/<name>` - Bug fixes
- `refactor/<name>` - Code refactoring
- `docs/<name>` - Documentation updates
- `test/<name>` - Test additions/improvements

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add raw input system with purity levels
fix: correct power calculation for sommersloop multiplier
docs: update architecture diagram with new components
test: add integration tests for factory calculations
refactor: extract logistics validation logic
```

### Pull Request Process

1. Create feature branch from `master`
2. Implement feature with tests
3. Run full test suite: `cargo test --package satisfflow-engine`
4. Format code: `cargo fmt`
5. Check lints: `cargo clippy`
6. Update relevant documentation in `.kilocode/rules/memory-bank/`
7. Create PR with description of changes
8. Address review comments
9. Squash merge to `master`

## Performance Optimization

### Profiling

```bash
# Build with debug symbols
cargo build --package satisflow-engine --release --profile release-with-debug

# Run with profiler (Linux)
perf record -g ./target/release-with-debug/satisflow-engine
perf report
```

### Benchmarking

```rust
// In benches/ directory (future)
#[bench]
fn bench_factory_calculation(b: &mut Bencher) {
    let mut engine = SatisflowEngine::new();
    let factory_id = engine.create_factory("Test".into(), None);
    
    b.iter(|| {
        engine.update();
    });
}
```

## Troubleshooting

### Common Issues

**Issue**: Cargo fails to compile with "cannot find type `Item`"  
**Solution**: Ensure `use crate::models::Item;` in your module

**Issue**: Tests fail with `MutexPoison` error  
**Solution**: Check for panics in shared state access, use `lock().unwrap()` carefully

**Issue**: Power calculation doesn't match game  
**Solution**: Verify formula: `base_power × (1 + s/max_s)² × (clock/100)^1.321928`

### Debug Logging

```rust
// Add debug prints in tests
#[test]
fn debug_calculation() {
    let factory = create_test_factory();
    println!("Items: {:?}", factory.items);
    factory.calculate_item();
    println!("After calculation: {:?}", factory.items);
}
```

## Future Development

### WASM Preparation Checklist

- [ ] No `std::thread` usage
- [ ] No file I/O (`std::fs`)
- [ ] All external dependencies support WASM
- [ ] Use `wasm-bindgen` compatible types
- [ ] Deterministic calculations (no random numbers)

### Vue.js Integration Points

The engine will expose these WASM APIs:

```rust
// Future API (pseudocode)
#[wasm_bindgen]
impl SatisflowEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self;
    
    #[wasm_bindgen]
    pub fn create_factory_js(name: String, desc: Option<String>) -> u64;
    
    #[wasm_bindgen]
    pub fn update_js() -> JsValue; // Returns JSON
}
```

## Resources

- [Satisfactory Wiki](https://satisfactory.wiki.gg/) - Game mechanics reference
- [Rust Book](https://doc.rust-lang.org/book/) - Rust language guide
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/) - Rust/WASM interop
- [Vue.js Docs](https://vuejs.org/) - Future UI framework

## Getting Help

- **Project Issues**: Check `.kilocode/rules/memory-bank/implementation-status.md`
- **Architecture Questions**: See `.kilocode/rules/memory-bank/architecture.md`
- **Game Mechanics**: Consult `.kilocode/rules/memory-bank/brief.md`