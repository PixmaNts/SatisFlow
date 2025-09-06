<template>
  <div class="logistics-view">
    <header class="logistics-header">
      <h1>Logistics Overview</h1>
      <div class="logistics-actions">
        <button @click="refreshData" class="btn btn-secondary">
          🔄 Refresh
        </button>
        <button @click="showAddLogistics = true" class="btn btn-primary">
          <span class="icon">🚛</span>
          Add Logistics Connection
        </button>
      </div>
    </header>

    <!-- Filters -->
    <div v-if="logisticsFluxes.length > 0" class="filters-section">
      <div class="filters-grid">
        <div class="filter-group">
          <label for="sourceFilter">Source Factory</label>
          <select id="sourceFilter" v-model="filters.sourceFactory" class="filter-select">
            <option value="">All Sources</option>
            <option v-for="factory in uniqueFactories" :key="factory.id" :value="factory.id">
              {{ factory.name }}
            </option>
          </select>
        </div>

        <div class="filter-group">
          <label for="destinationFilter">Destination Factory</label>
          <select id="destinationFilter" v-model="filters.destinationFactory" class="filter-select">
            <option value="">All Destinations</option>
            <option v-for="factory in uniqueFactories" :key="factory.id" :value="factory.id">
              {{ factory.name }}
            </option>
          </select>
        </div>

        <div class="filter-group">
          <label for="transportFilter">Transport Type</label>
          <select id="transportFilter" v-model="filters.transportType" class="filter-select">
            <option value="">All Types</option>
            <option v-for="type in uniqueTransportTypesList" :key="type" :value="type">
              {{ getTransportIcon(type) }} {{ type }}
            </option>
          </select>
        </div>

        <div class="filter-group">
          <label for="itemFilter">Item</label>
          <select id="itemFilter" v-model="filters.item" class="filter-select">
            <option value="">All Items</option>
            <option v-for="item in uniqueItemsList" :key="item.id" :value="item.id">
              {{ item.name }}
            </option>
          </select>
        </div>

        <div class="filter-group">
          <label for="viewMode">View Mode</label>
          <select id="viewMode" v-model="viewMode" class="filter-select">
            <option value="list">List View</option>
            <option value="grouped">Grouped by Buses</option>
          </select>
        </div>

        <div class="filter-group">
          <label for="density">Display Density</label>
          <select id="density" v-model="displayDensity" class="filter-select">
            <option value="comfortable">Comfortable</option>
            <option value="compact">Compact</option>
            <option value="dense">Dense</option>
          </select>
        </div>

        <div class="filter-actions">
          <button @click="clearFilters" class="btn btn-small btn-secondary">
            Clear Filters
          </button>
        </div>
      </div>
    </div>

    <div class="logistics-content">
      <div v-if="logisticsFluxes.length === 0" class="empty-state">
        <h2>No Logistics Connections</h2>
        <p>Create logistics connections to transport items between factories.</p>
        <button @click="showAddLogistics = true" class="btn btn-primary btn-large">
          🚛 Add First Logistics Connection
        </button>
      </div>

      <div v-else class="logistics-table-container">

        <!-- List View -->
        <div v-if="viewMode === 'list'" :class="['logistics-table', `density-${displayDensity}`]">
          <div class="table-header">
            <div class="col-id">Connection ID</div>
            <div class="col-from">From Factory</div>
            <div class="col-to">To Factory</div>
            <div class="col-item">Item</div>
            <div class="col-quantity">Quantity/min</div>
            <div class="col-transport">Transport</div>
            <div class="col-details">Details</div>
            <div class="col-actions">Actions</div>
          </div>
          <div 
            v-for="flux in filteredLogisticsFluxes" 
            :key="flux.id" 
            class="table-row"
          >
            <div class="col-id">
              <span class="connection-id">{{ flux.id }}</span>
            </div>
            <div class="col-from">
              <span class="factory-name">{{ getFactoryName(flux.from_factory) }}</span>
            </div>
            <div class="col-to">
              <span class="factory-name">{{ getFactoryName(flux.to_factory) }}</span>
            </div>
            <div class="col-item">
              <span class="item-name">{{ getItemName(flux.item) }}</span>
            </div>
            <div class="col-quantity">
              <span class="quantity">{{ flux.quantity_per_min.toFixed(1) }}</span>
            </div>
            <div class="col-transport">
              <span :class="['transport-type', flux.transport_type.toLowerCase()]">
                {{ getTransportIcon(flux.transport_type) }} {{ flux.transport_type }}
              </span>
            </div>
            <div class="col-details">
              <span class="transport-details">{{ flux.transport_details || '-' }}</span>
            </div>
            <div class="col-actions">
              <div class="action-buttons">
                <button 
                  @click="handleEditLogisticsFlux(flux)"
                  class="action-btn edit-btn"
                  title="Edit logistics connection"
                >
                  ✏️
                </button>
                <button 
                  @click="handleRemoveLogisticsFlux(flux.id)"
                  class="action-btn delete-btn"
                  title="Delete logistics connection"
                >
                  🗑️
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Grouped View -->
        <div v-else-if="viewMode === 'grouped'" :class="['grouped-view', `density-${displayDensity}`]">
          <div v-for="group in groupedLogisticsList" :key="group.key" class="logistics-group">
            <div class="group-header">
              <h3 class="group-title">
                <span class="group-icon">{{ getGroupIcon(group.type) }}</span>
                {{ group.title }}
              </h3>
              <div class="group-stats">
                <span class="connection-count">{{ group.connections.length }} connections</span>
                <span class="total-throughput">{{ group.totalThroughput.toFixed(1) }} items/min</span>
              </div>
            </div>
            <div class="group-connections">
              <div 
                v-for="flux in group.connections" 
                :key="flux.id" 
                class="connection-item"
              >
                <div class="connection-route">
                  <span class="factory-name">{{ getFactoryName(flux.from_factory) }}</span>
                  <span class="route-arrow">→</span>
                  <span class="factory-name">{{ getFactoryName(flux.to_factory) }}</span>
                </div>
                <div class="connection-details">
                  <span class="item-name">{{ getItemName(flux.item) }}</span>
                  <span class="quantity">{{ flux.quantity_per_min.toFixed(1) }}/min</span>
                  <span v-if="flux.transport_details" class="transport-details">{{ flux.transport_details }}</span>
                </div>
                <div class="connection-actions">
                  <div class="connection-id-small">{{ flux.id }}</div>
                  <div class="action-buttons">
                    <button 
                      @click="handleEditLogisticsFlux(flux)"
                      class="action-btn edit-btn"
                      title="Edit logistics connection"
                    >
                      ✏️
                    </button>
                    <button 
                      @click="handleRemoveLogisticsFlux(flux.id)"
                      class="action-btn delete-btn"
                      title="Delete logistics connection"
                    >
                      🗑️
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div v-if="filteredLogisticsFluxes.length === 0 && logisticsFluxes.length > 0" class="no-results">
          <p>No logistics connections match the current filters.</p>
          <button @click="clearFilters" class="btn btn-secondary">Clear Filters</button>
        </div>
      </div>
    </div>

    <!-- Add Logistics Modal -->
    <AddLogisticsModal 
      :isOpen="showAddLogistics" 
      @close="showAddLogistics = false" 
      @logistics-added="onLogisticsAdded"
    />
    
    <!-- Edit Logistics Modal -->
    <EditLogisticsFluxModal
      :isOpen="showEditLogistics"
      :logisticsFlux="editingLogisticsFlux"
      :fromFactoryName="getFactoryName(editingLogisticsFlux?.from_factory)"
      :toFactoryName="getFactoryName(editingLogisticsFlux?.to_factory)"
      :itemName="getItemName(editingLogisticsFlux?.item)"
      @close="showEditLogistics = false"
      @logistics-flux-updated="onLogisticsUpdated"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { getLogisticsFluxes, getFactories, getItems, removeLogisticsFlux } from '../lib/tracker'
