# Satisfflow Development Guide

**Last Updated**: 2025-10-27

## Project Setup

### Prerequisites

- **Rust**: 1.70+ (stable)
- **Node.js**: 18+ (for frontend development)
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

### Starting Development Servers

```bash
# Terminal 1: Backend (Port 3000)
cd crates/satisflow-server
cargo run

# Terminal 2: Frontend (Port 5173)
cd frontend
npm run dev

# Browser: http://localhost:5173
```

### Testing Commands

```bash
# Backend tests
cd crates/satisflow-server
cargo test

# Frontend unit tests
cd frontend
npm run test
npm run test:watch

# Frontend E2E tests
npm run test:e2e
```

## Project Structure

```text
Satisflow/
├── .kilocode/                    # Kilo Code configuration
│   └── rules/
│       └── memory-bank/          # Project documentation
│           ├── brief.md          # Project overview and requirements
│           ├── architecture.md   # Technical architecture details
│           ├── implementation-status.md  # Current progress tracking
│           └── development-guide.md      # This file
├── crates/
│   ├── satisflow-engine/         # Core Rust engine
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # SatisflowEngine main API
│   │       ├── models/            # Data models and game data
│   │       ├── examples/         # Example implementations and demos
│   │       └── bin/              # Executable binaries
│   └── satisflow-server/          # Backend REST API server
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs           # Server entry point
│           ├── lib.rs            # Server library
│           ├── state.rs          # Application state management
│           ├── error.rs          # Error handling
│           └── handlers/         # API route handlers
├── frontend/                     # Vue.js frontend application
│   ├── src/
│   │   ├── components/           # Reusable Vue components
│   │   ├── views/                # Main application views
│   │   ├── stores/               # Pinia state management
│   │   ├── composables/          # Vue composables
│   │   ├── api/                  # API client and types
│   │   └── router/               # Vue Router configuration
│   ├── package.json
│   └── vite.config.ts
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

```text
feat: add raw input system with purity levels
fix: correct power calculation for somersloop multiplier
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

### WASM Integration (Optional)

The current REST API architecture provides excellent debugging, multi-user support, and standard web patterns. WASM integration can be added later for offline mode if needed.

**WASM Preparation Checklist**:

- [x] No `std::thread` usage (pure computation)
- [x] No file I/O (`std::fs`) - serialization only
- [x] All external dependencies support WASM
- [x] Use `wasm-bindgen` compatible types
- [x] Deterministic calculations (no random numbers)

### Vue.js Integration Points

The frontend integrates with the backend REST API through a comprehensive API layer:

**API Client** (`frontend/src/api/client.ts`):

- Axios-based HTTP client with interceptors
- Automatic error handling and retry logic
- Type-safe request/response handling

**API Endpoints** (`frontend/src/api/endpoints.ts`):

- Complete CRUD operations for all entities
- Dashboard data aggregation endpoints
- Game data endpoints (recipes, items, machines)

**State Management** (`frontend/src/stores/`):

- Pinia stores for factories, logistics, dashboard, game data, preferences
- Reactive data management with local storage persistence
- Optimistic updates and error recovery

**Example Integration**:

```typescript
// API usage in stores
const factoryStore = useFactoryStore()
await factoryStore.create({
  name: 'Iron Processing',
  description: 'Main iron production facility'
})

// Reactive data in components
const factories = computed(() => factoryStore.factories)
const loading = computed(() => factoryStore.loading)
```

## Frontend Components

### Key UI Components

**RecipeAutocomplete** (`frontend/src/components/factory/RecipeAutocomplete.vue`)

- Purpose: Enhanced recipe selection with intelligent search and rich details display
- Features:
  - Real-time filtering by recipe name OR machine type
  - Rich details in dropdown (inputs/outputs with quantities)
  - Full keyboard navigation (↑↓ Enter Escape Tab)
  - WCAG accessibility compliance with ARIA labels
  - Performance optimized for 1000+ recipes (max 8 suggestions)
  - Case-insensitive substring matching
  - Clear button for quick reset
  - Mouse and keyboard support
- Usage: Integrated in ProductionLineForm for recipe selection
- Status: ✅ Complete and Production Ready

**Complete Component Architecture**:

The frontend includes 88 files with ~14,100 lines of production-ready code:

- **Views**: DashboardView, FactoryView, LogisticsView with full functionality
- **Components**: 40+ reusable components with proper separation of concerns
- **State Management**: Pinia stores for factories, logistics, dashboard, game data, preferences
- **API Layer**: Complete REST API integration with error handling
- **Testing**: Vitest unit tests and Playwright E2E tests configured

**Integration Pattern**:

```vue
<RecipeAutocomplete
  v-model="selectedRecipe"
  :recipes="availableRecipes"
  @selected="onRecipeSelected"
/>
```

For more details, see the component documentation and implementation status.

## Documentation and Examples

### Project Documentation

The project includes comprehensive documentation in the `.kilocode/rules/memory-bank/` directory:

- **implementation-status.md**: Current progress tracking and feature status
- **architecture.md**: Technical architecture and design decisions
- **brief.md**: Project overview and requirements
- **development-guide.md**: Development workflow and guidelines (this file)
- **api-and-testing.md**: Complete API contract and testing infrastructure

### Additional Documentation

- **CALCULATION_MIGRATION_PLAN.md**: Plan for migrating calculations from frontend to backend
- **CALCULATION_MIGRATION_SUMMARY.md**: Summary of calculation migration work
- **FRONTEND_CALCULATION_REVIEW.md**: Review of frontend calculation implementation
- **MIGRATION-STRATEGY.md**: Comprehensive migration strategy for save file versions
- **SAVE-LOAD-IMPLEMENTATION.md**: Complete implementation documentation for save/load functionality
- **BLUEPRINT_LIBRARY_IMPLEMENTATION.md**: Implementation guide for blueprint library system

### Blueprint Examples

Ready-to-use blueprint examples are available in the project root:

- **example-blueprint-iron-plates.json**: Example blueprint for iron plate production
- **example-blueprint-motor-production.json**: Example blueprint for motor production
- **example-blueprint-reinforced-plates.json**: Example blueprint for reinforced plate production

### Archive Documentation

Historical documentation is preserved in `docs/archive/`:

- **BLUEPRINT_FEATURE_STATUS.md**: Status tracking for blueprint feature development
- **docs/archive/blueprint-v1-import-export/**: Complete blueprint import/export documentation

## Resources

- [Satisfactory Wiki](https://satisfactory.wiki.gg/) - Game mechanics reference
- [Rust Book](https://doc.rust-lang.org/book/) - Rust language guide
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/) - Rust/WASM interop
- [Vue.js Docs](https://vuejs.org/) - UI framework

## Getting Help

- **Project Issues**: Check `.kilocode/rules/memory-bank/implementation-status.md`
- **Architecture Questions**: See `.kilocode/rules/memory-bank/architecture.md`
- **Game Mechanics**: Consult `.kilocode/rules/memory-bank/brief.md`
- **Development Workflow**: See `.kilocode/rules/memory-bank/development-guide.md`
- **API Documentation**: Check `.kilocode/rules/memory-bank/api-and-testing.md`
