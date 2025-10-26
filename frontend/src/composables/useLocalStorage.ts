import { ref, watch, onUnmounted, getCurrentInstance } from 'vue'

/**
 * Options for useLocalStorage composable
 */
interface UseLocalStorageOptions<T> {
  /**
   * Default value to use if no value is stored
   */
  defaultValue: T

  /**
   * Custom serializer (defaults to JSON.stringify/parse)
   */
  serializer?: {
    read: (value: string) => T
    write: (value: T) => string
  }

  /**
   * Whether to sync changes across tabs/windows
   * @default true
   */
  syncAcrossTabs?: boolean

  /**
   * Custom error handler
   */
  onError?: (error: Error) => void
}

/**
 * Reactive localStorage composable
 *
 * Provides a reactive reference that syncs with localStorage.
 * Handles serialization, errors, and cross-tab synchronization.
 *
 * @param key - localStorage key
 * @param options - Configuration options
 * @returns Reactive reference and utilities
 *
 * @example
 * ```ts
 * const { value, stored, remove, error } = useLocalStorage('user-theme', {
 *   defaultValue: 'light',
 *   syncAcrossTabs: true
 * })
 * ```
 */
export function useLocalStorage<T>(
  key: string,
  options: UseLocalStorageOptions<T>
) {
  const {
    defaultValue,
    serializer = {
      read: (v: string) => {
        try {
          return JSON.parse(v) as T
        } catch {
          return defaultValue
        }
      },
      write: (v: T) => JSON.stringify(v)
    },
    syncAcrossTabs = true,
    onError = (error) => console.error(`useLocalStorage error (${key}):`, error)
  } = options

  // Internal state
  const storedValue = ref<T>(defaultValue)
  const error = ref<Error | null>(null)
  const isAvailable = ref<boolean>(true)
  const exists = ref<boolean>(false)

  // Check if localStorage is available
  const checkLocalStorageAvailability = (): boolean => {
    try {
      const testKey = '__localStorage_test__'
      localStorage.setItem(testKey, 'test')
      localStorage.removeItem(testKey)
      return true
    } catch (err) {
      const availabilityError =
        err instanceof Error ? err : new Error(String(err))
      error.value = availabilityError
      exists.value = false
      onError(availabilityError)
      return false
    }
  }

  // Read from localStorage
  const read = (): T => {
    if (!isAvailable.value) {
      exists.value = false
      return defaultValue
    }

    try {
      const rawValue = localStorage.getItem(key)
      const hasValue = rawValue !== null
      exists.value = hasValue
      if (!hasValue) {
        return defaultValue
      }
      return serializer.read(rawValue)
    } catch (err) {
      error.value = err instanceof Error ? err : new Error(String(err))
      onError(error.value)
      return defaultValue
    }
  }

  // Write to localStorage
  const write = (value: T): void => {
    if (!isAvailable.value) {
      return
    }

    try {
      localStorage.setItem(key, serializer.write(value))
      exists.value = true
      error.value = null
    } catch (err) {
      error.value = err instanceof Error ? err : new Error(String(err))
      onError(error.value)
    }
  }

  // Remove from localStorage
  const remove = (): void => {
    if (!isAvailable.value) {
      return
    }

    try {
      localStorage.removeItem(key)
      // Temporarily stop the main watcher to prevent it from writing back the default value
      stopWatcher()
      storedValue.value = defaultValue
      exists.value = false
      error.value = null
      // Restart the watcher
      stopWatcher = setupWatcher()
    } catch (err) {
      error.value = err instanceof Error ? err : new Error(String(err))
      onError(error.value)
    }
  }

  // Initialize value
  const initialize = () => {
    isAvailable.value = checkLocalStorageAvailability()
    if (isAvailable.value) {
      storedValue.value = read()
    } else {
      exists.value = false
    }
  }

  // Handle storage events (cross-tab synchronization)
  let storageEventHandler: ((event: StorageEvent) => void) | null = null

  const setupStorageListener = () => {
    if (!syncAcrossTabs || !isAvailable.value) {
      return
    }

    storageEventHandler = (event: StorageEvent) => {
      if (event.key === key && event.newValue !== null) {
        try {
          const newValue = serializer.read(event.newValue)
          storedValue.value = newValue
          exists.value = true
        } catch (err) {
          error.value = err instanceof Error ? err : new Error(String(err))
          onError(error.value)
        }
      } else if (event.key === key && event.newValue === null) {
        storedValue.value = defaultValue
        exists.value = false
      }
    }

    window.addEventListener('storage', storageEventHandler)
  }

  const cleanupStorageListener = () => {
    if (storageEventHandler) {
      window.removeEventListener('storage', storageEventHandler)
      storageEventHandler = null
    }
  }

  // Watch for changes and sync to localStorage
  const setupWatcher = () => {
    return watch(
      storedValue,
      (newValue) => {
        if (isAvailable.value) {
          write(newValue)
        }
      },
      { deep: true }
    )
  }

  let stopWatcher = setupWatcher()

  // Initialize and setup listeners
  initialize()
  setupStorageListener()

  // Cleanup on unmount (only if in a Vue component context)
  if (typeof window !== 'undefined' && window.document) {
    try {
      // Check if we're in a Vue component context
      const instance = getCurrentInstance()
      if (instance) {
        onUnmounted(() => {
          cleanupStorageListener()
          stopWatcher()
        })
      }
    } catch {
      // Not in a Vue component context, skip lifecycle hooks
    }
  }

  return {
    /**
     * Reactive value that syncs with localStorage
     */
    value: storedValue,

    /**
     * Current error state
     */
    error,

    /**
     * Whether localStorage is available
     */
    isAvailable,

    /**
     * Whether the key exists in localStorage
     */
    exists,

    /**
     * Manually write a value to localStorage
     */
    write,

    /**
     * Remove the key from localStorage
     */
    remove,

    /**
     * Force re-read from localStorage
     */
    refresh: initialize
  }
}

/**
 * Simple localStorage composable for primitive values
 *
 * @param key - localStorage key
 * @param defaultValue - Default value
 * @returns Reactive reference
 */
export function useLocalStorageSimple<T>(
  key: string,
  defaultValue: T
) {
  const { value } = useLocalStorage(key, { defaultValue })
  return value
}

/**
 * Type-safe localStorage composable with validation
 *
 * @param key - localStorage key
 * @param options - Configuration options with validator
 * @returns Reactive reference and utilities
 */
export function useLocalStorageValidated<T>(
  key: string,
  options: UseLocalStorageOptions<T> & {
    validator: (value: unknown) => value is T
  }
) {
  const { validator, ...restOptions } = options

  const validatingSerializer = {
    read: (raw: string) => {
      try {
        const parsed = JSON.parse(raw)
        return validator(parsed) ? parsed : restOptions.defaultValue
      } catch {
        return restOptions.defaultValue
      }
    },
    write: restOptions.serializer?.write || JSON.stringify
  }

  return useLocalStorage(key, {
    ...restOptions,
    serializer: validatingSerializer
  })
}
