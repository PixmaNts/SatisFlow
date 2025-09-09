<template>
  <div class="factories-view">
    <header class="factories-header">
      <div class="header-content">
        <h1>Factory Management</h1>
        <div v-if="factories.length > 0" class="factory-selector">
          <label for="factorySelect">Viewing Factory:</label>
          <select id="factorySelect" v-model="selectedFactoryId" class="factory-select">
            <option value="">Select a factory...</option>
            <option v-for="factory in factories" :key="factory.id" :value="factory.id">
              {{ factory.name }}
            </option>
          </select>
        </div>
      </div>
      <div class="header-actions">
        <button @click="refreshData" class="btn btn-secondary">
          🔄 Refresh
        </button>
        <button @click="showAddFactory = true" class="btn btn-primary">
          <span class="icon">🏭</span>
          Add Factory
        </button>
      </div>
    </header>

    <!-- No Factories State -->
    <div v-if="factories.length === 0" class="empty-state">
      <h2>Welcome to SatisFlow!</h2>
      <p>Start by creating your first factory to manage production.</p>
      <button @click="showAddFactory = true" class="btn btn-primary btn-large">
        🏭 Create Your First Factory
      </button>
    </div>

    <!-- No Factory Selected -->
    <div v-else-if="!selectedFactoryId" class="no-selection">
      <h2>Select a Factory</h2>
      <p>Choose a factory from the dropdown above to view and manage its production lines.</p>
    </div>

    <!-- Selected Factory View -->
    <div v-else-if="selectedFactory" class="factory-view">
      <!-- Factory Tabs -->
      <div class="factory-tabs">
        <div class="tab-list">
          <button 
            :class="['tab-button', { active: activeTab === 'production-lines' }]"
            @click="activeTab = 'production-lines'"
          >
            <span class="tab-icon">⚙️</span>
            Production Lines
            <span class="tab-count">{{ selectedFactory.production_lines?.length || 0 }}</span>
          </button>
          <button 
            :class="['tab-button', { active: activeTab === 'raw-inputs' }]"
            @click="activeTab = 'raw-inputs'"
          >
            <span class="tab-icon">⛏️</span>
            Raw Inputs
            <span class="tab-count">{{ selectedFactory.raw_inputs?.length || 0 }}</span>
          </button>
        </div>
      </div>

      <!-- Production Lines Tab -->
      <div v-if="activeTab === 'production-lines'" class="tab-content">
        <!-- Filters and Density Controls -->
        <div v-if="selectedFactory.production_lines?.length > 0" class="controls-section">
          <div class="controls-grid">
            <div class="control-group">
              <label for="densityControl">Display Density</label>
              <select id="densityControl" v-model="displayDensity" class="control-select">
                <option value="comfortable">Comfortable</option>
                <option value="compact">Compact</option>
                <option value="dense">Dense</option>
              </select>
            </div>

            <div class="control-group">
              <label for="groupFilter">Filter by Group</label>
              <select id="groupFilter" v-model="groupFilter" class="control-select">
                <option value="">All Groups</option>
                <option v-for="group in uniqueGroups" :key="group" :value="group">
                  {{ group || 'Ungrouped' }}
                </option>
              </select>
            </div>

            <div class="control-group">
              <label for="recipeFilter">Filter by Recipe</label>
              <select id="recipeFilter" v-model="recipeFilter" class="control-select">
                <option value="">All Recipes</option>
                <option v-for="recipe in uniqueRecipes" :key="recipe" :value="recipe">
                  {{ recipe }}
                </option>
              </select>
            </div>

            <div class="control-actions">
              <button @click="clearFilters" class="btn btn-small btn-secondary">
                Clear Filters
              </button>
            </div>
          </div>
        </div>

        <!-- Production Lines Content -->
        <div class="production-content">
          <!-- Production Lines Table -->
          <div :class="['production-table', `density-${displayDensity}`]">
            <div class="table-header">
              <div class="col-line-id">Line ID</div>
              <div class="col-recipe">Recipe</div>
              <div class="col-machines">Machines</div>
              <div class="col-clock">Clock Speed</div>
              <div class="col-boosters">Boosters</div>
              <div class="col-group">Group</div>
              <div class="col-efficiency">Efficiency</div>
              <div class="col-actions">Actions</div>
            </div>
            
            <!-- Add Production Line Row -->
            <div class="table-row add-row">
              <div class="col-line-id">
                <span class="add-label">➕ New Line</span>
              </div>
              <div class="col-recipe">
                <button @click="showAddProductionLine = true" class="btn-add-inline">
                  <span class="icon">⚙️</span>
                  Add Production Line
                </button>
              </div>
              <div class="col-machines">—</div>
              <div class="col-clock">—</div>
              <div class="col-boosters">—</div>
              <div class="col-group">—</div>
              <div class="col-efficiency">—</div>
              <div class="col-actions">—</div>
            </div>
            
            <div 
              v-for="line in filteredProductionLines" 
              :key="line.id" 
              class="table-row"
            >
              <div class="col-line-id">
                <span class="line-id" :title="line.id">{{ shortenLineId(line.id) }}</span>
              </div>
              <div class="col-recipe">
                <span class="recipe-name">{{ line.recipe_id }}</span>
              </div>
              <div class="col-machines">
                <span class="quantity">{{ line.machine_count }}x</span>
              </div>
              <div class="col-clock">
                <span :class="['clock-speed', getClockSpeedClass(line.clock_ratio)]">
                  {{ Math.round(line.clock_ratio * 100) }}%
                </span>
              </div>
              <div class="col-boosters">
                <span v-if="line.strange_matter_boosted > 0" class="quantity">
                  {{ line.strange_matter_boosted }}
                </span>
                <span v-else>—</span>
              </div>
              <div class="col-group">
                <span class="source-type">{{ line.group_name || 'Ungrouped' }}</span>
              </div>
              <div class="col-efficiency">
                <span class="source-type">{{ getEfficiencyStatus(line) }}</span>
              </div>
              <div class="col-actions">
                <button 
                  @click="handleEditProductionLine(line)"
                  class="action-btn edit-btn"
                  title="Edit production line"
                >
                  ✏️
                </button>
              </div>
            </div>
          </div>

          <!-- No Results -->
          <div v-if="selectedFactory.production_lines?.length > 0 && filteredProductionLines.length === 0" class="no-results">
            <p>No production lines match the current filters.</p>
            <button @click="clearFilters" class="btn btn-secondary">Clear Filters</button>
          </div>
        </div>
      </div>

      <!-- Raw Inputs Tab -->
      <div v-if="activeTab === 'raw-inputs'" class="tab-content">
        <!-- Raw Inputs Table -->
        <div class="raw-inputs-content">
          <div :class="['raw-inputs-table', `density-${displayDensity}`]">
            <div class="table-header">
              <div class="col-item">Item</div>
              <div class="col-quantity">Quantity/min</div>
              <div class="col-source">Source Type</div>
              <div class="col-actions">Actions</div>
            </div>
            
            <!-- Add Raw Input Row -->
            <div class="table-row add-row">
              <div class="col-item">
                <span class="add-label">➕ New Raw Input</span>
              </div>
              <div class="col-quantity">
                <button @click="showAddRawInput = true" class="btn-add-inline">
                  <span class="icon">⛏️</span>
                  Add Raw Input
                </button>
              </div>
              <div class="col-source">—</div>
              <div class="col-actions">—</div>
            </div>
            
            <div 
              v-for="input in selectedFactory.raw_inputs" 
              :key="input.item" 
              class="table-row"
            >
              <div class="col-item">
                <span class="item-name">{{ getItemName(input.item) }}</span>
              </div>
              <div class="col-quantity">
                <span class="quantity">{{ input.quantity_per_min }}/min</span>
              </div>
              <div class="col-source">
                <span class="source-type">{{ input.source_type || 'Unknown' }}</span>
                <div v-if="input.comment" class="raw-input-comment">
                  {{ input.comment }}
                </div>
              </div>
              <div class="col-actions">
                <button 
                  @click="handleEditRawInput(input)"
                  class="action-btn edit-btn"
                  title="Edit raw input"
                >
                  ✏️
                </button>
                <button 
                  @click="handleRemoveRawInput(input.id)"
                  class="action-btn delete-btn"
                  title="Remove raw input"
                >
                  🗑️
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Modals -->
    <AddFactoryModal 
      :isOpen="showAddFactory" 
      @close="showAddFactory = false" 
      @factory-created="onFactoryCreated"
    />
    
    <AddProductionLineModal 
      :isOpen="showAddProductionLine" 
      :preselectedFactoryId="selectedFactoryId"
      @close="showAddProductionLine = false" 
      @line-added="onLineAdded"
    />
    
    <AddRawInputModal
      :isOpen="showAddRawInput"
      :factoryId="selectedFactoryId"
      @close="showAddRawInput = false"
      @raw-input-added="onRawInputAdded"
    />
    
    <EditRawInputModal
      :isOpen="showEditRawInput"
      :factoryId="selectedFactoryId"
      :rawInput="editingRawInput"
      :itemName="getItemName(editingRawInput?.item || '')"
      @close="showEditRawInput = false"
      @raw-input-updated="onRawInputUpdated"
    />
    
    <EditProductionLineModal
      :isOpen="showEditProductionLine"
      :productionLine="editingProductionLine"
      :factoryName="selectedFactory?.name || ''"
      :recipeName="editingProductionLine?.recipe_id || ''"
      @close="showEditProductionLine = false"
      @production-line-updated="onProductionLineUpdated"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { useRoute } from 'vue-router'
