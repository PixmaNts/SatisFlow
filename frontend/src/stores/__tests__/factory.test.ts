import { describe, it, expect, beforeEach, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useFactoryStore } from '../factory'
import type {
  FactoryResponse,
  CreateFactoryRequest,
  UpdateFactoryRequest,
  CreateProductionLineRequest,
  UpdateProductionLineRequest,
  CreateRawInputRequest,
  UpdateRawInputRequest,
  CreatePowerGeneratorRequest,
  UpdatePowerGeneratorRequest,
} from '@/api/types'

vi.mock('@/api/endpoints', () => ({
  factories: {
    getAll: vi.fn(),
    getById: vi.fn(),
    create: vi.fn(),
    update: vi.fn(),
    delete: vi.fn(),
    productionLines: {
      create: vi.fn(),
      update: vi.fn(),
      delete: vi.fn(),
    },
    rawInputs: {
      create: vi.fn(),
      update: vi.fn(),
      delete: vi.fn(),
    },
    powerGenerators: {
      create: vi.fn(),
      update: vi.fn(),
      delete: vi.fn(),
    },
  },
}))

vi.mock('@/api', () => ({
  handleApiError: vi.fn((e: any) => e?.message || 'API Error'),
}))

import { factories as factoriesApi } from '@/api/endpoints'

