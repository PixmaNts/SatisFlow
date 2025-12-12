<template>
  <button
    :class="buttonClasses"
    :disabled="disabled || loading"
    :aria-disabled="disabled || loading"
    :aria-busy="loading"
    :data-test="`button-${variant}`"
    @click="handleClick"
  >
    <LoadingSpinner
      v-if="loading"
      :size="spinnerSize"
      class="button-spinner"
    />
    <slot v-if="!loading" name="icon" />
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
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost'
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
  border-radius: var(--border-radius-sm, 3px);
  font-family: var(--font-family-sans, system-ui, sans-serif);
  font-weight: var(--font-weight-medium, 500);
  text-decoration: none;
  cursor: pointer;
  transition: all var(--transition-normal, 200ms) cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  white-space: nowrap;

  &:focus-visible {
    outline: 2px solid var(--color-ficsit-orange);
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

  // Style variants - Industrial FICSIT theme
  &.btn-primary {
    background: var(--color-ficsit-orange);
    color: var(--color-text-primary);
    border: 1px solid var(--color-ficsit-orange-dark);
    box-shadow: var(--shadow-sm);
    border-radius: var(--border-radius-sm, 3px);

    &:hover:not(.btn-disabled):not(.btn-loading) {
      background: var(--color-ficsit-orange-light);
      border-color: var(--color-ficsit-orange);
      box-shadow: var(--shadow-glow-orange);
      transform: translateY(-1px);
    }

    &:active:not(.btn-disabled):not(.btn-loading) {
      transform: translateY(0);
      box-shadow: var(--shadow-inset-light);
      background: var(--color-ficsit-orange-dark);
    }

    &:focus-visible {
      outline: 2px solid var(--color-ficsit-orange);
      outline-offset: 2px;
    }
  }

  &.btn-secondary {
    background: var(--color-surface);
    color: var(--color-text-primary);
    border: 1px solid var(--color-border);
    box-shadow: var(--shadow-inset-light);
    border-radius: var(--border-radius-sm, 3px);

    &:hover:not(.btn-disabled):not(.btn-loading) {
      background: var(--color-surface-hover);
      border-color: var(--color-ficsit-orange);
      box-shadow: var(--shadow-inset);
      transform: translateY(-1px);
    }

    &:active:not(.btn-disabled):not(.btn-loading) {
      transform: translateY(0);
      box-shadow: var(--shadow-inset);
      background: var(--color-surface-active);
    }

    &:focus-visible {
      outline: 2px solid var(--color-ficsit-orange);
      outline-offset: 2px;
    }
  }

  &.btn-danger {
    background: var(--color-error);
    color: var(--color-text-primary);
    border: 1px solid var(--color-error-dark);
    box-shadow: var(--shadow-sm);
    border-radius: var(--border-radius-sm, 3px);

    &:hover:not(.btn-disabled):not(.btn-loading) {
      background: var(--color-error-dark);
      box-shadow: 0 0 8px rgba(239, 68, 68, 0.4);
      transform: translateY(-1px);
    }

    &:active:not(.btn-disabled):not(.btn-loading) {
      transform: translateY(0);
      box-shadow: var(--shadow-inset-light);
    }

    &:focus-visible {
      outline: 2px solid var(--color-error);
      outline-offset: 2px;
    }
  }

  &.btn-ghost {
    background-color: transparent;
    color: var(--color-text-secondary);
    border: 1px solid transparent;
    border-radius: var(--border-radius-sm, 3px);

    &:hover:not(.btn-disabled):not(.btn-loading) {
      background-color: var(--color-surface);
      color: var(--color-text-primary);
      border-color: var(--color-border);
    }

    &:active:not(.btn-disabled):not(.btn-loading) {
      background-color: var(--color-surface-active);
    }

    &:focus-visible {
      outline: 2px solid var(--color-ficsit-orange);
      outline-offset: 2px;
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
