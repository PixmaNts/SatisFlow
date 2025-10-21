import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useTheme } from '../useTheme'

describe('useTheme', () => {
  beforeEach(() => {
    // Reset document
    document.documentElement.className = ''
    document.documentElement.removeAttribute('data-theme')

    // Create fresh Pinia instance
    const pinia = createPinia()
    setActivePinia(pinia)
  })

  afterEach(() => {
    document.documentElement.className = ''
    document.documentElement.removeAttribute('data-theme')
    vi.clearAllMocks()
  })

  it('should initialize with auto theme as default', () => {
    const { currentTheme, effectiveTheme } = useTheme()

    expect(currentTheme.value).toBe('auto')
    expect(effectiveTheme.value).toBe('light') // Default to light in tests
  })

  it('should apply dark theme when set', () => {
    const { setTheme, currentTheme, effectiveTheme } = useTheme()

    setTheme('dark')

    expect(currentTheme.value).toBe('dark')
    expect(effectiveTheme.value).toBe('dark')
    expect(document.documentElement.classList.contains('dark-theme')).toBe(true)
    expect(document.documentElement.getAttribute('data-theme')).toBe('dark')
  })

  it('should apply light theme when set', () => {
    const { setTheme, currentTheme, effectiveTheme } = useTheme()

    setTheme('light')

    expect(currentTheme.value).toBe('light')
    expect(effectiveTheme.value).toBe('light')
    expect(document.documentElement.classList.contains('light-theme')).toBe(true)
    expect(document.documentElement.getAttribute('data-theme')).toBe('light')
  })

  it('should apply auto theme when set', () => {
    const { setTheme, currentTheme, effectiveTheme } = useTheme()

    setTheme('auto')

    expect(currentTheme.value).toBe('auto')
    expect(effectiveTheme.value).toBe('light') // Default to light in tests
  })

  it('should toggle between light and dark themes', () => {
    const { toggleTheme, currentTheme, effectiveTheme } = useTheme()

    // Start with auto (default)
    expect(currentTheme.value).toBe('auto')
    expect(effectiveTheme.value).toBe('light')

    // Toggle to dark
    toggleTheme()
    expect(currentTheme.value).toBe('dark')
    expect(effectiveTheme.value).toBe('dark')

    // Toggle back to light
    toggleTheme()
    expect(currentTheme.value).toBe('light')
    expect(effectiveTheme.value).toBe('light')

    // Toggle to dark again
    toggleTheme()
    expect(currentTheme.value).toBe('dark')
    expect(effectiveTheme.value).toBe('dark')
  })

  it('should detect system preference for dark mode', () => {
    // Mock matchMedia to return dark mode preference
    Object.defineProperty(window, 'matchMedia', {
      writable: true,
      value: {
        matches: true,
        media: '(prefers-color-scheme: dark)',
        onchange: null,
        addListener: vi.fn(),
        removeListener: vi.fn(),
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      },
    })

    const { currentTheme, effectiveTheme } = useTheme()

    // With auto theme, should respect system preference
    expect(currentTheme.value).toBe('auto')
    expect(effectiveTheme.value).toBe('dark')

    // But when explicitly set to light, should override
    const { setTheme } = useTheme()
    setTheme('light')
    expect(effectiveTheme.value).toBe('light')
  })

  it('should get system theme correctly', () => {
    // Mock light mode
    Object.defineProperty(window, 'matchMedia', {
      writable: true,
      value: {
        matches: false,
        media: '(prefers-color-scheme: dark)',
        onchange: null,
        addListener: vi.fn(),
        removeListener: vi.fn(),
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      },
    })

    const { getSystemTheme } = useTheme()
    expect(getSystemTheme()).toBe('light')

    // Mock dark mode
    Object.defineProperty(window, 'matchMedia', {
      writable: true,
      value: {
        matches: true,
        media: '(prefers-color-scheme: dark)',
        onchange: null,
        addListener: vi.fn(),
        removeListener: vi.fn(),
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      },
    })

    expect(getSystemTheme()).toBe('dark')
  })

  it('should apply theme correctly', () => {
    const { applyTheme } = useTheme()

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

    const { getSystemTheme } = useTheme()
    expect(getSystemTheme()).toBe('light') // Should default to light

    // Restore
    window.matchMedia = originalMatchMedia
  })
})
