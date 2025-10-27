<template>
  <Modal
    :show="show"
    title="Select Factory"
    size="md"
    @close="$emit('close')"
  >
    <div class="factory-selector">
      <div v-if="loading" class="loading-state">
        <LoadingSpinner size="md" />
        <p class="loading-text">Loading factories...</p>
      </div>

      <div v-else-if="error" class="error-state">
        <Alert variant="error" :message="error" />
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

<style scoped>
.factory-selector {
  @apply space-y-4;
}

.loading-state, .error-state, .empty-state {
  @apply py-8;
}

.loading-text {
  @apply mt-4 text-center text-gray-600 dark:text-gray-400;
}

.factory-list {
  @apply space-y-4;
}

.form-field {
  @apply space-y-1;
}

.field-label {
  @apply block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1;
}

.factory-preview {
  @apply mt-4 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600;
}

.preview-title {
  @apply text-lg font-semibold text-gray-900 dark:text-gray-100;
}

.preview-description {
  @apply mt-2 text-sm text-gray-600 dark:text-gray-400;
}

.preview-stats {
  @apply mt-3 flex gap-4;
}

.stat-item {
  @apply flex flex-col;
}

.stat-label {
  @apply text-sm text-gray-600 dark:text-gray-400;
}

.stat-value {
  @apply text-base font-medium text-gray-900 dark:text-gray-100;
}

.modal-actions {
  @apply flex gap-3 justify-end;
}
</style>
