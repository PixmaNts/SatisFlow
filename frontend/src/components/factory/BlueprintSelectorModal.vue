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
        <Alert type="error" :message="error" />
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

<style scoped lang="scss">
.blueprint-selector {
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

.template-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4, 1rem);
}

.list-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-4, 1rem);
}

.list-title {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-text-primary);
}

.template-count {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-muted);
}

.template-grid {
  display: grid;
  grid-template-columns: repeat(1, minmax(0, 1fr));
  gap: var(--spacing-4, 1rem);
  max-height: 24rem;
  overflow-y: auto;
}

.template-card {
  padding: var(--spacing-4, 1rem);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-lg, 0.5rem);
  cursor: pointer;
  transition: all var(--transition-normal, 200ms) cubic-bezier(0.4, 0, 0.2, 1);
}

.template-card:hover {
  box-shadow: var(--shadow-md);
  border-color: var(--color-ficsit-orange);
}

.template-card.selected {
  border-color: var(--color-ficsit-orange);
  background-color: rgba(245, 139, 0, 0.1);
}

.template-header {
  margin-bottom: var(--spacing-3, 0.75rem);
}

.template-name {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-text-primary);
}

.template-stats {
  display: flex;
  gap: var(--spacing-4, 1rem);
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-muted);
}

.stat-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-1, 0.25rem);
}

.stat-icon {
  font-size: var(--font-size-base, 1rem);
}

.template-description {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-muted);
  margin-bottom: var(--spacing-3, 0.75rem);
}

.template-items {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2, 0.5rem);
}

.items-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-1, 0.25rem);
}

.items-title {
  font-size: var(--font-size-xs, 0.75rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: var(--spacing-1, 0.25rem);
}

.items-list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-1, 0.25rem);
}

.item-tag {
  padding: var(--spacing-1, 0.25rem) var(--spacing-2, 0.5rem);
  font-size: var(--font-size-xs, 0.75rem);
  border-radius: var(--border-radius-full, 9999px);
  font-weight: var(--font-weight-medium, 500);
}

.input-item {
  background-color: rgba(239, 68, 68, 0.15);
  color: #fca5a5;
  border: 1px solid rgba(239, 68, 68, 0.3);
}

.output-item {
  background-color: rgba(34, 197, 94, 0.15);
  color: #86efac;
  border: 1px solid rgba(34, 197, 94, 0.3);
}

.selected-details {
  margin-top: var(--spacing-6, 1.5rem);
  padding: var(--spacing-4, 1rem);
  background-color: var(--color-surface);
  border-radius: var(--border-radius-lg, 0.5rem);
  border: 1px solid var(--color-border);
}

.details-title {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-ficsit-orange);
  margin-bottom: var(--spacing-3, 0.75rem);
}

.details-card {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4, 1rem);
}

.selected-name {
  font-size: var(--font-size-xl, 1.25rem);
  font-weight: var(--font-weight-bold, 700);
  color: var(--color-text-primary);
}

.selected-description {
  color: var(--color-text-muted);
}

.selected-stats {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--spacing-4, 1rem);
}

.selected-stat {
  display: flex;
  flex-direction: column;
}

.stat-label {
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-muted);
}

.stat-value {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-text-primary);
}

.name-override {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2, 0.5rem);
}

.override-label {
  display: block;
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-secondary);
  margin-bottom: var(--spacing-1, 0.25rem);
}

.modal-actions {
  display: flex;
  gap: var(--spacing-3, 0.75rem);
  justify-content: flex-end;
}
</style>
