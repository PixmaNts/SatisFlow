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
            <span class="info-label">Capacity:</span>
            <span class="info-value">{{ getTruckCapacity() }} items/min</span>
          </div>
          <div class="info-item">
            <span class="info-label">Fuel Type:</span>
            <span class="info-value">{{ getFuelType() }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Range:</span>
            <span class="info-value">{{ getTruckRange() }}m</span>
          </div>
          <div class="info-item">
            <span class="info-label">Speed:</span>
            <span class="info-value">{{ getTruckSpeed() }} km/h</span>
          </div>
        </div>
      </div>
    </div>

    <div class="efficiency-section">
      <h4 class="efficiency-title">Efficiency Analysis</h4>
      <div class="efficiency-stats">
        <div class="efficiency-item">
          <div class="efficiency-label">Utilization</div>
          <div class="efficiency-bar">
            <div
              class="efficiency-fill"
              :style="{ width: `${getUtilization()}%` }"
              :class="getUtilizationClass()"
            ></div>
          </div>
          <div class="efficiency-value">{{ getUtilization() }}%</div>
        </div>

        <div class="efficiency-item">
          <div class="efficiency-label">Trips per Hour</div>
          <div class="efficiency-value">{{ getTripsPerHour() }}</div>
        </div>

        <div class="efficiency-item">
          <div class="efficiency-label">Fuel Consumption</div>
          <div class="efficiency-value">{{ getFuelConsumption() }} units/hr</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import type { TruckConfig } from '@/api/logistics-types'
import type { Item } from '@/api/types'

interface Props {
  modelValue: TruckConfig
}

interface Emits {
  'update:modelValue': [value: TruckConfig]
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const truckConfig = ref<TruckConfig>({
  transport_type: 'Truck',
  item: '' as Item,
  quantity_per_min: 0,
  truck_id: 'TRK-001'
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

const formatItemName = (item: Item): string => {
  return item.replace(/([A-Z])/g, ' $1').trim()
}

// Truck specifications (simplified for demo)
const TRUCK_MAX_CAPACITY = 120 // items per minute
const TRUCK_RANGE = 500 // meters
const TRUCK_SPEED = 45 // km/h
const FUEL_CONSUMPTION_RATE = 2.5 // units per hour at full capacity

const getTruckCapacity = (): number => {
  return Math.min(truckConfig.value.quantity_per_min, TRUCK_MAX_CAPACITY)
}

const getFuelType = (): string => {
  return truckConfig.value.quantity_per_min > 80 ? 'Turbofuel' : 'Fuel'
}

const getTruckRange = (): number => {
  return TRUCK_RANGE
}

const getTruckSpeed = (): number => {
  return TRUCK_SPEED
}

const getUtilization = (): number => {
  if (truckConfig.value.quantity_per_min === 0) return 0
  return Math.min(Math.round((truckConfig.value.quantity_per_min / TRUCK_MAX_CAPACITY) * 100), 100)
}

const getUtilizationClass = (): string => {
  const utilization = getUtilization()
  if (utilization < 50) return 'efficiency-low'
  if (utilization < 80) return 'efficiency-medium'
  return 'efficiency-high'
}

const getTripsPerHour = (): number => {
  if (truckConfig.value.quantity_per_min === 0) return 0
  const itemsPerTrip = 480 // Assuming 8 items per trip, 60 trips per hour
  return Math.ceil(truckConfig.value.quantity_per_min * 60 / itemsPerTrip)
}

const getFuelConsumption = (): number => {
  const utilization = getUtilization() / 100
  return (FUEL_CONSUMPTION_RATE * utilization).toFixed(1) as unknown as number
}

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

.truck-info {
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

.efficiency-section {
  background-color: var(--color-green-50, #f0fdf4);
  border: 1px solid var(--color-green-200, #bbf7d0);
  border-radius: var(--border-radius-md, 0.375rem);
  padding: var(--spacing-md, 0.75rem);
}

.efficiency-title {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-green-900, #14532d);
  margin: 0 0 var(--spacing-sm, 0.5rem) 0;
}

.efficiency-stats {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm, 0.5rem);
}

.efficiency-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 0.5rem);
  padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-sm, 0.25rem);
  border: 1px solid var(--color-green-100, #dcfce7);
}

.efficiency-label {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-green-700, #15803d);
  font-weight: var(--font-weight-medium, 500);
  min-width: 120px;
}

.efficiency-bar {
  flex: 1;
  height: 8px;
  background-color: var(--color-gray-200, #e5e7eb);
  border-radius: var(--border-radius-full, 9999px);
  overflow: hidden;
}

.efficiency-fill {
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

.efficiency-value {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-green-900, #14532d);
  font-weight: var(--font-weight-semibold, 600);
  min-width: 60px;
  text-align: right;
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
