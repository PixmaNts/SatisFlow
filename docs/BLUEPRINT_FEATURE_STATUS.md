# Blueprint Feature - Current Status

**Last Updated**: 2025-10-26

---

## Quick Summary

✅ **Phase 1 Complete**: Basic Import/Export
🚧 **Phase 2 In Progress**: Blueprint Library System

---

## Phase 1: Import/Export (COMPLETE) ✅

### What's Working

**Backend:**
- ✅ `GET /api/factories/:id/production-lines/:line_id/export` - Export blueprint to JSON
- ✅ `POST /api/factories/:id/production-lines/import` - Import blueprint from JSON
- ✅ `POST /api/blueprints/preview` - Preview blueprint metadata (NEW!)
  - Calculates accurate power consumption using engine
  - Extracts input/output items
  - Returns total machines count
- ✅ Validation and UUID regeneration
- ✅ 14 passing unit tests

**Frontend:**
- ✅ Import button opens file picker
- ✅ BlueprintPreviewModal shows accurate metadata from engine
- ✅ Export button on blueprint production lines (conditional rendering)
- ✅ Name override functionality
- ✅ Error handling with user-friendly messages
- ✅ Factory refresh after import

**Documentation:**
- ✅ 3 example blueprint JSON files with proper structure
- ✅ EXAMPLE_BLUEPRINTS_README.md with usage guide
- ✅ Archived Phase 1 implementation docs

### Files Created/Modified

**Backend:**
- `crates/satisflow-server/src/handlers/blueprint.rs` - Export, import, preview endpoints
- Tests: 14 comprehensive tests

**Frontend:**
- `frontend/src/components/factory/BlueprintPreviewModal.vue` - Preview UI (NEW)
- `frontend/src/components/factory/ProductionLineList.vue` - Import/export buttons
- `frontend/src/api/endpoints.ts` - Blueprint API client

**Documentation:**
- `example-blueprint-iron-plates.json`
- `example-blueprint-reinforced-plates.json`
- `example-blueprint-motor-production.json`
- `EXAMPLE_BLUEPRINTS_README.md`

### User Workflows (Working)

1. **Export Blueprint**:
   - Navigate to Factory → Production Lines
   - Find blueprint in list
   - Click "Export" button (only visible on blueprints)
   - JSON file downloads: `blueprint-{name}-{timestamp}.json`

2. **Import Blueprint**:
   - Navigate to Factory → Production Lines
   - Click "Import Blueprint" button
   - Select JSON file
   - Preview modal shows:
     - Name, description
     - Total machines, power (accurate from engine!)
     - Input/output items
   - Optional: Override name
   - Click "Import"
   - Blueprint added to factory
   - Factory refreshes automatically

---

## Phase 2: Blueprint Library System (NEXT)

### Architecture Overview

**Template vs Instance Pattern:**
- **Template**: Blueprint stored in library (reusable, editable)
- **Instance**: Blueprint copy in factory (independent, factory-specific)
- **Instantiation**: Deep copy with UUID regeneration

**New UI Section: "Blueprints"**
- Separate navigation item (after Logistics)
- Library view showing all saved templates
- Create/Edit/Delete/Export templates
- Import blueprints to library (not factory)

**Factory Integration:**
- "Add Production Line" becomes dropdown
- Options: "Recipe" or "From Blueprint Template"
- Select template → Creates instance in factory

### What Needs to Be Built

#### Backend (~13 hours)

**Engine Changes:**
```rust
pub struct SatisflowEngine {
    factories: HashMap<Uuid, Factory>,
    logistics: Vec<Logistics>,
    blueprint_templates: HashMap<Uuid, ProductionLineBlueprint>,  // NEW
}
```

**New API Endpoints:**
- `GET /api/blueprints/templates` - List all templates
- `POST /api/blueprints/templates` - Create new template
- `GET /api/blueprints/templates/:id` - Get template by ID
- `PUT /api/blueprints/templates/:id` - Edit (creates new version)
- `DELETE /api/blueprints/templates/:id` - Delete template
- `POST /api/blueprints/templates/import` - Import to library
- `GET /api/blueprints/templates/:id/export` - Export template
- `POST /api/factories/:id/production-lines/from-template/:tid` - Create instance

**Tasks:**
1. Add `blueprint_templates` to engine (2h)
2. Update save/load serialization (1h)
3. Create template CRUD handlers (4h)
4. Create from-template handler (2h)
5. Write comprehensive tests (3h)
6. Update routes (30min)

#### Frontend (~29 hours)

**New Components:**
- `BlueprintLibraryView.vue` - Main library view (3h)
- `BlueprintCard.vue` - Template card display (2h)
- `BlueprintFormModal.vue` - Create/edit blueprint from scratch (6h)
  - Recipe selector
  - Machine groups editor
  - Preview with backend calculation
