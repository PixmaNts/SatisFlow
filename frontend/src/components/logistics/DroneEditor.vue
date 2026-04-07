<template>
  <div class="drone-editor">
    <div class="editor-section">
      <h3 class="section-title">Drone Configuration</h3>
      <div class="drone-form">
        <div class="form-row">
          <div class="form-field">
            <label for="drone-item">Item</label>
            <select
              id="drone-item"
              v-model="droneConfig.item"
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
            <label for="drone-quantity">Quantity/min</label>
            <input
              id="drone-quantity"
              v-model.number="droneConfig.quantity_per_min"
              type="number"
              min="0"
              max="60"
              step="0.1"
              class="form-input"
              @input="updateConfig"
            />
          </div>
        </div>

        <div class="form-row">
          <div class="form-field">
            <label for="drone-id">Drone ID</label>
            <input
              id="drone-id"
              v-model="droneConfig.drone_id"
              type="text"
              class="form-input"
              placeholder="e.g., DRN-001"
              @input="updateConfig"
            />
          </div>

          <div class="form-field">
            <label for="drone-pad">Pad Name</label>
            <input
              id="drone-pad"
              v-model="padName"
              type="text"
              class="form-input"
              placeholder="e.g., Iron Ore Pad"
              @input="updateConfig"
            />
          </div>
        </div>
      </div>

      <div class="drone-info">
        <h4 class="info-title">Drone Information</h4>
        <div class="info-grid">
          <div class="info-item">
            <span class="info-label">Drone ID:</span>
            <span class="info-value">{{ droneConfig.drone_id }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import type { DroneConfig } from '@/api/logistics-types'
import type { Item } from '@/api/types'
import { useItemIcon } from '@/composables/useItemIcon'

interface Props {
  modelValue: DroneConfig
}

interface Emits {
  'update:modelValue': [value: DroneConfig]
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const droneConfig = ref<DroneConfig>({
  ...props.modelValue,
})

const padName = ref('')

// Sample items - in a real app, these would come from the API
const allItems: Item[] = [
  'IronOre', 'IronIngot', 'IronPlate', 'IronRod', 'Screw',
  'CopperOre', 'CopperIngot', 'CopperSheet', 'Wire', 'Cable',
  'Coal', 'Biomass', 'Concrete', 'Limestone',
  'SteelBeam', 'SteelPipe', 'ModularFrame', 'Rotor', 'Stator',
  'Water', 'CrudeOil', 'HeavyOilResidue', 'Fuel', 'Turbofuel',
  'LiquidBiofuel', 'NitrogenGas', 'PackagedWater', 'PackagedOil'
]

// Drone specifications
const DRONE_MAX_CAPACITY = 60 // items per minute
const DRONE_RANGE = 500 // meters
const DRONE_SPEED = 65 // km/h
const DRONE_BATTERY_LIFE = 10 // minutes
const DRONE_ITEMS_PER_TRIP = 5 // items per trip

const { formatItemName } = useItemIcon()

const updateConfig = () => {
  emit('update:modelValue', { ...droneConfig.value })
}

// Watch for prop changes
watch(() => props.modelValue, (newValue) => {
  droneConfig.value = { ...newValue }
}, { deep: true })

// Initialize drone ID if not provided
if (!droneConfig.value.drone_id) {
  droneConfig.value.drone_id = 'DRN-001'
  updateConfig()
}
</script>

<style scoped lang="scss">
.drone-editor {
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

.drone-form {
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
    color: var(--color-text-secondary, #b8b8b8);
  }
}

.form-input,
.form-select {
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  border: 1px solid var(--color-border, #404040);
  border-radius: var(--border-radius-sm, 0.25rem);
  font-size: var(--font-size-base, 1rem);
  background-color: var(--color-surface-inset, #1f1f1f);
  transition: border-color 0.2s ease-in-out;

  &:focus {
    outline: none;
    border-color: var(--color-ficsit-orange, #f58b00);
    box-shadow: 0 0 0 3px rgba(245, 139, 0, 0.1);
  }
}

.drone-info {
  background-color: var(--color-surface-inset, #1f1f1f);
  border: 1px solid var(--color-border, #404040);
  border-radius: var(--border-radius-sm, 0.25rem);
  padding: var(--spacing-md, 0.75rem);
}

.info-title {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-text-primary, #e5e5e5);
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

  .performance-item {
    flex-wrap: wrap;
  }

  .performance-label {
    min-width: auto;
    width: 100%;
  }
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'LogisticsDroneEditor'
}
</script>
