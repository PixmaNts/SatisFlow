import { describe, it, expect, beforeEach, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useDashboardStore } from '../dashboard'
import type { DashboardSummary, ItemBalance, PowerStats, Item } from '@/api/types'

// Mock the API client
vi.mock('@/api/endpoints', () => ({
  dashboard: {
    getSummary: vi.fn(),
    getItemBalances: vi.fn(),
    getPowerStats: vi.fn(),
  },
}))

import { dashboard as dashboardApi } from '@/api/endpoints'

describe('Dashboard Store', () => {
  let dashboardStore: ReturnType<typeof useDashboardStore>
  const mockSummary: DashboardSummary = {
    total_factories: 5,
    total_production_lines: 12,
    total_logistics_lines: 8,
    total_power_consumption: 250.5,
    total_power_generation: 300.0,
    net_power: 49.5,
  }

  const mockItemBalances: ItemBalance[] = [
    { item: 'IronPlate' as Item, balance: 30, state: 'overflow' },
    { item: 'Wire' as Item, balance: -20, state: 'underflow' },
  ]

  const mockPowerStats: PowerStats = {
    total_generation: 300.0,
    total_consumption: 250.5,
    power_balance: 49.5,
    has_surplus: true,
    has_deficit: false,
    is_balanced: false,
    factory_stats: [
      {
        factory_id: 1,
        factory_name: 'Iron Processing',
        generation: 150.0,
        consumption: 120.0,
        balance: 30.0,
        generator_count: 2,
        generator_types: ['Coal', 'Biomass'],
      },
    ],
  }

  beforeEach(() => {
    // Create fresh Pinia instance
    const pinia = createPinia()
    setActivePinia(pinia)

    // Create store instance
    dashboardStore = useDashboardStore()

    // Clear all mocks
    vi.clearAllMocks()
  })

  describe('Initial state', () => {
    it('should have correct initial state', () => {
      expect(dashboardStore.summary).toBeNull()
      expect(dashboardStore.itemBalances).toEqual([])
      expect(dashboardStore.powerStats).toBeNull()
      expect(dashboardStore.loading).toBe(false)
      expect(dashboardStore.error).toBe(null)
    })
  })

  describe('Fetching dashboard data', () => {
    it('should fetch summary data successfully', async () => {
      vi.mocked(dashboardApi.getSummary).mockResolvedValue(mockSummary)

      await dashboardStore.fetchSummary()

      expect(dashboardApi.getSummary).toHaveBeenCalled()
      expect(dashboardStore.summary).toEqual(mockSummary)
      expect(dashboardStore.loading).toBe(false)
      expect(dashboardStore.error).toBe(null)
    })

    it('should fetch item balances successfully', async () => {
      vi.mocked(dashboardApi.getItemBalances).mockResolvedValue(mockItemBalances)

      await dashboardStore.fetchItemBalances()

      expect(dashboardApi.getItemBalances).toHaveBeenCalled()
      expect(dashboardStore.itemBalances).toEqual(mockItemBalances)
      expect(dashboardStore.loading).toBe(false)
      expect(dashboardStore.error).toBe(null)
    })

    it('should fetch power stats successfully', async () => {
      vi.mocked(dashboardApi.getPowerStats).mockResolvedValue(mockPowerStats)

      await dashboardStore.fetchPowerStats()

      expect(dashboardApi.getPowerStats).toHaveBeenCalled()
      expect(dashboardStore.powerStats).toEqual(mockPowerStats)
      expect(dashboardStore.loading).toBe(false)
      expect(dashboardStore.error).toBe(null)
    })

    it('should fetch all dashboard data', async () => {
      vi.mocked(dashboardApi.getSummary).mockResolvedValue(mockSummary)
      vi.mocked(dashboardApi.getItemBalances).mockResolvedValue(mockItemBalances)
      vi.mocked(dashboardApi.getPowerStats).mockResolvedValue(mockPowerStats)

      await dashboardStore.fetchAllData()

      expect(dashboardApi.getSummary).toHaveBeenCalled()
      expect(dashboardApi.getItemBalances).toHaveBeenCalled()
      expect(dashboardApi.getPowerStats).toHaveBeenCalled()
      expect(dashboardStore.summary).toEqual(mockSummary)
      expect(dashboardStore.itemBalances).toEqual(mockItemBalances)
      expect(dashboardStore.powerStats).toEqual(mockPowerStats)
      expect(dashboardStore.loading).toBe(false)
      expect(dashboardStore.error).toBe(null)
    })
  })

  describe('Error handling', () => {
    it('should handle API errors when fetching summary', async () => {
      const error = new Error('Failed to fetch summary')
      vi.mocked(dashboardApi.getSummary).mockRejectedValue(error)

      await dashboardStore.fetchSummary()

      expect(dashboardStore.summary).toBeNull()
      expect(dashboardStore.loading).toBe(false)
      expect(dashboardStore.error).toContain('Failed to fetch summary')
    })
  })

  describe('Loading state', () => {
    it('should set loading state during fetch', async () => {
      vi.mocked(dashboardApi.getSummary).mockImplementation(() => new Promise(resolve => setTimeout(() => resolve(mockSummary), 100)))

      const fetchPromise = dashboardStore.fetchSummary()

      expect(dashboardStore.loading).toBe(true)

      await fetchPromise

      expect(dashboardStore.loading).toBe(false)
    })
  })

  describe('Computed properties', () => {
    it('should compute totalFactories correctly', () => {
      expect(dashboardStore.totalFactories).toBe(0)

      dashboardStore.summary = mockSummary

      expect(dashboardStore.totalFactories).toBe(5)
    })

    it('should compute totalProductionLines correctly', () => {
      expect(dashboardStore.totalProductionLines).toBe(0)

      dashboardStore.summary = mockSummary

      expect(dashboardStore.totalProductionLines).toBe(12)
    })

    it('should compute totalLogisticsLines correctly', () => {
      expect(dashboardStore.totalLogisticsLines).toBe(0)

      dashboardStore.summary = mockSummary

      expect(dashboardStore.totalLogisticsLines).toBe(8)
    })

    it('should compute netPower correctly', () => {
      expect(dashboardStore.netPower).toBe(0)

      dashboardStore.powerStats = mockPowerStats

      expect(dashboardStore.netPower).toBe(49.5)
    })

    it('should compute hasPowerSurplus correctly', () => {
      expect(dashboardStore.hasPowerSurplus).toBe(false)

      dashboardStore.powerStats = mockPowerStats

      expect(dashboardStore.hasPowerSurplus).toBe(true)
    })

    it('should compute hasPowerDeficit correctly', () => {
      expect(dashboardStore.hasPowerDeficit).toBe(false)

      const deficitPowerStats = { ...mockPowerStats, has_surplus: false, has_deficit: true }
      dashboardStore.powerStats = deficitPowerStats

      expect(dashboardStore.hasPowerDeficit).toBe(true)
    })

    it('should compute isPowerBalanced correctly', () => {
      expect(dashboardStore.isPowerBalanced).toBe(false)

      const balancedPowerStats = { ...mockPowerStats, has_surplus: false, has_deficit: false, is_balanced: true }
      dashboardStore.powerStats = balancedPowerStats

      expect(dashboardStore.isPowerBalanced).toBe(true)
    })
  })

  describe('Actions', () => {
    it('should clear error', () => {
      dashboardStore.error = 'Test error'

      dashboardStore.clearError()

      expect(dashboardStore.error).toBe(null)
    })

    it('should reset store', () => {
      dashboardStore.summary = mockSummary
      dashboardStore.itemBalances = mockItemBalances
      dashboardStore.powerStats = mockPowerStats
      dashboardStore.error = 'Test error'

      dashboardStore.reset()

      expect(dashboardStore.summary).toBeNull()
      expect(dashboardStore.itemBalances).toEqual([])
      expect(dashboardStore.powerStats).toBeNull()
      expect(dashboardStore.error).toBe(null)
      expect(dashboardStore.loading).toBe(false)
    })
  })
})
