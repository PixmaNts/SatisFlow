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
  border-radius: var(--border-radius-lg);
  overflow: hidden;
}

.resource-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.resource-name {
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  font-family: var(--font-family-sans);
}

.extractor-type {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
  font-style: italic;
}

.purity-badge {
  display: inline-block;
  padding: var(--spacing-xs) var(--spacing-md);
  border-radius: var(--border-radius-full);
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.02em;

  &.purity-impure {
    background-color: rgba(239, 68, 68, 0.15);
    color: var(--color-error);
    border: 1px solid rgba(239, 68, 68, 0.3);
  }

  &.purity-normal {
    background-color: rgba(138, 138, 138, 0.15);
    color: var(--color-text-secondary);
    border: 1px solid var(--color-border);
  }

  &.purity-pure {
    background-color: rgba(34, 197, 94, 0.15);
    color: var(--color-success);
    border: 1px solid rgba(34, 197, 94, 0.3);
  }
}

.purity-none {
  color: var(--color-text-muted);
  font-style: italic;
}

.rate-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.rate-value {
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  font-family: var(--font-family-mono);
}

.pressurized-indicator {
  color: var(--color-warning);
  font-size: var(--font-size-sm);
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

.extractor-count {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
}

.count-badge {
  display: inline-block;
  padding: var(--spacing-2xs) var(--spacing-sm);
  border-radius: var(--border-radius-sm);
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-semibold);
  background-color: rgba(245, 139, 0, 0.15);
  color: var(--color-primary-400);
  border: 1px solid rgba(245, 139, 0, 0.3);
}

.oc-value {
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  font-family: var(--font-family-mono);
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

.input-name {
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

// Responsive design
@media (max-width: 768px) {
  .action-buttons {
    flex-direction: column;
    gap: var(--spacing-xs);
  }
}
</style>