import AddLogisticsModal from '../components/AddLogisticsModal.vue'
import EditLogisticsFluxModal from '../components/EditLogisticsFluxModal.vue'

const logisticsFluxes = ref<any[]>([])
const factories = ref<any[]>([])
const items = ref<any[]>([])
const showAddLogistics = ref(false)
const showEditLogistics = ref(false)
const editingLogisticsFlux = ref<any>(null)
const viewMode = ref('list')
const displayDensity = ref('comfortable')

// Filters
const filters = ref({
  sourceFactory: '',
  destinationFactory: '',
  transportType: '',
  item: ''
})

const refreshData = async () => {
  try {
    const [fluxes, factoryList, itemList] = await Promise.all([
      getLogisticsFluxes(),
      getFactories(),
      getItems()
    ])
    
    logisticsFluxes.value = fluxes
    factories.value = factoryList
    items.value = itemList
  } catch (error) {
    console.error('Failed to refresh logistics data:', error)
  }
}

const onLogisticsAdded = () => {
  refreshData()
}

const onLogisticsUpdated = () => {
  refreshData()
}

const handleEditLogisticsFlux = (flux: any) => {
  editingLogisticsFlux.value = flux
  showEditLogistics.value = true
}

const handleRemoveLogisticsFlux = async (fluxId: string) => {
  if (confirm('Are you sure you want to remove this logistics connection?')) {
    try {
      await removeLogisticsFlux(fluxId)
      refreshData()
    } catch (error: any) {
      console.error('Failed to remove logistics flux:', error)
      alert('Failed to remove logistics connection: ' + error.message)
    }
  }
}

