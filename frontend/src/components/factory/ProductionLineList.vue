<template>
  <div class="production-line-list">
    <DataTable
      :columns="columns"
      :data="productionLines"
      :loading="loading"
      :empty-text="emptyText"
      @row-click="(row: Record<string, unknown>) => handleRowClick(row as ProductionLineResponse)"
    >
      <template #cell-name="{ row }">
        <div class="production-line-name">
          <span class="name">{{ getProductionLineName(row as ProductionLineResponse) }}</span>
          <span v-if="isProductionLineRecipe(row as ProductionLineResponse) && (row as any).ProductionLineRecipe?.description" class="description">
            {{ (row as any).ProductionLineRecipe.description }}
          </span>
        </div>
      </template>

      <template #cell-recipe="{ row }">
        <span v-if="isProductionLineRecipe(row as ProductionLineResponse)" class="recipe-name">
          {{ (row as any).ProductionLineRecipe.recipe }}
        </span>
        <span v-else class="recipe-name">
          Blueprint ({{ (row as any).ProductionLineBlueprint?.production_lines?.length || 0 }} recipes)
        </span>
      </template>

      <template #cell-machines="{ row }">
        <div class="machine-info">
          <span class="machine-count">{{ getTotalMachines(row as ProductionLineResponse) }}</span>
          <span class="machine-details">{{ getMachineDetails(row as ProductionLineResponse) }}</span>
        </div>
      </template>

      <template #cell-power="{ row }">
        <div class="power-info">
          <span class="power-value">{{ formatPower((row as ProductionLineResponse).total_power_consumption) }}</span>
          <span v-if="getTotalSomersloops(row as ProductionLineResponse) > 0" class="somersloop-count">
            {{ getTotalSomersloops(row as ProductionLineResponse) }}×
          </span>
        </div>
      </template>

      <template #cell-consumed="{ row }">
        <div class="consumed-items">
          <div
            v-for="input in (row as ProductionLineResponse).input_rate"
            :key="input.item"
            class="consumed-item"
          >
            <ItemDisplay
              :item="input.item"
              :show-name="false"
              size="sm"
            />
            <span class="consumed-quantity">{{ formatQuantity(input.quantity) }}</span>
          </div>
          <span v-if="(row as ProductionLineResponse).input_rate.length === 0" class="no-input">
            No input
          </span>
        </div>
      </template>

      <template #cell-produced="{ row }">
        <div class="produced-items">
          <div
            v-for="output in (row as ProductionLineResponse).output_rate"
            :key="output.item"
            class="produced-item"
          >
            <ItemDisplay
              :item="output.item"
              :show-name="false"
              size="sm"
            />
            <span class="produced-quantity">{{ formatQuantity(output.quantity) }}</span>
          </div>
          <span v-if="(row as ProductionLineResponse).output_rate.length === 0" class="no-output">
            No output
          </span>
        </div>
      </template>

      <template #cell-actions="{ row }">
        <div class="action-buttons">
          <!-- Export button for blueprints only -->
          <Button
            v-if="isProductionLineBlueprint(row as ProductionLineResponse)"
            variant="secondary"
            size="sm"
            @click.stop="handleExportBlueprint(getProductionLineId(row as ProductionLineResponse) || '', getProductionLineName(row as ProductionLineResponse))"
            title="Export this blueprint to JSON file"
          >
            <span class="button-icon">💾</span>
            Export
          </Button>
          <Button
            variant="secondary"
            size="sm"
            @click.stop="handleEdit(row as ProductionLineResponse)"
          >
            Edit
          </Button>
          <Button
            variant="danger"
            size="sm"
            @click.stop="handleDelete(row as ProductionLineResponse)"
          >
            Delete
          </Button>
        </div>
      </template>
    </DataTable>

    <!-- Create/Edit Production Line Modal -->
    <ProductionLineForm
      v-model:show="showCreateModal"
      :factory-id="factoryId"
      :production-line="editingLine"
      @saved="handleSaved"
    />

    <!-- Delete Confirmation Modal -->
    <Modal
      v-model:show="showDeleteModal"
      title="Delete Production Line"
      @close="showDeleteModal = false"
    >
      <div class="delete-confirmation">
        <p>Are you sure you want to delete this production line?</p>
        <p v-if="deletingLine" class="line-name">
          <strong>{{ getProductionLineName(deletingLine) }}</strong>
        </p>
        <p class="warning-text">This action cannot be undone.</p>
      </div>

      <template #footer>
        <div class="modal-actions">
          <Button
            variant="secondary"
            @click="showDeleteModal = false"
          >
            Cancel
          </Button>
          <Button
            variant="danger"
            :loading="deleting"
            @click="confirmDelete"
          >
            Delete
          </Button>
        </div>
      </template>
    </Modal>

    <!-- Blueprint Preview Modal -->
    <BlueprintPreviewModal
      :show="showBlueprintPreview"
      :metadata="blueprintMetadata"
      @close="handleCloseBlueprintPreview"
      @import="handleConfirmImport"
    />

    <!-- Blueprint Selector Modal -->
    <BlueprintSelectorModal
      :show="showBlueprintSelector"
      :factory-id="factoryId"
      @close="showBlueprintSelector = false"
      @select="handleBlueprintSelected"
    />

    <!-- Error Alert -->
    <Alert
      v-if="error"
      type="error"
      @close="clearError"
    >
      {{ error }}
    </Alert>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useFactoryStore } from '@/stores/factory'
