# Satisflow Architecture

## System Overview

Satisfflow is built with a **three-tier architecture** separating concerns between the frontend UI, backend REST API, and core engine.

```text
┌─────────────────────────────────────────┐
│      Frontend UI (Vue.js)               │
│    Vue 3 + TypeScript + Vite            │
│    - Dashboard View                     │
│    - Factory View                       │
│    - Logistics View                     │
└─────────────────────────────────────────┘
                    ↓ HTTP/REST
┌─────────────────────────────────────────┐
│      Backend Server (Axum)              │
│    Rust Web Server                      │
│    - REST API Endpoints                 │
│    - State Management                   │
│    - Error Handling                     │
│    - CORS & Logging                     │
└─────────────────────────────────────────┘
                    ↓ Direct calls
┌─────────────────────────────────────────┐
│      Satisfflow Engine (Rust)           │
│    Core Business Logic                  │
│    - Data Models                        │
│    - Calculations                       │
│    - Game Data                          │
└─────────────────────────────────────────┘
```

## Core Engine Components

### 1. SatisflowEngine (`lib.rs`)

**Purpose**: Main orchestrator managing factories and logistics networks

**Key Responsibilities**:

- Factory lifecycle management (CRUD)
- LogisticsFlux coordination between factories
- Global state updates and item aggregation

**Current API**:

- `create_factory(name, description) -> u64`
- `get_factory(id) -> Option<&Factory>`
- `get_factory_mut(id) -> Option<&mut Factory>`
- `create_logistics_line(from, to, transport_type, details) -> Result<u64, Error>`
- `get_logistics_line(id) -> Option<Arc<Mutex<LogisticsFlux>>>`
- `update() -> HashMap<Item, f32>` - Updates all factories and returns global item balance

**Architecture Decisions**:

- Uses `Arc<Mutex<>>` for shared ownership of LogisticsFlux between factories
- Factory IDs are simple u64 counters (currently sequential)
- Factories maintain references to both input and output logistics lines

### 2. Data Models (`models/`)

#### Factory (`models/factory.rs`)

**Structure**:

```rust
pub struct Factory {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub production_lines: HashMap<u64, Box<dyn ProductionLine>>,
    pub logistics_output: HashMap<u64, Arc<Mutex<LogisticsFlux>>>,
    pub logistics_input: HashMap<u64, Arc<Mutex<LogisticsFlux>>>,
    pub raw_inputs: HashMap<Item, f32>,
    pub items: HashMap<Item, f32>, // Calculated inventory
}
```

**Key Methods**:

- `add_production_line(line: Box<dyn ProductionLine>)` - Adds production and triggers recalculation
- `total_power_consumption() -> f32` - Aggregates power from all production lines
- `calculate_item()` - **Core calculation**: Computes net item balance from:
  - Raw inputs (+)
  - Logistics inputs (+)
  - Logistics outputs (-)
  - Production line inputs (-)
  - Production line outputs (+)

#### ProductionLine (`models/production_line.rs`)

**Trait**: Polymorphic interface for different production types

```rust
pub trait ProductionLine {
    fn id(&self) -> u64;
    fn total_machines(&self) -> u32;
    fn total_somersloop(&self) -> u32;
    fn output_rate(&self) -> Vec<(Item, f32)>;
    fn input_rate(&self) -> Vec<(Item, f32)>;
    fn total_power_consumption(&self) -> f32;
}
```

**Implementations**:

1. **ProductionLineRecipe**: Standard recipe-based production
   - Contains: `id, name, description, recipe, machine_groups: Vec<MachineGroup>`
   - MachineGroup: `(num_machines: u32, overclock: f32, somersloop: u8)`
   - Validates: Overclock (0-250%), Somersloop limits per machine type
   - Power formula: `base_power × (1 + s/max_s)² × (clock/100)^1.321928`

2. **ProductionLineBlueprint**: Composite of multiple recipes
   - Aggregates outputs/inputs from nested ProductionLineRecipe instances
   - Enables custom multi-step production tracking

#### Logistics (`models/logistics.rs`)

**Architecture**: Type-safe transport system with polymorphic design

```rust
pub enum TransportType {
    Bus(Bus),
    Train(Train),
    Truck(TruckTransport),
    Drone(DroneTransport),
}

pub struct LogisticsFlux {
    pub id: u64,
    pub from_factory: u64,
    pub to_factory: u64,
    pub transport_type: TransportType,
    pub transport_details: String,
}
```

**Transport Trait**: Unified interface for all transport types

```rust
pub trait Transport {
    fn get_items(&self) -> Vec<ItemFlow>;
    fn get_transport_id(&self) -> String;
    fn get_transport_name(&self) -> Option<String>;
    fn get_transport_type_name(&self) -> &'static str;
}
```

**Transport Capacity Constants**:

