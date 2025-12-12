<template>
  <Modal
    :show="show"
    title="Keyboard Shortcuts"
    @close="handleClose"
  >
    <div class="shortcuts-modal">
      <div
        v-for="category in groupedShortcuts"
        :key="category.name"
        class="shortcut-category"
      >
        <h3 class="category-title">{{ category.name }}</h3>
        <div class="shortcuts-list">
          <div
            v-for="shortcut in category.shortcuts"
            :key="shortcut.key"
            class="shortcut-item"
          >
            <div class="shortcut-keys">
              <kbd
                v-for="(key, idx) in shortcut.keys"
                :key="idx"
                class="shortcut-key"
              >
                {{ key }}
              </kbd>
            </div>
            <span class="shortcut-description">{{ shortcut.description }}</span>
          </div>
        </div>
      </div>

      <div class="shortcuts-footer">
        <p class="footer-note">
          Press <kbd class="shortcut-key">?</kbd> or <kbd class="shortcut-key">Shift</kbd> + <kbd class="shortcut-key">?</kbd> to open this help at any time.
        </p>
      </div>
    </div>
  </Modal>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import Modal from './Modal.vue'
import { keyboardShortcuts } from '@/composables/useKeyboardShortcuts'

interface Props {
  show: boolean
}

interface Emits {
  (e: 'close'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

interface ShortcutGroup {
  name: string
  shortcuts: Array<{ key: string; keys: string[]; description: string }>
}

// Get all registered shortcuts
const allShortcuts = computed(() => keyboardShortcuts.getShortcutsList())

// Group shortcuts by category
const groupedShortcuts = computed((): ShortcutGroup[] => {
  const shortcuts = allShortcuts.value.map(item => {
    const keys = item.key.split(' + ').map(k => k.trim())
    return {
      key: item.key,
      keys,
      description: item.description
    }
  })

  // Categorize shortcuts
  const categories: Record<string, ShortcutGroup> = {
    Navigation: {
      name: 'Navigation',
      shortcuts: shortcuts.filter(s =>
        s.description.toLowerCase().includes('go to') ||
        s.description.toLowerCase().includes('navigate')
      )
    },
    Actions: {
      name: 'Actions',
      shortcuts: shortcuts.filter(s =>
        s.description.toLowerCase().includes('create') ||
        s.description.toLowerCase().includes('new') ||
        s.description.toLowerCase().includes('edit') ||
        s.description.toLowerCase().includes('delete') ||
        s.description.toLowerCase().includes('duplicate') ||
        s.description.toLowerCase().includes('save') ||
        s.description.toLowerCase().includes('refresh')
      )
    },
    Tables: {
      name: 'Tables',
      shortcuts: shortcuts.filter(s =>
        s.description.toLowerCase().includes('table') ||
        s.description.toLowerCase().includes('row') ||
        s.description.toLowerCase().includes('column') ||
        s.description.toLowerCase().includes('filter') ||
        s.description.toLowerCase().includes('search')
      )
    },
    Forms: {
      name: 'Forms',
      shortcuts: shortcuts.filter(s =>
        s.description.toLowerCase().includes('form') ||
        s.description.toLowerCase().includes('submit') ||
        s.description.toLowerCase().includes('cancel')
      )
    },
    General: {
      name: 'General',
      shortcuts: shortcuts.filter(s =>
        !s.description.toLowerCase().includes('go to') &&
        !s.description.toLowerCase().includes('navigate') &&
        !s.description.toLowerCase().includes('create') &&
        !s.description.toLowerCase().includes('new') &&
        !s.description.toLowerCase().includes('edit') &&
        !s.description.toLowerCase().includes('delete') &&
        !s.description.toLowerCase().includes('duplicate') &&
        !s.description.toLowerCase().includes('save') &&
        !s.description.toLowerCase().includes('refresh') &&
        !s.description.toLowerCase().includes('table') &&
        !s.description.toLowerCase().includes('row') &&
        !s.description.toLowerCase().includes('column') &&
        !s.description.toLowerCase().includes('filter') &&
        !s.description.toLowerCase().includes('search') &&
        !s.description.toLowerCase().includes('form') &&
        !s.description.toLowerCase().includes('submit') &&
        !s.description.toLowerCase().includes('cancel')
      )
    }
  }

  // Return only non-empty categories
  return Object.values(categories).filter(cat => cat.shortcuts.length > 0)
})

const handleClose = () => {
  emit('close')
}
</script>

<style scoped lang="scss">
.shortcuts-modal {
  max-width: 600px;
  max-height: 70vh;
  overflow-y: auto;
}

.shortcut-category {
  margin-bottom: var(--spacing-xl, 1.25rem);

  &:last-of-type {
    margin-bottom: 0;
  }
}

.category-title {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-gray-900, #111827);
  margin: 0 0 var(--spacing-md, 0.75rem) 0;
  padding-bottom: var(--spacing-xs, 0.25rem);
  border-bottom: 2px solid var(--color-gray-200, #e5e7eb);
}

.shortcuts-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm, 0.5rem);
}

.shortcut-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-lg, 1rem);
  padding: var(--spacing-sm, 0.5rem) 0;
}

.shortcut-keys {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
  flex-shrink: 0;
}

.shortcut-key {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 24px;
  height: 24px;
  padding: 0 var(--spacing-xs, 0.25rem);
  background: linear-gradient(135deg, #f3f4f6 0%, #e5e7eb 100%);
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-md, 0.375rem);
  font-size: var(--font-size-xs, 0.75rem);
  font-weight: var(--font-weight-semibold, 600);
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, 'Liberation Mono', monospace;
  color: var(--color-gray-900, #111827);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  line-height: 1;
}

.shortcut-description {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-gray-700, #374151);
  text-align: right;
  flex: 1;
}

.shortcuts-footer {
  margin-top: var(--spacing-xl, 1.25rem);
  padding-top: var(--spacing-lg, 1rem);
  border-top: 1px solid var(--color-gray-200, #e5e7eb);
}

.footer-note {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-gray-600, #4b5563);
  margin: 0;
  text-align: center;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-xs, 0.25rem);
}

// Dark theme support
:global(.dark-theme) {
  .category-title {
    color: var(--color-gray-100, #f3f4f6);
    border-bottom-color: var(--color-gray-700, #374151);
  }

  .shortcut-key {
    background: linear-gradient(135deg, #374151 0%, #1f2937 100%);
    border-color: var(--color-gray-600, #4b5563);
    color: var(--color-gray-100, #f3f4f6);
  }

  .shortcut-description {
    color: var(--color-gray-300, #d1d5db);
  }

  .shortcuts-footer {
    border-top-color: var(--color-gray-700, #374151);
  }

  .footer-note {
    color: var(--color-gray-400, #9ca3af);
  }
}
</style>



