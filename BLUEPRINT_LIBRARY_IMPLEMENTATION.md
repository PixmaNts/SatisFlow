# Blueprint Library Feature - Implementation Plan

## Status: 🟡 In Progress

**Phase 1 Complete**: ✅ Basic Import/Export
**Phase 2 Current**: 🚧 Blueprint Library System

---

## Overview

This document outlines the implementation of the **Blueprint Library** feature, which allows users to create, store, and reuse production line blueprints across factories. This follows the Satisfactory in-game blueprint workflow.

### What's Already Complete (Phase 1)

✅ **Backend:**
- Export endpoint (`GET /api/factories/:id/production-lines/:line_id/export`)
- Import endpoint (`POST /api/factories/:id/production-lines/import`)
- Preview endpoint (`POST /api/blueprints/preview`)
- Validation and UUID regeneration
- 14 passing unit tests

✅ **Frontend:**
- Import button with file picker
- Export button on blueprint production lines
- Preview modal with accurate metadata from engine
- Name override functionality
- 3 example blueprint JSON files

✅ **Documentation:**
- Example blueprints with README
- Frontend calculation review
- Architecture analysis

### What We're Building (Phase 2)

🎯 **Blueprint Library System** - A central place to:
- Create blueprints from scratch
- Store blueprint templates
- Reuse templates across factories
- Import/export for sharing
- Edit templates (creates new versions)

---

## Architecture Design

### Core Principle: Template vs Instance Pattern

```
Blueprint Template (Library)          Blueprint Instance (Factory)
┌─────────────────────────┐          ┌─────────────────────────┐
│ ID: template-uuid-1     │          │ ID: new-uuid-generated  │
│ Name: "Motor Complex"   │  ─────>  │ Name: "Motor Complex"   │
│ Production Lines: [...]  │ instanti │ Production Lines: [...]  │
│                         │  -ate    │ (deep copy with new IDs)│
│ Stored in: Library      │          │ Stored in: Factory A    │
└─────────────────────────┘          └─────────────────────────┘
       ↓                                       ↓
   Reusable                              Independent copy
   Editable                              Factory-specific
```

### Engine Structure

```rust
pub struct SatisflowEngine {
    // Existing
    factories: HashMap<Uuid, Factory>,
    logistics: Vec<Logistics>,

    // NEW: Blueprint template storage
    blueprint_templates: HashMap<Uuid, ProductionLineBlueprint>,
}
```

### Save File Structure

```json
{
  "factories": [/* existing */],
  "logistics": [/* existing */],
  "blueprint_templates": [  // NEW
    {
      "id": "uuid-1",
      "name": "Motor Production Complex",
      "description": "Complete motor setup with all prerequisites",
      "production_lines": [
        {
          "id": "uuid-line-1",
          "name": "Iron Ingot Production",
          "recipe": "IronIngot",
          "machine_groups": [/* ... */]
        }
        // ... more lines
      ]
    }
  ]
}
```

---

## User Workflows

### Workflow 1: Create New Blueprint Template

```
User Journey:
1. Navigate to "Blueprints" section in main nav
2. Click "Create New Blueprint" button
3. Modal opens:
   - Enter name: "Motor Production Complex"
   - Enter description: "Complete motor setup..."
   - Click "Add Production Line" (multiple times)
     - For each line:
       - Select recipe (e.g., "IronIngot", "Motor")
       - Configure machine groups (quantity, overclock, somersloops)
   - Preview shows: total machines, total power
4. Click "Save Blueprint"
5. Blueprint added to library
6. See success message: "Blueprint saved to library"
```

**Backend Flow:**
```
POST /api/blueprints/templates
├─> Validate blueprint structure
├─> Generate UUIDs for blueprint + all production lines
├─> Calculate metadata (power, machines)
├─> Add to engine.blueprint_templates
└─> Return BlueprintTemplateResponse
```

### Workflow 2: Use Blueprint in Factory

