import { ref, computed, watch } from 'vue'
import { defineStore } from 'pinia'

/**
 * User preferences interface
 */
interface UserPreferences {
  selectedFactoryId: number | null
  dashboardFilters: {
    state: 'all' | 'overflow' | 'underflow' | 'balanced'
    searchText: string
    sortBy: 'name' | 'balance' | 'state'
    sortOrder: 'asc' | 'desc'
  }
  factoryViewTab: 'production' | 'raw-inputs' | 'power-generation'
  logisticsFilters: {
    transportType: 'all' | string
    sourceFactory: 'all' | number
    destinationFactory: 'all' | number
    searchText: string
  }
  uiPreferences: {
    theme: 'light' | 'dark' | 'auto'
    language: string
    autoRefresh: boolean
    refreshInterval: number // in seconds
  }
}

/**
 * Default preferences
 */
const DEFAULT_PREFERENCES: UserPreferences = {
  selectedFactoryId: null,
  dashboardFilters: {
    state: 'all',
    searchText: '',
    sortBy: 'name',
    sortOrder: 'asc'
  },
  factoryViewTab: 'production',
  logisticsFilters: {
    transportType: 'all',
    sourceFactory: 'all',
    destinationFactory: 'all',
    searchText: ''
  },
  uiPreferences: {
    theme: 'auto',
    language: 'en',
    autoRefresh: false,
    refreshInterval: 30
  }
}

/**
 * Preferences Store
 *
 * Manages user preferences with localStorage persistence.
 * Handles UI state, filters, and user settings that persist across sessions.
 */
