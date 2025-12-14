<template>
  <div class="dashboard-view" data-test="dashboard-view">
    <!-- Header -->
    <div class="dashboard-header" data-test="dashboard-header">
      <div class="header-content">
        <h1 class="dashboard-title">Dashboard</h1>
        <p class="dashboard-subtitle">Global overview of all production</p>
      </div>
      <div class="header-actions">
        <SaveLoadControls
          @saved="handleSaved"
          @loaded="handleLoaded"
          @reset="handleReset"
        />
        <div class="refresh-info">
          <span v-if="lastUpdated" class="last-updated">
            Last updated: {{ formatTime(lastUpdated) }}
          </span>
          <span v-if="autoRefreshEnabled" class="auto-refresh">
            Auto-refresh: {{ refreshInterval }}s
          </span>
        </div>
      </div>
    </div>

    <!-- Error Banner -->
    <Alert
      v-if="error"
      type="error"
      dismissible
      class="error-banner"
      @dismiss="handleDismissError"
    >
      <div class="error-content">
        <p class="error-message">{{ error }}</p>
        <Button variant="secondary" size="sm" @click="handleRetry">
          Retry
        </Button>
      </div>
    </Alert>

    <!-- Loading State -->
    <div v-if="loading && !hasData" class="loading-container">
      <LoadingSpinner size="lg" />
      <p class="loading-text">Loading dashboard data...</p>
    </div>

    <!-- Dashboard Content -->
    <div v-else-if="hasData" class="dashboard-content">
      <!-- Power Statistics Section -->
      <section class="dashboard-section">
        <PowerStatsChart
          :power-stats="powerStats"
          :loading="loading"
        />
      </section>

      <!-- Item Balance Table Section -->
      <section class="dashboard-section">
        <div class="section-header">
          <h2 class="section-title">Item Balance</h2>
          <div class="section-controls">
            <div class="filter-controls">
              <select
                v-model="balanceStateFilter"
                class="filter-select"
                @change="handleFilterChange"
              >
                <option value="">All States</option>
                <option value="overflow">Overflow</option>
                <option value="underflow">Underflow</option>
                <option value="balanced">Balanced</option>
              </select>
              <input
                v-model="searchFilter"
                type="text"
                placeholder="Search items..."
                class="filter-input"
                @input="handleFilterChange"
              />
            </div>
          </div>
        </div>

        <DataTable
          :columns="itemBalanceColumns"
          :data="filteredItemBalances"
          :loading="loading"
          :paginated="true"
          :page-size="20"
          :filterable="false"
          empty-text="No items found matching the current filters"
          @sort-change="handleSortChange"
        >
          <template #cell-item="{ row }">
            <div class="item-name">
              <ItemDisplay :item="row.item" :show-name="false" size="sm" />
              <span class="item-text">{{ formatItemName(String(row.item)) }}</span>
            </div>
          </template>

          <template #cell-balance="{ row }">
            <span
              class="balance-value"
              :class="{
                'overflow': row.state === 'overflow',
                'underflow': row.state === 'underflow',
                'balanced': row.state === 'balanced'
              }"
            >
              {{ formatBalance(Number(row.balance)) }}
            </span>
          </template>

          <template #cell-state="{ row }">
            <span
              class="state-badge"
              :class="row.state"
            >
              {{ formatState(String(row.state)) }}
            </span>
          </template>
        </DataTable>
      </section>
    </div>

    <!-- Empty State -->
    <div v-else class="empty-state">
      <div class="empty-icon">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M3 9L12 2L21 9V20C21 20.5304 20.7893 21.0391 20.4142 21.4142C20.0391 21.7893 19.5304 22 19 22H5C4.46957 22 3.96086 21.7893 3.58579 21.4142C3.21071 21.0391 3 20.5304 3 20V9Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M9 22V12H15V22" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </div>
      <h2 class="empty-title">No Data Available</h2>
      <p class="empty-description">
        Start by creating factories and production lines to see dashboard data.
      </p>
      <Button variant="primary" @click="handleRefresh">
        Refresh Dashboard
      </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { useDashboardStore, useFactoryStore, useLogisticsStore, useGameDataStore } from '@/stores'
import { usePreferencesStore } from '@/stores/preferences'
import { Button, Alert, LoadingSpinner, DataTable } from '@/components/ui'
import { PowerStatsChart, SaveLoadControls } from '@/components/dashboard'
import ItemDisplay from '@/components/ui/ItemDisplay.vue'
import type { Column } from '@/components/ui/DataTable.vue'

// Stores
const dashboardStore = useDashboardStore()
const factoryStore = useFactoryStore()
const logisticsStore = useLogisticsStore()
const gameDataStore = useGameDataStore()
const preferencesStore = usePreferencesStore()

// State
const lastUpdated = ref<Date | null>(null)
let refreshTimer: number | null = null

// Use preferences for filters
const balanceStateFilter = computed({
  get: () => preferencesStore.dashboardFilters.state === 'all' ? '' : preferencesStore.dashboardFilters.state,
  set: (value) => preferencesStore.updateDashboardFilters({ state: (value || 'all') as 'all' | 'overflow' | 'underflow' | 'balanced' })
})

