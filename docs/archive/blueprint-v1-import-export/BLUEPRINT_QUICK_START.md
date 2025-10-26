# Blueprint Feature - Quick Start

## TL;DR

Implement blueprint import/export to let users save/share complex production setups as JSON files.

## What to Build

### Backend (3-4h)
```
GET  /api/factories/:id/production-lines/:line_id/export
POST /api/factories/:id/production-lines/import
```

### Frontend (5-7h)
1. Export button → Download blueprint JSON
2. Import button → Upload blueprint JSON
3. Preview modal before import

## Engine Already Has It ✅

```rust
// crates/satisflow-engine/src/models/production_line.rs
pub struct ProductionLineBlueprint {
    pub id: ProductionLineId,
    pub name: String,
    pub description: Option<String>,
    pub production_lines: Vec<ProductionLineRecipe>, // Nested lines
}
```

All methods work: `total_power()`, `input_rate()`, `output_rate()`, serialization, etc.

## Reference Implementation

**Similar feature**: Save/Load system
- Backend: `crates/satisflow-server/src/handlers/save_load.rs`
- Frontend: `frontend/src/components/dashboard/SaveLoadControls.vue`

**Use same pattern**:
1. Serialize ProductionLineBlueprint to JSON string
2. Download/upload with `useFileDownload`/`useFileUpload` composables
3. Add endpoint, types, and UI buttons

## File Locations

```
NEW:    crates/satisflow-server/src/handlers/blueprint.rs
MODIFY: frontend/src/api/endpoints.ts (add blueprints.export/import)
MODIFY: frontend/src/api/types.ts (add BlueprintExportResponse/ImportRequest)
MODIFY: frontend/src/views/FactoryView.vue (add export/import buttons)
NEW:    frontend/src/components/factory/BlueprintPreviewModal.vue
```

## Testing Checklist

- [ ] Export blueprint → downloads JSON
- [ ] Import JSON → creates blueprint in factory
- [ ] Imported blueprint calculates power/items correctly
- [ ] Invalid JSON → shows error message
- [ ] Export/import between different factories works

## Key Details

**UUID Handling**: Generate new UUIDs on import to avoid ID conflicts

**Validation**: Check recipe names, overclock ranges (0-250%), somersloop limits

**File Format**: Standard ProductionLineBlueprint JSON (already serializable)

## Full Details

See `BLUEPRINT_FEATURE_PROMPT.md` for complete implementation guide.

## Current Status

- ✅ Engine fully supports blueprints
- ✅ Save/load includes blueprints
- ❌ No API endpoints to export/import individual blueprints
- ❌ No UI for import/export

---

**Estimated**: 9-13 hours total | **Priority**: P0 Critical
