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
          <h3>Factories</h3>
          <p class="stat-value">{{ factoryCount }}</p>
        </div>
        <div class="stat-card">
          <h3>Production Lines</h3>
          <p class="stat-value">{{ productionLineCount }}</p>
        </div>
        <div class="stat-card">
          <h3>Items Tracked</h3>
          <p class="stat-value">{{ itemCount }}</p>
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
import { ref, onMounted } from 'vue'
import { getCounts, exportJson, loadSample, getFactories, getOverview } from '../lib/tracker'
import AddFactoryModal from '../components/AddFactoryModal.vue'
import AddProductionLineModal from '../components/AddProductionLineModal.vue'
import AddLogisticsModal from '../components/AddLogisticsModal.vue'

const factoryCount = ref(0)
const productionLineCount = ref(0)
const itemCount = ref(0)
const factories = ref<any[]>([])
const overview = ref<any[]>([])

// Modal states
const showAddFactory = ref(false)
const showAddProductionLine = ref(false)
const showAddLogistics = ref(false)

const refreshData = async () => {
  try {
    const [counts, factoryList, overviewData] = await Promise.all([
      getCounts(),
      getFactories(),
      getOverview()
    ])
    
    factoryCount.value = counts.factories
    productionLineCount.value = counts.lines
    itemCount.value = counts.items
    factories.value = factoryList
    overview.value = overviewData
  } catch (error) {
    console.error('Failed to refresh data:', error)
  }
}

const exportData = async () => {
  const json = await exportJson()
  const blob = new Blob([json], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'satisflow_data.json'
  a.click()
  URL.revokeObjectURL(url)
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
</script>

<style scoped>
.dashboard {
  max-width: 1200px;
  margin: 0 auto;
}

.dashboard-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.dashboard-header h1 {
  color: #2c3e50;
  margin: 0;
}

.dashboard-actions {
  display: flex;
  gap: 1rem;
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

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.stat-card {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  text-align: center;
}

.stat-card h3 {
  color: #666;
  margin: 0 0 0.5rem 0;
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.stat-value {
  font-size: 2rem;
  font-weight: bold;
  color: #2c3e50;
  margin: 0;
}

.main-content {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 2rem;
}

.welcome-message {
  text-align: center;
  padding: 2rem;
}

.welcome-message h2 {
  color: #2c3e50;
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
  color: #2c3e50;
  margin: 0;
}

.btn-small {
  padding: 0.5rem 1rem;
  font-size: 0.85rem;
}

.btn-success {
  background-color: #27ae60;
  color: white;
}

.btn-success:hover {
  background-color: #229954;
}

.btn-info {
  background-color: #3498db;
  color: white;
}

.btn-info:hover {
  background-color: #2980b9;
}

.factories-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  gap: 2rem;
  margin-bottom: 2rem;
}

.factory-card {
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.factory-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
}

.factory-header h3 {
  color: #2c3e50;
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
  color: #2c3e50;
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
  color: #2c3e50;
  margin-bottom: 1.5rem;
}

.overview-table {
  background: white;
  border-radius: 6px;
  border: 1px solid #e5e7eb;
  overflow: hidden;
}

.table-header {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr 1fr 1fr;
  gap: 1rem;
  padding: 1rem;
  background: #f8f9fa;
  border-bottom: 1px solid #e5e7eb;
  font-weight: 600;
  color: #374151;
}

.table-row {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr 1fr 1fr;
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

.col-item {
  font-weight: 500;
  color: #1f2937;
}

.col-produced,
.col-consumed,
.col-available {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-weight: 500;
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