```
User Journey:
1. Navigate to Factory → Production Lines tab
2. Click "Add Production Line" dropdown
3. Select "From Blueprint Template"
4. Modal shows list of available templates:
   ┌────────────────────────────────────┐
   │ Motor Production Complex           │
   │ 45 machines | 187 MW | 9 lines    │
   │ [Select]                           │
   ├────────────────────────────────────┤
   │ Reinforced Plates Setup            │
   │ 17 machines | 104 MW | 4 lines    │
   │ [Select]                           │
   └────────────────────────────────────┘
5. Click template to preview full details
6. Optional: Override name
7. Click "Add to Factory"
8. Instance created with NEW UUIDs
9. Factory refreshes, new blueprint appears in list
```

**Backend Flow:**
```
POST /api/factories/:factory_id/production-lines/from-template/:template_id
├─> Get template from engine.blueprint_templates
├─> Deep clone blueprint
├─> Regenerate ALL UUIDs (blueprint + all sub-lines)
├─> Apply name override if provided
├─> Add to factory.production_lines
└─> Return updated FactoryResponse
```

### Workflow 3: Import Blueprint to Library

```
User Journey:
1. Navigate to "Blueprints" section
2. Click "Import from File" button
3. File picker opens
4. Select blueprint JSON file (e.g., "motor-complex-shared.json")
5. Preview modal shows:
   - Name, description
   - Total machines, power
   - Input/output items (from engine calculation)
   - Option to override name
6. Click "Import to Library"
7. Blueprint added to library (NOT to any factory)
8. Success message: "Blueprint imported to library"
9. Can now use Workflow 2 to add to factories
```

**Backend Flow:**
```
POST /api/blueprints/templates/import
├─> Parse JSON
├─> Validate structure
├─> Regenerate UUIDs
├─> Calculate metadata
├─> Add to engine.blueprint_templates
└─> Return new template ID
```

### Workflow 4: Export Blueprint from Library

```
User Journey:
1. Navigate to "Blueprints" section
2. Find template in grid view
3. Click "Export" button
4. JSON file downloads: "blueprint-motor-complex-1730000000.json"
5. Can share file with other players
```

**Backend Flow:**
```
GET /api/blueprints/templates/:id/export
├─> Get template from engine.blueprint_templates
├─> Serialize to JSON
├─> Calculate metadata
└─> Return BlueprintExportResponse
```

### Workflow 5: Edit Blueprint (Creates New Version)

```
User Journey:
1. Navigate to "Blueprints" section
2. Click "Edit" on existing template
3. Editor opens (same as create modal)
4. Modify production lines:
   - Add new lines
   - Remove existing lines
   - Change machine configurations
5. Click "Save as New Blueprint"
6. NEW template created with new UUID
7. Original template unchanged
8. Existing factory instances unaffected
```

**Important:** This creates a NEW template, not an update. This matches Satisfactory's behavior where editing a blueprint creates a new blueprint.

### Workflow 6: Delete Blueprint from Library

```
User Journey:
1. Navigate to "Blueprints" section
2. Click "Delete" on template
3. Confirmation dialog:
   "Delete 'Motor Complex'? Factory instances will not be affected."
4. Click "Confirm Delete"
5. Template removed from library
6. Factory instances remain (they're independent copies)
```

---

## API Specification

### Blueprint Template Endpoints

#### `GET /api/blueprints/templates`
**Purpose:** Get all blueprint templates in library

**Response:**
```json
[
  {
    "id": "uuid-1",
    "name": "Motor Production Complex",
    "description": "Complete motor setup",
    "total_machines": 45,
    "total_power": 187.5,
    "production_lines": [/* ... */],
    "input_items": [["IronOre", 450.0], ["CopperOre", 300.0]],
    "output_items": [["Motor", 7.5]],
    "created_at": "2025-10-26T12:00:00Z"
  }
]
```

#### `POST /api/blueprints/templates`
**Purpose:** Create new blueprint template

**Request:**
```json
{
  "name": "Motor Production Complex",
  "description": "Complete motor setup",
  "production_lines": [
    {
      "name": "Iron Ingot Production",
      "description": null,
      "recipe": "IronIngot",
      "machine_groups": [
        {
          "number_of_machine": 8,
          "oc_value": 100.0,
          "somersloop": 0
        }
      ]
    }
    // ... more lines
  ]
}
```

**Response:** Same as GET (single template)

#### `GET /api/blueprints/templates/:id`
**Purpose:** Get specific template by ID

**Response:** Same as GET (single template)

#### `PUT /api/blueprints/templates/:id`
**Purpose:** Update template (creates NEW version)

