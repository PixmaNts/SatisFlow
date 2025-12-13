<template>
  <div class="raw-input-list">
    <DataTable
      :columns="columns"
      :data="rawInputs"
      :loading="loading"
      :empty-text="emptyText"
      @row-click="(row: Record<string, unknown>) => handleRowClick(row as unknown as RawInputResponse)"
    >
      <template #cell-resource="{ row }">
        <div class="resource-info">
          <ItemDisplay :item="String(row.item)" size="sm" />
          <span v-if="getExtractorDisplayName((row as unknown as RawInputResponse).extractor_type)" class="extractor-type">
            {{ getExtractorDisplayName((row as unknown as RawInputResponse).extractor_type) }}
          </span>
        </div>
      </template>

      <template #cell-purity="{ row }">
        <span v-if="(row as unknown as RawInputResponse).purity" class="purity-badge" :class="purityClass((row as unknown as RawInputResponse).purity!)">
          {{ (row as unknown as RawInputResponse).purity }}
        </span>
        <span v-else class="purity-none">N/A</span>
      </template>

      <template #cell-rate="{ row }">
        <div class="rate-info">
          <span class="rate-value">{{ formatRate((row as unknown as RawInputResponse).quantity_per_min) }}</span>
          <span v-if="(row as unknown as RawInputResponse).pressurizer" class="pressurized-indicator" title="Pressurized">
            ⚡
          </span>
          <span v-if="(row as unknown as RawInputResponse).count && (row as unknown as RawInputResponse).count > 1" class="count-badge" :title="`Group of ${(row as unknown as RawInputResponse).count} extractors`">
            ×{{ (row as unknown as RawInputResponse).count }}
          </span>
        </div>
      </template>

      <template #cell-oc="{ row }">
        <span class="oc-value">{{ ((row as unknown as RawInputResponse).overclock_percent ?? 100).toFixed(1) }}%</span>
      </template>

      <template #cell-power="{ row }">
        <div class="power-info">
          <span class="power-value">{{ formatPower((row as unknown as RawInputResponse).power_consumption) }}</span>
          <span v-if="(row as unknown as RawInputResponse).extractors && (row as unknown as RawInputResponse).extractors.length > 1" class="extractor-count">
            {{ (row as unknown as RawInputResponse).extractors.length }} nodes
          </span>
        </div>
      </template>

      <template #cell-actions="{ row }">
        <div class="action-buttons">
          <Button
            variant="secondary"
            size="sm"
            @click.stop="handleEdit(row as unknown as RawInputResponse)"
          >
            Edit
          </Button>
          <Button
            variant="danger"
            size="sm"
            @click.stop="handleDelete(row as unknown as RawInputResponse)"
          >
            Delete
          </Button>
        </div>
      </template>
    </DataTable>

    <!-- Create/Edit Raw Input Modal -->
    <RawInputForm
      v-model:show="showCreateModal"
      :factory-id="factoryId"
      :raw-input="editingInput"
      @saved="handleSaved"
    />

    <!-- Delete Confirmation Modal -->
    <Modal
      v-model:show="showDeleteModal"
      title="Delete Raw Input"
      @close="showDeleteModal = false"
    >
      <div class="delete-confirmation">
        <p>Are you sure you want to delete this raw input?</p>
        <p v-if="deletingInput" class="input-name">
          <strong>{{ deletingInput.item }}</strong>
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
import type { RawInputResponse, ExtractorType, Purity } from '@/api/types'
import Button from '@/components/ui/Button.vue'
import DataTable from '@/components/ui/DataTable.vue'
import Modal from '@/components/ui/Modal.vue'
import Alert from '@/components/ui/Alert.vue'
import ItemDisplay from '@/components/ui/ItemDisplay.vue'
import RawInputForm from './RawInputForm.vue'

interface Props {
  factoryId: string
}

const props = defineProps<Props>()
const factoryStore = useFactoryStore()

// State
const showCreateModal = ref(false)
const showDeleteModal = ref(false)
const editingInput = ref<RawInputResponse | null>(null)
const deletingInput = ref<RawInputResponse | null>(null)
const deleting = ref(false)
const error = ref<string | null>(null)

// Computed
const currentFactory = computed(() => factoryStore.currentFactory)
const rawInputs = computed(() => currentFactory.value?.raw_inputs || [])
const loading = computed(() => factoryStore.loading)

const emptyText = computed(() => {
  if (!props.factoryId) {
    return 'Please select a factory first'
  }
  return 'No raw inputs found. Add your first raw input to get started.'
})

