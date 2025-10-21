<template>
  <div class="power-generator-list">
    <div class="list-header">
      <h3 class="list-title">Power Generation</h3>
      <Button
        variant="primary"
        size="sm"
        @click="showCreateModal = true"
      >
        Add Power Generator
      </Button>
    </div>

    <DataTable
      :columns="columns"
      :data="powerGenerators"
      :loading="loading"
      :empty-text="emptyText"
      @row-click="(row: Record<string, unknown>) => handleRowClick(row as unknown as PowerGeneratorResponse)"
    >
      <template #cell-type="{ row }">
        <div class="generator-info">
          <span class="generator-type">{{ getGeneratorDisplayName((row as unknown as PowerGeneratorResponse).generator_type) }}</span>
          <span v-if="(row as unknown as PowerGeneratorResponse).fuel_type" class="fuel-type">
            {{ formatItemName((row as unknown as PowerGeneratorResponse).fuel_type || '') }}
          </span>
        </div>
      </template>

      <template #cell-generators="{ row }">
        <div class="generators-info">
          <span class="generator-count">{{ getTotalGenerators(row as unknown as PowerGeneratorResponse) }}</span>
          <span v-if="(row as unknown as PowerGeneratorResponse).groups.length > 1" class="group-count">
            {{ (row as unknown as PowerGeneratorResponse).groups.length }} groups
          </span>
        </div>
      </template>

      <template #cell-power="{ row }">
        <div class="power-info">
          <span class="power-value">{{ formatPower(getPowerOutput(row as unknown as PowerGeneratorResponse)) }}</span>
          <span v-if="(row as unknown as PowerGeneratorResponse).fuel_type" class="fuel-rate">
            {{ formatFuelRate(getFuelConsumption(row as unknown as PowerGeneratorResponse)) }}
          </span>
        </div>
      </template>

      <template #cell-actions="{ row }">
        <div class="action-buttons">
          <Button
            variant="secondary"
            size="sm"
            @click.stop="handleEdit(row as unknown as PowerGeneratorResponse)"
          >
            Edit
          </Button>
          <Button
            variant="danger"
            size="sm"
            @click.stop="handleDelete(row as unknown as PowerGeneratorResponse)"
          >
            Delete
          </Button>
        </div>
      </template>
    </DataTable>

    <!-- Create/Edit Power Generator Modal -->
    <PowerGeneratorForm
      v-model:show="showCreateModal"
      :factory-id="factoryId"
      :power-generator="editingGenerator"
      @saved="handleSaved"
    />

    <!-- Delete Confirmation Modal -->
    <Modal
      v-model:show="showDeleteModal"
      title="Delete Power Generator"
      @close="showDeleteModal = false"
    >
      <div class="delete-confirmation">
        <p>Are you sure you want to delete this power generator?</p>
        <p v-if="deletingGenerator" class="generator-name">
          <strong>{{ getGeneratorDisplayName(deletingGenerator.generator_type) }}</strong>
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
import { useGameDataStore } from '@/stores/gameData'
import type { PowerGeneratorResponse, GeneratorType } from '@/api/types'
import Button from '@/components/ui/Button.vue'
import DataTable from '@/components/ui/DataTable.vue'
import Modal from '@/components/ui/Modal.vue'
import Alert from '@/components/ui/Alert.vue'
import PowerGeneratorForm from './PowerGeneratorForm.vue'

interface Props {
  factoryId: number
}

const props = defineProps<Props>()
const factoryStore = useFactoryStore()
// const gameDataStore = useGameDataStore()

// State
const showCreateModal = ref(false)
const showDeleteModal = ref(false)
const editingGenerator = ref<PowerGeneratorResponse | null>(null)
const deletingGenerator = ref<PowerGeneratorResponse | null>(null)
const deleting = ref(false)
const error = ref<string | null>(null)

// Computed
const currentFactory = computed(() => factoryStore.currentFactory)
const powerGenerators = computed(() => currentFactory.value?.power_generators || [])
const loading = computed(() => factoryStore.loading)

const emptyText = computed(() => {
  if (!props.factoryId) {
    return 'Please select a factory first'
  }
  return 'No power generators found. Add your first power generator to get started.'
})

