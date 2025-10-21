import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import { factories as factoriesApi } from '@/api/endpoints'
import type { FactoryResponse, CreateFactoryRequest, UpdateFactoryRequest } from '@/api/types'
import { handleApiError } from '@/api'

/**
 * Factory Store
 *
 * Manages factory data including CRUD operations and current factory selection.
 * Integrates with the factory API endpoints for all data operations.
 */
export const useFactoryStore = defineStore('factory', () => {
  // State
  const factoriesList = ref<FactoryResponse[]>([])
  const currentFactoryId = ref<number | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const currentFactory = computed(() => {
    if (!currentFactoryId.value) return null
    return factoriesList.value.find(f => f.id === currentFactoryId.value) || null
  })

  const factoriesById = computed(() => {
    const result: Record<number, FactoryResponse> = {}
    factoriesList.value.forEach(factory => {
      result[factory.id] = factory
    })
    return result
  })

  const factoryNames = computed(() => {
    return factoriesList.value.map(f => ({ id: f.id, name: f.name }))
  })

  // Actions

  /**
   * Fetch all factories from the API
   */
  const fetchAll = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      const data = await factoriesApi.getAll()
      factoriesList.value = data
    } catch (err) {
      error.value = handleApiError(err)
      console.error('Failed to fetch factories:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * Fetch a specific factory by ID
   * @param id - The factory ID to fetch
   */
  const fetchById = async (id: number): Promise<FactoryResponse | null> => {
    loading.value = true
    error.value = null

    try {
      const factory = await factoriesApi.getById(id)

      // Update factory in the array if it exists
      const index = factoriesList.value.findIndex(f => f.id === id)
      if (index !== -1) {
        factoriesList.value[index] = factory
      } else {
        factoriesList.value.push(factory)
      }

      return factory
    } catch (err) {
      error.value = handleApiError(err)
      console.error(`Failed to fetch factory ${id}:`, err)
      return null
    } finally {
      loading.value = false
    }
  }

  /**
   * Create a new factory
   * @param factoryData - The factory creation data
   * @returns The created factory or null if creation failed
   */
  const create = async (factoryData: CreateFactoryRequest): Promise<FactoryResponse | null> => {
    loading.value = true
    error.value = null

    try {
      const newFactory = await factoriesApi.create(factoryData)
      factoriesList.value.push(newFactory)
      return newFactory
    } catch (err) {
      error.value = handleApiError(err)
      console.error('Failed to create factory:', err)
      return null
    } finally {
      loading.value = false
    }
  }

  /**
   * Update an existing factory
   * @param id - The factory ID to update
   * @param factoryData - The factory update data
   * @returns The updated factory or null if update failed
   */
  const update = async (id: number, factoryData: UpdateFactoryRequest): Promise<FactoryResponse | null> => {
    loading.value = true
    error.value = null

    try {
      const updatedFactory = await factoriesApi.update(id, factoryData)

      // Update factory in the array
      const index = factoriesList.value.findIndex(f => f.id === id)
      if (index !== -1) {
        factoriesList.value[index] = updatedFactory
      }

      return updatedFactory
    } catch (err) {
      error.value = handleApiError(err)
      console.error(`Failed to update factory ${id}:`, err)
      return null
    } finally {
      loading.value = false
    }
  }

  /**
   * Delete a factory
   * @param id - The factory ID to delete
   * @returns True if deletion was successful, false otherwise
   */
  const deleteFactory = async (id: number): Promise<boolean> => {
    loading.value = true
    error.value = null

    try {
      await factoriesApi.delete(id)

      // Remove factory from the array
      const index = factoriesList.value.findIndex(f => f.id === id)
      if (index !== -1) {
        factoriesList.value.splice(index, 1)
      }

      // Clear current factory if it was the deleted one
      if (currentFactoryId.value === id) {
        currentFactoryId.value = null
      }

      return true
    } catch (err) {
      error.value = handleApiError(err)
      console.error(`Failed to delete factory ${id}:`, err)
      return false
    } finally {
      loading.value = false
    }
  }

  /**
   * Set the current factory by ID
   * @param id - The factory ID to set as current, or null to clear
   */
  const setCurrentFactory = (id: number | null): void => {
    currentFactoryId.value = id
  }

  /**
   * Clear the error state
   */
  const clearError = (): void => {
    error.value = null
  }

  /**
   * Reset the store to its initial state
   */
  const reset = (): void => {
    factoriesList.value = []
    currentFactoryId.value = null
    loading.value = false
    error.value = null
  }

  return {
    // State
    factories: factoriesList,
    currentFactoryId,
    loading,
    error,

    // Getters
    currentFactory,
    factoriesById,
    factoryNames,

    // Actions
    fetchAll,
    fetchById,
    create,
    update,
    deleteFactory,
    setCurrentFactory,
    clearError,
    reset
  }
})
