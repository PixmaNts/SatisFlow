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
        <label for="recipe-select" class="form-label">Recipe *</label>
        <select
          id="recipe-select"
          v-model="formData.recipe"
          class="form-select"
          required
          @change="handleRecipeChange"
        >
          <option value="">Select a recipe...</option>
          <option
            v-for="recipe in recipes"
            :key="recipe.name"
            :value="recipe.name"
          >
            {{ recipe.name }} ({{ recipe.machine }})
          </option>
        </select>
      </div>

      <!-- Machine Groups -->
      <div v-if="formData.type === 'recipe' && selectedRecipe" class="form-group">
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
                  <label :for="`oc-${index}`" class="field-label">Overclock (%)</label>
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
                    Somersloops (max: {{ selectedMachineInfo.max_sommersloop }})
                  </label>
                  <input
                    :id="`somersloop-${index}`"
                    v-model.number="group.somersloop"
                    type="number"
                    min="0"
                    :max="selectedMachineInfo.max_sommersloop"
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
        <div v-if="calculatedPower > 0" class="power-preview">
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

      <!-- Blueprint (simplified for now) -->
      <div v-if="formData.type === 'blueprint'" class="form-group">
        <p class="blueprint-notice">
          Blueprint creation will be implemented in a future update.
          For now, please use the Recipe type.
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
import { ref, computed, watch, onMounted } from 'vue'
import { useGameDataStore } from '@/stores/gameData'
import type { ProductionLineResponse, MachineGroup } from '@/api/types'
import Button from '@/components/ui/Button.vue'
import Modal from '@/components/ui/Modal.vue'

interface Props {
  show: boolean
  factoryId: number
  productionLine?: ProductionLineResponse | null
}

interface Emits {
  (e: 'update:show', value: boolean): void
  (e: 'saved'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const gameDataStore = useGameDataStore()

// State
const saving = ref(false)
const formData = ref({
  name: '',
  description: '',
  type: 'recipe' as 'recipe' | 'blueprint',
  recipe: '',
  machine_groups: [] as MachineGroup[]
})

// Computed
const isEditing = computed(() => !!props.productionLine)
const recipes = computed(() => gameDataStore.recipes)
const selectedRecipe = computed(() => {
  if (!formData.value.recipe) return null
  return gameDataStore.getRecipeByName(formData.value.recipe)
})
const selectedMachineInfo = computed(() => {
  if (!selectedRecipe.value) return null
  return gameDataStore.getMachineByName(selectedRecipe.value.machine)
})

const calculatedPower = computed(() => {
  if (!selectedMachineInfo.value || !formData.value.machine_groups.length) {
    return 0
  }

  return formData.value.machine_groups.reduce((total, group) => {
    const basePower = selectedMachineInfo.value!.base_power
    const clockSpeed = group.oc_value / 100
    const powerMultiplier = Math.pow(clockSpeed, 1.321928)
    const somersloopMultiplier = Math.pow(1 + group.somersloop / selectedMachineInfo.value!.max_sommersloop, 2)

    return total + (group.number_of_machine * basePower * powerMultiplier * somersloopMultiplier)
  }, 0)
})

const totalMachines = computed(() => {
  return formData.value.machine_groups.reduce((total, group) => total + group.number_of_machine, 0)
})

const canSubmit = computed(() => {
  return (
    formData.value.name.trim() &&
    formData.value.type === 'recipe' &&
    formData.value.recipe &&
    formData.value.machine_groups.length > 0 &&
    formData.value.machine_groups.every(group =>
      group.number_of_machine > 0 &&
      group.oc_value >= 0 &&
      group.oc_value <= 250
    )
  )
})

// Methods
const handleClose = () => {
  emit('update:show', false)
  resetForm()
}

const handleTypeChange = () => {
  if (formData.value.type === 'blueprint') {
    // Reset recipe-related fields
    formData.value.recipe = ''
    formData.value.machine_groups = []
  }
}

const handleRecipeChange = () => {
  // Reset machine groups when recipe changes
  formData.value.machine_groups = [createDefaultMachineGroup()]
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
    machine_groups: []
  }
}

const loadProductionLine = () => {
  if (!props.productionLine) return

  // Handle both recipe and blueprint types
  if ('ProductionLineRecipe' in props.productionLine) {
    const recipe = props.productionLine.ProductionLineRecipe
    formData.value = {
      name: recipe.name,
      description: recipe.description,
      type: 'recipe',
      recipe: recipe.recipe,
      machine_groups: [...recipe.machine_groups]
    }
  } else if ('ProductionLineBlueprint' in props.productionLine) {
    const blueprint = props.productionLine.ProductionLineBlueprint
    formData.value = {
      name: blueprint.name,
      description: blueprint.description,
      type: 'blueprint',
      recipe: '',
      machine_groups: []
    }
  }
}

const handleSubmit = async () => {
  if (!canSubmit.value) return

  saving.value = true

  try {
    // TODO: Implement API call to save production line
    console.log('Saving production line:', {
      factory_id: props.factoryId,
      ...formData.value
    })

    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1000))

    emit('saved')
    handleClose()
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
  } else {
    resetForm()
  }
})

// Load recipes on mount
onMounted(async () => {
  await gameDataStore.fetchRecipes()
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