const clearFilters = () => {
  filters.value = {
    sourceFactory: '',
    destinationFactory: '',
    transportType: '',
    item: ''
  }
}

// Filtered logistics fluxes based on active filters
const filteredLogisticsFluxes = computed(() => {
  return logisticsFluxes.value.filter(flux => {
    if (filters.value.sourceFactory && flux.from_factory !== filters.value.sourceFactory) return false
    if (filters.value.destinationFactory && flux.to_factory !== filters.value.destinationFactory) return false
    if (filters.value.transportType && flux.transport_type !== filters.value.transportType) return false
    if (filters.value.item && flux.item !== filters.value.item) return false
    return true
  })
})

// Unique factories involved in logistics
const uniqueFactories = computed(() => {
  const factoryIds = new Set([
    ...logisticsFluxes.value.map(flux => flux.from_factory),
    ...logisticsFluxes.value.map(flux => flux.to_factory)
  ])
  return factories.value.filter(factory => factoryIds.has(factory.id))
})

// Unique transport types list
const uniqueTransportTypesList = computed(() => {
  const types = new Set(logisticsFluxes.value.map(flux => flux.transport_type))
  return Array.from(types).sort()
})

// Unique items list  
const uniqueItemsList = computed(() => {
  const itemIds = new Set(logisticsFluxes.value.map(flux => flux.item))
  return items.value.filter(item => itemIds.has(item.id))
})


// Grouped logistics for bus view
const groupedLogistics = computed(() => {
  const groups = new Map()

  filteredLogisticsFluxes.value.forEach(flux => {
    let groupKey = ''
    let groupTitle = ''
    let groupType = ''

    if (flux.transport_type === 'Conveyor') {
      // Group conveyors by route (from->to factories)
      const fromFactory = getFactoryName(flux.from_factory)
      const toFactory = getFactoryName(flux.to_factory)
      groupKey = `conveyor-${flux.from_factory}-${flux.to_factory}`
      groupTitle = `Conveyor Bus: ${fromFactory} → ${toFactory}`
      groupType = 'conveyor'
    } else {
      // Group other transport types by type + details (if any)
      const details = flux.transport_details ? ` (${flux.transport_details})` : ''
      groupKey = `${flux.transport_type.toLowerCase()}-${flux.transport_details || 'default'}`
      groupTitle = `${flux.transport_type}${details}`
      groupType = flux.transport_type.toLowerCase()
    }

    if (!groups.has(groupKey)) {
      groups.set(groupKey, {
        title: groupTitle,
        type: groupType,
        connections: [],
        totalThroughput: 0
      })
    }

    const group = groups.get(groupKey)
    group.connections.push(flux)
    group.totalThroughput += flux.quantity_per_min
  })

  // Sort groups by type and then by title
  return new Map([...groups.entries()].sort(([, a], [, b]) => {
    if (a.type !== b.type) {
      const typeOrder = { conveyor: 0, train: 1, truck: 2, drone: 3 }
      return (typeOrder[a.type] || 99) - (typeOrder[b.type] || 99)
    }
    return a.title.localeCompare(b.title)
  }))
})

// Convert grouped logistics Map to array for v-for
const groupedLogisticsList = computed(() => {
  return Array.from(groupedLogistics.value.entries()).map(([key, group]) => ({
    key,
    ...group
  }))
})

