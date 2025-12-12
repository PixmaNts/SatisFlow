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
            v-if="totalConsumption > 0"
            class="power-segment consumption"
            :style="{ width: consumptionPercentage + '%' }"
            :title="`Consumption: ${formatPower(totalConsumption)}`"
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
      <div v-if="hasDeficit" class="legend-item">
        <div class="legend-color deficit"></div>
        <span class="legend-text">Deficit: {{ formatPower(Math.abs(powerBalance)) }}</span>
      </div>
    </div>

    <!-- Factory Power Breakdown -->
    <div v-if="factoryStats.length > 0" class="factory-breakdown">
      <div class="breakdown-header" @click="toggleBreakdown">
        <h3 class="breakdown-title">Factory Power Breakdown</h3>
        <button
          type="button"
          class="toggle-button"
          :aria-expanded="isBreakdownExpanded"
          :aria-label="isBreakdownExpanded ? 'Collapse factory breakdown' : 'Expand factory breakdown'"
          @click.stop="toggleBreakdown"
        >
          <svg
            class="chevron-icon"
            :class="{ 'expanded': isBreakdownExpanded }"
            width="20"
            height="20"
            viewBox="0 0 20 20"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M6 8L10 12L14 8"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
        </button>
      </div>
      <Transition name="slide-fade">
        <div v-show="isBreakdownExpanded" class="factory-list">
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
                  :title="`Generation: ${formatPower(factory.generation)}`"
                ></div>
                <div
                  v-if="factory.consumption > 0"
                  class="power-segment-small consumption"
                  :style="{ width: getFactoryConsumptionPercentage(factory) + '%' }"
                  :title="`Consumption: ${formatPower(factory.consumption)}`"
                ></div>
              </div>
            </div>
            <div class="power-numbers">
              <span class="generation-text">{{ formatPower(factory.generation) }}</span>
              <span class="consumption-text">{{ formatPower(factory.consumption) }}</span>
            </div>
          </div>
        </div>
      </div>
      </Transition>
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
import { computed, ref } from 'vue'
import LoadingSpinner from '@/components/ui/LoadingSpinner.vue'
import type { PowerStats } from '@/api/types'

interface Props {
  powerStats?: PowerStats | null
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
})

// Factory breakdown collapse state (default: folded)
const isBreakdownExpanded = ref(false)

const toggleBreakdown = () => {
  isBreakdownExpanded.value = !isBreakdownExpanded.value
}

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

