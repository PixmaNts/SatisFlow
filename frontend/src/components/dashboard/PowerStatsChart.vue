<template>
  <div class="power-stats-chart">
    <div class="chart-header">
      <h2 class="chart-title">Power Statistics</h2>
      <div class="power-status" :class="powerStatusClass">
        <span class="status-indicator"></span>
        <span class="status-text">{{ powerStatusText }}</span>
      </div>
    </div>

    <!-- Power Bar Chart -->
    <div class="power-chart">
      <div class="power-bar-container">
        <div class="power-bar">
          <div
            class="power-segment generation"
            :style="{ width: generationPercentage + '%' }"
            :title="`Generation: ${formatPower(totalGeneration)}`"
          ></div>
          <div
            v-if="hasDeficit"
            class="power-segment deficit"
            :style="{ width: deficitPercentage + '%' }"
            :title="`Deficit: ${formatPower(Math.abs(powerBalance))}`"
          ></div>
        </div>
        <div class="power-labels">
          <span class="label label-zero">0</span>
          <span class="label label-max">{{ formatPower(maxPower) }}</span>
        </div>
      </div>
    </div>

    <!-- Power Legend -->
    <div class="power-legend">
      <div class="legend-item">
        <div class="legend-color generation"></div>
        <span class="legend-text">Generation: {{ formatPower(totalGeneration) }}</span>
      </div>
      <div class="legend-item">
        <div class="legend-color consumption"></div>
        <span class="legend-text">Consumption: {{ formatPower(totalConsumption) }}</span>
      </div>
      <div v-if="hasSurplus" class="legend-item">
        <div class="legend-color surplus"></div>
        <span class="legend-text">Surplus: {{ formatPower(powerBalance) }}</span>
      </div>
      <div v-if="hasDeficit" class="legend-item">
        <div class="legend-color deficit"></div>
        <span class="legend-text">Deficit: {{ formatPower(Math.abs(powerBalance)) }}</span>
      </div>
    </div>

    <!-- Factory Power Breakdown -->
    <div v-if="factoryStats.length > 0" class="factory-breakdown">
      <h3 class="breakdown-title">Factory Power Breakdown</h3>
      <div class="factory-list">
        <div
          v-for="factory in factoryStats"
          :key="factory.factory_id"
          class="factory-item"
        >
          <div class="factory-info">
            <h4 class="factory-name">{{ factory.factory_name }}</h4>
            <div class="factory-details">
              <span class="generator-count">{{ factory.generator_count }} generators</span>
              <span class="generator-types">{{ factory.generator_types.join(', ') }}</span>
            </div>
          </div>
          <div class="factory-power">
            <div class="power-bars">
              <div class="power-bar-small">
                <div
                  class="power-segment-small generation"
                  :style="{ width: getFactoryGenerationPercentage(factory) + '%' }"
                ></div>
              </div>
            </div>
            <div class="power-numbers">
              <span class="generation-text">{{ formatPower(factory.generation) }}</span>
              <span class="consumption-text">{{ formatPower(factory.consumption) }}</span>
              <span
                class="balance-text"
                :class="{
                  'surplus': factory.balance > 0,
                  'deficit': factory.balance < 0,
                  'balanced': factory.balance === 0
                }"
              >
                {{ formatPower(factory.balance) }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="loading-state">
      <LoadingSpinner size="lg" />
      <p class="loading-text">Loading power statistics...</p>
    </div>

    <!-- Empty State -->
    <div v-else-if="!hasData" class="empty-state">
      <div class="empty-icon">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M13 2L3 14H12L11 22L21 10H12L13 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </div>
      <p class="empty-text">No power data available</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import LoadingSpinner from '@/components/ui/LoadingSpinner.vue'
import type { PowerStats } from '@/api/types'

interface Props {
  powerStats?: PowerStats | null
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
})

// Computed properties
const totalGeneration = computed(() => props.powerStats?.total_generation || 0)
const totalConsumption = computed(() => props.powerStats?.total_consumption || 0)
const powerBalance = computed(() => props.powerStats?.power_balance || 0)
const factoryStats = computed(() => props.powerStats?.factory_stats || [])

const hasSurplus = computed(() => props.powerStats?.has_surplus || false)
const hasDeficit = computed(() => props.powerStats?.has_deficit || false)
const isBalanced = computed(() => props.powerStats?.is_balanced || false)

const hasData = computed(() => {
  return totalGeneration.value > 0 || totalConsumption.value > 0 || factoryStats.value.length > 0
})

const maxPower = computed(() => {
  return Math.max(totalGeneration.value, totalConsumption.value, 1)
})

const generationPercentage = computed(() => {
  return (totalGeneration.value / maxPower.value) * 100
})

const deficitPercentage = computed(() => {
  if (!hasDeficit.value) return 0
  return (Math.abs(powerBalance.value) / maxPower.value) * 100
})

const powerStatusClass = computed(() => ({
  'surplus': hasSurplus.value,
  'deficit': hasDeficit.value,
  'balanced': isBalanced.value,
}))

const powerStatusText = computed(() => {
  if (hasSurplus.value) return 'Power Surplus'
  if (hasDeficit.value) return 'Power Deficit'
  return 'Power Balanced'
})

// Helper functions
const formatPower = (value: number): string => {
  if (Math.abs(value) >= 1000) {
    return `${(value / 1000).toFixed(1)} GW`
  }
  return `${value.toFixed(0)} MW`
}

const getFactoryGenerationPercentage = (factory: { generation: number; consumption: number }): number => {
  const maxFactoryPower = Math.max(factory.generation, factory.consumption, 1)
  return (factory.generation / maxFactoryPower) * 100
}
</script>