const getFactoryName = (factoryId: string) => {
  const factory = factories.value.find(f => f.id === factoryId)
  return factory ? factory.name : factoryId
}

const getItemName = (itemId: string) => {
  const item = items.value.find(i => i.id === itemId)
  return item ? item.name : itemId
}

const getTransportIcon = (transportType: string) => {
  switch (transportType) {
    case 'Conveyor': return '🔗'
    case 'Train': return '🚂'
    case 'Truck': return '🚛'
    case 'Drone': return '🚁'
    default: return '📦'
  }
}

const getGroupIcon = (groupType: string) => {
  switch (groupType) {
    case 'conveyor': return '🔗'
    case 'train': return '🚂'
    case 'truck': return '🚛'
    case 'drone': return '🚁'
    default: return '📦'
  }
}

onMounted(async () => {
  await refreshData()
})
</script>

<style scoped>
.logistics-view {
  max-width: 1400px;
  margin: 0 auto;
}

.logistics-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.logistics-header h1 {
  color: #2c3e50;
  margin: 0;
}

.logistics-actions {
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

.logistics-content {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 2rem;
}

.empty-state {
  text-align: center;
  padding: 3rem;
}

.empty-state h2 {
  color: #2c3e50;
  margin-bottom: 1rem;
}

.empty-state p {
  color: #666;
  margin-bottom: 2rem;
  font-size: 1.1rem;
}


.logistics-table-container {
  overflow-x: auto;
}

.logistics-table {
  min-width: 100%;
  border: 1px solid #e5e7eb;
  border-radius: 6px;
  overflow: hidden;
}

.table-header {
  display: grid;
  grid-template-columns: 1.5fr 1.5fr 1.5fr 1.2fr 1fr 1.2fr 1fr 0.8fr;
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
  grid-template-columns: 1.5fr 1.5fr 1.5fr 1.2fr 1fr 1.2fr 1fr 0.8fr;
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

.connection-id {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.85rem;
  color: #6b7280;
  background: #f3f4f6;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
}

.factory-name {
  font-weight: 500;
  color: #1f2937;
}

.item-name {
  font-weight: 500;
  color: #059669;
}

.quantity {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-weight: 600;
  color: #7c3aed;
}

.transport-type {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.75rem;
  border-radius: 20px;
  font-size: 0.85rem;
  font-weight: 500;
}

.transport-type.conveyor {
  background: #e0f2fe;
  color: #0277bd;
}

.transport-type.train {
  background: #f3e5f5;
  color: #7b1fa2;
}

.transport-type.truck {
  background: #fff3e0;
  color: #ef6c00;
}

.transport-type.drone {
  background: #e8f5e8;
  color: #2e7d32;
}

.transport-details {
  font-size: 0.85rem;
  color: #6b7280;
  font-style: italic;
}

.icon {
  margin-right: 0.5rem;
}

/* Filters Section */
.filters-section {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 1.5rem;
  margin-bottom: 2rem;
}

.filters-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  align-items: end;
}

.filter-group {
  display: flex;
  flex-direction: column;
}

.filter-group label {
  font-weight: 500;
  color: #374151;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
}

.filter-select {
  padding: 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 1rem;
  background: white;
  cursor: pointer;
}

.filter-select:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.filter-actions {
  display: flex;
  align-items: end;
}

.btn-small {
  padding: 0.5rem 1rem;
  font-size: 0.85rem;
}

/* Grouped View */
.grouped-view {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.logistics-group {
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  overflow: hidden;
  background: white;
}

.group-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  background: #f8f9fa;
  border-bottom: 1px solid #e5e7eb;
}

.group-title {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin: 0;
  color: #1f2937;
  font-size: 1.1rem;
}

.group-icon {
  font-size: 1.2rem;
}

.group-stats {
  display: flex;
  gap: 1rem;
  font-size: 0.9rem;
}

.connection-count {
  color: #6b7280;
  background: #f3f4f6;
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
}

.total-throughput {
  color: #7c3aed;
  font-weight: 600;
  background: #f3e8ff;
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
}

.group-connections {
  padding: 1rem;
}

.connection-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border: 1px solid #f3f4f6;
  border-radius: 6px;
  margin-bottom: 0.75rem;
  background: #fefefe;
  transition: all 0.2s;
}