**Request:** Same as POST

**Response:** NEW template with new UUID

**Note:** This doesn't actually update the existing template - it creates a new one. The old template can be manually deleted if desired.

#### `DELETE /api/blueprints/templates/:id`
**Purpose:** Delete template from library

**Response:** `204 No Content`

#### `POST /api/blueprints/templates/import`
**Purpose:** Import blueprint JSON to library

**Request:**
```json
{
  "blueprint_json": "{\"id\":\"...\", \"name\":\"...\", ...}",
  "name": "Optional Name Override"
}
```

**Response:** Same as POST (created template)

#### `GET /api/blueprints/templates/:id/export`
**Purpose:** Export template as JSON file

**Response:**
```json
{
  "blueprint_json": "{...}",
  "metadata": {
    "name": "Motor Production Complex",
    "total_machines": 45,
    "total_power": 187.5,
    // ... etc
  }
}
```

#### `POST /api/factories/:factory_id/production-lines/from-template/:template_id`
**Purpose:** Create instance from template in factory

**Request:**
```json
{
  "name": "Optional Name Override"
}
```

**Response:** Complete FactoryResponse with new instance

---

## Implementation Tasks

### Phase 2A: Backend - Engine & Storage (Priority 1)

#### Task 1: Add blueprint_templates to SatisflowEngine
**File:** `crates/satisflow-engine/src/lib.rs`

- [ ] Add `blueprint_templates: HashMap<Uuid, ProductionLineBlueprint>` field
- [ ] Implement `add_blueprint_template()`
- [ ] Implement `get_blueprint_template()`
- [ ] Implement `get_all_blueprint_templates()`
- [ ] Implement `remove_blueprint_template()`
- [ ] Add unit tests for template storage

**Estimated Time:** 2 hours

#### Task 2: Update Save/Load Serialization
**File:** `crates/satisflow-engine/src/lib.rs`

- [ ] Add `blueprint_templates` to `SaveData` struct
- [ ] Update `save()` method to include templates
- [ ] Update `load()` method to restore templates
- [ ] Add migration test (old saves without templates should work)
- [ ] Test roundtrip save/load with templates

**Estimated Time:** 1 hour

#### Task 3: Update Save/Load API Endpoints
**File:** `crates/satisfflow-server/src/handlers/save_load.rs`

- [ ] Verify save endpoint includes templates
- [ ] Verify load endpoint restores templates
- [ ] Test with existing save files (backward compatibility)
- [ ] Update API documentation

**Estimated Time:** 30 minutes

---

### Phase 2B: Backend - Blueprint Template API (Priority 1)

#### Task 4: Create Blueprint Template Handlers
**File:** `crates/satisflow-server/src/handlers/blueprint_templates.rs` (NEW)

- [ ] Implement `get_all_templates()`
- [ ] Implement `get_template(id)`
- [ ] Implement `create_template()`
- [ ] Implement `update_template()` (creates new version)
- [ ] Implement `delete_template()`
- [ ] Implement `import_to_library()`
- [ ] Implement `export_template()`
- [ ] Add request/response types
- [ ] Add validation functions
- [ ] Add error handling

**Estimated Time:** 4 hours

#### Task 5: Create "From Template" Handler
**File:** `crates/satisflow-server/src/handlers/factory.rs`

- [ ] Implement `create_from_template()` endpoint
- [ ] Deep clone template
- [ ] Regenerate all UUIDs
- [ ] Apply name override
- [ ] Add to factory
- [ ] Return updated factory
- [ ] Add unit tests

**Estimated Time:** 2 hours

#### Task 6: Update Route Configuration
**File:** `crates/satisflow-server/src/handlers/mod.rs`

- [ ] Add blueprint_templates module
- [ ] Register template routes
- [ ] Update main router

**Estimated Time:** 30 minutes

#### Task 7: Write Comprehensive Backend Tests
**File:** `crates/satisflow-server/src/handlers/blueprint_templates.rs`

- [ ] Test create template
- [ ] Test get all templates
- [ ] Test get single template
- [ ] Test delete template
- [ ] Test import to library
- [ ] Test export from library
- [ ] Test create instance from template (UUID regeneration)
- [ ] Test update creates new version
- [ ] Test with invalid data
- [ ] Test edge cases

