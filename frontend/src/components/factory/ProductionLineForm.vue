<template>
  <Modal
    :show="show"
    :title="isEditing ? 'Edit Production Line' : 'Create Production Line'"
    @close="handleClose"
  >
    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="line-name" class="form-label">Production Line Name *</label>
        <input
          id="line-name"
          v-model="formData.name"
          type="text"
          class="form-input"
          placeholder="Enter production line name..."
          required
        />
      </div>

      <div class="form-group">
        <label for="line-description" class="form-label">Description</label>
        <textarea
          id="line-description"
          v-model="formData.description"
          class="form-textarea"
          placeholder="Enter production line description..."
          rows="2"
        />
      </div>

      <div class="form-group">
        <label class="form-label">Production Type *</label>
        <div class="radio-group">
          <label class="radio-option">
            <input
              v-model="formData.type"
              type="radio"
              value="recipe"
              @change="handleTypeChange"
            />
            <span>Recipe</span>
          </label>
          <label class="radio-option">
            <input
              v-model="formData.type"
              type="radio"
              value="blueprint"
              @change="handleTypeChange"
            />
            <span>Blueprint</span>
          </label>
        </div>
      </div>

      <!-- Recipe Selection -->
      <div v-if="formData.type === 'recipe'" class="form-group">
        <label for="recipe-autocomplete" class="form-label">Recipe *</label>
        <RecipeAutocomplete
          id="recipe-autocomplete"
          v-model="formData.recipe"
          :recipes="sortedRecipes"
          placeholder="Start typing to search recipes..."
          :disabled="!sortedRecipes.length"
          @selected="handleRecipeSelected"
          @cleared="handleRecipeCleared"
        />
      </div>

      <!-- Machine Groups -->
      <!-- Show machine groups if recipe is selected OR if we're editing with machine groups (recipes might not be loaded yet) -->
      <div v-if="formData.type === 'recipe' && (selectedRecipe || (isEditing && formData.machine_groups.length > 0))" class="form-group">
        <label class="form-label">Machine Groups</label>
        <div class="machine-groups">
          <div
            v-for="(group, index) in formData.machine_groups"
            :key="index"
            class="machine-group"
          >
            <div class="group-header">
              <h4>Group {{ index + 1 }}</h4>
              <Button
                v-if="formData.machine_groups!.length > 1"
                variant="danger"
                size="sm"
                @click="removeMachineGroup(index)"
              >
                Remove
              </Button>
            </div>

            <div class="group-fields">
              <div class="field-row">
                <div class="field">
                  <label :for="`machines-${index}`" class="field-label">Number of Machines</label>
                  <input
                    :id="`machines-${index}`"
                    v-model.number="group.number_of_machine"
                    type="number"
                    min="1"
                    class="form-input"
                    required
                  />
                </div>

                <div class="field">
                  <label :for="`oc-${index}`" class="field-label">
                    <div class="label-with-icon">
                      <img src="/icons/Clock_speed.webp" alt="Clock Speed" class="label-icon" />
                      <span>Overclock (%)</span>
                    </div>
                  </label>
                  <input
                    :id="`oc-${index}`"
                    v-model.number="group.oc_value"
                    type="number"
                    min="0"
                    max="250"
                    step="0.1"
                    class="form-input"
                    required
                  />
                </div>

                <div v-if="selectedMachineInfo" class="field">
                  <label :for="`somersloop-${index}`" class="field-label">
                    <div class="label-with-icon">
                      <img src="/icons/Somersloop.webp" alt="Somersloop" class="label-icon" />
                      <span>Somersloops (max: {{ selectedMachineInfo.max_somersloop }})</span>
                    </div>
                  </label>
                  <input
                    :id="`somersloop-${index}`"
                    v-model.number="group.somersloop"
                    type="number"
                    min="0"
                    :max="selectedMachineInfo.max_somersloop"
                    class="form-input"
                  />
                </div>
              </div>
            </div>
          </div>

          <Button
            variant="secondary"
            size="sm"
            @click="addMachineGroup"
          >
            Add Machine Group
          </Button>
        </div>

        <!-- Power Calculation Preview -->
        <div v-if="calculatedPower > 0 || previewLoading" class="power-preview">
          <div v-if="previewLoading" class="preview-item">
            <span class="preview-label">Calculating...</span>
            <span class="preview-value">Loading...</span>
          </div>
          <div v-else>
            <div class="preview-item">
              <span class="preview-label">Total Power:</span>
              <span class="preview-value">{{ formatPower(calculatedPower) }}</span>
            </div>
            <div class="preview-item">
              <span class="preview-label">Total Machines:</span>
              <span class="preview-value">{{ totalMachines }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Blueprint Selection -->
      <div v-if="formData.type === 'blueprint'" class="form-group">
        <label for="blueprint-select" class="form-label">Blueprint Template *</label>
        <SearchableSelect
          id="blueprint-select"
          v-model="formData.blueprint_template_id"
          :options="blueprintOptions"
          placeholder="Search for a blueprint template..."
          :disabled="!blueprintTemplates_list.length || loadingTemplates"
        />
        <p v-if="loadingTemplates" class="helper-text">Loading blueprint templates...</p>
        <p v-else-if="!blueprintTemplates_list.length" class="helper-text">
          No blueprint templates available. Create one in the Blueprint Library first.
        </p>
      </div>

      <div class="form-actions">
        <Button
          type="button"
          variant="secondary"
          @click="handleClose"
        >
          Cancel
        </Button>
        <Button
          type="submit"
          variant="primary"
          :loading="saving"
          :disabled="!canSubmit"
        >
          {{ isEditing ? 'Update' : 'Create' }} Production Line
        </Button>
      </div>
    </form>
  </Modal>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick } from 'vue'
