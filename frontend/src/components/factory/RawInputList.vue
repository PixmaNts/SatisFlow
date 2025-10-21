<template>
  <div class="raw-input-list">
    <div class="list-header">
      <h3 class="list-title">Raw Inputs</h3>
      <Button
        variant="primary"
        size="sm"
        @click="showCreateModal = true"
      >
        Add Raw Input
      </Button>
    </div>

    <DataTable
      :columns="columns"
      :data="rawInputs"
      :loading="loading"
      :empty-text="emptyText"
      @row-click="(row: Record<string, unknown>) => handleRowClick(row as unknown as RawInputResponse)"
    >
      <template #cell-resource="{ row }">
        <div class="resource-info">
          <span class="resource-name">{{ row.item }}</span>
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
        </div>
      </template>

      <template #cell-power="{ row }">
        <div class="power-info">
          <span class="power-value">{{ formatPower(getPowerConsumption(row as unknown as RawInputResponse)) }}</span>
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
import RawInputForm from './RawInputForm.vue'

interface Props {
  factoryId: number
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
    width: '25%'
  },
  {
    key: 'purity',
    label: 'Purity',
    sortable: true,
    width: '15%'
  },
  {
    key: 'rate',
    label: 'Rate',
    sortable: true,
    width: '20%'
  },
  {
    key: 'power',
    label: 'Power',
    sortable: true,
    width: '20%'
  },
  {
    key: 'actions',
    label: 'Actions',
    sortable: false,
    width: '20%'
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

const getPowerConsumption = (input: RawInputResponse): number => {
  // Base power consumption for different extractor types
  const basePower: Record<ExtractorType, number> = {
    MinerMk1: 5,
    MinerMk2: 10,
    MinerMk3: 20,
    WaterExtractor: 10,
    OilExtractor: 30,
    ResourceWellExtractor: 0 // Extractors are powered by pressurizer
  }

  if (input.pressurizer) {
    // Resource well pressurizer power consumption
    const clockSpeed = input.pressurizer.clock_speed / 100
    return 150 * Math.pow(clockSpeed, 1.321928)
  }

  return basePower[input.extractor_type] || 0
}

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
    // TODO: Implement delete API call when available
    // await factoryStore.deleteRawInput(deletingInput.value.id)

    // For now, just close the modal
    showDeleteModal.value = false
    deletingInput.value = null

    // Refresh factory data
    await factoryStore.fetchById(props.factoryId)
  } catch (err) {
    error.value = 'Failed to delete raw input'
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
</script>

<style scoped lang="scss">
.raw-input-list {
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

.resource-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
}

.resource-name {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-900, #111827);
}

.extractor-type {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-500, #6b7280);
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
    background-color: var(--color-red-100, #fee2e2);
    color: var(--color-red-800, #991b1b);
  }

  &.purity-normal {
    background-color: var(--color-gray-100, #f3f4f6);
    color: var(--color-gray-800, #1f2937);
  }

  &.purity-pure {
    background-color: var(--color-green-100, #d1fae5);
    color: var(--color-green-800, #065f46);
  }
}

.purity-none {
  color: var(--color-gray-500, #6b7280);
  font-style: italic;
}

.rate-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
}

.rate-value {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-900, #111827);
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
  color: var(--color-gray-900, #111827);
}

.extractor-count {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-500, #6b7280);
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