import { getFactories, removeRawInput, getItems } from '../lib/tracker'
import AddFactoryModal from '../components/AddFactoryModal.vue'
import AddProductionLineModal from '../components/AddProductionLineModal.vue'
import AddRawInputModal from '../components/AddRawInputModal.vue'
import EditRawInputModal from '../components/EditRawInputModal.vue'
import EditProductionLineModal from '../components/EditProductionLineModal.vue'

const factories = ref<any[]>([])
const selectedFactoryId = ref('')
const displayDensity = ref('comfortable')
const showAddFactory = ref(false)
const showAddProductionLine = ref(false)
const showAddRawInput = ref(false)
const showEditRawInput = ref(false)
const editingRawInput = ref<any>(null)
const showEditProductionLine = ref(false)
const editingProductionLine = ref<any>(null)
const items = ref<any[]>([])
const activeTab = ref<'raw-inputs' | 'production-lines'>('production-lines')

// Filters
const groupFilter = ref('')
const recipeFilter = ref('')

const refreshData = async () => {
  try {
    factories.value = await getFactories()
    items.value = await getItems()
    
    // Auto-select first factory if none is selected
    if (factories.value.length > 0 && !selectedFactoryId.value) {
      selectedFactoryId.value = factories.value[0].id
    }
    
    // Clear selection if selected factory no longer exists
    if (selectedFactoryId.value && !factories.value.find(f => f.id === selectedFactoryId.value)) {
      selectedFactoryId.value = ''
    }
  } catch (error) {
    console.error('Failed to refresh factory data:', error)
  }
}

