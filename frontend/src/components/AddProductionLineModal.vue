<template>
  <div v-if="isOpen" class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>Add Production Line</h3>
        <button @click="close" class="close-btn">&times;</button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="modal-body">
        <div class="form-group">
          <label for="factory">Factory</label>
          <select id="factory" v-model="selectedFactoryId" required>
            <option value="">Select a factory...</option>
            <option v-for="factory in factories" :key="factory.id" :value="factory.id">
              {{ factory.name }}
            </option>
          </select>
        </div>

        <div class="form-group">
          <label for="recipe">Recipe</label>
          <SearchableSelect
            v-model="selectedRecipe"
            :options="recipes"
            placeholder="Search for a recipe..."
            label-key="name"
            meta-key="machine_type"
            key-key="name"
            :filter-keys="['name', 'machine_type']"
          />
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="machineCount">Machine Count</label>
            <input
              id="machineCount"
              v-model.number="machineCount"
              type="number"
              min="1"
              max="999"
              required
            />
          </div>

          <div class="form-group">
            <label for="clockRatio">Clock Speed (%)</label>
            <input
              id="clockRatio"
              v-model.number="clockRatio"
              type="number"
              min="10"
              max="250"
              step="5"
              required
            />
          </div>
        </div>

        <div class="form-group">
          <label for="boostedMachines">Strange Matter Boosters</label>
          <input
            id="boostedMachines"
            v-model.number="boostedMachines"
            type="number"
            min="0"
            :max="machineCount"
          />
          <div class="help-text">Number of machines with Strange Matter boosters (max: {{ machineCount }})</div>
        </div>

        <div class="form-group">
          <label for="groupName">Group Name (optional)</label>
          <div class="group-input-container">
            <select v-if="existingGroups.length > 0" v-model="selectedExistingGroup" @change="handleGroupSelection" class="group-select">
              <option value="">Select existing group...</option>
              <option v-for="group in existingGroups" :key="group" :value="group">
                {{ group }}
              </option>
            </select>
            <input
              id="groupName"
              v-model="groupName"
              type="text"
              :placeholder="existingGroups.length > 0 ? 'Or enter new group name...' : 'e.g., Iron Processing, Early Game...'"
            />
          </div>
          <div v-if="existingGroups.length > 0" class="help-text">
            Select an existing group or enter a new name
          </div>
        </div>

        <div v-if="error" class="error-message">{{ error }}</div>

        <div class="form-actions">
          <button type="button" @click="close" class="btn btn-secondary">
            Cancel
          </button>
          <button type="submit" class="btn btn-primary" :disabled="isLoading">
            {{ isLoading ? 'Adding...' : 'Add Production Line' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { getFactories, getRecipes, addProductionLine, generateLineId, getFactoryGroups } from '../lib/tracker'
import SearchableSelect from './SearchableSelect.vue'

interface Props {
  isOpen: boolean
  preselectedFactoryId?: string
}

interface Emits {
  (e: 'close'): void
  (e: 'line-added'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const factories = ref<any[]>([])
const recipes = ref<any[]>([])
const existingGroups = ref<string[]>([])
const selectedFactoryId = ref('')
const selectedRecipe = ref<any>(null)
const selectedExistingGroup = ref('')
const machineCount = ref(1)
const clockRatio = ref(100)
const boostedMachines = ref(0)
const groupName = ref('')
const error = ref('')
const isLoading = ref(false)

// Load data on mount
onMounted(async () => {
  try {
    factories.value = await getFactories()
    recipes.value = await getRecipes()
  } catch (err: any) {
    error.value = 'Failed to load data: ' + err.message
  }
})

// Reset form when modal opens and refresh factories list
watch(() => props.isOpen, async (isOpen) => {
  if (isOpen) {
    selectedFactoryId.value = props.preselectedFactoryId || ''
    selectedRecipe.value = null
    selectedExistingGroup.value = ''
    machineCount.value = 1
    clockRatio.value = 100
    boostedMachines.value = 0
    groupName.value = ''
    existingGroups.value = []
    error.value = ''
    isLoading.value = false
    
    // Refresh factories and recipes when modal opens
    try {
      factories.value = await getFactories()
      recipes.value = await getRecipes()
      
      // If we have a preselected factory, load its groups immediately
      if (props.preselectedFactoryId) {
        try {
          existingGroups.value = await getFactoryGroups(props.preselectedFactoryId)
        } catch (err: any) {
          console.warn('Failed to load existing groups:', err.message)
          existingGroups.value = []
        }
      }
    } catch (err: any) {
      error.value = 'Failed to refresh data: ' + err.message
    }
  }
})

// Load existing groups when factory is selected
watch(selectedFactoryId, async (factoryId) => {
  if (factoryId) {
    try {
      existingGroups.value = await getFactoryGroups(factoryId)
    } catch (err: any) {
      console.warn('Failed to load existing groups:', err.message)
      existingGroups.value = []
    }
  } else {
    existingGroups.value = []
    selectedExistingGroup.value = ''
    groupName.value = ''
  }
})

// Validate boosted machines when machine count changes
watch(machineCount, (newCount) => {
  if (boostedMachines.value > newCount) {
    boostedMachines.value = newCount
  }
})

const close = () => {
  emit('close')
}

const handleOverlayClick = () => {
  close()
}

const handleGroupSelection = () => {
  if (selectedExistingGroup.value) {
    groupName.value = selectedExistingGroup.value
  }
}

const handleSubmit = async () => {
  if (!selectedFactoryId.value || !selectedRecipe.value) {
    error.value = 'Please select both a factory and recipe'
    return
  }

  if (machineCount.value < 1 || machineCount.value > 999) {
    error.value = 'Machine count must be between 1 and 999'
    return
  }

  if (clockRatio.value < 10 || clockRatio.value > 250) {
    error.value = 'Clock speed must be between 10% and 250%'
    return
  }

  if (boostedMachines.value > machineCount.value) {
    error.value = 'Cannot have more boosted machines than total machines'
    return
  }

  isLoading.value = true
  error.value = ''

  try {
    // Generate a unique line ID
    const lineId = await generateLineId(selectedFactoryId.value, selectedRecipe.value.name)
    
    const productionLine = {
      id: lineId,
      factory_id: selectedFactoryId.value,
      recipe_id: selectedRecipe.value.id,
      machine_count: machineCount.value,
      clock_ratio: clockRatio.value / 100, // Convert percentage to decimal
      group_name: groupName.value || null,
      output_routing: {}, // Empty for now
      strange_matter_boosted: boostedMachines.value,
    }
    
    await addProductionLine(productionLine)
    emit('line-added')
    close()
  } catch (err: any) {
    error.value = err.message || 'Failed to add production line'
  } finally {
    isLoading.value = false
  }
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background: white;
  border-radius: 8px;
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  overflow-y: auto;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #e5e7eb;
}

.modal-header h3 {
  margin: 0;
  color: #1f2937;
  font-size: 1.25rem;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #6b7280;
  padding: 0;
  width: 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-btn:hover {
  color: #374151;
}

.modal-body {
  padding: 1.5rem;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #374151;
}

.form-group input,
.form-group select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 1rem;
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.help-text {
  font-size: 0.875rem;
  color: #6b7280;
  margin-top: 0.25rem;
}

.error-message {
  color: #ef4444;
  font-size: 0.875rem;
  margin-bottom: 1rem;
  padding: 0.5rem;
  background-color: #fef2f2;
  border-radius: 4px;
  border: 1px solid #fecaca;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
}

.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 6px;
  font-size: 1rem;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background-color: #3b82f6;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background-color: #2563eb;
}

.btn-secondary {
  background-color: #f3f4f6;
  color: #374151;
}

.btn-secondary:hover {
  background-color: #e5e7eb;
}

.group-input-container {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.group-select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 1rem;
  background-color: #f8f9fa;
}

.group-select:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}
</style>