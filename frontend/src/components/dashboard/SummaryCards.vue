<template>
  <div class="summary-cards">
    <div class="cards-grid">
      <!-- Total Factories Card -->
      <div class="summary-card">
        <div class="card-icon factories">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M3 9L12 2L21 9V20C21 20.5304 20.7893 21.0391 20.4142 21.4142C20.0391 21.7893 19.5304 22 19 22H5C4.46957 22 3.96086 21.7893 3.58579 21.4142C3.21071 21.0391 3 20.5304 3 20V9Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M9 22V12H15V22" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <div class="card-content">
          <h3 class="card-title">Total Factories</h3>
          <div class="card-value">
            <LoadingSpinner v-if="loading" size="sm" />
            <span v-else>{{ totalFactories }}</span>
          </div>
        </div>
      </div>

      <!-- Total Production Lines Card -->
      <div class="summary-card">
        <div class="card-icon production">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M13 2L3 14H12L11 22L21 10H12L13 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <div class="card-content">
          <h3 class="card-title">Production Lines</h3>
          <div class="card-value">
            <LoadingSpinner v-if="loading" size="sm" />
            <span v-else>{{ totalProductionLines }}</span>
          </div>
        </div>
      </div>

      <!-- Total Logistics Lines Card -->
      <div class="summary-card">
        <div class="card-icon logistics">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M8 7H16M8 12H16M8 17H13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2"/>
          </svg>
        </div>
        <div class="card-content">
          <h3 class="card-title">Logistics Lines</h3>
          <div class="card-value">
            <LoadingSpinner v-if="loading" size="sm" />
            <span v-else>{{ totalLogisticsLines }}</span>
          </div>
        </div>
      </div>

      <!-- Power Consumption Card -->
      <div class="summary-card">
        <div class="card-icon consumption">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M13 2L3 14H12L11 22L21 10H12L13 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <div class="card-content">
          <h3 class="card-title">Power Consumption</h3>
          <div class="card-value">
            <LoadingSpinner v-if="loading" size="sm" />
            <span v-else>{{ formatPower(totalPowerConsumption) }}</span>
          </div>
        </div>
      </div>

      <!-- Power Generation Card -->
      <div class="summary-card">
        <div class="card-icon generation">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <circle cx="12" cy="12" r="5" stroke="currentColor" stroke-width="2"/>
            <line x1="12" y1="1" x2="12" y2="3" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            <line x1="12" y1="21" x2="12" y2="23" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            <line x1="1" y1="12" x2="3" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            <line x1="21" y1="12" x2="23" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
        </div>
        <div class="card-content">
          <h3 class="card-title">Power Generation</h3>
          <div class="card-value">
            <LoadingSpinner v-if="loading" size="sm" />
            <span v-else>{{ formatPower(totalPowerGeneration) }}</span>
          </div>
        </div>
      </div>

      <!-- Net Power Card -->
      <div class="summary-card" :class="netPowerClass">
        <div class="card-icon" :class="netPowerIconClass">
          <svg v-if="hasPowerSurplus" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M7 14L12 9L17 14" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M12 9V20" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <svg v-else-if="hasPowerDeficit" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M17 10L12 15L7 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M12 15V4" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <svg v-else width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M5 12H19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <div class="card-content">
          <h3 class="card-title">Net Power</h3>
          <div class="card-value">
            <LoadingSpinner v-if="loading" size="sm" />
            <span v-else>{{ formatPower(netPower) }}</span>
          </div>
          <div class="card-status">
            <span v-if="hasPowerSurplus" class="status-text surplus">Surplus</span>
            <span v-else-if="hasPowerDeficit" class="status-text deficit">Deficit</span>
            <span v-else class="status-text balanced">Balanced</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import LoadingSpinner from '@/components/ui/LoadingSpinner.vue'
import type { DashboardSummary, PowerStats } from '@/api/types'

interface Props {
  summary?: DashboardSummary | null
  powerStats?: PowerStats | null
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
})

// Computed properties
const totalFactories = computed(() => props.summary?.total_factories || 0)
const totalProductionLines = computed(() => props.summary?.total_production_lines || 0)
const totalLogisticsLines = computed(() => props.summary?.total_logistics_lines || 0)
const totalPowerConsumption = computed(() => props.summary?.total_power_consumption || 0)
const totalPowerGeneration = computed(() => props.summary?.total_power_generation || 0)
const netPower = computed(() => props.powerStats?.power_balance || 0)

const hasPowerSurplus = computed(() => props.powerStats?.has_surplus || false)
const hasPowerDeficit = computed(() => props.powerStats?.has_deficit || false)
const isPowerBalanced = computed(() => props.powerStats?.is_balanced || false)

