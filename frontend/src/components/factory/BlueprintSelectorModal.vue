<template>
  <Modal
    :show="show"
    title="Select Blueprint Template"
    size="lg"
    @close="$emit('close')"
  >
    <div class="blueprint-selector">
      <!-- Loading State -->
      <div v-if="loading" class="loading-state">
        <LoadingSpinner size="md" />
        <p class="loading-text">Loading blueprint templates...</p>
      </div>

      <!-- Error State -->
      <div v-else-if="error" class="error-state">
        <Alert variant="error" :message="error" />
        <Button @click="loadTemplates" variant="primary">Retry</Button>
      </div>

      <!-- Empty State -->
      <div v-else-if="templates.length === 0" class="empty-state">
        <EmptyState
          title="No Blueprint Templates"
          description="You haven't created any blueprint templates yet. Go to the Blueprint Library to create your first template."
          icon="blueprint"
        >
          <template #actions>
            <Button @click="$emit('close')" variant="secondary">Close</Button>
          </template>
        </EmptyState>
      </div>

      <!-- Template List -->
      <div v-else class="template-list">
        <div class="list-header">
          <h3 class="list-title">Available Templates</h3>
          <div class="template-count">{{ templates.length }} templates</div>
        </div>

        <div class="template-grid">
          <div
            v-for="template in templates"
            :key="template.id"
            class="template-card"
            :class="{ 'selected': selectedTemplateId === template.id }"
            @click="selectTemplate(template)"
          >
            <div class="template-header">
              <h4 class="template-name">{{ template.name }}</h4>
              <div class="template-stats">
                <span class="stat-item">
                  <span class="stat-icon">⚙️</span>
                  {{ template.total_machines }} machines
                </span>
                <span class="stat-item">
                  <span class="stat-icon">⚡</span>
                  {{ template.total_power.toFixed(1) }} MW
                </span>
                <span class="stat-item">
                  <span class="stat-icon">📋</span>
                  {{ template.production_lines.length }} lines
                </span>
              </div>
            </div>

            <div v-if="template.description" class="template-description">
              {{ template.description }}
            </div>

            <div class="template-items">
              <div v-if="template.input_items.length > 0" class="items-section">
                <h5 class="items-title">Inputs</h5>
                <div class="items-list">
                  <div
                    v-for="[item, quantity] in template.input_items"
                    :key="item"
                    class="item-tag input-item"
                  >
                    {{ item }}: {{ quantity.toFixed(1) }}/min
                  </div>
                </div>
              </div>

              <div v-if="template.output_items.length > 0" class="items-section">
                <h5 class="items-title">Outputs</h5>
                <div class="items-list">
                  <div
                    v-for="[item, quantity] in template.output_items"
                    :key="item"
                    class="item-tag output-item"
                  >
                    {{ item }}: {{ quantity.toFixed(1) }}/min
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Selected Template Details -->
      <div v-if="selectedTemplate" class="selected-details">
        <h3 class="details-title">Selected Template</h3>
        <div class="details-card">
          <h4 class="selected-name">{{ selectedTemplate.name }}</h4>
          <p v-if="selectedTemplate.description" class="selected-description">
            {{ selectedTemplate.description }}
          </p>

          <div class="selected-stats">
            <div class="selected-stat">
              <span class="stat-label">Machines:</span>
              <span class="stat-value">{{ selectedTemplate.total_machines }}</span>
            </div>
            <div class="selected-stat">
              <span class="stat-label">Power:</span>
              <span class="stat-value">{{ selectedTemplate.total_power.toFixed(1) }} MW</span>
            </div>
            <div class="selected-stat">
              <span class="stat-label">Production Lines:</span>
              <span class="stat-value">{{ selectedTemplate.production_lines.length }}</span>
            </div>
          </div>

          <div class="name-override">
            <label for="blueprint-name" class="override-label">Blueprint Name (Optional)</label>
            <BaseInput
              id="blueprint-name"
              v-model="blueprintName"
              placeholder="Override blueprint name"
            />
          </div>
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
          @click="confirmSelection"
          :disabled="!selectedTemplateId || loading"
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
import { blueprintTemplates } from '@/api/endpoints';
import type { BlueprintTemplateResponse } from '@/api/types';
import BaseInput from '@/components/forms/BaseInput.vue';
import Button from '@/components/ui/Button.vue';
import Modal from '@/components/ui/Modal.vue';
import LoadingSpinner from '@/components/ui/LoadingSpinner.vue';
import Alert from '@/components/ui/Alert.vue';
import EmptyState from '@/components/ui/EmptyState.vue';