describe('Factory Store', () => {
  let store: ReturnType<typeof useFactoryStore>

  const mockFactory = (overrides = {}): FactoryResponse => ({
    id: 'factory-1',
    name: 'Test Factory',
    description: 'A test factory',
    notes: null,
    production_lines: [],
    raw_inputs: [],
    power_generators: [],
    items: [],
    total_power_consumption: 0,
    total_power_generation: 0,
    power_balance: 0,
    ...overrides,
  })

  beforeEach(() => {
    const pinia = createPinia()
    setActivePinia(pinia)
    store = useFactoryStore()
    vi.clearAllMocks()
  })

  describe('Initial state', () => {
    it('factories is empty array', () => {
      expect(store.factories).toEqual([])
    })

    it('currentFactoryId is null', () => {
      expect(store.currentFactoryId).toBe(null)
    })

    it('loading is false', () => {
      expect(store.loading).toBe(false)
    })

    it('error is null', () => {
      expect(store.error).toBe(null)
    })
  })

  describe('Getters', () => {
    it('currentFactory returns matching factory when currentFactoryId is set', () => {
      const factory = mockFactory({ id: 'factory-1', name: 'Factory One' })
      store.factories.push(factory)
      store.currentFactoryId = 'factory-1'

      expect(store.currentFactory).toEqual(factory)
    })

    it('currentFactory returns null when currentFactoryId is null', () => {
      store.currentFactoryId = null

      expect(store.currentFactory).toBe(null)
    })

    it('factoriesById maps id to factory correctly', () => {
      const factory1 = mockFactory({ id: 'factory-1', name: 'Factory One' })
      const factory2 = mockFactory({ id: 'factory-2', name: 'Factory Two' })
      store.factories.push(factory1, factory2)

      expect(store.factoriesById).toEqual({
        'factory-1': factory1,
        'factory-2': factory2,
      })
    })

    it('factoryNames extracts id/name pairs from all factories', () => {
      const factory1 = mockFactory({ id: 'factory-1', name: 'Factory One' })
      const factory2 = mockFactory({ id: 'factory-2', name: 'Factory Two' })
      store.factories.push(factory1, factory2)

      expect(store.factoryNames).toEqual([
        { id: 'factory-1', name: 'Factory One' },
        { id: 'factory-2', name: 'Factory Two' },
      ])
    })
  })

  describe('Factory CRUD', () => {
    it('fetchAll replaces factories on success', async () => {
      const factories = [
        mockFactory({ id: 'factory-1', name: 'Factory One' }),
        mockFactory({ id: 'factory-2', name: 'Factory Two' }),
      ]
      vi.mocked(factoriesApi.getAll).mockResolvedValue(factories)

      await store.fetchAll()

      expect(store.factories).toEqual(factories)
      expect(factoriesApi.getAll).toHaveBeenCalled()
    })

    it('fetchAll sets loading during call, false after', async () => {
      vi.mocked(factoriesApi.getAll).mockImplementation(
        () => new Promise(resolve => setTimeout(() => resolve([]), 100))
      )

      const fetchPromise = store.fetchAll()

      expect(store.loading).toBe(true)

      await fetchPromise

      expect(store.loading).toBe(false)
    })

    it('fetchAll sets error on API failure', async () => {
      const error = new Error('Failed to fetch')
      vi.mocked(factoriesApi.getAll).mockRejectedValue(error)

      await store.fetchAll()

      expect(store.error).toBe('Failed to fetch')
      expect(store.factories).toEqual([])
    })

    it('fetchById appends new factory if not in list', async () => {
      const newFactory = mockFactory({ id: 'factory-new', name: 'New Factory' })
      vi.mocked(factoriesApi.getById).mockResolvedValue(newFactory)

      const result = await store.fetchById('factory-new')

      expect(result).toEqual(newFactory)
      expect(store.factories).toHaveLength(1)
      expect(store.factories[0].id).toBe('factory-new')
      expect(store.factories[0].name).toBe('New Factory')
    })

    it('fetchById updates existing factory data if already in list', async () => {
      const existingFactory = mockFactory({ id: 'factory-1', name: 'Old Name' })
      store.factories.push(existingFactory)

      const updatedFactory = mockFactory({ id: 'factory-1', name: 'Updated Name' })
      vi.mocked(factoriesApi.getById).mockResolvedValue(updatedFactory)

      await store.fetchById('factory-1')

      expect(store.factories).toHaveLength(1)
      expect(store.factories[0].name).toBe('Updated Name')
    })

    it('create upserts new factory into list', async () => {
      const newFactory = mockFactory({ id: 'factory-new', name: 'New Factory' })
      vi.mocked(factoriesApi.create).mockResolvedValue(newFactory)

      const createData: CreateFactoryRequest = { name: 'New Factory' }
      const result = await store.create(createData)

      expect(result).toEqual(newFactory)
      expect(store.factories).toHaveLength(1)
      expect(store.factories[0].id).toBe('factory-new')
      expect(factoriesApi.create).toHaveBeenCalledWith(createData)
    })

    it('update upserts updated factory in-place', async () => {
      const existingFactory = mockFactory({ id: 'factory-1', name: 'Old Name' })
      store.factories.push(existingFactory)

      const updatedFactory = mockFactory({ id: 'factory-1', name: 'Updated Name' })
      vi.mocked(factoriesApi.update).mockResolvedValue(updatedFactory)

      const updateData: UpdateFactoryRequest = { name: 'Updated Name' }
      const result = await store.update('factory-1', updateData)

      expect(result).toEqual(updatedFactory)
      expect(store.factories).toHaveLength(1)
      expect(store.factories[0].name).toBe('Updated Name')
      expect(factoriesApi.update).toHaveBeenCalledWith('factory-1', updateData)
    })

    it('deleteFactory removes factory from list and clears currentFactoryId if matches', async () => {
      const factory1 = mockFactory({ id: 'factory-1' })
      const factory2 = mockFactory({ id: 'factory-2' })
      store.factories.push(factory1, factory2)
      store.currentFactoryId = 'factory-1'

      vi.mocked(factoriesApi.delete).mockResolvedValue()

      const result = await store.deleteFactory('factory-1')

      expect(result).toBe(true)
      expect(store.factories).toHaveLength(1)
      expect(store.factories[0].id).toBe('factory-2')
      expect(store.currentFactoryId).toBe(null)
      expect(factoriesApi.delete).toHaveBeenCalledWith('factory-1')
    })

    it('deleteFactory returns false on API failure', async () => {
      const error = new Error('Delete failed')
      vi.mocked(factoriesApi.delete).mockRejectedValue(error)

      const result = await store.deleteFactory('factory-1')

      expect(result).toBe(false)
      expect(store.error).toBe('Delete failed')
    })
  })

  describe('Production Line CRUD', () => {
    it('createProductionLine calls API and upserts result', async () => {
      const factory = mockFactory({ id: 'factory-1' })
      store.factories.push(factory)

      const updatedFactory = mockFactory({
        id: 'factory-1',
        production_lines: [
          {
            ProductionLineRecipe: {
              id: 'line-1',
              name: 'New Line',
              description: null,
              recipe: 'IronPlate',
              machine_groups: [{ number_of_machine: 1, oc_value: 100, somersloop: 0 }],
            },
            total_power_consumption: 100,
            total_machines: 1,
            total_somersloop: 0,
            input_rate: [],
            output_rate: [],
          },
        ],
      })
      vi.mocked(factoriesApi.productionLines.create).mockResolvedValue(updatedFactory)

      const payload: CreateProductionLineRequest = {
        name: 'New Line',
        type: 'recipe',
        recipe: 'IronPlate',
        machine_groups: [{ number_of_machine: 1, oc_value: 100, somersloop: 0 }],
      }
      const result = await store.createProductionLine('factory-1', payload)

      expect(result).toEqual(updatedFactory)
      expect(store.factories[0].production_lines).toHaveLength(1)
      expect(factoriesApi.productionLines.create).toHaveBeenCalledWith('factory-1', payload)
    })

    it('updateProductionLine calls API with 3 args and upserts result', async () => {
      const factory = mockFactory({
        id: 'factory-1',
        production_lines: [
          {
            ProductionLineRecipe: {
              id: 'line-1',
              name: 'Old Line',
              description: null,
              recipe: 'IronPlate',
              machine_groups: [{ number_of_machine: 1, oc_value: 100, somersloop: 0 }],
            },
            total_power_consumption: 100,
            total_machines: 1,
            total_somersloop: 0,
            input_rate: [],
            output_rate: [],
          },
        ],
      })
      store.factories.push(factory)

      const updatedFactory = mockFactory({
        id: 'factory-1',
        production_lines: [
          {
            ProductionLineRecipe: {
              id: 'line-1',
              name: 'Updated Line',
              description: null,
              recipe: 'CopperPlate',
              machine_groups: [{ number_of_machine: 2, oc_value: 150, somersloop: 0 }],
            },
            total_power_consumption: 200,
            total_machines: 2,
            total_somersloop: 0,
            input_rate: [],
            output_rate: [],
          },
        ],
      })
      vi.mocked(factoriesApi.productionLines.update).mockResolvedValue(updatedFactory)

      const payload: UpdateProductionLineRequest = {
        name: 'Updated Line',
        type: 'recipe',
        recipe: 'CopperPlate',
        machine_groups: [{ number_of_machine: 2, oc_value: 150, somersloop: 0 }],
      }
      const result = await store.updateProductionLine('factory-1', 'line-1', payload)

      expect(result).toEqual(updatedFactory)
      expect(factoriesApi.productionLines.update).toHaveBeenCalledWith('factory-1', 'line-1', payload)
    })

    it('deleteProductionLine calls API and upserts result', async () => {
      const factory = mockFactory({
        id: 'factory-1',
        production_lines: [
          {
            ProductionLineRecipe: {
              id: 'line-1',
              name: 'Line to Delete',
              description: null,
              recipe: 'IronPlate',
              machine_groups: [{ number_of_machine: 1, oc_value: 100, somersloop: 0 }],
            },
            total_power_consumption: 100,
            total_machines: 1,
            total_somersloop: 0,
            input_rate: [],
            output_rate: [],
          },
        ],
      })
      store.factories.push(factory)

      const updatedFactory = mockFactory({ id: 'factory-1', production_lines: [] })
      vi.mocked(factoriesApi.productionLines.delete).mockResolvedValue(updatedFactory)

      const result = await store.deleteProductionLine('factory-1', 'line-1')

      expect(result).toEqual(updatedFactory)
      expect(store.factories[0].production_lines).toHaveLength(0)
      expect(factoriesApi.productionLines.delete).toHaveBeenCalledWith('factory-1', 'line-1')
    })
  })

  describe('Raw Input CRUD', () => {
    it('createRawInput calls API and upserts result', async () => {
      const factory = mockFactory({ id: 'factory-1' })
      store.factories.push(factory)

      const updatedFactory = mockFactory({
        id: 'factory-1',
        raw_inputs: [
          {
            id: 'input-1',
            extractor_type: 'MinerMk1',
            item: 'IronOre',
            purity: 'Normal',
            quantity_per_min: 60,
            overclock_percent: 100,
            count: 1,
            pressurizer: null,
            extractors: [],
            power_consumption: 100,
          },
        ],
      })
      vi.mocked(factoriesApi.rawInputs.create).mockResolvedValue(updatedFactory)

      const payload: CreateRawInputRequest = {
        extractor_type: 'MinerMk1',
        item: 'IronOre',
        purity: 'Normal',
      }
      const result = await store.createRawInput('factory-1', payload)

      expect(result).toEqual(updatedFactory)
      expect(store.factories[0].raw_inputs).toHaveLength(1)
      expect(factoriesApi.rawInputs.create).toHaveBeenCalledWith('factory-1', payload)
    })

    it('updateRawInput calls API and upserts result', async () => {
      const factory = mockFactory({
        id: 'factory-1',
        raw_inputs: [
          {
            id: 'input-1',
            extractor_type: 'MinerMk1',
            item: 'IronOre',
            purity: 'Normal',
            quantity_per_min: 60,
            overclock_percent: 100,
            count: 1,
            pressurizer: null,
            extractors: [],
            power_consumption: 100,
          },
        ],
      })
      store.factories.push(factory)

      const updatedFactory = mockFactory({
        id: 'factory-1',
        raw_inputs: [
          {
            id: 'input-1',
            extractor_type: 'MinerMk1',
            item: 'IronOre',
            purity: 'Pure',
            quantity_per_min: 120,
            overclock_percent: 200,
            count: 2,
            pressurizer: null,
            extractors: [],
            power_consumption: 200,
          },
        ],
      })
      vi.mocked(factoriesApi.rawInputs.update).mockResolvedValue(updatedFactory)

      const payload: UpdateRawInputRequest = {
        extractor_type: 'MinerMk1',
        item: 'IronOre',
        purity: 'Pure',
        overclock_percent: 200,
        count: 2,
      }
      const result = await store.updateRawInput('factory-1', 'input-1', payload)

      expect(result).toEqual(updatedFactory)
      expect(store.factories[0].raw_inputs[0].purity).toBe('Pure')
      expect(factoriesApi.rawInputs.update).toHaveBeenCalledWith('factory-1', 'input-1', payload)
    })

    it('deleteRawInput calls API and upserts result', async () => {
      const factory = mockFactory({
        id: 'factory-1',
        raw_inputs: [
          {
            id: 'input-1',
            extractor_type: 'MinerMk1',
            item: 'IronOre',
            purity: 'Normal',
            quantity_per_min: 60,
            overclock_percent: 100,
            count: 1,
            pressurizer: null,
            extractors: [],
            power_consumption: 100,
          },
        ],
      })
      store.factories.push(factory)

      const updatedFactory = mockFactory({ id: 'factory-1', raw_inputs: [] })
      vi.mocked(factoriesApi.rawInputs.delete).mockResolvedValue(updatedFactory)

      const result = await store.deleteRawInput('factory-1', 'input-1')

      expect(result).toEqual(updatedFactory)
      expect(store.factories[0].raw_inputs).toHaveLength(0)
      expect(factoriesApi.rawInputs.delete).toHaveBeenCalledWith('factory-1', 'input-1')
    })
  })

  describe('Power Generator CRUD', () => {
    it('createPowerGenerator calls API and upserts result', async () => {
      const factory = mockFactory({ id: 'factory-1' })
      store.factories.push(factory)

      const updatedFactory = mockFactory({
        id: 'factory-1',
        power_generators: [
          {
            id: 'gen-1',
            generator_type: 'Coal',
            fuel_type: null,
            groups: [{ number_of_generators: 1, clock_speed: 100 }],
            total_power_generation: 150,
            total_fuel_consumption: 10,
            waste_production_rate: 0,
            waste_product: null,
          },
        ],
      })
      vi.mocked(factoriesApi.powerGenerators.create).mockResolvedValue(updatedFactory)

      const payload: CreatePowerGeneratorRequest = {
        generator_type: 'Coal',
        groups: [{ number_of_generators: 1, clock_speed: 100 }],
      }
      const result = await store.createPowerGenerator('factory-1', payload)

      expect(result).toEqual(updatedFactory)
      expect(store.factories[0].power_generators).toHaveLength(1)
      expect(factoriesApi.powerGenerators.create).toHaveBeenCalledWith('factory-1', payload)
    })

    it('updatePowerGenerator calls API and upserts result', async () => {
      const factory = mockFactory({
        id: 'factory-1',
        power_generators: [
          {
            id: 'gen-1',
            generator_type: 'Coal',
            fuel_type: null,
            groups: [{ number_of_generators: 1, clock_speed: 100 }],
            total_power_generation: 150,
            total_fuel_consumption: 10,
            waste_production_rate: 0,
            waste_product: null,
          },
        ],
      })
      store.factories.push(factory)

      const updatedFactory = mockFactory({
        id: 'factory-1',
        power_generators: [
          {
            id: 'gen-1',
            generator_type: 'Coal',
            fuel_type: null,
            groups: [{ number_of_generators: 2, clock_speed: 150 }],
            total_power_generation: 300,
            total_fuel_consumption: 20,
            waste_production_rate: 0,
            waste_product: null,
          },
        ],
      })
      vi.mocked(factoriesApi.powerGenerators.update).mockResolvedValue(updatedFactory)

      const payload: UpdatePowerGeneratorRequest = {
        generator_type: 'Coal',
        groups: [{ number_of_generators: 2, clock_speed: 150 }],
      }
      const result = await store.updatePowerGenerator('factory-1', 'gen-1', payload)

      expect(result).toEqual(updatedFactory)
      expect(store.factories[0].power_generators[0].groups[0].number_of_generators).toBe(2)
      expect(factoriesApi.powerGenerators.update).toHaveBeenCalledWith('factory-1', 'gen-1', payload)
    })

    it('deletePowerGenerator calls API and upserts result', async () => {
      const factory = mockFactory({
        id: 'factory-1',
        power_generators: [
          {
            id: 'gen-1',
            generator_type: 'Coal',
            fuel_type: null,
            groups: [{ number_of_generators: 1, clock_speed: 100 }],
            total_power_generation: 150,
            total_fuel_consumption: 10,
            waste_production_rate: 0,
            waste_product: null,
          },
        ],
      })
      store.factories.push(factory)

      const updatedFactory = mockFactory({ id: 'factory-1', power_generators: [] })
      vi.mocked(factoriesApi.powerGenerators.delete).mockResolvedValue(updatedFactory)

      const result = await store.deletePowerGenerator('factory-1', 'gen-1')

      expect(result).toEqual(updatedFactory)
      expect(store.factories[0].power_generators).toHaveLength(0)
      expect(factoriesApi.powerGenerators.delete).toHaveBeenCalledWith('factory-1', 'gen-1')
    })
  })

  describe('Sync actions and reset', () => {
    it('setCurrentFactory sets the ID', () => {
      store.setCurrentFactory('factory-1')

      expect(store.currentFactoryId).toBe('factory-1')
    })

    it('setCurrentFactory can set to null', () => {
      store.currentFactoryId = 'factory-1'

      store.setCurrentFactory(null)

      expect(store.currentFactoryId).toBe(null)
    })

    it('clearError resets error to null', () => {
      store.error = 'Some error'

      store.clearError()

      expect(store.error).toBe(null)
    })

    it('reset clears ALL state', () => {
      const factory = mockFactory({ id: 'factory-1' })
      store.factories.push(factory)
      store.currentFactoryId = 'factory-1'
      store.loading = true
      store.error = 'Some error'

      store.reset()

      expect(store.factories).toEqual([])
      expect(store.currentFactoryId).toBe(null)
      expect(store.loading).toBe(false)
      expect(store.error).toBe(null)
    })
  })
})