import type { ProductionLineResponse, MachineGroup, BlueprintMetadata } from '@/api/types'
import { blueprints, blueprintTemplates } from '@/api/endpoints'
import { useToast } from '@/composables/useToast'
import { useErrorHandler } from '@/composables/useErrorHandler'
import Button from '@/components/ui/Button.vue'
import DataTable from '@/components/ui/DataTable.vue'
import Modal from '@/components/ui/Modal.vue'
import Alert from '@/components/ui/Alert.vue'
import ItemDisplay from '@/components/ui/ItemDisplay.vue'
import ProductionLineForm from './ProductionLineForm.vue'
import BlueprintPreviewModal from './BlueprintPreviewModal.vue'
import BlueprintSelectorModal from './BlueprintSelectorModal.vue'

interface Props {
  factoryId: string
}

const props = defineProps<Props>()
const factoryStore = useFactoryStore()

// Composables
const { showSuccess, showError } = useToast()
const { handleError } = useErrorHandler()

// State
const showCreateModal = ref(false)
const showDeleteModal = ref(false)
const editingLine = ref<ProductionLineResponse | null>(null)
const deletingLine = ref<ProductionLineResponse | null>(null)
const deleting = ref(false)
const error = ref<string | null>(null)
const showDropdown = ref(false)
const showBlueprintSelector = ref(false)

// Blueprint import/export state
const showBlueprintPreview = ref(false)
const blueprintMetadata = ref<BlueprintMetadata | null>(null)
const blueprintJsonToImport = ref<string>('')

// Computed
const currentFactory = computed(() => factoryStore.currentFactory)
const productionLines = computed(() => currentFactory.value?.production_lines || [])
const loading = computed(() => factoryStore.loading)

const emptyText = computed(() => {
  if (!props.factoryId) {
    return 'Please select a factory first'
  }
  return 'No production lines found. Create your first production line to get started.'
})

// DataTable columns
const columns = [
  {
    key: 'name',
    label: 'Name',
    sortable: true,
    width: '15%'
  },
  {
    key: 'recipe',
    label: 'Recipe',
    sortable: true,
    width: '15%'
  },
  {
    key: 'machines',
    label: 'Machines',
    sortable: false,
    width: '12%'
  },
  {
    key: 'consumed',
    label: 'Consumed',
    sortable: false,
    width: '18%'
  },
  {
    key: 'produced',
    label: 'Produced',
    sortable: false,
    width: '18%'
  },
  {
    key: 'power',
    label: 'Power',
    sortable: true,
    width: '10%'
  },
  {
    key: 'actions',
    label: 'Actions',
    sortable: false,
    width: '12%'
  }
]

// Helper type guard functions
const isProductionLineRecipe = (line: ProductionLineResponse): line is ProductionLineResponse & { ProductionLineRecipe: { id: string; name: string; description: string; recipe: string; machine_groups: MachineGroup[] } } => {
  return 'ProductionLineRecipe' in line
}

const isProductionLineBlueprint = (line: ProductionLineResponse): line is ProductionLineResponse & { ProductionLineBlueprint: { id: string; name: string; description: string; production_lines: ProductionLineResponse[] } } => {
  return 'ProductionLineBlueprint' in line
}

// Methods
const getProductionLineId = (line: ProductionLineResponse): string | null => {
  if (isProductionLineRecipe(line)) {
    return line.ProductionLineRecipe.id
  }
  if (isProductionLineBlueprint(line)) {
    return line.ProductionLineBlueprint.id
  }
  return null
}

const getProductionLineName = (line: ProductionLineResponse): string => {
  if (isProductionLineRecipe(line)) {
    return line.ProductionLineRecipe.name
  } else if (isProductionLineBlueprint(line)) {
    return line.ProductionLineBlueprint.name
  }
  return 'Unknown'
}

