<template>
  <div class="logistics-line-form">
    <div class="form-header">
      <h3 class="form-title">{{ isEditing ? 'Edit Logistics Line' : 'Create Logistics Line' }}</h3>
      <p class="form-description">
        Configure a logistics line to transport items between factories
      </p>
    </div>

    <form @submit.prevent="handleSubmit" class="logistics-form">
      <!-- Factory Selection -->
      <div class="form-section">
        <h4 class="section-title">Factory Connection</h4>
        <div class="factory-selection">
          <div class="form-field">
            <label for="from-factory">Source Factory</label>
            <select
              id="from-factory"
              v-model.number="formData.from_factory"
              class="form-select"
              required
              @change="validateFactories"
            >
              <option value="">Select source factory</option>
              <option
                v-for="factory in factories"
                :key="factory.id"
                :value="factory.id"
                :disabled="factory.id === formData.to_factory"
              >
                {{ factory.name }}
              </option>
            </select>
          </div>

          <div class="form-field">
            <label for="to-factory">Destination Factory</label>
            <select
              id="to-factory"
              v-model.number="formData.to_factory"
              class="form-select"
              required
              @change="validateFactories"
            >
              <option value="">Select destination factory</option>
              <option
                v-for="factory in factories"
                :key="factory.id"
                :value="factory.id"
                :disabled="factory.id === formData.from_factory"
              >
                {{ factory.name }}
              </option>
            </select>
          </div>
        </div>

        <div v-if="factoryError" class="error-message">
          {{ factoryError }}
        </div>
      </div>

      <!-- Transport Type Selection -->
      <div class="form-section">
        <TransportSelector
          v-model="selectedTransportType"
          @update:modelValue="handleTransportTypeChange"
        />
      </div>

      <!-- Transport Configuration -->
      <div v-if="selectedTransportType" class="form-section">
        <h4 class="section-title">Transport Configuration</h4>

        <!-- Bus Editor -->
        <BusEditor
          v-if="selectedTransportType === 'Bus'"
          v-model="transportConfig as BusConfig"
          @update:modelValue="handleTransportConfigChange"
        />

        <!-- Train Editor -->
        <TrainEditor
          v-else-if="selectedTransportType === 'Train'"
          v-model="transportConfig as TrainConfig"
          @update:modelValue="handleTransportConfigChange"
        />

        <!-- Truck Editor -->
        <TruckEditor
          v-else-if="selectedTransportType === 'Truck'"
          v-model="transportConfig as TruckConfig"
          @update:modelValue="handleTransportConfigChange"
        />

        <!-- Drone Editor -->
        <DroneEditor
          v-else-if="selectedTransportType === 'Drone'"
          v-model="transportConfig as DroneConfig"
          @update:modelValue="handleTransportConfigChange"
        />
      </div>

      <!-- Form Actions -->
      <div class="form-actions">
        <Button
          type="button"
          variant="secondary"
          @click="handleCancel"
        >
          Cancel
        </Button>

        <Button
          type="submit"
          variant="primary"
          :loading="loading"
          :disabled="!isFormValid"
        >
          {{ isEditing ? 'Update' : 'Create' }} Logistics Line
        </Button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useFactoryStore } from '@/stores/factory'
import { useLogisticsStore } from '@/stores/logistics'
import Button from '@/components/ui/Button.vue'
import TransportSelector from './TransportSelector.vue'
import BusEditor from './BusEditor.vue'
import TrainEditor from './TrainEditor.vue'
import TruckEditor from './TruckEditor.vue'
import DroneEditor from './DroneEditor.vue'
import type { TransportType } from '@/api/types'
import type {
  TransportConfig,
  BusConfig,
  TrainConfig,
  TruckConfig,
  DroneConfig,
  LogisticsFormData
} from '@/api/logistics-types'
import type { CreateLogisticsRequest, LogisticsResponse, Item } from '@/api/types'

interface Props {
  logisticsLine?: LogisticsResponse | null
  show?: boolean
}

interface Emits {
  close: []
  created: [logistics: LogisticsResponse]
  updated: [logistics: LogisticsResponse]
}

const props = withDefaults(defineProps<Props>(), {
  logisticsLine: null,
  show: true
})

const emit = defineEmits<Emits>()

const factoryStore = useFactoryStore()
const logisticsStore = useLogisticsStore()

