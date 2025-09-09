<template>
  <div class="dashboard">
    <header class="dashboard-header">
      <h1>Factory Dashboard</h1>
      <div class="dashboard-actions">
        <button @click="showAddFactory = true" class="btn btn-primary">
          <span class="icon">🏭</span>
          Add Factory
        </button>
        <button @click="showAddProductionLine = true" class="btn btn-success">
          <span class="icon">⚙️</span>
          Add Production Line
        </button>
        <button @click="showAddLogistics = true" class="btn btn-info">
          <span class="icon">🚛</span>
          Add Logistics
        </button>
        <button @click="exportData" class="btn btn-secondary">
          Export Data
        </button>
        <button @click="loadDemo" class="btn btn-secondary">
          Load Sample Demo
        </button>
      </div>
    </header>

    <div class="dashboard-content">
      <div class="stats-grid">
        <div class="stat-card">
          <h3>🏭 Factories</h3>
          <p class="stat-value">{{ factoryCount }}</p>
        </div>
        <div class="stat-card">
          <h3>⚙️ Production Lines</h3>
          <p class="stat-value">{{ productionLineCount }}</p>
        </div>
        <div class="stat-card">
          <h3>📦 Items Tracked</h3>
          <p class="stat-value">{{ itemCount }}</p>
        </div>
        <div class="stat-card">
          <h3>🚛 Logistics Lines</h3>
          <p class="stat-value">{{ logisticsCount }}</p>
        </div>
      </div>

      <div class="main-content">
        <div class="factory-section" v-if="factories.length > 0">
          <div class="section-header">
            <h2>Your Factories</h2>
            <button @click="refreshData" class="btn btn-small btn-secondary">
              🔄 Refresh
            </button>
          </div>
          
          <div class="factories-grid">
            <div v-for="factory in factories" :key="factory.id" class="factory-card">
              <div class="factory-header">
                <h3>{{ factory.name }}</h3>
                <div class="factory-stats">
                  <span class="stat">{{ factory.production_lines?.length || 0 }} lines</span>
                  <span class="stat">{{ factory.raw_inputs?.length || 0 }} inputs</span>
                </div>
              </div>
              
              <div class="factory-content">
                <div v-if="factory.production_lines?.length > 0" class="production-lines">
                  <h4>Production Lines</h4>
                  <div v-for="line in factory.production_lines" :key="line.id" class="production-line">
                    <div class="line-info">
                      <span class="recipe">{{ line.recipe_id }}</span>
                      <span class="machines">{{ line.machine_count }}x</span>
                      <span class="clock">{{ Math.round(line.clock_ratio * 100) }}%</span>
                      <span v-if="line.strange_matter_boosted > 0" class="boosters">
                        +{{ line.strange_matter_boosted }} boosted
                      </span>
                    </div>
                  </div>
                </div>
                
                <div v-if="factory.raw_inputs?.length > 0" class="raw-inputs">
                  <h4>Raw Inputs</h4>
                  <div v-for="input in factory.raw_inputs" :key="input.item" class="raw-input">
                    <span class="item">{{ input.item }}</span>
                    <span class="quantity">{{ input.quantity_per_min }}/min</span>
                  </div>
                </div>
                
                <div v-if="!factory.production_lines?.length && !factory.raw_inputs?.length" class="empty-factory">
                  <p>No production lines or inputs configured</p>
                  <button @click="showAddProductionLine = true" class="btn btn-small btn-primary">
                    Add Production Line
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <div class="welcome-message" v-else>
          <h2>Welcome to SatisFlow!</h2>
          <p>Start by creating your first factory to track production.</p>
          <button @click="showAddFactory = true" class="btn btn-primary btn-large">
            🏭 Create Your First Factory
          </button>
        </div>

        <!-- Production Overview Section -->
        <div v-if="overview.length > 0" class="overview-section">
          <h2>Production Overview</h2>
          <div class="overview-table">
            <div class="table-header">
              <div class="col-item">Item</div>
              <div class="col-produced">Produced/min</div>
              <div class="col-consumed">Consumed/min</div>
              <div class="col-available">Available/min</div>
              <div class="col-status">Status</div>
            </div>
            <div v-for="item in overview" :key="item.item_id" class="table-row">
              <div class="col-item">{{ item.item_name }}</div>
              <div class="col-produced">{{ item.total_produced_per_min.toFixed(1) }}</div>
              <div class="col-consumed">{{ item.total_consumed_per_min.toFixed(1) }}</div>
              <div class="col-available">{{ item.available_per_min.toFixed(1) }}</div>
              <div class="col-status">
                <span :class="['status', item.status.toLowerCase()]">
                  {{ getStatusIcon(item.status) }} {{ item.status }}
                </span>
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
    
    <AddLogisticsModal 
      :isOpen="showAddLogistics" 
      @close="showAddLogistics = false" 
      @logistics-added="onLogisticsAdded"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import { getCounts, exportJson, loadSample, getFactories, getOverview, getLogisticsFluxes } from '../lib/tracker'
