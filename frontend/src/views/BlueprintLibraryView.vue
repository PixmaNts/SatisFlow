<template>
  <div class="blueprint-library-view">
    <!-- Page Header -->
    <PageHeader
      title="Blueprint Library"
      subtitle="Manage and reuse production line blueprints"
    >
      <template #actions>
        <div class="header-actions">
          <Button
            variant="primary"
            @click="showCreateModal = true"
            :disabled="loading"
          >
            <template #icon>
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
            </template>
            Create New
          </Button>

          <Button
            variant="secondary"
            @click="triggerFileImport"
            :disabled="loading"
          >
            <template #icon>
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
              </svg>
            </template>
            Import from File
          </Button>
        </div>
      </template>
    </PageHeader>

    <!-- Loading State -->
    <div v-if="loading" class="loading-container">
      <LoadingSpinner size="lg" />
      <p class="loading-text">Loading blueprint library...</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="error-container">
      <Alert variant="error" :message="error" />
      <Button @click="fetchTemplates" variant="primary">Retry</Button>
    </div>

    <!-- Empty State -->
    <div v-else-if="templates.length === 0" class="empty-state">
      <EmptyState
        title="No Blueprints Yet"
        description="Create your first blueprint or import one from a file to get started."
        icon="blueprint"
      >
        <template #actions>
          <Button @click="showCreateModal = true" variant="primary">
            Create First Blueprint
          </Button>
          <Button @click="triggerFileImport" variant="secondary">
            Import Blueprint File
          </Button>
        </template>
      </EmptyState>
    </div>

    <!-- Blueprint Grid -->
    <div v-else class="blueprint-grid">
      <BlueprintCard
        v-for="template in templates"
        :key="template.id"
        :template="template"
        @edit="onEditTemplate"
        @export="onExportTemplate"
        @delete="onDeleteTemplate"
        @use-in-factory="onUseInFactory"
      />
    </div>

    <!-- Create/Edit Modal -->
    <BlueprintFormModal
      v-if="showCreateModal || editingTemplate"
      :show="showCreateModal || !!editingTemplate"
      :template="editingTemplate"
      @close="closeModal"
      @save="onSaveTemplate"
    />

    <!-- Import Preview Modal -->
    <BlueprintPreviewModal
      v-if="showImportModal"
      :show="showImportModal"
      :metadata="blueprintMetadata"
      @close="closeImportModal"
      @import="onImportBlueprint"
    />

    <!-- Factory Selector Modal -->
    <FactorySelectorModal
      v-if="showFactorySelector && selectedTemplate"
      :show="showFactorySelector"
      :template="selectedTemplate"
      @close="showFactorySelector = false"
      @select="onSelectFactory"
    />

    <!-- Hidden file input for import -->
    <input
      ref="fileInput"
      type="file"
      accept=".json"
      style="display: none"
      @change="onFileSelected"
    />

    <!-- Confirmation Dialog -->
    <ConfirmDialog
      v-if="deleteConfirmation"
      :show="!!deleteConfirmation"
      :title="`Delete '${deleteConfirmation.name}'?`"
      :message="`Are you sure you want to delete '${deleteConfirmation.name}'? Factory instances will not be affected.`"
      :confirm-text="'Delete Blueprint'"
      :cancel-text="'Cancel'"
      variant="danger"
      @confirm="confirmDelete"
      @cancel="cancelDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { blueprintTemplates } from '@/api/endpoints';
import type { BlueprintTemplateResponse, CreateBlueprintTemplateRequest, ImportTemplateRequest, BlueprintMetadata } from '@/api/types';
import { useToast } from '@/composables/useToast';
import { useErrorHandler } from '@/composables/useErrorHandler';

// Components
import PageHeader from '@/components/layout/PageHeader.vue';
import Button from '@/components/ui/Button.vue';
import BlueprintCard from '@/components/blueprints/BlueprintCard.vue';
import BlueprintFormModal from '@/components/blueprints/BlueprintFormModal.vue';
import BlueprintPreviewModal from '@/components/factory/BlueprintPreviewModal.vue';
import FactorySelectorModal from '@/components/blueprints/FactorySelectorModal.vue';
import LoadingSpinner from '@/components/ui/LoadingSpinner.vue';
import Alert from '@/components/ui/Alert.vue';
import EmptyState from '@/components/ui/EmptyState.vue';
import ConfirmDialog from '@/components/ui/ConfirmDialog.vue';

// Composables
const { showSuccess, showError } = useToast();
const { handleError } = useErrorHandler();

// State
const templates = ref<BlueprintTemplateResponse[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);
const showCreateModal = ref(false);
const showImportModal = ref(false);
const showFactorySelector = ref(false);
const editingTemplate = ref<BlueprintTemplateResponse | null>(null);
const selectedTemplate = ref<BlueprintTemplateResponse | null>(null);
const deleteConfirmation = ref<BlueprintTemplateResponse | null>(null);
const fileInput = ref<HTMLInputElement>();

// Blueprint import state
const blueprintMetadata = ref<BlueprintMetadata | null>(null);
const blueprintJsonToImport = ref<string>('');

// Methods
const fetchTemplates = async () => {
  loading.value = true;
  error.value = null;

  try {
    templates.value = await blueprintTemplates.getAll();
  } catch {
    error.value = 'Failed to load blueprint library';
  } finally {
    loading.value = false;
  }
};

const triggerFileImport = () => {
  fileInput.value?.click();
};

