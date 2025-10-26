import { afterEach, vi } from 'vitest'

// Create a storage map to simulate localStorage behavior
const storageMap = new Map<string, string>()

// Mock localStorage for tests
const localStorageMock = {
  getItem: vi.fn((key: string) => storageMap.get(key) || null),
  setItem: vi.fn((key: string, value: string) => {
    storageMap.set(key, value)
  }),
  removeItem: vi.fn((key: string) => {
    storageMap.delete(key)
  }),
  clear: vi.fn(() => {
    storageMap.clear()
  }),
  get length() {
    return storageMap.size
  },
  key: vi.fn((index: number) => {
    const keys = Array.from(storageMap.keys())
    return keys[index] || null
  }),
  // Add mockReset functions for compatibility
  mockReset: vi.fn(),
  // Add mockImplementation for better compatibility
  mockImplementation: vi.fn(),
}

Object.defineProperty(window, 'localStorage', {
  value: localStorageMock,
  writable: true,
})

// Helper to reset localStorage mock between tests
export const resetLocalStorageMock = () => {
  storageMap.clear()
  localStorageMock.getItem.mockReset()
  localStorageMock.setItem.mockReset()
  localStorageMock.removeItem.mockReset()
  localStorageMock.clear.mockReset()
  localStorageMock.key.mockReset()
}

// Create a more flexible matchMedia mock
const createMatchMediaMock = (matches: boolean = false, query: string = '') => ({
  matches,
  media: query,
  onchange: null,
  addListener: vi.fn(), // deprecated
  removeListener: vi.fn(), // deprecated
  addEventListener: vi.fn(),
  removeEventListener: vi.fn(),
  dispatchEvent: vi.fn(),
})

// Mock window.matchMedia with a default implementation
const matchMediaMock = vi.fn().mockImplementation((query: string) => {
  // Default to light mode (matches: false for dark query)
  return createMatchMediaMock(false, query)
})

Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: matchMediaMock,
})

// Export helper for tests to control the mock
export const setMatchMediaMatches = (matches: boolean) => {
  matchMediaMock.mockImplementation((query: string) => createMatchMediaMock(matches, query))
}

// Export helper to reset the mock
export const resetMatchMediaMock = () => {
  matchMediaMock.mockImplementation((query: string) => createMatchMediaMock(false, query))
}

// Mock ResizeObserver
global.ResizeObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}))

// Mock IntersectionObserver
global.IntersectionObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}))

// Reset after each test
afterEach(() => {
  // Reset localStorage mock
  resetLocalStorageMock()

  // Cleanup Vue test utils if available
  if (typeof document !== 'undefined') {
    document.body.innerHTML = ''
  }
})
