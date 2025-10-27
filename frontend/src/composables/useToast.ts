import { ref } from 'vue'

// Toast message interface
export interface ToastMessage {
  id: string
  message: string
  type: 'success' | 'error' | 'warning' | 'info'
  duration?: number
  persistent?: boolean
}

// Toast state
const toasts = ref<ToastMessage[]>([])

/**
 * Toast notification composable
 * Provides methods to show toast notifications
 */
export function useToast() {
  /**
   * Generate unique toast ID
   */
  const generateId = (): string => {
    return `toast_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
  }

  /**
   * Add a new toast message
   */
  const addToast = (message: string, type: ToastMessage['type'], options?: { duration?: number; persistent?: boolean }) => {
    const toast: ToastMessage = {
      id: generateId(),
      message,
      type,
      duration: options?.duration || (type === 'error' ? 5000 : 3000),
      persistent: options?.persistent || false,
    }

    toasts.value.push(toast)

    // Auto-remove if not persistent
    if (!toast.persistent) {
      setTimeout(() => {
        removeToast(toast.id)
      }, toast.duration)
    }

    return toast.id
  }

  /**
   * Remove a toast message
   */
  const removeToast = (id: string) => {
    const index = toasts.value.findIndex(toast => toast.id === id)
    if (index !== -1) {
      toasts.value.splice(index, 1)
    }
  }

  /**
   * Show success message
   */
  const showSuccess = (message: string, options?: { duration?: number }) => {
    return addToast(message, 'success', options)
  }

  /**
   * Show error message
   */
  const showError = (message: string, options?: { duration?: number }) => {
    return addToast(message, 'error', options)
  }

  /**
   * Show warning message
   */
  const showWarning = (message: string, options?: { duration?: number }) => {
    return addToast(message, 'warning', options)
  }

  /**
   * Show info message
   */
  const showInfo = (message: string, options?: { duration?: number }) => {
    return addToast(message, 'info', options)
  }

  /**
   * Clear all toasts
   */
  const clearAll = () => {
    toasts.value = []
  }

  return {
    toasts,
    addToast,
    removeToast,
    showSuccess,
    showError,
    showWarning,
    showInfo,
    clearAll,
  }
}

// Export singleton instance for global use
export const toast = useToast()
