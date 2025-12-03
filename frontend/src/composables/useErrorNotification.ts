import { AxiosError } from 'axios'
import type { ErrorResponse } from '@/api/types'

/**
 * Normalized error structure
 */
export interface NormalizedError {
  message: string
  code?: string
  fieldErrors?: Record<string, string[]>
  statusCode?: number
  retryable?: boolean
}

/**
 * Error notification options
 */
export interface ErrorNotificationOptions {
  silent?: boolean
  showToast?: boolean
  inline?: boolean
  retryCallback?: () => Promise<unknown>
}

/**
 * Centralized Error Notification Composable
 * 
 * Provides a unified way to handle errors across the application:
 * - Normalizes API errors into a consistent shape
 * - Integrates with ToastContainer for user notifications
 * - Supports inline error hints for forms
 * - Provides retry mechanisms for transient failures
 */
export function useErrorNotification() {
  /**
   * Normalize an error from various sources (Axios, Error, string, etc.)
   */
  const normalizeError = (error: unknown): NormalizedError => {
    // Handle Axios errors
    if (error instanceof AxiosError) {
      const response = error.response
      const data = response?.data as ErrorResponse | undefined

      // Extract field errors if present
      const fieldErrors: Record<string, string[]> | undefined = data?.fieldErrors
        ? Object.entries(data.fieldErrors).reduce((acc, [key, value]) => {
            acc[key] = Array.isArray(value) ? value : [value]
            return acc
          }, {} as Record<string, string[]>)
        : undefined

      // Determine if error is retryable
      const retryable = 
        !response || // Network error
        response.status === 429 || // Rate limit
        response.status === 502 || // Bad gateway
        response.status === 503 || // Service unavailable
        response.status === 504    // Gateway timeout

      return {
        message: data?.error || error.message || 'An unexpected error occurred',
        code: data?.code || `HTTP_${response?.status || 'NETWORK_ERROR'}`,
        fieldErrors,
        statusCode: response?.status,
        retryable
      }
    }

    // Handle standard Error objects
    if (error instanceof Error) {
      return {
        message: error.message,
        code: error.name,
        retryable: false
      }
    }

    // Handle string errors
    if (typeof error === 'string') {
      return {
        message: error,
        retryable: false
      }
    }

    // Fallback for unknown error types
    return {
      message: 'An unexpected error occurred',
      code: 'UNKNOWN_ERROR',
      retryable: false
    }
  }

  /**
   * Show error notification via toast
   */
  const showErrorToast = (error: NormalizedError, options?: ErrorNotificationOptions) => {
    if (options?.silent || options?.showToast === false) {
      return
    }

    // Dispatch event to ToastContainer
    window.dispatchEvent(new CustomEvent('app-error', {
      detail: {
        id: `error_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        message: error.message,
        severity: error.statusCode && error.statusCode >= 500 ? 'critical' : 
                 error.statusCode && error.statusCode >= 400 ? 'high' : 'medium',
        category: error.statusCode ? 'server' : 'network',
        retryable: error.retryable && !!options?.retryCallback,
        retryCallback: options?.retryCallback
      }
    }))
  }

  /**
   * Show success notification via toast
   */
  const showSuccessToast = (message: string, duration?: number) => {
    window.dispatchEvent(new CustomEvent('app-toast', {
      detail: {
        type: 'success',
        title: message,
        duration
      }
    }))
  }

  /**
   * Show info notification via toast
   */
  const showInfoToast = (message: string, duration?: number) => {
    window.dispatchEvent(new CustomEvent('app-toast', {
      detail: {
        type: 'info',
        title: message,
        duration
      }
    }))
  }

  /**
   * Show warning notification via toast
   */
  const showWarningToast = (message: string, duration?: number) => {
    window.dispatchEvent(new CustomEvent('app-toast', {
      detail: {
        type: 'warning',
        title: message,
        duration
      }
    }))
  }

  /**
   * Handle an error with full notification pipeline
   */
  const handleError = (
    error: unknown,
    options?: ErrorNotificationOptions
  ): NormalizedError => {
    const normalized = normalizeError(error)
    
    // Show toast notification unless silent
    showErrorToast(normalized, options)

    // Log to console in development
    if (import.meta.env.DEV) {
      console.error('Error handled:', {
        message: normalized.message,
        code: normalized.code,
        statusCode: normalized.statusCode,
        fieldErrors: normalized.fieldErrors,
        originalError: error
      })
    }

    return normalized
  }

  /**
   * Handle API error with retry support
   */
  const handleApiError = async (
    error: unknown,
    retryFn?: () => Promise<unknown>,
    options?: ErrorNotificationOptions
  ): Promise<NormalizedError> => {
    const normalized = normalizeError(error)
    
    // If retryable and retry function provided, include retry callback
    const retryCallback = normalized.retryable && retryFn
      ? async () => {
          try {
            await retryFn()
            showSuccessToast('Operation retried successfully')
          } catch (retryError) {
            handleError(retryError, { ...options, showToast: true })
          }
        }
      : undefined

    handleError(error, {
      ...options,
      retryCallback
    })

    return normalized
  }

  /**
   * Get field errors for a specific field
   */
  const getFieldError = (error: NormalizedError, fieldName: string): string | undefined => {
    return error.fieldErrors?.[fieldName]?.[0]
  }

  /**
   * Get all field errors
   */
  const getFieldErrors = (error: NormalizedError): Record<string, string> | undefined => {
    if (!error.fieldErrors) return undefined
    
    return Object.entries(error.fieldErrors).reduce((acc, [key, values]) => {
      acc[key] = values[0] // Take first error message per field
      return acc
    }, {} as Record<string, string>)
  }

  return {
    normalizeError,
    handleError,
    handleApiError,
    showErrorToast,
    showSuccessToast,
    showInfoToast,
    showWarningToast,
    getFieldError,
    getFieldErrors
  }
}

// Export singleton instance for global use
export const errorNotification = useErrorNotification()

