import { describe, it, expect, beforeEach, afterEach, beforeAll, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useTheme } from '../useTheme'
import { setMatchMediaMatches, resetMatchMediaMock } from '@/test-utils/setup'
import { mount } from '@vue/test-utils'
import { defineComponent } from 'vue'

// Helper to test composables in a proper component context
function testComposable<T>(composable: () => T): T {
  let result: T
  const TestComponent = defineComponent({
    setup() {
      result = composable()
      return () => null
    }
  })

  mount(TestComponent, {
    global: {
      plugins: [createPinia()]
    }
  })

  return result!
}

describe('useTheme', () => {
  let pinia: ReturnType<typeof createPinia>

  beforeAll(() => {
    // Create a single Pinia instance for all tests
    pinia = createPinia()
    setActivePinia(pinia)
  })

  beforeEach(() => {
    // Reset document
    document.documentElement.className = ''
    document.documentElement.removeAttribute('data-theme')

    // Reset matchMedia mock to default (light mode)
    resetMatchMediaMock()

    // Clear localStorage to reset preferences
    localStorage.clear()
  })

  afterEach(() => {
    document.documentElement.className = ''
    document.documentElement.removeAttribute('data-theme')
    vi.clearAllMocks()
    resetMatchMediaMock()
    localStorage.clear()
  })

  it('should initialize with auto theme as default', () => {
    const { currentTheme, effectiveTheme } = testComposable(() => useTheme())

    expect(currentTheme.value).toBe('auto')
    expect(effectiveTheme.value).toBe('light') // Default to light in tests
  })

  it('should apply dark theme when set', async () => {
    let themeApi: ReturnType<typeof useTheme>

    const TestComponent = defineComponent({
      setup() {
        themeApi = useTheme()
        return () => null
      }
    })

    mount(TestComponent, {
      global: {
        plugins: [pinia]
      }
    })

    const { setTheme, currentTheme, effectiveTheme } = themeApi!

    setTheme('dark')

    // Wait a tick for the watcher to apply the theme
    await new Promise(resolve => setTimeout(resolve, 0))

    expect(currentTheme.value).toBe('dark')
    expect(effectiveTheme.value).toBe('dark')
    expect(document.documentElement.classList.contains('dark-theme')).toBe(true)
    expect(document.documentElement.getAttribute('data-theme')).toBe('dark')
  })

  it('should apply light theme when set', async () => {
    let themeApi: ReturnType<typeof useTheme>

    const TestComponent = defineComponent({
      setup() {
        themeApi = useTheme()
        return () => null
      }
    })

    mount(TestComponent, {
      global: {
        plugins: [pinia]
      }
    })

    const { setTheme, currentTheme, effectiveTheme } = themeApi!

    setTheme('light')

    // Wait a tick for the watcher to apply the theme
    await new Promise(resolve => setTimeout(resolve, 0))

    expect(currentTheme.value).toBe('light')
    expect(effectiveTheme.value).toBe('light')
    expect(document.documentElement.classList.contains('light-theme')).toBe(true)
    expect(document.documentElement.getAttribute('data-theme')).toBe('light')
  })

  it('should apply auto theme when set', async () => {
    let themeApi: ReturnType<typeof useTheme>

    const TestComponent = defineComponent({
      setup() {
        themeApi = useTheme()
        return () => null
      }
    })

    mount(TestComponent, {
      global: {
        plugins: [pinia]
      }
    })

    const { setTheme, currentTheme, effectiveTheme } = themeApi!

    setTheme('auto')

    // Wait a tick for the watcher to apply the theme
    await new Promise(resolve => setTimeout(resolve, 0))

    expect(currentTheme.value).toBe('auto')
    expect(effectiveTheme.value).toBe('light') // Default to light in tests
  })

  it('should toggle between light and dark themes', async () => {
    let themeApi: ReturnType<typeof useTheme>

    const TestComponent = defineComponent({
      setup() {
        themeApi = useTheme()
        return () => null
      }
    })

    mount(TestComponent, {
      global: {
        plugins: [pinia]
      }
    })

    const { toggleTheme, currentTheme, effectiveTheme, setTheme } = themeApi!

    // Start with auto (default)
    expect(currentTheme.value).toBe('auto')
    expect(effectiveTheme.value).toBe('light')

    // Set to light first to have a known starting point
    setTheme('light')
    await new Promise(resolve => setTimeout(resolve, 0))
    expect(currentTheme.value).toBe('light')
    expect(effectiveTheme.value).toBe('light')

    // Toggle to dark
    toggleTheme()
    await new Promise(resolve => setTimeout(resolve, 0))
    expect(currentTheme.value).toBe('dark')
    expect(effectiveTheme.value).toBe('dark')

    // Toggle back to light
    toggleTheme()
    await new Promise(resolve => setTimeout(resolve, 0))
    expect(currentTheme.value).toBe('light')
    expect(effectiveTheme.value).toBe('light')

    // Toggle to dark again
    toggleTheme()
    await new Promise(resolve => setTimeout(resolve, 0))
    expect(currentTheme.value).toBe('dark')
    expect(effectiveTheme.value).toBe('dark')
  })

  it('should detect system preference for dark mode', async () => {
    // Mock matchMedia to return dark mode preference
    setMatchMediaMatches(true)

    let themeApi: ReturnType<typeof useTheme>

    const TestComponent = defineComponent({
      setup() {
        themeApi = useTheme()
        return () => null
      }
    })

    mount(TestComponent, {
      global: {
        plugins: [pinia]
      }
    })

    const { currentTheme, effectiveTheme, setTheme } = themeApi!

    // Reset to auto first
    setTheme('auto')
    await new Promise(resolve => setTimeout(resolve, 0))

    // With auto theme, should respect system preference
    expect(currentTheme.value).toBe('auto')
    expect(effectiveTheme.value).toBe('dark')

    // But when explicitly set to light, should override
    setTheme('light')
    await new Promise(resolve => setTimeout(resolve, 0))
    expect(effectiveTheme.value).toBe('light')
  })

  it('should get system theme correctly', () => {
    // Mock light mode
    setMatchMediaMatches(false)
    const { getSystemTheme } = testComposable(() => useTheme())
    expect(getSystemTheme()).toBe('light')

    // Mock dark mode
    setMatchMediaMatches(true)
    expect(getSystemTheme()).toBe('dark')
  })

  it('should apply theme correctly', () => {
    const { applyTheme } = testComposable(() => useTheme())

    applyTheme('dark')
    expect(document.documentElement.classList.contains('dark-theme')).toBe(true)
    expect(document.documentElement.getAttribute('data-theme')).toBe('dark')

    applyTheme('light')
    expect(document.documentElement.classList.contains('light-theme')).toBe(true)
    expect(document.documentElement.getAttribute('data-theme')).toBe('light')
  })

  it('should handle missing window API gracefully', () => {
    // Mock window without matchMedia
    const originalMatchMedia = window.matchMedia
    delete (window as unknown as Record<string, unknown>).matchMedia

    const { getSystemTheme } = testComposable(() => useTheme())
    expect(getSystemTheme()).toBe('light') // Should default to light

    // Restore
    window.matchMedia = originalMatchMedia
  })
})
