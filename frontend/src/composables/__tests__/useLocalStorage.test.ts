import { describe, it, expect, beforeEach, afterEach } from 'vitest'
import { useLocalStorage } from '../useLocalStorage'

describe('useLocalStorage', () => {
  const testKey = 'test-key'
  const defaultValue = { name: 'test', value: 42 }

  beforeEach(() => {
    // Clear localStorage before each test
    localStorage.clear()
  })

  afterEach(() => {
    // Clean up after each test
    localStorage.clear()
  })

  it('should return default value when no value is stored', () => {
    const { value } = useLocalStorage(testKey, { defaultValue })

    expect(value.value).toEqual(defaultValue)
  })

  it('should save and retrieve values from localStorage', () => {
    const { value } = useLocalStorage(testKey, { defaultValue })

    // Update the value
    value.value = { name: 'updated', value: 100 }

    // Check that it's saved to localStorage
    const stored = localStorage.getItem(testKey)
    expect(stored).toBe(JSON.stringify({ name: 'updated', value: 100 }))

    // Check that the reactive value is updated
    expect(value.value).toEqual({ name: 'updated', value: 100 })
  })

  it('should load existing values from localStorage', () => {
    // Set a value in localStorage first
    const existingValue = { name: 'existing', value: 999 }
    localStorage.setItem(testKey, JSON.stringify(existingValue))

    // Create the composable
    const { value } = useLocalStorage(testKey, { defaultValue })

    // Check that it loads the existing value
    expect(value.value).toEqual(existingValue)
  })

  it('should handle invalid JSON gracefully', () => {
    // Set invalid JSON in localStorage
    localStorage.setItem(testKey, 'invalid-json')

    // Create the composable
    const { value } = useLocalStorage(testKey, { defaultValue })

    // Check that it falls back to the default value
    expect(value.value).toEqual(defaultValue)
  })

  it('should provide error handling', () => {
    // Mock localStorage to throw an error
    const originalSetItem = localStorage.setItem
    localStorage.setItem = () => {
      throw new Error('Storage quota exceeded')
    }

    const { value, error } = useLocalStorage(testKey, { defaultValue })

    // Try to set a value
    value.value = { name: 'error-test', value: 1 }

    // Check that an error is set
    expect(error.value).toBeInstanceOf(Error)

    // Restore localStorage
    localStorage.setItem = originalSetItem
  })

  it('should provide exists property', () => {
    const { exists } = useLocalStorage(testKey, { defaultValue })

    // Should not exist initially
    expect(exists.value).toBe(false)

    // Set a value
    localStorage.setItem(testKey, JSON.stringify(defaultValue))

    // Create a new composable to check exists
    const { exists: exists2 } = useLocalStorage(testKey, { defaultValue })

    // Should exist now
    expect(exists2.value).toBe(true)
  })

  it('should provide remove method', () => {
    const { value, remove, exists } = useLocalStorage(testKey, { defaultValue })

    // Set a value
    value.value = { name: 'to-remove', value: 1 }

    // Check that it exists
    expect(exists.value).toBe(true)

    // Remove it
    remove()

    // Check that it's gone
    expect(exists.value).toBe(false)
    expect(value.value).toEqual(defaultValue)
  })

  it('should work with custom serializer', () => {
    const customSerializer = {
      read: (value: string) => {
        const parsed = JSON.parse(value)
        return { ...parsed, custom: true }
      },
      write: (value: unknown) => {
        return JSON.stringify({ ...(value as Record<string, unknown>), serialized: true })
      }
    }

    const { value } = useLocalStorage(testKey, {
      defaultValue,
      serializer: customSerializer
    })

    // Update the value
    value.value = { name: 'custom', value: 1 }

    // Check the stored value
    const stored = localStorage.getItem(testKey)
    expect(stored).toBe(JSON.stringify({ name: 'custom', value: 1, serialized: true }))

    // Create a new composable to test reading
    const { value: value2 } = useLocalStorage(testKey, {
      defaultValue,
      serializer: customSerializer
    })

    expect(value2.value).toEqual({ name: 'custom', value: 1, serialized: true, custom: true })
  })
})
