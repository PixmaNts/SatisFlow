<template>
  <div v-if="isOpen" class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>Edit Logistics Connection</h3>
        <button @click="close" class="close-btn">&times;</button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="modal-body">
        <div class="form-row">
          <div class="form-group">
            <label for="fromFactory">From Factory</label>
            <input
              id="fromFactory"
              :value="fromFactoryName"
              type="text"
              readonly
              class="readonly-input"
            />
            <small class="help-text">Source factory cannot be changed. Create a new connection for different routing.</small>
          </div>

          <div class="form-group">
            <label for="toFactory">To Factory</label>
            <input
              id="toFactory"
              :value="toFactoryName"
              type="text"
              readonly
              class="readonly-input"
            />
            <small class="help-text">Destination factory cannot be changed.</small>
          </div>
        </div>

        <div class="form-group">
          <label for="item">Item</label>
          <input
            id="item"
            :value="itemName"
            type="text"
            readonly
            class="readonly-input"
          />
          <small class="help-text">Item type cannot be changed. Adjust quantity and transport method only.</small>
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
            <input
              id="transportType"
              :value="transportType"
              type="text"
              readonly
              class="readonly-input"
            />
            <small class="help-text">Transport type cannot be changed as it affects the connection ID.</small>
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

        <div v-if="logisticsFlux" class="connection-info">
          <div class="info-card">
            <h4>{{ fromFactoryName }} → {{ toFactoryName }}</h4>
            <div class="connection-details">
              <div class="detail-item">
                <strong>Item:</strong> {{ itemName }}
              </div>
              <div class="detail-item">
                <strong>Current Flow:</strong> {{ quantity }}/min via {{ transportType }}
              </div>
              <div class="detail-item" v-if="transportDetails">
                <strong>Details:</strong> {{ transportDetails }}
              </div>
            </div>
          </div>
        </div>

        <div v-if="error" class="error-message">{{ error }}</div>

        <div class="form-actions">
          <button type="button" @click="close" class="btn btn-secondary">
            Cancel
          </button>
          <button type="submit" class="btn btn-primary" :disabled="isLoading">
            {{ isLoading ? 'Updating...' : 'Update Connection' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { updateLogisticsFlux } from '../lib/tracker'

interface Props {
  isOpen: boolean
  logisticsFlux: any | null
  fromFactoryName: string
  toFactoryName: string
  itemName: string
}

interface Emits {
  (e: 'close'): void
  (e: 'logistics-flux-updated'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const quantity = ref<number>(0)
const transportType = ref('Conveyor')
const transportDetails = ref('')
const error = ref('')
const isLoading = ref(false)

// Reset form when modal opens or logisticsFlux changes
watch([() => props.isOpen, () => props.logisticsFlux], ([isOpen, flux]) => {
  if (isOpen && flux) {
    quantity.value = flux.quantity_per_min || 0
    transportType.value = flux.transport_type || 'Conveyor'
    transportDetails.value = flux.transport_details || ''
    error.value = ''
    isLoading.value = false
  }
})

const getTransportPlaceholder = () => {
  switch (transportType.value) {
    case 'Train':
      return 'e.g., Main Line Express, Freight Route A'
    case 'Truck':
      return 'e.g., Highway Route 1, Local Delivery'
    case 'Drone':
      return 'e.g., Drone Port Alpha to Beta'
    default:
      return ''
  }
}

const getTransportHelpText = () => {
  switch (transportType.value) {
    case 'Train':
      return 'Specify train route name or line identifier'
    case 'Truck':
      return 'Specify truck route or station names'
    case 'Drone':
      return 'Specify drone port connection details'
    default:
      return ''
  }
}

const close = () => {
  emit('close')
}

const handleOverlayClick = () => {
  close()
}

const handleSubmit = async () => {
  if (!props.logisticsFlux) {
    error.value = 'No logistics connection to update'
    return
  }

  if (quantity.value <= 0) {
    error.value = 'Quantity must be greater than 0'
    return
  }

  isLoading.value = true
  error.value = ''

  try {
    const logisticsData = {
      id: props.logisticsFlux.id,
      from_factory: props.logisticsFlux.from_factory,
      to_factory: props.logisticsFlux.to_factory,
      item: props.logisticsFlux.item,
      quantity_per_min: quantity.value,
      transport_type: transportType.value,
      transport_details: transportDetails.value || '',
    }

    await updateLogisticsFlux(props.logisticsFlux.id, logisticsData)
    emit('logistics-flux-updated')
    close()
  } catch (err: any) {
    error.value = err.message || 'Failed to update logistics connection'
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

.readonly-input {
  background-color: #f9fafb;
  color: #6b7280;
  cursor: not-allowed;
}

.help-text {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.875rem;
  color: #6b7280;
}

.connection-info {
  margin-bottom: 1.5rem;
}

.info-card {
  background: #f0f9ff;
  border: 1px solid #bae6fd;
  border-left: 4px solid #0ea5e9;
  border-radius: 6px;
  padding: 1rem;
}

.info-card h4 {
  margin: 0 0 0.75rem 0;
  color: #0f172a;
  font-size: 1.1rem;
}

.connection-details {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.detail-item {
  font-size: 0.9rem;
  color: #334155;
}

.detail-item strong {
  color: #0f172a;
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