import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import type {
  FactoryResponse,
  CreateProductionLineRequest,
  UpdateProductionLineRequest,
  CreateRawInputRequest,
  UpdateRawInputRequest,
  CreatePowerGeneratorRequest,
  UpdatePowerGeneratorRequest,
} from '../types'

// Mock factory data
const mockFactoryId = '550e8400-e29b-41d4-a716-446655440000'
const mockProductionLineId = '550e8400-e29b-41d4-a716-446655440001'
const mockRawInputId = '550e8400-e29b-41d4-a716-446655440002'
const mockGeneratorId = '550e8400-e29b-41d4-a716-446655440003'

const mockFactoryResponse: FactoryResponse = {
  id: mockFactoryId,
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
}

const mockProductionLinePayload: CreateProductionLineRequest = {
  name: 'Iron Ingot Production',
  description: 'Produces iron ingots',
  type: 'recipe',
  recipe: 'Iron Ingot',
  machine_groups: [
    {
      number_of_machine: 4,
      oc_value: 100,
      somersloop: 0,
    },
  ],
}

const mockRawInputPayload: CreateRawInputRequest = {
  extractor_type: 'MinerMk2',
  item: 'IronOre',
  purity: 'Normal',
  quantity_per_min: 120,
}

const mockPowerGeneratorPayload: CreatePowerGeneratorRequest = {
  generator_type: 'Coal',
  fuel_type: 'Coal',
  groups: [
    {
      number_of_generators: 5,
      clock_speed: 150,
    },
  ],
}

describe('Production Line Endpoints', () => {
  let apiClient: any
  let productionLines: any

  beforeEach(async () => {
    // Clear modules to reset imports
    vi.resetModules()

    // Setup mock Axios
    const mockPost = vi.fn()
    const mockPut = vi.fn()
    const mockDelete = vi.fn()

    apiClient = {
      post: mockPost,
      put: mockPut,
      delete: mockDelete,
    }

    vi.doMock('../client', () => ({
      api: apiClient,
    }))

    // Import endpoints and extract productionLines
    const { factories } = await import('../endpoints')
    productionLines = factories.productionLines
  })

  afterEach(() => {
    vi.resetModules()
    vi.doUnmock('../client')
  })

  describe('productionLines.create', () => {
    it('should POST to /factories/{id}/production-lines with payload', async () => {
      apiClient.post.mockResolvedValue(mockFactoryResponse)

      const result = await productionLines.create(mockFactoryId, mockProductionLinePayload)

      expect(apiClient.post).toHaveBeenCalledWith(
        `/factories/${mockFactoryId}/production-lines`,
        mockProductionLinePayload
      )
      expect(result).toEqual(mockFactoryResponse)
    })

    it('should handle errors when creating production line', async () => {
      const error = new Error('Creation failed')
      apiClient.post.mockRejectedValue(error)

      await expect(
        productionLines.create(mockFactoryId, mockProductionLinePayload)
      ).rejects.toThrow('Creation failed')
    })

    it('should include all required fields in request', async () => {
      apiClient.post.mockResolvedValue(mockFactoryResponse)

      const payload: CreateProductionLineRequest = {
        name: 'Test Line',
        description: 'Test Description',
        type: 'recipe',
        recipe: 'Iron Ingot',
        machine_groups: [
          {
            number_of_machine: 2,
            oc_value: 125,
            somersloop: 1,
          },
        ],
      }

      await productionLines.create(mockFactoryId, payload)

      expect(apiClient.post).toHaveBeenCalledWith(
        expect.stringContaining(mockFactoryId),
        expect.objectContaining({
          name: 'Test Line',
          type: 'recipe',
          recipe: 'Iron Ingot',
        })
      )
    })
  })

  describe('productionLines.update', () => {
    it('should PUT to /factories/{id}/production-lines/{lineId}', async () => {
      const updatePayload: UpdateProductionLineRequest = {
        ...mockProductionLinePayload,
        name: 'Updated Production Line',
      }

      apiClient.put.mockResolvedValue(mockFactoryResponse)

      const result = await productionLines.update(mockFactoryId, mockProductionLineId, updatePayload)

      expect(apiClient.put).toHaveBeenCalledWith(
        `/factories/${mockFactoryId}/production-lines/${mockProductionLineId}`,
        updatePayload
      )
      expect(result).toEqual(mockFactoryResponse)
    })

    it('should handle 404 errors for missing production line', async () => {
      const error = new Error('Not found')
      apiClient.put.mockRejectedValue(error)

      await expect(
        productionLines.update(mockFactoryId, mockProductionLineId, mockProductionLinePayload)
      ).rejects.toThrow('Not found')
    })

    it('should support partial updates', async () => {
      const partialUpdate: UpdateProductionLineRequest = {
        name: 'Renamed Line',
        type: 'recipe',
      }

      apiClient.put.mockResolvedValue(mockFactoryResponse)

      await productionLines.update(mockFactoryId, mockProductionLineId, partialUpdate)

      expect(apiClient.put).toHaveBeenCalledWith(
        expect.stringContaining(mockProductionLineId),
        expect.objectContaining({ name: 'Renamed Line' })
      )
    })
  })

  describe('productionLines.delete', () => {
    it('should DELETE /factories/{id}/production-lines/{lineId}', async () => {
      apiClient.delete.mockResolvedValue(mockFactoryResponse)

      const result = await productionLines.delete(mockFactoryId, mockProductionLineId)

      expect(apiClient.delete).toHaveBeenCalledWith(
        `/factories/${mockFactoryId}/production-lines/${mockProductionLineId}`
      )
      expect(result).toEqual(mockFactoryResponse)
    })

    it('should handle deletion errors', async () => {
      const error = new Error('Deletion failed')
      apiClient.delete.mockRejectedValue(error)

      await expect(
        productionLines.delete(mockFactoryId, mockProductionLineId)
      ).rejects.toThrow('Deletion failed')
    })
  })
})

