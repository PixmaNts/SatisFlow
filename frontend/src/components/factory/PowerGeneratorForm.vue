<template>
  <Modal
    :show="show"
    :title="isEditing ? 'Edit Power Generator' : 'Add Power Generator'"
    @close="handleClose"
  >
    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="generator-type" class="form-label">Generator Type *</label>
        <select
          id="generator-type"
          v-model="formData.generator_type"
          class="form-select"
          required
          @change="handleGeneratorTypeChange"
        >
          <option value="">Select a generator type...</option>
          <option
            v-for="generator in availableGenerators"
            :key="generator.type"
            :value="generator.type"
          >
            {{ generator.name }}
          </option>
        </select>
      </div>

      <!-- Fuel Type (for generators that require fuel) -->
      <div v-if="showFuelType" class="form-group">
        <label for="fuel-type" class="form-label">Fuel Type *</label>
        <select
          id="fuel-type"
          v-model="formData.fuel_type"
          class="form-select"
          required
        >
          <option value="">Select a fuel type...</option>
          <option
            v-for="fuel in availableFuels"
            :key="fuel"
            :value="fuel"
          >
            {{ formatItemName(fuel) }}
          </option>
        </select>
      </div>

      <!-- Generator Groups -->
      <div class="form-group">
        <label class="form-label">Generator Groups</label>
        <div class="generator-groups">
          <div
            v-for="(group, index) in formData.groups"
            :key="index"
            class="generator-group"
          >
            <div class="group-header">
              <h4>Group {{ index + 1 }}</h4>
              <Button
                v-if="formData.groups.length > 1"
                variant="danger"
                size="sm"
                @click="removeGeneratorGroup(index)"
              >
                Remove
              </Button>
            </div>

            <div class="group-fields">
              <div class="field-row">
                <div class="field">
                  <label :for="`generators-${index}`" class="field-label">Number of Generators</label>
                  <input
                    :id="`generators-${index}`"
                    v-model.number="group.number_of_generators"
                    type="number"
                    min="1"
                    class="form-input"
                    required
                  />
                </div>

                <div class="field">
                  <label :for="`clock-speed-${index}`" class="field-label">Clock Speed (%)</label>
                  <input
                    :id="`clock-speed-${index}`"
                    v-model.number="group.clock_speed"
                    type="number"
                    min="0"
                    max="250"
                    step="0.1"
                    class="form-input"
                    required
                  />
                </div>
              </div>
            </div>
          </div>

          <Button
            variant="secondary"
            size="sm"
            @click="addGeneratorGroup"
          >
            Add Generator Group
          </Button>
        </div>

        <!-- Power Calculation Preview -->
        <div v-if="calculatedPower > 0" class="power-preview">
          <div class="preview-item">
            <span class="preview-label">Total Power:</span>
            <span class="preview-value">{{ formatPower(calculatedPower) }}</span>
          </div>
          <div class="preview-item">
            <span class="preview-label">Total Generators:</span>
            <span class="preview-value">{{ totalGenerators }}</span>
          </div>
          <div v-if="calculatedFuelRate > 0" class="preview-item">
            <span class="preview-label">Fuel Consumption:</span>
            <span class="preview-value">{{ formatFuelRate(calculatedFuelRate) }}</span>
          </div>
        </div>
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
          {{ isEditing ? 'Update' : 'Add' }} Power Generator
        </Button>
      </div>
    </form>
  </Modal>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useGameDataStore } from '@/stores/gameData'
import type { PowerGeneratorResponse, GeneratorType, Item, GeneratorGroup } from '@/api/types'
import Button from '@/components/ui/Button.vue'
import Modal from '@/components/ui/Modal.vue'

interface Props {
  show: boolean
  factoryId: number
  powerGenerator?: PowerGeneratorResponse | null
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
  generator_type: '' as GeneratorType,
  fuel_type: null as Item | null,
  groups: [] as GeneratorGroup[]
})

// Generator configurations
const generatorConfigs = [
  { type: 'Biomass' as GeneratorType, name: 'Biomass Burner', requiresFuel: true },
  { type: 'Coal' as GeneratorType, name: 'Coal Generator', requiresFuel: true },
  { type: 'Fuel' as GeneratorType, name: 'Fuel Generator', requiresFuel: true },
  { type: 'Nuclear' as GeneratorType, name: 'Nuclear Power Plant', requiresFuel: true },
  { type: 'Geothermal' as GeneratorType, name: 'Geothermal Generator', requiresFuel: false }
]

// Computed
const isEditing = computed(() => !!props.powerGenerator)
const items = computed(() => gameDataStore.items)

const availableGenerators = computed(() => {
  return generatorConfigs
})

const selectedGenerator = computed(() => {
  return generatorConfigs.find(config => config.type === formData.value.generator_type)
})

const showFuelType = computed(() => {
  return selectedGenerator.value && selectedGenerator.value.requiresFuel
})

const availableFuels = computed(() => {
  if (!formData.value.generator_type) return []

  // Filter fuels based on generator type
  switch (formData.value.generator_type) {
    case 'Biomass':
      return items.value.filter(item =>
        item === 'Biomass' ||
        item === 'Leaves' ||
        item === 'Mycelia' ||
        item === 'Wood' ||
        item === 'AlienProtein' ||
        item === 'AlienDNA' ||
        item === 'FlowerPetals' ||
        item === 'BaconAgaric' ||
        item === 'BerylNut' ||
        item === 'Pollen'
      )
    case 'Coal':
      return items.value.filter(item => item === 'Coal' || item === 'CompactedCoal')
    case 'Fuel':
      return items.value.filter(item =>
        item === 'Fuel' ||
        item === 'Turbofuel' ||
        item === 'LiquidBiofuel' ||
        item === 'PackagedLiquidBiofuel'
      )
    case 'Nuclear':
      return items.value.filter(item =>
        item === 'UraniumFuelRod' ||
        item === 'PlutoniumFuelRod' ||
        item === 'FicsoniumFuelRod'
      )
    default:
      return []
  }
})

