// Removed onMounted, onUnmounted - not needed for module-level event listeners

export interface KeyboardShortcut {
  key: string
  ctrlKey?: boolean
  altKey?: boolean
  shiftKey?: boolean
  metaKey?: boolean
  description: string
  handler: (event: KeyboardEvent) => void
  preventDefault?: boolean
  stopPropagation?: boolean
}

interface KeyboardShortcutsState {
  shortcuts: Map<string, KeyboardShortcut>
  isEnabled: boolean
}

const state: KeyboardShortcutsState = {
  shortcuts: new Map(),
  isEnabled: true
}

/**
 * Generate a unique key for a shortcut combination
 */
const getShortcutKey = (shortcut: Omit<KeyboardShortcut, 'description' | 'handler'>): string => {
  return [
    shortcut.ctrlKey ? 'ctrl' : '',
    shortcut.altKey ? 'alt' : '',
    shortcut.shiftKey ? 'shift' : '',
    shortcut.metaKey ? 'meta' : '',
    shortcut.key.toLowerCase()
  ].filter(Boolean).join('+')
}

/**
 * Check if a keyboard event matches a shortcut
 */
const matchesShortcut = (event: KeyboardEvent, shortcut: KeyboardShortcut): boolean => {
  return (
    event.key.toLowerCase() === shortcut.key.toLowerCase() &&
    !!event.ctrlKey === !!shortcut.ctrlKey &&
    !!event.altKey === !!shortcut.altKey &&
    !!event.shiftKey === !!shortcut.shiftKey &&
    !!event.metaKey === !!shortcut.metaKey
  )
}

/**
 * Handle keyboard events
 */
const handleKeyDown = (event: KeyboardEvent) => {
  if (!state.isEnabled) return

  // Ignore shortcuts when user is typing in input fields
  const target = event.target as HTMLElement
  if (
    target.tagName === 'INPUT' ||
    target.tagName === 'TEXTAREA' ||
    target.contentEditable === 'true'
  ) {
    return
  }

  // Find matching shortcut
  for (const shortcut of state.shortcuts.values()) {
    if (matchesShortcut(event, shortcut)) {
      if (shortcut.preventDefault) {
        event.preventDefault()
      }
      if (shortcut.stopPropagation) {
        event.stopPropagation()
      }
      shortcut.handler(event)
      break
    }
  }
}

/**
 * Keyboard Shortcuts Composable
 *
 * Provides a way to register and manage keyboard shortcuts
 * throughout the application.
 */
export function useKeyboardShortcuts() {
  /**
   * Register a new keyboard shortcut
   */
  const registerShortcut = (shortcut: KeyboardShortcut): () => void => {
    const key = getShortcutKey(shortcut)
    state.shortcuts.set(key, shortcut)

    // Return unregister function
    return () => {
      state.shortcuts.delete(key)
    }
  }

  /**
   * Register multiple shortcuts at once
   */
  const registerShortcuts = (shortcuts: KeyboardShortcut[]): (() => void) => {
    const unregisterFunctions = shortcuts.map(registerShortcut)

    // Return function to unregister all
    return () => {
      unregisterFunctions.forEach(fn => fn())
    }
  }

  /**
   * Unregister a shortcut
   */
  const unregisterShortcut = (key: string, ctrlKey = false, altKey = false, shiftKey = false, metaKey = false): void => {
    const shortcutKey = getShortcutKey({ key, ctrlKey, altKey, shiftKey, metaKey })
    state.shortcuts.delete(shortcutKey)
  }

  /**
   * Enable/disable all shortcuts
   */
  const setEnabled = (enabled: boolean): void => {
    state.isEnabled = enabled
  }

  /**
   * Get all registered shortcuts
   */
  const getShortcuts = (): KeyboardShortcut[] => {
    return Array.from(state.shortcuts.values())
  }

  /**
   * Get shortcuts as a formatted list for display
   */
  const getShortcutsList = (): Array<{ key: string; description: string }> => {
    return Array.from(state.shortcuts.values()).map(shortcut => ({
      key: getShortcutKey(shortcut)
        .replace('ctrl', 'Ctrl')
        .replace('alt', 'Alt')
        .replace('shift', 'Shift')
        .replace('meta', 'Cmd')
        .split('+')
        .map(part => part.length === 1 ? part.toUpperCase() : part)
        .join(' + '),
      description: shortcut.description
    }))
  }

  return {
    registerShortcut,
    registerShortcuts,
    unregisterShortcut,
    setEnabled,
    getShortcuts,
    getShortcutsList
  }
}

/**
 * Global keyboard shortcuts instance
 */
export const keyboardShortcuts = useKeyboardShortcuts()

// Set up global event listeners at module level
if (typeof document !== 'undefined') {
  document.addEventListener('keydown', handleKeyDown)
}

/**
 * Common keyboard shortcuts
 */
export const commonShortcuts = {
  // Navigation
  goToDashboard: {
    key: 'g',
    shiftKey: true,
    description: 'Go to Dashboard',
    handler: () => {
      // Navigate to dashboard
      window.location.hash = '#/'
    }
  },

  goToFactory: {
    key: 'f',
    shiftKey: true,
    description: 'Go to Factory',
    handler: () => {
      // Navigate to factory
      window.location.hash = '#/factory'
    }
  },

  goToLogistics: {
    key: 'l',
    shiftKey: true,
    description: 'Go to Logistics',
    handler: () => {
      // Navigate to logistics
      window.location.hash = '#/logistics'
    }
  },

  // Actions
  refresh: {
    key: 'r',
    ctrlKey: true,
    description: 'Refresh current page',
    handler: () => {
      window.location.reload()
    }
  },

  save: {
    key: 's',
    ctrlKey: true,
    description: 'Save current data',
    handler: () => {
      // Trigger save action
      window.dispatchEvent(new CustomEvent('app-save'))
    }
  },

  // Help
  showHelp: {
    key: '?',
    shiftKey: true,
    description: 'Show keyboard shortcuts help',
    preventDefault: true,
    stopPropagation: true,
    handler: () => {
      // Show help modal
      window.dispatchEvent(new CustomEvent('app-show-help'))
    }
  },


  // Escape
  escape: {
    key: 'Escape',
    description: 'Close modal or cancel action',
    handler: () => {
      // Close modal or cancel action
      window.dispatchEvent(new CustomEvent('app-escape'))
    }
  },

  // Command Palette
  openCommandPalette: {
    key: 'k',
    ctrlKey: true,
    description: 'Open command palette',
    preventDefault: true,
    stopPropagation: true,
    handler: () => {
      window.dispatchEvent(new CustomEvent('app-open-command-palette'))
    }
  },

  // Quick Create (context-aware)
  createNew: {
    key: 'n',
    ctrlKey: true,
    description: 'Create new item (context-aware)',
    handler: () => {
      window.dispatchEvent(new CustomEvent('app-create-new'))
    }
  },

  // Search/Focus
  focusSearch: {
    key: '/',
    ctrlKey: true,
    description: 'Focus search input',
    handler: () => {
      window.dispatchEvent(new CustomEvent('app-focus-search'))
    }
  }
}

/**
 * Register common shortcuts
 */
export const registerCommonShortcuts = (): (() => void) => {
  return keyboardShortcuts.registerShortcuts(Object.values(commonShortcuts))
}
