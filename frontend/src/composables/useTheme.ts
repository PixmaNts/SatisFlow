import { computed, watch, onMounted } from 'vue'
import { usePreferencesStore } from '@/stores/preferences'

/**
 * Theme composable for managing application theme
 *
 * Provides reactive theme management with automatic CSS class application
 * and system preference detection for 'auto' theme mode.
 */
export function useTheme() {
  const preferencesStore = usePreferencesStore()

  // Computed property for current theme
  const currentTheme = computed(() => preferencesStore.uiPreferences.theme)

  // Computed property for effective theme (resolves 'auto' to system preference)
  const effectiveTheme = computed(() => {
    if (currentTheme.value === 'auto') {
      return getSystemTheme()
    }
    return currentTheme.value
  })

  /**
   * Get system theme preference
   */
  const getSystemTheme = (): 'light' | 'dark' => {
    if (typeof window !== 'undefined' && window.matchMedia) {
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
    }
    return 'light'
  }

  /**
   * Apply theme to document root element
   */
  const applyTheme = (theme: 'light' | 'dark') => {
    if (typeof document !== 'undefined') {
      const root = document.documentElement

      // Remove existing theme classes
      root.classList.remove('light-theme', 'dark-theme')

      // Add new theme class
      root.classList.add(`${theme}-theme`)

      // Update data-theme attribute for CSS selectors
      root.setAttribute('data-theme', theme)
    }
  }

  /**
   * Set theme and update preferences
   */
  const setTheme = (theme: 'light' | 'dark' | 'auto') => {
    preferencesStore.setTheme(theme)
  }

  /**
   * Toggle between light and dark themes
   */
  const toggleTheme = () => {
    const newTheme = currentTheme.value === 'light' ? 'dark' : 'light'
    setTheme(newTheme)
  }

  /**
   * Initialize theme system
   */
  const initializeTheme = () => {
    // Apply initial theme
    applyTheme(effectiveTheme.value)

    // Set up system theme listener for 'auto' mode
    if (typeof window !== 'undefined' && window.matchMedia) {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

      const handleSystemThemeChange = () => {
        if (currentTheme.value === 'auto') {
          applyTheme(effectiveTheme.value)
        }
      }

      // Add event listener (with fallback for older browsers)
      if (mediaQuery.addEventListener) {
        mediaQuery.addEventListener('change', handleSystemThemeChange)
      } else if (mediaQuery.addListener) {
        // Fallback for older browsers
        mediaQuery.addListener(handleSystemThemeChange)
      }

      // Return cleanup function
      return () => {
        if (mediaQuery.removeEventListener) {
          mediaQuery.removeEventListener('change', handleSystemThemeChange)
        } else if (mediaQuery.removeListener) {
          mediaQuery.removeListener(handleSystemThemeChange)
        }
      }
    }

    return () => {} // No-op cleanup if no media query support
  }

  // Watch for theme changes
  watch(
    effectiveTheme,
    (newTheme) => {
      applyTheme(newTheme)
    },
    { immediate: true }
  )

  // Auto-initialize on mount
  onMounted(() => {
    initializeTheme()
  })

  // Return theme management API
  return {
    currentTheme,
    effectiveTheme,
    setTheme,
    toggleTheme,
    applyTheme,
    getSystemTheme
  }
}

/**
 * Simple theme detection utility
 */
export function detectSystemTheme(): 'light' | 'dark' {
  if (typeof window !== 'undefined' && window.matchMedia) {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
  }
  return 'light'
}

/**
 * Theme-aware CSS custom property helper
 */
export function useThemeAwareColor(
  lightColor: string,
  darkColor: string
) {
  const { effectiveTheme } = useTheme()

  return computed(() => {
    return effectiveTheme.value === 'dark' ? darkColor : lightColor
  })
}
