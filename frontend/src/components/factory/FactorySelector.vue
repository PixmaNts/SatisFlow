<template>
  <div class="factory-selector">
    <div class="selector-header">
      <h2 class="selector-title">Factory</h2>
      <Button
        variant="primary"
        size="sm"
        @click="showCreateModal = true"
      >
        Create Factory
      </Button>
    </div>

    <div v-if="currentFactory" class="current-factory">
      <div class="factory-info">
        <h3 class="factory-name">{{ currentFactory.name }}</h3>
        <p v-if="currentFactory.description" class="factory-description">
          {{ currentFactory.description }}
        </p>
      </div>
    </div>

    <div class="selector-controls">
      <label for="factory-select" class="selector-label">Select Factory:</label>
      <select
        id="factory-select"
        v-model="selectedFactoryId"
        class="factory-select"
        @change="handleFactoryChange"
        :disabled="factories.length === 0"
      >
        <option
          v-for="factory in factories"
          :key="factory.id"
          :value="factory.id"
        >
          {{ factory.name }}
        </option>
        <option v-if="factories.length === 0" :value="null" disabled>No factories available</option>
      </select>
    </div>

    <!-- Create Factory Modal -->
    <Modal
      v-model:show="showCreateModal"
      title="Create New Factory"
      @close="showCreateModal = false"
    >
      <form @submit.prevent="handleCreateFactory">
        <div class="form-group">
          <label for="factory-name" class="form-label">Factory Name *</label>
          <input
            id="factory-name"
            v-model="newFactory.name"
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
            v-model="newFactory.description"
            class="form-textarea"
            placeholder="Enter factory description..."
            rows="3"
          />
        </div>

        <div class="form-group">
          <label for="factory-notes" class="form-label">Notes</label>
          <textarea
            id="factory-notes"
            v-model="newFactory.notes"
            class="form-textarea"
            placeholder="Enter factory notes..."
            rows="3"
          />
        </div>

        <div class="form-actions">
          <Button
            type="button"
            variant="secondary"
            @click="showCreateModal = false"
          >
            Cancel
          </Button>
          <Button
            type="submit"
            variant="primary"
            :loading="creating"
          >
            Create Factory
          </Button>
        </div>
      </form>
    </Modal>

    <!-- Error Alert -->
    <Alert
      v-if="error"
      variant="danger"
      @close="clearError"
    >
      {{ error }}
    </Alert>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useFactoryStore } from '@/stores/factory'
import { usePreferencesStore } from '@/stores/preferences'
import type { CreateFactoryRequest } from '@/api/types'
import Button from '@/components/ui/Button.vue'
import Modal from '@/components/ui/Modal.vue'
import Alert from '@/components/ui/Alert.vue'

const factoryStore = useFactoryStore()
const preferencesStore = usePreferencesStore()

// State
const showCreateModal = ref(false)
const creating = ref(false)
const error = ref<string | null>(null)

// Form data
const newFactory = ref<CreateFactoryRequest>({
  name: '',
  description: '',
  notes: ''
})

// Computed
const factories = computed(() => factoryStore.factories)
const currentFactory = computed(() => factoryStore.currentFactory)
const selectedFactoryId = ref<string | null>(null)

// Methods
const handleFactoryChange = () => {
  const rawValue = selectedFactoryId.value

  // Don't allow null/empty selection - select first factory if null
  if (rawValue === null || rawValue === undefined || rawValue === '' || rawValue === 'null') {
    if (factories.value.length > 0) {
      selectedFactoryId.value = factories.value[0].id
      factoryStore.setCurrentFactory(factories.value[0].id)
      preferencesStore.setSelectedFactoryId(factories.value[0].id)
    } else {
      selectedFactoryId.value = null
      factoryStore.setCurrentFactory(null)
      preferencesStore.setSelectedFactoryId(null)
    }
    return
  }

  selectedFactoryId.value = String(rawValue)
  factoryStore.setCurrentFactory(selectedFactoryId.value)
  preferencesStore.setSelectedFactoryId(selectedFactoryId.value)
}

const handleCreateFactory = async () => {
  if (!newFactory.value.name.trim()) {
    error.value = 'Factory name is required'
    return
  }

  creating.value = true
  error.value = null

  try {
    const createdFactory = await factoryStore.create(newFactory.value)
    if (createdFactory) {
      // Select the newly created factory
      selectedFactoryId.value = createdFactory.id
      factoryStore.setCurrentFactory(createdFactory.id)
      preferencesStore.setSelectedFactoryId(createdFactory.id)

      // Reset form and close modal
      newFactory.value = { name: '', description: '', notes: '' }
      showCreateModal.value = false
    } else {
      error.value = 'Failed to create factory'
    }
  } catch (err) {
    error.value = 'An error occurred while creating the factory'
    console.error('Create factory error:', err)
  } finally {
    creating.value = false
  }
}