const netPowerClass = computed(() => ({
  'surplus': hasPowerSurplus.value,
  'deficit': hasPowerDeficit.value,
  'balanced': isPowerBalanced.value,
}))

const netPowerIconClass = computed(() => ({
  'surplus': hasPowerSurplus.value,
  'deficit': hasPowerDeficit.value,
  'balanced': isPowerBalanced.value,
}))

// Format power value with appropriate unit
const formatPower = (value: number): string => {
  if (Math.abs(value) >= 1000) {
    return `${(value / 1000).toFixed(1)} GW`
  }
  return `${value.toFixed(0)} MW`
}
</script>

<style scoped lang="scss">
.summary-cards {
  width: 100%;
  margin-bottom: var(--spacing-xl, 1.25rem);
}

.cards-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: var(--spacing-lg, 1rem);
}

.summary-card {
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-lg, 0.5rem);
  box-shadow: var(--shadow-sm, 0 1px 2px 0 rgba(0, 0, 0, 0.05));
  padding: var(--spacing-lg, 1rem);
  display: flex;
  align-items: center;
  gap: var(--spacing-md, 0.75rem);
  transition: all 0.2s ease-in-out;
  border: 2px solid transparent;

  &:hover {
    box-shadow: var(--shadow-md, 0 4px 6px -1px rgba(0, 0, 0, 0.1));
    transform: translateY(-2px);
  }

  &.surplus {
    border-color: var(--color-success-500, #10b981);
    background-color: var(--color-success-50, #ecfdf5);
  }

  &.deficit {
    border-color: var(--color-error-500, #ef4444);
    background-color: var(--color-error-50, #fef2f2);
  }

  &.balanced {
    border-color: var(--color-warning-500, #f59e0b);
    background-color: var(--color-warning-50, #fffbeb);
  }
}

.card-icon {
  width: 48px;
  height: 48px;
  border-radius: var(--border-radius-lg, 0.5rem);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  &.factories {
    background-color: var(--color-blue-100, #dbeafe);
    color: var(--color-blue-600, #2563eb);
  }

  &.production {
    background-color: var(--color-purple-100, #ede9fe);
    color: var(--color-purple-600, #9333ea);
  }

  &.logistics {
    background-color: var(--color-green-100, #d1fae5);
    color: var(--color-green-600, #059669);
  }

  &.consumption {
    background-color: var(--color-orange-100, #fed7aa);
    color: var(--color-orange-600, #ea580c);
  }

  &.generation {
    background-color: var(--color-yellow-100, #fef3c7);
    color: var(--color-yellow-600, #d97706);
  }

  &.surplus {
    background-color: var(--color-success-100, #d1fae5);
    color: var(--color-success-600, #059669);
  }

  &.deficit {
    background-color: var(--color-error-100, #fee2e2);
    color: var(--color-error-600, #dc2626);
  }

  &.balanced {
    background-color: var(--color-warning-100, #fef3c7);
    color: var(--color-warning-600, #d97706);
  }
}

.card-content {
  flex: 1;
  min-width: 0;
}

.card-title {
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-600, #4b5563);
  margin: 0 0 var(--spacing-xs, 0.25rem) 0;
  text-transform: uppercase;
  letter-spacing: 0.025em;
}

.card-value {
  font-size: var(--font-size-xl, 1.25rem);
  font-weight: var(--font-weight-bold, 700);
  color: var(--color-gray-900, #111827);
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
}

.card-status {
  margin-top: var(--spacing-xs, 0.25rem);
}

.status-text {
  font-size: var(--font-size-xs, 0.75rem);
  font-weight: var(--font-weight-medium, 500);
  padding: 2px 8px;
  border-radius: var(--border-radius-full, 9999px);
  text-transform: uppercase;
  letter-spacing: 0.05em;

  &.surplus {
    background-color: var(--color-success-200, #a7f3d0);
    color: var(--color-success-800, #065f46);
  }

  &.deficit {
    background-color: var(--color-error-200, #fecaca);
    color: var(--color-error-800, #991b1b);
  }

  &.balanced {
    background-color: var(--color-warning-200, #fde68a);
    color: var(--color-warning-800, #92400e);
  }
}

// Responsive design
@media (max-width: 768px) {
  .cards-grid {
    grid-template-columns: 1fr;
  }

  .summary-card {
    padding: var(--spacing-md, 0.75rem);
  }

  .card-icon {
    width: 40px;
    height: 40px;
  }

  .card-title {
    font-size: var(--font-size-xs, 0.75rem);
  }

  .card-value {
    font-size: var(--font-size-lg, 1.125rem);
  }
}

@media (max-width: 480px) {
  .summary-card {
    flex-direction: column;
    text-align: center;
    gap: var(--spacing-sm, 0.5rem);
  }

  .card-icon {
    width: 36px;
    height: 36px;
  }
}
</style>