// Form state
const loading = ref(false)
const factoryError = ref('')
const selectedTransportType = ref<TransportType | null>(null)
const transportConfig = ref<TransportConfig | null>(null)

// Form data
const formData = ref<LogisticsFormData>({
  from_factory: 0,
  to_factory: 0,
  transport_config: {
    transport_type: 'Truck',
    item: '' as Item,
    quantity_per_min: 0,
    truck_id: 'TRK-001'
  } as TruckConfig
})

// Computed properties
const isEditing = computed(() => !!props.logisticsLine)
const factories = computed(() => factoryStore.factories)

const isFormValid = computed(() => {
  return (
    formData.value.from_factory > 0 &&
    formData.value.to_factory > 0 &&
    formData.value.from_factory !== formData.value.to_factory &&
    selectedTransportType.value !== null &&
    transportConfig.value !== null &&
    isTransportConfigValid()
  )
})

// Methods
const isTransportConfigValid = (): boolean => {
  if (!transportConfig.value) return false

  const config = transportConfig.value

  switch (config.transport_type) {
    case 'Bus':
      const busConfig = config as BusConfig
      return (
        busConfig.conveyors.some(c => c.item && c.quantity_per_min > 0) ||
        busConfig.pipelines.some(p => p.item && p.quantity_per_min > 0)
      )

    case 'Train':
      const trainConfig = config as TrainConfig
      return trainConfig.wagons.some(w => w.item && w.quantity_per_min > 0)

    case 'Truck':
      const truckConfig = config as TruckConfig
      return !!(truckConfig.item && truckConfig.quantity_per_min > 0)

    case 'Drone':
      const droneConfig = config as DroneConfig
      return !!(droneConfig.item && droneConfig.quantity_per_min > 0)

    default:
      return false
  }
}

const validateFactories = () => {
  if (formData.value.from_factory === formData.value.to_factory && formData.value.from_factory > 0) {
    factoryError.value = 'Source and destination factories must be different'
  } else {
    factoryError.value = ''
  }
}

const handleTransportTypeChange = (transportType: TransportType) => {
  selectedTransportType.value = transportType

  // Initialize default transport config based on type
  switch (transportType) {
    case 'Bus':
      transportConfig.value = {
        transport_type: 'Bus',
        conveyors: [],
        pipelines: []
      } as BusConfig
      break

    case 'Train':
      transportConfig.value = {
        transport_type: 'Train',
        wagons: []
      } as TrainConfig
      break

    case 'Truck':
      transportConfig.value = {
        transport_type: 'Truck',
        item: '' as Item,
        quantity_per_min: 0,
        truck_id: 'TRK-001'
      } as TruckConfig
      break

    case 'Drone':
      transportConfig.value = {
        transport_type: 'Drone',
        item: '' as Item,
        quantity_per_min: 0,
        drone_id: 'DRN-001'
      } as DroneConfig
      break
  }

  if (transportConfig.value) {
    formData.value.transport_config = transportConfig.value
  }
}

const handleTransportConfigChange = (config: TransportConfig) => {
  transportConfig.value = config
  formData.value.transport_config = config
}

const handleSubmit = async () => {
  if (!isFormValid.value) return

  loading.value = true
  factoryError.value = ''

  try {
    // Convert transport config to API format
    const apiRequest = convertToApiRequest(formData.value)

    if (isEditing.value && props.logisticsLine) {
      // Update existing logistics line
      // Note: The current API doesn't support updates, so we'll create a new one
      const newLogistics = await logisticsStore.create(apiRequest)
      if (newLogistics) {
        emit('updated', newLogistics)
      }
    } else {
      // Create new logistics line
      const newLogistics = await logisticsStore.create(apiRequest)
      if (newLogistics) {
        emit('created', newLogistics)
      }
    }

    emit('close')
  } catch (error) {
    console.error('Failed to save logistics line:', error)
    factoryError.value = 'Failed to save logistics line. Please try again.'
  } finally {
    loading.value = false
  }
}

const handleCancel = () => {
  emit('close')
}

