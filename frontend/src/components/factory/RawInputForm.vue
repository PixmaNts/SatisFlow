<template>
  <Modal
    :show="show"
    :title="isEditing ? 'Edit Raw Input' : 'Add Raw Input'"
    @close="handleClose"
  >
    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="resource-select" class="form-label">Resource *</label>
        <select
          id="resource-select"
          v-model="formData.item"
          class="form-select"
          required
          @change="handleResourceChange"
        >
          <option value="">Select a resource...</option>
          <optgroup label="Solid Resources">
            <option
              v-for="resource in solidResources"
              :key="resource"
              :value="resource"
            >
              {{ formatItemName(resource) }}
            </option>
          </optgroup>
          <optgroup label="Liquid Resources">
            <option
              v-for="resource in liquidResources"
              :key="resource"
              :value="resource"
            >
              {{ formatItemName(resource) }}
            </option>
          </optgroup>
          <optgroup label="Gas Resources">
            <option
              v-for="resource in gasResources"
              :key="resource"
              :value="resource"
            >
              {{ formatItemName(resource) }}
            </option>
          </optgroup>
        </select>
      </div>

      <div class="form-group">
        <label for="extractor-select" class="form-label">Extractor Type *</label>
        <select
          id="extractor-select"
          v-model="formData.extractor_type"
          class="form-select"
          required
          @change="handleExtractorChange"
        >
          <option value="">Select an extractor...</option>
          <option
            v-for="extractor in availableExtractors"
            :key="extractor.type"
            :value="extractor.type"
          >
            {{ extractor.name }}
          </option>
        </select>
      </div>

      <!-- Purity (for solid resources and oil) -->
      <div v-if="showPurity" class="form-group">
        <label class="form-label">
          Purity
        </label>
        <div class="radio-group">
          <label class="radio-option">
            <input
              v-model="formData.purity"
              type="radio"
              value="Impure"
            />
            <span>Impure (50%)</span>
          </label>
          <label class="radio-option">
            <input
              v-model="formData.purity"
              type="radio"
              value="Normal"
            />
            <span>Normal (100%)</span>
          </label>
          <label class="radio-option">
            <input
              v-model="formData.purity"
              type="radio"
              value="Pure"
            />
            <span>Pure (200%)</span>
          </label>
        </div>
      </div>

      <!-- Resource Well Pressurizer (for Resource Well Extractors) -->
      <div v-if="showPressurizer" class="form-group">
        <div class="checkbox-option">
          <input
            id="use-pressurizer"
            v-model="usePressurizer"
            type="checkbox"
            @change="handlePressurizerToggle"
          />
          <label for="use-pressurizer">Use Resource Well Pressurizer</label>
        </div>

        <div v-if="usePressurizer && formData.pressurizer" class="pressurizer-config">
          <div class="form-group">
            <label for="clock-speed" class="form-label">Clock Speed (%)</label>
            <input
              id="clock-speed"
              v-model.number="formData.pressurizer.clock_speed"
              type="number"
              min="0"
              max="250"
              step="0.1"
              class="form-input"
            />
          </div>

          <div class="form-group">
            <label class="form-label">Extractors</label>
            <div class="extractors-list">
              <div
                v-for="(extractor, index) in formData.extractors"
                :key="index"
                class="extractor-item"
              >
                <div class="extractor-fields">
                  <div class="field">
                    <label :for="`extractor-item-${index}`" class="field-label">Resource</label>
                    <select
                      :id="`extractor-item-${index}`"
                      v-model="extractor.item"
                      class="form-select"
                    >
                      <option
                        v-for="resource in gasResources"
                        :key="resource"
                        :value="resource"
                      >
                        {{ formatItemName(resource) }}
                      </option>
                    </select>
                  </div>

                  <div class="field">
                    <label :for="`extractor-purity-${index}`" class="field-label">Purity</label>
                    <select
                      :id="`extractor-purity-${index}`"
                      v-model="extractor.purity"
                      class="form-select"
                    >
                      <option value="Impure">Impure</option>
                      <option value="Normal">Normal</option>
                      <option value="Pure">Pure</option>
                    </select>
                  </div>

                  <div class="field">
                    <label :for="`extractor-rate-${index}`" class="field-label">Rate (m³/min)</label>
                    <input
                      :id="`extractor-rate-${index}`"
                      v-model.number="extractor.quantity_per_min"
                      type="number"
                      min="0"
                      step="0.1"
                      class="form-input"
                      readonly
                      :title="`Calculated based on purity: ${getExtractorRate(extractor.purity)} m³/min`"
                    />
                  </div>
                </div>

                <Button
                  v-if="formData.extractors!.length > 1"
                  variant="danger"
                  size="sm"
                  @click="removeExtractor(index)"
                >
                  Remove
                </Button>
              </div>

              <Button
                variant="secondary"
                size="sm"
                @click="addExtractor"
              >
                Add Extractor
              </Button>
            </div>
          </div>
        </div>
      </div>

      <!-- Manual Rate Input (for non-pressurized extractors) -->
      <div v-if="!usePressurizer" class="form-group">
        <label for="rate-input" class="form-label">
          Extraction Rate
          <span v-if="selectedExtractor" class="rate-unit">
            ({{ getRateUnit() }})
          </span>
        </label>
        <input
          id="rate-input"
          v-model.number="formData.quantity_per_min"
          type="number"
          min="0"
          step="0.1"
          class="form-input"
          required
        />
        <div v-if="calculatedRate > 0" class="rate-hint">
          Calculated rate for {{ formData.purity || 'selected' }} purity: {{ calculatedRate }} {{ getRateUnit() }}
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
          {{ isEditing ? 'Update' : 'Add' }} Raw Input
        </Button>
      </div>
    </form>
  </Modal>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useGameDataStore } from '@/stores/gameData'