const getTotalMachines = (line: ProductionLineResponse): number => {
  // Use the backend-calculated total machines
  return line.total_machines
}

const getMachineDetails = (line: ProductionLineResponse): string => {
  if (isProductionLineRecipe(line)) {
    const groups = line.ProductionLineRecipe.machine_groups
    if (groups.length === 0) return 'No machines'
    if (groups.length === 1) {
      const group = groups[0]
      if (group) {
        return `${group.number_of_machine}× ${group.oc_value}%`
      }
    }
    return `${groups.length} groups`
  } else if (isProductionLineBlueprint(line)) {
    return 'Blueprint'
  }
  return 'Unknown'
}

// Removed getPowerConsumption wrapper - use line.total_power_consumption directly

const getTotalSomersloops = (line: ProductionLineResponse): number => {
  if (isProductionLineRecipe(line)) {
    return line.ProductionLineRecipe.machine_groups.reduce(
      (total: number, group: MachineGroup) => total + (group.number_of_machine * group.somersloop), 0
    )
  } else if (isProductionLineBlueprint(line)) {
    return line.ProductionLineBlueprint.production_lines.reduce(
      (total, subLine) => total + getTotalSomersloops(subLine), 0
    )
  }
  return 0
}

const formatPower = (power: number): string => {
  if (power < 1) {
    return `${(power * 1000).toFixed(0)} kW`
  }
  return `${power.toFixed(1)} MW`
}

const formatQuantity = (quantity: number): string => {
  // Format quantity per minute
  if (quantity >= 1000) {
    return `${(quantity / 1000).toFixed(1)}k/min`
  }
  return `${quantity.toFixed(1)}/min`
}

const handleRowClick = (row: ProductionLineResponse) => {
  handleEdit(row)
}

const handleEdit = (line: ProductionLineResponse) => {
  editingLine.value = line
  showCreateModal.value = true
}

const handleDelete = (line: ProductionLineResponse) => {
  deletingLine.value = line
  showDeleteModal.value = true
}

const confirmDelete = async () => {
  if (!deletingLine.value) return

  deleting.value = true
  error.value = null

  try {
    const lineId = getProductionLineId(deletingLine.value)
    if (!lineId) {
      throw new Error('Unable to determine production line identifier')
    }

    const result = await factoryStore.deleteProductionLine(props.factoryId, lineId)
    if (!result) {
      throw new Error('Delete operation failed')
    }

    showDeleteModal.value = false
    deletingLine.value = null
  } catch (err) {
    error.value = factoryStore.error || 'Failed to delete production line'
    console.error('Delete production line error:', err)
  } finally {
    deleting.value = false
  }
}

const handleSaved = async () => {
  showCreateModal.value = false
  editingLine.value = null

  // Refresh factory data
  await factoryStore.fetchById(props.factoryId)
}

const clearError = () => {
  error.value = null
  factoryStore.clearError()
}

/**
 * Export a blueprint to JSON file
 */
const handleExportBlueprint = async (lineId: string, lineName: string) => {
  try {
    if (!props.factoryId) return

    const response = await blueprints.export(props.factoryId, lineId)

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

  } catch (err) {
    error.value = 'Failed to export blueprint'
    console.error('Failed to export blueprint:', err)
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

      // Basic validation before sending to backend
      if (!parsed.id || !parsed.name || !Array.isArray(parsed.production_lines)) {
        throw new Error('Invalid blueprint file format - must have id, name, and production_lines array')
      }

      // Validate UUID format for blueprint ID
      const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i
      if (!uuidRegex.test(parsed.id)) {
        throw new Error('Invalid blueprint ID - must be a valid UUID')
      }

      // Validate production lines structure
      if (parsed.production_lines.length === 0) {
        throw new Error('Blueprint must contain at least one production line')
      }

      for (const line of parsed.production_lines) {
        if (!line.id || !line.recipe || !Array.isArray(line.machine_groups)) {
          throw new Error('Invalid production line structure')
        }
      }

      // Get metadata from backend preview endpoint
      // This calculates power, items, etc. using the engine
      blueprintJsonToImport.value = text
      blueprintMetadata.value = await blueprints.preview(text)
      showBlueprintPreview.value = true
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Invalid blueprint file'
      console.error('Failed to read blueprint file:', err)
    }
  }
  input.click()
}

/**
 * Confirm and perform blueprint import
 */
const handleConfirmImport = async (customName?: string) => {
  try {
    if (!props.factoryId) return

    await blueprints.import(props.factoryId, {
      blueprint_json: blueprintJsonToImport.value,
      name: customName
    })

    // Refresh factory data
    await factoryStore.fetchById(props.factoryId)

    // Close modal
    showBlueprintPreview.value = false
    blueprintMetadata.value = null
    blueprintJsonToImport.value = ''
  } catch (err) {
    error.value = 'Failed to import blueprint'
    console.error('Failed to import blueprint:', err)
  }
}

