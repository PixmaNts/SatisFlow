# Satisflow Architecture

## System Overview

Satisflow is built with a **layered architecture** separating concerns between the core engine (Rust) and future UI implementations (Vue.js/WASM).

```
┌─────────────────────────────────────────┐
│         UI Layer (Future)               │
│    Vue.js + TypeScript + WASM           │
└─────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│      Satisflow Engine (Rust)            │
│  - Core Business Logic                  │
│  - Data Models                          │
│  - Calculations                         │
│  - Persistence (JSON)                   │
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
    fn total_sommersloop(&self) -> u32;
    fn output_rate(&self) -> Vec<(Item, f32)>;
    fn input_rate(&self) -> Vec<(Item, f32)>;
    fn total_power_consumption(&self) -> f32;
}
```

**Implementations**:

1. **ProductionLineRecipe**: Standard recipe-based production
   - Contains: `id, name, description, recipe, machine_groups: Vec<MachineGroup>`
   - MachineGroup: `(num_machines: u32, overclock: f32, sommersloop: u8)`
   - Validates: Overclock (0-250%), Sommersloop limits per machine type
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

- Sommersloop limits: Constructor(1), Assembler(2), Manufacturer(4), etc.
- Base power consumption: Constructor(4MW), Assembler(16MW), etc.

**Recipes**: Auto-generated from `recipes_data.inc`

- Uses macro `define_recipes!` for compile-time recipe database
- Lazy-loaded HashMaps for O(1) lookup by name or Recipe enum
- Each recipe defines: name, machine, inputs, outputs

**Items**: Similar pattern to recipes

- Lazy-loaded item database
- ItemParseError for validation

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

```
User Action (Future UI)
    ↓
SatisflowEngine API
    ↓
Factory/Logistics Updates
    ↓
calculate_item() per Factory
    ↓
Aggregate Global State
    ↓
Return to UI (JSON via WASM)
```

## Missing Components (To Be Implemented)

1. **RawInput** tracking (mentioned in brief, not yet in models)
2. **PowerGenerator** as distinct from ProductionLine
3. **Persistence layer** (JSON serialization infrastructure)
4. **Blueprint custom recipes** (ProductionLineBlueprint exists but needs UI integration)
5. **Validation layer** for user inputs (partially done in add_machine_group)

## Performance Considerations

- Lazy static initialization of game data (recipes/items)
- HashMap-based lookups for O(1) access
- Calculate-on-demand model (no automatic updates)
- Future: Consider caching aggregated results if performance issues arise

## Testing Strategy

- Unit tests per module (logistics, production_line, etc.)
- Integration tests needed for SatisflowEngine workflows
- TDD approach: Tests written before features (per brief)
