<template>
  <div class="train-editor">
    <div class="editor-section">
      <h3 class="section-title">Train Wagons</h3>
      <div class="items-list">
        <div
          v-for="(wagon, index) in trainConfig.wagons"
          :key="index"
          class="item-row"
        >
          <div class="item-field">
            <label>Wagon Type</label>
            <select
              v-model="wagon.wagon_type"
              class="form-select"
              @change="updateConfig"
            >
              <option value="Cargo">Cargo Wagon</option>
              <option value="Fluid">Fluid Wagon</option>
            </select>
          </div>

          <div class="item-field">
            <label>Item</label>
            <select
              v-model="wagon.item"
              class="form-select"
              @change="updateConfig"
            >
              <option value="">Select item</option>
              <option
                v-for="item in availableItems(wagon.wagon_type)"
                :key="item"
                :value="item"
              >
                {{ formatItemName(item) }}
              </option>
            </select>
          </div>

          <div class="item-field">
            <label>Quantity/min</label>
            <input
              v-model.number="wagon.quantity_per_min"
              type="number"
              min="0"
              step="0.1"
              class="form-input"
              @input="updateConfig"
            />
          </div>

          <div class="item-field">
            <label>Wagon ID</label>
            <input
              v-model="wagon.wagon_id"
              type="text"
              class="form-input"
              placeholder="e.g., WG-001"
              @input="updateConfig"
            />
          </div>

          <button
            type="button"
            class="remove-button"
            @click="removeWagon(index)"
          >
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M2 4h12v1H2V4zm1 2v8a1 1 0 001 1h6a1 1 0 001-1V6H3zm2 1h1v6H5V7zm3 0h1v6H8V7z"/>
            </svg>
          </button>
        </div>
      </div>

      <div class="add-wagon-section">
        <div class="add-wagon-buttons">
          <button
            type="button"
            class="add-button"
            @click="addWagon('Cargo')"
          >
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M8 2v6m0 0v6m0-6h6m-6 0H2" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            </svg>
            Add Cargo Wagon
          </button>

          <button
            type="button"
            class="add-button"
            @click="addWagon('Fluid')"
          >
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M8 2v6m0 0v6m0-6h6m-6 0H2" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            </svg>
            Add Fluid Wagon
          </button>
        </div>
      </div>
    </div>

    <div class="train-summary">
      <h4 class="summary-title">Train Summary</h4>
      <div class="summary-stats">
        <div class="stat">
          <span class="stat-label">Total Wagons:</span>
          <span class="stat-value">{{ trainConfig.wagons.length }}</span>
        </div>
        <div class="stat">
          <span class="stat-label">Cargo Wagons:</span>
          <span class="stat-value">{{ cargoWagonCount }}</span>
        </div>
        <div class="stat">
          <span class="stat-label">Fluid Wagons:</span>
          <span class="stat-value">{{ fluidWagonCount }}</span>
        </div>
        <div class="stat">
          <span class="stat-label">Total Capacity:</span>
          <span class="stat-value">{{ totalCapacity }} items/min</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { TrainConfig, WagonConfig } from '@/api/logistics-types'
import type { Item } from '@/api/types'

interface Props {
  modelValue: TrainConfig
}

interface Emits {
  'update:modelValue': [value: TrainConfig]
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const trainConfig = ref<TrainConfig>({ ...props.modelValue })

// Sample items - in a real app, these would come from the API
const solidItems: Item[] = [
  'IronOre', 'IronIngot', 'IronPlate', 'IronRod', 'Screw',
  'CopperOre', 'CopperIngot', 'CopperSheet', 'Wire', 'Cable',
  'Coal', 'Biomass', 'Concrete', 'Limestone',
  'SteelBeam', 'SteelPipe', 'ModularFrame', 'Rotor', 'Stator'
]

const fluidItems: Item[] = [
  'Water', 'CrudeOil', 'HeavyOilResidue', 'Fuel', 'Turbofuel',
  'LiquidBiofuel', 'NitrogenGas', 'PackagedWater', 'PackagedOil',
  'PackagedFuel', 'PackagedTurbofuel'
]

const formatItemName = (item: Item): string => {
  return item.replace(/([A-Z])/g, ' $1').trim()
}

const availableItems = (wagonType: 'Cargo' | 'Fluid'): Item[] => {
  return wagonType === 'Cargo' ? solidItems : fluidItems
}

const cargoWagonCount = computed(() =>
  trainConfig.value.wagons.filter(w => w.wagon_type === 'Cargo').length
)

const fluidWagonCount = computed(() =>
  trainConfig.value.wagons.filter(w => w.wagon_type === 'Fluid').length
)

const totalCapacity = computed(() =>
  trainConfig.value.wagons.reduce((sum, wagon) => sum + wagon.quantity_per_min, 0)
)

const addWagon = (wagonType: 'Cargo' | 'Fluid') => {
  const wagonNumber = trainConfig.value.wagons.length + 1
  const newWagon: WagonConfig = {
    wagon_id: `WG-${String(wagonNumber).padStart(3, '0')}`,
    wagon_type: wagonType,
    item: '' as Item,
    quantity_per_min: 0
  }
  trainConfig.value.wagons.push(newWagon)
  updateConfig()
}

const removeWagon = (index: number) => {
  trainConfig.value.wagons.splice(index, 1)
  // Renumber remaining wagons
  trainConfig.value.wagons.forEach((wagon, idx) => {
    wagon.wagon_id = `WG-${String(idx + 1).padStart(3, '0')}`
  })
  updateConfig()
}

const updateConfig = () => {
  emit('update:modelValue', { ...trainConfig.value })
}

// Watch for prop changes
watch(() => props.modelValue, (newValue) => {
  trainConfig.value = { ...newValue }
}, { deep: true })
</script>

<style scoped lang="scss">
.train-editor {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg, 1rem);
}