import AddFactoryModal from '../components/AddFactoryModal.vue'
import AddProductionLineModal from '../components/AddProductionLineModal.vue'
import AddLogisticsModal from '../components/AddLogisticsModal.vue'

const factoryCount = ref(0)
const productionLineCount = ref(0)
const itemCount = ref(0)
const logisticsCount = ref(0)
const factories = ref<any[]>([])
const overview = ref<any[]>([])

// Modal states
const showAddFactory = ref(false)
const showAddProductionLine = ref(false)
const showAddLogistics = ref(false)

const refreshData = async () => {
  try {
    const [counts, factoryList, overviewData, logisticsData] = await Promise.all([
      getCounts(),
      getFactories(),
      getOverview(),
      getLogisticsFluxes()
    ])
    
    factoryCount.value = counts.factories
    productionLineCount.value = counts.lines
    itemCount.value = counts.items
    logisticsCount.value = logisticsData.length
    factories.value = factoryList
    overview.value = overviewData
  } catch (error) {
    console.error('Failed to refresh data:', error)
  }
}

const exportData = async () => {
  try {
    // Show loading state
    const button = document.querySelector('.btn-secondary')
    const originalText = button?.textContent
    if (button) button.textContent = 'Exporting...'
    
    const json = await exportJson()
    const blob = new Blob([json], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    
    // Generate filename with timestamp
    const now = new Date()
    const timestamp = now.toISOString().slice(0, 19).replace(/[:.]/g, '-')
    a.download = `satisfflow-factory-${timestamp}.json`
    
    a.click()
    URL.revokeObjectURL(url)
    
    // Show success message
    alert('Factory data exported successfully!')
    
    // Restore button text
    if (button) button.textContent = originalText
  } catch (error) {
    console.error('Export failed:', error)
    alert('Failed to export factory data: ' + error.message)
  }
}

const loadDemo = async () => {
  await loadSample()
  await refreshData()
}

const onFactoryCreated = () => {
  refreshData()
}

const onLineAdded = () => {
  refreshData()
}

const onLogisticsAdded = () => {
  refreshData()
}

const getStatusIcon = (status: string) => {
  switch (status) {
    case 'Balanced': return '✅'
    case 'Overflow': return '⚠️'
    case 'Underflow': return '❌'
    default: return '❓'
  }
}

onMounted(async () => {
  await refreshData()
})

const route = useRoute()

// Refresh data when route changes to dashboard
watch(() => route.path, async (newPath) => {
  if (newPath === '/dashboard') {
    await refreshData()
  }
})
</script>

<style scoped>
.dashboard {
  max-width: 1400px;
  margin: 0 auto;
  padding: var(--spacing-xl);
}

.dashboard-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-2xl);
}

.dashboard-header h1 {
  color: var(--color-text-primary);
  margin: 0;
  font-size: var(--font-size-2xl);
  font-weight: 700;
}

.dashboard-actions {
  display: flex;
  gap: var(--spacing-lg);
  flex-wrap: wrap;
}


.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: var(--spacing-xl);
  margin-bottom: var(--spacing-2xl);
}

.stat-card {
  background: var(--gradient-card);
  border: 1px solid var(--color-border);
  padding: var(--spacing-2xl);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-md);
  text-align: center;
  transition: all var(--transition-normal);
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
  border-color: var(--color-primary-light);
}

.stat-card h3 {
  color: var(--color-text-secondary);
  margin: 0 0 var(--spacing-md) 0;
  font-size: var(--font-size-sm);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  font-weight: 600;
}

.stat-value {
  font-size: var(--font-size-3xl);
  font-weight: 700;
  color: var(--color-primary);
  margin: 0;
}

.main-content {
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-lg);
  padding: var(--spacing-2xl);
}

.welcome-message {
  text-align: center;
  padding: 2rem;
}

.welcome-message h2 {
  color: var(--color-text-primary);
  margin-bottom: 1rem;
}