const searchFilter = computed({
  get: () => preferencesStore.dashboardFilters.searchText,
  set: (value) => preferencesStore.updateDashboardFilters({ searchText: value })
})

const autoRefreshEnabled = computed({
  get: () => preferencesStore.uiPreferences.autoRefresh,
  set: (value) => preferencesStore.setAutoRefresh(value)
})

const refreshInterval = computed({
  get: () => preferencesStore.uiPreferences.refreshInterval,
  set: (value) => preferencesStore.setRefreshInterval(value)
})

// Computed properties
const loading = computed(() => dashboardStore.loading)
const error = computed(() => dashboardStore.error)
const summary = computed(() => dashboardStore.summary)
const powerStats = computed(() => dashboardStore.powerStats)
const itemBalances = computed(() => dashboardStore.itemBalances)

const hasData = computed(() => {
  return summary.value || powerStats.value || itemBalances.value.length > 0
})

const filteredItemBalances = computed(() => {
  return dashboardStore.filteredItemBalances({
    state: balanceStateFilter.value as 'overflow' | 'underflow' | 'balanced' | undefined,
    searchText: searchFilter.value || undefined
  })
})

// Table columns
const itemBalanceColumns: Column[] = [
  {
    key: 'item',
    label: 'Item',
    sortable: true,
    width: '40%'
  },
  {
    key: 'balance',
    label: 'Balance',
    sortable: true,
    width: '25%',
    formatter: (value: unknown) => formatBalance(Number(value))
  },
  {
    key: 'state',
    label: 'State',
    sortable: true,
    width: '20%'
  }
]

// Methods
const fetchData = async () => {
  try {
    await dashboardStore.fetchAllData()
    lastUpdated.value = new Date()
  } catch (err) {
    console.error('Failed to fetch dashboard data:', err)
  }
}

const handleRefresh = async () => {
  await fetchData()
}

const handleRetry = async () => {
  dashboardStore.clearError()
  await fetchData()
}

const handleDismissError = () => {
  dashboardStore.clearError()
}

const handleFilterChange = () => {
  // Filters are automatically saved through computed setters
}

const handleSortChange = ({ key, direction }: { key: string; direction: 'asc' | 'desc' }) => {
  // Sorting is handled by the DataTable component
  console.log('Sort changed:', { key, direction })
}

const handleSaved = () => {
  // Optionally refresh dashboard after save
  console.log('Engine state saved successfully')
}

const handleLoaded = async () => {
  // Reset all stores to clear cached data (since we just loaded new state)
  console.log('Engine state loaded successfully, clearing stores and refreshing dashboard...')

  // Reset all stores (except preferences - we want to keep user settings)
  dashboardStore.reset()
  factoryStore.reset()
  logisticsStore.reset()
  // Note: gameDataStore contains static game data, so we don't reset it

  // Clear factory/logistics references in preferences (they no longer exist)
  preferencesStore.setSelectedFactoryId(null)
  preferencesStore.resetLogisticsFilters() // Clears sourceFactory/destinationFactory filters

  // Refresh dashboard data
  await fetchData()
}

const handleReset = async () => {
  // Reset all stores to clear cached data
  console.log('Engine state reset successfully, clearing stores and refreshing dashboard...')

  // Reset all stores (except preferences - we want to keep user settings)
  dashboardStore.reset()
  factoryStore.reset()
  logisticsStore.reset()
  // Note: gameDataStore contains static game data, so we don't reset it

  // Clear factory/logistics references in preferences (they no longer exist)
  preferencesStore.setSelectedFactoryId(null)
  preferencesStore.resetLogisticsFilters() // Clears sourceFactory/destinationFactory filters

  // Refresh dashboard data
  await fetchData()
}

const setupAutoRefresh = () => {
  if (autoRefreshEnabled.value && refreshInterval.value > 0) {
    refreshTimer = window.setInterval(() => {
      fetchData()
    }, refreshInterval.value * 1000)
  }
}

const clearAutoRefresh = () => {
  if (refreshTimer) {
    clearInterval(refreshTimer)
    refreshTimer = null
  }
}

// Utility functions
const formatTime = (date: Date): string => {
  return date.toLocaleTimeString()
}

const formatBalance = (value: number): string => {
  if (Math.abs(value) >= 1000) {
    return `${(value / 1000).toFixed(1)}k`
  }
  return value.toFixed(1)
}

const formatState = (state: string): string => {
  return state.charAt(0).toUpperCase() + state.slice(1)
}

const formatItemName = (item: string): string => {
  return item.replace(/([A-Z])/g, ' $1').trim()
}

// Lifecycle
onMounted(() => {
  fetchData()
  setupAutoRefresh()
})

onUnmounted(() => {
  clearAutoRefresh()
})

// Watch for auto-refresh settings changes
watch([autoRefreshEnabled, refreshInterval], () => {
  clearAutoRefresh()
  setupAutoRefresh()
}, { deep: true })
</script>

