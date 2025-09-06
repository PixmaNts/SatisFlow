<template>
  <div v-if="isOpen" class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>Add New Factory</h3>
        <button @click="close" class="close-btn">&times;</button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="modal-body">
        <div class="form-group">
          <label for="factoryName">Factory Name</label>
          <input
            id="factoryName"
            v-model="factoryName"
            type="text"
            placeholder="Enter factory name..."
            required
            :class="{ error: nameError }"
          />
          <div v-if="nameError" class="error-message">{{ nameError }}</div>
        </div>

        <div class="form-actions">
          <button type="button" @click="close" class="btn btn-secondary">
            Cancel
          </button>
          <button type="submit" class="btn btn-primary" :disabled="isLoading">
            {{ isLoading ? 'Creating...' : 'Create Factory' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { createFactory } from '../lib/tracker'

interface Props {
  isOpen: boolean
}

interface Emits {
  (e: 'close'): void
  (e: 'factory-created', factory: any): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const factoryName = ref('')
const nameError = ref('')
const isLoading = ref(false)

// Reset form when modal opens
watch(() => props.isOpen, (isOpen) => {
  if (isOpen) {
    factoryName.value = ''
    nameError.value = ''
    isLoading.value = false
  }
})

const close = () => {
  emit('close')
}

const handleOverlayClick = () => {
  close()
}

const handleSubmit = async () => {
  if (!factoryName.value.trim()) {
    nameError.value = 'Factory name is required'
    return
  }

  if (factoryName.value.trim().length < 2) {
    nameError.value = 'Factory name must be at least 2 characters'
    return
  }

  isLoading.value = true
  nameError.value = ''

  try {
    const factory = await createFactory(factoryName.value.trim())
    emit('factory-created', factory)
    close()
  } catch (error: any) {
    nameError.value = error.message || 'Failed to create factory'
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
  max-width: 500px;
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

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #374151;
}

.form-group input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 1rem;
}

.form-group input:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.form-group input.error {
  border-color: #ef4444;
}

.error-message {
  color: #ef4444;
  font-size: 0.875rem;
  margin-top: 0.25rem;
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