- Conveyors: Mk1(60), Mk2(120), Mk3(270), Mk4(480), Mk5(780), Mk6(1200) items/min
- Pipelines: Mk1(300), Mk2(600) m³/min

**Design Patterns**:

- **Composite**: Bus and Train contain multiple sub-transports (conveyors/wagons)
- **Polymorphism**: TransportType enum delegates to concrete implementations
- **Builder**: Bus/Train have `with_conveyor()`/`with_wagon()` fluent APIs

#### Game Data (`models/game_data.rs`, `models/recipes.rs`, `models/items.rs`)

**Purpose**: Static game content from Satisfactory

**MachineType**: Defines production machine characteristics

- Somersloop limits: Constructor(1), Assembler(2), Manufacturer(4), etc.
- Base power consumption: Constructor(4MW), Assembler(16MW), etc.

**Recipes**: Auto-generated from `recipes_data.inc`

- Uses macro `define_recipes!` for compile-time recipe database
- Lazy-loaded HashMaps for O(1) lookup by name or Recipe enum
- Each recipe defines: name, machine, inputs, outputs

**Items**: Similar pattern to recipes

- Lazy-loaded item database
- ItemParseError for validation

#### PowerGenerator (`models/power_generator.rs`)

**Structure**:

```rust
pub struct PowerGenerator {
    pub id: u64,
    pub generator_type: GeneratorType,
    pub fuel: Option<Item>,
    pub num_generators: u32,
    pub overclock: f32,
}

pub enum GeneratorType {
    BiomassBurner,
    CoalGenerator,
    FuelGenerator,
    NuclearPowerPlant,
    GeothermalGenerator,
}
```

**Purpose**: Power generation system separate from production lines

**Power Generation Mechanics**:

- **Biomass Burner**: 30 MW base, uses solid biofuel (Biomass, Solid Biofuel, etc.)
- **Coal Generator**: 75 MW base, uses Coal or Compacted Coal
- **Fuel Generator**: 150 MW base, uses Fuel, Turbofuel, or Liquid Biofuel
- **Nuclear Power Plant**: 2500 MW base, uses Uranium/Plutonium/Ficsonium Fuel Rods (produces waste)
- **Geothermal Generator**: 200 MW fixed (no fuel, no overclocking)

**Important Notes**:

- Power generators have different overclocking behavior than consumers
- Fuel consumption rate scales proportionally with power production
- Overclocking doesn't increase fuel efficiency (burns fuel faster/slower)
- Nuclear generators produce waste items (Uranium/Plutonium Waste)

**Power Calculation**:
```
Power = Base Power × (overclock/100) × num_generators
```

**Fuel Consumption**:
```
Fuel Rate = Base Fuel Rate × fuel_multiplier × (overclock/100) × num_generators
```

#### Raw Input (`models/raw_input.rs`)

**Structure**:

```rust
pub struct RawInput {
    pub id: u64,
    pub extractor_type: ExtractorType,
    pub item: Item,
    pub purity: Option<Purity>,
    pub quantity_per_min: f32,
}

pub enum ExtractorType {
    MinerMk1,
    MinerMk2,
    MinerMk3,
    WaterExtractor,
    OilExtractor,
    ResourceWellExtractor,
}

pub enum Purity {
    Impure,   // 50% yield
    Normal,   // 100% yield
    Pure,     // 200% yield
}
```

**Purpose**: Raw inputs represents a resource extraction source in the game

**Extraction Mechanics**:

- **Solid Resources** (Iron Ore, Copper Ore, etc.): Use Miners (Mk1/Mk2/Mk3) with purity affecting yield
- **Liquids** (Water): Use Water Extractor at fixed 120 m³/min (no purity concept)
- **Oil**: Use Oil Extractor with purity (Impure: 60, Normal: 120, Pure: 240 m³/min)
- **Gases** (Nitrogen, etc.): Use Resource Well Extractor with purity

##### Resource Well Pressurizer System

**Purpose**: Advanced extraction mechanics for Resource Wells with satellite nodes

**Key Components**:

- **ResourceWellPressurizer**: Main building with 150MW base power consumption
  - Overclocking: 0.000-250.000 (same as other machines)
  - Power formula: `base_power × (clock/100)^1.321928`
  - Controls all satellite extractors

- **ResourceWellExtractor**: Satellite nodes powered by pressurizer
  - No individual power consumption
  - Individual purity per node
  - Base rates: Impure 30, Normal 60, Pure 120 m³/min

**System Logic**:

- Extractors can only exist with a pressurizer
- Clock speed affects all extractors simultaneously
- Total extraction rate = sum of all extractor rates
- Power consumption = pressurizer only

**Validation**:

- Extractor-resource compatibility
- Clock speed range validation
- Pressurizer requirement enforcement