**Estimated Time:** 3 hours

---

### Phase 2C: Frontend - API & Types (Priority 2)

#### Task 8: Add Blueprint Template Types
**File:** `frontend/src/api/types.ts`

- [ ] Add `BlueprintTemplateResponse` interface
- [ ] Add `CreateBlueprintTemplateRequest` interface
- [ ] Add `UpdateBlueprintTemplateRequest` interface
- [ ] Add `CreateFromTemplateRequest` interface
- [ ] Export all new types

**Estimated Time:** 30 minutes

#### Task 9: Create Blueprint Templates API Client
**File:** `frontend/src/api/endpoints.ts`

- [ ] Add `blueprintTemplates.getAll()`
- [ ] Add `blueprintTemplates.getById()`
- [ ] Add `blueprintTemplates.create()`
- [ ] Add `blueprintTemplates.update()`
- [ ] Add `blueprintTemplates.delete()`
- [ ] Add `blueprintTemplates.importToLibrary()`
- [ ] Add `blueprintTemplates.export()`
- [ ] Add `blueprintTemplates.createInstanceInFactory()`
- [ ] Add JSDoc comments

**Estimated Time:** 1 hour

---

### Phase 2D: Frontend - Blueprint Library View (Priority 2)

#### Task 10: Create Blueprint Library View
**File:** `frontend/src/views/BlueprintLibraryView.vue`

- [ ] Create main layout (header + grid)
- [ ] Add "Create New" button
- [ ] Add "Import from File" button
- [ ] Display blueprint cards in grid
- [ ] Add empty state
- [ ] Add loading states
- [ ] Add error handling
- [ ] Fetch templates on mount
- [ ] Handle create/import/edit actions
- [ ] Add responsive styling

**Estimated Time:** 3 hours

#### Task 11: Create Blueprint Card Component
**File:** `frontend/src/components/blueprints/BlueprintCard.vue`

- [ ] Display template name, description
- [ ] Show stats (machines, power, lines count)
- [ ] Add Edit button
- [ ] Add Export button
- [ ] Add Delete button (with confirmation)
- [ ] Add hover effects
- [ ] Style as card layout
- [ ] Add icons for stats

**Estimated Time:** 2 hours

#### Task 12: Add Blueprint Library Route
**File:** `frontend/src/router/index.ts`

- [ ] Add `/blueprints` route
- [ ] Configure component
- [ ] Add meta (title)
- [ ] Update navigation

**File:** `frontend/src/components/layout/Navigation.vue`

- [ ] Add "Blueprints" nav link
- [ ] Add icon
- [ ] Position after Logistics

**Estimated Time:** 30 minutes

---

### Phase 2E: Frontend - Blueprint Creation (Priority 3)

#### Task 13: Create Blueprint Form Modal
**File:** `frontend/src/components/blueprints/BlueprintFormModal.vue`

This is the most complex component - allows creating blueprints from scratch.

- [ ] Create modal wrapper
- [ ] Add name/description inputs
- [ ] Create production lines array editor
- [ ] For each production line:
  - [ ] Add recipe selector (dropdown from game data)
  - [ ] Add name/description inputs
  - [ ] Add machine groups editor (reusable component)
- [ ] Add "Add Production Line" button
- [ ] Add "Remove Line" button for each line
- [ ] Calculate preview (total machines, power)
  - [ ] Call backend preview API on change (debounced)
- [ ] Add save button
- [ ] Add cancel button
- [ ] Add validation
- [ ] Add error messages
- [ ] Style form layout
- [ ] Handle loading states

**Estimated Time:** 6 hours

#### Task 14: Create Machine Groups Editor Component
**File:** `frontend/src/components/blueprints/MachineGroupsEditor.vue`

Reusable component for editing machine groups within a production line.

- [ ] Display list of machine groups
- [ ] For each group:
  - [ ] Number of machines input
  - [ ] Overclock slider/input
  - [ ] Somersloop input
  - [ ] Remove button
- [ ] Add "Add Machine Group" button
- [ ] Validate inputs
- [ ] Emit updates to parent

**Estimated Time:** 2 hours

#### Task 15: Create Recipe Selector Component
**File:** `frontend/src/components/blueprints/RecipeSelector.vue`

