<template>
  <Teleport to="body">
    <Transition name="command-palette">
      <div
        v-if="show"
        class="command-palette-overlay"
        @click.self="handleClose"
      >
        <div
          ref="paletteRef"
          class="command-palette"
          @keydown="handleKeyDown"
        >
          <div class="command-palette-header">
            <div class="search-icon">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path
                  d="M21 21L15 15M17 10C17 13.866 13.866 17 10 17C6.13401 17 3 13.866 3 10C3 6.13401 6.13401 3 10 3C13.866 3 17 6.13401 17 10Z"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                />
              </svg>
            </div>
            <input
              ref="searchInputRef"
              v-model="searchQuery"
              type="text"
              class="command-palette-input"
              placeholder="Type a command or search..."
              autocomplete="off"
              @input="handleSearch"
            />
            <kbd class="command-palette-hint">Esc</kbd>
          </div>

          <div v-if="filteredCommands.length > 0" class="command-palette-list">
            <button
              v-for="(command, index) in filteredCommands"
              :key="command.id"
              type="button"
              class="command-item"
              :class="{ 'command-item-selected': selectedIndex === index }"
              @click="executeCommand(command)"
            >
              <component :is="command.icon" v-if="command.icon" class="command-icon" />
              <div class="command-content">
                <div class="command-title">{{ command.title }}</div>
                <div v-if="command.description" class="command-description">{{ command.description }}</div>
              </div>
              <div v-if="command.shortcut" class="command-shortcut">
                <kbd
                  v-for="(key, idx) in command.shortcut.split('+')"
                  :key="idx"
                  class="command-key"
                >
                  {{ key.trim() }}
                </kbd>
              </div>
            </button>
          </div>

          <div v-else class="command-palette-empty">
            <p class="empty-text">No commands found</p>
            <p class="empty-hint">Try a different search term</p>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'

export interface Command {
  id: string
  title: string
  description?: string
  icon?: any
  shortcut?: string
  category?: string
  keywords?: string[]
  action: () => void
}

interface Props {
  show: boolean
  commands: Command[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  close: []
  'command-execute': [command: Command]
}>()

const router = useRouter()
const searchQuery = ref('')
const selectedIndex = ref(0)
const paletteRef = ref<HTMLElement>()
const searchInputRef = ref<HTMLInputElement>()

const filteredCommands = computed(() => {
  if (!searchQuery.value.trim()) {
    return props.commands
  }

  const query = searchQuery.value.toLowerCase()
  return props.commands.filter(command => {
    const titleMatch = command.title.toLowerCase().includes(query)
    const descriptionMatch = command.description?.toLowerCase().includes(query)
    const keywordMatch = command.keywords?.some(kw => kw.toLowerCase().includes(query))
    return titleMatch || descriptionMatch || keywordMatch
  })
})

const handleSearch = () => {
  selectedIndex.value = 0
}

const handleKeyDown = (event: KeyboardEvent) => {
  switch (event.key) {
    case 'Escape':
      event.preventDefault()
      handleClose()
      break

    case 'ArrowDown':
      event.preventDefault()
      if (selectedIndex.value < filteredCommands.value.length - 1) {
        selectedIndex.value++
        scrollToSelected()
      }
      break

    case 'ArrowUp':
      event.preventDefault()
      if (selectedIndex.value > 0) {
        selectedIndex.value--
        scrollToSelected()
      }
      break

    case 'Enter':
      event.preventDefault()
      const selectedCommand = filteredCommands.value[selectedIndex.value]
      if (selectedCommand) {
        executeCommand(selectedCommand)
      }
      break
  }
}

const scrollToSelected = () => {
  nextTick(() => {
    const selectedItem = paletteRef.value?.querySelector('.command-item-selected') as HTMLElement
    if (selectedItem) {
      selectedItem.scrollIntoView({ block: 'nearest', behavior: 'smooth' })
    }
  })
}

const executeCommand = (command: Command) => {
  command.action()
  emit('command-execute', command)
  handleClose()
}

const handleClose = () => {
  searchQuery.value = ''
  selectedIndex.value = 0
  emit('close')
}

// Focus input when palette opens
watch(() => props.show, (isOpen) => {
  if (isOpen) {
    nextTick(() => {
      searchInputRef.value?.focus()
    })
  }
})

// Close on Escape key globally
const handleGlobalEscape = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && props.show) {
    handleClose()
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleGlobalEscape)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleGlobalEscape)
})
</script>

<style scoped lang="scss">
.command-palette-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 10vh;
  z-index: 2000;
}

