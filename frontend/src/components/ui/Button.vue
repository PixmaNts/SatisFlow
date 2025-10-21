<template>
  <button
    :class="buttonClasses"
    :disabled="disabled || loading"
    :aria-disabled="disabled || loading"
    :aria-busy="loading"
    @click="handleClick"
  >
    <LoadingSpinner
      v-if="loading"
      :size="spinnerSize"
      class="button-spinner"
    />
    <slot />
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import LoadingSpinner from './LoadingSpinner.vue'

/**
 * Button component props
 */
interface Props {
  /** Button variant style */
  variant?: 'primary' | 'secondary' | 'danger'
  /** Button size */
  size?: 'sm' | 'md' | 'lg'
  /** Whether the button is disabled */
  disabled?: boolean
  /** Whether to show loading state */
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  disabled: false,
  loading: false,
})

/** Emit click event */
const emit = defineEmits<{
  click: [event: MouseEvent]
}>()

/** Compute button classes based on props */
const buttonClasses = computed(() => [
  'btn',
  `btn-${props.variant}`,
  `btn-${props.size}`,
  {
    'btn-disabled': props.disabled,
    'btn-loading': props.loading,
  },
])

/** Compute spinner size based on button size */
const spinnerSize = computed<'sm' | 'md' | 'lg'>(() => {
  if (props.size === 'sm') return 'sm'
  if (props.size === 'lg') return 'lg'
  return 'md'
})

/** Handle click event */
const handleClick = (event: MouseEvent) => {
  if (!props.disabled && !props.loading) {
    emit('click', event)
  }
}
</script>

<style scoped lang="scss">
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm, 0.5rem);
  border: none;
  border-radius: var(--border-radius-md, 0.375rem);
  font-family: var(--font-family-sans, system-ui, sans-serif);
  font-weight: var(--font-weight-medium, 500);
  text-decoration: none;
  cursor: pointer;
  transition: all 0.2s ease-in-out;
  position: relative;
  white-space: nowrap;

  &:focus-visible {
    outline: 2px solid var(--color-primary-500, #3b82f6);
    outline-offset: 2px;
  }

  &.btn-disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  &.btn-loading {
    cursor: wait;
  }

  // Size variants
  &.btn-sm {
    padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
    font-size: var(--font-size-sm, 0.875rem);
    min-height: 2rem;
  }

  &.btn-md {
    padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
    font-size: var(--font-size-base, 1rem);
    min-height: 2.5rem;
  }

  &.btn-lg {
    padding: var(--spacing-md, 0.75rem) var(--spacing-lg, 1rem);
    font-size: var(--font-size-lg, 1.125rem);
    min-height: 3rem;
  }

  // Style variants
  &.btn-primary {
    background-color: var(--color-primary-600, #2563eb);
    color: var(--color-white, #ffffff);

    &:hover:not(.btn-disabled):not(.btn-loading) {
      background-color: var(--color-primary-700, #1d4ed8);
    }

    &:active:not(.btn-disabled):not(.btn-loading) {
      background-color: var(--color-primary-800, #1e40af);
    }
  }

  &.btn-secondary {
    background-color: var(--color-gray-100, #f3f4f6);
    color: var(--color-gray-900, #111827);
    border: 1px solid var(--color-gray-300, #d1d5db);

    &:hover:not(.btn-disabled):not(.btn-loading) {
      background-color: var(--color-gray-200, #e5e7eb);
    }

    &:active:not(.btn-disabled):not(.btn-loading) {
      background-color: var(--color-gray-300, #d1d5db);
    }
  }

  &.btn-danger {
    background-color: var(--color-red-600, #dc2626);
    color: var(--color-white, #ffffff);

    &:hover:not(.btn-disabled):not(.btn-loading) {
      background-color: var(--color-red-700, #b91c1c);
    }

    &:active:not(.btn-disabled):not(.btn-loading) {
      background-color: var(--color-red-800, #991b1b);
    }
  }
}

.button-spinner {
  flex-shrink: 0;
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'UiButton'
}
</script>
