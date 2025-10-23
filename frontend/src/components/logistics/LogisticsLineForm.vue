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
    const apiRequest = convertToApiRequest(formData.value)

    if (isEditing.value && props.logisticsLine) {
      const updatedLogistics = await logisticsStore.update(props.logisticsLine.id, apiRequest)
      if (updatedLogistics) {
        emit('updated', updatedLogistics)
      }
    } else {
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
</script>
