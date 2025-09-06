<template>
  <div v-if="isOpen" class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>Add Logistics Connection</h3>
        <button @click="close" class="close-btn">&times;</button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="modal-body">
        <div class="form-row">
          <div class="form-group">
            <label for="fromFactory">From Factory</label>
            <select id="fromFactory" v-model="fromFactoryId" required>
              <option value="">Select source factory...</option>
              <option v-for="factory in factories" :key="factory.id" :value="factory.id">
                {{ factory.name }}
              </option>
            </select>
          </div>

          <div class="form-group">
            <label for="toFactory">To Factory</label>
            <select id="toFactory" v-model="toFactoryId" required>
              <option value="">Select destination factory...</option>
              <option 
                v-for="factory in factories" 
                :key="factory.id" 
                :value="factory.id"
                :disabled="factory.id === fromFactoryId"
              >
                {{ factory.name }}
              </option>
            </select>
          </div>
        </div>

        <div class="form-group">
          <label for="item">Item</label>
          <select id="item" v-model="selectedItem" required>
            <option value="">Select item to transport...</option>
            <option v-for="item in items" :key="item.name" :value="item">
              {{ item.name }}
            </option>
          </select>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="quantity">Quantity per Minute</label>
            <input
              id="quantity"
              v-model.number="quantity"
              type="number"
              min="0.1"
              step="0.1"
              required
            />
          </div>

          <div class="form-group">
            <label for="transportType">Transport Type</label>
            <select id="transportType" v-model="transportType" required>
              <option value="Conveyor">Conveyor</option>
              <option value="Train">Train</option>
              <option value="Truck">Truck</option>
              <option value="Drone">Drone</option>
            </select>
          </div>
        </div>

        <div class="form-group" v-if="transportType && transportType !== 'Conveyor'">
          <label for="transportDetails">Transport Details</label>
          <input
            id="transportDetails"
            v-model="transportDetails"
            type="text"
            :placeholder="getTransportPlaceholder()"
          />
          <div class="help-text">{{ getTransportHelpText() }}</div>
        </div>

        <div v-if="error" class="error-message">{{ error }}</div>

        <div class="form-actions">
          <button type="button" @click="close" class="btn btn-secondary">
            Cancel
          </button>
          <button type="submit" class="btn btn-primary" :disabled="isLoading">
            {{ isLoading ? 'Adding...' : 'Add Logistics Connection' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { getFactories, getItems, addLogisticsFlux, generateLogisticsId } from '../lib/tracker'

interface Props {
  isOpen: boolean
}

interface Emits {
  (e: 'close'): void
  (e: 'logistics-added'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const factories = ref<any[]>([])
const items = ref<any[]>([])
const fromFactoryId = ref('')
const toFactoryId = ref('')
const selectedItem = ref<any>(null)
const quantity = ref<number>(0)
const transportType = ref('Conveyor')
const transportDetails = ref('')
const error = ref('')
const isLoading = ref(false)

// Load data on mount
onMounted(async () => {
  try {
    factories.value = await getFactories()
    items.value = await getItems()
  } catch (err: any) {
    error.value = 'Failed to load data: ' + err.message
  }
})

// Reset form when modal opens and refresh factories list
watch(() => props.isOpen, async (isOpen) => {
  if (isOpen) {
    fromFactoryId.value = ''
    toFactoryId.value = ''
    selectedItem.value = null
    quantity.value = 0
    transportType.value = 'Conveyor'
    transportDetails.value = ''
    error.value = ''
    isLoading.value = false
    
    // Refresh factories list when modal opens
    try {
      factories.value = await getFactories()
    } catch (err: any) {
      error.value = 'Failed to refresh factories: ' + err.message
    }
  }
})

// Clear transport details when type changes
watch(transportType, () => {
  transportDetails.value = ''
})

const getTransportPlaceholder = () => {
  switch (transportType.value) {
    case 'Train': return 'W01 (Wagon number)'
    case 'Truck': return 'Route name or number'
    case 'Drone': return 'Route name or number'
    default: return ''
  }
}

const getTransportHelpText = () => {
  switch (transportType.value) {
    case 'Train': return 'Enter wagon designation (W01, W02, etc.)'
    case 'Truck': return 'Optional: route identifier for organization'
    case 'Drone': return 'Optional: route identifier for organization'
    default: return ''
  }
}

const close = () => {
  emit('close')
}

const handleOverlayClick = () => {
  close()
}

const handleSubmit = async () => {
  if (!fromFactoryId.value || !toFactoryId.value) {
    error.value = 'Please select both source and destination factories'
    return
  }

  if (fromFactoryId.value === toFactoryId.value) {
    error.value = 'Source and destination factories must be different'
    return
  }

  if (!selectedItem.value) {
    error.value = 'Please select an item to transport'
    return
  }

  if (quantity.value <= 0) {
    error.value = 'Quantity must be greater than 0'
    return
  }

  isLoading.value = true
  error.value = ''

  try {
    // Generate a unique logistics ID
    const logisticsId = await generateLogisticsId(transportType.value)
    
    const logisticsFlux = {
      id: logisticsId,
      from_factory: fromFactoryId.value,
      to_factory: toFactoryId.value,
      item: selectedItem.value.id,
      quantity_per_min: quantity.value,
      transport_type: transportType.value,
      transport_details: transportDetails.value || '',
    }

    await addLogisticsFlux(logisticsFlux)
    emit('logistics-added')
    close()
  } catch (err: any) {
    error.value = err.message || 'Failed to add logistics connection'
  } finally {
    isLoading.value = false
  }
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background: white;
  border-radius: 8px;
  width: 90%;
  max-width: 700px;
  max-height: 80vh;
  overflow-y: auto;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #e5e7eb;
}

.modal-header h3 {
  margin: 0;
  color: #1f2937;
  font-size: 1.25rem;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #6b7280;
  padding: 0;
  width: 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-btn:hover {
  color: #374151;
}

.modal-body {
  padding: 1.5rem;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #374151;
}

.form-group input,
.form-group select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 1rem;
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.form-group select option:disabled {
  color: #9ca3af;
}

.help-text {
  font-size: 0.875rem;
  color: #6b7280;
  margin-top: 0.25rem;
}

.error-message {
  color: #ef4444;
  font-size: 0.875rem;
  margin-bottom: 1rem;
  padding: 0.5rem;
  background-color: #fef2f2;
  border-radius: 4px;
  border: 1px solid #fecaca;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
}

.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 6px;
  font-size: 1rem;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background-color: #3b82f6;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background-color: #2563eb;
}

.btn-secondary {
  background-color: #f3f4f6;
  color: #374151;
}

.btn-secondary:hover {
  background-color: #e5e7eb;
}
</style>