.welcome-message p {
  color: #666;
  margin-bottom: 2rem;
  font-size: 1.1rem;
}

.placeholder {
  color: #666;
  font-style: italic;
  text-align: center;
  padding: 2rem;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.section-header h2 {
  color: var(--color-text-primary);
  margin: 0;
}


.factories-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(450px, 1fr));
  gap: var(--spacing-2xl);
  margin-bottom: var(--spacing-2xl);
}

.factory-card {
  background: var(--gradient-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-xl);
  padding: var(--spacing-2xl);
  box-shadow: var(--shadow-md);
  transition: all var(--transition-normal);
}

.factory-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
  border-color: var(--color-primary-light);
}

.factory-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
}

.factory-header h3 {
  color: var(--color-text-primary);
  margin: 0;
  font-size: 1.25rem;
}

.factory-stats {
  display: flex;
  gap: 1rem;
}

.factory-stats .stat {
  background: #f8f9fa;
  padding: 0.25rem 0.75rem;
  border-radius: 20px;
  font-size: 0.85rem;
  color: #666;
  border: 1px solid #e9ecef;
}

.factory-content h4 {
  color: #374151;
  margin: 1rem 0 0.5rem 0;
  font-size: 1rem;
  font-weight: 600;
}

.production-lines {
  margin-bottom: 1.5rem;
}

.production-line {
  background: #f8f9fa;
  padding: 0.75rem;
  border-radius: 6px;
  margin-bottom: 0.5rem;
  border-left: 4px solid #3498db;
}

.line-info {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
  align-items: center;
}

.line-info .recipe {
  font-weight: 600;
  color: var(--color-text-primary);
}

.line-info .machines {
  background: #e3f2fd;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.85rem;
  color: #1976d2;
}

.line-info .clock {
  background: #f3e5f5;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.85rem;
  color: #7b1fa2;
}

.line-info .boosters {
  background: #fff3e0;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.85rem;
  color: #ef6c00;
}

.raw-inputs {
  margin-bottom: 1.5rem;
}

.raw-input {
  display: flex;
  justify-content: space-between;
  padding: 0.5rem;
  background: #f0f9ff;
  border-radius: 4px;
  margin-bottom: 0.25rem;
  border-left: 4px solid #0ea5e9;
}

.raw-input .item {
  font-weight: 500;
  color: #0f172a;
}

.raw-input .quantity {
  color: #0369a1;
  font-weight: 600;
}

.empty-factory {
  text-align: center;
  padding: 2rem;
  color: #666;
  background: #f8f9fa;
  border-radius: 6px;
  border: 2px dashed #dee2e6;
}

.empty-factory p {
  margin-bottom: 1rem;
}

.overview-section {
  margin-top: 3rem;
  padding-top: 2rem;
  border-top: 1px solid #e5e7eb;
}

.overview-section h2 {
  color: var(--color-text-primary);
  margin-bottom: 1.5rem;
}

.overview-table {
  background: var(--color-bg-card);
  border-radius: var(--radius-lg);
  border: 1px solid var(--color-border);
  overflow: hidden;
  box-shadow: var(--shadow-md);
}

.table-header {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr 1fr 1.2fr;
  gap: var(--spacing-lg);
  padding: var(--spacing-lg);
  background: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border);
  font-weight: 600;
  color: var(--color-text-secondary);
  font-size: var(--font-size-md);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.table-row {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr 1fr 1.2fr;
  gap: var(--spacing-lg);
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--color-border-light);
  align-items: center;
  transition: all var(--transition-fast);
}

.table-row:last-child {
  border-bottom: none;
}

.table-row:hover {
  background: var(--color-bg-hover);
  transform: translateX(2px);
  border-left: 3px solid var(--color-primary);
}

.col-item {
  font-weight: 600;
  color: var(--color-production);
  font-size: var(--font-size-lg);
}

.col-produced,
.col-consumed,
.col-available {
  font-family: var(--font-family-mono);
  font-weight: 600;
  color: var(--color-logistics);
  font-size: var(--font-size-md);
}

.status {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.75rem;
  border-radius: 20px;
  font-size: 0.85rem;
  font-weight: 500;
}

.status.balanced {
  background: #dcfce7;
  color: #166534;
}

.status.overflow {
  background: #fef3c7;
  color: #92400e;
}

.status.underflow {
  background: #fee2e2;
  color: #991b1b;
}

.icon {
  margin-right: 0.5rem;
}
</style>
