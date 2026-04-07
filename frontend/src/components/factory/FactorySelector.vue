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
      size="lg"
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
      type="error"
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
      const firstFactory = factories.value[0]
      if (firstFactory) {
        selectedFactoryId.value = firstFactory.id
        factoryStore.setCurrentFactory(firstFactory.id)
        preferencesStore.setSelectedFactoryId(firstFactory.id)
      }
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
    const firstFactory = factories.value[0]
    if (firstFactory) {
      selectedFactoryId.value = firstFactory.id
      factoryStore.setCurrentFactory(firstFactory.id)
      preferencesStore.setSelectedFactoryId(firstFactory.id)
    }
  }
})

// Watch for factories changes to set default if needed
watch(() => factories.value.length, (newLength, oldLength) => {
  // When factories are loaded and no factory is selected, select the first one
  if (newLength > 0 && (!selectedFactoryId.value || !factories.value.some(f => f.id === selectedFactoryId.value))) {
    const firstFactory = factories.value[0]
    if (firstFactory) {
      selectedFactoryId.value = firstFactory.id
      factoryStore.setCurrentFactory(firstFactory.id)
      preferencesStore.setSelectedFactoryId(firstFactory.id)
    }
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
  background-color: var(--color-surface);
  border-radius: var(--border-radius-md);
  box-shadow: var(--shadow-inset);
  border: 1px solid var(--color-border);
  padding: var(--spacing-xl);
  margin-bottom: var(--spacing-lg);
}

.selector-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-lg);
}

.selector-title {
  font-size: var(--font-size-2xl);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
  margin: 0;
  font-family: var(--font-family-sans);
  letter-spacing: -0.01em;
}

.current-factory {
  background-color: var(--color-surface-inset);
  border-radius: var(--border-radius-md);
  border: 1px solid var(--color-border-dark);
  padding: var(--spacing-lg);
  margin-bottom: var(--spacing-lg);
  transition: all var(--transition-normal);
  
  &:hover {
    border-color: var(--color-border);
  }
}

.factory-info {
  margin-bottom: var(--spacing-sm);
}

.factory-name {
  font-size: var(--font-size-xl);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  margin: 0 0 var(--spacing-xs) 0;
  font-family: var(--font-family-sans);
}

.factory-description {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.6;
}

.selector-controls {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.selector-label {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.factory-select {
  padding: var(--spacing-md) var(--spacing-lg);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-md);
  font-size: var(--font-size-base);
  background-color: var(--color-surface-inset);
  color: var(--color-text-primary);
  transition: all var(--transition-normal);
  box-shadow: var(--shadow-inset-light);
  font-family: var(--font-family-sans);
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%238a8a8a' d='M6 8L1 3h10z'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right var(--spacing-md) center;
  padding-right: var(--spacing-3xl);

  &:focus {
    outline: none;
    border-color: var(--color-ficsit-orange);
    box-shadow: var(--shadow-glow-orange);
    background-color: var(--color-surface);
  }

  &:hover:not(:disabled) {
    border-color: var(--color-border-light);
    background-color: var(--color-surface);
  }

  &:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
}

// Form styles
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
  transition: all var(--transition-normal);
  background-color: var(--color-surface-inset);
  color: var(--color-text-primary);
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
  align-items: center;
  justify-content: flex-end;
  gap: var(--spacing-md);
  margin-top: var(--spacing-xl);
  padding-top: var(--spacing-lg);
  border-top: 1px solid var(--color-border-dark);
}

// Responsive design
@media (max-width: 640px) {
  .factory-selector {
    padding: var(--spacing-lg);
  }
  
  .selector-header {
    flex-direction: column;
    align-items: stretch;
    gap: var(--spacing-md);
  }

  .form-actions {
    flex-direction: column;
    gap: var(--spacing-md);
  }

  .form-actions button {
    width: 100%;
  }
}
</style>