const convertToApiRequest = (formData: LogisticsFormData): CreateLogisticsRequest => {
  // Convert the complex transport config to the simple API format
  // This is a simplified conversion - in a real implementation, you'd
  // serialize the full transport config to JSON
  const config = formData.transport_config

  let transportDetails = ''

  switch (config.transport_type) {
    case 'Bus':
      const busConfig = config as BusConfig
      transportDetails = JSON.stringify({
        conveyors: busConfig.conveyors,
        pipelines: busConfig.pipelines
      })
      break

    case 'Train':
      const trainConfig = config as TrainConfig
      transportDetails = JSON.stringify({
        wagons: trainConfig.wagons
      })
      break

    case 'Truck':
      const truckConfig = config as TruckConfig
      transportDetails = JSON.stringify({
        item: truckConfig.item,
        quantity_per_min: truckConfig.quantity_per_min,
        truck_id: truckConfig.truck_id
      })
      break

    case 'Drone':
      const droneConfig = config as DroneConfig
      transportDetails = JSON.stringify({
        item: droneConfig.item,
        quantity_per_min: droneConfig.quantity_per_min,
        drone_id: droneConfig.drone_id
      })
      break
  }

  return {
    from_factory: formData.from_factory,
    to_factory: formData.to_factory,
    transport_type: config.transport_type.toLowerCase() as 'truck' | 'drone',
    transport_details: transportDetails
  }
}

const initializeForm = () => {
  if (props.logisticsLine) {
    // Edit mode - populate form with existing data
    formData.value.from_factory = props.logisticsLine.from_factory
    formData.value.to_factory = props.logisticsLine.to_factory
    selectedTransportType.value = props.logisticsLine.transport_type

    // Parse transport details from existing line
    try {
      const transportDetails = JSON.parse(props.logisticsLine.transport_details)
      // Initialize transport config based on parsed data
      handleTransportTypeChange(props.logisticsLine.transport_type)
      if (transportConfig.value) {
        Object.assign(transportConfig.value, transportDetails)
      }
    } catch (error) {
      console.error('Failed to parse transport details:', error)
      handleTransportTypeChange(props.logisticsLine.transport_type)
    }
  } else {
    // Create mode - reset form
    formData.value = {
      from_factory: 0,
      to_factory: 0,
      transport_config: {
        transport_type: 'Truck',
        item: '' as Item,
        quantity_per_min: 0,
        truck_id: 'TRK-001'
      } as TruckConfig
    }
    selectedTransportType.value = null
    transportConfig.value = null
  }
}

// Lifecycle
onMounted(async () => {
  // Fetch factories if not already loaded
  if (factories.value.length === 0) {
    await factoryStore.fetchAll()
  }

  initializeForm()
})

// Watch for logistics line changes
watch(() => props.logisticsLine, initializeForm)
</script>

<style scoped lang="scss">
.logistics-line-form {
  max-width: 800px;
  margin: 0 auto;
}

.form-header {
  text-align: center;
  margin-bottom: var(--spacing-xl, 1.25rem);
}

.form-title {
  font-size: var(--font-size-xl, 1.25rem);
  font-weight: var(--font-weight-bold, 700);
  color: var(--color-gray-900, #111827);
  margin: 0 0 var(--spacing-sm, 0.5rem) 0;
}

.form-description {
  font-size: var(--font-size-base, 1rem);
  color: var(--color-gray-600, #4b5563);
  margin: 0;
}

.logistics-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg, 1rem);
}

.form-section {
  border: 1px solid var(--color-gray-200, #e5e7eb);
  border-radius: var(--border-radius-md, 0.375rem);
  padding: var(--spacing-lg, 1rem);
  background-color: var(--color-white, #ffffff);
}

.section-title {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-gray-900, #111827);
  margin: 0 0 var(--spacing-md, 0.75rem) 0;
}

.factory-selection {
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

  &:disabled {
    background-color: var(--color-gray-100, #f3f4f6);
    color: var(--color-gray-500, #6b7280);
    cursor: not-allowed;
  }
}

.error-message {
  margin-top: var(--spacing-sm, 0.5rem);
  padding: var(--spacing-sm, 0.5rem);
  background-color: var(--color-red-50, #fef2f2);
  border: 1px solid var(--color-red-200, #fecaca);
  border-radius: var(--border-radius-sm, 0.25rem);
  color: var(--color-red-700, #b91c1c);
  font-size: var(--font-size-sm, 0.875rem);
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm, 0.5rem);
  padding: var(--spacing-lg, 1rem) 0;
}

// Responsive design
@media (max-width: 768px) {
  .factory-selection {
    grid-template-columns: 1fr;
  }

  .form-actions {
    flex-direction: column;
  }

  .form-section {
    padding: var(--spacing-md, 0.75rem);
  }
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'LogisticsLineForm'
}
</script>
