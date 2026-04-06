# SATISFLOW ENGINE

Core domain logic for Satisfactory factory simulation. Pure Rust library — no I/O dependencies beyond `std::fs` for save/load.

## STRUCTURE

```
src/
├── lib.rs              # SatisflowEngine + SaveFile (entry point)
├── version.rs          # SaveVersion parsing & compatibility checks
├── models/
│   ├── mod.rs          # Barrel re-exports
│   ├── ids.rs          # UUID type aliases (FactoryId, LogisticsId, etc.)
│   ├── items.rs        # Item enum (200+ variants, data in items_data.inc)
│   ├── recipes.rs      # Recipe enum (480+ variants, data in recipes_data.inc)
│   ├── factory.rs      # Factory aggregate root (production lines, items, power)
│   ├── production_line.rs  # ProductionLineRecipe + ProductionLineBlueprint
│   ├── raw_input.rs    # RawInput, ExtractorType, Purity, resource nodes
│   ├── power_generator.rs  # PowerGenerator, GeneratorType, fuel/waste calcs
│   ├── logistics.rs    # Transport types (Bus/Train/Truck/Drone), conveyors, pipelines
│   └── game_data.rs    # Static game data lookups
├── examples/           # Demo programs (5-factory network example)
└── bin/                # Binary executables (factory_demo, save_load_demo)
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add a game item | `models/items.rs` + `models/items_data.inc` | Also update `frontend/src/api/types.ts` |
| Add a recipe | `models/recipes.rs` + `models/recipes_data.inc` | Also update frontend types |
| Add a transport type | `models/logistics.rs` | Implement `Transport` trait |
| Change power calculation | `models/power_generator.rs` | NEVER duplicate in frontend |
| Change extraction logic | `models/raw_input.rs` | Purity multipliers, extractor rates |
| Add save migration | `version.rs` + `lib.rs` | See `crates/MIGRATION-STRATEGY.md` |
| Add blueprint feature | `models/production_line.rs` | ProductionLineBlueprint |

## KEY TYPES

- **`SatisflowEngine`** — Main entry. Holds `HashMap<FactoryId, Factory>`, logistics, blueprint templates. All mutations go through methods here.
- **`SaveFile`** — Wrapper with version, timestamps, game version. Serialize/deserialize with serde.
- **`Factory`** — Aggregate root. Owns production lines, raw inputs, power generators. Calculates item balances via `calculate_item()`.
- **`ProductionLine`** — Tagged union: `ProductionLineRecipe` (single recipe) or `ProductionLineBlueprint` (nested group).
- **`LogisticsFlux`** — Directed edge between factories. Holds `TransportType` enum.
- **`TransportType`** — `Bus(Bus)`, `Train(Train)`, `Truck(TruckTransport)`, `Drone(DroneTransport)`.

## CONVENTIONS

- **`.inc` files** contain raw data arrays included via `include_str!()` — do not edit the `.rs` file's data section
- **ID types** are UUID aliases from `ids.rs` — always use the type alias, never raw `Uuid`
- **Error handling** — domain errors use `thiserror` derives; engine methods return `Result<_, Box<dyn std::error::Error>>`
- **Serde** — all public types derive `Serialize`/`Deserialize`; `#[serde(default)]` on new fields for backward compat

## ANTI-PATTERNS

- **NEVER** remove a struct field — add migration in `version.rs` instead
- **NEVER** skip a version number — increment sequentially
- **NEVER** use `.unwrap()` outside `#[cfg(test)]` — use `?` or `.ok_or()`
- **NEVER** hardcode game constants in other crates — define here, expose via API

## COMPLEXITY HOTSPOTS

| File | Lines | Why large |
|------|-------|-----------|
| `models/raw_input.rs` | 1643 | Extractor types, purity calcs, resource well nodes |
| `models/power_generator.rs` | 1133 | Fuel consumption, overclocking, waste production |
| `lib.rs` | 1057 | Engine methods + embedded unit tests |
| `models/factory.rs` | 932 | Item balance calculations, production aggregation |
| `models/logistics.rs` | 768 | 4 transport variants with sub-types |