const consumptionPercentage = computed(() => {
  return (totalConsumption.value / maxPower.value) * 100
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

const getFactoryConsumptionPercentage = (factory: { generation: number; consumption: number }): number => {
  const maxFactoryPower = Math.max(factory.generation, factory.consumption, 1)
  return (factory.consumption / maxFactoryPower) * 100
}
</script>

<style scoped lang="scss">
.power-stats-chart {
  background: var(--color-surface, #252525);
  border-radius: var(--border-radius-sm, 3px);
  box-shadow: var(--shadow-inset);
  border: 1px solid var(--color-border, #404040);
  padding: var(--spacing-xl, 1.25rem);
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
  color: var(--color-text-primary, #e5e5e5);
  margin: 0;
  letter-spacing: -0.01em;
}

.power-status {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
  padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
  border-radius: var(--border-radius-sm, 3px);
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);

  &.surplus {
    background-color: rgba(34, 197, 94, 0.1);
    color: var(--color-success, #22c55e);
    border: 1px solid var(--color-success, #22c55e);
  }

  &.deficit {
    background-color: rgba(239, 68, 68, 0.1);
    color: var(--color-error, #ef4444);
    border: 1px solid var(--color-error, #ef4444);
  }

  &.balanced {
    background-color: rgba(245, 158, 11, 0.1);
    color: var(--color-warning, #f59e0b);
    border: 1px solid var(--color-warning, #f59e0b);
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
  height: 40px;
  background: linear-gradient(135deg, #e5e7eb 0%, #f3f4f6 100%);
  border-radius: var(--border-radius-lg, 0.5rem);
  overflow: hidden;
  display: flex;
  margin-bottom: var(--spacing-sm, 0.5rem);
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(226, 232, 240, 0.8);
}

.power-segment {
  height: 100%;
  transition: width 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  
  &:not(:last-child) {
    border-right: 2px solid rgba(255, 255, 255, 0.3);
  }
  
  &:hover {
    filter: brightness(1.1);
    z-index: 1;
  }

  &.generation {
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    box-shadow: inset 0 2px 4px rgba(16, 185, 129, 0.2);
  }

  &.consumption {
    background: linear-gradient(135deg, #f97316 0%, #ea580c 100%);
    box-shadow: inset 0 2px 4px rgba(249, 115, 22, 0.2);
  }

  &.deficit {
    background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
    box-shadow: inset 0 2px 4px rgba(239, 68, 68, 0.2);
  }
}

.power-labels {
  display: flex;
  justify-content: space-between;
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-text-secondary, #b8b8b8);
}

.power-legend {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-md, 0.75rem);
  margin-bottom: var(--spacing-lg, 1rem);
  padding-bottom: var(--spacing-lg, 1rem);
  border-bottom: 1px solid var(--color-border, #404040);
}

.legend-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-secondary, #b8b8b8);
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
  padding-top: var(--spacing-lg, 1rem);
  border-top: 1px solid var(--color-border, #404040);
}

.breakdown-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  cursor: pointer;
  user-select: none;
  padding: var(--spacing-sm, 0.5rem);
  margin: calc(var(--spacing-sm, 0.5rem) * -1);
  border-radius: var(--border-radius-md, 0.375rem);
  transition: background-color 0.2s ease;

  &:hover {
    background-color: var(--color-surface-hover, #2a2a2a);
  }
}

.breakdown-title {
  font-size: var(--font-size-xl, 1.25rem);
  font-weight: var(--font-weight-bold, 700);
  color: var(--color-text-primary, #e5e5e5);
  margin: 0;
  letter-spacing: -0.02em;
}

.toggle-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  border-radius: var(--border-radius-md, 0.375rem);
  cursor: pointer;
  color: var(--color-text-secondary, #b8b8b8);
  transition: all 0.2s ease;

  &:hover {
    background-color: var(--color-surface-hover, #2a2a2a);
    color: var(--color-text-primary, #e5e5e5);
  }

  &:focus {
    outline: none;
    box-shadow: 0 0 0 3px rgba(245, 139, 0, 0.2);
  }
}

.chevron-icon {
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);

  &.expanded {
    transform: rotate(180deg);
  }
}

.factory-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md, 0.75rem);
  margin-top: var(--spacing-lg, 1rem);
}

// Slide-fade transition for factory breakdown
.slide-fade-enter-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.slide-fade-leave-active {
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.slide-fade-enter-from {
  opacity: 0;
  transform: translateY(-10px);
}

.slide-fade-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

.factory-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg, 1rem);
  background: var(--color-surface, #252525);
  border-radius: var(--border-radius-sm, 3px);
  border: 1px solid var(--color-border, #404040);
  box-shadow: var(--shadow-inset-light);
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);

  &:hover {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
    transform: translateY(-1px);
    border-color: rgba(59, 130, 246, 0.2);
  }
}

.factory-info {
  flex: 1;
  min-width: 0;
}

.factory-name {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary, #e5e5e5);
  margin: 0 0 var(--spacing-xs, 0.25rem) 0;
}

.factory-details {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-text-secondary, #b8b8b8);
}

.factory-power {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: var(--spacing-sm, 0.5rem);
  min-width: 200px;
}

.power-bars {
  width: 100%;
  max-width: 100px;
}

.power-bar-small {
  width: 100%;
  height: 12px;
  background: linear-gradient(135deg, #e5e7eb 0%, #f3f4f6 100%);
  border-radius: var(--border-radius-md, 0.375rem);
  overflow: hidden;
  display: flex;
  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(226, 232, 240, 0.8);
}

.power-segment-small {
  height: 100%;
  transition: width 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;

  &:not(:last-child) {
    border-right: 2px solid rgba(255, 255, 255, 0.3);
  }

  &:hover {
    filter: brightness(1.1);
    z-index: 1;
  }

  &.generation {
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  }

  &.consumption {
    background: linear-gradient(135deg, #f97316 0%, #ea580c 100%);
  }
}

.power-numbers {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: var(--spacing-md, 0.75rem);
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-semibold, 600);
  margin-top: var(--spacing-xs, 0.25rem);
}

.generation-text {
  color: var(--color-success, #22c55e);
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);

  &::before {
    content: '';
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    box-shadow: 0 0 0 2px rgba(16, 185, 129, 0.3);
  }
}

.consumption-text {
  color: var(--color-warning, #f59e0b);
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);

  &::before {
    content: '';
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: linear-gradient(135deg, #f97316 0%, #ea580c 100%);
    box-shadow: 0 0 0 2px rgba(249, 115, 22, 0.3);
  }
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl, 1.25rem);
  gap: var(--spacing-md, 0.75rem);
  color: var(--color-text-secondary, #b8b8b8);
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
  color: var(--color-text-muted, #8a8a8a);
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