const calculatedPower = computed(() => {
  if (!selectedGenerator.value || !formData.value.groups.length) {
    return 0
  }

  // Base power output for different generator types
  const basePower: Record<GeneratorType, number> = {
    Biomass: 30,
    Coal: 150,
    Fuel: 150,
    Nuclear: 2500,
    Geothermal: 200
  }

  const basePowerValue = basePower[formData.value.generator_type] || 0

  return formData.value.groups.reduce((total, group) => {
    const clockSpeed = group.clock_speed / 100
    const powerMultiplier = Math.pow(clockSpeed, 1.321928)
    return total + (group.number_of_generators * basePowerValue * powerMultiplier)
  }, 0)
})

const calculatedFuelRate = computed(() => {
  if (!selectedGenerator.value || !formData.value.groups.length) {
    return 0
  }

  // Base fuel consumption rates (items/min) for different generators
  const baseFuelRate: Record<GeneratorType, number> = {
    Biomass: 4,
    Coal: 15.3,
    Fuel: 4.5,
    Nuclear: 0.025, // Uranium Fuel Rods/min
    Geothermal: 0 // No fuel consumption
  }

  const baseRate = baseFuelRate[formData.value.generator_type] || 0

  return formData.value.groups.reduce((total, group) => {
    const clockSpeed = group.clock_speed / 100
    const fuelMultiplier = Math.pow(clockSpeed, 1.321928)
    return total + (group.number_of_generators * baseRate * fuelMultiplier)
  }, 0)
})

const totalGenerators = computed(() => {
  return formData.value.groups.reduce((total, group) => total + group.number_of_generators, 0)
})

const canSubmit = computed(() => {
  return (
    formData.value.generator_type &&
    formData.value.groups.length > 0 &&
    formData.value.groups.every(group =>
      group.number_of_generators > 0 &&
      group.clock_speed >= 0 &&
      group.clock_speed <= 250
    ) &&
    (!showFuelType.value || formData.value.fuel_type)
  )
})

// Methods
const formatItemName = (item: string): string => {
  return item.replace(/([A-Z])/g, ' $1').trim()
}

const formatPower = (power: number): string => {
  if (power < 1) {
    return `${(power * 1000).toFixed(0)} kW`
  }
  return `${power.toFixed(1)} MW`
}

const formatFuelRate = (rate: number): string => {
  if (rate < 1) {
    return `${(rate * 60).toFixed(1)}/h`
  }
  return `${rate.toFixed(2)}/min`
}

const createDefaultGeneratorGroup = (): GeneratorGroup => {
  return {
    number_of_generators: 1,
    clock_speed: 100
  }
}

const addGeneratorGroup = () => {
  formData.value.groups.push(createDefaultGeneratorGroup())
}

const removeGeneratorGroup = (index: number) => {
  formData.value.groups.splice(index, 1)
}

const handleGeneratorTypeChange = () => {
  // Reset fuel type when generator type changes
  if (!showFuelType.value) {
    formData.value.fuel_type = null
  } else {
    // Set default fuel type if available
    if (availableFuels.value.length > 0 && !formData.value.fuel_type) {
      formData.value.fuel_type = availableFuels.value[0] || null
    }
  }

  // Reset generator groups
  formData.value.groups = [createDefaultGeneratorGroup()]
}

const handleClose = () => {
  emit('update:show', false)
  resetForm()
}

const resetForm = () => {
  formData.value = {
    generator_type: '' as GeneratorType,
    fuel_type: null,
    groups: []
  }
}

const loadPowerGenerator = () => {
  if (!props.powerGenerator) return

  formData.value = {
    generator_type: props.powerGenerator.generator_type,
    fuel_type: props.powerGenerator.fuel_type,
    groups: [...props.powerGenerator.groups]
  }
}

const handleSubmit = async () => {
  if (!canSubmit.value) return

  saving.value = true

  try {
    // TODO: Implement API call to save power generator
    console.log('Saving power generator:', {
      factory_id: props.factoryId,
      ...formData.value
    })

    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1000))

    emit('saved')
    handleClose()
  } catch (error) {
    console.error('Failed to save power generator:', error)
  } finally {
    saving.value = false
  }
}

// Watch for power generator changes
watch(() => props.powerGenerator, () => {
  if (props.show) {
    loadPowerGenerator()
  }
}, { immediate: true })

// Watch for show changes
watch(() => props.show, (show) => {
  if (show) {
    loadPowerGenerator()
  } else {
    resetForm()
  }
})

// Load items on mount
onMounted(async () => {
  await gameDataStore.fetchItems()
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

.generator-groups {
  border: 1px solid var(--color-gray-200, #e5e7eb);
  border-radius: var(--border-radius-md, 0.375rem);
  padding: var(--spacing-md, 0.75rem);
  background-color: var(--color-gray-50, #f9fafb);
}

.generator-group {
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
  background-color: var(--color-green-50, #f0fdf4);
  border-radius: var(--border-radius-md, 0.375rem);
  border: 1px solid var(--color-green-200, #dcfce7);
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
