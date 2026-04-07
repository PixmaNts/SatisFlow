<template>
  <div class="transport-selector">
    <label class="selector-label">Transport Type</label>
    <div class="transport-options">
      <div
        v-for="option in transportOptions"
        :key="option.value"
        class="transport-option"
        :class="{ 'transport-option--selected': modelValue === option.value }"
        @click="selectTransport(option.value)"
      >
        <div class="transport-icon">
          <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            v-html="option.icon"
          />
        </div>
        <div class="transport-info">
          <h3 class="transport-name">{{ option.label }}</h3>
          <p class="transport-description">{{ option.description }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { TransportType } from '@/api/types'
import type { TransportTypeOption } from '@/api/logistics-types'

interface Props {
  modelValue: TransportType | null
}

interface Emits {
  'update:modelValue': [value: TransportType]
}

defineProps<Props>()
const emit = defineEmits<Emits>()

const transportOptions: TransportTypeOption[] = [
  {
    value: 'Bus',
    label: 'Bus',
    description: 'Multiple conveyors and pipelines for high throughput',
    icon: `<path d="M4 6h16v2H4V6zm0 5h16v2H4v-2zm0 5h16v2H4v-2z" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>`
  },
  {
    value: 'Train',
    label: 'Train',
    description: 'Multiple wagons for bulk transport',
    icon: `<rect x="3" y="8" width="18" height="8" rx="1" stroke="currentColor" stroke-width="2" fill="none"/>
               <circle cx="7" cy="18" r="2" stroke="currentColor" stroke-width="2" fill="none"/>
               <circle cx="17" cy="18" r="2" stroke="currentColor" stroke-width="2" fill="none"/>
               <rect x="8" y="10" width="3" height="4" fill="currentColor"/>
               <rect x="13" y="10" width="3" height="4" fill="currentColor"/>`
  },
  {
    value: 'Truck',
    label: 'Truck',
    description: 'Single vehicle for flexible transport',
    icon: `<rect x="4" y="10" width="12" height="6" rx="1" stroke="currentColor" stroke-width="2" fill="none"/>
               <rect x="16" y="12" width="4" height="4" rx="1" stroke="currentColor" stroke-width="2" fill="none"/>
               <circle cx="8" cy="18" r="2" stroke="currentColor" stroke-width="2" fill="none"/>
               <circle cx="14" cy="18" r="2" stroke="currentColor" stroke-width="2" fill="none"/>
               <circle cx="18" cy="18" r="1.5" stroke="currentColor" stroke-width="2" fill="none"/>`
  },
  {
    value: 'Drone',
    label: 'Drone',
    description: 'Fast aerial transport for small quantities',
    icon: `<ellipse cx="12" cy="14" rx="3" ry="2" stroke="currentColor" stroke-width="2" fill="none"/>
               <path d="M12 12v6M9 9l6 6M15 9l-6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
               <circle cx="9" cy="9" r="1" fill="currentColor"/>
               <circle cx="15" cy="9" r="1" fill="currentColor"/>`
  }
]

const selectTransport = (transportType: TransportType) => {
  emit('update:modelValue', transportType)
}
</script>

<style scoped lang="scss">
.transport-selector {
  width: 100%;
}

.selector-label {
  display: block;
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-secondary, #b8b8b8);
  margin-bottom: var(--spacing-sm, 0.5rem);
}

.transport-options {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: var(--spacing-md, 0.75rem);
}

.transport-option {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-md, 0.75rem);
  padding: var(--spacing-md, 0.75rem);
  border: 2px solid var(--color-border, #404040);
  border-radius: var(--border-radius-md, 0.375rem);
  cursor: pointer;
  transition: all 0.2s ease-in-out;
  background-color: var(--color-surface-inset, #1f1f1f);

  &:hover {
    border-color: var(--color-border-light);
    background-color: var(--color-surface-hover);
  }

  &--selected {
    border-color: var(--color-ficsit-orange);
    background-color: var(--color-surface-inset);
    box-shadow: 0 0 0 2px rgba(245, 139, 0, 0.2);
  }
}

.transport-icon {
  flex-shrink: 0;
  width: 3rem;
  height: 3rem;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-secondary);
  background-color: var(--color-surface-inset);
  border-radius: var(--border-radius-md, 0.375rem);
  transition: color 0.2s ease-in-out;

  .transport-option--selected & {
    color: var(--color-ficsit-orange);
    background-color: rgba(245, 139, 0, 0.1);
  }
}

.transport-info {
  flex: 1;
  min-width: 0;
}

.transport-name {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-text-primary, #e5e5e5);
  margin: 0 0 var(--spacing-xs, 0.25rem) 0;
}

.transport-description {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-muted, #8a8a8a);
  margin: 0;
  line-height: 1.4;
}

// Responsive design
@media (max-width: 640px) {
  .transport-options {
    grid-template-columns: 1fr;
  }

  .transport-option {
    padding: var(--spacing-sm, 0.5rem);
    gap: var(--spacing-sm, 0.5rem);
  }

  .transport-icon {
    width: 2.5rem;
    height: 2.5rem;
  }

  .transport-name {
    font-size: var(--font-size-sm, 0.875rem);
  }

  .transport-description {
    font-size: var(--font-size-xs, 0.75rem);
  }
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'LogisticsTransportSelector'
}
</script>
