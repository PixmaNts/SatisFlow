<template>
  <Modal
    :show="show"
    title="Edit Factory"
    size="lg"
    @close="handleClose"
  >
    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="factory-name" class="form-label">Factory Name *</label>
        <input
          id="factory-name"
          v-model="formData.name"
          type="text"
          class="form-input"
          placeholder="Enter factory name..."
          required
        />
      </div>

      <div class="form-group">
        <label for="factory-description" class="form-label">Description</label>
        <textarea
          id="factory-description"
          v-model="formData.description"
          class="form-textarea"
          placeholder="Enter factory description..."
          rows="3"
        />
      </div>

      <div class="form-group">
        <label for="factory-notes" class="form-label">Notes</label>
        <textarea
          id="factory-notes"
          v-model="formData.notes"
          class="form-textarea"
          placeholder="Enter additional notes..."
          rows="3"
        />
      </div>

      <div class="form-actions">
        <Button
          type="button"
          variant="secondary"
          @click="handleClose"
        >
          Cancel
        </Button>
        <Button
          type="submit"
          variant="primary"
          :loading="saving"
          :disabled="!canSubmit"
        >
          Save Changes
        </Button>
      </div>
    </form>
  </Modal>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useFactoryStore } from '@/stores/factory'
import { useToast } from '@/composables/useToast'
import type { FactoryResponse, UpdateFactoryRequest } from '@/api/types'
import Button from '@/components/ui/Button.vue'
import Modal from '@/components/ui/Modal.vue'

interface Props {
  show: boolean
  factory: FactoryResponse | null
}

interface Emits {
  (e: 'update:show', value: boolean): void
  (e: 'saved'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const factoryStore = useFactoryStore()
const { showSuccess, showError } = useToast()

// State
const saving = ref(false)
const formData = ref({
  name: '',
  description: '',
  notes: ''
})

// Computed
const canSubmit = computed(() => {
  return formData.value.name.trim().length > 0
})

// Methods
const handleClose = () => {
  emit('update:show', false)
  resetForm()
}

const resetForm = () => {
  formData.value = {
    name: '',
    description: '',
    notes: ''
  }
}

const loadFactory = () => {
  if (!props.factory) return

  formData.value = {
    name: props.factory.name,
    description: props.factory.description || '',
    notes: props.factory.notes || ''
  }
}

const handleSubmit = async () => {
  if (!canSubmit.value || !props.factory) return

  saving.value = true

  try {
    const payload: UpdateFactoryRequest = {
      name: formData.value.name.trim(),
      description: formData.value.description?.trim() || undefined,
      notes: formData.value.notes?.trim() || undefined
    }

    const response = await factoryStore.update(props.factory.id, payload)

    if (response) {
      showSuccess('Factory updated successfully')
      emit('saved')
      handleClose()
    } else {
      showError('Failed to update factory')
    }
  } catch (error) {
    console.error('Failed to update factory:', error)
    showError('An error occurred while updating the factory')
  } finally {
    saving.value = false
  }
}

// Watch for factory changes
watch(() => props.factory, () => {
  if (props.show) {
    loadFactory()
  }
}, { immediate: true })

// Watch for show changes
watch(() => props.show, (show) => {
  if (show) {
    loadFactory()
  } else {
    resetForm()
  }
})
</script>

<style scoped lang="scss">
.form-group {
  margin-bottom: var(--spacing-lg);
}

.form-label {
  display: block;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--color-text-secondary);
  margin-bottom: var(--spacing-xs);
}

.form-input,
.form-textarea {
  width: 100%;
  padding: var(--spacing-md) var(--spacing-lg);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-md);
  font-size: var(--font-size-base);
  background-color: var(--color-surface-inset);
  color: var(--color-text-primary);
  transition: all var(--transition-normal);
  font-family: var(--font-family-sans);

  &:focus {
    outline: none;
    border-color: var(--color-ficsit-orange);
    box-shadow: 0 0 0 3px rgba(245, 139, 0, 0.2);
    background-color: var(--color-surface);
  }

  &::placeholder {
    color: var(--color-text-muted);
  }
}

.form-textarea {
  resize: vertical;
  min-height: 100px;
  line-height: 1.5;
}

.form-actions {
  display: flex;
  gap: var(--spacing-md);
  justify-content: flex-end;
  margin-top: var(--spacing-xl);
  padding-top: var(--spacing-lg);
  border-top: 1px solid var(--color-border-dark);
}

// Responsive design
@media (max-width: 640px) {
  .form-actions {
    flex-direction: column;
    gap: var(--spacing-md);
  }

  .form-actions button {
    width: 100%;
  }
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'FactoryEditModal'
}
</script>