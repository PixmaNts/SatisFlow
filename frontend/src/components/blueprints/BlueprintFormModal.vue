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
                <label :for="`line-recipe-${index}`">Recipe *</label>
                <SearchableSelect
                  :id="`line-recipe-${index}`"
                  v-model="line.recipe"
                  :options="recipeOptions"
                  placeholder="Search and select a recipe..."
                  :error="getLineError(index, 'recipe')"
                  required
                />
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
                      <th>#</th>
                      <th>Machines</th>
                      <th>Overclock (%)</th>
                      <th>Somersloops</th>
                      <th></th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="(group, groupIndex) in line.machine_groups" :key="groupIndex">
                      <td class="group-number">{{ groupIndex + 1 }}</td>
                      <td class="group-cell">
                        <FormNumber
                          :id="`group-machines-${index}-${groupIndex}`"
                          v-model="group.number_of_machine"
                          :min="1"
                          placeholder="1"
                          :error="getGroupError(index, groupIndex, 'number_of_machine')"
                          required
                        />
                      </td>
                      <td class="group-cell">
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
                        />
                      </td>
                      <td class="group-cell">
                        <FormNumber
                          :id="`group-somersloop-${index}-${groupIndex}`"
                          v-model="group.somersloop"
                          :min="0"
                          :max="4"
                          placeholder="0"
                          :error="getGroupError(index, groupIndex, 'somersloop')"
                        />
                      </td>
                      <td class="group-actions">
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
import type { BlueprintTemplateResponse, CreateBlueprintTemplateRequest, MachineGroupInfo } from '@/api/types';
import { gameData } from '@/api/endpoints';
import BaseInput from '@/components/forms/BaseInput.vue';
import SearchableSelect from '@/components/forms/SearchableSelect.vue';
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
const recipes = ref<Array<{ value: string; label: string }>>([]);
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
const recipeOptions = computed(() => recipes.value);

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
    const recipeData = await gameData.getRecipes();
    recipes.value = recipeData.map(recipe => ({
      value: recipe.name,
      label: `${recipe.name} (${recipe.machine})`,
    }));
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
  gap: 1rem;
}

/* Form Sections */
.form-section {
  padding: 1rem;
  background: #f9fafb;
  border: 1px solid #e5e7eb;
  border-radius: 0.5rem;
}

:global(.dark) .form-section {
  background: #1f2937;
  border-color: #374151;
}

.form-section.preview {
  background: #f0fdf4;
  border-color: #bbf7d0;
}

:global(.dark) .form-section.preview {
  background: #064e3b;
  border-color: #065f46;
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
  color: #111827;
  margin: 0 0 0.75rem 0;
}

:global(.dark) .section-title {
  color: #f9fafb;
}

.subsection-title {
  font-size: 0.8125rem;
  font-weight: 600;
  color: #374151;
  margin: 0;
}

:global(.dark) .subsection-title {
  color: #d1d5db;
}

/* Form Layout */
.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.75rem;
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.form-field.full-width {
  grid-column: 1 / -1;
}

.form-field label {
  font-size: 0.8125rem;
  font-weight: 500;
  color: #374151;
}

:global(.dark) .form-field label {
  color: #d1d5db;
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
}

.line-number {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 1.25rem;
  height: 1.25rem;
  background: #dbeafe;
  color: #1e40af;
  border-radius: 50%;
  font-size: 0.75rem;
  font-weight: 700;
  flex-shrink: 0;
}

:global(.dark) .line-number {
  background: #1e3a8a;
  color: #93c5fd;
}

.line-name {
  font-weight: 500;
  font-size: 0.875rem;
  color: #111827;
}

:global(.dark) .line-name {
  color: #f9fafb;
}

.line-recipe {
  font-size: 0.75rem;
  color: #6b7280;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

:global(.dark) .line-recipe {
  color: #9ca3af;
}

.line-stats {
  display: flex;
  gap: 0.5rem;
  font-size: 0.75rem;
}

.line-stats .stat {
  padding: 0.125rem 0.5rem;
  background: #e0e7ff;
  color: #3730a3;
  border-radius: 0.25rem;
  white-space: nowrap;
}

:global(.dark) .line-stats .stat {
  background: #312e81;
  color: #c7d2fe;
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
}

.groups-table thead {
  background: #f3f4f6;
  border-bottom: 2px solid #e5e7eb;
}

:global(.dark) .groups-table thead {
  background: #374151;
  border-color: #4b5563;
}

.groups-table th {
  padding: 0.5rem;
  text-align: left;
  font-size: 0.75rem;
  font-weight: 600;
  color: #6b7280;
  text-transform: uppercase;
}

:global(.dark) .groups-table th {
  color: #9ca3af;
}

.groups-table tbody tr {
  border-bottom: 1px solid #e5e7eb;
}

:global(.dark) .groups-table tbody tr {
  border-color: #374151;
}

.groups-table tbody tr:hover {
  background: #f9fafb;
}

:global(.dark) .groups-table tbody tr:hover {
  background: #1f2937;
}

.groups-table td {
  padding: 0.5rem;
}

.group-number {
  font-weight: 600;
  color: #111827;
  width: 2rem;
}

:global(.dark) .group-number {
  color: #f9fafb;
}

.group-cell {
  min-width: 8rem;
}

.group-actions {
  text-align: right;
  width: 3rem;
}

/* Empty States */
.empty-state {
  padding: 2rem;
  text-align: center;
  color: #6b7280;
  font-size: 0.875rem;
  background: #fff;
  border: 2px dashed #d1d5db;
  border-radius: 0.5rem;
}

:global(.dark) .empty-state {
  background: #111827;
  border-color: #4b5563;
  color: #9ca3af;
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
  background: #fff;
  border: 1px solid #d1d5db;
  border-radius: 0.375rem;
}

:global(.dark) .stat-item {
  background: #1f2937;
  border-color: #374151;
}

.stat-label {
  font-size: 0.75rem;
  color: #6b7280;
  font-weight: 500;
}

:global(.dark) .stat-label {
  color: #9ca3af;
}

.stat-value {
  font-size: 1.25rem;
  font-weight: 700;
  color: #111827;
}

:global(.dark) .stat-value {
  color: #f9fafb;
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
