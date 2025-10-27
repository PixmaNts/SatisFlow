<template>
  <Modal
    :show="show"
    title="Import Blueprint"
    size="md"
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

<style scoped>
.import-form {
  @apply space-y-4;
}

.form-field {
  @apply space-y-2;
}

.field-label {
  @apply block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1;
}

.json-textarea {
  @apply w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:text-white font-mono text-sm;
}

.preview-section {
  @apply mt-4 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg;
}

.preview-title {
  @apply text-base font-medium text-gray-900 dark:text-gray-100 mb-2;
}

.preview-stats {
  @apply space-y-2;
}

.stat-item {
  @apply flex justify-between;
}

.stat-label {
  @apply text-sm text-gray-600 dark:text-gray-400;
}

.stat-value {
  @apply text-sm font-medium text-gray-900 dark:text-gray-100;
}

.modal-actions {
  @apply flex gap-3 justify-end;
}
</style>