describe('Raw Input Endpoints', () => {
  let apiClient: any
  let rawInputs: any

  beforeEach(async () => {
    vi.resetModules()

    const mockPost = vi.fn()
    const mockPut = vi.fn()
    const mockDelete = vi.fn()

    apiClient = {
      post: mockPost,
      put: mockPut,
      delete: mockDelete,
    }

    vi.doMock('../client', () => ({
      api: apiClient,
    }))

    const { factories } = await import('../endpoints')
    rawInputs = factories.rawInputs
  })

  afterEach(() => {
    vi.resetModules()
    vi.doUnmock('../client')
  })

  describe('rawInputs.create', () => {
    it('should POST to /factories/{id}/raw-inputs with payload', async () => {
      apiClient.post.mockResolvedValue(mockFactoryResponse)

      const result = await rawInputs.create(mockFactoryId, mockRawInputPayload)

      expect(apiClient.post).toHaveBeenCalledWith(
        `/factories/${mockFactoryId}/raw-inputs`,
        mockRawInputPayload
      )
      expect(result).toEqual(mockFactoryResponse)
    })

    it('should handle different extractor types', async () => {
      apiClient.post.mockResolvedValue(mockFactoryResponse)

      const minerPayload: CreateRawInputRequest = {
        extractor_type: 'MinerMk3',
        item: 'CopperOre',
        purity: 'Pure',
        quantity_per_min: 240,
      }

      await rawInputs.create(mockFactoryId, minerPayload)

      expect(apiClient.post).toHaveBeenCalledWith(
        expect.stringContaining('/raw-inputs'),
        expect.objectContaining({
          extractor_type: 'MinerMk3',
          item: 'CopperOre',
        })
      )
    })

    it('should handle creation errors', async () => {
      const error = new Error('Invalid item type')
      apiClient.post.mockRejectedValue(error)

      await expect(
        rawInputs.create(mockFactoryId, mockRawInputPayload)
      ).rejects.toThrow('Invalid item type')
    })
  })

  describe('rawInputs.update', () => {
    it('should PUT to /factories/{id}/raw-inputs/{rawInputId}', async () => {
      const updatePayload: UpdateRawInputRequest = {
        ...mockRawInputPayload,
        quantity_per_min: 240,
      }

      apiClient.put.mockResolvedValue(mockFactoryResponse)

      const result = await rawInputs.update(mockFactoryId, mockRawInputId, updatePayload)

      expect(apiClient.put).toHaveBeenCalledWith(
        `/factories/${mockFactoryId}/raw-inputs/${mockRawInputId}`,
        updatePayload
      )
      expect(result).toEqual(mockFactoryResponse)
    })

    it('should handle update errors', async () => {
      const error = new Error('Not found')
      apiClient.put.mockRejectedValue(error)

      await expect(
        rawInputs.update(mockFactoryId, mockRawInputId, mockRawInputPayload)
      ).rejects.toThrow('Not found')
    })
  })

  describe('rawInputs.delete', () => {
    it('should DELETE /factories/{id}/raw-inputs/{rawInputId}', async () => {
      apiClient.delete.mockResolvedValue(mockFactoryResponse)

      const result = await rawInputs.delete(mockFactoryId, mockRawInputId)

      expect(apiClient.delete).toHaveBeenCalledWith(
        `/factories/${mockFactoryId}/raw-inputs/${mockRawInputId}`
      )
      expect(result).toEqual(mockFactoryResponse)
    })

    it('should handle deletion errors', async () => {
      const error = new Error('Deletion failed')
      apiClient.delete.mockRejectedValue(error)

      await expect(
        rawInputs.delete(mockFactoryId, mockRawInputId)
      ).rejects.toThrow('Deletion failed')
    })
  })
})