- [ ] Fetch recipes from game data API
- [ ] Display searchable dropdown
- [ ] Group by category (optional)
- [ ] Show recipe icon (if available)
- [ ] Emit selection to parent

**Estimated Time:** 1.5 hours

---

### Phase 2F: Frontend - Factory Integration (Priority 3)

#### Task 16: Update Production Line List
**File:** `frontend/src/components/factory/ProductionLineList.vue`

- [ ] Change "Add Production Line" to dropdown
- [ ] Add "Recipe" option (existing flow)
- [ ] Add "From Blueprint Template" option (new flow)
- [ ] Handle dropdown selection
- [ ] Open appropriate modal based on selection

**Estimated Time:** 1 hour

#### Task 17: Create Blueprint Selector Modal
**File:** `frontend/src/components/factory/BlueprintSelectorModal.vue`

- [ ] Fetch available templates
- [ ] Display as selectable list
- [ ] Show template details on hover/click
- [ ] Add preview section
- [ ] Add name override input (optional)
- [ ] Add "Add to Factory" button
- [ ] Call create-from-template API
- [ ] Refresh factory data on success
- [ ] Show success message
- [ ] Handle errors

**Estimated Time:** 2 hours

---

### Phase 2G: Frontend - Import/Export Updates (Priority 4)

#### Task 18: Update Import Flow
**File:** `frontend/src/components/factory/ProductionLineList.vue`

Current: Import button in factory → imports to factory
New: Move to Blueprint Library → imports to library

- [ ] Remove import button from ProductionLineList
- [ ] Move import logic to BlueprintLibraryView
- [ ] Update import to call library endpoint (not factory)
- [ ] Update success message

**Estimated Time:** 1 hour

#### Task 19: Keep Export in Factory
**File:** `frontend/src/components/factory/ProductionLineList.vue`

- [ ] Keep export button (already works)
- [ ] Maybe add tooltip: "Export for sharing"
- [ ] No changes needed (already complete)

**Estimated Time:** 15 minutes

---

### Phase 2H: Testing & Polish (Priority 5)

#### Task 20: End-to-End Testing

- [ ] Test create blueprint from scratch
- [ ] Test import blueprint to library
- [ ] Test export blueprint from library
- [ ] Test use blueprint in factory
- [ ] Test edit blueprint (creates new version)
- [ ] Test delete blueprint from library
- [ ] Test save/load includes templates
- [ ] Test cross-factory blueprint usage
- [ ] Test error cases (invalid data, network errors)
- [ ] Test edge cases (empty blueprints, large blueprints)

**Estimated Time:** 3 hours

#### Task 21: UI/UX Polish

- [ ] Add loading spinners
- [ ] Add success/error toasts
- [ ] Add confirmation dialogs for destructive actions
- [ ] Improve empty states with helpful text
- [ ] Add keyboard shortcuts (ESC to close modals)
- [ ] Test responsive layouts (mobile, tablet)
- [ ] Add animations/transitions
- [ ] Improve accessibility (ARIA labels, focus management)

**Estimated Time:** 2 hours

#### Task 22: Documentation

- [ ] Update EXAMPLE_BLUEPRINTS_README.md
- [ ] Create user guide for Blueprint Library
- [ ] Document API endpoints in OpenAPI spec
- [ ] Add inline code comments
- [ ] Create video/GIF demos of workflows
- [ ] Update main README with new features

**Estimated Time:** 2 hours

---

## Total Estimated Time

| Phase | Tasks | Estimated Time |
|-------|-------|----------------|
| 2A: Backend Engine | 3 tasks | 3.5 hours |
| 2B: Backend API | 4 tasks | 10 hours |
| 2C: Frontend API | 2 tasks | 1.5 hours |
| 2D: Frontend Library View | 3 tasks | 6 hours |
| 2E: Frontend Creation | 3 tasks | 9.5 hours |
| 2F: Frontend Factory | 2 tasks | 3 hours |
| 2G: Frontend Import/Export | 2 tasks | 1.25 hours |
| 2H: Testing & Polish | 3 tasks | 7 hours |
| **TOTAL** | **22 tasks** | **~42 hours** |

**Note:** This is a rough estimate. Some tasks may be faster/slower depending on complexity and debugging needs.

---

## Implementation Order (Recommended)