- `MachineGroupsEditor.vue` - Reusable machine group editor (2h)
- `RecipeSelector.vue` - Recipe dropdown (1.5h)
- `BlueprintSelectorModal.vue` - Select template for factory (2h)

**Modified Components:**
- `ProductionLineList.vue` - Change button to dropdown (1h)
- `Navigation.vue` - Add Blueprints nav link (30min)
- Update import flow to library (1h)

**API Layer:**
- `blueprintTemplates` client functions (1h)
- TypeScript types (30min)

**Tasks:**
1. API types & client (1.5h)
2. Blueprint Library view (6h)
3. Blueprint creation modal (9.5h)
4. Factory integration (3h)
5. Import/export updates (1.25h)
6. Testing & polish (7h)

### User Workflows (Phase 2)

1. **Create Blueprint Template**:
   - Navigate to Blueprints section
   - Click "Create New Blueprint"
   - Enter name & description
   - Add production lines (multiple recipes)
   - Configure machine groups for each line
   - Preview shows total machines & power
   - Save to library

2. **Use Template in Factory**:
   - Navigate to Factory → Production Lines
   - Click "Add Production Line" dropdown
   - Select "From Blueprint Template"
   - Choose template from list
   - Optional: Override name
   - Instance created with new UUIDs

3. **Edit Template (Creates New Version)**:
   - Navigate to Blueprints section
   - Click "Edit" on template
   - Make changes
   - Save → Creates NEW template (old unchanged)

4. **Import to Library**:
   - Navigate to Blueprints section
   - Click "Import from File"
   - Select JSON
   - Preview → Import to library (NOT factory)
   - Use "From Blueprint Template" to add to factory

---

## Key Decisions Made

### Q1: Where should imported blueprints go?
**A:** Always to library. User then adds to factory via template selector.
**Reason:** Clean separation, encourages reuse.

### Q2: Should templates be editable?
**A:** Yes, but editing creates a new version (new UUID).
**Reason:** Prevents breaking existing factory instances. Matches Satisfactory behavior.

### Q3: How to create blueprints?
**A:** Create in Blueprint Library section from scratch (not from existing lines).
**Reason:** Reduces complexity, clear workflow.

### Q4: Should we store templates?
**A:** Yes, in engine memory with save/load persistence.
**Reason:** No database needed, matches engine architecture.

---

## Implementation Plan

### Sprint 1: Foundation (Backend + Basic API)
- Add `blueprint_templates` to engine
- Implement basic CRUD API
- Add frontend types/client
- **Goal:** Backend complete, API working

### Sprint 2: Library View (Display & Simple Actions)
- Create Blueprint Library view
- Display templates as cards
- Import to library working
- Export from library working
- **Goal:** Can view/manage templates

### Sprint 3: Creation & Usage (Complex Forms)
- Blueprint creation modal (from scratch)
- Factory integration (dropdown + selector)
- **Goal:** Can create blueprints, use in factories

### Sprint 4: Polish & Testing
- Comprehensive backend tests
- End-to-end testing
- UI/UX polish
- Documentation
- **Goal:** Production-ready

---

## Related Documents

- **[BLUEPRINT_LIBRARY_IMPLEMENTATION.md](../BLUEPRINT_LIBRARY_IMPLEMENTATION.md)** - Full implementation plan (22 tasks, ~42 hours)
- **[EXAMPLE_BLUEPRINTS_README.md](../EXAMPLE_BLUEPRINTS_README.md)** - Example blueprint files and usage
- **[FRONTEND_CALCULATION_REVIEW.md](../FRONTEND_CALCULATION_REVIEW.md)** - Frontend refactoring needs
- **[docs/archive/blueprint-v1-import-export/](archive/blueprint-v1-import-export/)** - Phase 1 archived docs

---

## Memory Bank Updates

**Architecture:**
- Added Blueprint Library System section to `architecture.md`
- Documented template vs instance pattern
- Listed all new API endpoints

**Implementation Status:**
- Updated Phase 1 as complete
- Added Phase 2 as in progress
- Updated P0 critical gaps section
- Reflected 2025-10-26 date

---

## Next Steps

**Immediate:**
1. Review and approve implementation plan
2. Begin Sprint 1 (Backend foundation)
3. Add `blueprint_templates` to engine
4. Update save/load serialization

**After Phase 2 Complete:**
- Address frontend calculation violations (see FRONTEND_CALCULATION_REVIEW.md)
- Move all game calculations to backend
- Add preview APIs for forms

---

**Status**: ✅ Phase 1 Complete | 🚧 Phase 2 Ready to Implement
**Estimated Phase 2 Time**: ~42 hours (4 sprints)
**Documentation**: Complete and up-to-date
