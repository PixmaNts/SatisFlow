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
          <option
            v-for="resource in availableItems"
            :key="resource"
            :value="resource"
          >
            {{ formatItemName(resource) }}
          </option>
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
              @change="handlePurityChange"
            />
            <span>Impure (50%)</span>
          </label>
          <label class="radio-option">
            <input
              v-model="formData.purity"
              type="radio"
              value="Normal"
              @change="handlePurityChange"
            />
            <span>Normal (100%)</span>
          </label>
          <label class="radio-option">
            <input
              v-model="formData.purity"
              type="radio"
              value="Pure"
              @change="handlePurityChange"
            />
            <span>Pure (200%)</span>
          </label>
        </div>
      </div>

      <!-- Overclock and Count (for all extractors except Resource Well) -->
      <div v-if="!showPressurizer" class="form-group">
        <div class="form-row">
          <div class="form-col form-col-oc">
            <label for="overclock-input" class="form-label">
              Overclock (%)
            </label>
            <div class="slider-container">
              <input
                id="overclock-slider"
                v-model.number="formData.overclock_percent"
                type="range"
                min="0"
                max="250"
                step="0.5"
                class="form-slider"
                @input="handleOverclockSliderChange"
              />
              <input
                id="overclock-input"
                v-model.number="formData.overclock_percent"
                type="number"
                min="0"
                max="250"
                step="0.001"
                class="form-input oc-input"
                @input="handleOverclockChange"
              />
            </div>
          </div>
          <div class="form-col form-col-count">
            <label for="count-input" class="form-label">
              Count
            </label>
            <input
              id="count-input"
              v-model.number="formData.count"
              type="number"
              min="1"
              step="1"
              class="form-input count-input"
              required
              @input="handleCountChange"
            />
          </div>
        </div>
      </div>

      <!-- Overclock for Pressurizer (Resource Well only) -->
      <div v-if="showPressurizer && formData.pressurizer" class="form-group">
        <label for="pressurizer-oc-input" class="form-label">
          Pressurizer Overclock (%)
        </label>
        <div class="slider-container">
          <input
            id="pressurizer-oc-slider"
            v-model.number="formData.pressurizer.clock_speed"
            type="range"
            min="0"
            max="250"
            step="0.5"
            class="form-slider"
            @input="handlePressurizerOcSliderChange"
          />
              <input
                id="pressurizer-oc-input"
                v-model.number="formData.pressurizer.clock_speed"
                type="number"
                min="0"
                max="250"
                step="0.001"
                class="form-input oc-input"
                @input="handlePressurizerOcChange"
              />
        </div>
        <div class="rate-hint">
          OC applies to the sum of all node extraction rates
        </div>
      </div>

      <!-- Resource Well Pressurizer (for Resource Well Extractors) -->
      <div v-if="showPressurizer" class="form-group">
        <div v-if="formData.pressurizer" class="pressurizer-config">
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
                    <label :for="`extractor-purity-${index}`" class="field-label">Purity</label>
                    <select
                      :id="`extractor-purity-${index}`"
                      v-model="extractor.purity"
                      class="form-select"
                      @change="handleExtractorPurityChange(index)"
                    >
                      <option value="Impure">Impure</option>
                      <option value="Normal">Normal</option>
                      <option value="Pure">Pure</option>
                    </select>
                  </div>

                  <div class="field">
                    <label :for="`extractor-count-${index}`" class="field-label">Count</label>
                    <input
                      :id="`extractor-count-${index}`"
                      v-model.number="extractor.count"
                      type="number"
                      min="1"
                      step="1"
                      class="form-input extractor-count-input"
                      @input="handleExtractorCountChange(index)"
                    />
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
                      :title="`Calculated based on purity and clock speed`"
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
                type="button"
                @click="addExtractor"
              >
                Add Extractor
              </Button>
            </div>
          </div>
        </div>
      </div>

      <!-- Total Rate Preview for Pressurizer (calculated by engine) -->
      <div v-if="showPressurizer && formData.pressurizer" class="form-group">
        <label class="form-label">
          Total Extraction Rate
          <span class="rate-unit">
            ({{ getRateUnit() }})
          </span>
        </label>
        <input
          v-model.number="calculatedRate"
          type="number"
          min="0"
          step="0.1"
          class="form-input rate-input"
          readonly
        />
        <div v-if="calculatedRate > 0" class="rate-hint">
          <div>
            Total: {{ calculatedRate.toFixed(2) }} {{ getRateUnit() }}
            <span v-if="formData.extractors.length > 0">
              ({{ formData.extractors.reduce((sum, ext) => sum + (ext.count || 1), 0) }} extractor{{ formData.extractors.reduce((sum, ext) => sum + (ext.count || 1), 0) > 1 ? 's' : '' }} at {{ formData.pressurizer?.clock_speed.toFixed(1) }}% OC)
            </span>
          </div>
          <div class="power-hint">
            Power consumption: {{ calculatedPower.toFixed(2) }} MW
          </div>
        </div>
      </div>

      <!-- Calculated Rate Display (read-only, calculated by engine) -->
      <div v-if="!showPressurizer" class="form-group">
        <label for="rate-display" class="form-label">
          Extraction Rate
          <span v-if="selectedExtractor" class="rate-unit">
            ({{ getRateUnit() }})
          </span>
        </label>
        <input
          id="rate-display"
          v-model.number="formData.quantity_per_min"
          type="number"
          min="0"
          step="0.1"
          class="form-input rate-input"
          readonly
          required
        />
        <div v-if="calculatedRate > 0" class="rate-hint">
          <div v-if="formData.count > 1 || formData.overclock_percent !== 100">
            <div>
              {{ baseRatePerExtractor.toFixed(2) }} {{ getRateUnit() }}
              <span v-if="formData.overclock_percent !== 100"> × {{ formData.overclock_percent.toFixed(1) }}%</span>
              <span v-if="formData.count > 1"> × {{ formData.count }}</span>
              = {{ calculatedRate.toFixed(2) }} {{ getRateUnit() }}
            </div>
          </div>
          <div v-else>
            <div>Rate: {{ calculatedRate.toFixed(2) }} {{ getRateUnit() }}</div>
          </div>
          <div class="power-hint">
            Power consumption: {{ calculatedPower.toFixed(2) }} MW
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
import { factories, gameData } from '@/api/endpoints'
import type {
  RawInputResponse,
  ExtractorType,
  Purity,
  Item,
  ResourceWellPressurizer,
  ResourceWellExtractor,
  CreateRawInputRequest,
  ExtractorCompatibleItemsResponse,
} from '@/api/types'
import Button from '@/components/ui/Button.vue'
import Modal from '@/components/ui/Modal.vue'
import { useItemIcon } from '@/composables/useItemIcon'

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
const formData = ref({
  item: '',
  extractor_type: '' as ExtractorType,
  purity: null as Purity | null,
  overclock_percent: 100,
  count: 1,
  quantity_per_min: 0,
  pressurizer: null as ResourceWellPressurizer | null,
  extractors: [] as (ResourceWellExtractor & { count?: number })[]
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

// Extractor compatible items loaded from backend (source of truth: engine)
const extractorCompatibleItems = ref<Map<ExtractorType, Item[]>>(new Map())
const loadingCompatibleItems = ref(false)

// Load extractor compatible items from backend
const loadExtractorCompatibleItems = async () => {
  loadingCompatibleItems.value = true
  try {
    const response = await gameData.getExtractorCompatibleItems()
    const itemsMap = new Map<ExtractorType, Item[]>()
    response.forEach((item: ExtractorCompatibleItemsResponse) => {
      itemsMap.set(item.extractor_type, item.compatible_items)
    })
    extractorCompatibleItems.value = itemsMap
  } catch (error) {
    console.error('Failed to load extractor compatible items:', error)
  } finally {
    loadingCompatibleItems.value = false
  }
}

// Helper to get compatible items for an extractor type
const getCompatibleItems = (extractorType: ExtractorType | ''): Item[] => {
  if (!extractorType) return []
  return extractorCompatibleItems.value.get(extractorType) || []
}

// Available items based on selected extractor type (from backend/engine)
const availableItems = computed(() => {
  if (!formData.value.extractor_type) {
    // If no extractor selected, show all compatible items from all extractors
    const allCompatible = new Set<Item>()
    extractorCompatibleItems.value.forEach((items) => {
      items.forEach(item => allCompatible.add(item))
    })
    return Array.from(allCompatible).filter(item => items.value.includes(item))
  }

  const compatibleItems = getCompatibleItems(formData.value.extractor_type)
  return compatibleItems.filter(item => items.value.includes(item))
})

const selectedExtractor = computed(() => {
  return extractorConfigs.find(config => config.type === formData.value.extractor_type)
})

const availableExtractors = computed(() => {
  if (!formData.value.item) return extractorConfigs

  const selectedItem = formData.value.item as Item

  // Determine which extractors are compatible with the selected item (from backend/engine)
  return extractorConfigs.filter(config => {
    const compatibleItems = getCompatibleItems(config.type)
    return compatibleItems.includes(selectedItem)
  })
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

  // For Resource Well, we need pressurizer and extractors
  if (showPressurizer.value) {
    if (!formData.value.pressurizer || formData.value.extractors.length === 0) {
      previewData.value = null
      return
    }
  } else {
    // For other extractors, we need purity (except WaterExtractor)
    if (formData.value.extractor_type !== 'WaterExtractor' && !formData.value.purity) {
      previewData.value = null
      return
    }
  }

  previewLoading.value = true
  try {
    const request: any = {
      extractor_type: formData.value.extractor_type,
      item: formData.value.item,
      purity: formData.value.purity ?? undefined,
      overclock_percent: formData.value.overclock_percent,
      count: formData.value.count,
    }

    if (showPressurizer.value && formData.value.pressurizer) {
      request.pressurizer = {
        clock_speed: formData.value.pressurizer.clock_speed,
      }
      // Expand extractors by count: each extractor entry with count N becomes N extractors
      request.extractors = formData.value.extractors.flatMap(ext => {
        const count = ext.count || 1
        return Array(count).fill(null).map(() => ({
          purity: ext.purity,
        }))
      })
      // For pressurizer, OC is in the pressurizer clock_speed
      request.overclock_percent = formData.value.pressurizer.clock_speed
    }

    const preview = await factories.preview.rawInput(props.factoryId, request)
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

const calculatedPower = computed(() => {
  return previewData.value?.power_consumption || 0
})

// Base rate per extractor (calculated from preview, without OC and count multipliers)
const baseRatePerExtractor = computed(() => {
  if (!calculatedRate.value || formData.value.count === 0) return 0
  // Calculate base rate per extractor: total_rate / (oc/100) / count
  const ocMultiplier = formData.value.overclock_percent / 100.0
  if (ocMultiplier === 0) return 0
  return calculatedRate.value / ocMultiplier / formData.value.count
})

const canSubmit = computed(() => {
  return (
    formData.value.item &&
    formData.value.extractor_type &&
    formData.value.quantity_per_min > 0 &&
    formData.value.overclock_percent >= 0 &&
    formData.value.overclock_percent <= 250 &&
    formData.value.count >= 1 &&
    (!showPurity.value || formData.value.purity) &&
    (!showPressurizer.value || formData.value.extractors.length > 0)
  )
})

// Methods
const { formatItemName } = useItemIcon()

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
  previewData.value = null
  
  // For Resource Well, update all extractors to use the new item
  if (formData.value.extractor_type === 'ResourceWellExtractor' && formData.value.extractors.length > 0) {
    formData.value.extractors.forEach(extractor => {
      extractor.item = formData.value.item as Item
    })
    updateExtractorRates()
    fetchPreview()
  }
}

const handleExtractorChange = async () => {
  // Set default purity for solid resources, oil, and water extractors
  if (!formData.value.purity) {
    if (selectedExtractor.value?.resources === 'solid') {
      // Miners: default to Normal
      formData.value.purity = 'Normal'
    } else if (formData.value.extractor_type === 'OilExtractor') {
      // Oil Extractor: default to Normal
      formData.value.purity = 'Normal'
    } else if (formData.value.extractor_type === 'WaterExtractor') {
      // Water Extractor: no purity needed
      formData.value.purity = null
    } else if (formData.value.extractor_type === 'ResourceWellExtractor') {
      // Resource Well Extractor: always use pressurizer
      // Initialize pressurizer
      formData.value.pressurizer = {
        id: 0, // Will be assigned by backend
        clock_speed: 100,
        power_consumption: 0 // Will be calculated by backend
      }

      // Initialize with one extractor (only if item is selected)
      if (formData.value.extractors.length === 0 && formData.value.item) {
        const clockSpeed = formData.value.pressurizer.clock_speed
        formData.value.extractors = [{
          id: 0, // Will be assigned by backend
          item: formData.value.item as Item,
          purity: 'Normal',
          quantity_per_min: calculateExtractorRate('Normal', clockSpeed),
          count: 1
        }]
      }
    }
  }

  // Reset OC and count to defaults when changing extractor
  if (!isEditing.value) {
    formData.value.overclock_percent = 100
    formData.value.count = 1
  }

  // Fetch preview to get calculated rate (will auto-fill via watch)
  await fetchPreview()
}

const handlePurityChange = async () => {
  // When purity changes, fetch preview to autofill the base value
  await fetchPreview()
}

// Sticky values for OC slider (snap points)
const STICKY_OC_VALUES = [0, 50, 100, 125, 150, 175, 200, 225, 250]
const STICKY_THRESHOLD = 2.0 // Snap if within 2% of a sticky value

const handleOverclockSliderChange = async (event: Event) => {
  const target = event.target as HTMLInputElement
  let value = parseFloat(target.value)
  
  // Check if we're close to a sticky value and snap to it
  for (const sticky of STICKY_OC_VALUES) {
    if (Math.abs(value - sticky) <= STICKY_THRESHOLD) {
      value = sticky
      formData.value.overclock_percent = value
      target.value = value.toString()
      break
    }
  }
  
  // Clamp OC value
  if (value < 0) value = 0
  if (value > 250) value = 250
  
  formData.value.overclock_percent = value
  await fetchPreview()
}

const handleOverclockChange = async () => {
  // Clamp OC value
  if (formData.value.overclock_percent < 0) formData.value.overclock_percent = 0
  if (formData.value.overclock_percent > 250) formData.value.overclock_percent = 250
  await fetchPreview()
}

const handleCountChange = async () => {
  // Clamp count value
  if (formData.value.count < 1) formData.value.count = 1
  await fetchPreview()
}

const handlePressurizerOcSliderChange = async (event: Event) => {
  if (!formData.value.pressurizer) return
  
  const target = event.target as HTMLInputElement
  let value = parseFloat(target.value)
  
  // Check if we're close to a sticky value and snap to it (UI behavior, not calculation)
  for (const sticky of STICKY_OC_VALUES) {
    // Math.abs used for UI sticky value detection (display formatting)
    if (Math.abs(value - sticky) <= STICKY_THRESHOLD) {
      value = sticky
      formData.value.pressurizer.clock_speed = value
      target.value = value.toString()
      break
    }
  }
  
  // Clamp pressurizer OC value
  if (value < 0) value = 0
  if (value > 250) value = 250
  
  formData.value.pressurizer.clock_speed = value
  // Update individual extractor rates
  updateExtractorRates()
  await fetchPreview()
}

const handlePressurizerOcChange = async () => {
  // Clamp pressurizer OC value
  if (formData.value.pressurizer) {
    if (formData.value.pressurizer.clock_speed < 0) formData.value.pressurizer.clock_speed = 0
    if (formData.value.pressurizer.clock_speed > 250) formData.value.pressurizer.clock_speed = 250
  }
  // Update individual extractor rates
  updateExtractorRates()
  await fetchPreview()
}

const addExtractor = (event?: Event) => {
  // Prevent form submission if button is inside a form
  if (event) {
    event.preventDefault()
    event.stopPropagation()
  }
  
  if (!formData.value.item) return // Need item selected first
  
  const clockSpeed = formData.value.pressurizer?.clock_speed || 100
  const newExtractor = {
    id: 0, // Will be assigned by backend
    item: formData.value.item as Item,
    purity: 'Normal' as Purity,
    quantity_per_min: calculateExtractorRate('Normal', clockSpeed),
    count: 1
  }
  formData.value.extractors.push(newExtractor)
  // Fetch preview to update total rate
  fetchPreview()
}

const handleExtractorPurityChange = async (index: number) => {
  // Update the individual extractor rate
  if (formData.value.pressurizer) {
    const extractor = formData.value.extractors[index]
    if (extractor) {
      extractor.quantity_per_min = calculateExtractorRate(extractor.purity, formData.value.pressurizer.clock_speed)
    }
  }
  // Fetch preview to update total rate
  await fetchPreview()
}

const handleExtractorCountChange = async (index: number) => {
  // Clamp count value
  const extractor = formData.value.extractors[index]
  if (extractor && extractor.count !== undefined) {
    if (extractor.count < 1) extractor.count = 1
  }
  // Fetch preview to update total rate
  await fetchPreview()
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
    overclock_percent: 100,
    count: 1,
    quantity_per_min: 0,
    pressurizer: null,
    extractors: []
  }
}

const loadRawInput = () => {
  if (!props.rawInput) return

  formData.value = {
    item: props.rawInput.item,
    extractor_type: props.rawInput.extractor_type,
    purity: props.rawInput.purity,
    overclock_percent: props.rawInput.overclock_percent ?? 100,
    count: props.rawInput.count ?? 1,
    quantity_per_min: props.rawInput.quantity_per_min,
    pressurizer: props.rawInput.pressurizer,
    extractors: [...props.rawInput.extractors]
  }

  // Group extractors by purity and add count
  if (props.rawInput.extractors && props.rawInput.extractors.length > 0) {
    const grouped = new Map<string, { extractor: ResourceWellExtractor; count: number }>()
    props.rawInput.extractors.forEach(ext => {
      const key = `${ext.purity}`
      if (grouped.has(key)) {
        grouped.get(key)!.count++
      } else {
        grouped.set(key, { extractor: ext, count: 1 })
      }
    })
    formData.value.extractors = Array.from(grouped.values()).map(g => ({
      ...g.extractor,
      count: g.count
    }))
    // Update extractor rates based on pressurizer clock speed
    if (formData.value.pressurizer) {
      updateExtractorRates()
    }
  }
}

const handleSubmit = async () => {
  if (!canSubmit.value) return

  saving.value = true

  try {
    const payload: CreateRawInputRequest = {
      extractor_type: formData.value.extractor_type,
      item: formData.value.item as Item,
      overclock_percent: formData.value.overclock_percent,
      count: formData.value.count,
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

    if (showPressurizer.value && formData.value.pressurizer) {
      payload.pressurizer = {
        id: formData.value.pressurizer?.id ?? undefined,
        clock_speed: formData.value.pressurizer?.clock_speed ?? 100,
      }
      // Expand extractors by count: each extractor entry with count N becomes N extractors
      let extractorIdCounter = 1
      payload.extractors = formData.value.extractors.flatMap((extractor) => {
        const count = extractor.count || 1
        return Array(count).fill(null).map(() => {
          const id = extractor.id || extractorIdCounter++
          return {
            id: id !== 0 ? id : undefined,
            purity: extractor.purity,
          }
        })
      })
      // For pressurizer, OC is in the pressurizer clock_speed
      payload.overclock_percent = formData.value.pressurizer?.clock_speed ?? 100
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

// Calculate individual extractor rate based on purity and pressurizer clock speed
// Formula matches engine: base_rate * (clock_speed / 100)
// Base rates: Impure 30, Normal 60, Pure 120 m³/min at 100% clock
const calculateExtractorRate = (purity: Purity, clockSpeed: number): number => {
  const baseRate = purity === 'Impure' ? 30.0 : purity === 'Normal' ? 60.0 : 120.0
  return baseRate * (clockSpeed / 100.0)
}

// Update individual extractor rates when purity or pressurizer OC changes
const updateExtractorRates = () => {
  if (!showPressurizer.value || !formData.value.pressurizer) return
  
  const clockSpeed = formData.value.pressurizer.clock_speed
  formData.value.extractors.forEach(extractor => {
    extractor.quantity_per_min = calculateExtractorRate(extractor.purity, clockSpeed)
  })
}

// Watch for extractor changes to fetch preview and update rates
watch(() => formData.value.extractors, () => {
  if (showPressurizer.value) {
    updateExtractorRates()
    fetchPreview()
  }
}, { deep: true })

// Watch for pressurizer clock speed changes to update extractor rates
watch(() => formData.value.pressurizer?.clock_speed, () => {
  if (showPressurizer.value) {
    updateExtractorRates()
  }
})

// Watch for item changes to update extractor items and rates
watch(() => formData.value.item, () => {
  if (showPressurizer.value && formData.value.extractors.length > 0) {
    formData.value.extractors.forEach(extractor => {
      extractor.item = formData.value.item as Item
    })
    updateExtractorRates()
    fetchPreview()
  }
})

// Watch form changes to fetch preview and auto-fill quantity
watch([() => formData.value.extractor_type, () => formData.value.item, () => formData.value.purity, () => formData.value.overclock_percent, () => formData.value.count, () => formData.value.pressurizer?.clock_speed], async () => {
  await fetchPreview()
  // Auto-fill quantity_per_min when preview is available (always update since field is read-only)
  if (previewData.value?.quantity_per_min && previewData.value.quantity_per_min > 0) {
    formData.value.quantity_per_min = previewData.value.quantity_per_min
  }
}, { deep: true })

// Watch preview data changes to update quantity
watch(() => previewData.value?.quantity_per_min, (newQuantity) => {
  if (newQuantity && newQuantity > 0) {
    // Always update since the field is read-only and shows calculated value
    formData.value.quantity_per_min = newQuantity
  }
})

// Load items and extractor compatible items on mount
onMounted(async () => {
  await Promise.all([
    gameDataStore.fetchItems(),
    loadExtractorCompatibleItems()
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
  background-color: var(--color-surface-inset, #1f1f1f);
  border-radius: var(--border-radius-md, 0.375rem);
  border: 1px solid var(--color-border, #404040);
}

.extractors-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md, 0.75rem);
}

.extractor-item {
  padding: var(--spacing-md, 0.75rem);
  background-color: var(--color-surface-inset, #1f1f1f);
  border-radius: var(--border-radius-md, 0.375rem);
  border: 1px solid var(--color-border, #404040);
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-md, 0.75rem);
  justify-content: space-between;
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

.slider-container {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 0.5rem);
  min-width: 100px;
}

.form-slider {
  flex: 1 1 auto;
  min-width: 0;
  height: 0.5rem;
  border-radius: var(--border-radius-md, 0.375rem);
  background: var(--color-gray-200, #e5e7eb);
  outline: none;
  -webkit-appearance: none;
  appearance: none;

  &::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 1.25rem;
    height: 1.25rem;
    border-radius: 50%;
    background: var(--color-primary-500, #3b82f6);
    cursor: pointer;
  }

  &::-moz-range-thumb {
    width: 1.25rem;
    height: 1.25rem;
    border-radius: 50%;
    background: var(--color-primary-500, #3b82f6);
    cursor: pointer;
    border: none;
  }
}

.form-row {
  display: flex;
  gap: var(--spacing-md, 0.75rem);
  align-items: flex-start;
}

.form-col {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.form-col-oc {
  flex: 1 1 auto;
  min-width: 0;
  max-width: calc(100% - 90px);
}

.form-col-count {
  flex: 0 0 auto;
  width: 70px;
  margin-right: var(--spacing-md, 0.75rem);
}

.count-input {
  width: 100%;
}

.extractor-count-input {
  width: 70px;
  flex-shrink: 0;
}

.rate-input {
  max-width: 150px;
}

.oc-input {
  width: 90px;
  flex-shrink: 0;
}

.clock-speed-input {
  max-width: 150px;
  width: 100%;
}

.rate-total {
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-primary-600, #2563eb);
}

.power-hint {
  margin-top: var(--spacing-xs, 0.25rem);
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-600, #4b5563);
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
