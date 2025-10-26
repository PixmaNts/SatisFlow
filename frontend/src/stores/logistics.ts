import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import { logistics as logisticsApi } from '@/api/endpoints'
import type { LogisticsResponse, CreateLogisticsRequest, UpdateLogisticsRequest } from '@/api/types'
import { handleApiError } from '@/api'

/**
 * Logistics Store
 *
 * Manages logistics line data including CRUD operations.
 * Integrates with the logistics API endpoints for all data operations.
 */
export const useLogisticsStore = defineStore('logistics', () => {
  // State
  const logistics = ref<LogisticsResponse[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const logisticsById = computed(() => {
    const result: Record<string, LogisticsResponse> = {}
    logistics.value.forEach(line => {
      result[line.id] = line
    })
    return result
  })

  const logisticsByFactory = computed(() => {
    const result: Record<string, LogisticsResponse[]> = {}

    // Group logistics lines by factory
    logistics.value.forEach(line => {
      // Initialize arrays if they don't exist
      if (!result[line.from_factory]) {
        result[line.from_factory] = []
      }
      if (!result[line.to_factory]) {
        result[line.to_factory] = []
      }

      // Add to source factory
      result[line.from_factory]!.push(line)
      // Note: We don't add to to_factory here to avoid duplicates
      // Components can filter by from_factory or to_factory as needed
    })

    return result
  })

  const logisticsByTransportType = computed(() => {
    const result: Record<string, LogisticsResponse[]> = {}

    logistics.value.forEach(line => {
      const type = line.transport_type
      if (!result[type]) {
        result[type] = []
      }
      result[type].push(line)
    })

    return result
  })

  const uniqueFactoryIds = computed(() => {
    const factoryIds = new Set<string>()
    logistics.value.forEach(line => {
      factoryIds.add(line.from_factory)
      factoryIds.add(line.to_factory)
    })
    return Array.from(factoryIds)
  })

  const upsertLogistics = (line: LogisticsResponse): void => {
    const index = logistics.value.findIndex(l => l.id === line.id)
    if (index !== -1) {
      logistics.value.splice(index, 1, line)
    } else {
      logistics.value.push(line)
    }
  }

  // Actions

  /**
   * Fetch all logistics lines from the API
   */
  const fetchAll = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      const data = await logisticsApi.getAll()
      logistics.value = data
    } catch (err) {
      error.value = handleApiError(err)
      console.error('Failed to fetch logistics lines:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * Fetch a specific logistics line by ID
   * @param id - The logistics line ID to fetch
   */
  const fetchById = async (id: string): Promise<LogisticsResponse | null> => {
    loading.value = true
    error.value = null

    try {
      const line = await logisticsApi.getById(id)

      upsertLogistics(line)

      return line
    } catch (err) {
      error.value = handleApiError(err)
      console.error(`Failed to fetch logistics line ${id}:`, err)
      return null
    } finally {
      loading.value = false
    }
  }

  /**
   * Create a new logistics line
   * @param logisticsData - The logistics line creation data
   * @returns The created logistics line or null if creation failed
   */
  const create = async (logisticsData: CreateLogisticsRequest): Promise<LogisticsResponse | null> => {
    loading.value = true
    error.value = null

    try {
      const newLine = await logisticsApi.create(logisticsData)
      upsertLogistics(newLine)
      return newLine
    } catch (err) {
      error.value = handleApiError(err)
      console.error('Failed to create logistics line:', err)
      return null
    } finally {
      loading.value = false
    }
  }

  /**
   * Update an existing logistics line
   * @param id - The logistics line ID
   * @param logisticsData - The update payload
   * @returns The updated logistics line or null if update failed
   */
  const update = async (
    id: string,
    logisticsData: UpdateLogisticsRequest
  ): Promise<LogisticsResponse | null> => {
    loading.value = true
    error.value = null

    try {
      const updatedLine = await logisticsApi.update(id, logisticsData)
      upsertLogistics(updatedLine)
      return updatedLine
    } catch (err) {
      error.value = handleApiError(err)
      console.error(`Failed to update logistics line ${id}:`, err)
      return null
    } finally {
      loading.value = false
    }
  }

  /**
   * Delete a logistics line
   * @param id - The logistics line ID to delete
   * @returns True if deletion was successful, false otherwise
   */
  const deleteLogistics = async (id: string): Promise<boolean> => {
    loading.value = true
    error.value = null

    try {
      await logisticsApi.delete(id)

      // Remove logistics line from the array
      const index = logistics.value.findIndex(l => l.id === id)
      if (index !== -1) {
        logistics.value.splice(index, 1)
      }

      return true
    } catch (err) {
      error.value = handleApiError(err)
      console.error(`Failed to delete logistics line ${id}:`, err)
      return false
    } finally {
      loading.value = false
    }
  }

  /**
   * Get logistics lines for a specific factory (as source or destination)
   * @param factoryId - The factory ID to get logistics for
   * @param direction - 'from', 'to', or 'both' to filter by direction
   * @returns Array of logistics lines
   */
  const getLogisticsForFactory = (
    factoryId: string,
    direction: 'from' | 'to' | 'both' = 'both'
  ): LogisticsResponse[] => {
    return logistics.value.filter(line => {
      if (direction === 'from') return line.from_factory === factoryId
      if (direction === 'to') return line.to_factory === factoryId
      return line.from_factory === factoryId || line.to_factory === factoryId
    })
  }

  /**
   * Get logistics lines between two specific factories
   * @param fromFactoryId - The source factory ID
   * @param toFactoryId - The destination factory ID
   * @returns Array of logistics lines between the two factories
   */
  const getLogisticsBetweenFactories = (
    fromFactoryId: string,
    toFactoryId: string
  ): LogisticsResponse[] => {
    return logistics.value.filter(
      line => line.from_factory === fromFactoryId && line.to_factory === toFactoryId
    )
  }

  /**
   * Filter logistics lines by transport type
   * @param transportType - The transport type to filter by
   * @returns Array of logistics lines with the specified transport type
   */
  const getLogisticsByTransportType = (transportType: string): LogisticsResponse[] => {
    return logistics.value.filter(line => line.transport_type === transportType)
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
    logistics.value = []
    loading.value = false
    error.value = null
  }

  return {
    // State
    logistics,
    loading,
    error,

    // Getters
    logisticsById,
    logisticsByFactory,
    logisticsByTransportType,
    uniqueFactoryIds,

    // Actions
    fetchAll,
    fetchById,
    create,
    update,
    deleteLogistics,
    getLogisticsForFactory,
    getLogisticsBetweenFactories,
    getLogisticsByTransportType,
    clearError,
    reset
  }
})
