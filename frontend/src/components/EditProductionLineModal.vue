<template>
  <div v-if="isOpen" class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>Edit Production Line</h3>
        <button @click="close" class="close-btn">&times;</button>
      </div>
      
      <form @submit.prevent="handleSubmit" class="modal-body">
        <div class="form-group">
          <label for="factory">Factory</label>
          <input 
            id="factory" 
            :value="factoryName" 
            type="text" 
            readonly 
            class="readonly-input"
          />
          <small class="help-text">Factory cannot be changed. Create a new production line to move recipes between factories.</small>
        </div>

        <div class="form-group">
          <label for="recipe">Recipe</label>
          <input 
            id="recipe" 
            :value="recipeName" 
            type="text" 
            readonly 
            class="readonly-input"
          />
          <small class="help-text">Recipe cannot be changed. Adjust machine count and clock speed to optimize production.</small>
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

        <div class="form-row">
          <div class="form-group">
            <label for="groupName">Group Name (Optional)</label>
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
                :placeholder="existingGroups.length > 0 ? 'Or enter new group name...' : 'e.g., Basic Iron Processing'"
              />
            </div>
            <div v-if="existingGroups.length > 0" class="help-text">
              Select an existing group or enter a new name
            </div>
          </div>

          <div class="form-group">
            <label for="boostedMachines">Boosted Machines</label>
            <input
              id="boostedMachines"
              v-model.number="boostedMachines"
              type="number"
              min="0"
              :max="machineCount || 0"
              placeholder="0"
            />
          </div>
        </div>

        <div v-if="productionLine" class="production-info">
          <div class="info-card">
            <h4>{{ recipeName }}</h4>
            <div class="recipe-details">
              <div class="detail-item">
                <strong>Machine Type:</strong> {{ productionLine.recipe_id }}
              </div>
              <div class="detail-item">
                <strong>Current Output:</strong> 
                {{ calculateOutput() }}/min
              </div>
              <div class="detail-item">
                <strong>Efficiency:</strong> 
                {{ clockRatio }}% clock speed
                <span v-if="boostedMachines > 0">({{ boostedMachines }} boosted)</span>
              </div>
            </div>
          </div>
        </div>

        <div v-if="error" class="error-message">{{ error }}</div>

        <div class="form-actions">
          <button type="button" @click="close" class="btn btn-secondary">
            Cancel
          </button>
          <button type="submit" class="btn btn-primary" :disabled="isLoading">
            {{ isLoading ? 'Updating...' : 'Update Production Line' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { updateProductionLine, getFactoryGroups } from '../lib/tracker'

interface Props {
  isOpen: boolean
  productionLine: any | null
  factoryName: string
  recipeName: string
}

interface Emits {
  (e: 'close'): void
  (e: 'production-line-updated'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const existingGroups = ref<string[]>([])
const selectedExistingGroup = ref('')
const machineCount = ref<number>(1)
const clockRatio = ref<number>(100)
const groupName = ref('')
const boostedMachines = ref<number>(0)
const error = ref('')
const isLoading = ref(false)

// Reset form when modal opens or productionLine changes
watch([() => props.isOpen, () => props.productionLine], async ([isOpen, line]) => {
  if (isOpen && line) {
    machineCount.value = line.machine_count || 1
    clockRatio.value = Math.round((line.clock_ratio || 1) * 100)
    groupName.value = line.group_name || ''
    boostedMachines.value = line.strange_matter_boosted || 0
    selectedExistingGroup.value = ''
    error.value = ''
    isLoading.value = false
    
    // Load existing groups for this factory
    try {
      existingGroups.value = await getFactoryGroups(line.factory_id)
    } catch (err: any) {
      console.warn('Failed to load existing groups:', err.message)
      existingGroups.value = []
    }
  }
})

const calculateOutput = () => {
  if (!props.productionLine) return '0'
  // This is a simplified calculation - would need actual recipe data for precise output
  const baseRate = 60 // placeholder - would come from recipe data
  const totalOutput = baseRate * machineCount.value * (clockRatio.value / 100)
  return Math.round(totalOutput * 10) / 10
}

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
  if (!props.productionLine) {
    error.value = 'No production line to update'
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
    error.value = 'Boosted machines cannot exceed total machine count'
    return
  }

  isLoading.value = true
  error.value = ''

  try {
    const productionLineData = {
      id: props.productionLine.id,
      factory_id: props.productionLine.factory_id,
      recipe_id: props.productionLine.recipe_id,
      machine_count: machineCount.value,
      clock_ratio: clockRatio.value / 100,
      group_name: groupName.value || undefined,
      output_routing: props.productionLine.output_routing || {},
      strange_matter_boosted: boostedMachines.value || 0,
    }

    await updateProductionLine(props.productionLine.id, productionLineData)
    emit('production-line-updated')
    close()
  } catch (err: any) {
    error.value = err.message || 'Failed to update production line'
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
  max-width: 700px;
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

.readonly-input {
  background-color: #f9fafb;
  color: #6b7280;
  cursor: not-allowed;
}

.help-text {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.875rem;
  color: #6b7280;
}

.production-info {
  margin-bottom: 1.5rem;
}

.info-card {
  background: #f0f9ff;
  border: 1px solid #bae6fd;
  border-left: 4px solid #0ea5e9;
  border-radius: 6px;
  padding: 1rem;
}

.info-card h4 {
  margin: 0 0 0.75rem 0;
  color: #0f172a;
  font-size: 1.1rem;
}

.recipe-details {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.detail-item {
  font-size: 0.9rem;
  color: #334155;
}

.detail-item strong {
  color: #0f172a;
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