#### Blueprint Library System (`models/blueprint_templates.rs`)

**Purpose**: Centralized storage and management of reusable production line blueprints

**Architecture Pattern**: Template vs Instance

```rust
// Engine structure
pub struct SatisflowEngine {
    factories: HashMap<Uuid, Factory>,
    logistics: Vec<Logistics>,
    blueprint_templates: HashMap<Uuid, ProductionLineBlueprint>,  // NEW
}
```

**Key Concepts**:

1. **Template**: Blueprint stored in library (reusable, editable)
2. **Instance**: Blueprint copy in factory (independent, factory-specific)
3. **Instantiation**: Deep copy with UUID regeneration

**Workflows**:

- **Create**: User builds blueprint from scratch → Saved to library
- **Import**: JSON file → Validated → Added to library (not factory)
- **Use**: Template → Deep clone → Regenerate UUIDs → Add to factory
- **Edit**: Template → Modify → Creates NEW template (versioning)
- **Export**: Template → Serialize → JSON file (for sharing)

**Storage**:

- In-memory: `HashMap<Uuid, ProductionLineBlueprint>` in engine
- Persistence: Included in save file as `blueprint_templates` field
- No database required (matches engine philosophy)

**UUID Management**:

- Templates have their own UUIDs
- Instantiation regenerates ALL UUIDs (blueprint + sub-lines)
- Ensures instances are independent from templates
- Factory modifications don't affect templates

**API Endpoints**:

- `GET /api/blueprints/templates` - List all templates
- `POST /api/blueprints/templates` - Create new template
- `PUT /api/blueprints/templates/:id` - Edit (creates new version)
- `DELETE /api/blueprints/templates/:id` - Remove from library
- `POST /api/blueprints/templates/import` - Import JSON to library
- `GET /api/blueprints/templates/:id/export` - Export as JSON
- `POST /api/factories/:id/production-lines/from-template/:tid` - Create instance

**Design Philosophy**:

- Matches Satisfactory in-game blueprint behavior
- Templates are reusable across factories
- Editing doesn't break existing factory setups
- Clear separation: Library (templates) vs Factory (instances)

## Key Design Decisions

### 1. Type Safety Over Flexibility

- Strong Rust typing prevents invalid game data
- Compile-time recipe/item validation via macros
- Enum-based transport system catches errors early

### 2. Trait-Based Extensibility

- `ProductionLine` trait enables blueprints and future custom production types
- `Transport` trait allows new logistics modes without core engine changes

### 3. Shared State Management

- `Arc<Mutex<LogisticsFlux>>` enables safe concurrent access
- Factories hold references to logistics lines for bidirectional updates

### 4. Calculation Model

- **Pull-based**: `update()` triggers factory recalculations
- **Aggregation**: Items summed globally for dashboard view
- **Validation**: Factories validate logistics line endpoints

### 5. Future WASM Compatibility

- Pure Rust logic layer (no I/O, threading, or system dependencies)
- Serde-serializable types for JSON persistence and JS interop
- GUI-agnostic design

## Data Flow

```text
User Action (Vue.js UI)
    ↓ HTTP Request
Backend Server (Axum)
    ↓ Function call
SatisflowEngine API
    ↓
Factory/Logistics Updates
    ↓
calculate_item() per Factory
    ↓
Aggregate Global State
    ↓ JSON Response
Backend Server
    ↓ HTTP Response
Vue.js UI (Update)
```

## Backend Server Components

### Axum Web Server (`crates/satisflow-server/`)

**Purpose**: Production-ready REST API server providing HTTP access to the engine

**Key Components**:

- **State Management** (`state.rs`): Thread-safe engine wrapper using `Arc<RwLock<SatisflowEngine>>`
- **Error Handling** (`error.rs`): Custom error types with proper HTTP status codes
- **API Handlers** (`handlers/`):
  - `factory.rs`: Factory CRUD operations
  - `logistics.rs`: Logistics line management
  - `dashboard.rs`: Global statistics and aggregations
  - `game_data.rs`: Static game data endpoints (recipes, items, machines)

**Features**:

- Environment-based configuration (.env support)
- CORS middleware (configurable for dev/prod)
- Structured logging (tracing + tracing-subscriber)
- Graceful shutdown (SIGINT, SIGTERM)
- Health check endpoint (/health)

**Deployment**:

- Docker support with multi-stage builds
- Non-root container execution
- Docker Compose configuration
- Production-ready with health checks

### REST API Endpoints

**Factory Endpoints**:
- `GET /api/factories` - List all factories
- `GET /api/factories/:id` - Get specific factory
- `POST /api/factories` - Create factory
- `PUT /api/factories/:id` - Update factory
- `DELETE /api/factories/:id` - Delete factory