interface Props {
  /** Whether modal is shown */
  show: boolean;
  /** Factory ID to add blueprint to */
  factoryId: string;
}

interface Emits {
  (e: 'close'): void;
  (e: 'select', templateId: string, name: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// State
const loading = ref(false);
const error = ref<string | null>(null);
const templates = ref<BlueprintTemplateResponse[]>([]);
const selectedTemplateId = ref<string | null>(null);
const blueprintName = ref('');

// Computed
const selectedTemplate = computed(() =>
  templates.value.find(t => t.id === selectedTemplateId.value) || null
);

// Methods
const loadTemplates = async () => {
  loading.value = true;
  error.value = null;

  try {
    templates.value = await blueprintTemplates.getAll();
  } catch (err) {
    error.value = 'Failed to load blueprint templates';
    console.error('Failed to load templates:', err);
  } finally {
    loading.value = false;
  }
};

const selectTemplate = (template: BlueprintTemplateResponse) => {
  selectedTemplateId.value = template.id;
  // Reset name override when selecting a new template
  if (blueprintName.value === '') {
    blueprintName.value = template.name;
  }
};

const confirmSelection = () => {
  if (!selectedTemplateId.value) return;

  const template = templates.value.find(t => t.id === selectedTemplateId.value);
  if (!template) return;

  emit('select', template.id, blueprintName.value.trim() || template.name);
};

// Watchers
watch(() => props.show, (show) => {
  if (show) {
    loadTemplates();
    // Reset state when opening
    selectedTemplateId.value = null;
    blueprintName.value = '';
  }
});
</script>

<style scoped>
.blueprint-selector {
  @apply space-y-4;
}

.loading-state, .error-state, .empty-state {
  @apply py-8;
}

.loading-text {
  @apply mt-4 text-center text-gray-600;
}

.template-list {
  @apply space-y-4;
}

.list-header {
  @apply flex items-center justify-between mb-4;
}

.list-title {
  @apply text-lg font-semibold text-gray-900 dark:text-gray-100;
}

.template-count {
  @apply text-sm text-gray-500 dark:text-gray-400;
}

.template-grid {
  @apply grid grid-cols-1 gap-4 max-h-96 overflow-y-auto;
}

.template-card {
  @apply p-4 border border-gray-200 dark:border-gray-700 rounded-lg cursor-pointer transition-all duration-200 hover:shadow-md;
}

.template-card:hover {
  @apply border-primary-500 dark:border-primary-400;
}

.template-card.selected {
  @apply border-primary-500 dark:border-primary-400 bg-primary-50 dark:bg-primary-900/20;
}

.template-header {
  @apply mb-3;
}

.template-name {
  @apply text-base font-semibold text-gray-900 dark:text-gray-100;
}

.template-stats {
  @apply flex gap-4 text-sm text-gray-600 dark:text-gray-400;
}

.stat-item {
  @apply flex items-center gap-1;
}

.stat-icon {
  @apply text-base;
}

.template-description {
  @apply text-sm text-gray-600 dark:text-gray-400 mb-3;
}

.template-items {
  @apply space-y-2;
}

.items-section {
  @apply space-y-1;
}

.items-title {
  @apply text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-1;
}

.items-list {
  @apply flex flex-wrap gap-1;
}

.item-tag {
  @apply px-2 py-1 text-xs rounded-full font-medium;
}

.input-item {
  @apply bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200;
}

.output-item {
  @apply bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200;
}

.selected-details {
  @apply mt-6 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-700;
}

.details-title {
  @apply text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3;
}

.details-card {
  @apply space-y-4;
}

.selected-name {
  @apply text-xl font-bold text-gray-900 dark:text-gray-100;
}

.selected-description {
  @apply text-gray-600 dark:text-gray-400;
}

.selected-stats {
  @apply grid grid-cols-2 gap-4;
}

.selected-stat {
  @apply flex flex-col;
}

.stat-label {
  @apply text-sm font-medium text-gray-500 dark:text-gray-400;
}

.stat-value {
  @apply text-base font-semibold text-gray-900 dark:text-gray-100;
}

.name-override {
  @apply space-y-2;
}

.override-label {
  @apply block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1;
}

.modal-actions {
  @apply flex gap-3 justify-end;
}
</style>
