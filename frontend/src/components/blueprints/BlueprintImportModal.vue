<template>
  <Modal
    :show="show"
    title="Import Blueprint"
    size="xl"
    @close="$emit('close')"
  >
    <div class="import-form">
      <div class="form-field">
        <label for="blueprint-name" class="field-label">Blueprint Name (Optional)</label>
        <BaseInput
          id="blueprint-name"
          v-model="blueprintName"
          placeholder="Override imported blueprint name"
        />
      </div>

      <div class="form-field">
        <label for="blueprint-file" class="field-label">Blueprint JSON</label>
        <textarea
          id="blueprint-file"
          v-model="blueprintJson"
          placeholder="Paste blueprint JSON here..."
          class="json-textarea"
          rows="10"
        />
      </div>

      <div v-if="preview" class="preview-section">
        <h4 class="preview-title">Preview</h4>
        <div class="preview-stats">
          <div class="stat-item">
            <span class="stat-label">Name:</span>
            <span class="stat-value">{{ preview.name }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Machines:</span>
            <span class="stat-value">{{ preview.total_machines }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Power:</span>
            <span class="stat-value">{{ preview.total_power?.toFixed(1) }} MW</span>
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
          @click="onImport"
          :disabled="!isValid || loading"
          :loading="loading"
        >
          Import to Library
        </Button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import type { ImportTemplateRequest, TemplateMetadata } from '@/api/types';
import BaseInput from '@/components/forms/BaseInput.vue';
import Button from '@/components/ui/Button.vue';
import Modal from '@/components/ui/Modal.vue';

interface Props {
  /** Whether modal is shown */
  show: boolean;
  /** Initial blueprint JSON */
  initialJson?: string;
}

interface Emits {
  (e: 'close'): void;
  (e: 'import', data: ImportTemplateRequest): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// State
const loading = ref(false);
const blueprintName = ref('');
const blueprintJson = ref(props.initialJson || '');
const preview = ref<TemplateMetadata | null>(null);
const error = ref<string | null>(null);

// Watch for initialJson changes
watch(() => props.initialJson, (newJson) => {
  if (newJson) {
    blueprintJson.value = newJson;
  }
});

// Computed
const isValid = computed(() => {
  if (!blueprintJson.value.trim()) return false;

  try {
    JSON.parse(blueprintJson.value);
    return true;
  } catch {
    return false;
  }
});

// Methods
const parseBlueprint = () => {
  if (!blueprintJson.value.trim()) {
    preview.value = null;
    error.value = null;
    return;
  }

  try {
    const parsed = JSON.parse(blueprintJson.value);

    // Create a simple preview from parsed data
    preview.value = {
      name: parsed.name || 'Unknown Blueprint',
      description: parsed.description || null,
      total_machines: 0, // Would need backend calculation for accurate value
      total_power: 0,
      input_items: [],
      output_items: [],
      exported_at: new Date().toISOString(),
    };
    error.value = null;
  } catch (err) {
    console.error('JSON parsing error:', err);
    preview.value = null;
    error.value = 'Invalid JSON format';
  }
};

const onImport = async () => {
  if (!isValid.value || !preview.value) return;

  loading.value = true;

  try {
    const importData: ImportTemplateRequest = {
      blueprint_json: blueprintJson.value,
      name: blueprintName.value.trim() || undefined,
    };

    emit('import', importData);
  } catch (err) {
    console.error('Import failed:', err);
  } finally {
    loading.value = false;
  }
};

// Watchers
watch(blueprintJson, () => parseBlueprint());
</script>

<style scoped lang="scss">
.import-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4, 1rem);
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2, 0.5rem);
}

.field-label {
  display: block;
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-secondary);
  margin-bottom: var(--spacing-1, 0.25rem);
}

.json-textarea {
  width: 100%;
  padding: var(--spacing-2, 0.5rem) var(--spacing-3, 0.75rem);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-md, 0.375rem);
  box-shadow: var(--shadow-sm);
  background-color: var(--color-surface-inset);
  color: var(--color-text-primary);
  font-family: var(--font-family-mono, 'JetBrains Mono', monospace);
  font-size: var(--font-size-sm, 0.875rem);
  resize: vertical;

  &:focus {
    outline: none;
    border-color: var(--color-ficsit-orange);
    box-shadow: var(--shadow-glow-orange);
  }
}

.preview-section {
  margin-top: var(--spacing-4, 1rem);
  padding: var(--spacing-4, 1rem);
  background-color: var(--color-surface);
  border-radius: var(--border-radius-lg, 0.5rem);
}

.preview-title {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary);
  margin-bottom: var(--spacing-2, 0.5rem);
}

.preview-stats {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2, 0.5rem);
}

.stat-item {
  display: flex;
  justify-content: space-between;
}

.stat-label {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-muted);
}

.stat-value {
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary);
}

.modal-actions {
  display: flex;
  gap: var(--spacing-3, 0.75rem);
  justify-content: flex-end;
}
</style>
