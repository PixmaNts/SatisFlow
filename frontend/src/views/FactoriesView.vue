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
      <!-- Factory Header -->
      <div class="factory-info-card">
        <div class="factory-title">
          <h2>{{ selectedFactory.name }}</h2>
          <div class="factory-stats">
            <span class="stat">{{ selectedFactory.production_lines?.length || 0 }} production lines</span>
            <span class="stat">{{ selectedFactory.raw_inputs?.length || 0 }} raw inputs</span>
          </div>
        </div>
        <div class="factory-actions">
          <button @click="showAddProductionLine = true" class="btn btn-success">
            <span class="icon">⚙️</span>
            Add Production Line
          </button>
        </div>
      </div>

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
        <!-- Empty Factory -->
        <div v-if="!selectedFactory.production_lines?.length" class="empty-factory">
          <h3>No Production Lines</h3>
          <p>This factory doesn't have any production lines yet.</p>
          <button @click="showAddProductionLine = true" class="btn btn-success btn-large">
            ⚙️ Add First Production Line
          </button>
        </div>

        <!-- Production Lines Table -->
        <div v-else-if="filteredProductionLines.length > 0" :class="['production-table', `density-${displayDensity}`]">
          <div class="table-header">
            <div class="col-line-id">Line ID</div>
            <div class="col-recipe">Recipe</div>
            <div class="col-machines">Machines</div>
            <div class="col-clock">Clock Speed</div>
            <div class="col-boosters">Boosters</div>
            <div class="col-group">Group</div>
            <div class="col-efficiency">Efficiency</div>
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
          </div>
        </div>

        <!-- No Results -->
        <div v-else class="no-results">
          <p>No production lines match the current filters.</p>
          <button @click="clearFilters" class="btn btn-secondary">Clear Filters</button>
        </div>
      </div>

      <!-- Raw Inputs Section -->
      <div v-if="selectedFactory.raw_inputs?.length > 0" class="raw-inputs-section">
        <h3>Raw Material Inputs</h3>
        <div class="raw-inputs-grid">
          <div 
            v-for="input in selectedFactory.raw_inputs" 
            :key="input.item" 
            class="raw-input-card"
          >
            <div class="input-item">{{ input.item }}</div>
            <div class="input-quantity">{{ input.quantity_per_min }}/min</div>
            <div class="input-source">{{ input.source_type }}</div>
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { getFactories } from '../lib/tracker'
import AddFactoryModal from '../components/AddFactoryModal.vue'
import AddProductionLineModal from '../components/AddProductionLineModal.vue'

const factories = ref<any[]>([])
const selectedFactoryId = ref('')
const displayDensity = ref('comfortable')
const showAddFactory = ref(false)
const showAddProductionLine = ref(false)

// Filters
const groupFilter = ref('')
const recipeFilter = ref('')

const refreshData = async () => {
  try {
    factories.value = await getFactories()
    
    // Auto-select factory if only one exists
    if (factories.value.length === 1 && !selectedFactoryId.value) {
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
  grid-template-columns: 2fr 2fr 1fr 1fr 1fr 1.5fr 1fr;
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
  grid-template-columns: 2fr 2fr 1fr 1fr 1fr 1.5fr 1fr;
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

.raw-inputs-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}

.raw-input-card {
  background: #f0f9ff;
  border: 1px solid #bae6fd;
  border-left: 4px solid #0ea5e9;
  border-radius: 6px;
  padding: 1rem;
}

.input-item {
  font-weight: 600;
  color: #0f172a;
  margin-bottom: 0.5rem;
}

.input-quantity {
  color: #0369a1;
  font-weight: 600;
  font-size: 1.1rem;
}

.input-source {
  color: #64748b;
  font-size: 0.85rem;
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