export const usePreferencesStore = defineStore('preferences', () => {
  // State
  const preferences = ref<UserPreferences>({ ...DEFAULT_PREFERENCES })
  const loading = ref(false)
  const error = ref<string | null>(null)

  // localStorage key
  const STORAGE_KEY = 'satisflow-preferences'

  // Getters
  const selectedFactoryId = computed(() => preferences.value.selectedFactoryId)

  const dashboardFilters = computed(() => preferences.value.dashboardFilters)

  const factoryViewTab = computed(() => preferences.value.factoryViewTab)

  const logisticsFilters = computed(() => preferences.value.logisticsFilters)

  const uiPreferences = computed(() => preferences.value.uiPreferences)

  const hasValidFactorySelection = computed(() => {
    return preferences.value.selectedFactoryId !== null
  })

  // Actions

  /**
   * Load preferences from localStorage
   */
  const loadPreferences = (): void => {
    loading.value = true
    error.value = null

    try {
      const stored = localStorage.getItem(STORAGE_KEY)

      if (stored) {
        const parsed = JSON.parse(stored) as Partial<UserPreferences>

        // Merge with defaults to ensure all properties exist
        preferences.value = {
          ...DEFAULT_PREFERENCES,
          ...parsed,
          dashboardFilters: {
            ...DEFAULT_PREFERENCES.dashboardFilters,
            ...parsed.dashboardFilters
          },
          logisticsFilters: {
            ...DEFAULT_PREFERENCES.logisticsFilters,
            ...parsed.logisticsFilters
          },
          uiPreferences: {
            ...DEFAULT_PREFERENCES.uiPreferences,
            ...parsed.uiPreferences
          }
        }
      }
    } catch (err) {
      error.value = 'Failed to load preferences from localStorage'
      console.error('Failed to load preferences:', err)
      // Reset to defaults on error
      preferences.value = { ...DEFAULT_PREFERENCES }
    } finally {
      loading.value = false
    }
  }

  /**
   * Save preferences to localStorage
   */
  const savePreferences = (): void => {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(preferences.value))
    } catch (err) {
      error.value = 'Failed to save preferences to localStorage'
      console.error('Failed to save preferences:', err)
    }
  }

  /**
   * Update selected factory ID
   * @param factoryId - The factory ID to set as selected, or null to clear
   */
  const setSelectedFactoryId = (factoryId: number | null): void => {
    preferences.value.selectedFactoryId = factoryId
  }

  /**
   * Update dashboard filters
   * @param filters - The dashboard filters to update
   */
  const updateDashboardFilters = (filters: Partial<UserPreferences['dashboardFilters']>): void => {
    preferences.value.dashboardFilters = {
      ...preferences.value.dashboardFilters,
      ...filters
    }
  }

  /**
   * Reset dashboard filters to defaults
   */
  const resetDashboardFilters = (): void => {
    preferences.value.dashboardFilters = { ...DEFAULT_PREFERENCES.dashboardFilters }
  }

  /**
   * Update factory view tab
   * @param tab - The tab to set as active
   */
  const setFactoryViewTab = (tab: UserPreferences['factoryViewTab']): void => {
    preferences.value.factoryViewTab = tab
  }

  /**
   * Update logistics filters
   * @param filters - The logistics filters to update
   */
  const updateLogisticsFilters = (filters: Partial<UserPreferences['logisticsFilters']>): void => {
    preferences.value.logisticsFilters = {
      ...preferences.value.logisticsFilters,
      ...filters
    }
  }

  /**
   * Reset logistics filters to defaults
   */
  const resetLogisticsFilters = (): void => {
    preferences.value.logisticsFilters = { ...DEFAULT_PREFERENCES.logisticsFilters }
  }

  /**
   * Update UI preferences
   * @param prefs - The UI preferences to update
   */
  const updateUIPreferences = (prefs: Partial<UserPreferences['uiPreferences']>): void => {
    preferences.value.uiPreferences = {
      ...preferences.value.uiPreferences,
      ...prefs
    }
  }

  /**
   * Set theme preference
   * @param theme - The theme to set
   */
  const setTheme = (theme: UserPreferences['uiPreferences']['theme']): void => {
    preferences.value.uiPreferences.theme = theme
  }

  /**
   * Set language preference
   * @param language - The language code to set
   */
  const setLanguage = (language: string): void => {
    preferences.value.uiPreferences.language = language
  }

  /**
   * Set auto refresh preference
   * @param autoRefresh - Whether to enable auto refresh
   */
  const setAutoRefresh = (autoRefresh: boolean): void => {
    preferences.value.uiPreferences.autoRefresh = autoRefresh
  }

  /**
   * Set refresh interval
   * @param interval - The refresh interval in seconds
   */
  const setRefreshInterval = (interval: number): void => {
    preferences.value.uiPreferences.refreshInterval = interval
  }

  /**
   * Reset all preferences to defaults
   */
  const resetPreferences = (): void => {
    preferences.value = { ...DEFAULT_PREFERENCES }
  }

  /**
   * Clear the error state
   */
  const clearError = (): void => {
    error.value = null
  }

  /**
   * Export preferences as JSON
   * @returns JSON string of current preferences
   */
  const exportPreferences = (): string => {
    return JSON.stringify(preferences.value, null, 2)
  }

  /**
   * Import preferences from JSON
   * @param json - JSON string to import preferences from
   * @returns True if import was successful, false otherwise
   */
  const importPreferences = (json: string): boolean => {
    try {
      const imported = JSON.parse(json) as UserPreferences

      // Validate imported data structure
      if (typeof imported === 'object' && imported !== null) {
        preferences.value = {
          ...DEFAULT_PREFERENCES,
          ...imported,
          dashboardFilters: {
            ...DEFAULT_PREFERENCES.dashboardFilters,
            ...imported.dashboardFilters
          },
          logisticsFilters: {
            ...DEFAULT_PREFERENCES.logisticsFilters,
            ...imported.logisticsFilters
          },
          uiPreferences: {
            ...DEFAULT_PREFERENCES.uiPreferences,
            ...imported.uiPreferences
          }
        }
        return true
      }

      return false
    } catch (err) {
      error.value = 'Failed to import preferences: Invalid JSON format'
      console.error('Failed to import preferences:', err)
      return false
    }
  }

  // Auto-save preferences whenever they change
  watch(
    preferences,
    () => {
      savePreferences()
    },
    { deep: true }
  )

  // Load preferences on store initialization
  loadPreferences()

  return {
    // State
    preferences,
    loading,
    error,

    // Getters
    selectedFactoryId,
    dashboardFilters,
    factoryViewTab,
    logisticsFilters,
    uiPreferences,
    hasValidFactorySelection,

    // Actions
    loadPreferences,
    savePreferences,
    setSelectedFactoryId,
    updateDashboardFilters,
    resetDashboardFilters,
    setFactoryViewTab,
    updateLogisticsFilters,
    resetLogisticsFilters,
    updateUIPreferences,
    setTheme,
    setLanguage,
    setAutoRefresh,
    setRefreshInterval,
    resetPreferences,
    clearError,
    exportPreferences,
    importPreferences
  }
})