const onFileSelected = async (event: Event) => {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];

  if (!file) return;

  try {
    const text = await file.text();
    const parsed = JSON.parse(text);

    // Basic validation before sending to backend
    if (!parsed.id || !parsed.name || !Array.isArray(parsed.production_lines)) {
      throw new Error('Invalid blueprint file format - must have id, name, and production_lines array');
    }

    // Validate UUID format for blueprint ID
    const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
    if (!uuidRegex.test(parsed.id)) {
      throw new Error('Invalid blueprint ID - must be a valid UUID');
    }

    // Validate production lines structure
    if (parsed.production_lines.length === 0) {
      throw new Error('Blueprint must contain at least one production line');
    }

    for (const line of parsed.production_lines) {
      if (!line.id || !line.recipe || !Array.isArray(line.machine_groups)) {
        throw new Error('Invalid production line structure');
      }
    }

    // Get metadata from backend preview endpoint
    // This calculates power, items, etc. using the engine
    blueprintJsonToImport.value = text;
    blueprintMetadata.value = await blueprintTemplates.preview(text);
    showImportModal.value = true;
  } catch (err) {
    const message = err instanceof Error ? err.message : 'Failed to read blueprint file';
    showError(message);
    console.error('Failed to read blueprint file:', err);
  }

  // Reset file input
  target.value = '';
};

const onEditTemplate = (template: BlueprintTemplateResponse) => {
  editingTemplate.value = template;
};

const onExportTemplate = async (template: BlueprintTemplateResponse) => {
  try {
    const exportData = await blueprintTemplates.export(template.id);

    // Create download link
    const blob = new Blob([exportData.blueprint_json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `blueprint-${template.name.toLowerCase().replace(/\s+/g, '-')}-${Date.now()}.json`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);

    showSuccess(`Blueprint '${template.name}' exported successfully`);
  } catch {
    showError('Failed to export blueprint');
  }
};

const onDeleteTemplate = (template: BlueprintTemplateResponse) => {
  deleteConfirmation.value = template;
};

const confirmDelete = async () => {
  if (!deleteConfirmation.value) return;

  try {
    await blueprintTemplates.delete(deleteConfirmation.value.id);
    templates.value = templates.value.filter(t => t.id !== deleteConfirmation.value!.id);
    showSuccess(`Blueprint '${deleteConfirmation.value.name}' deleted successfully`);
  } catch {
    showError('Failed to delete blueprint');
  } finally {
    deleteConfirmation.value = null;
  }
};

const cancelDelete = () => {
  deleteConfirmation.value = null;
};

const onUseInFactory = (template: BlueprintTemplateResponse) => {
  selectedTemplate.value = template;
  showFactorySelector.value = true;
};

const onSelectFactory = async (factoryId: string) => {
  if (!selectedTemplate.value) return;

  try {
    const response = await blueprintTemplates.createInstanceInFactory(
      factoryId,
      selectedTemplate.value.id,
      {}
    );

    showSuccess(response.message);
    showFactorySelector.value = false;
    selectedTemplate.value = null;
  } catch (err) {
    const appError = handleError(err, { action: 'create_blueprint_instance' });
    showError(appError.userMessage || 'Failed to create blueprint instance');
  }
};

const onSaveTemplate = async (templateData: CreateBlueprintTemplateRequest) => {
  try {
    if (editingTemplate.value) {
      // Update creates new version
      const newTemplate = await blueprintTemplates.update(editingTemplate.value.id, templateData);
      templates.value.push(newTemplate);
      showSuccess(`Blueprint '${newTemplate.name}' updated (new version created)`);
    } else {
      // Create new template
      const newTemplate = await blueprintTemplates.create(templateData);
      templates.value.push(newTemplate);
      showSuccess(`Blueprint '${newTemplate.name}' created successfully`);
    }

    closeModal();
  } catch (err) {
    const appError = handleError(err, { action: 'save_blueprint' });
    showError(appError.userMessage || 'Failed to save blueprint');
  }
};

const onImportBlueprint = async (customName?: string) => {
  try {
    const importData: ImportTemplateRequest = {
      blueprint_json: blueprintJsonToImport.value,
      name: customName || undefined,
    };

    const importedTemplate = await blueprintTemplates.importToLibrary(importData);
    templates.value.push(importedTemplate);

    // Close modal and clear state
    showImportModal.value = false;
    blueprintMetadata.value = null;
    blueprintJsonToImport.value = '';

    showSuccess(`Blueprint '${importedTemplate.name}' imported successfully`);
  } catch (err) {
    const appError = handleError(err, { action: 'import_blueprint' });
    showError(appError.userMessage || 'Failed to import blueprint');
  }
};

const closeModal = () => {
  showCreateModal.value = false;
  editingTemplate.value = null;
};

const closeImportModal = () => {
  showImportModal.value = false;
  blueprintMetadata.value = null;
  blueprintJsonToImport.value = '';
};

// Lifecycle
onMounted(() => {
  fetchTemplates();
});
</script>

<style scoped>
.blueprint-library-view {
  @apply p-6;
}

.header-actions {
  @apply flex gap-3;
}

.loading-container {
  @apply flex flex-col items-center justify-center py-12;
}

.loading-text {
  @apply mt-4 text-gray-600;
}

.error-container {
  @apply py-12;
}

.empty-state {
  @apply py-12;
}

.blueprint-grid {
  @apply grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 mt-6;
}
</style>