// DataTable columns
const columns = [
  {
    key: 'resource',
    label: 'Resource',
    sortable: true,
    width: '20%'
  },
  {
    key: 'purity',
    label: 'Purity',
    sortable: true,
    width: '12%'
  },
  {
    key: 'rate',
    label: 'Rate',
    sortable: true,
    width: '18%'
  },
  {
    key: 'oc',
    label: 'OC',
    sortable: true,
    width: '10%'
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
    width: '25%'
  }
]

// Methods
const getExtractorDisplayName = (type: ExtractorType): string => {
  const displayNames: Record<ExtractorType, string> = {
    MinerMk1: 'Miner Mk1',
    MinerMk2: 'Miner Mk2',
    MinerMk3: 'Miner Mk3',
    WaterExtractor: 'Water Extractor',
    OilExtractor: 'Oil Extractor',
    ResourceWellExtractor: 'Resource Well'
  }
  return displayNames[type] || type
}

const purityClass = (purity: Purity): string => {
  return `purity-${purity.toLowerCase()}`
}

const formatRate = (rate: number): string => {
  if (rate >= 1000) {
    return `${(rate / 1000).toFixed(1)}k`
  }
  return rate.toFixed(0)
}

// Removed getPowerConsumption wrapper - use input.power_consumption directly

const formatPower = (power: number): string => {
  if (power < 1) {
    return `${(power * 1000).toFixed(0)} kW`
  }
  return `${power.toFixed(1)} MW`
}

const handleRowClick = (row: RawInputResponse) => {
  handleEdit(row)
}

const handleEdit = (input: RawInputResponse) => {
  editingInput.value = input
  showCreateModal.value = true
}

const handleDelete = (input: RawInputResponse) => {
  deletingInput.value = input
  showDeleteModal.value = true
}

const confirmDelete = async () => {
  if (!deletingInput.value) return

  deleting.value = true
  error.value = null

  try {
    const result = await factoryStore.deleteRawInput(props.factoryId, deletingInput.value.id)
    if (!result) {
      throw new Error('Delete operation failed')
    }

    showDeleteModal.value = false
    deletingInput.value = null
  } catch (err) {
    error.value = factoryStore.error || 'Failed to delete raw input'
    console.error('Delete raw input error:', err)
  } finally {
    deleting.value = false
  }
}

const handleSaved = async () => {
  showCreateModal.value = false
  editingInput.value = null

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

// Expose methods for parent components
defineExpose({
  openCreateModal: () => {
    showCreateModal.value = true
  }
})
</script>

<style scoped lang="scss">
.raw-input-list {
  background-color: transparent;
  border-radius: var(--border-radius-lg, 0.5rem);
  overflow: hidden;
}

.resource-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
}

.resource-name {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary, #e5e5e5);
}

.extractor-type {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-text-secondary, #b8b8b8);
  font-style: italic;
}

.purity-badge {
  display: inline-block;
  padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
  border-radius: var(--border-radius-full, 9999px);
  font-size: var(--font-size-xs, 0.75rem);
  font-weight: var(--font-weight-medium, 500);
  text-transform: uppercase;

  &.purity-impure {
    background-color: rgba(239, 68, 68, 0.2);
    color: var(--color-error, #ef4444);
    border: 1px solid var(--color-error, #ef4444);
  }

  &.purity-normal {
    background-color: rgba(184, 184, 184, 0.2);
    color: var(--color-text-secondary, #b8b8b8);
    border: 1px solid var(--color-border, #404040);
  }

  &.purity-pure {
    background-color: rgba(34, 197, 94, 0.2);
    color: var(--color-success, #22c55e);
    border: 1px solid var(--color-success, #22c55e);
  }
}

.purity-none {
  color: var(--color-text-secondary, #b8b8b8);
  font-style: italic;
}

.rate-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
}

.rate-value {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary, #e5e5e5);
}

.pressurized-indicator {
  color: var(--color-amber-500, #f59e0b);
  font-size: var(--font-size-sm, 0.875rem);
}

.power-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
}

.power-value {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary, #e5e5e5);
}

.extractor-count {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-text-secondary, #b8b8b8);
}

.count-badge {
  display: inline-block;
  padding: 0.125rem 0.375rem;
  border-radius: var(--border-radius-sm, 0.25rem);
  font-size: var(--font-size-xs, 0.75rem);
  font-weight: var(--font-weight-medium, 500);
  background-color: rgba(59, 130, 246, 0.2);
  color: var(--color-primary-400, #60a5fa);
  border: 1px solid var(--color-primary-500, #3b82f6);
}

.oc-value {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary, #e5e5e5);
}

.action-buttons {
  display: flex;
  gap: var(--spacing-xs, 0.25rem);
}

.delete-confirmation {
  text-align: center;
  padding: var(--spacing-md, 0.75rem) 0;
}

.input-name {
  margin: var(--spacing-sm, 0.5rem) 0;
  color: var(--color-text-primary, #e5e5e5);
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
