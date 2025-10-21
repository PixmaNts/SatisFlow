<template>
  <Transition name="alert" appear>
    <div
      v-if="visible"
      :class="alertClasses"
      role="alert"
      :aria-live="type === 'error' ? 'assertive' : 'polite'"
    >
      <div class="alert-icon">
        <component :is="iconComponent" />
      </div>

      <div class="alert-content">
        <slot />
      </div>

      <button
        v-if="dismissible"
        type="button"
        class="alert-close"
        :aria-label="closeAriaLabel"
        @click="handleDismiss"
      >
        <svg
          width="20"
          height="20"
          viewBox="0 0 20 20"
          fill="currentColor"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            fill-rule="evenodd"
            d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
            clip-rule="evenodd"
          />
        </svg>
      </button>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

/**
 * Alert component props
 */
interface Props {
  /** Alert type/style */
  type?: 'info' | 'success' | 'warning' | 'error'
  /** Whether the alert can be dismissed */
  dismissible?: boolean
  /** Auto-dismiss timeout in milliseconds (0 to disable) */
  timeout?: number
  /** Accessibility label for close button */
  closeAriaLabel?: string
}

const props = withDefaults(defineProps<Props>(), {
  type: 'info',
  dismissible: false,
  timeout: 0,
  closeAriaLabel: 'Dismiss alert',
})

/** Emit alert events */
const emit = defineEmits<{
  dismiss: []
}>()

// Internal state
const visible = ref(true)
let timeoutId: number | undefined

// Icon components as simple SVG paths
const icons = {
  info: {
    viewBox: '0 0 20 20',
    path: 'M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z'
  },
  success: {
    viewBox: '0 0 20 20',
    path: 'M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z'
  },
  warning: {
    viewBox: '0 0 20 20',
    path: 'M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z'
  },
  error: {
    viewBox: '0 0 20 20',
    path: 'M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z'
  }
}

/** Compute alert classes based on type */
const alertClasses = computed(() => [
  'alert',
  `alert-${props.type}`,
  {
    'alert-dismissible': props.dismissible,
  },
])

/** Get the appropriate icon component for the alert type */
const iconComponent = computed(() => {
  const icon = icons[props.type]
  return {
    template: `
      <svg viewBox="${icon.viewBox}" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
        <path fill-rule="evenodd" d="${icon.path}" clip-rule="evenodd" />
      </svg>
    `
  }
})

/** Handle dismiss action */
const handleDismiss = () => {
  visible.value = false
  emit('dismiss')
}

/** Set up auto-dismiss timeout */
const setupTimeout = () => {
  if (props.timeout > 0) {
    timeoutId = window.setTimeout(() => {
      handleDismiss()
    }, props.timeout)
  }
}

/** Clear timeout if component is unmounted */
const clearTimeout = () => {
  if (timeoutId) {
    window.clearTimeout(timeoutId)
    timeoutId = undefined
  }
}

// Set up timeout on mount
onMounted(() => {
  setupTimeout()
})

// Clean up on unmount
onUnmounted(() => {
  clearTimeout()
})
</script>

<style scoped lang="scss">
.alert {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-sm, 0.5rem);
  padding: var(--spacing-md, 0.75rem);
  border-radius: var(--border-radius-md, 0.375rem);
  font-family: var(--font-family-sans, system-ui, sans-serif);
  font-size: var(--font-size-sm, 0.875rem);
  line-height: 1.5;
  position: relative;

  &.alert-info {
    background-color: var(--color-blue-50, #eff6ff);
    color: var(--color-blue-800, #1e40af);
    border: 1px solid var(--color-blue-200, #bfdbfe);
  }

  &.alert-success {
    background-color: var(--color-green-50, #f0fdf4);
    color: var(--color-green-800, #166534);
    border: 1px solid var(--color-green-200, #bbf7d0);
  }

  &.alert-warning {
    background-color: var(--color-yellow-50, #fffbeb);
    color: var(--color-yellow-800, #92400e);
    border: 1px solid var(--color-yellow-200, #fde68a);
  }

  &.alert-error {
    background-color: var(--color-red-50, #fef2f2);
    color: var(--color-red-800, #991b1b);
    border: 1px solid var(--color-red-200, #fecaca);
  }

  &.alert-dismissible {
    padding-right: 2.5rem;
  }
}

.alert-icon {
  flex-shrink: 0;
  width: 1.25rem;
  height: 1.25rem;
  margin-top: 0.125rem;
}

.alert-content {
  flex: 1;
  min-width: 0;
}

.alert-close {
  position: absolute;
  top: var(--spacing-sm, 0.5rem);
  right: var(--spacing-sm, 0.5rem);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 1.5rem;
  height: 1.5rem;
  border: none;
  background: none;
  color: inherit;
  opacity: 0.7;
  border-radius: var(--border-radius-sm, 0.25rem);
  cursor: pointer;
  transition: opacity 0.2s ease-in-out;

  &:hover,
  &:focus {
    opacity: 1;
  }

  &:focus-visible {
    outline: 2px solid currentColor;
    outline-offset: 1px;
  }
}

// Transition animations
.alert-enter-active,
.alert-leave-active {
  transition: all 0.3s ease;
}

.alert-enter-from {
  opacity: 0;
  transform: translateY(-0.5rem);
}

.alert-leave-to {
  opacity: 0;
  transform: translateY(-0.5rem);
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'UiAlert'
}
</script>
