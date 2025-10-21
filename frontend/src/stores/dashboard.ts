import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import { dashboard as dashboardApi } from '@/api/endpoints'
import type { DashboardSummary, ItemBalance, PowerStats } from '@/api/types'
import { handleApiError } from '@/api'

/**
 * Dashboard Store
 *
 * Manages dashboard data including summary statistics, item balances, and power statistics.
 * Integrates with the dashboard API endpoints for all data operations.
 */
export const useDashboardStore = defineStore('dashboard', () => {
  // State
  const summary = ref<DashboardSummary | null>(null)
  const itemBalances = ref<ItemBalance[]>([])
  const powerStats = ref<PowerStats | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const netPower = computed(() => {
    return powerStats.value?.power_balance || 0
  })

  const totalFactories = computed(() => {
    return summary.value?.total_factories || 0
  })

  const totalProductionLines = computed(() => {
    return summary.value?.total_production_lines || 0
  })

  const totalLogisticsLines = computed(() => {
    return summary.value?.total_logistics_lines || 0
  })

  const hasPowerSurplus = computed(() => {
    return powerStats.value?.has_surplus || false
  })

  const hasPowerDeficit = computed(() => {
    return powerStats.value?.has_deficit || false
  })

  const isPowerBalanced = computed(() => {
    return powerStats.value?.is_balanced || false
  })

  const filteredItemBalances = computed(() => {
    return (filters: {
      state?: 'overflow' | 'underflow' | 'balanced'
      searchText?: string
    }) => {
      let filtered = itemBalances.value

      // Filter by state
      if (filters.state) {
        filtered = filtered.filter(item => item.state === filters.state)
      }

      // Filter by search text
      if (filters.searchText) {
        const searchLower = filters.searchText.toLowerCase()
        filtered = filtered.filter(item =>
          item.item.toLowerCase().includes(searchLower)
        )
      }

      return filtered
    }
  })

  const overflowItems = computed(() => {
    return itemBalances.value.filter(item => item.state === 'overflow')
  })

  const underflowItems = computed(() => {
    return itemBalances.value.filter(item => item.state === 'underflow')
  })

  const balancedItems = computed(() => {
    return itemBalances.value.filter(item => item.state === 'balanced')
  })

  const topOverflowItems = computed(() => {
    return [...overflowItems.value]
      .sort((a, b) => b.balance - a.balance)
      .slice(0, 10)
  })

  const topUnderflowItems = computed(() => {
    return [...underflowItems.value]
      .sort((a, b) => a.balance - b.balance)
      .slice(0, 10)
  })

  // Actions

  /**
   * Fetch dashboard summary statistics
   */
  const fetchSummary = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      const data = await dashboardApi.getSummary()
      summary.value = data
    } catch (err) {
      error.value = handleApiError(err)
      console.error('Failed to fetch dashboard summary:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * Fetch item balance data across all factories
   */
  const fetchItemBalances = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      const data = await dashboardApi.getItemBalances()
      itemBalances.value = data
    } catch (err) {
      error.value = handleApiError(err)
      console.error('Failed to fetch item balances:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * Fetch power statistics across all factories
   */
  const fetchPowerStats = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      const data = await dashboardApi.getPowerStats()
      powerStats.value = data
    } catch (err) {
      error.value = handleApiError(err)
      console.error('Failed to fetch power statistics:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * Fetch all dashboard data at once
   */
  const fetchAllData = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      await Promise.all([
        fetchSummary(),
        fetchItemBalances(),
        fetchPowerStats()
      ])
    } catch (err) {
      error.value = handleApiError(err)
      console.error('Failed to fetch dashboard data:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * Refresh dashboard data
   */
  const refresh = async (): Promise<void> => {
    await fetchAllData()
  }

  /**
   * Get item balance for a specific item
   * @param item - The item to get balance for
   * @returns The item balance or null if not found
   */
  const getItemBalance = (item: string): ItemBalance | null => {
    return itemBalances.value.find(balance => balance.item === item) || null
  }

  /**
   * Get factory power statistics
   * @param factoryId - The factory ID to get power stats for
   * @returns The factory power stats or null if not found
   */
  const getFactoryPowerStats = (factoryId: number) => {
    return powerStats.value?.factory_stats.find(stats => stats.factory_id === factoryId) || null
  }

  /**
   * Get items with a specific balance state
   * @param state - The balance state to filter by
   * @returns Array of items with the specified state
   */
  const getItemsByState = (state: 'overflow' | 'underflow' | 'balanced'): ItemBalance[] => {
    return itemBalances.value.filter(item => item.state === state)
  }

  /**
   * Search items by name
   * @param searchText - The text to search for
   * @returns Array of items matching the search
   */
  const searchItems = (searchText: string): ItemBalance[] => {
    const searchLower = searchText.toLowerCase()
    return itemBalances.value.filter(item =>
      item.item.toLowerCase().includes(searchLower)
    )
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
    summary.value = null
    itemBalances.value = []
    powerStats.value = null
    loading.value = false
    error.value = null
  }

  return {
    // State
    summary,
    itemBalances,
    powerStats,
    loading,
    error,

    // Getters
    netPower,
    totalFactories,
    totalProductionLines,
    totalLogisticsLines,
    hasPowerSurplus,
    hasPowerDeficit,
    isPowerBalanced,
    filteredItemBalances,
    overflowItems,
    underflowItems,
    balancedItems,
    topOverflowItems,
    topUnderflowItems,

    // Actions
    fetchSummary,
    fetchItemBalances,
    fetchPowerStats,
    fetchAllData,
    refresh,
    getItemBalance,
    getFactoryPowerStats,
    getItemsByState,
    searchItems,
    clearError,
    reset
  }
})
