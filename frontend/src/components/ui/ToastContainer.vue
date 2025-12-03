<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

// Toast notification interface
interface Toast {
  id: string
  type: 'success' | 'error' | 'warning' | 'info'
  title: string
  message?: string
  duration?: number
  persistent?: boolean
  action?: {
    label: string
    handler: () => void
  }
  timestamp: Date
}

// Toast state
const toasts = ref<Toast[]>([])

// Auto-remove timer
let autoRemoveTimer: number | null = null

// Toast icons
const icons = {
  success: '<svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/></svg>',
  error: '<svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/></svg>',
  warning: '<svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/></svg>',
  info: '<svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/></svg>'
}

// Computed styles
const toastTypes = computed(() => ({
  success: 'bg-green-50 border-green-200 text-green-800 dark:bg-green-900/20 dark:border-green-800 dark:text-green-200',
  error: 'bg-red-50 border-red-200 text-red-800 dark:bg-red-900/20 dark:border-red-800 dark:text-red-200',
  warning: 'bg-yellow-50 border-yellow-200 text-yellow-800 dark:bg-yellow-900/20 dark:border-yellow-800 dark:text-yellow-200',
  info: 'bg-blue-50 border-blue-200 text-blue-800 dark:bg-blue-900/20 dark:border-blue-800 dark:text-blue-200'
}))

// Add a new toast
const addToast = (toast: Omit<Toast, 'id' | 'timestamp'>) => {
  const newToast: Toast = {
    ...toast,
    id: `toast_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
    timestamp: new Date(),
    duration: toast.duration ?? (toast.type === 'error' ? 8000 : 5000)
  }

  toasts.value.push(newToast)

  // Auto-remove if not persistent
  if (!newToast.persistent && newToast.duration && newToast.duration > 0) {
    setTimeout(() => {
      removeToast(newToast.id)
    }, newToast.duration)
  }

  return newToast.id
}

// Remove a toast
const removeToast = (id: string) => {
  const index = toasts.value.findIndex(toast => toast.id === id)
  if (index !== -1) {
    toasts.value.splice(index, 1)
  }
}

// Clear all toasts
const clearAllToasts = () => {
  toasts.value = []
}

// Handle app-error events
const handleAppError = (event: CustomEvent) => {
  const { id, message, severity, retryable, retryCallback } = event.detail

  let type: Toast['type'] = 'info'
  if (severity === 'critical' || severity === 'high') {
    type = 'error'
  } else if (severity === 'medium') {
    type = 'warning'
  }

  const toastId = addToast({
    type,
    title: message,
    persistent: severity === 'critical',
    action: retryable && retryCallback ? {
      label: 'Retry',
      handler: () => {
        // Execute retry callback if provided
        if (retryCallback) {
          retryCallback()
        }
        // Also emit retry event for backward compatibility
        window.dispatchEvent(new CustomEvent('toast-retry', {
          detail: { errorId: id, toastId }
        }))
      }
    } : undefined
  })
}

// Handle app-toast events (for success/info/warning)
const handleAppToast = (event: CustomEvent) => {
  const { type, title, message, duration } = event.detail
  addToast({
    type: type || 'info',
    title: title || message || '',
    message: message && title ? message : undefined,
    duration
  })
}

// Setup event listeners
onMounted(() => {
  window.addEventListener('app-error', handleAppError as EventListener)
  window.addEventListener('app-toast', handleAppToast as EventListener)

  // Setup auto-remove timer
  autoRemoveTimer = window.setInterval(() => {
    const now = new Date()
    toasts.value = toasts.value.filter(toast => {
      if (toast.persistent) return true
      if (!toast.duration) return true

      const elapsed = now.getTime() - toast.timestamp.getTime()
      return elapsed < toast.duration
    })
  }, 1000)
})

onUnmounted(() => {
  window.removeEventListener('app-error', handleAppError as EventListener)
  window.removeEventListener('app-toast', handleAppToast as EventListener)
  if (autoRemoveTimer) {
    clearInterval(autoRemoveTimer)
  }
})

// Expose methods for programmatic use
defineExpose({
  addToast,
  removeToast,
  clearAllToasts,
  success: (title: string, message?: string) => addToast({ type: 'success', title, message }),
  error: (title: string, message?: string, persistent?: boolean) => addToast({ type: 'error', title, message, persistent }),
  warning: (title: string, message?: string) => addToast({ type: 'warning', title, message }),
  info: (title: string, message?: string) => addToast({ type: 'info', title, message })
})
</script>

<template>
  <Teleport to="body">
    <div class="toast-container">
      <TransitionGroup
        name="toast"
        tag="div"
        class="fixed top-4 right-4 z-50 space-y-2 max-w-sm w-full"
      >
        <div
          v-for="toast in toasts"
          :key="toast.id"
          :data-test="`toast-${toast.type}`"
          :class="[
            'toast',
            toastTypes[toast.type],
            'border rounded-lg shadow-lg p-4 flex items-start space-x-3 min-w-0 max-w-full'
          ]"
          role="alert"
          :aria-live="toast.type === 'error' ? 'assertive' : 'polite'"
        >
          <!-- Icon -->
          <div class="flex-shrink-0" v-html="icons[toast.type]"></div>

          <!-- Content -->
          <div class="flex-1 min-w-0">
            <h4 class="text-sm font-medium truncate">{{ toast.title }}</h4>
            <p v-if="toast.message" class="text-sm mt-1 opacity-90">{{ toast.message }}</p>

            <!-- Action button -->
            <button
              v-if="toast.action"
              @click="toast.action.handler"
              class="mt-2 text-sm font-medium underline hover:no-underline focus:outline-none focus:ring-2 focus:ring-offset-2 rounded"
              :class="{
                'focus:ring-green-500': toast.type === 'success',
                'focus:ring-red-500': toast.type === 'error',
                'focus:ring-yellow-500': toast.type === 'warning',
                'focus:ring-blue-500': toast.type === 'info'
              }"
            >
              {{ toast.action.label }}
            </button>
          </div>

          <!-- Close button -->
          <button
            @click="removeToast(toast.id)"
            class="flex-shrink-0 p-1 rounded-md hover:bg-black/10 dark:hover:bg-white/10 focus:outline-none focus:ring-2 focus:ring-offset-2"
            :class="{
              'focus:ring-green-500': toast.type === 'success',
              'focus:ring-red-500': toast.type === 'error',
              'focus:ring-yellow-500': toast.type === 'warning',
              'focus:ring-blue-500': toast.type === 'info'
            }"
            :aria-label="`Close ${toast.type} notification`"
          >
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8 2.146 2.854Z"/>
            </svg>
          </button>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<style scoped>
.toast-container {
  pointer-events: none;
}

.toast {
  pointer-events: auto;
}

/* Toast transitions */
.toast-enter-active {
  transition: all 0.3s ease-out;
}

.toast-leave-active {
  transition: all 0.2s ease-in;
}

.toast-enter-from {
  transform: translateX(100%);
  opacity: 0;
}

.toast-leave-to {
  transform: translateX(100%);
  opacity: 0;
}

.toast-move {
  transition: transform 0.3s ease;
}

/* Ensure proper stacking */
.toast-enter-active,
.toast-leave-active {
  position: absolute;
  right: 0;
  left: 0;
}

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
  .toast {
    backdrop-filter: blur(8px);
  }
}

/* Mobile adjustments */
@media (max-width: 640px) {
  .fixed.top-4.right-4 {
    top: 1rem;
    right: 1rem;
    left: 1rem;
    max-width: none;
  }
}
</style>