const selectedFactory = computed(() => {
  return factories.value.find(f => f.id === selectedFactoryId.value) || null
})

const filteredProductionLines = computed(() => {
  const lines = selectedFactory.value?.production_lines || []
  return lines.filter(line => {
    if (groupFilter.value && (line.group_name || '') !== groupFilter.value) return false
    if (recipeFilter.value && line.recipe_id !== recipeFilter.value) return false
    return true
  })
})

const uniqueGroups = computed(() => {
  const groups = new Set((selectedFactory.value?.production_lines || []).map(line => line.group_name || ''))
  return Array.from(groups).sort()
})

const uniqueRecipes = computed(() => {
  const recipes = new Set((selectedFactory.value?.production_lines || []).map(line => line.recipe_id))
  return Array.from(recipes).sort()
})

const clearFilters = () => {
  groupFilter.value = ''
  recipeFilter.value = ''
}

const onFactoryCreated = (factory: any) => {
  refreshData()
  selectedFactoryId.value = factory.id
}

const onLineAdded = () => {
  refreshData()
  activeTab.value = 'production-lines'
}

const onRawInputAdded = () => {
  refreshData()
  activeTab.value = 'raw-inputs'
}

const onRawInputUpdated = () => {
  refreshData()
}

const onProductionLineUpdated = () => {
  refreshData()
}

const handleEditRawInput = (rawInput: any) => {
  editingRawInput.value = rawInput
  showEditRawInput.value = true
}

const handleEditProductionLine = (productionLine: any) => {
  editingProductionLine.value = productionLine
  showEditProductionLine.value = true
}

const handleRemoveRawInput = async (rawInputId: string) => {
  if (!selectedFactoryId.value) return
  
  if (confirm('Are you sure you want to remove this raw input?')) {
    try {
      await removeRawInput(selectedFactoryId.value, rawInputId)
      await refreshData()
    } catch (error) {
      console.error('Failed to remove raw input:', error)
      alert('Failed to remove raw input: ' + error.message)
    }
  }
}

const getItemName = (itemId: string) => {
  const item = items.value.find(i => i.id === itemId)
  return item ? item.name : itemId
}

const shortenLineId = (lineId: string) => {
  // Line IDs follow pattern: line_{factory_id}_{recipe_slug}_{number}
  // Extract just the recipe part and number for display by matching from the end
  const match = lineId.match(/^line_.*_(.+?)_(\d+)$/)
  if (match) {
    const [, recipeSlug, number] = match
    // Convert recipe slug back to readable format and add number
    const readableRecipe = recipeSlug
      .split('_')
      .map(word => word.charAt(0).toUpperCase() + word.slice(1))
      .join(' ')
    return `${readableRecipe} #${number}`
  }
  // Fallback: just remove "line_" prefix
  return lineId.replace(/^line_/, '')
}

const getClockSpeedClass = (clockRatio: number) => {
  const percentage = clockRatio * 100
  if (percentage < 100) return 'under-clocked'
  if (percentage > 100) return 'over-clocked'
  return 'normal-clock'
}

const getEfficiencyClass = (line: any) => {
  // This would need actual efficiency calculation logic
  return 'normal'
}

const getEfficiencyStatus = (line: any) => {
  // Placeholder - would calculate actual efficiency
  const boostedMachines = line.strange_matter_boosted || 0
  const totalMachines = line.machine_count || 0
  
  if (boostedMachines > 0) {
    return `${boostedMachines}/${totalMachines} boosted`
  }
  return 'Standard'
}

onMounted(async () => {
  await refreshData()
})

const route = useRoute()

// Refresh data when route changes to factories
watch(() => route.path, async (newPath) => {
  if (newPath === '/factories') {
    await refreshData()
  }
})
</script>

<style scoped>
.raw-input-comment {
  color: #6b7280;
  font-size: 0.8rem;
  font-style: italic;
  margin-top: 0.25rem;
  line-height: 1.3;
}
</style>