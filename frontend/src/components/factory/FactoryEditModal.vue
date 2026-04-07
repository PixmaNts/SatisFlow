<template>
  <Modal
    :show="show"
    title="Edit Factory"
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
  margin-bottom: var(--spacing-md, 0.75rem);
}

.form-label {
  display: block;
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-secondary, #b8b8b8);
  margin-bottom: var(--spacing-xs, 0.25rem);
}

.form-input,
.form-textarea {
  width: 100%;
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  border: 1px solid var(--color-border, #404040);
  border-radius: var(--border-radius-sm, 3px);
  font-size: var(--font-size-base, 1rem);
  background-color: var(--color-surface, #252525);
  color: var(--color-text-primary, #e5e5e5);
  transition: border-color 0.2s ease-in-out;

  &:focus {
    outline: none;
    border-color: var(--color-ficsit-orange, #f58b00);
    box-shadow: 0 0 0 3px rgba(245, 139, 0, 0.1);
  }

  &::placeholder {
    color: var(--color-text-muted, #8a8a8a);
  }
}

.form-textarea {
  resize: vertical;
  min-height: 60px;
}

.form-actions {
  display: flex;
  gap: var(--spacing-sm, 0.5rem);
  justify-content: flex-end;
  margin-top: var(--spacing-lg, 1rem);
  padding-top: var(--spacing-lg, 1rem);
  border-top: 1px solid var(--color-border, #404040);
}

// Responsive design
@media (max-width: 640px) {
  .form-actions {
    flex-direction: column;
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