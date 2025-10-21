<script setup lang="ts">
import { ref, onErrorCaptured, computed } from 'vue'
import { useRouter } from 'vue-router'
import { errorHandler } from '@/composables/useErrorHandler'

interface ErrorInfo {
  error: Error
  info: string
  timestamp: Date
  componentStack?: string
}

const props = withDefaults(defineProps<{
  fallback?: 'default' | 'minimal' | 'custom'
  showRetry?: boolean
  showDetails?: boolean
  onError?: (error: Error, info: string) => void
}>(), {
  fallback: 'default',
  showRetry: true,
  showDetails: false
})

const emit = defineEmits<{
  error: [error: Error, info: string]
  recovered: []
}>()

const router = useRouter()
const hasError = ref(false)
const errorInfo = ref<ErrorInfo | null>(null)
const isRetrying = ref(false)

// Computed error message
const errorMessage = computed(() => {
  if (!errorInfo.value) return ''

  const error = errorInfo.value.error
  return error.message || 'An unexpected error occurred'
})

// Computed error details
const errorDetails = computed(() => {
  if (!errorInfo.value) return ''

  const { error, info, componentStack } = errorInfo.value
  let details = `Error: ${error.name}\n`
  details += `Message: ${error.message}\n`
  details += `Info: ${info}\n`

  if (error.stack) {
    details += `\nStack:\n${error.stack}`
  }

  if (componentStack) {
    details += `\n\nComponent Stack:\n${componentStack}`
  }

  return details
})

// Handle error capture
onErrorCaptured((error: Error, instance, info: string) => {
  console.error('ErrorBoundary caught an error:', error, info)

  // Store error info
  errorInfo.value = {
    error,
    info,
    timestamp: new Date(),
    componentStack: 'Unknown'
  }

  // Set error state
  hasError.value = true

  // Handle through global error handler
  errorHandler.handleError(error, {
    component: 'Unknown',
    info,
    route: router.currentRoute.value.path
  }, {
    userMessage: 'Something went wrong in this component. Please try refreshing the page.',
    severity: 'high'
  })

  // Call custom error handler if provided
  if (props.onError) {
    props.onError(error, info)
  }

  // Emit error event
  emit('error', error, info)

  // Prevent error from propagating
  return false
})

// Retry function
const retry = async () => {
  if (!errorInfo.value) return

  isRetrying.value = true

  try {
    // Clear error state
    hasError.value = false
    errorInfo.value = null

    // Emit recovery event
    emit('recovered')

    // Wait a bit for the component to re-render
    await new Promise(resolve => setTimeout(resolve, 100))
  } catch (error) {
    console.error('Retry failed:', error)
    errorHandler.handleError(error, {
      action: 'retry',
      originalError: errorInfo.value?.error.message
    })
  } finally {
    isRetrying.value = false
  }
}

// Reset error state
const reset = () => {
  hasError.value = false
  errorInfo.value = null
}

// Go home
const goHome = () => {
  router.push('/')
  reset()
}

// Reload page
const reloadPage = () => {
  window.location.reload()
}

// Copy error details
const copyErrorDetails = async () => {
  try {
    await navigator.clipboard.writeText(errorDetails.value)
    // Show success feedback (you could use the toast system here)
    console.log('Error details copied to clipboard')
  } catch (error) {
    console.error('Failed to copy error details:', error)
  }
}
</script>

<template>
  <div class="error-boundary">
    <!-- Normal content -->
    <div v-if="!hasError">
      <slot />
    </div>

    <!-- Default error fallback -->
    <div
      v-else-if="fallback === 'default'"
      class="error-fallback-default"
      role="alert"
      aria-live="assertive"
    >
      <div class="max-w-md mx-auto text-center p-6">
        <!-- Error icon -->
        <div class="mb-4">
          <svg
            class="w-16 h-16 mx-auto text-red-500"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z"
            />
          </svg>
        </div>

        <!-- Error message -->
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
          Something went wrong
        </h2>
        <p class="text-gray-600 dark:text-gray-300 mb-6">
          {{ errorMessage }}
        </p>

        <!-- Action buttons -->
        <div class="space-y-3">
          <button
            v-if="showRetry"
            @click="retry"
            :disabled="isRetrying"
            class="w-full px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span v-if="isRetrying">Retrying...</span>
            <span v-else>Try Again</span>
          </button>

          <button
            @click="reloadPage"
            class="w-full px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500"
          >
            Refresh Page
          </button>

          <button
            @click="goHome"
            class="w-full px-4 py-2 border border-gray-300 text-gray-700 dark:border-gray-600 dark:text-gray-300 rounded-md hover:bg-gray-50 dark:hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-gray-500"
          >
            Go Home
          </button>
        </div>

        <!-- Error details toggle -->
        <div v-if="showDetails" class="mt-6 text-left">
          <button
            @click="copyErrorDetails"
            class="text-sm text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          >
            Copy Error Details
          </button>
        </div>
      </div>
    </div>

    <!-- Minimal error fallback -->
    <div
      v-else-if="fallback === 'minimal'"
      class="error-fallback-minimal p-4 text-center"
      role="alert"
      aria-live="assertive"
    >
      <p class="text-red-600 dark:text-red-400 mb-2">
        Component failed to load
      </p>
      <button
        v-if="showRetry"
        @click="retry"
        :disabled="isRetrying"
        class="text-sm text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-200 disabled:opacity-50"
      >
        <span v-if="isRetrying">Retrying...</span>
        <span v-else>Retry</span>
      </button>
    </div>

    <!-- Custom fallback slot -->
    <div
      v-else-if="fallback === 'custom'"
      class="error-fallback-custom"
      role="alert"
      aria-live="assertive"
    >
      <slot
        name="fallback"
        :error="errorInfo?.error"
        :info="errorInfo?.info"
        :retry="retry"
        :reset="reset"
        :isRetrying="isRetrying"
      />
    </div>
  </div>
</template>

<style scoped>
.error-boundary {
  min-height: 100%;
}

.error-fallback-default {
  background-color: var(--color-background, #ffffff);
  color: var(--color-text-primary, #111827);
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.error-fallback-minimal {
  background-color: var(--color-error-background, #fef2f2);
  border: 1px solid var(--color-error-border, #fecaca);
  border-radius: 4px;
}

.error-fallback-custom {
  /* Custom styling handled by slot content */
}

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
  .error-fallback-default {
    background-color: var(--color-background-dark, #1f2937);
    color: var(--color-text-primary-dark, #f9fafb);
  }

  .error-fallback-minimal {
    background-color: var(--color-error-background-dark, #7f1d1d);
    border-color: var(--color-error-border-dark, #991b1b);
  }
}

/* Accessibility improvements */
@media (prefers-reduced-motion: reduce) {
  .error-fallback-default button {
    transition: none;
  }
}

/* Focus visible for better keyboard navigation */
.error-fallback-default button:focus-visible {
  outline: 2px solid currentColor;
  outline-offset: 2px;
}
</style>