const clearError = () => {
  error.value = null
  factoryStore.clearError()
}

// Watch for current factory changes
watch(() => factoryStore.currentFactoryId, (newId) => {
  selectedFactoryId.value = newId ?? null
}, { immediate: true })

// Load factories on mount
onMounted(async () => {
  await factoryStore.fetchAll()

  // Restore selected factory from preferences, or default to first factory
  const prefFactoryId = preferencesStore.selectedFactoryId
  if (prefFactoryId && factories.value.some(f => f.id === prefFactoryId)) {
    selectedFactoryId.value = prefFactoryId
    factoryStore.setCurrentFactory(prefFactoryId)
  } else if (factories.value.length > 0) {
    // Default to first factory if no preference or preference is invalid
    selectedFactoryId.value = factories.value[0].id
    factoryStore.setCurrentFactory(factories.value[0].id)
    preferencesStore.setSelectedFactoryId(factories.value[0].id)
  }
})

// Watch for factories changes to set default if needed
watch(() => factories.value.length, (newLength, oldLength) => {
  // When factories are loaded and no factory is selected, select the first one
  if (newLength > 0 && (!selectedFactoryId.value || !factories.value.some(f => f.id === selectedFactoryId.value))) {
    selectedFactoryId.value = factories.value[0].id
    factoryStore.setCurrentFactory(factories.value[0].id)
    preferencesStore.setSelectedFactoryId(factories.value[0].id)
  }
  // If all factories are deleted and we had one selected, clear it
  else if (newLength === 0 && oldLength > 0 && selectedFactoryId.value) {
    selectedFactoryId.value = null
    factoryStore.setCurrentFactory(null)
    preferencesStore.setSelectedFactoryId(null)
  }
}, { immediate: false })
</script>

<style scoped lang="scss">
.factory-selector {
  background-color: var(--color-surface, #252525);
  border-radius: var(--border-radius-sm, 3px);
  box-shadow: var(--shadow-inset);
  border: 1px solid var(--color-border, #404040);
  padding: var(--spacing-lg, 1rem);
  margin-bottom: var(--spacing-lg, 1rem);
}

.selector-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-md, 0.75rem);
}

.selector-title {
  font-size: var(--font-size-xl, 1.25rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-text-primary, #e5e5e5);
  margin: 0;
}

.current-factory {
  background-color: var(--color-surface-inset, #1f1f1f);
  border-radius: var(--border-radius-sm, 3px);
  border: 1px solid var(--color-border, #404040);
  padding: var(--spacing-md, 0.75rem);
  margin-bottom: var(--spacing-md, 0.75rem);
}

.factory-info {
  margin-bottom: var(--spacing-sm, 0.5rem);
}

.factory-name {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary, #e5e5e5);
  margin: 0 0 var(--spacing-xs, 0.25rem) 0;
}

.factory-description {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-secondary, #b8b8b8);
  margin: 0;
  line-height: 1.5;
}

.selector-controls {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm, 0.5rem);
}

.selector-label {
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-secondary, #b8b8b8);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.factory-select {
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  border: 1px solid var(--color-border, #404040);
  border-radius: var(--border-radius-sm, 3px);
  font-size: var(--font-size-base, 1rem);
  background-color: var(--color-surface-inset, #1f1f1f);
  color: var(--color-text-primary, #e5e5e5);
  transition: all var(--transition-normal, 200ms);
  box-shadow: var(--shadow-inset-light);

  &:focus {
    outline: none;
    border-color: var(--color-ficsit-orange, #f58b00);
    box-shadow: var(--shadow-glow-orange);
    background-color: var(--color-surface, #252525);
  }

  &:hover:not(:disabled) {
    border-color: var(--color-border-light, #4a4a4a);
    background-color: var(--color-surface, #252525);
  }

  &:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
}

// Form styles
.form-group {
  margin-bottom: var(--spacing-md, 0.75rem);
}

.form-label {
  display: block;
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-700, #374151);
  margin-bottom: var(--spacing-xs, 0.25rem);
}

.form-input,
.form-textarea {
  width: 100%;
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-md, 0.375rem);
  font-size: var(--font-size-base, 1rem);
  transition: border-color 0.2s ease-in-out;

  &:focus {
    outline: none;
    border-color: var(--color-primary-500, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  &::placeholder {
    color: var(--color-gray-400, #9ca3af);
  }
}

.form-textarea {
  resize: vertical;
  min-height: 80px;
}

.form-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--spacing-sm, 0.5rem);
  margin-top: var(--spacing-lg, 1rem);
}

// Responsive design
@media (max-width: 640px) {
  .selector-header {
    flex-direction: column;
    align-items: stretch;
    gap: var(--spacing-sm, 0.5rem);
  }

  .form-actions {
    flex-direction: column;
    gap: var(--spacing-sm, 0.5rem);
  }

  .form-actions button {
    width: 100%;
  }
}
</style>
