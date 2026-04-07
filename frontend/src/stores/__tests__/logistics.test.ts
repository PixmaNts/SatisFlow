import { describe, it, expect, beforeEach, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useLogisticsStore } from '../logistics'
import type { LogisticsResponse, CreateLogisticsRequest, Item } from '@/api/types'

vi.mock('@/api/endpoints', () => ({
  logistics: {
    getAll: vi.fn(),
    getById: vi.fn(),
    create: vi.fn(),
    update: vi.fn(),
    delete: vi.fn(),
  },
}))

vi.mock('@/api', () => ({
  handleApiError: vi.fn((e: unknown) => (e instanceof Error ? e.message : 'API Error')),
}))

import { logistics as logisticsApi } from '@/api/endpoints'

describe('Logistics Store', () => {
  let store: ReturnType<typeof useLogisticsStore>

  const mockItemFlow = (item: Item, qty: number) => ({
    item,
    quantity_per_min: qty,
  })

  const createMockLogistics = (
    id: string,
    fromFactory: string,
    toFactory: string,
    transportType: 'Bus' | 'Train' | 'Truck' | 'Drone' = 'Truck'
  ): LogisticsResponse => ({
    id,
    from_factory: fromFactory,
    to_factory: toFactory,
    transport_type: transportType,
    transport_id: `${transportType.toLowerCase()}-1`,
    transport_name: `${transportType} 1`,
    transport_details: `${transportType} Details`,
    items: [mockItemFlow('IronPlate' as Item, 60)],
    total_quantity_per_min: 60,
  })

  beforeEach(() => {
    const pinia = createPinia()
    setActivePinia(pinia)
    store = useLogisticsStore()
    vi.clearAllMocks()
  })

  describe('Initial state', () => {
    it('logistics is empty array', () => {
      expect(store.logistics).toEqual([])
    })

    it('loading is false', () => {
      expect(store.loading).toBe(false)
    })

    it('error is null', () => {
      expect(store.error).toBe(null)
    })
  })

  describe('Getters', () => {
    beforeEach(() => {
      store.logistics = [
        createMockLogistics('log-1', 'factory-a', 'factory-b', 'Truck'),
        createMockLogistics('log-2', 'factory-a', 'factory-c', 'Train'),
        createMockLogistics('log-3', 'factory-b', 'factory-c', 'Drone'),
      ]
    })

    it('logisticsById maps correctly with multiple items', () => {
      const byId = store.logisticsById

      expect(byId['log-1']).toEqual(store.logistics[0])
      expect(byId['log-2']).toEqual(store.logistics[1])
      expect(byId['log-3']).toEqual(store.logistics[2])
      expect(Object.keys(byId)).toHaveLength(3)
    })

    it('logisticsByFactory groups by from_factory ONLY', () => {
      const byFactory = store.logisticsByFactory

      // factory-a has 2 outbound logistics (log-1, log-2)
      expect(byFactory['factory-a']).toHaveLength(2)
      expect(byFactory['factory-a']).toContain(store.logistics[0])
      expect(byFactory['factory-a']).toContain(store.logistics[1])

      // factory-b has 1 outbound logistics (log-3)
      expect(byFactory['factory-b']).toHaveLength(1)
      expect(byFactory['factory-b']).toContain(store.logistics[2])

      // factory-c has 0 outbound logistics (it's only a destination)
      expect(byFactory['factory-c']).toHaveLength(0)

      // Verify log-3 appears in from_factory array but NOT in to_factory array
      expect(byFactory['factory-b']).toContain(store.logistics[2])
      expect(byFactory['factory-c']).not.toContain(store.logistics[2])
    })

    it('logisticsByTransportType groups correctly', () => {
      const byType = store.logisticsByTransportType

      expect(byType['Truck']).toHaveLength(1)
      expect(byType['Train']).toHaveLength(1)
      expect(byType['Drone']).toHaveLength(1)
      expect(byType['Bus']).toBeUndefined()
    })

    it('uniqueFactoryIds collects from BOTH from_factory AND to_factory', () => {
      const unique = store.uniqueFactoryIds

      expect(unique).toContain('factory-a')
      expect(unique).toContain('factory-b')
      expect(unique).toContain('factory-c')
      expect(unique).toHaveLength(3)
    })
  })

  describe('Fetch operations', () => {
    it('fetchAll replaces logistics on success', async () => {
      const mockData = [
        createMockLogistics('log-1', 'f1', 'f2', 'Truck'),
        createMockLogistics('log-2', 'f2', 'f3', 'Train'),
      ]
      vi.mocked(logisticsApi.getAll).mockResolvedValue(mockData)

      await store.fetchAll()

      expect(store.logistics).toEqual(mockData)
      expect(store.loading).toBe(false)
      expect(store.error).toBe(null)
    })

    it('fetchAll loading state toggles correctly', async () => {
      vi.mocked(logisticsApi.getAll).mockImplementation(
        () => new Promise(resolve => setTimeout(() => resolve([]), 50))
      )

      const fetchPromise = store.fetchAll()

      expect(store.loading).toBe(true)

      await fetchPromise

      expect(store.loading).toBe(false)
    })

    it('fetchAll error handling on API failure', async () => {
      const error = new Error('Network failure')
      vi.mocked(logisticsApi.getAll).mockRejectedValue(error)

      await store.fetchAll()

      expect(store.error).toBe('Network failure')
      expect(store.loading).toBe(false)
      expect(store.logistics).toEqual([])
    })
  })

  describe('CRUD + upsert', () => {
    const mockLogistics = createMockLogistics('log-1', 'factory-1', 'factory-2', 'Truck')

    beforeEach(() => {
      store.logistics = [mockLogistics]
    })

    it('fetchById appends new logistics line', async () => {
      const newLine = createMockLogistics('log-2', 'factory-2', 'factory-3', 'Train')
      vi.mocked(logisticsApi.getById).mockResolvedValue(newLine)

      const result = await store.fetchById('log-2')

      expect(result).toEqual(newLine)
      expect(store.logistics).toHaveLength(2)
      expect(store.logistics).toContainEqual(newLine)
    })

    it('fetchById updates existing logistics line (same id)', async () => {
      const updatedLine = {
        ...mockLogistics,
        transport_type: 'Drone' as const,
        total_quantity_per_min: 120,
      }
      vi.mocked(logisticsApi.getById).mockResolvedValue(updatedLine)

      const result = await store.fetchById('log-1')

      expect(result).toEqual(updatedLine)
      expect(store.logistics).toHaveLength(1)
      expect(store.logistics[0]).toEqual(updatedLine)
    })

    it('create upserts new line', async () => {
      const newLine = createMockLogistics('log-3', 'factory-3', 'factory-4', 'Bus')
      vi.mocked(logisticsApi.create).mockResolvedValue(newLine)

      const createData = {
        from_factory: 'factory-3',
        to_factory: 'factory-4',
        transport_type: 'Bus' as const,
        bus_name: 'New Bus',
        conveyors: [],
        pipelines: [],
      }

      const result = await store.create(createData as unknown as CreateLogisticsRequest)

      expect(result).toEqual(newLine)
      expect(store.logistics).toHaveLength(2)
      expect(store.logistics).toContainEqual(newLine)
    })

    it('update upserts updated line', async () => {
      const updatedLine = {
        ...mockLogistics,
        total_quantity_per_min: 200,
      }
      vi.mocked(logisticsApi.update).mockResolvedValue(updatedLine)

      const updateData = {
        from_factory: 'factory-1',
        to_factory: 'factory-2',
        transport_type: 'Truck' as const,
        item: 'CopperPlate' as Item,
        quantity_per_min: 200,
      }

      const result = await store.update('log-1', updateData as unknown as CreateLogisticsRequest)

      expect(result).toEqual(updatedLine)
      expect(store.logistics).toHaveLength(1)
      expect(store.logistics[0]).toEqual(updatedLine)
    })

    it('deleteLogistics removes from list and returns true on success', async () => {
      vi.mocked(logisticsApi.delete).mockResolvedValue(undefined)

      const result = await store.deleteLogistics('log-1')

      expect(result).toBe(true)
      expect(store.logistics).toHaveLength(0)
    })

    it('deleteLogistics returns false on failure', async () => {
      const error = new Error('Delete failed')
      vi.mocked(logisticsApi.delete).mockRejectedValue(error)

      const result = await store.deleteLogistics('log-1')

      expect(result).toBe(false)
      expect(store.logistics).toHaveLength(1)
    })
  })

  describe('Filtering methods', () => {
    beforeEach(() => {
      store.logistics = [
        createMockLogistics('log-1', 'factory-a', 'factory-b', 'Truck'),
        createMockLogistics('log-2', 'factory-a', 'factory-c', 'Train'),
        createMockLogistics('log-3', 'factory-b', 'factory-a', 'Drone'),
        createMockLogistics('log-4', 'factory-c', 'factory-a', 'Bus'),
        createMockLogistics('log-5', 'factory-b', 'factory-c', 'Truck'),
      ]
    })

    it('getLogisticsForFactory direction=from returns outbound only', () => {
      const result = store.getLogisticsForFactory('factory-a', 'from')

      expect(result).toHaveLength(2)
      expect(result.every(l => l.from_factory === 'factory-a')).toBe(true)
    })

    it('getLogisticsForFactory direction=to returns inbound only', () => {
      const result = store.getLogisticsForFactory('factory-a', 'to')

      expect(result).toHaveLength(2)
      expect(result.every(l => l.to_factory === 'factory-a')).toBe(true)
    })

    it('getLogisticsForFactory direction=both returns both directions', () => {
      const result = store.getLogisticsForFactory('factory-a', 'both')

      expect(result).toHaveLength(4)
    })

    it('getLogisticsForFactory default direction is both', () => {
      const result = store.getLogisticsForFactory('factory-a')

      expect(result).toHaveLength(4)
    })

    it('getLogisticsBetweenFactories returns exact match connections', () => {
      const result = store.getLogisticsBetweenFactories('factory-a', 'factory-b')

      expect(result).toHaveLength(1)
      expect(result[0].id).toBe('log-1')
    })

    it('getLogisticsByTransportType filters correctly', () => {
      const trucks = store.getLogisticsByTransportType('Truck')
      const trains = store.getLogisticsByTransportType('Train')
      const drones = store.getLogisticsByTransportType('Drone')
      const buses = store.getLogisticsByTransportType('Bus')

      expect(trucks).toHaveLength(2)
      expect(trains).toHaveLength(1)
      expect(drones).toHaveLength(1)
      expect(buses).toHaveLength(1)
    })
  })

  describe('clearError + reset', () => {
    it('clearError sets error to null', () => {
      store.error = 'Some error'

      store.clearError()

      expect(store.error).toBe(null)
    })

    it('reset clears all state to initial values', () => {
      store.logistics = [
        createMockLogistics('log-1', 'factory-1', 'factory-2', 'Truck'),
      ]
      store.loading = true
      store.error = 'Some error'

      store.reset()

      expect(store.logistics).toEqual([])
      expect(store.loading).toBe(false)
      expect(store.error).toBe(null)
    })
  })
})
