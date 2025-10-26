<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useFactoryStore } from '@/stores/factory'
import { useLogisticsStore } from '@/stores/logistics'
import { usePreferencesStore } from '@/stores/preferences'
import LogisticsList from '@/components/logistics/LogisticsList.vue'
import LogisticsLineForm from '@/components/logistics/LogisticsLineForm.vue'
import Modal from '@/components/ui/Modal.vue'
import Button from '@/components/ui/Button.vue'
import type { LogisticsResponse } from '@/api/types'

// Store
const factoryStore = useFactoryStore()
const logisticsStore = useLogisticsStore()
const preferencesStore = usePreferencesStore()

// State
const showCreateModal = ref(false)
const showEditModal = ref(false)
const selectedLogistics = ref<LogisticsResponse | null>(null)
const refreshTrigger = ref(0)

// Use preferences for logistics filters
const transportTypeFilter = computed({
  get: () => preferencesStore.logisticsFilters.transportType,
  set: (value) => preferencesStore.updateLogisticsFilters({ transportType: value })
})

const sourceFactoryFilter = computed({
  get: () => preferencesStore.logisticsFilters.sourceFactory,
  set: (value) => preferencesStore.updateLogisticsFilters({ sourceFactory: value })
})

const destinationFactoryFilter = computed({
  get: () => preferencesStore.logisticsFilters.destinationFactory,
  set: (value) => preferencesStore.updateLogisticsFilters({ destinationFactory: value })
})

const searchFilter = computed({
  get: () => preferencesStore.logisticsFilters.searchText,
  set: (value) => preferencesStore.updateLogisticsFilters({ searchText: value })
})

// Methods
const handleCreateLogistics = () => {
  selectedLogistics.value = null
  showCreateModal.value = true
}

const handleEditLogistics = (logistics: LogisticsResponse) => {
  selectedLogistics.value = logistics
  showEditModal.value = true
}

const handleSelectLogistics = (logistics: LogisticsResponse) => {
  selectedLogistics.value = logistics
}

const handleDeleteLogistics = (logistics: LogisticsResponse) => {
  // The list component already handles the deletion
  // Just refresh the list
  console.log(`Deleted logistics line: ${logistics.id}`)
  refreshTrigger.value++
}

const handleLogisticsCreated = (logistics: LogisticsResponse) => {
  console.log(`Created logistics line: ${logistics.id}`)
  showCreateModal.value = false
  refreshTrigger.value++
}

const handleLogisticsUpdated = (logistics: LogisticsResponse) => {
  console.log(`Updated logistics line: ${logistics.id}`)
  showEditModal.value = false
  selectedLogistics.value = null
  refreshTrigger.value++
}

const handleCloseCreateModal = () => {
  showCreateModal.value = false
}

const handleCloseEditModal = () => {
  showEditModal.value = false
  selectedLogistics.value = null
}

const resetFilters = () => {
  preferencesStore.resetLogisticsFilters()
}

// Lifecycle
onMounted(async () => {
  // Fetch initial data
  await Promise.all([
    factoryStore.fetchAll(),
    logisticsStore.fetchAll()
  ])
})
</script>

