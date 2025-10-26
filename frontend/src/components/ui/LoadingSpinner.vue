<template>
  <div
    :class="spinnerClasses"
    :style="spinnerStyles"
    :aria-label="ariaLabel"
    role="status"
    aria-live="polite"
  >
    <svg
      :class="['spinner-svg', `spinner-svg-${size}`]"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
    >
      <circle
        class="spinner-circle"
        cx="12"
        cy="12"
        r="10"
        fill="none"
        :stroke="strokeColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-dasharray="60"
        stroke-dashoffset="0"
      />
    </svg>
    <span v-if="showLabel" class="sr-only">{{ ariaLabel }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

/**
 * LoadingSpinner component props
 */
interface Props {
  /** Spinner size */
  size?: 'sm' | 'md' | 'lg'
  /** Spinner color */
  color?: string
  /** Accessibility label for screen readers */
  ariaLabel?: string
  /** Whether to show the label for screen readers */
  showLabel?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  size: 'md',
  color: '',
  ariaLabel: 'Loading',
  showLabel: true,
})

/** Compute spinner classes */
const spinnerClasses = computed(() => [
  'loading-spinner',
  `loading-spinner-${props.size}`,
])

/** Compute spinner styles including custom color */
const spinnerStyles = computed(() => {
  if (props.color) {
    return { '--spinner-color': props.color }
  }
  return {}
})

/** Compute stroke color based on color prop or CSS variable */
const strokeColor = computed(() => {
  return props.color || 'var(--color-primary-600, #2563eb)'
})
</script>

<style scoped lang="scss">
.loading-spinner {
  display: inline-flex;
  align-items: center;
  justify-content: center;

  // Size variants
  &.loading-spinner-sm {
    width: 1rem;
    height: 1rem;
  }

  &.loading-spinner-md {
    width: 1.5rem;
    height: 1.5rem;
  }

  &.loading-spinner-lg {
    width: 2rem;
    height: 2rem;
  }
}

.spinner-svg {
  animation: spin 1s linear infinite;

  &.spinner-svg-sm {
    width: 1rem;
    height: 1rem;
  }

  &.spinner-svg-md {
    width: 1.5rem;
    height: 1.5rem;
  }

  &.spinner-svg-lg {
    width: 2rem;
    height: 2rem;
  }
}

.spinner-circle {
  animation: dash 1.5s ease-in-out infinite;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

@keyframes dash {
  0% {
    stroke-dasharray: 1, 60;
    stroke-dashoffset: 0;
  }
  50% {
    stroke-dasharray: 30, 60;
    stroke-dashoffset: -15;
  }
  100% {
    stroke-dasharray: 1, 60;
    stroke-dashoffset: -60;
  }
}

// Screen reader only class
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'UiLoadingSpinner'
}
</script>
