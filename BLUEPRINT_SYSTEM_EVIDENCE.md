# Blueprint Template System - Evidence

## Task
Fix the blueprint template system: CRUD operations + instantiation into factories.

## Changes Made

### 1. Engine Method: `instantiate_blueprint_into_factory`
**File:** `crates/satisflow-engine/src/lib.rs`

Added the missing method that instantiates a blueprint template into a factory as an independent copy:

```rust
pub fn instantiate_blueprint_into_factory(
    &mut self,
    factory_id: FactoryId,
    blueprint_id: ProductionLineId,
    custom_name: Option<String>,
) -> Result<(ProductionLineId, String), Box<dyn std::error::Error>>
```

**Key behaviors:**
- Creates a deep clone of the blueprint template with new UUIDs
- Validates blueprint has at least 1 production line (new validation)
- Assigns new UUIDs to the instance and all production lines within it
- Supports optional custom name override
- Returns `(instance_id, instance_name)` on success

### 2. Validation: Blueprint Must Have At Least 1 Production Line
**File:** `crates/satisflow-engine/src/lib.rs` (engine method)
**File:** `crates/satisflow-server/src/handlers/blueprint_templates.rs` (API handler)

Both the engine method and API handler now validate that blueprints contain at least one production line before they can be added/instantiated.

### 3. TDD Tests for Blueprint Lifecycle
**File:** `crates/satisflow-engine/src/lib.rs` (embedded tests)

Added 7 comprehensive tests:

| Test | Description |
|------|-------------|
| `test_blueprint_template_crud` | Create, read, list, and delete blueprint templates |
| `test_blueprint_template_remove_not_found` | Error handling for removing non-existent template |
| `test_instantiate_blueprint_into_factory` | Basic instantiation creates correct factory structure |
| `test_instantiate_blueprint_twice_creates_independent_copies` | Double instantiation creates separate instances with different IDs |
| `test_instantiate_blueprint_with_custom_name` | Custom name override works |
| `test_instantiate_blueprint_not_found` | Error when blueprint doesn't exist |
| `test_instantiate_blueprint_factory_not_found` | Error when factory doesn't exist |

### 4. Import Fix: ProductionLine in lib.rs
**File:** `crates/satisflow-engine/src/lib.rs`

Added `ProductionLine` to the imports so the `instantiate_blueprint_into_factory` method compiles correctly.

### 5. BlueprintId Type
**Status:** Verified - `BlueprintId = Uuid` exists in `ids.rs` and is exported via `models/mod.rs`. It is not currently used in the codebase (blueprints use `ProductionLineId`), but this is consistent with the architecture since blueprints are stored in a `HashMap<ProductionLineId, ProductionLineBlueprint>`.

## Evidence: All Tests Pass

```
cargo test --workspace
  → 368 passed (12 suites, 0.20s)
```

## Blueprint Import/Export (Already Implemented)
The API handlers already support:
- `POST /api/blueprints/templates` - Create blueprint template
- `GET /api/blueprints/templates` - List all templates
- `GET /api/blueprints/templates/:id` - Get specific template
- `PUT /api/blueprints/templates/:id` - Update template (creates new version)
- `DELETE /api/blueprints/templates/:id` - Delete template
- `POST /api/blueprints/templates/import` - Import from JSON
- `GET /api/blueprints/templates/:id/export` - Export to JSON
- `POST /api/factories/:factory_id/production-lines/from-template/:template_id` - Instantiate into factory

## Architecture Summary

```
Blueprint Template (ProductionLineBlueprint)
    │
    ├── Stored in: SatisflowEngine.blueprint_templates (HashMap)
    ├── Has: Vec<ProductionLineRecipe>
    └── Instantiate → creates independent copy in Factory.production_lines
                           │
                           └── Stored as: ProductionLine::ProductionLineBlueprint
```

The instantiate workflow:
1. User creates blueprint template via API → stored in engine library
2. User calls instantiate endpoint → engine creates deep copy with new UUIDs
3. Copy is added to factory's production_lines HashMap
4. Subsequent instantiations create completely independent copies
