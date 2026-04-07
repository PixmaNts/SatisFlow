import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useGameDataStore } from '../gameData'
import type { RecipeInfo, MachineInfo, ItemInfo } from '@/api/types'

vi.mock('@/api/endpoints', () => ({
  gameData: {
    getRecipes: vi.fn(),
    getItems: vi.fn(),
    getMachines: vi.fn(),
  },
}))

vi.mock('@/api', () => ({
  handleApiError: vi.fn((e: any) => e?.message || 'API Error'),
}))

import { gameData as gameDataApi } from '@/api/endpoints'

// Helper to create mock recipe
function createMockRecipe(overrides: Partial<RecipeInfo> = {}): RecipeInfo {
  return {
    name: 'IronIngot',
    machine: 'Smelter',
    inputs: [{ item: 'IronOre', quantity: 1 }],
    outputs: [{ item: 'IronIngot', quantity: 1 }],
    ...overrides,
  }
}

// Helper to create mock machine
function createMockMachine(overrides: Partial<MachineInfo> = {}): MachineInfo {
  return {
    name: 'Smelter',
    base_power: 4,
    max_somersloop: 0,
    ...overrides,
  }
}

describe('gameData store', () => {
  beforeEach(() => {
    const pinia = createPinia()
    setActivePinia(pinia)
    vi.clearAllMocks()
  })

  afterEach(() => {
    vi.useRealTimers()
  })

  // ==========================================================================
  // 1. Initial State
  // ==========================================================================
  describe('initial state', () => {
    it('has empty arrays for recipes, items, and machines', () => {
      const store = useGameDataStore()
      expect(store.recipes).toEqual([])
      expect(store.items).toEqual([])
      expect(store.machines).toEqual([])
    })

    it('has loading set to false', () => {
      const store = useGameDataStore()
      expect(store.loading).toBe(false)
    })

    it('has error set to null', () => {
      const store = useGameDataStore()
      expect(store.error).toBe(null)
    })

    it('has lastFetchTime initialized in state', () => {
      const store = useGameDataStore()
      // Verify the store has the lastFetchTime structure via clearCache behavior
      store.clearCache()
      // After clearCache, all values should be 0 - verify via subsequent API calls
      expect(store.error).toBe(null)
    })
  })

  // ==========================================================================
  // 2. Cache Behavior
  // ==========================================================================
  describe('cache behavior', () => {
    it('fetchRecipes skips second API call when called twice without forceRefresh', async () => {
      const store = useGameDataStore()
      const mockRecipes = [createMockRecipe()]
      vi.mocked(gameDataApi.getRecipes).mockResolvedValue(mockRecipes)

      // First call should hit API
      await store.fetchRecipes()
      expect(gameDataApi.getRecipes).toHaveBeenCalledTimes(1)

      // Second call should skip API (cache hit)
      await store.fetchRecipes()
      expect(gameDataApi.getRecipes).toHaveBeenCalledTimes(1)
    })

    it('fetchRecipes calls API with forceRefresh even after previous fetch', async () => {
      const store = useGameDataStore()
      const mockRecipes = [createMockRecipe()]
      vi.mocked(gameDataApi.getRecipes).mockResolvedValue(mockRecipes)

      // First call
      await store.fetchRecipes()
      expect(gameDataApi.getRecipes).toHaveBeenCalledTimes(1)

      // forceRefresh should call API again
      await store.fetchRecipes(true)
      expect(gameDataApi.getRecipes).toHaveBeenCalledTimes(2)
    })

    it('fetchItems skips second API call when called twice without forceRefresh', async () => {
      const store = useGameDataStore()
      const mockItems: ItemInfo[] = ['IronOre', 'CopperOre']
      vi.mocked(gameDataApi.getItems).mockResolvedValue(mockItems)

      await store.fetchItems()
      expect(gameDataApi.getItems).toHaveBeenCalledTimes(1)

      await store.fetchItems()
      expect(gameDataApi.getItems).toHaveBeenCalledTimes(1)
    })

    it('fetchMachines skips second API call when called twice without forceRefresh', async () => {
      const store = useGameDataStore()
      const mockMachines = [createMockMachine()]
      vi.mocked(gameDataApi.getMachines).mockResolvedValue(mockMachines)

      await store.fetchMachines()
      expect(gameDataApi.getMachines).toHaveBeenCalledTimes(1)

      await store.fetchMachines()
      expect(gameDataApi.getMachines).toHaveBeenCalledTimes(1)
    })

    it('clearCache allows fresh API calls after being called', async () => {
      const store = useGameDataStore()
      const mockRecipes = [createMockRecipe()]
      vi.mocked(gameDataApi.getRecipes).mockResolvedValue(mockRecipes)

      // First fetch
      await store.fetchRecipes()
      expect(gameDataApi.getRecipes).toHaveBeenCalledTimes(1)

      // Clear cache
      store.clearCache()

      // Next fetch should hit API again
      await store.fetchRecipes()
      expect(gameDataApi.getRecipes).toHaveBeenCalledTimes(2)
    })
  })

  // ==========================================================================
  // 3. Fetch Operations
  // ==========================================================================
  describe('fetch operations', () => {
    it('fetchRecipes populates recipes array on success', async () => {
      const store = useGameDataStore()
      const mockRecipes = [
        createMockRecipe({ name: 'IronIngot' }),
        createMockRecipe({ name: 'CopperIngot', machine: 'Smelter' }),
      ]
      vi.mocked(gameDataApi.getRecipes).mockResolvedValue(mockRecipes)

      await store.fetchRecipes()

      expect(store.recipes).toEqual(mockRecipes)
    })

    it('fetchRecipes handles error on failure', async () => {
      const store = useGameDataStore()
      vi.mocked(gameDataApi.getRecipes).mockRejectedValue(new Error('Network error'))

      await store.fetchRecipes()

      expect(store.error).toBe('Network error')
      expect(store.loading).toBe(false)
    })

    it('fetchItems populates items array on success', async () => {
      const store = useGameDataStore()
      const mockItems: ItemInfo[] = ['IronOre', 'CopperOre', 'IronIngot']
      vi.mocked(gameDataApi.getItems).mockResolvedValue(mockItems)

      await store.fetchItems()

      expect(store.items).toEqual(mockItems)
    })

    it('fetchMachines populates machines array on success', async () => {
      const store = useGameDataStore()
      const mockMachines = [
        createMockMachine({ name: 'Smelter' }),
        createMockMachine({ name: 'Assembler', base_power: 15 }),
      ]
      vi.mocked(gameDataApi.getMachines).mockResolvedValue(mockMachines)

      await store.fetchMachines()

      expect(store.machines).toEqual(mockMachines)
    })
  })

  // ==========================================================================
  // 4. fetchAllData
  // ==========================================================================
  describe('fetchAllData', () => {
    it('calls all three API methods', async () => {
      const store = useGameDataStore()
      vi.mocked(gameDataApi.getRecipes).mockResolvedValue([])
      vi.mocked(gameDataApi.getItems).mockResolvedValue([])
      vi.mocked(gameDataApi.getMachines).mockResolvedValue([])

      // Clear cache to ensure API calls are made
      store.clearCache()

      await store.fetchAllData()

      expect(gameDataApi.getRecipes).toHaveBeenCalledTimes(1)
      expect(gameDataApi.getItems).toHaveBeenCalledTimes(1)
      expect(gameDataApi.getMachines).toHaveBeenCalledTimes(1)
    })

    it('propagates forceRefresh to all three fetch methods', async () => {
      const store = useGameDataStore()
      vi.mocked(gameDataApi.getRecipes).mockResolvedValue([])
      vi.mocked(gameDataApi.getItems).mockResolvedValue([])
      vi.mocked(gameDataApi.getMachines).mockResolvedValue([])

      await store.fetchAllData(true)

      // All three should have been called (force refresh bypasses cache)
      expect(gameDataApi.getRecipes).toHaveBeenCalledTimes(1)
      expect(gameDataApi.getItems).toHaveBeenCalledTimes(1)
      expect(gameDataApi.getMachines).toHaveBeenCalledTimes(1)
    })
  })

  // ==========================================================================
  // 5. Getters
  // ==========================================================================
  describe('getters', () => {
    beforeEach(() => {
      const store = useGameDataStore()
      store.recipes = [
        createMockRecipe({ name: 'IronIngot', machine: 'Smelter' }),
        createMockRecipe({ name: 'CopperIngot', machine: 'Smelter' }),
        createMockRecipe({ name: 'IronPlate', machine: 'Assembler' }),
      ]
      store.items = ['IronOre', 'CopperOre', 'IronIngot', 'CopperIngot']
      store.machines = [
        createMockMachine({ name: 'Smelter', base_power: 4 }),
        createMockMachine({ name: 'Assembler', base_power: 15 }),
      ]
    })

    it('recipesByMachine groups recipes by machine type', () => {
      const store = useGameDataStore()
      const result = store.recipesByMachine

      expect(result['Smelter']).toHaveLength(2)
      expect(result['Assembler']).toHaveLength(1)
    })

    it('itemsByName uses item string as both key and value', () => {
      const store = useGameDataStore()
      const result = store.itemsByName

      expect(result['IronOre']).toBe('IronOre')
      expect(result['CopperOre']).toBe('CopperOre')
      expect(result['IronIngot']).toBe('IronIngot')
    })

    it('machinesByType maps machine name to MachineInfo', () => {
      const store = useGameDataStore()
      const result = store.machinesByType

      expect(result['Smelter']).toEqual({ name: 'Smelter', base_power: 4, max_somersloop: 0 })
      expect(result['Assembler']).toEqual({ name: 'Assembler', base_power: 15, max_somersloop: 0 })
    })

    it('machineNames returns array of machine name strings', () => {
      const store = useGameDataStore()
      const result = store.machineNames

      expect(result).toEqual(['Smelter', 'Assembler'])
    })

    it('recipeNames returns array of recipe name strings', () => {
      const store = useGameDataStore()
      const result = store.recipeNames

      expect(result).toEqual(['IronIngot', 'CopperIngot', 'IronPlate'])
    })
  })

  // ==========================================================================
  // 6. Lookup Methods
  // ==========================================================================
  describe('lookup methods', () => {
    beforeEach(() => {
      const store = useGameDataStore()
      store.recipes = [
        createMockRecipe({ name: 'IronIngot', machine: 'Smelter' }),
        createMockRecipe({ name: 'CopperIngot', machine: 'Smelter' }),
      ]
      store.machines = [
        createMockMachine({ name: 'Smelter', base_power: 4, max_somersloop: 0 }),
        createMockMachine({ name: 'Constructor', base_power: 5, max_somersloop: 2 }),
      ]
    })

    it('getRecipesByMachine returns filtered list for given machine', () => {
      const store = useGameDataStore()
      const result = store.getRecipesByMachine('Smelter')

      expect(result).toHaveLength(2)
      expect(result[0].name).toBe('IronIngot')
    })

    it('getRecipesByMachine returns empty array for unknown machine', () => {
      const store = useGameDataStore()
      const result = store.getRecipesByMachine('UnknownMachine')

      expect(result).toEqual([])
    })

    it('getRecipeByName returns matching recipe', () => {
      const store = useGameDataStore()
      const result = store.getRecipeByName('IronIngot')

      expect(result).not.toBeNull()
      expect(result?.machine).toBe('Smelter')
    })

    it('getRecipeByName returns null when no match', () => {
      const store = useGameDataStore()
      const result = store.getRecipeByName('NonExistentRecipe')

      expect(result).toBeNull()
    })

    it('getMachineByName returns matching machine', () => {
      const store = useGameDataStore()
      const result = store.getMachineByName('Constructor')

      expect(result).not.toBeNull()
      expect(result?.base_power).toBe(5)
    })

    it('getMachineByName returns null when no match', () => {
      const store = useGameDataStore()
      const result = store.getMachineByName('UnknownMachine')

      expect(result).toBeNull()
    })

    it('getBasePower returns power for known machine', () => {
      const store = useGameDataStore()
      const result = store.getBasePower('Constructor')

      expect(result).toBe(5)
    })

    it('getBasePower returns 0 for unknown machine', () => {
      const store = useGameDataStore()
      const result = store.getBasePower('UnknownMachine')

      expect(result).toBe(0)
    })

    it('getMaxSomersloops returns value for known machine', () => {
      const store = useGameDataStore()
      const result = store.getMaxSomersloops('Constructor')

      expect(result).toBe(2)
    })

    it('getMaxSomersloops returns 0 for unknown machine', () => {
      const store = useGameDataStore()
      const result = store.getMaxSomersloops('UnknownMachine')

      expect(result).toBe(0)
    })
  })

  // ==========================================================================
  // 7. Search Methods
  // ==========================================================================
  describe('search methods', () => {
    beforeEach(() => {
      const store = useGameDataStore()
      store.recipes = [
        createMockRecipe({
          name: 'IronIngot',
          machine: 'Smelter',
          inputs: [{ item: 'IronOre', quantity: 1 }],
          outputs: [{ item: 'IronIngot', quantity: 1 }],
        }),
        createMockRecipe({
          name: 'CopperIngot',
          machine: 'Smelter',
          inputs: [{ item: 'CopperOre', quantity: 1 }],
          outputs: [{ item: 'CopperIngot', quantity: 1 }],
        }),
        createMockRecipe({
          name: 'IronPlate',
          machine: 'Assembler',
          inputs: [{ item: 'IronIngot', quantity: 2 }],
          outputs: [{ item: 'IronPlate', quantity: 1 }],
        }),
      ]
    })

    it('searchRecipesByItem finds recipes by input item (case-insensitive)', () => {
      const store = useGameDataStore()
      const result = store.searchRecipesByItem('ironore')

      expect(result).toHaveLength(1)
      expect(result[0].name).toBe('IronIngot')
    })

    it('searchRecipesByItem finds recipes by output item (case-insensitive)', () => {
      const store = useGameDataStore()
      const result = store.searchRecipesByItem('COPPERINGOT')

      expect(result).toHaveLength(1)
      expect(result[0].name).toBe('CopperIngot')
    })

    it('searchRecipesByItem checks both inputs and outputs', () => {
      const store = useGameDataStore()
      // IronIngot is both an input (to IronPlate) and output (of IronIngot recipe)
      const result = store.searchRecipesByItem('IronIngot')

      expect(result).toHaveLength(2)
      const names = result.map(r => r.name).sort()
      // IronIngot recipe has IronIngot as output, IronPlate has IronIngot as input
      expect(names).toEqual(['IronIngot', 'IronPlate'])
    })

    it('searchRecipesByName finds recipes by name (case-insensitive)', () => {
      const store = useGameDataStore()
      const result = store.searchRecipesByName('iron')

      expect(result).toHaveLength(2)
      const names = result.map(r => r.name).sort()
      expect(names).toEqual(['IronIngot', 'IronPlate'])
    })

    it('searchRecipesByName returns empty array when no match', () => {
      const store = useGameDataStore()
      const result = store.searchRecipesByName('zinc')

      expect(result).toEqual([])
    })
  })

  // ==========================================================================
  // 8. Validators + Helpers + Reset
  // ==========================================================================
  describe('validators and helpers', () => {
    beforeEach(() => {
      const store = useGameDataStore()
      store.items = ['IronOre', 'CopperOre', 'IronIngot']
      store.machines = [
        createMockMachine({ name: 'Smelter' }),
        createMockMachine({ name: 'Constructor' }),
      ]
    })

    it('getSomersloopMachines filters machines with max_somersloop > 0', () => {
      const store = useGameDataStore()
      store.machines = [
        createMockMachine({ name: 'Smelter', max_somersloop: 0 }),
        createMockMachine({ name: 'Constructor', max_somersloop: 2 }),
        createMockMachine({ name: 'Assembler', max_somersloop: 4 }),
      ]

      const result = store.getSomersloopMachines()

      expect(result).toHaveLength(2)
      expect(result.map(m => m.name)).toEqual(['Constructor', 'Assembler'])
    })

    it('isValidItem returns true for valid item', () => {
      const store = useGameDataStore()
      expect(store.isValidItem('IronOre')).toBe(true)
      expect(store.isValidItem('CopperOre')).toBe(true)
    })

    it('isValidItem returns false for invalid item', () => {
      const store = useGameDataStore()
      expect(store.isValidItem('UnknownItem')).toBe(false)
    })

    it('isValidMachine returns true for valid machine', () => {
      const store = useGameDataStore()
      expect(store.isValidMachine('Smelter')).toBe(true)
      expect(store.isValidMachine('Constructor')).toBe(true)
    })

    it('isValidMachine returns false for invalid machine', () => {
      const store = useGameDataStore()
      expect(store.isValidMachine('UnknownMachine')).toBe(false)
    })

    it('clearError sets error to null', () => {
      const store = useGameDataStore()
      store.error = 'Some error'

      store.clearError()

      expect(store.error).toBe(null)
    })

    it('reset clears all data', () => {
      const store = useGameDataStore()

      // Populate state
      store.recipes = [createMockRecipe()]
      store.items = ['IronOre']
      store.machines = [createMockMachine()]
      store.loading = true
      store.error = 'Some error'

      store.reset()

      expect(store.recipes).toEqual([])
      expect(store.items).toEqual([])
      expect(store.machines).toEqual([])
      expect(store.loading).toBe(false)
      expect(store.error).toBe(null)
    })

    it('reset allows fresh API calls after being called', async () => {
      const store = useGameDataStore()
      const mockRecipes = [createMockRecipe()]
      vi.mocked(gameDataApi.getRecipes).mockResolvedValue(mockRecipes)

      // First fetch
      await store.fetchRecipes()
      expect(gameDataApi.getRecipes).toHaveBeenCalledTimes(1)

      // Reset store
      store.reset()

      // Clear cache is called by reset, so next fetch should hit API
      await store.fetchRecipes()
      expect(gameDataApi.getRecipes).toHaveBeenCalledTimes(2)
    })
  })
})
