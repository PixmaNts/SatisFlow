# Blueprint Import/Export Feature - Implementation Guide

## Context

Satisflow is a production tracker for Satisfactory players. We need to implement the **Blueprint Import/Export** feature to allow users to export/import complex production line configurations as reusable blueprints.

### What are Blueprints?

In Satisflow, a **ProductionLineBlueprint** is a special type of production line that contains multiple nested ProductionLineRecipe entries. Think of it as a "recipe of recipes" - a custom, reusable production configuration that groups multiple production steps together.

**Example Use Case:**
A player creates a complex "Reinforced Plate Production" setup that:
- Takes 120 iron ingots/min as input
- Uses 5 Assemblers for Iron Plates
- Uses 2 Constructors for Screws
- Uses 3 Assemblers for Reinforced Plates
- Outputs 10 Reinforced Plates/min

This entire configuration can be saved as a blueprint and:
1. Exported to a JSON file for backup or sharing with other players
2. Imported into other factories
3. Reused multiple times within the same factory

## Current Implementation Status

### ✅ Already Implemented (Engine Layer)

The engine already fully supports blueprints:

**File**: `crates/satisflow-engine/src/models/production_line.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductionLine {
    ProductionLineRecipe(ProductionLineRecipe),      // Standard recipe
    ProductionLineBlueprint(ProductionLineBlueprint), // Blueprint with nested lines
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionLineBlueprint {
    pub id: ProductionLineId,
    pub name: String,
    pub description: Option<String>,
    pub production_lines: Vec<ProductionLineRecipe>, // Nested production lines
}

impl ProductionLineBlueprint {
    pub fn new(id: ProductionLineId, name: String, description: Option<String>) -> Self
    pub fn add_production_line(&mut self, line: ProductionLineRecipe)
    pub fn total_machines(&self) -> u32
    pub fn total_somersloop(&self) -> u32
    pub fn output_rate(&self) -> Vec<(Item, f32)>
    pub fn input_rate(&self) -> Vec<(Item, f32)>
    pub fn total_power_consumption(&self) -> f32
}
```

**Key Features Already Working:**
- ✅ Blueprint serialization/deserialization (serde)
- ✅ Nested production line storage
- ✅ Power consumption calculation
- ✅ Input/output rate aggregation
- ✅ Machine and Somersloop counting
- ✅ Blueprints included in save/load system

### ❌ Missing Implementation

**Backend API** (~3-4 hours):
1. No endpoint to export a single blueprint as JSON
2. No endpoint to import a blueprint JSON into a factory
3. No dedicated blueprint management endpoints

**Frontend UI** (~4-6 hours):
1. No blueprint creation/editing UI
2. No blueprint import button with file picker
3. No blueprint export button
4. No nested production line editor
5. Placeholder banner in Production Line tab saying "Blueprints coming soon"

## Implementation Plan

### Phase 1: Backend API Endpoints (3-4 hours)

#### 1.1 Create Blueprint Handler

**New File**: `crates/satisflow-server/src/handlers/blueprint.rs`

Implement these endpoints:

```rust
// Export a blueprint as JSON
GET /api/factories/:factory_id/production-lines/:line_id/export
Response: BlueprintExportResponse {
    blueprint_json: String,  // Serialized ProductionLineBlueprint
    metadata: BlueprintMetadata {
        name: String,
        description: Option<String>,
        total_machines: u32,
        total_power: f32,
        input_items: Vec<(Item, f32)>,
        output_items: Vec<(Item, f32)>,
        exported_at: DateTime<Utc>,
    }
}

// Import a blueprint into a factory
POST /api/factories/:factory_id/production-lines/import
Request: BlueprintImportRequest {
    blueprint_json: String,  // JSON string to deserialize
    name: Option<String>,    // Override name if provided
}
Response: FactoryResponse  // Updated factory with new blueprint

// List all blueprints in a factory (optional, for future use)
GET /api/factories/:factory_id/blueprints
Response: Vec<BlueprintSummary>
```

#### 1.2 Implementation Steps

1. Create `handlers/blueprint.rs`
2. Define request/response types
3. Implement export handler:
   - Find factory by ID
   - Find production line by ID
   - Verify it's a blueprint (not a recipe)
   - Serialize to JSON
   - Return with metadata
4. Implement import handler:
   - Deserialize blueprint JSON
   - Validate structure
   - Generate new UUID for the blueprint
   - Add to factory's production_lines
   - Return updated factory
5. Add routes to `main.rs`
6. Write integration tests

#### 1.3 Error Handling

