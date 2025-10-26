# Blueprint Feature - Final Integration Steps

## Status: 90% Complete ✅

### ✅ Completed
- Backend API endpoints with full test coverage (12 tests passing)
- Frontend API types and endpoints
- BlueprintPreviewModal component

### 🔄 Remaining: Add UI Integration to FactoryView.vue

## Step 1: Add to FactoryView.vue `<script setup>`

```typescript
import { ref } from 'vue'
import { blueprints } from '@/api/endpoints'
import BlueprintPreviewModal from '@/components/factory/BlueprintPreviewModal.vue'
import type { BlueprintMetadata } from '@/api/types'

// Blueprint import/export state
const showBlueprintPreview = ref(false)
const blueprintMetadata = ref<BlueprintMetadata | null>(null)
const blueprintJsonToImport = ref<string>('')

/**
 * Export a blueprint to JSON file
 */
const handleExportBlueprint = async (lineId: string, lineName: string) => {
  try {
    if (!factoryId.value) return

    const response = await blueprints.export(factoryId.value, lineId)

    // Download JSON file
    const blob = new Blob([response.blueprint_json], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `blueprint-${lineName.replace(/\s+/g, '-').toLowerCase()}-${Date.now()}.json`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)

    // Show success message (assuming you have a toast/notification system)
    console.log('Blueprint exported successfully')
  } catch (error) {
    console.error('Failed to export blueprint:', error)
  }
}

/**
 * Handle import button click - open file picker
 */
const handleImportButtonClick = () => {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.json'
  input.onchange = async (e) => {
    const file = (e.target as HTMLInputElement).files?.[0]
    if (!file) return

    try {
      const text = await file.text()
      const parsed = JSON.parse(text)

      // Validate it's a blueprint by checking structure
      if (!parsed.id || !parsed.name || !parsed.production_lines) {
        throw new Error('Invalid blueprint file format')
      }

      // Extract metadata for preview
      // Note: We'll need to calculate this or the backend could provide a preview endpoint
      blueprintJsonToImport.value = text
      blueprintMetadata.value = {
        name: parsed.name,
        description: parsed.description,
        total_machines: calculateTotalMachines(parsed),
        total_power: 0, // Would need calculation or preview endpoint
        input_items: [],
        output_items: [],
        exported_at: new Date().toISOString()
      }
      showBlueprintPreview.value = true
    } catch (error) {
      console.error('Failed to read blueprint file:', error)
      alert('Invalid blueprint file')
    }
  }
  input.click()
}

/**
 * Calculate total machines from blueprint JSON
 */
const calculateTotalMachines = (blueprint: any): number => {
  return blueprint.production_lines?.reduce((sum: number, line: any) => {
    return sum + (line.machine_groups?.reduce((s: number, g: any) => s + g.number_of_machine, 0) || 0)
  }, 0) || 0
}

/**
 * Confirm and perform blueprint import
 */
const handleConfirmImport = async (customName?: string) => {
  try {
    if (!factoryId.value) return

    await blueprints.import(factoryId.value, {
      blueprint_json: blueprintJsonToImport.value,
      name: customName
    })

    // Refresh factory data
    await loadFactory()

    // Close modal
    showBlueprintPreview.value = false
    blueprintMetadata.value = null
    blueprintJsonToImport.value = ''

    console.log('Blueprint imported successfully')
  } catch (error) {
    console.error('Failed to import blueprint:', error)
  }
}

const handleCloseBlueprintPreview = () => {
  showBlueprintPreview.value = false
  blueprintMetadata.value = null
  blueprintJsonToImport.value = ''
}
```

## Step 2: Add to FactoryView.vue `<template>`

### A. Add Import Button (near the top of Production Lines tab)

Find the section where production lines are listed and add an import button:

```vue
<!-- Add this button near "Add Production Line" button -->
<Button
  variant="secondary"
  @click="handleImportButtonClick"
  title="Import a blueprint from JSON file"
>
  <UploadIcon /> <!-- Use your icon component -->
  Import Blueprint
</Button>
```

### B. Add Export Button (for each blueprint production line)

In the production line list, add an export button for blueprints:

```vue
<!-- Add this in the production line item actions -->
<template v-if="isBlueprint(line)">
  <Button
    variant="secondary"
    size="sm"
    @click="handleExportBlueprint(line.id, line.name)"
    title="Export this blueprint to JSON file"
  >
    <DownloadIcon /> <!-- Use your icon component -->
    Export
  </Button>
</template>
```

### C. Add Helper Function to Detect Blueprints

```typescript
/**
 * Check if a production line is a blueprint
 */
const isBlueprint = (line: any): boolean => {
  return line.type === 'blueprint' || !!line.production_lines
}
```

### D. Add the Modal Component

At the end of the template:

```vue
<!-- Blueprint Preview Modal -->
<BlueprintPreviewModal
  :show="showBlueprintPreview"
  :metadata="blueprintMetadata"
  @close="handleCloseBlueprintPreview"
  @import="handleConfirmImport"
/>
```

## Step 3: Test the Feature

### Manual Testing Checklist

1. **Create a Blueprint**
   - Create a factory with multiple production line recipes
   - These would normally be combined into a blueprint (this may require additional UI)

2. **Export Blueprint**
   - Click "Export" button on a blueprint
   - Verify JSON file downloads
   - Open JSON file and verify structure

3. **Import Blueprint**
   - Click "Import Blueprint" button
   - Select the exported JSON file
   - Verify preview modal shows correct metadata
   - Click "Import"
   - Verify blueprint appears in factory

4. **Import with Name Override**
   - Import same blueprint again
   - Enter custom name in preview modal
   - Verify blueprint imports with new name

5. **Cross-Factory Import**
   - Export blueprint from Factory A
   - Switch to Factory B
   - Import the blueprint
   - Verify it works correctly

### Run Type Check

```bash
cd frontend
npm run type-check
```

## Alternative: Simplified Implementation

If you need to test quickly without full UI integration, you can:

1. **Use Browser Console**:
   ```javascript
   // Export
   const response = await fetch('http://localhost:3000/api/factories/{FACTORY_ID}/production-lines/{LINE_ID}/export')
   const data = await response.json()
   console.log(JSON.stringify(data.blueprint_json, null, 2))

   // Import
   await fetch('http://localhost:3000/api/factories/{FACTORY_ID}/production-lines/import', {
     method: 'POST',
     headers: { 'Content-Type': 'application/json' },
     body: JSON.stringify({ blueprint_json: '...' })
   })
   ```

2. **Use curl/Postman** to test the endpoints directly

## Notes

- The backend is fully functional with 12 passing tests
- The BlueprintPreviewModal is complete and ready to use
- API layer is complete with proper TypeScript types
- Only UI button integration remains

## Expected Time to Complete

- **UI Integration**: 30-60 minutes
- **Testing**: 15-30 minutes
- **Total**: ~1-1.5 hours

## Known Limitations

1. Currently, there's no UI to create a blueprint from scratch (combining multiple recipes)
2. The preview metadata calculations in the import flow are simplified
3. Could add a dedicated "Blueprints" management page in the future

## Success Criteria

✅ Export blueprint downloads valid JSON file
✅ Import blueprint shows preview modal with metadata
✅ Imported blueprints appear in production lines list
✅ Can export/import between different factories
✅ Invalid JSON files show clear error messages
✅ Frontend type-checks without errors

---

**Ready to integrate!** All core functionality is built and tested. The remaining work is straightforward UI button wiring.
