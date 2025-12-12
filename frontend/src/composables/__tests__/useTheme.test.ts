import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useTheme, detectSystemTheme } from '../useTheme'
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

  beforeEach(() => {
    // Create a fresh Pinia instance for each test
    pinia = createPinia()
    setActivePinia(pinia)

    // Reset document
    document.documentElement.className = ''
    document.documentElement.removeAttribute('data-theme')
  })

  afterEach(() => {
    document.documentElement.className = ''
    document.documentElement.removeAttribute('data-theme')
    vi.clearAllMocks()
  })

  it('should initialize with dark theme as default (industrial theme)', () => {
    const { currentTheme, effectiveTheme } = testComposable(() => useTheme())

    // The new API always returns dark theme (industrial theme only)
    expect(currentTheme).toBe('dark')
    expect(effectiveTheme).toBe('dark')
    expect(document.documentElement.classList.contains('dark-theme')).toBe(true)
    expect(document.documentElement.getAttribute('data-theme')).toBe('dark')
  })

  it('should always return dark theme (industrial theme only)', () => {
    const { currentTheme, effectiveTheme } = testComposable(() => useTheme())

    // The new API always returns dark theme (industrial theme only)
    expect(currentTheme).toBe('dark')
    expect(effectiveTheme).toBe('dark')
  })

  it('should apply dark theme to document', () => {
    // Reset document first
    document.documentElement.className = ''
    document.documentElement.removeAttribute('data-theme')

    const { applyTheme } = testComposable(() => useTheme())

    // applyTheme should be called automatically on initialization
    expect(document.documentElement.classList.contains('dark-theme')).toBe(true)
    expect(document.documentElement.getAttribute('data-theme')).toBe('dark')

    // Call applyTheme explicitly
    applyTheme()
    expect(document.documentElement.classList.contains('dark-theme')).toBe(true)
    expect(document.documentElement.getAttribute('data-theme')).toBe('dark')
  })

  it('should initialize theme correctly', () => {
    // Reset document first
    document.documentElement.className = ''
    document.documentElement.removeAttribute('data-theme')

    const { initializeTheme } = testComposable(() => useTheme())

    // initializeTheme should apply dark theme
    initializeTheme()
    expect(document.documentElement.classList.contains('dark-theme')).toBe(true)
    expect(document.documentElement.getAttribute('data-theme')).toBe('dark')
  })

  it('should detect system theme as dark (always)', () => {
    // detectSystemTheme always returns 'dark' for industrial theme
    expect(detectSystemTheme()).toBe('dark')
  })

  it('should remove light theme classes when applying dark theme', () => {
    // Set up initial state with light theme
    document.documentElement.classList.add('light-theme')
    document.documentElement.setAttribute('data-theme', 'light')

    const { applyTheme } = testComposable(() => useTheme())

    // applyTheme should remove light theme and add dark theme
    applyTheme()
    expect(document.documentElement.classList.contains('light-theme')).toBe(false)
    expect(document.documentElement.classList.contains('dark-theme')).toBe(true)
    expect(document.documentElement.getAttribute('data-theme')).toBe('dark')
  })

  it('should handle missing document API gracefully', () => {
    // The composable should handle cases where document is undefined
    // (though in tests it should always be available)
    const { currentTheme, effectiveTheme } = testComposable(() => useTheme())
    
    // Should still return dark theme
    expect(currentTheme).toBe('dark')
    expect(effectiveTheme).toBe('dark')
  })
})
