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
              <span class="line-id">{{ line.id }}</span>
            </div>
            <div class="col-recipe">
              <span class="recipe-name">{{ line.recipe_id }}</span>
            </div>
            <div class="col-machines">
              <span class="machine-count">{{ line.machine_count }}x</span>
            </div>
            <div class="col-clock">
              <span :class="['clock-speed', getClockSpeedClass(line.clock_ratio)]">
                {{ Math.round(line.clock_ratio * 100) }}%
              </span>
            </div>
            <div class="col-boosters">
              <span v-if="line.strange_matter_boosted > 0" class="boosters-count">
                {{ line.strange_matter_boosted }} boosted
              </span>
              <span v-else class="no-boosters">-</span>
            </div>
            <div class="col-group">
              <span class="group-name">{{ line.group_name || 'Ungrouped' }}</span>
            </div>
            <div class="col-efficiency">
              <span :class="['efficiency', getEfficiencyClass(line)]">
                {{ getEfficiencyStatus(line) }}
              </span>
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
                  @click="handleRemoveRawInput(input.item)"
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
import { ref, onMounted, computed } from 'vue'
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

const handleRemoveRawInput = async (itemId: string) => {
  if (!selectedFactoryId.value) return
  
  if (confirm('Are you sure you want to remove this raw input?')) {
    try {
      await removeRawInput(selectedFactoryId.value, itemId)
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
</script>

<style scoped>
.factories-view {
  max-width: 1400px;
  margin: 0 auto;
}

.factories-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 2rem;
  gap: 2rem;
}

.header-content {
  flex: 1;
}

.header-content h1 {
  color: #2c3e50;
  margin: 0 0 1rem 0;
}

.factory-selector {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.factory-selector label {
  font-weight: 500;
  color: #374151;
  white-space: nowrap;
}

.factory-select {
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 1rem;
  background: white;
  min-width: 250px;
}

.factory-select:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.header-actions {
  display: flex;
  gap: 1rem;
  flex-shrink: 0;
}

.btn {
  padding: 0.75rem 1.5rem;
  font-size: 0.9rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.3s ease;
  font-weight: 600;
}

.btn-large {
  padding: 1rem 2rem;
  font-size: 1.1rem;
}

.btn-small {
  padding: 0.5rem 1rem;
  font-size: 0.85rem;
}

.btn-primary {
  background-color: #3498db;
  color: white;
}

.btn-primary:hover {
  background-color: #2980b9;
}

.btn-secondary {
  background-color: white;
  color: #3498db;
  border: 2px solid #3498db;
}

.btn-secondary:hover {
  background-color: #3498db;
  color: white;
}

.btn-success {
  background-color: #27ae60;
  color: white;
}

.btn-success:hover {
  background-color: #229954;
}

.empty-state,
.no-selection {
  text-align: center;
  padding: 3rem;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.empty-state h2,
.no-selection h2 {
  color: #2c3e50;
  margin-bottom: 1rem;
}

.empty-state p,
.no-selection p {
  color: #666;
  margin-bottom: 2rem;
  font-size: 1.1rem;
}

.factory-view {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.factory-info-card {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 1.5rem;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.factory-title h2 {
  color: #2c3e50;
  margin: 0 0 0.75rem 0;
  font-size: 1.5rem;
}

.factory-stats {
  display: flex;
  gap: 1rem;
}

.factory-stats .stat {
  background: #f8f9fa;
  padding: 0.25rem 0.75rem;
  border-radius: 15px;
  font-size: 0.9rem;
  color: #666;
  border: 1px solid #e9ecef;
}

.factory-actions {
  flex-shrink: 0;
}

.controls-section {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 1.5rem;
}

.controls-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  align-items: end;
}

.control-group {
  display: flex;
  flex-direction: column;
}

.control-group label {
  font-weight: 500;
  color: #374151;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
}

.control-select {
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 1rem;
  background: white;
}

.control-select:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.control-actions {
  display: flex;
  align-items: end;
}

.production-content {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 1.5rem;
}

.empty-factory {
  text-align: center;
  padding: 2rem;
}

.empty-factory h3 {
  color: #2c3e50;
  margin-bottom: 1rem;
}

.empty-factory p {
  color: #666;
  margin-bottom: 2rem;
}

.production-table {
  border: 1px solid #e5e7eb;
  border-radius: 6px;
  overflow: hidden;
}

.table-header {
  display: grid;
  grid-template-columns: 2fr 2fr 1fr 1fr 1fr 1.5fr 1fr 1fr;
  gap: 1rem;
  padding: 1rem;
  background: #f8f9fa;
  border-bottom: 1px solid #e5e7eb;
  font-weight: 600;
  color: #374151;
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.table-row {
  display: grid;
  grid-template-columns: 2fr 2fr 1fr 1fr 1fr 1.5fr 1fr 1fr;
  gap: 1rem;
  padding: 1rem;
  border-bottom: 1px solid #f3f4f6;
  align-items: center;
}

.table-row:last-child {
  border-bottom: none;
}

.table-row:hover {
  background: #f8f9fa;
}

.line-id {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.85rem;
  color: #6b7280;
  background: #f3f4f6;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
}

.recipe-name {
  font-weight: 500;
  color: #1f2937;
}

.machine-count {
  font-weight: 600;
  color: #059669;
}

.clock-speed {
  font-weight: 600;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.85rem;
}

.clock-speed.normal-clock {
  background: #e0f2fe;
  color: #0277bd;
}

.clock-speed.over-clocked {
  background: #fff3e0;
  color: #ef6c00;
}

.clock-speed.under-clocked {
  background: #f3e5f5;
  color: #7b1fa2;
}

.boosters-count {
  background: #fff3e0;
  color: #ef6c00;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.85rem;
  font-weight: 500;
}

.no-boosters {
  color: #9ca3af;
}

.group-name {
  font-size: 0.9rem;
  color: #6b7280;
}

.efficiency {
  font-size: 0.85rem;
  font-weight: 500;
}

.efficiency.normal {
  color: #059669;
}

.col-actions {
  display: flex;
  justify-content: center;
  align-items: center;
}

.action-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 1.1rem;
  line-height: 1;
  padding: 0.5rem;
  border-radius: 4px;
  transition: all 0.2s;
}

.action-btn.edit-btn {
  color: #3b82f6;
}

.action-btn.edit-btn:hover {
  background-color: #eff6ff;
  color: #1d4ed8;
}

.raw-inputs-section {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 1.5rem;
}

.raw-inputs-section h3 {
  color: #2c3e50;
  margin: 0 0 1rem 0;
}


.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.section-header h3 {
  margin: 0;
}

.btn-info {
  background-color: #0ea5e9;
  color: white;
}

.btn-info:hover {
  background-color: #0284c7;
}

.btn-outline {
  background-color: transparent;
  border: 2px solid;
}

.btn-outline.btn-info {
  border-color: #0ea5e9;
  color: #0ea5e9;
}

.btn-outline.btn-info:hover {
  background-color: #0ea5e9;
  color: white;
}

.btn-outline.btn-success {
  border-color: #27ae60;
  color: #27ae60;
}

.btn-outline.btn-success:hover {
  background-color: #27ae60;
  color: white;
}

/* Factory Tabs */
.factory-tabs {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  margin-bottom: 2rem;
}

.tab-list {
  display: flex;
  border-bottom: 1px solid #e5e7eb;
}

.tab-button {
  flex: 1;
  padding: 1rem 1.5rem;
  background: none;
  border: none;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 1rem;
  font-weight: 500;
  color: #6b7280;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  position: relative;
}

.tab-button:first-child {
  border-top-left-radius: 8px;
}

.tab-button:last-child {
  border-top-right-radius: 8px;
}

.tab-button:hover {
  background-color: #f8f9fa;
  color: #374151;
}

.tab-button.active {
  color: #3b82f6;
  background-color: #eff6ff;
}

.tab-button.active::after {
  content: '';
  position: absolute;
  bottom: -1px;
  left: 0;
  right: 0;
  height: 3px;
  background-color: #3b82f6;
  border-radius: 2px 2px 0 0;
}

.tab-icon {
  font-size: 1.1rem;
}

.tab-count {
  background: #e5e7eb;
  color: #6b7280;
  padding: 0.25rem 0.5rem;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: 600;
  min-width: 1.5rem;
  text-align: center;
}

.tab-button.active .tab-count {
  background: #dbeafe;
  color: #3b82f6;
}

/* Tab Content */
.tab-content {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 1.5rem;
}

.empty-section {
  text-align: center;
  padding: 3rem 2rem;
}

.empty-section h3 {
  color: #2c3e50;
  margin-bottom: 1rem;
}

.empty-section p {
  color: #666;
  margin-bottom: 2rem;
  font-size: 1.1rem;
  line-height: 1.6;
}

.raw-inputs-content {
  /* Remove the background/shadow since it's now in tab-content */
  background: none;
  box-shadow: none;
  padding: 0;
}

/* Logistics Table */
.logistics-content {
  background: none;
  box-shadow: none;
  padding: 0;
}

.logistics-table {
  border: 1px solid #e5e7eb;
  border-radius: 6px;
  overflow: hidden;
}

.logistics-table .table-header {
  display: grid;
  grid-template-columns: 2fr 1.5fr 1fr 1.5fr 1fr;
  gap: 1rem;
  padding: 1rem;
  background: #f8f9fa;
  border-bottom: 1px solid #e5e7eb;
  font-weight: 600;
  color: #374151;
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.logistics-table .table-row {
  display: grid;
  grid-template-columns: 2fr 1.5fr 1fr 1.5fr 1fr;
  gap: 1rem;
  padding: 1rem;
  border-bottom: 1px solid #f3f4f6;
  align-items: center;
}

.logistics-table .table-row:last-child {
  border-bottom: none;
}

.logistics-table .table-row:hover {
  background: #f8f9fa;
}

.connection-flow {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.from-factory,
.to-factory {
  font-weight: 500;
  color: #1f2937;
}

.flow-arrow {
  color: #6b7280;
  font-weight: bold;
}

.item-name {
  font-weight: 500;
  color: #059669;
}

.quantity {
  font-weight: 600;
  color: #0369a1;
}

.transport-type {
  font-weight: 500;
  color: #7c3aed;
  display: block;
}

.transport-details {
  font-size: 0.8rem;
  color: #6b7280;
  display: block;
  margin-top: 0.25rem;
}


.no-results {
  text-align: center;
  padding: 2rem;
  color: #6b7280;
}

.icon {
  margin-right: 0.5rem;
}

/* Density Variations */
.production-table.density-compact .table-row,
.production-table.density-compact .table-header {
  padding: 0.6rem;
  font-size: 0.9rem;
}

.production-table.density-compact .line-id {
  font-size: 0.75rem;
  padding: 0.15rem 0.4rem;
}

.production-table.density-dense .table-row,
.production-table.density-dense .table-header {
  padding: 0.4rem;
  font-size: 0.85rem;
  line-height: 1.2;
}

.production-table.density-dense .line-id {
  font-size: 0.7rem;
  padding: 0.1rem 0.35rem;
}

.production-table.density-dense .clock-speed,
.production-table.density-dense .boosters-count {
  padding: 0.15rem 0.4rem;
  font-size: 0.75rem;
}

/* Raw Inputs Table Styles */
.raw-inputs-table {
  min-width: 100%;
  border: 1px solid #e5e7eb;
  border-radius: 6px;
  overflow: hidden;
}

.raw-inputs-table .table-header {
  display: grid;
  grid-template-columns: 2fr 1.5fr 1.5fr 1fr;
  gap: 1rem;
  padding: 1rem;
  background: #f8f9fa;
  border-bottom: 1px solid #e5e7eb;
  font-weight: 600;
  color: #374151;
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.raw-inputs-table .table-row {
  display: grid;
  grid-template-columns: 2fr 1.5fr 1.5fr 1fr;
  gap: 1rem;
  padding: 1rem;
  border-bottom: 1px solid #f3f4f6;
  align-items: center;
}

.raw-inputs-table .table-row:last-child {
  border-bottom: none;
}

.raw-inputs-table .table-row:not(.add-row):hover {
  background: #f8f9fa;
  transform: translateX(2px);
  transition: all 0.2s ease;
}

.raw-inputs-table .item-name {
  font-weight: 600;
  color: #059669;
  font-size: 1rem;
}

.raw-inputs-table .quantity {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-weight: 600;
  color: #7c3aed;
  font-size: 0.95rem;
}

.raw-inputs-table .source-type {
  font-weight: 500;
  color: #6b7280;
  font-size: 0.9rem;
}

/* Compact density for raw inputs */
.raw-inputs-table.density-compact .table-row,
.raw-inputs-table.density-compact .table-header {
  padding: 0.5rem;
  font-size: 0.9rem;
}

/* Dense density for raw inputs */
.raw-inputs-table.density-dense .table-row,
.raw-inputs-table.density-dense .table-header {
  padding: 0.35rem 0.5rem;
  font-size: 0.85rem;
  line-height: 1.2;
}

.raw-inputs-table.density-dense .item-name {
  font-size: 0.9rem;
}

.raw-inputs-table.density-dense .quantity {
  font-size: 0.85rem;
}

.raw-inputs-table.density-dense .source-type {
  font-size: 0.8rem;
}

/* Add Button Styles */
.add-row {
  background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
  border: 2px dashed #0ea5e9;
  border-radius: 8px;
  margin-bottom: 0.5rem;
}

.add-row:hover {
  background: linear-gradient(135deg, #e0f2fe 0%, #bae6fd 100%);
  transform: none;
}

.add-label {
  color: #0ea5e9;
  font-weight: 600;
  font-size: 0.9rem;
}

.btn-add-inline {
  background: #0ea5e9;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s ease;
}

.btn-add-inline:hover {
  background: #0284c7;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(14, 165, 233, 0.3);
}


/* Responsive */
@media (max-width: 768px) {
  .factories-header {
    flex-direction: column;
    align-items: stretch;
  }
  
  .factory-info-card {
    flex-direction: column;
    gap: 1rem;
  }
  
  .controls-grid {
    grid-template-columns: 1fr;
  }
  
  .table-header,
  .table-row {
    grid-template-columns: 1fr;
    gap: 0.5rem;
  }
  
  .table-header > div,
  .table-row > div {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .table-header > div::before {
    content: attr(class);
    font-weight: normal;
    text-transform: capitalize;
  }
}
</style>