.command-palette {
  width: 100%;
  max-width: 640px;
  max-height: 70vh;
  background: linear-gradient(135deg, #ffffff 0%, #f8fafc 100%);
  border-radius: var(--border-radius-xl, 0.75rem);
  box-shadow: var(--shadow-2xl, 0 25px 50px -12px rgba(0, 0, 0, 0.25));
  overflow: hidden;
  display: flex;
  flex-direction: column;
  border: 1px solid rgba(226, 232, 240, 0.8);
}

.command-palette-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 0.5rem);
  padding: var(--spacing-lg, 1rem);
  border-bottom: 1px solid var(--color-gray-200, #e5e7eb);
}

.search-icon {
  color: var(--color-gray-400, #9ca3af);
  flex-shrink: 0;
}

.command-palette-input {
  flex: 1;
  border: none;
  outline: none;
  background: transparent;
  font-size: var(--font-size-base, 1rem);
  color: var(--color-gray-900, #111827);
  padding: var(--spacing-xs, 0.25rem) 0;

  &::placeholder {
    color: var(--color-gray-400, #9ca3af);
  }
}

.command-palette-hint {
  display: inline-flex;
  align-items: center;
  padding: 2px 6px;
  background: var(--color-gray-100, #f3f4f6);
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-sm, 0.25rem);
  font-size: var(--font-size-xs, 0.75rem);
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, 'Liberation Mono', monospace;
  color: var(--color-gray-600, #4b5563);
}

.command-palette-list {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-xs, 0.25rem);
}

.command-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: var(--spacing-md, 0.75rem);
  padding: var(--spacing-md, 0.75rem) var(--spacing-lg, 1rem);
  border: none;
  background: transparent;
  text-align: left;
  cursor: pointer;
  border-radius: var(--border-radius-md, 0.375rem);
  transition: all 0.15s ease;
  outline: none;

  &:hover,
  &.command-item-selected {
    background: linear-gradient(90deg, rgba(59, 130, 246, 0.1) 0%, rgba(59, 130, 246, 0.05) 100%);
  }

  &:focus-visible {
    outline: 2px solid var(--color-primary-500, #3b82f6);
    outline-offset: -2px;
  }
}

.command-icon {
  width: 20px;
  height: 20px;
  color: var(--color-gray-600, #4b5563);
  flex-shrink: 0;
}

.command-content {
  flex: 1;
  min-width: 0;
}

.command-title {
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-900, #111827);
  margin-bottom: 2px;
}

.command-description {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-600, #4b5563);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.command-shortcut {
  display: flex;
  align-items: center;
  gap: 2px;
  flex-shrink: 0;
}

.command-key {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 20px;
  height: 20px;
  padding: 0 4px;
  background: var(--color-gray-100, #f3f4f6);
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-sm, 0.25rem);
  font-size: var(--font-size-xs, 0.75rem);
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, 'Liberation Mono', monospace;
  color: var(--color-gray-700, #374151);
}

.command-palette-empty {
  padding: var(--spacing-2xl, 2rem);
  text-align: center;
  color: var(--color-gray-500, #6b7280);
}

.empty-text {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-medium, 500);
  margin: 0 0 var(--spacing-xs, 0.25rem) 0;
}

.empty-hint {
  font-size: var(--font-size-sm, 0.875rem);
  margin: 0;
}

// Transitions
.command-palette-enter-active,
.command-palette-leave-active {
  transition: opacity 0.2s ease;
}

.command-palette-enter-active .command-palette,
.command-palette-leave-active .command-palette {
  transition: transform 0.2s ease, opacity 0.2s ease;
}

.command-palette-enter-from,
.command-palette-leave-to {
  opacity: 0;
}

.command-palette-enter-from .command-palette,
.command-palette-leave-to .command-palette {
  transform: translateY(-20px) scale(0.95);
  opacity: 0;
}

// Dark theme support
:global(.dark-theme) {
  .command-palette {
    background: linear-gradient(135deg, #1f2937 0%, #111827 100%);
    border-color: rgba(55, 65, 81, 0.8);
  }

  .command-palette-header {
    border-bottom-color: var(--color-gray-700, #374151);
  }

  .command-palette-input {
    color: var(--color-gray-100, #f3f4f6);

    &::placeholder {
      color: var(--color-gray-500, #6b7280);
    }
  }

  .command-palette-hint {
    background: var(--color-gray-800, #1f2937);
    border-color: var(--color-gray-600, #4b5563);
    color: var(--color-gray-300, #d1d5db);
  }

  .command-item {
    &:hover,
    &.command-item-selected {
      background: linear-gradient(90deg, rgba(59, 130, 246, 0.2) 0%, rgba(59, 130, 246, 0.1) 100%);
    }
  }

  .command-icon {
    color: var(--color-gray-400, #9ca3af);
  }

  .command-title {
    color: var(--color-gray-100, #f3f4f6);
  }

  .command-description {
    color: var(--color-gray-400, #9ca3af);
  }

  .command-key {
    background: var(--color-gray-800, #1f2937);
    border-color: var(--color-gray-600, #4b5563);
    color: var(--color-gray-300, #d1d5db);
  }
}
</style>

