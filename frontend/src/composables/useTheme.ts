/**
 * Theme composable - Industrial dark theme only
 *
 * Always applies the industrial dark theme. No theme switching.
 */
export function useTheme() {
  /**
   * Apply industrial dark theme to document root element
   */
  const applyTheme = () => {
    if (typeof document !== 'undefined') {
      const root = document.documentElement

      // Remove any existing theme classes
      root.classList.remove('light-theme', 'dark-theme')

      // Always apply dark theme (industrial)
      root.classList.add('dark-theme')
      root.setAttribute('data-theme', 'dark')
    }
  }

  /**
   * Initialize theme system - always dark
   */
  const initializeTheme = () => {
    applyTheme()
  }

  // Apply theme immediately (synchronously)
  applyTheme()

  // Return simplified API (for backwards compatibility if needed)
  return {
    currentTheme: 'dark' as const,
    effectiveTheme: 'dark' as const,
    applyTheme,
    initializeTheme
  }
}

/**
 * Always returns dark theme (industrial)
 */
export function detectSystemTheme(): 'dark' {
  return 'dark'
}

/**
 * Theme-aware CSS custom property helper - always returns dark color
 */
export function useThemeAwareColor(
  _lightColor: string,
  darkColor: string
) {
  // Always return dark color for industrial theme
  return darkColor
}