<style scoped lang="scss">
.power-stats-chart {
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-lg, 0.5rem);
  box-shadow: var(--shadow-sm, 0 1px 2px 0 rgba(0, 0, 0, 0.05));
  padding: var(--spacing-lg, 1rem);
  margin-bottom: var(--spacing-xl, 1.25rem);
}

.chart-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-lg, 1rem);
}

.chart-title {
  font-size: var(--font-size-xl, 1.25rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-gray-900, #111827);
  margin: 0;
}

.power-status {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
  padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
  border-radius: var(--border-radius-full, 9999px);
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);

  &.surplus {
    background-color: var(--color-success-100, #d1fae5);
    color: var(--color-success-800, #065f46);
  }

  &.deficit {
    background-color: var(--color-error-100, #fee2e2);
    color: var(--color-error-800, #991b1b);
  }

  &.balanced {
    background-color: var(--color-warning-100, #fef3c7);
    color: var(--color-warning-800, #92400e);
  }
}

.status-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: currentColor;
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.power-chart {
  margin-bottom: var(--spacing-lg, 1rem);
}

.power-bar-container {
  width: 100%;
}

.power-bar {
  width: 100%;
  height: 32px;
  background-color: var(--color-gray-200, #e5e7eb);
  border-radius: var(--border-radius-md, 0.375rem);
  overflow: hidden;
  display: flex;
  margin-bottom: var(--spacing-sm, 0.5rem);
}

.power-segment {
  height: 100%;
  transition: width 0.3s ease-in-out;

  &.generation {
    background-color: var(--color-success-500, #10b981);
  }

  &.deficit {
    background-color: var(--color-error-500, #ef4444);
  }
}

.power-labels {
  display: flex;
  justify-content: space-between;
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-600, #4b5563);
}

.power-legend {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-md, 0.75rem);
  margin-bottom: var(--spacing-lg, 1rem);
  padding-bottom: var(--spacing-lg, 1rem);
  border-bottom: 1px solid var(--color-gray-200, #e5e7eb);
}

.legend-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-gray-700, #374151);
}

.legend-color {
  width: 12px;
  height: 12px;
  border-radius: var(--border-radius-sm, 0.25rem);

  &.generation {
    background-color: var(--color-success-500, #10b981);
  }

  &.consumption {
    background-color: var(--color-orange-500, #f97316);
  }

  &.surplus {
    background-color: var(--color-blue-500, #3b82f6);
  }

  &.deficit {
    background-color: var(--color-error-500, #ef4444);
  }
}

.factory-breakdown {
  margin-top: var(--spacing-lg, 1rem);
}

.breakdown-title {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-gray-900, #111827);
  margin: 0 0 var(--spacing-md, 0.75rem) 0;
}

.factory-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md, 0.75rem);
}

.factory-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md, 0.75rem);
  background-color: var(--color-gray-50, #f9fafb);
  border-radius: var(--border-radius-md, 0.375rem);
  border: 1px solid var(--color-gray-200, #e5e7eb);
}

.factory-info {
  flex: 1;
  min-width: 0;
}

.factory-name {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-900, #111827);
  margin: 0 0 var(--spacing-xs, 0.25rem) 0;
}

.factory-details {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-600, #4b5563);
}

.factory-power {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: var(--spacing-xs, 0.25rem);
  min-width: 120px;
}

.power-bars {
  width: 100%;
  max-width: 100px;
}

.power-bar-small {
  width: 100%;
  height: 8px;
  background-color: var(--color-gray-200, #e5e7eb);
  border-radius: var(--border-radius-sm, 0.25rem);
  overflow: hidden;
}

.power-segment-small {
  height: 100%;
  transition: width 0.3s ease-in-out;

  &.generation {
    background-color: var(--color-success-500, #10b981);
  }
}

.power-numbers {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: var(--spacing-xs, 0.25rem);
  font-size: var(--font-size-xs, 0.75rem);
  font-weight: var(--font-weight-medium, 500);
}

.generation-text {
  color: var(--color-success-700, #047857);
}

.consumption-text {
  color: var(--color-orange-700, #c2410c);
}

.balance-text {
  &.surplus {
    color: var(--color-blue-700, #1d4ed8);
  }

  &.deficit {
    color: var(--color-error-700, #b91c1c);
  }

  &.balanced {
    color: var(--color-warning-700, #a16207);
  }
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl, 1.25rem);
  gap: var(--spacing-md, 0.75rem);
  color: var(--color-gray-600, #4b5563);
}

.loading-text {
  font-size: var(--font-size-sm, 0.875rem);
  margin: 0;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl, 1.25rem);
  gap: var(--spacing-md, 0.75rem);
  color: var(--color-gray-500, #6b7280);
}

.empty-icon {
  opacity: 0.5;
}

.empty-text {
  font-size: var(--font-size-base, 1rem);
  margin: 0;
}

// Responsive design
@media (max-width: 768px) {
  .chart-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-sm, 0.5rem);
  }

  .power-legend {
    flex-direction: column;
  }

  .factory-item {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-sm, 0.5rem);
  }

  .factory-power {
    align-items: flex-start;
    width: 100%;
  }

  .power-numbers {
    flex-direction: row;
    justify-content: space-between;
    width: 100%;
  }
}

@media (max-width: 480px) {
  .power-stats-chart {
    padding: var(--spacing-md, 0.75rem);
  }

  .chart-title {
    font-size: var(--font-size-lg, 1.125rem);
  }

  .power-bar {
    height: 24px;
  }
}
</style>