<style scoped lang="scss">
.dashboard-view {
  width: 100%;
  padding: var(--spacing-lg, 1rem);
  max-width: 100%;
}

.dashboard-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-xl, 1.25rem);
  flex-wrap: wrap;
  gap: var(--spacing-md, 0.75rem);
}

.header-content {
  flex: 1;
  min-width: 0;
}

.dashboard-title {
  font-size: var(--font-size-2xl, 1.5rem);
  font-weight: var(--font-weight-bold, 700);
  color: var(--color-ficsit-orange, #f58b00);
  margin: 0 0 var(--spacing-xs, 0.25rem) 0;
  letter-spacing: -0.01em;
}

.dashboard-subtitle {
  font-size: var(--font-size-base, 1rem);
  color: var(--color-text-secondary, #b8b8b8);
  margin: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-md, 0.75rem);
  flex-shrink: 0;
}

.refresh-info {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: var(--spacing-xs, 0.25rem);
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-text-muted, #8a8a8a);
  font-family: var(--font-family-mono);
}

.error-banner {
  margin-bottom: var(--spacing-lg, 1rem);
}

.error-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-md, 0.75rem);
}

.error-message {
  margin: 0;
  flex: 1;
}

.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-2xl, 2rem);
  gap: var(--spacing-lg, 1rem);
  color: var(--color-text-secondary, #b8b8b8);
}

.loading-text {
  font-size: var(--font-size-lg, 1.125rem);
  margin: 0;
}

.dashboard-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xl, 1.25rem);
}

.dashboard-section {
  width: 100%;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-lg, 1rem);
  flex-wrap: wrap;
  gap: var(--spacing-md, 0.75rem);
}

.section-title {
  font-size: var(--font-size-xl, 1.25rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-text-primary, #e5e5e5);
  margin: 0;
}

.section-controls {
  display: flex;
  align-items: center;
  gap: var(--spacing-md, 0.75rem);
}

.filter-controls {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 0.5rem);
}

.filter-select,
.filter-input {
  padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-md, 0.375rem);
  font-size: var(--font-size-sm, 0.875rem);
  background-color: var(--color-white, #ffffff);

  &:focus {
    outline: none;
    border-color: var(--color-primary-500, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }
}

.filter-input {
  width: 200px;
}

.item-name {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
}


.item-text {
  font-weight: var(--font-weight-medium, 500);
}

.balance-value {
  font-weight: var(--font-weight-semibold, 600);
  font-family: var(--font-family-mono);

  &.overflow {
    color: var(--color-success, #22c55e);
  }

  &.underflow {
    color: var(--color-error, #ef4444);
  }

  &.balanced {
    color: var(--color-warning, #f59e0b);
  }
}

.state-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: var(--border-radius-sm, 3px);
  font-size: var(--font-size-xs, 0.75rem);
  font-weight: var(--font-weight-medium, 500);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  border: 1px solid;

  &.overflow {
    background-color: rgba(34, 197, 94, 0.1);
    color: var(--color-success, #22c55e);
    border-color: var(--color-success, #22c55e);
  }

  &.underflow {
    background-color: rgba(239, 68, 68, 0.1);
    color: var(--color-error, #ef4444);
    border-color: var(--color-error, #ef4444);
  }

  &.balanced {
    background-color: rgba(245, 158, 11, 0.1);
    color: var(--color-warning, #f59e0b);
    border-color: var(--color-warning, #f59e0b);
  }
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-3xl, 3rem) var(--spacing-lg, 1rem);
  text-align: center;
  color: var(--color-text-muted, #8a8a8a);
}

.empty-icon {
  opacity: 0.4;
  margin-bottom: var(--spacing-lg, 1rem);
  color: var(--color-text-muted, #8a8a8a);
}

.empty-title {
  font-size: var(--font-size-xl, 1.25rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-text-primary, #e5e5e5);
  margin: 0 0 var(--spacing-sm, 0.5rem) 0;
}

.empty-description {
  font-size: var(--font-size-base, 1rem);
  margin: 0 0 var(--spacing-lg, 1rem) 0;
  max-width: 400px;
  color: var(--color-text-secondary, #b8b8b8);
}

// Responsive design
@media (max-width: 768px) {
  .dashboard-view {
    padding: var(--spacing-md, 0.75rem);
  }

  .dashboard-header {
    flex-direction: column;
    align-items: stretch;
  }

  .header-actions {
    justify-content: space-between;
  }

  .section-header {
    flex-direction: column;
    align-items: stretch;
  }

  .filter-controls {
    flex-direction: column;
    width: 100%;
  }

  .filter-input {
    width: 100%;
  }

  .error-content {
    flex-direction: column;
    align-items: stretch;
    gap: var(--spacing-sm, 0.5rem);
  }
}

@media (max-width: 480px) {
  .dashboard-title {
    font-size: var(--font-size-xl, 1.25rem);
  }

  .section-title {
    font-size: var(--font-size-lg, 1.125rem);
  }
}
</style>
