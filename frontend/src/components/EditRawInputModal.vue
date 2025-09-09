<template>
  <div v-if="isOpen" class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>Edit Raw Material Input</h3>
        <button @click="close" class="close-btn">&times;</button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="modal-body">
        <div class="form-group">
          <label for="item">Raw Material</label>
          <input 
            id="item" 
            :value="itemName" 
            type="text" 
            readonly 
            class="readonly-input"
          />
          <small class="help-text">Material cannot be changed. Create a new input for different materials.</small>
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
            <label for="sourceType">Source Type</label>
            <select id="sourceType" v-model="sourceType" required>
              <option value="Miner Mk.1">Miner Mk.1</option>
              <option value="Miner Mk.2">Miner Mk.2</option>
              <option value="Miner Mk.3">Miner Mk.3</option>
              <option value="Oil Extractor">Oil Extractor</option>
              <option value="Water Extractor">Water Extractor</option>
              <option value="Resource Well Extractor">Resource Well Extractor</option>
              <option value="Fracking">Fracking</option>
              <option value="Manual">Manual</option>
              <option value="Other">Other</option>
            </select>
          </div>
        </div>

        <div v-if="rawInput" class="material-info">
          <div class="info-card">
            <h4>{{ itemName }}</h4>
            <p class="material-description">{{ getMaterialDescription(itemName) }}</p>
            <div class="suggested-sources">
              <strong>Recommended Sources:</strong>
              <span class="source-tags">
                <span 
                  v-for="source in getSuggestedSources(itemName)"
                  :key="source"
                  :class="['source-tag', { recommended: source === sourceType }]"
                >
                  {{ source }}
                </span>
              </span>
            </div>
          </div>
        </div>

        <div class="form-group">
          <label for="comment">Comment (optional)</label>
          <textarea
            id="comment"
            v-model="comment"
            placeholder="e.g., Three nodes clustered near main base..."
            rows="2"
          ></textarea>
        </div>

        <div v-if="error" class="error-message">{{ error }}</div>

        <div class="form-actions">
          <button type="button" @click="close" class="btn btn-secondary">
            Cancel
          </button>
          <button type="submit" class="btn btn-primary" :disabled="isLoading">
            {{ isLoading ? 'Updating...' : 'Update Raw Input' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { updateRawInput } from '../lib/tracker'

interface Props {
  isOpen: boolean
  factoryId: string
  rawInput: any | null
  itemName: string
}

interface Emits {
  (e: 'close'): void
  (e: 'raw-input-updated'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const quantity = ref<number>(0)
const sourceType = ref('Miner Mk.1')
const comment = ref('')
const error = ref('')
const isLoading = ref(false)

// Reset form when modal opens or rawInput changes
watch([() => props.isOpen, () => props.rawInput], ([isOpen, rawInput]) => {
  if (isOpen && rawInput) {
    quantity.value = rawInput.quantity_per_min || 0
    sourceType.value = rawInput.source_type || 'Miner Mk.1'
    comment.value = rawInput.comment || ''
    error.value = ''
    isLoading.value = false
  }
})

const getMaterialDescription = (itemName: string) => {
  const descriptions = {
    'Iron Ore': 'Basic ore used for iron production. Found in large deposits.',
    'Copper Ore': 'Essential for electrical components. Common near water sources.',
    'Limestone': 'Used for concrete and steel production. Abundant resource.',
    'Coal': 'Energy source and steel production component.',
    'Caterium Ore': 'Advanced material for quantum technology.',
    'Raw Quartz': 'Used for crystal oscillators and optics.',
    'Sulfur': 'Chemical component for advanced manufacturing.',
    'Bauxite': 'Primary source of aluminum.',
    'Uranium': 'Nuclear power and weapons production.',
    'Water': 'Essential for many industrial processes.',
    'Crude Oil': 'Base for petroleum products and plastics.',
    'Nitrogen Gas': 'Inert gas for various industrial processes.',
    'SAM': 'Strange Alien Material with unique properties.'
  }
  return descriptions[itemName] || 'Raw material used in production.'
}

const getSuggestedSources = (itemName: string) => {
  const sources = {
    'Iron Ore': ['Miner Mk.1', 'Miner Mk.2', 'Miner Mk.3'],
    'Copper Ore': ['Miner Mk.1', 'Miner Mk.2', 'Miner Mk.3'],
    'Limestone': ['Miner Mk.1', 'Miner Mk.2', 'Miner Mk.3'],
    'Coal': ['Miner Mk.1', 'Miner Mk.2', 'Miner Mk.3'],
    'Caterium Ore': ['Miner Mk.2', 'Miner Mk.3'],
    'Raw Quartz': ['Miner Mk.1', 'Miner Mk.2', 'Miner Mk.3'],
    'Sulfur': ['Miner Mk.1', 'Miner Mk.2', 'Miner Mk.3'],
    'Bauxite': ['Miner Mk.1', 'Miner Mk.2', 'Miner Mk.3'],
    'Uranium': ['Miner Mk.3'],
    'Water': ['Water Extractor'],
    'Crude Oil': ['Oil Extractor'],
    'Nitrogen Gas': ['Resource Well Extractor', 'Fracking'],
    'SAM': ['Miner Mk.1', 'Miner Mk.2', 'Miner Mk.3']
  }
  return sources[itemName] || ['Manual']
}

const close = () => {
  emit('close')
}

const handleOverlayClick = () => {
  close()
}

const handleSubmit = async () => {
  if (!props.factoryId) {
    error.value = 'No factory selected'
    return
  }

  if (!props.rawInput) {
    error.value = 'No raw input to update'
    return
  }

  if (quantity.value <= 0) {
    error.value = 'Quantity must be greater than 0'
    return
  }

  isLoading.value = true
  error.value = ''

  try {
    const rawInputData = {
      id: props.rawInput.id,
      item: props.rawInput.item,
      quantity_per_min: quantity.value,
      source_type: sourceType.value,
      comment: comment.value.trim() || null,
    }

    await updateRawInput(props.factoryId, props.rawInput.id, rawInputData)
    emit('raw-input-updated')
    close()
  } catch (err: any) {
    error.value = err.message || 'Failed to update raw input'
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
  max-width: 600px;
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
.form-group select,
.form-group textarea {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 1rem;
  font-family: inherit;
}

.form-group input:focus,
.form-group select:focus,
.form-group textarea:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.form-group textarea {
  resize: vertical;
  min-height: 3rem;
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

.material-info {
  margin-bottom: 1.5rem;
}

.info-card {
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 6px;
  padding: 1rem;
}

.info-card h4 {
  margin: 0 0 0.5rem 0;
  color: #2c3e50;
  font-size: 1.1rem;
}

.material-description {
  color: #666;
  margin-bottom: 1rem;
  font-size: 0.9rem;
  line-height: 1.4;
}

.suggested-sources {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.source-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.source-tag {
  background: #e0f2fe;
  color: #0277bd;
  padding: 0.25rem 0.5rem;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: 500;
}

.source-tag.recommended {
  background: #4caf50;
  color: white;
  font-weight: 600;
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