import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import { gameData as gameDataApi } from '@/api/endpoints'
import type { RecipeInfo, MachineInfo, ItemInfo } from '@/api/types'
import { handleApiError } from '@/api'

/**
 * Game Data Store
 *
 * Manages static game data (recipes, items, machines) with caching.
 * Integrates with the game data API endpoints and caches data to avoid repeated requests.
 */
export const useGameDataStore = defineStore('gameData', () => {
  // State
  const recipes = ref<RecipeInfo[]>([])
  const items = ref<ItemInfo[]>([])
  const machines = ref<MachineInfo[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Cache tracking
  const lastFetchTime = ref<Record<string, number>>({
    recipes: 0,
    items: 0,
    machines: 0
  })

  // Cache TTL in milliseconds (5 minutes)
  const CACHE_TTL = 5 * 60 * 1000

  // Getters
  const recipesByMachine = computed(() => {
    const result: Record<string, RecipeInfo[]> = {}

    recipes.value.forEach(recipe => {
      const machine = recipe.machine
      if (!result[machine]) {
        result[machine] = []
      }
      result[machine].push(recipe)
    })

    return result
  })

  const itemsByName = computed(() => {
    const result: Record<string, ItemInfo> = {}

    items.value.forEach(item => {
      result[item] = item
    })

    return result
  })

  const machinesByType = computed(() => {
    const result: Record<string, MachineInfo> = {}

    machines.value.forEach(machine => {
      result[machine.name] = machine
    })

    return result
  })

  const machineNames = computed(() => {
    return machines.value.map(m => m.name)
  })

  const itemNames = computed(() => {
    return items.value
  })

  const recipeNames = computed(() => {
    return recipes.value.map(r => r.name)
  })

  // Helper functions
  const isCacheValid = (cacheKey: string): boolean => {
    const lastFetch = lastFetchTime.value[cacheKey]
    if (!lastFetch) return false

    const now = Date.now()
    return (now - lastFetch) < CACHE_TTL
  }

  const updateCacheTime = (cacheKey: string): void => {
    lastFetchTime.value[cacheKey] = Date.now()
  }

  // Actions

  /**
   * Fetch all available recipes (with caching)
   */
  const fetchRecipes = async (forceRefresh = false): Promise<void> => {
    // Check cache first
    if (!forceRefresh && isCacheValid('recipes')) {
      return
    }

    loading.value = true
    error.value = null

    try {
      const data = await gameDataApi.getRecipes()
      recipes.value = data
      updateCacheTime('recipes')
    } catch (err) {
      error.value = handleApiError(err)
      console.error('Failed to fetch recipes:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * Fetch all available items (with caching)
   */
  const fetchItems = async (forceRefresh = false): Promise<void> => {
    // Check cache first
    if (!forceRefresh && isCacheValid('items')) {
      return
    }

    loading.value = true
    error.value = null

    try {
      const data = await gameDataApi.getItems()
      items.value = data
      updateCacheTime('items')
    } catch (err) {
      error.value = handleApiError(err)
      console.error('Failed to fetch items:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * Fetch all machine types with their specifications (with caching)
   */
  const fetchMachines = async (forceRefresh = false): Promise<void> => {
    // Check cache first
    if (!forceRefresh && isCacheValid('machines')) {
      return
    }

    loading.value = true
    error.value = null

    try {
      const data = await gameDataApi.getMachines()
      machines.value = data
      updateCacheTime('machines')
    } catch (err) {
      error.value = handleApiError(err)
      console.error('Failed to fetch machines:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * Fetch all game data at once (with caching)
   */
  const fetchAllData = async (forceRefresh = false): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      await Promise.all([
        fetchRecipes(forceRefresh),
        fetchItems(forceRefresh),
        fetchMachines(forceRefresh)
      ])
    } catch (err) {
      error.value = handleApiError(err)
      console.error('Failed to fetch game data:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * Get recipes for a specific machine
   * @param machineName - The machine name to get recipes for
   * @returns Array of recipes for the specified machine
   */
  const getRecipesByMachine = (machineName: string): RecipeInfo[] => {
    return recipesByMachine.value[machineName] || []
  }

  /**
   * Get recipe by name
   * @param recipeName - The recipe name to search for
   * @returns The recipe or null if not found
   */
  const getRecipeByName = (recipeName: string): RecipeInfo | null => {
    return recipes.value.find(recipe => recipe.name === recipeName) || null
  }

  /**
   * Get machine by name
   * @param machineName - The machine name to search for
   * @returns The machine or null if not found
   */
  const getMachineByName = (machineName: string): MachineInfo | null => {
    return machines.value.find(machine => machine.name === machineName) || null
  }

  /**
   * Search recipes by input or output items
   * @param itemName - The item name to search for
   * @returns Array of recipes that use or produce the specified item
   */
  const searchRecipesByItem = (itemName: string): RecipeInfo[] => {
    const searchLower = itemName.toLowerCase()
    return recipes.value.filter(recipe =>
      recipe.inputs.some(input => input.item.toLowerCase() === searchLower) ||
      recipe.outputs.some(output => output.item.toLowerCase() === searchLower)
    )
  }

  /**
   * Search recipes by name
   * @param searchText - The text to search for in recipe names
   * @returns Array of recipes matching the search
   */
  const searchRecipesByName = (searchText: string): RecipeInfo[] => {
    const searchLower = searchText.toLowerCase()
    return recipes.value.filter(recipe =>
      recipe.name.toLowerCase().includes(searchLower)
    )
  }

  /**
   * Get machines that can use Somersloops
   * @returns Array of machines with max_somersloop > 0
   */
  const getSomersloopMachines = (): MachineInfo[] => {
    return machines.value.filter(machine => machine.max_somersloop > 0)
  }

  /**
   * Get max Somersloop count for a machine
   * @param machineName - The machine name
   * @returns Maximum number of Somersloops for the machine
   */
  const getMaxSomersloops = (machineName: string): number => {
    const machine = getMachineByName(machineName)
    return machine?.max_somersloop || 0
  }

  /**
   * Get base power consumption for a machine
   * @param machineName - The machine name
   * @returns Base power consumption in MW
   */
  const getBasePower = (machineName: string): number => {
    const machine = getMachineByName(machineName)
    return machine?.base_power || 0
  }

  /**
   * Check if an item exists in the game data
   * @param itemName - The item name to check
   * @returns True if the item exists, false otherwise
   */
  const isValidItem = (itemName: string): boolean => {
    return items.value.includes(itemName as ItemInfo)
  }

  /**
   * Check if a machine exists in the game data
   * @param machineName - The machine name to check
   * @returns True if the machine exists, false otherwise
   */
  const isValidMachine = (machineName: string): boolean => {
    return machines.value.some(machine => machine.name === machineName)
  }

  /**
   * Clear the error state
   */
  const clearError = (): void => {
    error.value = null
  }

  /**
   * Clear all cached data
   */
  const clearCache = (): void => {
    lastFetchTime.value = {
      recipes: 0,
      items: 0,
      machines: 0
    }
  }

  /**
   * Reset the store to its initial state
   */
  const reset = (): void => {
    recipes.value = []
    items.value = []
    machines.value = []
    loading.value = false
    error.value = null
    clearCache()
  }

  return {
    // State
    recipes,
    items,
    machines,
    loading,
    error,

    // Getters
    recipesByMachine,
    itemsByName,
    machinesByType,
    machineNames,
    itemNames,
    recipeNames,

    // Actions
    fetchRecipes,
    fetchItems,
    fetchMachines,
    fetchAllData,
    getRecipesByMachine,
    getRecipeByName,
    getMachineByName,
    searchRecipesByItem,
    searchRecipesByName,
    getSomersloopMachines,
    getMaxSomersloops,
    getBasePower,
    isValidItem,
    isValidMachine,
    clearError,
    clearCache,
    reset
  }
})
