<template>
  <Modal
    :show="show"
    :title="template ? 'Edit Blueprint' : 'Create New Blueprint'"
    size="xl"
    @close="$emit('close')"
  >
    <div class="blueprint-form">
      <!-- Basic Info -->
      <div class="form-section">
        <h3 class="section-title">Blueprint Information</h3>
        <div class="form-row">
          <div class="form-field">
            <label for="blueprint-name">Name *</label>
            <BaseInput
              id="blueprint-name"
              v-model="formData.name"
              placeholder="Enter blueprint name"
              :error="errors.name"
              required
            />
          </div>
          <div class="form-field">
            <label for="blueprint-description">Description</label>
            <BaseInput
              id="blueprint-description"
              v-model="formData.description"
              placeholder="Optional description"
              :error="errors.description || undefined"
            />
          </div>
        </div>
      </div>

      <!-- Production Lines -->
      <div class="form-section">
        <div class="section-header">
          <h3 class="section-title">Production Lines ({{ formData.production_lines.length }})</h3>
          <Button variant="primary" size="sm" @click="addProductionLine">
            Add Line
          </Button>
        </div>

        <div v-if="formData.production_lines.length === 0" class="empty-state">
          No production lines yet. Add your first production line to get started.
        </div>

        <div v-else class="production-lines">
          <Collapsible
            v-for="(line, index) in formData.production_lines"
            :key="index"
            :default-open="index === 0"
          >
            <template #header>
              <div class="line-header">
                <span class="line-number">{{ index + 1 }}</span>
                <span class="line-name">{{ line.name || `Production Line ${index + 1}` }}</span>
                <span v-if="line.recipe" class="line-recipe">{{ line.recipe }}</span>
              </div>
            </template>

            <template #badge>
              <div v-if="line.machine_groups && line.machine_groups.length > 0" class="line-stats">
                <span class="stat">{{ getLineMachineCount(line) }} machines</span>
                <span class="stat">{{ getLinePower(line).toFixed(1) }} MW</span>
              </div>
            </template>

            <template #actions>
              <Button variant="ghost" size="sm" @click="confirmDeleteLine(index)">
                Delete
              </Button>
            </template>

            <div class="line-content">
              <div class="form-row">
                <div class="form-field">
                  <label :for="`line-name-${index}`">Name *</label>
                  <BaseInput
                    :id="`line-name-${index}`"
                    v-model="line.name"
                    placeholder="Production line name"
                    :error="getLineError(index, 'name')"
                    required
                  />
                </div>
                <div class="form-field">
                  <label :for="`line-description-${index}`">Description</label>
                  <BaseInput
                    :id="`line-description-${index}`"
                    v-model="line.description"
                    placeholder="Optional description"
                    :error="getLineError(index, 'description')"
                  />
                </div>
              </div>

              <div class="form-field full-width">
                <label :for="`line-recipe-${index}`" class="form-label">
                  Recipe
                  <span class="required-mark">*</span>
                </label>
                <RecipeAutocomplete
                  :id="`line-recipe-${index}`"
                  v-model="line.recipe"
                  :recipes="rawRecipes"
                  placeholder="Start typing to search recipes..."
                  :disabled="!rawRecipes.length"
                />
                <div v-if="getLineError(index, 'recipe')" class="error-message">
                  {{ getLineError(index, 'recipe') }}
                </div>
              </div>

              <!-- Machine Groups Table -->
              <div class="machine-groups">
                <div class="section-header">
                  <h4 class="subsection-title">Machine Groups</h4>
                  <Button variant="secondary" size="sm" @click="addMachineGroup(index)">
                    Add Group
                  </Button>
                </div>

                <div v-if="!line.machine_groups || line.machine_groups.length === 0" class="empty-state small">
                  No machine groups. Add at least one.
                </div>

                <table v-else class="groups-table">
                  <thead>
                    <tr>
                      <th class="col-number">#</th>
                      <th class="col-machines">Machines</th>
                      <th class="col-overclock">
                        <div class="header-with-icon">
                          <img src="/icons/Clock_speed.webp" alt="Clock Speed" class="header-icon" />
                          <span>Overclock (%)</span>
                        </div>
                      </th>
                      <th class="col-somersloop">
                        <div class="header-with-icon">
                          <img src="/icons/Somersloop.webp" alt="Somersloop" class="header-icon" />
                          <span>Somersloops</span>
                        </div>
                      </th>
                      <th class="col-actions"></th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="(group, groupIndex) in line.machine_groups" :key="groupIndex">
                      <td class="col-number">{{ groupIndex + 1 }}</td>
                      <td class="col-machines">
                        <FormNumber
                          :id="`group-machines-${index}-${groupIndex}`"
                          v-model="group.number_of_machine"
                          :min="1"
                          placeholder="1"
                          :error="getGroupError(index, groupIndex, 'number_of_machine')"
                          required
                        />
                      </td>
                      <td class="col-overclock">
                        <RangeSlider
                          :id="`group-oc-${index}-${groupIndex}`"
                          v-model="group.oc_value"
                          :min="0"
                          :max="250"
                          :step="1"
                          unit="%"
                          :presets="overclockPresets"
                          :show-quick-presets="false"
                          :error="getGroupError(index, groupIndex, 'oc_value')"
                          required
                          compact
                        />
                      </td>
                      <td class="col-somersloop">
                        <FormNumber
                          :id="`group-somersloop-${index}-${groupIndex}`"
                          v-model="group.somersloop"
                          :min="0"
                          :max="4"
                          placeholder="0"
                          :error="getGroupError(index, groupIndex, 'somersloop')"
                        />
                      </td>
                      <td class="col-actions">
                        <Button variant="ghost" size="sm" @click="confirmDeleteGroup(index, groupIndex)">
                          ×
                        </Button>
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>
          </Collapsible>
        </div>
      </div>

      <!-- Preview -->
      <div class="form-section preview">
        <h3 class="section-title">Summary</h3>
        <div class="stats-grid">
          <div class="stat-item">
            <span class="stat-label">Production Lines</span>
            <span class="stat-value">{{ formData.production_lines.length }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Total Machines</span>
            <span class="stat-value">{{ totalMachines }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Total Power</span>
            <span class="stat-value">{{ totalPower.toFixed(1) }} MW</span>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="modal-actions">
        <Button variant="secondary" @click="$emit('close')">Cancel</Button>
        <Button
          variant="primary"
          @click="onSave"
          :disabled="!isValid || loading"
          :loading="loading"
        >
          {{ template ? 'Save as New Version' : 'Create Blueprint' }}
        </Button>
      </div>
    </template>
  </Modal>

  <!-- Confirmation Dialog -->
  <ConfirmDialog
    :show="confirmDialog.show"
    :title="confirmDialog.title"
    :message="confirmDialog.message"
    :variant="confirmDialog.variant"
    :confirm-text="confirmDialog.confirmText"
    @confirm="handleConfirm"
    @cancel="closeConfirmDialog"
  />
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import type { BlueprintTemplateResponse, CreateBlueprintTemplateRequest, MachineGroupInfo, RecipeInfo } from '@/api/types';
import { gameData } from '@/api/endpoints';
import BaseInput from '@/components/forms/BaseInput.vue';
import RecipeAutocomplete from '@/components/factory/RecipeAutocomplete.vue';
import FormNumber from '@/components/forms/FormNumber.vue';
import RangeSlider from '@/components/forms/RangeSlider.vue';
import Button from '@/components/ui/Button.vue';
import Modal from '@/components/ui/Modal.vue';
import Collapsible from '@/components/ui/Collapsible.vue';
import ConfirmDialog from '@/components/ui/ConfirmDialog.vue';

interface Props {
  show: boolean;
  template?: BlueprintTemplateResponse | null;
}

interface Emits {
  (e: 'close'): void;
  (e: 'save', data: CreateBlueprintTemplateRequest): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// State
const loading = ref(false);
const rawRecipes = ref<RecipeInfo[]>([]);
const errors = ref<Record<string, string>>({});

// Confirmation dialog
const confirmDialog = ref({
  show: false,
  title: '',
  message: '',
  variant: 'danger' as 'primary' | 'secondary' | 'danger',
  confirmText: 'Delete',
  action: null as (() => void) | null,
});

// Overclock presets
const overclockPresets = [
  { value: 50, label: '50%' },
  { value: 100, label: '100%' },
  { value: 150, label: '150%' },
  { value: 200, label: '200%' },
  { value: 250, label: '250%' },
];

// Form data
interface FormData {
  name: string;
  description: string;
  production_lines: Array<{
    name: string;
    description: string;
    type: 'recipe' | 'blueprint';
    recipe: string;
    machine_groups?: MachineGroupInfo[];
  }>;
}

const formData = ref<FormData>({
  name: '',
  description: '',
  production_lines: [],
});

// Computed

const totalMachines = computed(() => {
  return formData.value.production_lines.reduce((total, line) => {
    return total + (line.machine_groups?.reduce((lineTotal, group) => {
      return lineTotal + (group.number_of_machine || 0);
    }, 0) || 0);
  }, 0);
});

const totalPower = computed(() => {
  return formData.value.production_lines.reduce((total, line) => {
    return total + ((line.machine_groups?.length || 0) * 16);
  }, 0);
});

const isValid = computed(() => {
  if (!formData.value.name.trim()) return false;
  if (formData.value.production_lines.length === 0) return false;

  return formData.value.production_lines.every(line => {
    if (!line.name.trim() || !line.recipe) return false;
    if (!line.machine_groups || line.machine_groups.length === 0) return false;

    return line.machine_groups.every(group => {
      return group.number_of_machine > 0 && group.oc_value >= 0 && group.oc_value <= 250;
    });
  });
});

// Methods
const loadRecipes = async () => {
  try {
    rawRecipes.value = await gameData.getRecipes();
  } catch (err) {
    console.error('Failed to load recipes:', err);
  }
};

const addProductionLine = () => {
  formData.value.production_lines.push({
    name: '',
    description: '',
    type: 'recipe' as const,
    recipe: '',
    machine_groups: [{
      number_of_machine: 1,
      oc_value: 100,
      somersloop: 0,
    }],
  });
};

const removeProductionLine = (index: number) => {
  formData.value.production_lines.splice(index, 1);
};

const addMachineGroup = (lineIndex: number) => {
  const line = formData.value.production_lines[lineIndex];
  if (!line) return;

  if (!line.machine_groups) {
    line.machine_groups = [];
  }
  line.machine_groups.push({
    number_of_machine: 1,
    oc_value: 100,
    somersloop: 0,
  });
};

const removeMachineGroup = (lineIndex: number, groupIndex: number) => {
  const line = formData.value.production_lines[lineIndex];
  if (!line || !line.machine_groups) return;

  line.machine_groups.splice(groupIndex, 1);
};

const getLineError = (lineIndex: number, field: string) => {
  const key = `lines.${lineIndex}.${field}`;
  return errors.value[key];
};

const getGroupError = (lineIndex: number, groupIndex: number, field: string) => {
  const key = `lines.${lineIndex}.groups.${groupIndex}.${field}`;
  return errors.value[key];
};

const getLineMachineCount = (line: FormData['production_lines'][0]): number => {
  if (!line.machine_groups) return 0;
  return line.machine_groups.reduce((total, group) => total + (group.number_of_machine || 0), 0);
};

const getLinePower = (line: FormData['production_lines'][0]): number => {
  if (!line.machine_groups) return 0;
  return line.machine_groups.length * 16;
};

const confirmDeleteLine = (index: number) => {
  const line = formData.value.production_lines[index];
  if (!line) return;

  confirmDialog.value = {
    show: true,
    title: 'Delete Production Line',
    message: `Are you sure you want to delete "${line.name || `Production Line ${index + 1}`}"?`,
    variant: 'danger',
    confirmText: 'Delete',
    action: () => removeProductionLine(index),
  };
};

const confirmDeleteGroup = (lineIndex: number, groupIndex: number) => {
  confirmDialog.value = {
    show: true,
    title: 'Delete Machine Group',
    message: `Are you sure you want to delete Machine Group ${groupIndex + 1}?`,
    variant: 'danger',
    confirmText: 'Delete',
    action: () => removeMachineGroup(lineIndex, groupIndex),
  };
};

const handleConfirm = () => {
  if (confirmDialog.value.action) {
    confirmDialog.value.action();
  }
  closeConfirmDialog();
};

const closeConfirmDialog = () => {
  confirmDialog.value = {
    show: false,
    title: '',
    message: '',
    variant: 'danger',
    confirmText: 'Delete',
    action: null,
  };
};

const onSave = async () => {
  if (!isValid.value) return;

  loading.value = true;
  errors.value = {};

  try {
    const requestData: CreateBlueprintTemplateRequest = {
      name: formData.value.name,
      description: formData.value.description || undefined,
      production_lines: formData.value.production_lines.map(line => ({
        name: line.name,
        description: line.description || undefined,
        type: line.type,
        recipe: line.recipe || undefined,
        machine_groups: line.machine_groups,
      })),
    };
    emit('save', requestData);
  } catch (err) {
    console.error('Failed to save blueprint:', err);
  } finally {
    loading.value = false;
  }
};

const initializeForm = () => {
  if (props.template) {
    formData.value = {
      name: props.template.name,
      description: props.template.description || '',
      production_lines: props.template.production_lines.map(line => ({
        name: line.name,
        description: line.description || '',
        type: 'recipe' as const,
        recipe: line.recipe,
        machine_groups: line.machine_groups.map(group => ({ ...group })),
      })),
    };
  } else {
    formData.value = {
      name: '',
      description: '',
      production_lines: [],
    };
  }
};

watch(() => props.show, (show) => {
  if (show) {
    initializeForm();
    errors.value = {};
  }
});

onMounted(() => {
  loadRecipes();
});
</script>

<style scoped>
.blueprint-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

/* Form Sections */
.form-section {
  padding: 1.5rem;
  background: var(--color-surface, #252525);
  border: 1px solid var(--color-border, #404040);
  border-radius: 0.5rem;
  margin-bottom: 0;
  overflow: visible;
  min-height: fit-content;
}

.form-section.preview {
  background: var(--color-surface, #252525);
  border-color: var(--color-border, #404040);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.75rem;
}

.section-title {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--color-text-primary, #e5e5e5);
  margin: 0 0 0.75rem 0;
}

.subsection-title {
  font-size: 0.8125rem;
  font-weight: 600;
  color: var(--color-text-secondary, #b8b8b8);
  margin: 0;
}

/* Form Layout */
.form-row {
  display: flex;
  gap: 1.5rem;
  margin-bottom: 0;
  align-items: flex-start;
}

.form-row > .form-field {
  flex: 1 1 0;
  min-width: 0;
  max-width: 100%;
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 0;
  min-height: fit-content;
  width: 100%;
  box-sizing: border-box;
}

.form-field.full-width {
  grid-column: 1 / -1;
}

.form-field label,
.form-field .form-label {
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--color-text-secondary, #b8b8b8);
  margin-bottom: 0.5rem;
}

.form-label .required-mark {
  color: var(--color-error, #ef4444);
  margin-left: 0.25rem;
}

.form-field .error-message {
  font-size: 0.75rem;
  color: var(--color-error, #ef4444);
  margin-top: 0.25rem;
}

/* Recipe Autocomplete styling to match form inputs */
.form-field :deep(.recipe-autocomplete .autocomplete-input) {
  padding: 0.625rem 0.875rem;
  padding-right: 2.25rem;
  font-size: 0.875rem;
  background-color: var(--color-surface-inset, #1f1f1f);
  border: 1px solid var(--color-border, #404040);
  border-radius: 6px;
  color: var(--color-text-primary, #e5e5e5);
  transition: border-color 0.2s, box-shadow 0.2s;
}

.form-field :deep(.recipe-autocomplete .autocomplete-input:focus) {
  border-color: var(--color-ficsit-orange, #f58b00);
  box-shadow: 0 0 0 3px rgba(245, 139, 0, 0.1);
}

.form-field :deep(.recipe-autocomplete .autocomplete-input::placeholder) {
  color: var(--color-text-muted, #8a8a8a);
}

/* Production Lines */
.production-lines {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.line-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

.line-number {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 1.25rem;
  height: 1.25rem;
  background: rgba(59, 130, 246, 0.2);
  color: #93c5fd;
  border-radius: 50%;
  font-size: 0.75rem;
  font-weight: 700;
  flex-shrink: 0;
}

.line-name {
  font-weight: 500;
  font-size: 0.875rem;
  color: var(--color-text-primary, #e5e5e5);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.line-recipe {
  font-size: 0.75rem;
  color: var(--color-text-muted, #8a8a8a);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.line-stats {
  display: inline-flex !important;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.75rem;
  flex-wrap: nowrap !important;
  white-space: nowrap !important;
  flex-shrink: 0;
}

.line-stats .stat {
  padding: 0.125rem 0.5rem;
  background: rgba(59, 130, 246, 0.15);
  color: #93c5fd;
  border-radius: 0.25rem;
  white-space: nowrap;
}

.line-content {
  padding: 0.75rem;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

/* Machine Groups */
.machine-groups {
  margin-top: 0.75rem;
}

.groups-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.875rem;
  table-layout: fixed;
}

.groups-table thead {
  background: var(--color-surface-inset, #1f1f1f);
  border-bottom: 2px solid var(--color-border, #404040);
}

.groups-table th {
  padding: 0.75rem 0.5rem;
  text-align: left;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--color-text-muted, #8a8a8a);
  text-transform: uppercase;
  vertical-align: middle;
}

.header-with-icon {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.header-icon {
  width: 1rem;
  height: 1rem;
  object-fit: contain;
  flex-shrink: 0;
}

.groups-table tbody tr {
  border-bottom: 1px solid var(--color-border, #404040);
}

.groups-table tbody tr:hover {
  background: var(--color-surface-hover, #2a2a2a);
}

.groups-table td {
  padding: 0.75rem 0.5rem;
  vertical-align: top;
}

.col-number {
  width: 3rem;
  text-align: center;
  font-weight: 600;
  color: var(--color-text-primary, #e5e5e5);
}

.col-machines {
  width: 6rem;
}

.col-machines .form-number {
  max-width: 100%;
}

.col-overclock {
  width: auto;
  min-width: 18rem;
}

.col-somersloop {
  width: 5rem;
}

.col-somersloop .form-number {
  max-width: 100%;
}

.col-machines .number-input,
.col-somersloop .number-input {
  min-width: 0;
}

.col-actions {
  width: 3rem;
  text-align: center;
}

/* Empty States */
.empty-state {
  padding: 2rem;
  text-align: center;
  color: var(--color-text-muted, #8a8a8a);
  font-size: 0.875rem;
  background: var(--color-surface-inset, #1f1f1f);
  border: 2px dashed var(--color-border, #404040);
  border-radius: 0.5rem;
}

.empty-state.small {
  padding: 1rem;
  font-size: 0.8125rem;
}

/* Stats Grid */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 0.75rem;
}

.stat-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  padding: 0.75rem;
  background: var(--color-surface-inset, #1f1f1f);
  border: 1px solid var(--color-border, #404040);
  border-radius: 0.375rem;
}

.stat-label {
  font-size: 0.75rem;
  color: var(--color-text-muted, #8a8a8a);
  font-weight: 500;
}

.stat-value {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--color-text-primary, #e5e5e5);
}

/* Modal Actions */
.modal-actions {
  display: flex;
  gap: 0.75rem;
  justify-content: flex-end;
}

/* Responsive */
@media (max-width: 768px) {
  .form-row {
    grid-template-columns: 1fr;
  }

  .stats-grid {
    grid-template-columns: 1fr;
  }
}
</style>
