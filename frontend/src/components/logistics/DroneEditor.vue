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
        <h4 class="info-title">Drone Specifications</h4>
        <div class="info-grid">
          <div class="info-item">
            <span class="info-label">Max Capacity:</span>
            <span class="info-value">{{ DRONE_MAX_CAPACITY }} items/min</span>
          </div>
          <div class="info-item">
            <span class="info-label">Max Range:</span>
            <span class="info-value">{{ DRONE_RANGE }}m</span>
          </div>
          <div class="info-item">
            <span class="info-label">Speed:</span>
            <span class="info-value">{{ DRONE_SPEED }} km/h</span>
          </div>
          <div class="info-item">
            <span class="info-label">Battery Life:</span>
            <span class="info-value">{{ DRONE_BATTERY_LIFE }} min</span>
          </div>
        </div>
      </div>
    </div>

    <div class="performance-section">
      <h4 class="performance-title">Performance Metrics</h4>
      <div class="performance-stats">
        <div class="performance-item">
          <div class="performance-label">Efficiency</div>
          <div class="performance-bar">
            <div
              class="performance-fill"
              :style="{ width: `${getEfficiency()}%` }"
              :class="getEfficiencyClass()"
            ></div>
          </div>
          <div class="performance-value">{{ getEfficiency() }}%</div>
        </div>

        <div class="performance-item">
          <div class="performance-label">Trips per Hour</div>
          <div class="performance-value">{{ getTripsPerHour() }}</div>
        </div>

        <div class="performance-item">
          <div class="performance-label">Battery Usage</div>
          <div class="performance-value">{{ getBatteryUsage() }}%/hour</div>
        </div>

        <div class="performance-item">
          <div class="performance-label">Range Utilization</div>
          <div class="performance-value">{{ getRangeUtilization() }}%</div>
        </div>
      </div>
    </div>

    <div class="warnings-section" v-if="getWarnings().length > 0">
      <h4 class="warnings-title">Performance Warnings</h4>
      <div class="warnings-list">
        <div
          v-for="(warning, index) in getWarnings()"
          :key="index"
          class="warning-item"
        >
          <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" class="warning-icon">
            <path d="M8 2a6 6 0 100 12A6 6 0 008 2zm0 9a1 1 0 110-2 1 1 0 010 2zm0-4a1 1 0 011 1v2a1 1 0 11-2 0V8a1 1 0 011-1z"/>
          </svg>
          {{ warning }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import type { DroneConfig } from '@/api/logistics-types'
import type { Item } from '@/api/types'

interface Props {
  modelValue: DroneConfig
}

interface Emits {
  'update:modelValue': [value: DroneConfig]
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const droneConfig = ref<DroneConfig>({
  transport_type: 'Drone',
  item: '' as Item,
  quantity_per_min: 0,
  drone_id: 'DRN-001'
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

const formatItemName = (item: Item): string => {
  return item.replace(/([A-Z])/g, ' $1').trim()
}

const getEfficiency = (): number => {
  if (droneConfig.value.quantity_per_min === 0) return 0
  return Math.min(Math.round((droneConfig.value.quantity_per_min / DRONE_MAX_CAPACITY) * 100), 100)
}

const getEfficiencyClass = (): string => {
  const efficiency = getEfficiency()
  if (efficiency < 50) return 'efficiency-low'
  if (efficiency < 80) return 'efficiency-medium'
  return 'efficiency-high'
}

const getTripsPerHour = (): number => {
  if (droneConfig.value.quantity_per_min === 0) return 0
  const tripsNeeded = Math.ceil(droneConfig.value.quantity_per_min * 60 / DRONE_ITEMS_PER_TRIP)
  return Math.min(tripsNeeded, Math.floor(DRONE_BATTERY_LIFE * 60 / 5)) // 5 minutes per trip cycle
}

const getBatteryUsage = (): number => {
  const tripsPerHour = getTripsPerHour()
  const batteryPerTrip = 100 / (DRONE_BATTERY_LIFE * 12) // 5 minutes per trip, 12 trips per battery
  return (tripsPerHour * batteryPerTrip).toFixed(1) as unknown as number
}

const getRangeUtilization = (): number => {
  // Assuming average trip distance of 300m for calculation
  const averageDistance = 300
  return Math.round((averageDistance / DRONE_RANGE) * 100)
}

const getWarnings = (): string[] => {
  const warnings: string[] = []
  const efficiency = getEfficiency()
  const batteryUsage = getBatteryUsage()

  if (efficiency > 90) {
    warnings.push('Drone is operating near maximum capacity')
  }

  if (batteryUsage > 80) {
    warnings.push('High battery consumption - consider multiple drones')
  }

  if (droneConfig.value.quantity_per_min > 45) {
    warnings.push('High throughput may require multiple drones for reliability')
  }

  if (getRangeUtilization() > 80) {
    warnings.push('Operating near maximum range - consider closer placement')
  }

  return warnings
}

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
    color: var(--color-gray-700, #374151);
  }
}

.form-input,
.form-select {
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-sm, 0.25rem);
  font-size: var(--font-size-base, 1rem);
  background-color: var(--color-white, #ffffff);
  transition: border-color 0.2s ease-in-out;

  &:focus {
    outline: none;
    border-color: var(--color-primary-500, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }
}

.drone-info {
  background-color: var(--color-white, #ffffff);
  border: 1px solid var(--color-gray-200, #e5e7eb);
  border-radius: var(--border-radius-sm, 0.25rem);
  padding: var(--spacing-md, 0.75rem);
}

.info-title {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-gray-900, #111827);
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
  background-color: var(--color-gray-50, #f9fafb);
  border-radius: var(--border-radius-sm, 0.25rem);
}

.info-label {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-gray-600, #4b5563);
  font-weight: var(--font-weight-medium, 500);
}

.info-value {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-gray-900, #111827);
  font-weight: var(--font-weight-semibold, 600);
}

.performance-section {
  background-color: var(--color-purple-50, #faf5ff);
  border: 1px solid var(--color-purple-200, #e9d5ff);
  border-radius: var(--border-radius-md, 0.375rem);
  padding: var(--spacing-md, 0.75rem);
}

.performance-title {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-purple-900, #581c87);
  margin: 0 0 var(--spacing-sm, 0.5rem) 0;
}

.performance-stats {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm, 0.5rem);
}

.performance-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 0.5rem);
  padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-sm, 0.25rem);
  border: 1px solid var(--color-purple-100, #f3e8ff);
}

.performance-label {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-purple-700, #7c3aed);
  font-weight: var(--font-weight-medium, 500);
  min-width: 120px;
}

.performance-bar {
  flex: 1;
  height: 8px;
  background-color: var(--color-gray-200, #e5e7eb);
  border-radius: var(--border-radius-full, 9999px);
  overflow: hidden;
}

.performance-fill {
  height: 100%;
  transition: width 0.3s ease-in-out;

  &.efficiency-low {
    background-color: var(--color-red-500, #ef4444);
  }

  &.efficiency-medium {
    background-color: var(--color-yellow-500, #eab308);
  }

  &.efficiency-high {
    background-color: var(--color-green-500, #22c55e);
  }
}

.performance-value {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-purple-900, #581c87);
  font-weight: var(--font-weight-semibold, 600);
  min-width: 60px;
  text-align: right;
}

.warnings-section {
  background-color: var(--color-yellow-50, #fefce8);
  border: 1px solid var(--color-yellow-200, #fde047);
  border-radius: var(--border-radius-md, 0.375rem);
  padding: var(--spacing-md, 0.75rem);
}

.warnings-title {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-yellow-900, #713f12);
  margin: 0 0 var(--spacing-sm, 0.5rem) 0;
}

.warnings-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
}

.warning-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
  padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-sm, 0.25rem);
  border: 1px solid var(--color-yellow-100, #fef3c7);
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-yellow-800, #854d0e);
}

.warning-icon {
  color: var(--color-yellow-600, #ca8a04);
  flex-shrink: 0;
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