Handle these cases:
- Factory not found (404)
- Production line not found (404)
- Production line is not a blueprint (400 Bad Request)
- Invalid JSON format (400 Bad Request)
- Blueprint validation errors (400 Bad Request)

### Phase 2: Frontend API Layer (1 hour)

#### 2.1 Update API Types

**File**: `frontend/src/api/types.ts`

```typescript
// Blueprint export response
export interface BlueprintExportResponse {
  blueprint_json: string;
  metadata: BlueprintMetadata;
}

export interface BlueprintMetadata {
  name: string;
  description?: string;
  total_machines: number;
  total_power: number;
  input_items: Array<[string, number]>;
  output_items: Array<[string, number]>;
  exported_at: string;
}

// Blueprint import request
export interface BlueprintImportRequest {
  blueprint_json: string;
  name?: string;
}
```

#### 2.2 Update API Endpoints

**File**: `frontend/src/api/endpoints.ts`

```typescript
export const blueprints = {
  /**
   * Export a production line blueprint as JSON
   */
  export: async (
    factoryId: string,
    lineId: string
  ): Promise<BlueprintExportResponse> => {
    return api.get<BlueprintExportResponse>(
      `/factories/${factoryId}/production-lines/${lineId}/export`
    );
  },

  /**
   * Import a blueprint JSON into a factory
   */
  import: async (
    factoryId: string,
    blueprint: BlueprintImportRequest
  ): Promise<FactoryResponse> => {
    return api.post<FactoryResponse>(
      `/factories/${factoryId}/production-lines/import`,
      blueprint
    );
  },
};
```

### Phase 3: Frontend UI Components (4-6 hours)

#### 3.1 Blueprint Export Button

**Location**: `frontend/src/views/FactoryView.vue` (Production Line tab)

Add export button next to each blueprint production line:

```vue
<Button
  v-if="isBlueprint(line)"
  variant="secondary"
  size="sm"
  @click="handleExportBlueprint(line.id)"
>
  <DownloadIcon />
  Export Blueprint
</Button>
```

**Handler Logic:**
1. Call `blueprints.export(factoryId, lineId)`
2. Parse `blueprint_json` to get pretty-printed JSON
3. Download as `.json` file using `useFileDownload` composable
4. Filename format: `blueprint-{name}-{timestamp}.json`

#### 3.2 Blueprint Import Button

**Location**: `frontend/src/views/FactoryView.vue` (Production Line tab header)

Add import button next to "Add Production Line":

```vue
<Button
  variant="secondary"
  size="sm"
  @click="handleImportBlueprint"
>
  <UploadIcon />
  Import Blueprint
</Button>
```

**Handler Logic:**
1. Use `useFileUpload` composable to select `.json` file
2. Read and parse JSON
3. Show preview modal with blueprint metadata
4. User confirms → Call `blueprints.import(factoryId, { blueprint_json, name })`
5. Refresh factory data
6. Show success message

#### 3.3 Blueprint Preview Modal

**Component**: `frontend/src/components/factory/BlueprintPreviewModal.vue`

Show before importing:
- Blueprint name and description
- Total machines and power consumption
- Input/output items list
- Nested production lines summary
- Name override input field
- Cancel and Confirm buttons

#### 3.4 Blueprint Badge/Indicator

Add visual indicator to distinguish blueprints from regular recipes:

```vue
<Badge v-if="isBlueprint(line)" variant="info">
  Blueprint ({{ line.production_lines.length }} lines)
</Badge>
```

### Phase 4: Testing & Validation (1-2 hours)

#### 4.1 Backend Tests

Add to `handlers/blueprint.rs`:

```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_export_blueprint()

    #[tokio::test]
    async fn test_export_non_blueprint_fails()

    #[tokio::test]
    async fn test_import_valid_blueprint()

    #[tokio::test]
    async fn test_import_invalid_json_fails()
}
```

#### 4.2 Manual Testing Checklist

- [ ] Create a blueprint with nested production lines
- [ ] Export blueprint downloads a valid JSON file
- [ ] Exported JSON contains all nested production lines
- [ ] Import blueprint from file succeeds
- [ ] Imported blueprint shows in production lines list
- [ ] Imported blueprint calculates power/items correctly
- [ ] Export/import between different factories works
- [ ] Error handling shows user-friendly messages

## Technical Considerations

### 1. UUID Handling

When importing a blueprint:
- Generate a **new UUID** for the blueprint itself
- Generate **new UUIDs** for all nested production lines
- This prevents ID conflicts when importing same blueprint multiple times

### 2. Validation