.editor-section {
  border: 1px solid var(--color-gray-200, #e5e7eb);
  border-radius: var(--border-radius-md, 0.375rem);
  padding: var(--spacing-md, 0.75rem);
  background-color: var(--color-gray-50, #f9fafb);
}

.section-title {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-gray-900, #111827);
  margin: 0 0 var(--spacing-md, 0.75rem) 0;
}

.items-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm, 0.5rem);
  margin-bottom: var(--spacing-md, 0.75rem);
}

.item-row {
  display: grid;
  grid-template-columns: 1.5fr 2fr 1fr 1.5fr auto;
  gap: var(--spacing-sm, 0.5rem);
  align-items: end;
  padding: var(--spacing-sm, 0.5rem);
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-sm, 0.25rem);
  border: 1px solid var(--color-gray-200, #e5e7eb);
}

.item-field {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);

  label {
    font-size: var(--font-size-xs, 0.75rem);
    font-weight: var(--font-weight-medium, 500);
    color: var(--color-gray-700, #374151);
  }
}

.form-input,
.form-select {
  padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-sm, 0.25rem);
  font-size: var(--font-size-sm, 0.875rem);
  background-color: var(--color-white, #ffffff);
  transition: border-color 0.2s ease-in-out;

  &:focus {
    outline: none;
    border-color: var(--color-primary-500, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }
}

.remove-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2rem;
  height: 2rem;
  padding: 0;
  border: none;
  background-color: var(--color-red-100, #fee2e2);
  color: var(--color-red-600, #dc2626);
  border-radius: var(--border-radius-sm, 0.25rem);
  cursor: pointer;
  transition: all 0.2s ease-in-out;

  &:hover {
    background-color: var(--color-red-200, #fecaca);
  }

  &:focus-visible {
    outline: 2px solid var(--color-red-500, #ef4444);
    outline-offset: 2px;
  }
}

.add-wagon-section {
  border-top: 1px solid var(--color-gray-200, #e5e7eb);
  padding-top: var(--spacing-md, 0.75rem);
}

.add-wagon-buttons {
  display: flex;
  gap: var(--spacing-sm, 0.5rem);
  flex-wrap: wrap;
}

.add-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  border: 1px dashed var(--color-gray-300, #d1d5db);
  background-color: var(--color-white, #ffffff);
  color: var(--color-gray-700, #374151);
  border-radius: var(--border-radius-sm, 0.25rem);
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  cursor: pointer;
  transition: all 0.2s ease-in-out;

  &:hover {
    border-color: var(--color-gray-400, #9ca3af);
    background-color: var(--color-gray-50, #f9fafb);
  }

  &:focus-visible {
    outline: 2px solid var(--color-primary-500, #3b82f6);
    outline-offset: 2px;
  }
}

.train-summary {
  background-color: var(--color-blue-50, #eff6ff);
  border: 1px solid var(--color-blue-200, #bfdbfe);
  border-radius: var(--border-radius-md, 0.375rem);
  padding: var(--spacing-md, 0.75rem);
}

.summary-title {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-blue-900, #1e3a8a);
  margin: 0 0 var(--spacing-sm, 0.5rem) 0;
}

.summary-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: var(--spacing-sm, 0.5rem);
}

.stat {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-xs, 0.25rem);
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-sm, 0.25rem);
  border: 1px solid var(--color-blue-100, #dbeafe);
}

.stat-label {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-blue-700, #1d4ed8);
  font-weight: var(--font-weight-medium, 500);
}

.stat-value {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-blue-900, #1e3a8a);
  font-weight: var(--font-weight-semibold, 600);
}

// Responsive design
@media (max-width: 768px) {
  .item-row {
    grid-template-columns: 1fr;
    gap: var(--spacing-xs, 0.25rem);
  }

  .remove-button {
    justify-self: end;
  }

  .add-wagon-buttons {
    flex-direction: column;
  }

  .summary-stats {
    grid-template-columns: 1fr;
  }
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'LogisticsTrainEditor'
}
</script>
