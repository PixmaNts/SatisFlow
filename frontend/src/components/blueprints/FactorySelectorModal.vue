<template>
  <Modal
    :show="show"
    title="Select Factory"
    size="lg"
    @close="$emit('close')"
  >
    <div class="factory-selector">
      <div v-if="loading" class="loading-state">
        <LoadingSpinner size="md" />
        <p class="loading-text">Loading factories...</p>
      </div>

      <div v-else-if="error" class="error-state">
        <Alert type="error" :message="error" />
        <Button @click="loadFactories" variant="primary">Retry</Button>
      </div>

      <div v-else-if="factoriesList.length === 0" class="empty-state">
        <EmptyState
          title="No Factories Available"
          description="You need to create a factory before you can add a blueprint to it."
          icon="factory"
        >
          <template #actions>
            <Button @click="$emit('close')" variant="secondary">Close</Button>
          </template>
        </EmptyState>
      </div>

      <div v-else class="factory-list">
        <div class="form-field">
          <label for="factory-select" class="field-label">Select Factory</label>
          <BaseSelect
            id="factory-select"
            v-model="selectedFactoryId"
            :options="factoryOptions"
            placeholder="Choose a factory"
            required
          />
        </div>

        <div v-if="selectedFactory" class="factory-preview">
          <h4 class="preview-title">{{ selectedFactory.name }}</h4>
          <p v-if="selectedFactory.description" class="preview-description">
            {{ selectedFactory.description }}
          </p>
          <div class="preview-stats">
            <div class="stat-item">
              <span class="stat-label">Production Lines:</span>
              <span class="stat-value">{{ selectedFactory.production_lines?.length || 0 }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Power Consumption:</span>
              <span class="stat-value">{{ selectedFactory.total_power_consumption?.toFixed(1) || 0 }} MW</span>
            </div>
          </div>
        </div>

        <div class="form-field">
          <label for="blueprint-name" class="field-label">Blueprint Name (Optional)</label>
          <BaseInput
            id="blueprint-name"
            v-model="blueprintName"
            placeholder="Override blueprint name"
          />
        </div>
      </div>
    </div>

    <template #footer>
      <div class="modal-actions">
        <Button variant="secondary" @click="$emit('close')">
          Cancel
        </Button>
        <Button
          variant="primary"
          @click="onAddToFactory"
          :disabled="!selectedFactoryId || loading"
          :loading="loading"
        >
          Add to Factory
        </Button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { factories } from '@/api/endpoints';
import type { FactoryResponse } from '@/api/types';
import BaseSelect from '@/components/forms/BaseSelect.vue';
import BaseInput from '@/components/forms/BaseInput.vue';
import Button from '@/components/ui/Button.vue';
import Modal from '@/components/ui/Modal.vue';
import LoadingSpinner from '@/components/ui/LoadingSpinner.vue';
import Alert from '@/components/ui/Alert.vue';
import EmptyState from '@/components/ui/EmptyState.vue';

interface Props {
  /** Whether modal is shown */
  show: boolean;
  /** Template to add to factory */
  template: {
    id: string;
    name: string;
    description?: string | null;
    total_machines: number;
    total_power: number;
  };
}

interface Emits {
  (e: 'close'): void;
  (e: 'select', factoryId: string, blueprintName: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// State
const loading = ref(false);
const error = ref<string | null>(null);
const factoriesList = ref<FactoryResponse[]>([]);
const selectedFactoryId = ref<string>('');
const blueprintName = ref('');

// Computed
const factoryOptions = computed(() =>
  factoriesList.value.map(factory => ({
    value: factory.id,
    label: factory.name,
  }))
);

const selectedFactory = computed(() =>
  factoriesList.value.find(factory => factory.id === selectedFactoryId.value)
);

// Methods
const loadFactories = async () => {
  loading.value = true;
  error.value = null;

  try {
    factoriesList.value = await factories.getAll();
  } catch (err) {
    error.value = 'Failed to load factories';
    console.error('Factory loading error:', err);
  } finally {
    loading.value = false;
  }
};

const onAddToFactory = () => {
  if (!selectedFactoryId.value) return;

  emit('select', selectedFactoryId.value, blueprintName.value.trim());
};

// Watchers
watch(() => props.show, (show) => {
  if (show) {
    loadFactories();
    selectedFactoryId.value = '';
    blueprintName.value = '';
  }
});

// Lifecycle
onMounted(() => {
  // Initial load will be triggered by watcher when show becomes true
});
</script>

<style scoped lang="scss">
.factory-selector {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4, 1rem);
}

.loading-state,
.error-state,
.empty-state {
  padding-top: var(--spacing-8, 2rem);
  padding-bottom: var(--spacing-8, 2rem);
}

.loading-text {
  margin-top: var(--spacing-4, 1rem);
  text-align: center;
  color: var(--color-text-muted);
}

.factory-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4, 1rem);
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-1, 0.25rem);
}

.field-label {
  display: block;
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-secondary);
  margin-bottom: var(--spacing-1, 0.25rem);
}

.factory-preview {
  margin-top: var(--spacing-4, 1rem);
  padding: var(--spacing-4, 1rem);
  background-color: var(--color-surface);
  border-radius: var(--border-radius-lg, 0.5rem);
  border: 1px solid var(--color-border);
}

.preview-title {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-text-primary);
}

.preview-description {
  margin-top: var(--spacing-2, 0.5rem);
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-muted);
}

.preview-stats {
  margin-top: var(--spacing-3, 0.75rem);
  display: flex;
  gap: var(--spacing-4, 1rem);
}

.stat-item {
  display: flex;
  flex-direction: column;
}

.stat-label {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-muted);
}

.stat-value {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary);
}

.modal-actions {
  display: flex;
  gap: var(--spacing-3, 0.75rem);
  justify-content: flex-end;
}
</style>