Validate imported blueprints:
- JSON structure matches `ProductionLineBlueprint`
- All nested recipes reference valid `Recipe` enum values
- Machine groups have valid overclock values (0-250%)
- Somersloop counts don't exceed machine type limits

### 3. File Format

Blueprint JSON format (example):

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Reinforced Plate Production",
  "description": "120 iron ingots → 10 reinforced plates",
  "production_lines": [
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "name": "Iron Plate Sub-Line",
      "description": null,
      "recipe": "IronPlate",
      "machine_groups": [
        {
          "number_of_machine": 5,
          "oc_value": 100.0,
          "somersloop": 0
        }
      ]
    },
    // More nested lines...
  ]
}
```

### 4. User Experience

**Export Flow:**
1. User clicks "Export Blueprint" on a blueprint
2. JSON file downloads immediately
3. Success toast: "Blueprint exported successfully"

**Import Flow:**
1. User clicks "Import Blueprint"
2. File picker opens
3. User selects `.json` file
4. Preview modal shows blueprint details
5. User optionally renames the blueprint
6. User clicks "Import"
7. Blueprint added to factory
8. Success toast: "Blueprint imported successfully"
9. Production lines list updates

## Existing Code to Reference

### Similar Features Already Implemented

**Save/Load System** (good reference for file operations):
- `crates/satisflow-server/src/handlers/save_load.rs` - Similar JSON serialization pattern
- `frontend/src/components/dashboard/SaveLoadControls.vue` - File download/upload UI

**Production Line Management**:
- `frontend/src/views/FactoryView.vue` - Production Line tab structure
- `frontend/src/components/factory/ProductionLineList.vue` - List display
- `frontend/src/components/factory/ProductionLineForm.vue` - Form handling

### Composables Available

- `useFileDownload` - For exporting blueprints to JSON file
- `useFileUpload` - For importing blueprints from JSON file
- `useErrorHandler` - For consistent error handling
- `useFactoryStore` - For factory state management

## File Structure Overview

```
crates/satisflow-server/src/handlers/
  └── blueprint.rs                    [NEW] Blueprint endpoints

frontend/src/
  ├── api/
  │   ├── endpoints.ts                [MODIFY] Add blueprint endpoints
  │   └── types.ts                    [MODIFY] Add blueprint types
  ├── components/factory/
  │   └── BlueprintPreviewModal.vue   [NEW] Import preview
  └── views/
      └── FactoryView.vue             [MODIFY] Add import/export buttons
```

## Acceptance Criteria

### Backend
- ✅ Export endpoint returns valid blueprint JSON
- ✅ Import endpoint creates blueprint from valid JSON
- ✅ Error handling for invalid data
- ✅ Integration tests pass
- ✅ Backend compiles without warnings

### Frontend
- ✅ Export button downloads `.json` file with blueprint data
- ✅ Import button opens file picker and accepts `.json` files
- ✅ Preview modal shows blueprint metadata before import
- ✅ Imported blueprints appear in production lines list
- ✅ Blueprint indicator/badge shows in UI
- ✅ Loading states during export/import
- ✅ Success/error messages display correctly
- ✅ Frontend type-checks without errors

### End-to-End
- ✅ Export blueprint from Factory A
- ✅ Import same blueprint into Factory B
- ✅ Blueprint functions correctly (power calc, item rates)
- ✅ Can export and re-import same blueprint multiple times
- ✅ Invalid JSON files show clear error messages

## Estimated Timeline

- **Backend API**: 3-4 hours
- **Frontend API Layer**: 1 hour
- **Frontend UI**: 4-6 hours
- **Testing & Polish**: 1-2 hours
- **Total**: 9-13 hours

## Priority

**P0 - Critical Gap**: This feature is currently blocking users from:
- Reusing complex production setups across factories
- Sharing production configurations with other players
- Backing up specific production line configurations
- Creating template blueprints for common production chains

## Related Documents

- Memory Bank: `.kilocode/rules/memory-bank/brief.md` (lines 264-302)
- Implementation Status: `.kilocode/rules/memory-bank/implementation-status.md` (lines 748-762)
- Engine Model: `crates/satisflow-engine/src/models/production_line.rs`

## Next Steps

1. Read this entire prompt carefully
2. Review the existing production_line.rs model
3. Start with backend implementation (handlers/blueprint.rs)
4. Add backend tests
5. Implement frontend API layer
6. Build UI components (export → import → preview)
7. Test end-to-end workflow
8. Commit with descriptive message

---

**Note**: The engine layer is already complete and tested. Focus on exposing the existing functionality through REST API endpoints and building the UI for export/import operations.