import { storeToRefs } from 'pinia'
import { useGameDataStore } from '@/stores/gameData'
import { useFactoryStore } from '@/stores/factory'
import { factories, blueprintTemplates } from '@/api/endpoints'
import type {
  ProductionLineResponse,
  MachineGroup,
  CreateProductionLineRequest,
  RecipeInfo,
  BlueprintTemplateResponse,
  ProductionLinePreviewResponse
} from '@/api/types'

interface BlueprintOption {
  value: string;
  label: string;
  category?: string;
}
import Button from '@/components/ui/Button.vue'
import Modal from '@/components/ui/Modal.vue'
import RecipeAutocomplete from './RecipeAutocomplete.vue'
import SearchableSelect from '@/components/forms/SearchableSelect.vue'

interface Props {
  show: boolean
  factoryId: string
  productionLine?: ProductionLineResponse | null
}

interface Emits {
  (e: 'update:show', value: boolean): void
  (e: 'saved'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const gameDataStore = useGameDataStore()
const factoryStore = useFactoryStore()
const { recipes } = storeToRefs(gameDataStore)

// State
const saving = ref(false)
const loadingTemplates = ref(false)
const previewLoading = ref(false)
const previewData = ref<ProductionLinePreviewResponse | null>(null)
const blueprintTemplates_list = ref<BlueprintTemplateResponse[]>([])
const formData = ref({
  name: '',
  description: '',
  type: 'recipe' as 'recipe' | 'blueprint',
  recipe: '',
  blueprint_template_id: '',
  machine_groups: [] as MachineGroup[]
})

// Computed
const isEditing = computed(() => !!props.productionLine)
const sortedRecipes = computed((): RecipeInfo[] => {
  if (!recipes.value.length) return []
  return [...recipes.value].sort((a, b) => a.name.localeCompare(b.name))
})
const selectedRecipe = computed(() => {
  if (!formData.value.recipe) return null
  return gameDataStore.getRecipeByName(formData.value.recipe)
})
const selectedMachineInfo = computed(() => {
  if (!selectedRecipe.value) return null
  return gameDataStore.getMachineByName(selectedRecipe.value.machine)
})

const calculatedPower = computed(() => {
  return previewData.value?.total_power_consumption || 0
})

const totalMachines = computed(() => {
  return previewData.value?.total_machines || formData.value.machine_groups.reduce((total, group) => total + group.number_of_machine, 0)
})

const blueprintOptions = computed((): BlueprintOption[] => {
  return blueprintTemplates_list.value.map(template => ({
    value: template.id,
    label: template.name,
    category: template.description || undefined
  }))
})

const canSubmit = computed(() => {
  if (!formData.value.name.trim()) return false

  if (formData.value.type === 'recipe') {
    return (
      formData.value.recipe &&
      formData.value.machine_groups.length > 0 &&
      formData.value.machine_groups.every(group =>
        group.number_of_machine > 0 &&
        group.oc_value >= 0 &&
        group.oc_value <= 250
      )
    )
  } else if (formData.value.type === 'blueprint') {
    return !!formData.value.blueprint_template_id
  }

  return false
})

// Preview calculation
const fetchPreview = async () => {
  if (!props.factoryId || !canSubmit.value || formData.value.type !== 'recipe') {
    previewData.value = null
    return
  }

  previewLoading.value = true
  try {
    const request = {
      name: formData.value.name.trim(),
      description: formData.value.description?.trim() || undefined,
      type: formData.value.type,
      recipe: formData.value.recipe,
      machine_groups: formData.value.machine_groups.map(group => ({
        number_of_machine: group.number_of_machine,
        oc_value: group.oc_value,
        somersloop: group.somersloop
      }))
    }

    previewData.value = await factories.preview.productionLine(props.factoryId, request)
  } catch (error) {
    console.error('Failed to fetch production line preview:', error)
    previewData.value = null
  } finally {
    previewLoading.value = false
  }
}

const getProductionLineId = (line?: ProductionLineResponse | null): string | null => {
  if (!line) return null
  if ('ProductionLineRecipe' in line) {
    return line.ProductionLineRecipe.id
  }
  if ('ProductionLineBlueprint' in line) {
    return line.ProductionLineBlueprint.id
  }
  return null
}

// Methods
const handleClose = () => {
  emit('update:show', false)
  resetForm()
}

const lastSelectedRecipe = ref<string | null>(null)

const handleTypeChange = () => {
  if (formData.value.type === 'blueprint') {
    // Reset recipe-related fields
    formData.value.recipe = ''
    formData.value.machine_groups = []
    lastSelectedRecipe.value = null
  } else if (formData.value.type === 'recipe') {
    // Reset blueprint-related fields
    formData.value.blueprint_template_id = ''
  }
}

const handleRecipeSelected = (recipe: RecipeInfo) => {
  // Always update lastSelectedRecipe to ensure recipe is recognized
  // This is important for the selectedRecipe computed to work correctly
  lastSelectedRecipe.value = recipe.name
  
  // Don't reset machine groups if this is the same recipe and groups already exist
  // This prevents losing machine group data when editing
  if (lastSelectedRecipe.value === recipe.name && formData.value.machine_groups.length > 0) {
    // Recipe is already selected and has groups, just ensure it's recognized
    return
  }

  // Only create default machine group if we don't have any groups yet
  if (formData.value.machine_groups.length === 0) {
    formData.value.machine_groups = [createDefaultMachineGroup()]
  }
}

const handleRecipeCleared = () => {
  lastSelectedRecipe.value = null
  if (formData.value.machine_groups.length) {
    formData.value.machine_groups = []
  }
}

const createDefaultMachineGroup = (): MachineGroup => {
  return {
    number_of_machine: 1,
    oc_value: 100,
    somersloop: 0
  }
}

const addMachineGroup = () => {
  formData.value.machine_groups.push(createDefaultMachineGroup())
}

const removeMachineGroup = (index: number) => {
  formData.value.machine_groups.splice(index, 1)
}

const formatPower = (power: number): string => {
  if (power < 1) {
    return `${(power * 1000).toFixed(0)} kW`
  }
  return `${power.toFixed(1)} MW`
}

const resetForm = () => {
  formData.value = {
    name: '',
    description: '',
    type: 'recipe',
    recipe: '',
    blueprint_template_id: '',
    machine_groups: []
  }
  lastSelectedRecipe.value = null
}

const fetchBlueprintTemplates = async () => {
  loadingTemplates.value = true
  try {
    blueprintTemplates_list.value = await blueprintTemplates.getAll()
  } catch (error) {
    console.error('Failed to fetch blueprint templates:', error)
    blueprintTemplates_list.value = []
  } finally {
    loadingTemplates.value = false
  }
}

const loadProductionLine = () => {
  if (!props.productionLine) return

  // Handle both recipe and blueprint types
  if ('ProductionLineRecipe' in props.productionLine) {
    const recipe = props.productionLine.ProductionLineRecipe
    formData.value = {
      name: recipe.name,
      description: recipe.description || '',
      type: 'recipe',
      recipe: recipe.recipe,
      blueprint_template_id: '',
      machine_groups: [...recipe.machine_groups]
    }
    // Don't set lastSelectedRecipe here - let handleRecipeSelected do it
    // This ensures the recipe is properly recognized
    
    // Manually trigger recipe selection to ensure it's recognized
    // Use nextTick to ensure recipes are loaded first
    // The watch on formData will handle fetching the preview once everything is ready
    nextTick(() => {
      const recipeInfo = gameDataStore.getRecipeByName(recipe.recipe)
      if (recipeInfo) {
        // Ensure recipe is selected even if machine groups exist
        handleRecipeSelected(recipeInfo)
      }
    })
  } else if ('ProductionLineBlueprint' in props.productionLine) {
    const blueprint = props.productionLine.ProductionLineBlueprint
    formData.value = {
      name: blueprint.name,
      description: blueprint.description || '',
      type: 'blueprint',
      recipe: '',
      blueprint_template_id: '',
      machine_groups: []
    }
    lastSelectedRecipe.value = null
  }
}

const handleSubmit = async () => {
  if (!canSubmit.value) return

  saving.value = true

  try {
    // For blueprint type, use the template creation endpoint
    if (formData.value.type === 'blueprint') {
      await blueprintTemplates.createInstanceInFactory(
        props.factoryId,
        formData.value.blueprint_template_id,
        {
          name: formData.value.name.trim() || undefined,
        }
      )
      emit('saved')
      handleClose()
      return
    }

    // For recipe type, use the standard production line endpoint
    const payload: CreateProductionLineRequest = {
      name: formData.value.name.trim(),
      description: formData.value.description?.trim() || undefined,
      type: formData.value.type,
      recipe: formData.value.recipe,
      machine_groups: formData.value.machine_groups.map(group => ({
        number_of_machine: group.number_of_machine,
        oc_value: group.oc_value,
        somersloop: group.somersloop
      }))
    }

    let response = null
    if (isEditing.value && props.productionLine) {
      const lineId = getProductionLineId(props.productionLine)
      if (!lineId) {
        throw new Error('Unable to determine production line identifier')
      }
      response = await factoryStore.updateProductionLine(props.factoryId, lineId, payload)
    } else {
      response = await factoryStore.createProductionLine(props.factoryId, payload)
    }

    if (response) {
      emit('saved')
      handleClose()
    }
  } catch (error) {
    console.error('Failed to save production line:', error)
  } finally {
    saving.value = false
  }
}

// Watch for production line changes
watch(() => props.productionLine, () => {
  if (props.show) {
    loadProductionLine()
  }
}, { immediate: true })

// Watch for show changes
watch(() => props.show, (show) => {
  if (show) {
    loadProductionLine()
    // Preview will be fetched by the watch on formData when form is valid
  } else {
    resetForm()
  }
})

// Watch for form data changes to update preview
watch(
  () => [formData.value.recipe, formData.value.machine_groups],
  () => {
    // Only fetch preview if:
    // 1. Type is recipe
    // 2. Recipe is set
    // 3. Machine groups exist and are valid
    // 4. Form can submit (validates all required fields)
    if (
      formData.value.type === 'recipe' &&
      formData.value.recipe &&
      formData.value.machine_groups.length > 0 &&
      canSubmit.value
    ) {
      fetchPreview()
    } else {
      // Clear preview if form is invalid
      previewData.value = null
    }
  },
  { deep: true }
)

// Watch for recipes to become available and ensure recipe is selected when editing
watch(
  () => [recipes.value.length, formData.value.recipe, props.productionLine],
  ([recipeCount, recipeValue, productionLine]) => {
    // If we're editing and have a recipe but it's not selected yet, select it
    if (productionLine && recipeValue && recipeCount > 0) {
      const recipeInfo = gameDataStore.getRecipeByName(recipeValue)
      if (recipeInfo) {
        // Always ensure recipe is selected when recipes become available
        // This handles the case where recipes load after the form
        if (lastSelectedRecipe.value !== recipeInfo.name) {
          handleRecipeSelected(recipeInfo)
        }
      }
    }
  },
  { immediate: true }
)

// Load recipes and blueprint templates on mount
onMounted(async () => {
  await Promise.all([
    gameDataStore.fetchRecipes(),
    fetchBlueprintTemplates()
  ])
})
</script>

<style scoped lang="scss">
.form-group {
  margin-bottom: var(--spacing-md, 0.75rem);
}

.form-label {
  display: block;
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-700, #374151);
  margin-bottom: var(--spacing-xs, 0.25rem);
}

.form-input,
.form-select,
.form-textarea {
  width: 100%;
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-md, 0.375rem);
  font-size: var(--font-size-base, 1rem);
  transition: border-color 0.2s ease-in-out;

  &:focus {
    outline: none;
    border-color: var(--color-primary-500, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  &::placeholder {
    color: var(--color-gray-400, #9ca3af);
  }
}

.form-textarea {
  resize: vertical;
  min-height: 60px;
}

.radio-group {
  display: flex;
  gap: var(--spacing-md, 0.75rem);
}

.radio-option {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
  cursor: pointer;
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-gray-700, #374151);

  input[type="radio"] {
    margin: 0;
  }
}

.machine-groups {
  border: 1px solid var(--color-gray-200, #e5e7eb);
  border-radius: var(--border-radius-md, 0.375rem);
  padding: var(--spacing-md, 0.75rem);
  background-color: var(--color-gray-50, #f9fafb);
}

.machine-group {
  margin-bottom: var(--spacing-md, 0.75rem);
  padding: var(--spacing-md, 0.75rem);
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-md, 0.375rem);
  border: 1px solid var(--color-gray-200, #e5e7eb);

  &:last-child {
    margin-bottom: 0;
  }
}

.group-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-sm, 0.5rem);

  h4 {
    margin: 0;
    font-size: var(--font-size-sm, 0.875rem);
    font-weight: var(--font-weight-medium, 500);
    color: var(--color-gray-900, #111827);
  }
}

.group-fields {
  margin-top: var(--spacing-sm, 0.5rem);
}

.field-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: var(--spacing-md, 0.75rem);
}

.field {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
}

.field-label {
  font-size: var(--font-size-xs, 0.75rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-600, #4b5563);
}

.label-with-icon {
  display: flex;
  align-items: center;
  gap: 0.375rem;
}

.label-icon {
  width: 1rem;
  height: 1rem;
  object-fit: contain;
  flex-shrink: 0;
}

.power-preview {
  margin-top: var(--spacing-md, 0.75rem);
  padding: var(--spacing-md, 0.75rem);
  background-color: var(--color-blue-50, #eff6ff);
  border-radius: var(--border-radius-md, 0.375rem);
  border: 1px solid var(--color-blue-200, #dbeafe);
}

.preview-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-xs, 0.25rem);

  &:last-child {
    margin-bottom: 0;
  }
}

.preview-label {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-gray-600, #4b5563);
}

.preview-value {
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-900, #111827);
}

.blueprint-notice {
  padding: var(--spacing-md, 0.75rem);
  background-color: var(--color-amber-50, #fffbeb);
  border-radius: var(--border-radius-md, 0.375rem);
  border: 1px solid var(--color-amber-200, #fde68a);
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-amber-800, #92400e);
  margin: 0;
}

.helper-text {
  margin-top: var(--spacing-xs, 0.25rem);
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-500, #6b7280);
  font-style: italic;
}

.form-actions {
  display: flex;
  gap: var(--spacing-sm, 0.5rem);
  justify-content: flex-end;
  margin-top: var(--spacing-lg, 1rem);
  padding-top: var(--spacing-lg, 1rem);
  border-top: 1px solid var(--color-gray-200, #e5e7eb);
}

// Responsive design
@media (max-width: 640px) {
  .radio-group {
    flex-direction: column;
    gap: var(--spacing-sm, 0.5rem);
  }

  .field-row {
    grid-template-columns: 1fr;
  }

  .group-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-sm, 0.5rem);
  }

  .form-actions {
    flex-direction: column;
  }

  .form-actions button {
    width: 100%;
  }
}
</style>