<template>
  <div class="logistics-view">
    <!-- Header -->
    <div class="logistics-header">
      <div class="header-content">
        <div class="header-title">
          <h1>Logistics Management</h1>
          <p class="header-subtitle">
            Configure and manage transport lines between factories
          </p>
        </div>

        <Button
          variant="primary"
          @click="handleCreateLogistics"
        >
          <svg
            width="16"
            height="16"
            viewBox="0 0 16 16"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            style="margin-right: 0.5rem;"
          >
            <path
              d="M8 2v6m0 0v6m0-6h6m-6 0H2"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
            />
          </svg>
          Create Logistics Line
        </Button>
      </div>
    </div>

    <!-- Filters -->
    <div class="logistics-filters">
      <div class="filter-group">
        <label class="filter-label">Transport Type</label>
        <select v-model="transportTypeFilter" class="filter-select">
          <option value="all">All Types</option>
          <option value="bus">Bus</option>
          <option value="train">Train</option>
          <option value="truck">Truck</option>
          <option value="drone">Drone</option>
        </select>
      </div>

      <div class="filter-group">
        <label class="filter-label">Source Factory</label>
        <select v-model="sourceFactoryFilter" class="filter-select">
          <option value="all">All Factories</option>
          <option v-for="factory in factoryStore.factories" :key="factory.id" :value="factory.id">
            {{ factory.name }}
          </option>
        </select>
      </div>

      <div class="filter-group">
        <label class="filter-label">Destination Factory</label>
        <select v-model="destinationFactoryFilter" class="filter-select">
          <option value="all">All Factories</option>
          <option v-for="factory in factoryStore.factories" :key="factory.id" :value="factory.id">
            {{ factory.name }}
          </option>
        </select>
      </div>

      <div class="filter-group">
        <label class="filter-label">Search</label>
        <input
          v-model="searchFilter"
          type="text"
          placeholder="Search logistics..."
          class="filter-input"
        />
      </div>

      <div class="filter-actions">
        <Button variant="secondary" @click="resetFilters">
          Reset Filters
        </Button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="logistics-content">
      <LogisticsList
        :refresh-trigger="refreshTrigger"
        :transport-type-filter="transportTypeFilter"
        :source-factory-filter="sourceFactoryFilter"
        :destination-factory-filter="destinationFactoryFilter"
        :search-filter="searchFilter"
        @select-logistics="handleSelectLogistics"
        @edit-logistics="handleEditLogistics"
        @delete-logistics="handleDeleteLogistics"
        @create-logistics="handleCreateLogistics"
      />
    </div>

    <!-- Create Modal -->
    <Modal
      v-model:show="showCreateModal"
      title="Create Logistics Line"
      @close="handleCloseCreateModal"
    >
      <LogisticsLineForm
        @close="handleCloseCreateModal"
        @created="handleLogisticsCreated"
      />
    </Modal>

    <!-- Edit Modal -->
    <Modal
      v-model:show="showEditModal"
      title="Edit Logistics Line"
      @close="handleCloseEditModal"
    >
      <LogisticsLineForm
        :logistics-line="selectedLogistics"
        @close="handleCloseEditModal"
        @updated="handleLogisticsUpdated"
      />
    </Modal>
  </div>
</template>

<style scoped lang="scss">
.logistics-view {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg, 1rem);
  padding: var(--spacing-lg, 1rem);
  max-width: 1200px;
  margin: 0 auto;
  width: 100%;
}

.logistics-header {
  background-color: var(--color-surface, #ffffff);
  border-radius: var(--border-radius-lg, 0.5rem);
  box-shadow: var(--shadow-sm, 0 1px 2px 0 rgba(0, 0, 0, 0.05));
  padding: var(--spacing-lg, 1rem);
  margin-bottom: var(--spacing-md, 0.75rem);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--spacing-md, 0.75rem);
}

.header-title {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
}

.header-title h1 {
  font-size: var(--font-size-2xl, 1.5rem);
  font-weight: var(--font-weight-bold, 700);
  color: var(--color-text-primary, #111827);
  margin: 0;
}

.header-subtitle {
  font-size: var(--font-size-base, 1rem);
  color: var(--color-text-secondary, #6b7280);
  margin: 0;
}

.logistics-filters {
  background-color: var(--color-surface, #ffffff);
  border-radius: var(--border-radius-lg, 0.5rem);
  box-shadow: var(--shadow-sm, 0 1px 2px 0 rgba(0, 0, 0, 0.05));
  padding: var(--spacing-lg, 1rem);
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-md, 0.75rem);
  align-items: end;
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
  min-width: 150px;
}

.filter-label {
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-secondary, #6b7280);
}

.filter-select,
.filter-input {
  padding: var(--spacing-sm, 0.5rem);
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: var(--border-radius-md, 0.375rem);
  background-color: var(--color-surface-secondary, #f9fafb);
  color: var(--color-text-primary, #111827);
  font-size: var(--font-size-sm, 0.875rem);

  &:focus {
    outline: none;
    border-color: var(--color-primary-500, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }
}

.filter-input {
  min-width: 200px;
}

.filter-actions {
  display: flex;
  align-items: center;
  margin-left: auto;
}

.logistics-content {
  flex: 1;
}

// Responsive design
@media (max-width: 768px) {
  .logistics-view {
    padding: var(--spacing-md, 0.75rem);
  }

  .header-content {
    flex-direction: column;
    align-items: stretch;
    gap: var(--spacing-md, 0.75rem);
  }

  .header-title h1 {
    font-size: var(--font-size-xl, 1.25rem);
  }

  .logistics-filters {
    flex-direction: column;
    align-items: stretch;
  }

  .filter-group {
    min-width: auto;
  }

  .filter-input {
    min-width: auto;
  }

  .filter-actions {
    margin-left: 0;
    margin-top: var(--spacing-sm, 0.5rem);
  }
}

// Dark theme adjustments
:root.dark {
  .logistics-header,
  .logistics-filters {
    background-color: var(--color-surface, #1e293b);
  }

  .filter-select,
  .filter-input {
    background-color: var(--color-surface-secondary, #334155);
    color: var(--color-text-primary, #f1f5f9);
    border-color: var(--color-border, #475569);

    &:focus {
      border-color: var(--color-primary-500, #3b82f6);
    }
  }
}
</style>
