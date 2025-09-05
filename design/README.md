# SatisFlow - Design Documentation

## Project Overview
SatisFlow is a Vue + TypeScript web application powered by a Rust engine compiled to WebAssembly for tracking and optimizing Satisfactory factory production chains.

## Core Architecture

### Tech Stack
- **Frontend**: Vue 3 + TypeScript (Vite)
- **Engine**: Rust compiled to WebAssembly via `wasm-bindgen`
- **Styling**: Tailwind CSS
- **Storage**: Browser localStorage (JSON)
- **Data**: Static Rust code generation from build.rs

### Module Structure
```
SatisFlow/
├── crates/
│   ├── satisflow-engine/    # Core calculations (Rust)
│   ├── satisflow-wasm/      # WASM bindings
│   └── satisflow-server/    # Optional SaaS backend
├── frontend/                # Vue.js frontend
└── build-web.sh            # Build script
```

## Core Features

### Factory Management
- Create factories with descriptive names
- Configure raw inputs (miners, extractors)
- Set up production lines with recipes, machine counts, clock speeds
- Support for Strange Matter boosters

### Production Tracking
- Real-time overview of all items produced/consumed
- Status indicators: ✅ Balanced, ⚠️ Overflow, ❌ Underflow
- Aggregate calculations across multiple factories

### Logistics Planning
- Track material flows between factories
- Support for conveyors, trains, trucks, drones
- ID conventions: `LG-BUS-001-C03`, `LG-TRN-01-W02`

## Data Model

### Core Types
- `ProductionTracker`: Root state containing all factories and logistics
- `Factory`: Contains production lines and raw inputs
- `ProductionLine`: Recipe + machine count + clock ratio + boosters
- `LogisticsFlux`: Material transfer between factories
- `Recipe`: Game data (inputs/outputs/machine type)
- `Item`: Game data (name/category)

### ID Conventions
- Factory IDs: `fac_<slug>_<n>` (e.g., `fac_northern_forest_1`)
- Production Line IDs: `line_<factory>_<recipe>_<n>`
- Logistics IDs: `LG-<TYPE>-<nnn>[-<detail>]`
  - Bus: `LG-BUS-001-C03` (Bus 001, Conveyor 03)
  - Train: `LG-TRN-01-W02` (Train 01, Wagon 02)
  - Truck: `LG-TRK-05` (Truck route 05)
  - Drone: `LG-DRN-12` (Drone route 12)

## Performance Strategy

### WebAssembly Integration
- All calculations performed in Rust/WASM for near-native speed
- Minimal data marshaling between JS and WASM
- Efficient serialization with serde

### State Management
- Single `ProductionTracker` instance as source of truth
- Derived computations cached and memoized
- Incremental updates to minimize re-rendering

## Development Workflow

### Quick Start
1. Build WASM: `cd crates/satisflow-wasm && wasm-pack build --target web --out-dir ../../frontend/src/wasm`
2. Start frontend: `cd frontend && npm install && npm run dev`
3. Or use: `./build-web.sh` for full build

### Game Data
- Items and recipes are compiled into static Rust code via `build.rs`
- No external JSON files needed at runtime
- Data is immutable and optimized for performance

### Testing
- Rust unit tests for all calculation logic
- Frontend integration tests with WASM
- Sample factory data for testing (`init.rs`)

## Validation Rules

### Production Lines
- Machine counts: 1..=999
- Clock ratio: 0.1..=2.5
- Strange Matter boosters: ≤ machine_count
- Recipes must reference valid items

### Logistics
- Origin and destination factories must exist
- Items must be valid and producible
- Quantities must be positive
- Transport types must match detail format

## Phase 1 Deliverables (Current)

- ✅ Factory creation and management
- ✅ Production line configuration  
- ✅ Logistics flux tracking
- ✅ Production overview calculations
- ✅ Sample data initialization
- ✅ WASM + Vue integration
- ✅ Export/import functionality

## Future Enhancements

### Planned Features
- Visual factory layout editor
- Power consumption tracking
- Recipe alternative comparisons
- Production optimization suggestions
- Cloud sync (SaaS model)
- Mobile-responsive design

### Technical Debt
- Simplify JSON data loading (use only generated static data)
- Consolidate duplicate build artifacts
- Add comprehensive error handling
- Implement localStorage autosave
- Add production line grouping UI

## Architecture Decisions

### Why WebAssembly?
- Complex calculations with 1000+ production lines
- Deterministic, testable business logic
- Share code between potential desktop/mobile versions
- Performance requirements for real-time updates

### Why Vue over React?
- Simpler learning curve
- Excellent TypeScript support
- Mature ecosystem with Vite
- Good WASM integration patterns

### Why Static Data Generation?
- No runtime JSON parsing overhead
- Compile-time validation of game data
- Smaller bundle size (no duplicate data files)
- Type safety for all game items/recipes