### Sprint 1: Foundation (Backend + Basic API)
1. ✅ Phase 2A: Engine storage (Tasks 1-3)
2. ✅ Phase 2B: Basic CRUD API (Tasks 4-6)
3. ✅ Phase 2C: Frontend types/client (Tasks 8-9)

**Goal:** Backend complete, API working, types defined

### Sprint 2: Library View (Display & Simple Actions)
4. ✅ Phase 2D: Library view (Tasks 10-12)
5. ✅ Phase 2G: Update import/export (Tasks 18-19)

**Goal:** Can view templates, import to library, export from library

### Sprint 3: Creation & Usage (Complex Forms)
6. ✅ Phase 2E: Blueprint creation (Tasks 13-15)
7. ✅ Phase 2F: Factory integration (Tasks 16-17)

**Goal:** Can create blueprints from scratch, use in factories

### Sprint 4: Polish & Testing
8. ✅ Phase 2B: Comprehensive tests (Task 7)
9. ✅ Phase 2H: Testing & polish (Tasks 20-22)

**Goal:** Production-ready, polished, documented

---

## Success Criteria

### Backend
- ✅ All blueprint templates stored in engine
- ✅ Save/load includes templates
- ✅ All API endpoints working
- ✅ UUID regeneration on instantiation
- ✅ 100% test coverage for template handlers

### Frontend
- ✅ Blueprint Library view functional
- ✅ Can create blueprints from scratch
- ✅ Can import/export blueprints
- ✅ Can use templates in factories
- ✅ Edit creates new versions
- ✅ Responsive design works
- ✅ No TypeScript errors
- ✅ All workflows tested

### User Experience
- ✅ Clear separation between templates and instances
- ✅ Intuitive workflows (no confusion)
- ✅ Helpful error messages
- ✅ Fast (no lag when creating/using blueprints)
- ✅ Works offline (after initial data load)

---

## Future Enhancements (Post-Phase 2)

These are ideas for future iterations, NOT part of current implementation:

- **Tags/Categories**: Organize blueprints by category (Power, Production, Mining, etc.)
- **Search & Filter**: Search templates by name, filter by category/tags
- **Favorites**: Mark frequently-used templates as favorites
- **Version History**: Keep track of blueprint versions (v1, v2, v3)
- **Sharing Platform**: Online repository to share blueprints with community
- **Thumbnail Preview**: Visual representation of blueprint layout
- **Dependency Analysis**: Show what items blueprint needs as input
- **Cost Calculator**: Calculate total building cost (materials, power slugs)
- **Comparison Tool**: Compare two blueprints side-by-side
- **Bulk Operations**: Export/import multiple blueprints at once

---

## Questions & Decisions Log

### Q1: Should editing create a new version or update in-place?
**Decision:** Create new version (new UUID). Original unchanged, factory instances unaffected.
**Reason:** Matches Satisfactory in-game behavior, prevents accidental breaking of existing setups.

### Q2: Where should imported blueprints go?
**Decision:** Always to library (not directly to factory).
**Reason:** Cleaner separation, encourages reuse, matches expected workflow.

### Q3: Should we keep import button in factory view?
**Decision:** No, move to Blueprint Library.
**Reason:** Consistent location for all template management.

### Q4: Should templates be editable?
**Decision:** Yes, but creates new version.
**Reason:** Allows iteration while preserving originals.

### Q5: Do we need version history tracking?
**Decision:** Not in Phase 2. Future enhancement.
**Reason:** Adds complexity, can add later if users request it.

---

## Related Documents

- [EXAMPLE_BLUEPRINTS_README.md](EXAMPLE_BLUEPRINTS_README.md) - Example blueprint files
- [FRONTEND_CALCULATION_REVIEW.md](FRONTEND_CALCULATION_REVIEW.md) - Frontend refactoring needs
- [docs/archive/blueprint-v1-import-export/](docs/archive/blueprint-v1-import-export/) - Phase 1 documentation

---

## Contact & Support

For questions about this implementation plan, refer to:
- Architecture decisions in memory bank
- Backend tests for expected behavior
- Frontend components for UI patterns

---

**Last Updated:** 2025-10-26
**Status:** Ready for Implementation
**Next Step:** Begin Sprint 1 - Backend Foundation