// DataTable columns
const columns = [
  {
    key: 'type',
    label: 'Type',
    sortable: true,
    width: '30%'
  },
  {
    key: 'generators',
    label: 'Generators',
    sortable: false,
    width: '25%'
  },
  {
    key: 'power',
    label: 'Power Output',
    sortable: true,
    width: '25%'
  },
  {
    key: 'actions',
    label: 'Actions',
    sortable: false,
    width: '20%'
  }
]

// Methods
const getGeneratorDisplayName = (type: GeneratorType): string => {
  const displayNames: Record<GeneratorType, string> = {
    Biomass: 'Biomass Burner',
    Coal: 'Coal Generator',
    Fuel: 'Fuel Generator',
    Nuclear: 'Nuclear Power Plant',
    Geothermal: 'Geothermal Generator'
  }
  return displayNames[type] || type
}

const formatItemName = (item: string): string => {
  return item.replace(/([A-Z])/g, ' $1').trim()
}

const getTotalGenerators = (generator: PowerGeneratorResponse): number => {
  return generator.groups.reduce((total, group) => total + group.number_of_generators, 0)
}

const getPowerOutput = (generator: PowerGeneratorResponse): number => {
  // Base power output for different generator types
  const basePower: Record<GeneratorType, number> = {
    Biomass: 30,
    Coal: 150,
    Fuel: 150,
    Nuclear: 2500,
    Geothermal: 200
  }

  const basePowerValue = basePower[generator.generator_type] || 0

  return generator.groups.reduce((total, group) => {
    const clockSpeed = group.clock_speed / 100
    const powerMultiplier = Math.pow(clockSpeed, 1.321928)
    return total + (group.number_of_generators * basePowerValue * powerMultiplier)
  }, 0)
}

const getFuelConsumption = (generator: PowerGeneratorResponse): number => {
  if (!generator.fuel_type) return 0

  // Base fuel consumption rates (items/min) for different generators
  const baseFuelRate: Record<GeneratorType, number> = {
    Biomass: 4,
    Coal: 15.3,
    Fuel: 4.5,
    Nuclear: 0.025, // Uranium Fuel Rods/min
    Geothermal: 0 // No fuel consumption
  }

  const baseRate = baseFuelRate[generator.generator_type] || 0

  return generator.groups.reduce((total, group) => {
    const clockSpeed = group.clock_speed / 100
    const fuelMultiplier = Math.pow(clockSpeed, 1.321928)
    return total + (group.number_of_generators * baseRate * fuelMultiplier)
  }, 0)
}

const formatPower = (power: number): string => {
  if (power < 1) {
    return `${(power * 1000).toFixed(0)} kW`
  }
  return `${power.toFixed(1)} MW`
}

const formatFuelRate = (rate: number): string => {
  if (rate < 1) {
    return `${(rate * 60).toFixed(1)}/h`
  }
  return `${rate.toFixed(2)}/min`
}

const handleRowClick = (row: PowerGeneratorResponse) => {
  handleEdit(row)
}

const handleEdit = (generator: PowerGeneratorResponse) => {
  editingGenerator.value = generator
  showCreateModal.value = true
}

const handleDelete = (generator: PowerGeneratorResponse) => {
  deletingGenerator.value = generator
  showDeleteModal.value = true
}

const confirmDelete = async () => {
  if (!deletingGenerator.value) return

  deleting.value = true
  error.value = null

  try {
    // TODO: Implement delete API call when available
    // await factoryStore.deletePowerGenerator(deletingGenerator.value.id)

    // For now, just close the modal
    showDeleteModal.value = false
    deletingGenerator.value = null

    // Refresh factory data
    await factoryStore.fetchById(props.factoryId)
  } catch (err) {
    error.value = 'Failed to delete power generator'
    console.error('Delete power generator error:', err)
  } finally {
    deleting.value = false
  }
}

const handleSaved = async () => {
  showCreateModal.value = false
  editingGenerator.value = null

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
.power-generator-list {
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

.generator-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
}

.generator-type {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-900, #111827);
}

.fuel-type {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-500, #6b7280);
  font-style: italic;
}

.generators-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
}

.generator-count {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-900, #111827);
}

.group-count {
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

.fuel-rate {
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

.generator-name {
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
