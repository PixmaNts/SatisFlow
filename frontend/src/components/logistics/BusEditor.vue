<template>
  <div class="bus-editor">
    <div class="editor-section">
      <h3 class="section-title">Conveyors</h3>
      <div class="items-list">
        <div
          v-for="(conveyor, index) in busConfig.conveyors"
          :key="index"
          class="item-row"
        >
          <div class="item-field">
            <label>Item</label>
            <select
              v-model="conveyor.item"
              class="form-select"
              @change="updateConfig"
            >
              <option value="">Select item</option>
              <option
                v-for="item in solidItems"
                :key="item"
                :value="item"
              >
                {{ formatItemName(item) }}
              </option>
            </select>
          </div>

          <div class="item-field">
            <label>Conveyor Type</label>
            <select
              v-model="conveyor.conveyor_type"
              class="form-select"
              @change="updateConfig"
            >
              <option
                v-for="(speed, type) in CONVEYOR_SPEEDS"
                :key="type"
                :value="type"
              >
                {{ type }} ({{ speed }} items/min)
              </option>
            </select>
          </div>

          <div class="item-field">
            <label>Quantity/min</label>
            <input
              v-model.number="conveyor.quantity_per_min"
              type="number"
              min="0"
              :max="CONVEYOR_SPEEDS[conveyor.conveyor_type]"
              class="form-input"
              @input="updateConfig"
            />
          </div>

          <div class="item-field">
            <label>Line ID</label>
            <input
              v-model="conveyor.line_id"
              type="text"
              class="form-input"
              placeholder="e.g., CV-001"
              @input="updateConfig"
            />
          </div>

          <button
            type="button"
            class="remove-button"
            @click="removeConveyor(index)"
          >
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M2 4h12v1H2V4zm1 2v8a1 1 0 001 1h6a1 1 0 001-1V6H3zm2 1h1v6H5V7zm3 0h1v6H8V7z"/>
            </svg>
          </button>
        </div>
      </div>

      <button
        type="button"
        class="add-button"
        @click="addConveyor"
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M8 2v6m0 0v6m0-6h6m-6 0H2" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        </svg>
        Add Conveyor
      </button>
    </div>

    <div class="editor-section">
      <h3 class="section-title">Pipelines</h3>
      <div class="items-list">
        <div
          v-for="(pipeline, index) in busConfig.pipelines"
          :key="index"
          class="item-row"
        >
          <div class="item-field">
            <label>Item</label>
            <select
              v-model="pipeline.item"
              class="form-select"
              @change="updateConfig"
            >
              <option value="">Select item</option>
              <option
                v-for="item in fluidItems"
                :key="item"
                :value="item"
              >
                {{ formatItemName(item) }}
              </option>
            </select>
          </div>

          <div class="item-field">
            <label>Pipeline Type</label>
            <select
              v-model="pipeline.pipeline_type"
              class="form-select"
              @change="updateConfig"
            >
              <option
                v-for="(capacity, type) in PIPELINE_CAPACITIES"
                :key="type"
                :value="type"
              >
                {{ type }} ({{ capacity }} m³/min)
              </option>
            </select>
          </div>

          <div class="item-field">
            <label>Quantity/min</label>
            <input
              v-model.number="pipeline.quantity_per_min"
              type="number"
              min="0"
              :max="PIPELINE_CAPACITIES[pipeline.pipeline_type]"
              class="form-input"
              @input="updateConfig"
            />
          </div>

          <div class="item-field">
            <label>Pipeline ID</label>
            <input
              v-model="pipeline.pipeline_id"
              type="text"
              class="form-input"
              placeholder="e.g., PL-001"
              @input="updateConfig"
            />
          </div>

          <button
            type="button"
            class="remove-button"
            @click="removePipeline(index)"
          >
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M2 4h12v1H2V4zm1 2v8a1 1 0 001 1h6a1 1 0 001-1V6H3zm2 1h1v6H5V7zm3 0h1v6H8V7z"/>
            </svg>
          </button>
        </div>
      </div>

      <button
        type="button"
        class="add-button"
        @click="addPipeline"
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M8 2v6m0 0v6m0-6h6m-6 0H2" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        </svg>
        Add Pipeline
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import type { BusConfig, ConveyorConfig, PipelineConfig } from '@/api/logistics-types'
import type { Item } from '@/api/types'
import { CONVEYOR_SPEEDS, PIPELINE_CAPACITIES } from '@/api/logistics-types'

interface Props {
  modelValue: BusConfig
}

interface Emits {
  'update:modelValue': [value: BusConfig]
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const busConfig = ref<BusConfig>({ ...props.modelValue })

// Sample items - in a real app, these would come from the API
const solidItems: Item[] = [
  'IronOre', 'IronIngot', 'IronPlate', 'IronRod', 'Screw',
  'CopperOre', 'CopperIngot', 'CopperSheet', 'Wire', 'Cable',
  'Coal', 'Biomass', 'Concrete', 'Limestone'
]

const fluidItems: Item[] = [
  'Water', 'CrudeOil', 'HeavyOilResidue', 'Fuel', 'Turbofuel',
  'LiquidBiofuel', 'NitrogenGas', 'PackagedWater', 'PackagedOil'
]

const formatItemName = (item: Item): string => {
  return item.replace(/([A-Z])/g, ' $1').trim()
}

const addConveyor = () => {
  const newConveyor: ConveyorConfig = {
    line_id: `CV-${String(busConfig.value.conveyors.length + 1).padStart(3, '0')}`,
    item: '' as Item,
    conveyor_type: 'Mk1',
    quantity_per_min: 0
  }
  busConfig.value.conveyors.push(newConveyor)
  updateConfig()
}

const removeConveyor = (index: number) => {
  busConfig.value.conveyors.splice(index, 1)
  updateConfig()
}

const addPipeline = () => {
  const newPipeline: PipelineConfig = {
    pipeline_id: `PL-${String(busConfig.value.pipelines.length + 1).padStart(3, '0')}`,
    item: '' as Item,
    pipeline_type: 'Mk1',
    quantity_per_min: 0
  }
  busConfig.value.pipelines.push(newPipeline)
  updateConfig()
}

const removePipeline = (index: number) => {
  busConfig.value.pipelines.splice(index, 1)
  updateConfig()
}

const updateConfig = () => {
  emit('update:modelValue', { ...busConfig.value })
}

// Watch for prop changes
watch(() => props.modelValue, (newValue) => {
  busConfig.value = { ...newValue }
}, { deep: true })
</script>

<style scoped lang="scss">
.bus-editor {
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
  grid-template-columns: 2fr 1.5fr 1fr 1.5fr auto;
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

// Responsive design
@media (max-width: 768px) {
  .item-row {
    grid-template-columns: 1fr;
    gap: var(--spacing-xs, 0.25rem);
  }

  .remove-button {
    justify-self: end;
  }
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'LogisticsBusEditor'
}
</script>