**Logistics Endpoints**:
- `GET /api/logistics` - List all logistics lines
- `GET /api/logistics/:id` - Get specific line
- `POST /api/logistics` - Create logistics line
- `DELETE /api/logistics/:id` - Delete line

**Dashboard Endpoints**:
- `GET /api/dashboard/summary` - Global statistics
- `GET /api/dashboard/items` - Item balance data
- `GET /api/dashboard/power` - Power statistics

**Game Data Endpoints**:
- `GET /api/game-data/recipes` - All available recipes
- `GET /api/game-data/items` - All game items
- `GET /api/game-data/machines` - All machine types

### Testing Infrastructure

**Integration Tests** (`crates/satisflow-server/tests/`):
- 484 lines of comprehensive test coverage
- Factory CRUD operation tests
- Logistics management tests
- Dashboard endpoint validation
- Game data endpoint tests
- CORS functionality tests
- Error handling tests
- Concurrent request handling tests

**Test Utilities**:
- Helper functions for common operations
- Assertion helpers for response validation
- Test data generators
- Isolated test server instances

## Completed Components

1. ✅ **PowerGenerator** system (5 generator types with fuel consumption)
2. ✅ **Raw Input** system with Resource Well Pressurizer mechanics
3. ✅ **Backend REST API** with complete CRUD operations
4. ✅ **Testing infrastructure** with 484 lines of integration tests
5. ✅ **Deployment setup** with Docker and Docker Compose
6. ✅ **Frontend UI implementation** with Vue 3 + TypeScript + Vite
7. ✅ **Complete component architecture** with 88 files and ~14,100 lines
8. ✅ **Comprehensive factory example** demonstrating all engine features
9. ✅ **Persistence layer** with full save/load functionality (2025-10-25)
10. ✅ **Type synchronization** between frontend/backend (2025-10-24)
11. ✅ **Blueprint import/export** Phase 1 (2025-10-26)
    - Backend: Export, import, preview endpoints with validation
    - Frontend: Import/export UI with BlueprintPreviewModal
    - 14 passing backend tests
    - 3 example blueprint files with documentation

## In Progress Components

1. 🚧 **Blueprint Library System** Phase 2 (2025-10-26)
   - Template storage in engine with save/load integration
   - Blueprint management UI (create, edit, delete, reuse)
   - Template vs instance pattern (like Satisfactory in-game)
   - See: [BLUEPRINT_LIBRARY_IMPLEMENTATION.md](../../../BLUEPRINT_LIBRARY_IMPLEMENTATION.md)

## Missing Components (To Be Implemented)

1. **Enhanced validation layer** for user inputs
2. **Migration system** (architecture designed, full implementation deferred - YAGNI)
3. **Frontend calculation refactoring** - Move all game calculations to backend
   - See: [FRONTEND_CALCULATION_REVIEW.md](../../../FRONTEND_CALCULATION_REVIEW.md)
   - 6+ components have hardcoded power/fuel values
   - Need preview APIs for forms (real-time calculation)

## Best Practices & Lessons Learned

### Type Synchronization (Frontend ↔ Backend)

**Key Principle**: Use native types instead of string representations for enums.

**Implementation**:
- Backend: Serialize enums directly with serde (e.g., `pub item: Item`)
- Frontend: Define matching TypeScript unions (e.g., `type Item = "IronOre" | "CopperOre" | ...`)
- Avoid: Manual string parsing or Debug format (`format!("{:?}", item)`)

**Benefits**:
- Compile-time type safety on both sides
- Eliminates 26+ lines of redundant parsing code
- Automatic serialization/deserialization via serde
- Future enum additions require zero changes to handlers

### Null Handling

**Rust Side**:
```rust
pub description: Option<String>  // Serializes to null or string
```

**TypeScript Side**:
```typescript
description: string | null  // Explicit null handling
```

**Rationale**: TypeScript strict null checking prevents runtime errors.

### Serde Best Practices

1. **Leverage serde defaults**: Enum serialization works correctly out-of-the-box
2. **Trust the type system**: No need for manual string conversions
3. **Version compatibility**: Use `#[serde(default)]` for new optional fields

### Code Quality Wins (2025-10-24 Type Sync)

- ✅ All 200+ Item variants work across entire API
- ✅ Removed redundant manual parsing in logistics handler
- ✅ Consistent serialization in all 4 API modules (factory, logistics, dashboard, game_data)
- ✅ Proper nullable types prevent undefined behavior

## Performance Considerations

- Lazy static initialization of game data (recipes/items)
- HashMap-based lookups for O(1) access
- Calculate-on-demand model (no automatic updates)
- Future: Consider caching aggregated results if performance issues arise

## Testing Strategy

- Unit tests per module (logistics, production_line, etc.)
- Integration tests needed for SatisflowEngine workflows
- TDD approach: Tests written before features (per brief)