describe('Power Generator Endpoints', () => {
  let apiClient: any
  let powerGenerators: any

  beforeEach(async () => {
    vi.resetModules()

    const mockPost = vi.fn()
    const mockPut = vi.fn()
    const mockDelete = vi.fn()

    apiClient = {
      post: mockPost,
      put: mockPut,
      delete: mockDelete,
    }

    vi.doMock('../client', () => ({
      api: apiClient,
    }))

    const { factories } = await import('../endpoints')
    powerGenerators = factories.powerGenerators
  })

  afterEach(() => {
    vi.resetModules()
    vi.doUnmock('../client')
  })

  describe('powerGenerators.create', () => {
    it('should POST to /factories/{id}/power-generators with payload', async () => {
      apiClient.post.mockResolvedValue(mockFactoryResponse)

      const result = await powerGenerators.create(mockFactoryId, mockPowerGeneratorPayload)

      expect(apiClient.post).toHaveBeenCalledWith(
        `/factories/${mockFactoryId}/power-generators`,
        mockPowerGeneratorPayload
      )
      expect(result).toEqual(mockFactoryResponse)
    })

    it('should handle different generator types', async () => {
      apiClient.post.mockResolvedValue(mockFactoryResponse)

      const fuelPayload: CreatePowerGeneratorRequest = {
        generator_type: 'Fuel',
        fuel_type: 'Fuel',
        groups: [
          {
            number_of_generators: 10,
            clock_speed: 100,
          },
        ],
      }

      await powerGenerators.create(mockFactoryId, fuelPayload)

      expect(apiClient.post).toHaveBeenCalledWith(
        expect.stringContaining('/power-generators'),
        expect.objectContaining({
          generator_type: 'Fuel',
          fuel_type: 'Fuel',
        })
      )
    })

    it('should handle creation errors', async () => {
      const error = new Error('Invalid fuel type')
      apiClient.post.mockRejectedValue(error)

      await expect(
        powerGenerators.create(mockFactoryId, mockPowerGeneratorPayload)
      ).rejects.toThrow('Invalid fuel type')
    })
  })

  describe('powerGenerators.update', () => {
    it('should PUT to /factories/{id}/power-generators/{generatorId}', async () => {
      const updatePayload: UpdatePowerGeneratorRequest = {
        ...mockPowerGeneratorPayload,
        groups: [
          {
            number_of_generators: 8,
            clock_speed: 175,
          },
        ],
      }

      apiClient.put.mockResolvedValue(mockFactoryResponse)

      const result = await powerGenerators.update(mockFactoryId, mockGeneratorId, updatePayload)

      expect(apiClient.put).toHaveBeenCalledWith(
        `/factories/${mockFactoryId}/power-generators/${mockGeneratorId}`,
        updatePayload
      )
      expect(result).toEqual(mockFactoryResponse)
    })

    it('should handle update errors', async () => {
      const error = new Error('Not found')
      apiClient.put.mockRejectedValue(error)

      await expect(
        powerGenerators.update(mockFactoryId, mockGeneratorId, mockPowerGeneratorPayload)
      ).rejects.toThrow('Not found')
    })
  })

  describe('powerGenerators.delete', () => {
    it('should DELETE /factories/{id}/power-generators/{generatorId}', async () => {
      apiClient.delete.mockResolvedValue(mockFactoryResponse)

      const result = await powerGenerators.delete(mockFactoryId, mockGeneratorId)

      expect(apiClient.delete).toHaveBeenCalledWith(
        `/factories/${mockFactoryId}/power-generators/${mockGeneratorId}`
      )
      expect(result).toEqual(mockFactoryResponse)
    })

    it('should handle deletion errors', async () => {
      const error = new Error('Deletion failed')
      apiClient.delete.mockRejectedValue(error)

      await expect(
        powerGenerators.delete(mockFactoryId, mockGeneratorId)
      ).rejects.toThrow('Deletion failed')
    })
  })
})

