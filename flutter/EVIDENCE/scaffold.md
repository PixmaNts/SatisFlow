# Flutter Project Scaffold Evidence

Created: 2026-04-07
Branch: main

## Verification Commands

```bash
# Flutter analyze
cd flutter && flutter analyze
# Expected: No issues found!

# Cargo build (workspace)
cargo build --workspace
# Expected: 0 errors
```

## Project Structure

```
flutter/
├── lib/
│   ├── main.dart                          # App entry point
│   └── src/rust/
│       ├── api.dart                       # FFI API exports
│       ├── frb_generated.dart             # Generated bindings (91.2K)
│       ├── frb_generated.io.dart          # IO bindings (40.4K)
│       └── frb_generated.web.dart         # Web bindings (31.1K)
├── rust/                                  # FRB Rust crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── api/
│           └── mod.rs                     # FFI functions with #[frb] annotations
├── pubspec.yaml                           # Dependencies configured
├── flutter_rust_bridge.yaml               # FRB configuration
└── analysis_options.yaml
```

## Dependencies (pubspec.yaml)

### Production
- flutter_rust_bridge: 2.12.0
- flutter_riverpod: 2.6.1
- riverpod_annotation: 2.6.1
- go_router: 14.8.1
- dynamic_color: 1.8.1
- shared_preferences: 2.5.5

### Dev
- build_runner: 2.5.4
- riverpod_generator: 2.6.5

## Feature-First Folder Structure

```
lib/
├── core/
│   ├── theme/         # AppTheme configuration
│   ├── router/        # GoRouter configuration
│   ├── errors/        # AppException types
│   ├── providers/      # Core providers
│   └── utils/          # Core utilities
├── shared/
│   ├── widgets/        # Reusable widgets
│   └── extensions/     # Dart extensions
└── features/
    ├── engine/
    │   └── providers/  # Engine/Rust bridge providers
    ├── game_data/
    │   └── providers/ # Game data providers
    ├── factory/        # Factory feature
    ├── logistics/       # Logistics feature
    ├── blueprints/      # Blueprints feature
    └── dashboard/       # Dashboard feature
```

## Generated Bindings

The FRB codegen generates bindings for all `#[frb]` annotated functions:

- `ffi_create_factory`
- `ffi_get_factory`
- `ffi_get_all_factories`
- `ffi_delete_factory`
- `ffi_create_logistics_line`
- `ffi_update_logistics_line`
- `ffi_get_logistics_line`
- `ffi_get_all_logistics`
- `ffi_delete_logistics_line`
- `ffi_reset`
- `ffi_global_power_stats`
- `ffi_update`
- `ffi_add_blueprint_template`
- `ffi_get_blueprint_template`
- `ffi_get_all_blueprint_templates`
- `ffi_remove_blueprint_template`
- `ffi_update_blueprint_template`
- `ffi_instantiate_blueprint_into_factory`
- `ffi_save_to_json`
- `ffi_load_from_json`

## Workspace Cargo.toml

The workspace now includes:

```toml
members = [
    "crates/satisflow-engine",
    "crates/satisflow-server",
    "crates/satisflow-bridge",
]
```