<template>
  <div class="factory-view">
    <!-- Factory Selector -->
    <FactorySelector />

    <!-- Factory Content (only show if a factory is selected) -->
    <div v-if="currentFactory" class="factory-content">
      <!-- Factory Overview -->
      <div class="factory-overview">
        <div class="overview-header">
          <h2 class="factory-name">{{ currentFactory.name }}</h2>
          <div class="factory-stats">
            <div class="stat-item">
              <span class="stat-label">Power:</span>
              <span class="stat-value" :class="powerBalanceClass">
                {{ formatPower(currentFactory.power_balance) }}
              </span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Items:</span>
              <span class="stat-value">{{ currentFactory.items.length }}</span>
            </div>
          </div>
        </div>

        <p v-if="currentFactory.description" class="factory-description">
          {{ currentFactory.description }}
        </p>
      </div>

      <!-- Factory Tabs -->
      <Tabs
        v-model:active-tab="activeTab"
        :tabs="tabs"
        class="factory-tabs"
      >
        <!-- Production Lines Tab -->
        <TabPanel tab-id="production">
          <ProductionLineList :factory-id="currentFactory.id" />
        </TabPanel>

        <!-- Raw Inputs Tab -->
        <TabPanel tab-id="raw-inputs">
          <RawInputList :factory-id="currentFactory.id" />
        </TabPanel>

        <!-- Power Generation Tab -->
        <TabPanel tab-id="power-generation">
          <PowerGeneratorList :factory-id="currentFactory.id" />
        </TabPanel>
      </Tabs>
    </div>

    <!-- Empty State (no factory selected) -->
    <div v-else class="empty-state">
      <div class="empty-icon">
        <svg
          width="64"
          height="64"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M19 3H5C3.89543 3 3 3.89543 3 5V19C3 20.1046 3.89543 21 5 21H19C20.1046 21 21 20.1046 21 19V5C21 3.89543 20.1046 3 19 3Z"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <path
            d="M8 12H16M12 8V16"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </div>
      <h3 class="empty-title">No Factory Selected</h3>
      <p class="empty-description">
        Select a factory from the dropdown above or create a new factory to get started.
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, watch } from 'vue'
import { useFactoryStore } from '@/stores/factory'
import { usePreferencesStore } from '@/stores/preferences'
import FactorySelector from '@/components/factory/FactorySelector.vue'
import ProductionLineList from '@/components/factory/ProductionLineList.vue'
import RawInputList from '@/components/factory/RawInputList.vue'
import PowerGeneratorList from '@/components/factory/PowerGeneratorList.vue'
import Tabs from '@/components/ui/Tabs.vue'
import TabPanel from '@/components/ui/TabPanel.vue'

const factoryStore = useFactoryStore()
const preferencesStore = usePreferencesStore()

// State
const activeTab = computed({
  get: () => preferencesStore.factoryViewTab,
  set: (value) => preferencesStore.setFactoryViewTab(value)
})

// Tab configuration
const tabs = [
  { id: 'production', label: 'Production Lines' },
  { id: 'raw-inputs', label: 'Raw Inputs' },
  { id: 'power-generation', label: 'Power Generation' }
]

// Computed
const currentFactory = computed(() => factoryStore.currentFactory)

const powerBalanceClass = computed(() => {
  if (!currentFactory.value) return ''

  const balance = currentFactory.value.power_balance
  if (balance > 0) return 'power-surplus'
  if (balance < 0) return 'power-deficit'
  return 'power-balanced'
})

// Methods
const formatPower = (power: number): string => {
  if (power < 0) {
    return `-${formatPower(Math.abs(power))}`
  }
  if (power < 1) {
    return `${(power * 1000).toFixed(0)} kW`
  }
  return `${power.toFixed(1)} MW`
}

// Watch for factory changes to refresh data
watch(() => currentFactory.value?.id, (factoryId) => {
  if (factoryId) {
    factoryStore.fetchById(factoryId)
    // Save selected factory to preferences
    preferencesStore.setSelectedFactoryId(factoryId)
  }
}, { immediate: true })

// Initialize selected factory from preferences
if (preferencesStore.selectedFactoryId && !currentFactory.value) {
  factoryStore.fetchById(preferencesStore.selectedFactoryId)
}

</script>

<style scoped lang="scss">
.factory-view {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg, 1rem);
}

.factory-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg, 1rem);
}

.factory-overview {
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-lg, 0.5rem);
  box-shadow: var(--shadow-sm, 0 1px 2px 0 rgba(0, 0, 0, 0.05));
  padding: var(--spacing-lg, 1rem);
}

.overview-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-md, 0.75rem);
  flex-wrap: wrap;
  gap: var(--spacing-md, 0.75rem);
}

.factory-name {
  font-size: var(--font-size-xl, 1.25rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-gray-900, #111827);
  margin: 0;
}

.factory-stats {
  display: flex;
  gap: var(--spacing-lg, 1rem);
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
}

.stat-label {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-500, #6b7280);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.stat-value {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);

  &.power-surplus {
    color: var(--color-green-600, #059669);
  }

  &.power-deficit {
    color: var(--color-red-600, #dc2626);
  }

  &.power-balanced {
    color: var(--color-blue-600, #2563eb);
  }
}

.factory-description {
  color: var(--color-gray-600, #4b5563);
  margin: 0;
  line-height: 1.5;
}

.factory-tabs {
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-lg, 0.5rem);
  box-shadow: var(--shadow-sm, 0 1px 2px 0 rgba(0, 0, 0, 0.05));
  overflow: hidden;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl, 1.25rem);
  text-align: center;
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-lg, 0.5rem);
  box-shadow: var(--shadow-sm, 0 1px 2px 0 rgba(0, 0, 0, 0.05));
  min-height: 300px;
}

.empty-icon {
  color: var(--color-gray-400, #9ca3af);
  margin-bottom: var(--spacing-md, 0.75rem);
}

.empty-title {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-gray-900, #111827);
  margin: 0 0 var(--spacing-sm, 0.5rem) 0;
}

.empty-description {
  color: var(--color-gray-600, #4b5563);
  margin: 0;
  max-width: 500px;
}

// Responsive design
@media (max-width: 768px) {
  .overview-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .factory-stats {
    width: 100%;
    justify-content: space-around;
  }

  .stat-item {
    min-width: 80px;
  }
}
</style>