describe('Endpoint Integration Tests', () => {
  beforeEach(async () => {
    vi.resetModules()

    vi.doMock('../client', () => ({
      api: {
        post: vi.fn(),
        put: vi.fn(),
        delete: vi.fn(),
      },
    }))
  })

  afterEach(() => {
    vi.resetModules()
    vi.doUnmock('../client')
  })

  it('should properly export productionLines as a sub-object of factories', async () => {
    const { factories } = await import('../endpoints')
    expect(factories.productionLines).toBeDefined()
    expect(typeof factories.productionLines.create).toBe('function')
    expect(typeof factories.productionLines.update).toBe('function')
    expect(typeof factories.productionLines.delete).toBe('function')
  })

  it('should properly export rawInputs as a sub-object of factories', async () => {
    const { factories } = await import('../endpoints')
    expect(factories.rawInputs).toBeDefined()
    expect(typeof factories.rawInputs.create).toBe('function')
    expect(typeof factories.rawInputs.update).toBe('function')
    expect(typeof factories.rawInputs.delete).toBe('function')
  })

  it('should properly export powerGenerators as a sub-object of factories', async () => {
    const { factories } = await import('../endpoints')
    expect(factories.powerGenerators).toBeDefined()
    expect(typeof factories.powerGenerators.create).toBe('function')
    expect(typeof factories.powerGenerators.update).toBe('function')
    expect(typeof factories.powerGenerators.delete).toBe('function')
  })

  it('should construct correct URLs with UUIDs', async () => {
    const mockPost = vi.fn().mockResolvedValue(mockFactoryResponse)

    vi.doMock('../client', () => ({
      api: {
        post: mockPost,
      },
    }), { virtual: true })

    const { factories } = await import('../endpoints')
    const factoryId = '123e4567-e89b-12d3-a456-426614174000'

    await factories.productionLines.create(factoryId, mockProductionLinePayload)

    expect(mockPost).toHaveBeenCalledWith(
      `/factories/${factoryId}/production-lines`,
      expect.any(Object)
    )
  })
})