.connection-item:hover {
  background: #f8f9fa;
  border-color: #e5e7eb;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.connection-item:last-child {
  margin-bottom: 0;
}

.connection-route {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex: 1;
}

.route-arrow {
  color: #6b7280;
  font-weight: bold;
}

.connection-details {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex: 1;
  justify-content: center;
}

.connection-id-small {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.75rem;
  color: #9ca3af;
  background: #f9fafb;
  padding: 0.25rem 0.5rem;
  border-radius: 3px;
  border: 1px solid #f3f4f6;
}

.connection-actions {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  align-items: flex-end;
}

.action-buttons {
  display: flex;
  gap: 0.5rem;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2rem;
  height: 2rem;
  padding: 0;
  border: none;
  border-radius: 4px;
  background: transparent;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.2s ease;
}

.action-btn:hover {
  transform: scale(1.1);
}

.edit-btn {
  background: #f0f9ff;
  border: 1px solid #bae6fd;
}

.edit-btn:hover {
  background: #0ea5e9;
  color: white;
}

.delete-btn {
  background: #fef2f2;
  border: 1px solid #fecaca;
}

.delete-btn:hover {
  background: #ef4444;
  color: white;
}

/* No Results */
.no-results {
  text-align: center;
  padding: 2rem;
  color: #6b7280;
  background: #f9fafb;
  border-radius: 8px;
  border: 2px dashed #d1d5db;
}

.no-results p {
  margin-bottom: 1rem;
  font-size: 1.1rem;
}

/* Display Density Variations */

/* Compact Density */
.logistics-table.density-compact .table-row,
.logistics-table.density-compact .table-header {
  padding: 0.5rem;
  font-size: 0.9rem;
}

.logistics-table.density-compact .connection-id {
  font-size: 0.75rem;
  padding: 0.15rem 0.4rem;
}

.logistics-table.density-compact .transport-type {
  padding: 0.15rem 0.5rem;
  font-size: 0.8rem;
}

.grouped-view.density-compact .connection-item {
  padding: 0.6rem;
  margin-bottom: 0.4rem;
}

.grouped-view.density-compact .group-header {
  padding: 0.75rem 1rem;
}

.grouped-view.density-compact .connection-details {
  gap: 0.6rem;
}

/* Dense Density */
.logistics-table.density-dense .table-row,
.logistics-table.density-dense .table-header {
  padding: 0.35rem 0.5rem;
  font-size: 0.85rem;
  line-height: 1.2;
}

.logistics-table.density-dense .connection-id {
  font-size: 0.7rem;
  padding: 0.1rem 0.35rem;
}

.logistics-table.density-dense .transport-type {
  padding: 0.1rem 0.4rem;
  font-size: 0.75rem;
}

.logistics-table.density-dense .transport-details {
  font-size: 0.75rem;
}

.logistics-table.density-dense .factory-name,
.logistics-table.density-dense .item-name {
  font-size: 0.85rem;
}

.logistics-table.density-dense .quantity {
  font-size: 0.8rem;
}

.grouped-view.density-dense .connection-item {
  padding: 0.4rem 0.75rem;
  margin-bottom: 0.25rem;
}

.grouped-view.density-dense .group-header {
  padding: 0.6rem 1rem;
}

.grouped-view.density-dense .group-title {
  font-size: 1rem;
}

.grouped-view.density-dense .connection-details {
  gap: 0.5rem;
  font-size: 0.85rem;
}

.grouped-view.density-dense .connection-route {
  gap: 0.5rem;
}

.grouped-view.density-dense .connection-id-small {
  font-size: 0.7rem;
  padding: 0.15rem 0.35rem;
}

.grouped-view.density-dense .connection-count,
.grouped-view.density-dense .total-throughput {
  padding: 0.15rem 0.5rem;
  font-size: 0.8rem;
}

/* Enhanced mobile responsiveness for dense layouts */
@media (max-width: 768px) {
  .logistics-table.density-dense .table-header,
  .logistics-table.density-dense .table-row {
    font-size: 0.8rem;
  }

  .logistics-table.density-compact .table-header,
  .logistics-table.density-compact .table-row {
    font-size: 0.85rem;
  }
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .filters-grid {
    grid-template-columns: 1fr;
  }
  
  .connection-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
  
  .connection-details {
    justify-content: flex-start;
  }
  
  .group-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.75rem;
  }
}
</style>