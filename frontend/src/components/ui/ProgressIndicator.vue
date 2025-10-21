<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  value?: number
  max?: number
  indeterminate?: boolean
  size?: 'sm' | 'md' | 'lg'
  variant?: 'primary' | 'secondary' | 'success' | 'warning' | 'error'
  showLabel?: boolean
  label?: string
  animated?: boolean
  striped?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  value: 0,
  max: 100,
  indeterminate: false,
  size: 'md',
  variant: 'primary',
  showLabel: false,
  animated: false,
  striped: false
})

// Computed percentage
const percentage = computed(() => {
  if (props.indeterminate) return 0
  return Math.min(Math.max((props.value / props.max) * 100, 0), 100)
})

// Computed label text
const labelText = computed(() => {
  if (props.label) return props.label
  return `${Math.round(percentage.value)}%`
})

// Size classes
const sizeClasses = computed(() => {
  const sizes = {
    sm: 'h-1',
    md: 'h-2',
    lg: 'h-3'
  }
  return sizes[props.size]
})

// Variant classes
const variantClasses = computed(() => {
  const variants = {
    primary: 'bg-blue-600',
    secondary: 'bg-gray-600',
    success: 'bg-green-600',
    warning: 'bg-yellow-600',
    error: 'bg-red-600'
  }
  return variants[props.variant]
})

// Background classes
const backgroundClasses = computed(() => {
  return 'bg-gray-200 dark:bg-gray-700'
})

// Animation classes
const animationClasses = computed(() => {
  if (!props.animated && !props.striped) return ''

  const classes: string[] = []

  if (props.striped) {
    classes.push('bg-stripes')
  }

  if (props.animated) {
    classes.push('animate-stripes')
  }

  return classes.join(' ')
})

// Progress bar classes
const progressBarClasses = computed(() => {
  return [
    variantClasses.value,
    animationClasses.value,
    'transition-all duration-300 ease-out',
    props.indeterminate ? 'progress-indeterminate' : ''
  ].filter(Boolean).join(' ')
})
</script>

<template>
  <div class="progress-indicator">
    <!-- Label -->
    <div
      v-if="showLabel"
      class="progress-label mb-2 text-sm font-medium text-gray-700 dark:text-gray-300"
    >
      {{ labelText }}
    </div>

    <!-- Progress bar container -->
    <div
      :class="[
        'progress-container',
        'relative overflow-hidden rounded-full',
        sizeClasses,
        backgroundClasses
      ]"
      role="progressbar"
      :aria-valuenow="indeterminate ? undefined : value"
      :aria-valuemin="0"
      :aria-valuemax="max"
      :aria-label="label || 'Progress indicator'"
    >
      <!-- Progress bar -->
      <div
        :class="['progress-bar', progressBarClasses]"
        :style="{
          width: indeterminate ? undefined : `${percentage}%`,
          transform: indeterminate ? 'translateX(-100%)' : undefined
        }"
      />
    </div>

    <!-- Optional additional info -->
    <div v-if="$slots.default" class="progress-info mt-2 text-sm text-gray-600 dark:text-gray-400">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.progress-indicator {
  width: 100%;
}

.progress-container {
  min-height: 0.25rem;
}

.progress-bar {
  height: 100%;
  border-radius: inherit;
}

/* Indeterminate animation */
.progress-indeterminate {
  animation: progress-indeterminate 1.5s infinite ease-in-out;
}

@keyframes progress-indeterminate {
  0% {
    transform: translateX(-100%);
  }
  50% {
    transform: translateX(0%);
  }
  100% {
    transform: translateX(100%);
  }
}

/* Striped pattern */
.bg-stripes {
  background-image: linear-gradient(
    45deg,
    rgba(255, 255, 255, 0.15) 25%,
    transparent 25%,
    transparent 50%,
    rgba(255, 255, 255, 0.15) 50%,
    rgba(255, 255, 255, 0.15) 75%,
    transparent 75%,
    transparent
  );
  background-size: 1rem 1rem;
}

/* Animated stripes */
.animate-stripes {
  animation: progress-stripes 1s linear infinite;
}

@keyframes progress-stripes {
  0% {
    background-position: 0 0;
  }
  100% {
    background-position: 1rem 0;
  }
}

/* Respect prefers-reduced-motion */
@media (prefers-reduced-motion: reduce) {
  .progress-bar {
    transition: none;
  }

  .progress-indeterminate,
  .animate-stripes {
    animation: none;
  }
}

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
  .bg-stripes {
    background-image: linear-gradient(
      45deg,
      rgba(0, 0, 0, 0.15) 25%,
      transparent 25%,
      transparent 50%,
      rgba(0, 0, 0, 0.15) 50%,
      rgba(0, 0, 0, 0.15) 75%,
      transparent 75%,
      transparent
    );
  }
}

/* Focus styles for accessibility */
.progress-container:focus-within {
  outline: 2px solid currentColor;
  outline-offset: 2px;
}
</style>
