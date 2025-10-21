import { ref, reactive } from 'vue'
import { AxiosError } from 'axios'

// Error severity levels
export type ErrorSeverity = 'low' | 'medium' | 'high' | 'critical'

// Error categories
export type ErrorCategory =
  | 'network'
  | 'validation'
  | 'authentication'
  | 'authorization'
  | 'server'
  | 'client'
  | 'unknown'

// Error interface
export interface AppError {
  id: string
  message: string
  category: ErrorCategory
  severity: ErrorSeverity
  timestamp: Date
  context?: Record<string, unknown>
  userMessage?: string
  retryable?: boolean
  retryCount?: number
  maxRetries?: number
}

// Error handler state
interface ErrorHandlerState {
  errors: AppError[]
  isOnline: boolean
  retryQueue: Array<{
    id: string
    fn: () => Promise<unknown>
    error: AppError
  }>
}

// Global error handler state
const state = reactive<ErrorHandlerState>({
  errors: [],
  isOnline: navigator.onLine,
  retryQueue: []
})

// Error history for debugging (limited to last 100 errors)
const errorHistory = ref<AppError[]>([])

/**
 * Global Error Handler Composable
 *
 * Provides centralized error handling, logging, and user notification
 * capabilities for the entire application.
 */
export function useErrorHandler() {
  /**
   * Generate a unique error ID
   */
  const generateErrorId = (): string => {
    return `error_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
  }

  /**
   * Categorize an error based on its properties
   */
  const categorizeError = (error: unknown): ErrorCategory => {
    if (error instanceof AxiosError) {
      if (!error.response) {
        return 'network'
      }

      const status = error.response.status
      if (status === 401) return 'authentication'
      if (status === 403) return 'authorization'
      if (status >= 400 && status < 500) return 'client'
      if (status >= 500) return 'server'
    }

    if (error instanceof Error) {
      if (error.name === 'ValidationError') return 'validation'
      if (error.name === 'NetworkError') return 'network'
    }

    return 'unknown'
  }

  /**
   * Determine error severity
   */
  const determineSeverity = (error: unknown, category: ErrorCategory): ErrorSeverity => {
    if (category === 'server') return 'high'
    if (category === 'authentication') return 'high'
    if (category === 'network') return 'medium'
    if (category === 'validation') return 'low'

    // For Axios errors, use status code to determine severity
    if (error instanceof AxiosError && error.response) {
      const status = error.response.status
      if (status >= 500) return 'critical'
      if (status === 429) return 'high' // Rate limiting
      if (status >= 400) return 'medium'
    }

    return 'medium'
  }

  /**
   * Extract user-friendly message from error
   */
  const extractUserMessage = (error: unknown, category: ErrorCategory): string => {
    // Check for custom user message in error response
    if (error instanceof AxiosError && error.response?.data?.error) {
      return error.response.data.error
    }

    // Provide category-specific messages
    switch (category) {
      case 'network':
        return 'Network connection issue. Please check your internet connection.'
      case 'authentication':
        return 'Authentication required. Please log in again.'
      case 'authorization':
        return 'You don\'t have permission to perform this action.'
      case 'validation':
        return 'Please check your input and try again.'
      case 'server':
        return 'Server error. Please try again later.'
      case 'client':
        return 'Invalid request. Please try again.'
      default:
        return error instanceof Error ? error.message : 'An unexpected error occurred.'
    }
  }

  /**
   * Determine if an error is retryable
   */
  const isRetryable = (error: unknown, category: ErrorCategory): boolean => {
    if (category === 'validation') return false
    if (category === 'authentication') return false
    if (category === 'authorization') return false

    if (error instanceof AxiosError) {
      const status = error.response?.status
      if (status === 429) return true // Rate limiting
      if (status === 502) return true // Bad gateway
      if (status === 503) return true // Service unavailable
      if (status === 504) return true // Gateway timeout
      if (!error.response) return true // Network error
    }

    return category === 'network' || category === 'server'
  }

  /**
   * Log error to console and external services
   */
  const logError = (appError: AppError, originalError: unknown) => {
    // Log to console with appropriate level
    const logMethod = {
      low: 'info',
      medium: 'warn',
      high: 'error',
      critical: 'error'
    }[appError.severity] as 'info' | 'warn' | 'error'

    console[logMethod]('Application Error:', {
      id: appError.id,
      message: appError.message,
      category: appError.category,
      severity: appError.severity,
      timestamp: appError.timestamp,
      context: appError.context,
      originalError
    })

    // In production, you would send this to your error tracking service
    // Example: Sentry, LogRocket, etc.
    if (import.meta.env.PROD) {
      // sendToErrorTracking(appError, originalError)
    }
  }

  /**
   * Handle a new error
   */
  const handleError = (
    error: unknown,
    context?: Record<string, unknown>,
    options?: {
      userMessage?: string
      severity?: ErrorSeverity
      silent?: boolean
    }
  ): AppError => {
    const category = categorizeError(error)
    const severity = options?.severity || determineSeverity(error, category)
    const userMessage = options?.userMessage || extractUserMessage(error, category)
    const retryable = isRetryable(error, category)

    const appError: AppError = {
      id: generateErrorId(),
      message: error instanceof Error ? error.message : String(error),
      category,
      severity,
      timestamp: new Date(),
      context,
      userMessage,
      retryable,
      retryCount: 0,
      maxRetries: retryable ? 3 : 0
    }

    // Add to state
    state.errors.push(appError)

    // Add to history (keep only last 100)
    errorHistory.value.push(appError)
    if (errorHistory.value.length > 100) {
      errorHistory.value.shift()
    }

    // Log the error
    logError(appError, error)

    // Show user notification (unless silent)
    if (!options?.silent) {
      showErrorNotification(appError)
    }

    return appError
  }

  /**
   * Show error notification to user
   */
  const showErrorNotification = (error: AppError) => {
    // This will be implemented with the toast notification system
    // For now, we'll use a simple console message
    console.warn('User Notification:', error.userMessage)

    // Emit custom event for toast system to handle
    window.dispatchEvent(new CustomEvent('app-error', {
      detail: {
        id: error.id,
        message: error.userMessage,
        severity: error.severity,
        category: error.category,
        retryable: error.retryable
      }
    }))
  }

  /**
   * Add a function to the retry queue
   */
  const addToRetryQueue = (errorId: string, fn: () => Promise<unknown>) => {
    const error = state.errors.find(e => e.id === errorId)
    if (error && error.retryable) {
      state.retryQueue.push({
        id: errorId,
        fn,
        error
      })
    }
  }

  /**
   * Process retry queue
   */
  const processRetryQueue = async () => {
    if (!state.isOnline) return

    const queue = [...state.retryQueue]
    state.retryQueue = []

    for (const item of queue) {
      try {
        await item.fn()
        // Remove the error if retry was successful
        const index = state.errors.findIndex(e => e.id === item.id)
        if (index !== -1) {
          state.errors.splice(index, 1)
        }
      } catch (error) {
        // Increment retry count
        item.error.retryCount = (item.error.retryCount || 0) + 1

        // Re-add to queue if retries remaining
        if (item.error.retryCount < (item.error.maxRetries || 0)) {
          state.retryQueue.push(item)
        } else {
          // Max retries reached, show final error
          handleError(error, { originalErrorId: item.id }, {
            userMessage: `Failed after ${item.error.maxRetries} attempts. ${item.error.userMessage}`
          })
        }
      }
    }
  }

  /**
   * Clear an error
   */
  const clearError = (errorId: string) => {
    const index = state.errors.findIndex(e => e.id === errorId)
    if (index !== -1) {
      state.errors.splice(index, 1)
    }
  }

  /**
   * Clear all errors
   */
  const clearAllErrors = () => {
    state.errors = []
    state.retryQueue = []
  }

  /**
   * Get errors by category
   */
  const getErrorsByCategory = (category: ErrorCategory): AppError[] => {
    return state.errors.filter(error => error.category === category)
  }

  /**
   * Get errors by severity
   */
  const getErrorsBySeverity = (severity: ErrorSeverity): AppError[] => {
    return state.errors.filter(error => error.severity === severity)
  }

  /**
   * Get recent errors (last 10)
   */
  const getRecentErrors = (): AppError[] => {
    return [...state.errors].sort((a, b) => b.timestamp.getTime() - a.timestamp.getTime()).slice(0, 10)
  }

  /**
   * Handle network status changes
   */
  const handleOnlineStatusChange = () => {
    state.isOnline = navigator.onLine
    if (state.isOnline) {
      processRetryQueue()
    }
  }

  // Set up network status listeners
  if (typeof window !== 'undefined') {
    window.addEventListener('online', handleOnlineStatusChange)
    window.addEventListener('offline', handleOnlineStatusChange)
  }

  return {
    // State
    errors: () => state.errors,
    isOnline: () => state.isOnline,
    retryQueue: () => state.retryQueue,
    errorHistory: () => errorHistory.value,

    // Actions
    handleError,
    clearError,
    clearAllErrors,
    addToRetryQueue,
    processRetryQueue,

    // Queries
    getErrorsByCategory,
    getErrorsBySeverity,
    getRecentErrors
  }
}

// Create singleton instance
export const errorHandler = useErrorHandler()

// Global error handlers
if (typeof window !== 'undefined') {
  // Handle unhandled promise rejections
  window.addEventListener('unhandledrejection', (event) => {
    errorHandler.handleError(event.reason, {
      type: 'unhandledPromiseRejection'
    }, {
      severity: 'high'
    })
  })

  // Handle uncaught errors
  window.addEventListener('error', (event) => {
    errorHandler.handleError(event.error || event.message, {
      type: 'uncaughtError',
      filename: event.filename,
      lineno: event.lineno,
      colno: event.colno
    }, {
      severity: 'critical'
    })
  })
}
