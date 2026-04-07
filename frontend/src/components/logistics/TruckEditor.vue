<template>
  <div class="truck-editor">
    <div class="editor-section">
      <h3 class="section-title">Truck Configuration</h3>
      <div class="truck-form">
        <div class="form-row">
          <div class="form-field">
            <label for="truck-item">Item</label>
            <select
              id="truck-item"
              v-model="truckConfig.item"
              class="form-select"
              @change="updateConfig"
            >
              <option value="">Select item</option>
              <option
                v-for="item in allItems"
                :key="item"
                :value="item"
              >
                {{ formatItemName(item) }}
              </option>
            </select>
          </div>

          <div class="form-field">
            <label for="truck-quantity">Quantity/min</label>
            <input
              id="truck-quantity"
              v-model.number="truckConfig.quantity_per_min"
              type="number"
              min="0"
              step="0.1"
              class="form-input"
              @input="updateConfig"
            />
          </div>
        </div>

        <div class="form-row">
          <div class="form-field">
            <label for="truck-id">Truck ID</label>
            <input
              id="truck-id"
              v-model="truckConfig.truck_id"
              type="text"
              class="form-input"
              placeholder="e.g., TRK-001"
              @input="updateConfig"
            />
          </div>

          <div class="form-field">
            <label for="truck-station">Station Name</label>
            <input
              id="truck-station"
              v-model="stationName"
              type="text"
              class="form-input"
              placeholder="e.g., Iron Ore Station"
              @input="updateConfig"
            />
          </div>
        </div>
      </div>

      <div class="truck-info">
        <h4 class="info-title">Truck Information</h4>
        <div class="info-grid">
          <div class="info-item">
            <span class="info-label">Truck ID:</span>
            <span class="info-value">{{ truckConfig.truck_id }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import type { TruckConfig } from '@/api/logistics-types'
import type { Item } from '@/api/types'
import { useItemIcon } from '@/composables/useItemIcon'

interface Props {
  modelValue: TruckConfig
}

interface Emits {
  'update:modelValue': [value: TruckConfig]
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const truckConfig = ref<TruckConfig>({
  ...props.modelValue,
})

const stationName = ref('')

// Sample items - in a real app, these would come from the API
const allItems: Item[] = [
  'IronOre', 'IronIngot', 'IronPlate', 'IronRod', 'Screw',
  'CopperOre', 'CopperIngot', 'CopperSheet', 'Wire', 'Cable',
  'Coal', 'Biomass', 'Concrete', 'Limestone',
  'SteelBeam', 'SteelPipe', 'ModularFrame', 'Rotor', 'Stator',
  'Water', 'CrudeOil', 'HeavyOilResidue', 'Fuel', 'Turbofuel',
  'LiquidBiofuel', 'NitrogenGas', 'PackagedWater', 'PackagedOil'
]

const { formatItemName } = useItemIcon()

const updateConfig = () => {
  emit('update:modelValue', { ...truckConfig.value })
}

// Watch for prop changes
watch(() => props.modelValue, (newValue) => {
  truckConfig.value = { ...newValue }
}, { deep: true })

// Initialize truck ID if not provided
if (!truckConfig.value.truck_id) {
  truckConfig.value.truck_id = 'TRK-001'
  updateConfig()
}
</script>

<style scoped lang="scss">
.truck-editor {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg, 1rem);
}

.editor-section {
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-md, 0.375rem);
  padding: var(--spacing-md, 0.75rem);
  background-color: var(--color-surface);
}

.section-title {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-text-primary);
  margin: 0 0 var(--spacing-md, 0.75rem) 0;
}

.truck-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md, 0.75rem);
  margin-bottom: var(--spacing-lg, 1rem);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-md, 0.75rem);
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);

  label {
    font-size: var(--font-size-sm, 0.875rem);
    font-weight: var(--font-weight-medium, 500);
    color: var(--color-text-secondary);
  }
}

.form-input,
.form-select {
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-sm, 0.25rem);
  font-size: var(--font-size-base, 1rem);
  background-color: var(--color-surface-inset);
  color: var(--color-text-primary);
  transition: border-color 0.2s ease-in-out;

  &:focus {
    outline: none;
    border-color: var(--color-transport-truck);
    box-shadow: 0 0 0 2px rgba(245, 158, 11, 0.2);
  }
}

.truck-info {
  background-color: var(--color-surface-inset);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-sm, 0.25rem);
  padding: var(--spacing-md, 0.75rem);
}

.info-title {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-text-primary);
  margin: 0 0 var(--spacing-sm, 0.5rem) 0;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: var(--spacing-sm, 0.5rem);
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
  background-color: var(--color-surface);
  border-radius: var(--border-radius-sm, 0.25rem);
}

.info-label {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-secondary);
  font-weight: var(--font-weight-medium, 500);
}

.info-value {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-primary);
  font-weight: var(--font-weight-semibold, 600);
}

// Responsive design
@media (max-width: 768px) {
  .form-row {
    grid-template-columns: 1fr;
  }

  .info-grid {
    grid-template-columns: 1fr;
  }

  .efficiency-item {
    flex-wrap: wrap;
  }

  .efficiency-label {
    min-width: auto;
    width: 100%;
  }
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'LogisticsTruckEditor'
}
</script>