const handleCloseBlueprintPreview = () => {
  showBlueprintPreview.value = false
  blueprintMetadata.value = null
  blueprintJsonToImport.value = ''
}

// Load data on mount
onMounted(async () => {
  if (props.factoryId) {
    await factoryStore.fetchById(props.factoryId)
  }
})

// Handle blueprint selector modal
const handleBlueprintSelectorClick = () => {
  showBlueprintSelector.value = true
  showDropdown.value = false
}

// Handle blueprint selection from selector modal
const handleBlueprintSelected = async (templateId: string, name: string) => {
  try {
    await blueprintTemplates.createInstanceInFactory(
      props.factoryId,
      templateId,
      { name: name || undefined }
    )

    // Refresh factory data
    await factoryStore.fetchById(props.factoryId)

    showBlueprintSelector.value = false
    showSuccess(`Blueprint '${name}' added to factory successfully`)
  } catch (err) {
    const appError = handleError(err, { action: 'create_blueprint_instance' })
    showError(appError.userMessage || 'Failed to create blueprint instance')
  }
}

// Expose methods for parent components
defineExpose({
  openCreateModal: () => {
    showCreateModal.value = true
  },
  handleImportButtonClick
})
</script>

<style scoped lang="scss">
.production-line-list {
  background-color: transparent;
  border-radius: var(--border-radius-lg);
  overflow: hidden;
}

.production-line-name {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.name {
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  font-family: var(--font-family-sans);
}

.description {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
  font-style: italic;
  line-height: 1.4;
}

.recipe-name {
  font-family: var(--font-family-mono);
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  background-color: var(--color-surface-inset);
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--border-radius-sm);
  display: inline-block;
}

.machine-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.machine-count {
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  font-family: var(--font-family-mono);
}

.machine-details {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
}

.power-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.power-value {
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  font-family: var(--font-family-mono);
}

.somersloop-count {
  font-size: var(--font-size-xs);
  color: var(--color-warning);
  font-weight: var(--font-weight-medium);
  background-color: rgba(245, 158, 11, 0.1);
  padding: var(--spacing-2xs) var(--spacing-xs);
  border-radius: var(--border-radius-sm);
  display: inline-block;
}

.consumed-items,
.produced-items {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.consumed-item,
.produced-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.consumed-quantity,
.produced-quantity {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  font-family: var(--font-family-mono);
}

.no-input,
.no-output {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
  font-style: italic;
}

.action-buttons {
  display: flex;
  gap: var(--spacing-xs);
  flex-wrap: wrap;
}

.delete-confirmation {
  text-align: center;
  padding: var(--spacing-lg) 0;
}

.line-name {
  margin: var(--spacing-md) 0;
  color: var(--color-text-primary);
  font-weight: var(--font-weight-semibold);
}

.warning-text {
  font-size: var(--font-size-sm);
  color: var(--color-error);
  margin-top: var(--spacing-md);
}

.modal-actions {
  display: flex;
  gap: var(--spacing-md);
  justify-content: flex-end;
}

// Production Line Type Selector Styles
.production-line-type-selector {
  position: relative;
  display: inline-block;
}

.dropdown-arrow {
  display: inline-block;
  width: 0;
  height: 0;
  margin-left: var(--spacing-xs);
  border-left: 6px solid transparent;
  border-right: 6px solid transparent;
  border-top: 6px solid var(--color-text-muted);
  transform: translateY(2px);
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  left: 0;
  z-index: 50;
  background-color: var(--color-surface-inset);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-md);
  box-shadow: var(--shadow-lg);
  min-width: 200px;
  overflow: hidden;
}

.dropdown-item {
  display: flex;
  align-items: center;
  padding: var(--spacing-md) var(--spacing-lg);
  cursor: pointer;
  transition: all var(--transition-normal);
  color: var(--color-text-primary);

  &:hover {
    background-color: var(--color-surface-hover);
    color: var(--color-ficsit-orange);
  }
}

.dropdown-icon {
  margin-right: var(--spacing-sm);
}

// Responsive design
@media (max-width: 768px) {
  .action-buttons {
    flex-direction: column;
    gap: var(--spacing-xs);
  }

  .production-line-type-selector {
    width: 100%;
    margin-bottom: var(--spacing-sm);
  }

  .dropdown-menu {
    position: static;
    box-shadow: none;
    border: none;
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
    background-color: transparent;
  }

  .dropdown-arrow {
    display: none;
  }
}
</style>
