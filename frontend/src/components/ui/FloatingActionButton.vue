<template>
  <div class="fab-container" :data-position="position">
    <button
      ref="fabButton"
      type="button"
      class="fab-button fab-main"
      :aria-label="mainButtonLabel"
      :aria-expanded="expanded"
      @click="toggleMenu"
    >
        <svg
          class="fab-icon"
          :class="{ 'rotated': expanded }"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M12 5V19M5 12H19"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </button>

    <TransitionGroup name="fab-item" tag="div" class="fab-menu">
      <button
        v-for="(action, index) in visibleActions"
        :key="action.id"
        type="button"
        class="fab-button fab-action"
        :class="action.class"
        :style="{ '--delay': `${index * 50}ms` }"
        :aria-label="action.label"
        @click="handleActionClick(action)"
      >
        <component :is="action.icon" v-if="action.icon" class="fab-action-icon" />
        <span v-if="expanded" class="fab-action-label">{{ action.label }}</span>
      </button>
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'

export interface FabAction {
  id: string
  label: string
  icon?: any
  class?: string
  onClick: () => void
}

interface Props {
  actions: FabAction[]
  mainButtonLabel?: string
  position?: 'bottom-right' | 'bottom-left' | 'top-right' | 'top-left'
  expanded?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  mainButtonLabel: 'Quick actions',
  position: 'bottom-right',
  expanded: false,
})

const emit = defineEmits<{
  'action-click': [action: FabAction]
}>()

const expanded = ref(props.expanded ?? false)
const fabButton = ref<HTMLButtonElement>()

const visibleActions = computed(() => {
  return expanded.value ? props.actions : []
})

const toggleMenu = () => {
  expanded.value = !expanded.value
}

const handleActionClick = (action: FabAction) => {
  action.onClick()
  emit('action-click', action)
  expanded.value = false
}

// Close menu when clicking outside
const handleClickOutside = (event: MouseEvent) => {
  if (expanded.value && fabButton.value && !fabButton.value.contains(event.target as Node)) {
    const fabContainer = (event.target as HTMLElement).closest('.fab-container')
    if (!fabContainer) {
      expanded.value = false
    }
  }
}

// Close menu on Escape key
const handleEscape = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && expanded.value) {
    expanded.value = false
  }
}

// Watch for prop changes
watch(() => props.expanded, (newValue) => {
  expanded.value = newValue
})

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  document.addEventListener('keydown', handleEscape)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  document.removeEventListener('keydown', handleEscape)
})
</script>

<style scoped lang="scss">
.fab-container {
  position: fixed;
  z-index: 1000;
  display: flex;
  flex-direction: column-reverse;
  align-items: center;
  gap: var(--spacing-md, 0.75rem);

  // Position variants
  &[data-position='bottom-right'] {
    bottom: var(--spacing-xl, 1.25rem);
    right: var(--spacing-xl, 1.25rem);
  }

  &[data-position='bottom-left'] {
    bottom: var(--spacing-xl, 1.25rem);
    left: var(--spacing-xl, 1.25rem);
  }

  &[data-position='top-right'] {
    top: var(--spacing-xl, 1.25rem);
    right: var(--spacing-xl, 1.25rem);
    flex-direction: column;
  }

  &[data-position='top-left'] {
    top: var(--spacing-xl, 1.25rem);
    left: var(--spacing-xl, 1.25rem);
    flex-direction: column;
  }
}

.fab-menu {
  display: flex;
  flex-direction: column-reverse;
  gap: var(--spacing-sm, 0.5rem);
  align-items: center;
}

.fab-button {
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: var(--border-radius-full, 9999px);
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: var(--shadow-lg, 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05));
  font-weight: var(--font-weight-medium, 500);
  outline: none;

  &:focus-visible {
    outline: 2px solid var(--color-primary-500, #3b82f6);
    outline-offset: 2px;
  }

  &:active {
    transform: scale(0.95);
  }
}

.fab-main {
  width: 56px;
  height: 56px;
  background: linear-gradient(135deg, var(--color-primary-600, #2563eb) 0%, var(--color-primary-500, #3b82f6) 100%);
  color: #ffffff;

  &:hover {
    background: linear-gradient(135deg, var(--color-primary-700, #1d4ed8) 0%, var(--color-primary-600, #2563eb) 100%);
    box-shadow: var(--shadow-xl, 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04));
    transform: translateY(-2px);
  }
}

.fab-icon {
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);

  &.rotated {
    transform: rotate(45deg);
  }
}

.fab-action {
  width: auto;
  min-width: 48px;
  height: 48px;
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  background: linear-gradient(135deg, #ffffff 0%, #f8fafc 100%);
  color: var(--color-gray-700, #374151);
  border: 1px solid var(--color-gray-200, #e5e7eb);
  gap: var(--spacing-sm, 0.5rem);
  white-space: nowrap;

  &:hover {
    background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
    border-color: var(--color-primary-300, #93c5fd);
    color: var(--color-primary-700, #1d4ed8);
    box-shadow: var(--shadow-md, 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06));
    transform: translateX(-4px);
  }
}

.fab-action-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

.fab-action-label {
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
}

// Transitions
.fab-enter-active,
.fab-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.fab-enter-from,
.fab-leave-to {
  opacity: 0;
  transform: scale(0.8) translateY(10px);
}

.fab-item-enter-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  transition-delay: var(--delay, 0ms);
}

.fab-item-leave-active {
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.fab-item-enter-from {
  opacity: 0;
  transform: scale(0.8) translateY(10px);
}

.fab-item-leave-to {
  opacity: 0;
  transform: scale(0.8) translateY(-10px);
}

// Responsive
@media (max-width: 640px) {
  .fab-container {
    &[data-position='bottom-right'],
    &[data-position='bottom-left'] {
      bottom: var(--spacing-lg, 1rem);
    }

    &[data-position='bottom-right'] {
      right: var(--spacing-lg, 1rem);
    }

    &[data-position='bottom-left'] {
      left: var(--spacing-lg, 1rem);
    }
  }

  .fab-action-label {
    display: none;
  }

  .fab-action {
    width: 48px;
    padding: var(--spacing-sm, 0.5rem);
  }
}
</style>