import { useFactoryStore } from '@/stores/factory'
import { factories } from '@/api/endpoints'
import type {
  RawInputResponse,
  ExtractorType,
  Purity,
  Item,
  ResourceWellPressurizer,
  ResourceWellExtractor,
  CreateRawInputRequest,
} from '@/api/types'
import Button from '@/components/ui/Button.vue'
import Modal from '@/components/ui/Modal.vue'

interface Props {
  show: boolean
  factoryId: string
  rawInput?: RawInputResponse | null
}

interface Emits {
  (e: 'update:show', value: boolean): void
  (e: 'saved'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const gameDataStore = useGameDataStore()
const factoryStore = useFactoryStore()

// State
const saving = ref(false)
const usePressurizer = ref(false)
const formData = ref({
  item: '',
  extractor_type: '' as ExtractorType,
  purity: null as Purity | null,
  quantity_per_min: 0,
  pressurizer: null as ResourceWellPressurizer | null,
  extractors: [] as ResourceWellExtractor[]
})

// Extractor configurations
const extractorConfigs = [
  { type: 'MinerMk1' as ExtractorType, name: 'Miner Mk1', resources: 'solid' },
  { type: 'MinerMk2' as ExtractorType, name: 'Miner Mk2', resources: 'solid' },
  { type: 'MinerMk3' as ExtractorType, name: 'Miner Mk3', resources: 'solid' },
  { type: 'WaterExtractor' as ExtractorType, name: 'Water Extractor', resources: 'liquid' },
  { type: 'OilExtractor' as ExtractorType, name: 'Oil Extractor', resources: 'liquid' },
  { type: 'ResourceWellExtractor' as ExtractorType, name: 'Resource Well Extractor', resources: 'gas' }
]

// Computed
const isEditing = computed(() => !!props.rawInput)
const items = computed(() => gameDataStore.items)

const solidResources = computed(() => {
  return items.value.filter(item =>
    ['Ore', 'Ingot', 'Plate', 'Sheet', 'Rod', 'Pipe', 'Beam'].some(type => item.includes(type))
  )
})

const liquidResources = computed(() => {
  return items.value.filter(item =>
    ['CrudeOil', 'Water', 'HeavyOilResidue', 'LiquidBiofuel', 'NitricAcid'].some(type => item.includes(type))
  )
})

const gasResources = computed(() => {
  return items.value.filter(item =>
    ['Gas', 'Nitrogen'].some(type => item.includes(type))
  )
})

const selectedExtractor = computed(() => {
  return extractorConfigs.find(config => config.type === formData.value.extractor_type)
})

const availableExtractors = computed(() => {
  if (!formData.value.item) return extractorConfigs

  // Determine resource type based on selected item
  let resourceType = 'solid'
  if (liquidResources.value.includes(formData.value.item as Item)) {
    resourceType = 'liquid'
  } else if (gasResources.value.includes(formData.value.item as Item)) {
    resourceType = 'gas'
  }

  return extractorConfigs.filter(config => config.resources === resourceType)
})

const showPurity = computed(() => {
  // Show purity for solid resources (Miners) and Oil Extractor
  return selectedExtractor.value && (
    selectedExtractor.value.resources === 'solid' ||
    selectedExtractor.value.type === 'OilExtractor'
  )
})

const showPressurizer = computed(() => {
  return formData.value.extractor_type === 'ResourceWellExtractor'
})

// Preview data from backend
const previewData = ref<{ quantity_per_min: number; power_consumption: number } | null>(null)
const previewLoading = ref(false)

// Fetch preview from backend when form changes
const fetchPreview = async () => {
  if (!props.factoryId || !formData.value.extractor_type || !formData.value.item) {
    previewData.value = null
    return
  }

  // For WaterExtractor, purity is not used, but we still need to call preview
  if (formData.value.extractor_type === 'WaterExtractor') {
    // Water extractor always extracts 120 m³/min, no purity
    previewData.value = { quantity_per_min: 120, power_consumption: 20 }
    return
  }

  // For other extractors, we need purity
  if (!formData.value.purity) {
    previewData.value = null
    return
  }

  previewLoading.value = true
  try {
    const request: any = {
      extractor_type: formData.value.extractor_type,
      item: formData.value.item,
      purity: formData.value.purity,
    }

    if (showPressurizer.value && usePressurizer.value && formData.value.pressurizer) {
      request.pressurizer = {
        clock_speed: formData.value.pressurizer.clock_speed,
      }
      request.extractors = formData.value.extractors.map(ext => ({
        purity: ext.purity,
      }))
    }

    const preview = await factories.rawInputs.rawInput(props.factoryId, request)
    previewData.value = {
      quantity_per_min: preview.quantity_per_min,
      power_consumption: preview.power_consumption,
    }
  } catch (error) {
    console.error('Failed to fetch raw input preview:', error)
    previewData.value = null
  } finally {
    previewLoading.value = false
  }
}

const calculatedRate = computed(() => {
  return previewData.value?.quantity_per_min || 0
})

const canSubmit = computed(() => {
  return (
    formData.value.item &&
    formData.value.extractor_type &&
    formData.value.quantity_per_min > 0 &&
    (!showPurity.value || formData.value.purity) &&
    (!showPressurizer.value || !usePressurizer.value || formData.value.extractors.length > 0)
  )
})

// Methods
const formatItemName = (item: string): string => {
  return item.replace(/([A-Z])/g, ' $1').trim()
}

const getRateUnit = (): string => {
  if (formData.value.extractor_type === 'WaterExtractor' ||
      formData.value.extractor_type === 'OilExtractor' ||
      formData.value.extractor_type === 'ResourceWellExtractor') {
    return 'm³/min'
  }
  return 'items/min'
}

// Removed getExtractorRate - use backend preview instead

const handleResourceChange = () => {
  // Reset extractor when resource changes
  formData.value.extractor_type = '' as ExtractorType
  formData.value.purity = null
  formData.value.quantity_per_min = 0
}

const handleExtractorChange = () => {
  // Set default purity for solid resources, oil, and water extractors
  if (!formData.value.purity) {
    if (selectedExtractor.value?.resources === 'solid') {
      // Miners: default to Normal
      formData.value.purity = 'Normal'
    } else if (formData.value.extractor_type === 'OilExtractor') {
      // Oil Extractor: default to Normal
      formData.value.purity = 'Normal'
    } else if (formData.value.extractor_type === 'WaterExtractor') {
      // Water Extractor: internally use Normal (no purity concept in UI)
      formData.value.purity = 'Normal'
    } else if (formData.value.extractor_type === 'ResourceWellExtractor') {
      // Resource Well Extractor: default to Normal
      formData.value.purity = 'Normal'
    }
  }

  // Set default rate
  if (calculatedRate.value > 0) {
    formData.value.quantity_per_min = calculatedRate.value
  }
}

const handlePressurizerToggle = () => {
  if (usePressurizer.value) {
    // Initialize pressurizer
    formData.value.pressurizer = {
      id: 0, // Will be assigned by backend
      clock_speed: 100,
      power_consumption: 0 // Will be calculated by backend
    }

    // Initialize with one extractor
    formData.value.extractors = [{
      id: 0, // Will be assigned by backend
      item: formData.value.item as Item,
      purity: 'Normal',
      quantity_per_min: 60
    }]
  } else {
    // Clear pressurizer data
    formData.value.pressurizer = null
    formData.value.extractors = []
  }
}

const addExtractor = () => {
  formData.value.extractors.push({
    id: 0, // Will be assigned by backend
    item: formData.value.item as Item,
    purity: 'Normal',
    quantity_per_min: 60
  })
}

const removeExtractor = (index: number) => {
  formData.value.extractors.splice(index, 1)
}

const handleClose = () => {
  emit('update:show', false)
  resetForm()
}

const resetForm = () => {
  formData.value = {
    item: '',
    extractor_type: '' as ExtractorType,
    purity: null,
    quantity_per_min: 0,
    pressurizer: null,
    extractors: []
  }
  usePressurizer.value = false
}

const loadRawInput = () => {
  if (!props.rawInput) return

  formData.value = {
    item: props.rawInput.item,
    extractor_type: props.rawInput.extractor_type,
    purity: props.rawInput.purity,
    quantity_per_min: props.rawInput.quantity_per_min,
    pressurizer: props.rawInput.pressurizer,
    extractors: [...props.rawInput.extractors]
  }

  usePressurizer.value = !!props.rawInput.pressurizer
}

const handleSubmit = async () => {
  if (!canSubmit.value) return

  saving.value = true

  try {
    const payload: CreateRawInputRequest = {
      extractor_type: formData.value.extractor_type,
      item: formData.value.item as Item,
      quantity_per_min: formData.value.quantity_per_min,
    }

    // Only include purity for extractors that support it
    // WaterExtractor does NOT support purity (backend will reject it)
    // All other extractors (Miners, OilExtractor, ResourceWellExtractor) require purity
    console.log('Extractor type:', formData.value.extractor_type, 'Purity:', formData.value.purity)
    if (formData.value.extractor_type !== 'WaterExtractor') {
      payload.purity = formData.value.purity ?? undefined
      console.log('Including purity in payload:', payload.purity)
    } else {
      console.log('Skipping purity for WaterExtractor')
    }

    if (showPressurizer.value && usePressurizer.value) {
      payload.pressurizer = {
        id: formData.value.pressurizer?.id ?? undefined,
        clock_speed: formData.value.pressurizer?.clock_speed ?? 100,
      }
      payload.extractors = formData.value.extractors.map((extractor, index) => ({
        id: extractor.id ?? index + 1,
        purity: extractor.purity,
      }))
    } else {
      delete payload.pressurizer
      delete payload.extractors
    }

    if (!payload.quantity_per_min || payload.quantity_per_min <= 0) {
      delete payload.quantity_per_min
    }

    let response = null
    if (isEditing.value && props.rawInput) {
      response = await factoryStore.updateRawInput(props.factoryId, props.rawInput.id, payload)
    } else {
      response = await factoryStore.createRawInput(props.factoryId, payload)
    }

    if (response) {
      emit('saved')
      handleClose()
    }
  } catch (error) {
    console.error('Failed to save raw input:', error)
  } finally {
    saving.value = false
  }
}

// Watch for raw input changes
watch(() => props.rawInput, () => {
  if (props.show) {
    loadRawInput()
  }
}, { immediate: true })

// Watch for show changes
watch(() => props.show, (show) => {
  if (show) {
    loadRawInput()
  } else {
    resetForm()
  }
})

// Watch for extractor changes to fetch preview
watch(() => formData.value.extractors, () => {
  if (showPressurizer.value && usePressurizer.value) {
    fetchPreview()
  }
}, { deep: true })

// Watch form changes to fetch preview
watch([() => formData.value.extractor_type, () => formData.value.item, () => formData.value.purity, () => formData.value.pressurizer?.clock_speed], () => {
  fetchPreview()
}, { deep: true })

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

  &[readonly] {
    background-color: var(--color-gray-100, #f3f4f6);
    color: var(--color-gray-600, #4b5563);
  }
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

.checkbox-option {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
  margin-bottom: var(--spacing-sm, 0.5rem);

  input[type="checkbox"] {
    margin: 0;
  }

  label {
    font-size: var(--font-size-sm, 0.875rem);
    font-weight: var(--font-weight-medium, 500);
    color: var(--color-gray-700, #374151);
    cursor: pointer;
  }
}

.pressurizer-config {
  margin-top: var(--spacing-md, 0.75rem);
  padding: var(--spacing-md, 0.75rem);
  background-color: var(--color-gray-50, #f9fafb);
  border-radius: var(--border-radius-md, 0.375rem);
  border: 1px solid var(--color-gray-200, #e5e7eb);
}

.extractors-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md, 0.75rem);
}

.extractor-item {
  padding: var(--spacing-md, 0.75rem);
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-md, 0.375rem);
  border: 1px solid var(--color-gray-200, #e5e7eb);
}

.extractor-fields {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: var(--spacing-md, 0.75rem);
  margin-bottom: var(--spacing-sm, 0.5rem);
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

.rate-unit {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-500, #6b7280);
  font-weight: var(--font-weight-normal, 400);
  margin-left: var(--spacing-xs, 0.25rem);
}

.rate-hint {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-blue-600, #2563eb);
  margin-top: var(--spacing-xs, 0.25rem);
}

.purity-note {
  display: block;
  font-size: var(--font-size-xs, 0.75rem);
  font-weight: var(--font-weight-normal, 400);
  color: var(--color-gray-500, #6b7280);
  margin-top: var(--spacing-xs, 0.25rem);
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

  .extractor-fields {
    grid-template-columns: 1fr;
  }

  .form-actions {
    flex-direction: column;
  }

  .form-actions button {
    width: 100%;
  }
}
</style>
