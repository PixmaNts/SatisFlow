<template>
  <div class="production-line-list">
    <div class="list-header">
      <h3 class="list-title">Production Lines</h3>
      <Button
        variant="primary"
        size="sm"
        @click="showCreateModal = true"
      >
        Add Production Line
      </Button>
    </div>

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
          <span class="power-value">{{ formatPower(getPowerConsumption(row as ProductionLineResponse)) }}</span>
          <span v-if="getTotalSomersloops(row as ProductionLineResponse) > 0" class="somersloop-count">
            {{ getTotalSomersloops(row as ProductionLineResponse) }}×
          </span>
        </div>
      </template>

      <template #cell-actions="{ row }">
        <div class="action-buttons">
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

    <!-- Error Alert -->
    <Alert
      v-if="error"
      variant="danger"
      @close="clearError"
    >
      {{ error }}
    </Alert>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useFactoryStore } from '@/stores/factory'
import type { ProductionLineResponse, MachineGroup } from '@/api/types'
import Button from '@/components/ui/Button.vue'
import DataTable from '@/components/ui/DataTable.vue'
import Modal from '@/components/ui/Modal.vue'
import Alert from '@/components/ui/Alert.vue'
import ProductionLineForm from './ProductionLineForm.vue'

interface Props {
  factoryId: string
}

const props = defineProps<Props>()
const factoryStore = useFactoryStore()

// State
const showCreateModal = ref(false)
const showDeleteModal = ref(false)
const editingLine = ref<ProductionLineResponse | null>(null)
const deletingLine = ref<ProductionLineResponse | null>(null)
const deleting = ref(false)
const error = ref<string | null>(null)

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
    width: '25%'
  },
  {
    key: 'recipe',
    label: 'Recipe',
    sortable: true,
    width: '25%'
  },
  {
    key: 'machines',
    label: 'Machines',
    sortable: false,
    width: '20%'
  },
  {
    key: 'power',
    label: 'Power',
    sortable: true,
    width: '15%'
  },
  {
    key: 'actions',
    label: 'Actions',
    sortable: false,
    width: '15%'
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
  if (isProductionLineRecipe(line)) {
    return line.ProductionLineRecipe.machine_groups.reduce(
      (total: number, group: MachineGroup) => total + group.number_of_machine, 0
    )
  } else if (isProductionLineBlueprint(line)) {
    return line.ProductionLineBlueprint.production_lines.reduce(
      (total, subLine) => total + getTotalMachines(subLine), 0
    )
  }
  return 0
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

const getPowerConsumption = (line: ProductionLineResponse): number => {
  // This would need to be calculated based on the recipe and machine groups
  // For now, return a placeholder value
  if (isProductionLineRecipe(line)) {
    // Calculate power based on machine groups and overclock
    return line.ProductionLineRecipe.machine_groups.reduce(
      (total: number, group: MachineGroup) => total + (group.number_of_machine * 4 * Math.pow(group.oc_value / 100, 1.321928)),
      0
    )
  } else if (isProductionLineBlueprint(line)) {
    return line.ProductionLineBlueprint.production_lines.reduce(
      (total, subLine) => total + getPowerConsumption(subLine), 0
    )
  }
  return 0
}

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

// Load data on mount
onMounted(async () => {
  if (props.factoryId) {
    await factoryStore.fetchById(props.factoryId)
  }
})
</script>

<style scoped lang="scss">
.production-line-list {
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-lg, 0.5rem);
  box-shadow: var(--shadow-sm, 0 1px 2px 0 rgba(0, 0, 0, 0.05));
  overflow: hidden;
}

.list-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg, 1rem);
  border-bottom: 1px solid var(--color-gray-200, #e5e7eb);
}

.list-title {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-gray-900, #111827);
  margin: 0;
}

.production-line-name {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
}

.name {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-900, #111827);
}

.description {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-500, #6b7280);
  font-style: italic;
}

.recipe-name {
  font-family: var(--font-family-mono, 'Courier New', monospace);
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-gray-700, #374151);
}

.machine-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
}

.machine-count {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-900, #111827);
}

.machine-details {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-500, #6b7280);
}

.power-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
}

.power-value {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-900, #111827);
}

.somersloop-count {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-amber-600, #d97706);
  font-weight: var(--font-weight-medium, 500);
}

.action-buttons {
  display: flex;
  gap: var(--spacing-xs, 0.25rem);
}

.delete-confirmation {
  text-align: center;
  padding: var(--spacing-md, 0.75rem) 0;
}

.line-name {
  margin: var(--spacing-sm, 0.5rem) 0;
  color: var(--color-gray-900, #111827);
}

.warning-text {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-red-600, #dc2626);
  margin-top: var(--spacing-md, 0.75rem);
}

.modal-actions {
  display: flex;
  gap: var(--spacing-sm, 0.5rem);
  justify-content: flex-end;
}

// Responsive design
@media (max-width: 768px) {
  .list-header {
    flex-direction: column;
    align-items: stretch;
    gap: var(--spacing-sm, 0.5rem);
  }

  .action-buttons {
    flex-direction: column;
    gap: var(--spacing-xs, 0.25rem);
  }
}
</style>
