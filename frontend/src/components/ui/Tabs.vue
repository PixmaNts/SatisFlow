<template>
  <div class="tabs">
    <!-- Tab navigation -->
    <div
      class="tab-nav"
      role="tablist"
      :aria-label="ariaLabel"
    >
      <button
        v-for="tab in tabs"
        :key="tab.id"
        type="button"
        :class="tabClasses(tab)"
        :id="`tab-${tab.id}`"
        :disabled="tab.disabled"
        :aria-selected="activeTab === tab.id"
        :aria-controls="`panel-${tab.id}`"
        :tabindex="activeTab === tab.id ? 0 : -1"
        role="tab"
        @click="handleTabClick(tab)"
        @keydown="handleKeyNavigation($event, tab)"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- Tab panels -->
    <div class="tab-panels">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, provide, nextTick } from 'vue'

/**
 * Tab interface
 */
export interface Tab {
  /** Unique identifier for the tab */
  id: string
  /** Display label for the tab */
  label: string
  /** Whether the tab is disabled */
  disabled?: boolean
}

/**
 * Tabs component props
 */
interface Props {
  /** Array of tabs to display */
  tabs: Tab[]
  /** Initially active tab ID */
  activeTab?: string
  /** Accessibility label for tablist */
  ariaLabel?: string
}

const props = withDefaults(defineProps<Props>(), {
  activeTab: '',
  ariaLabel: 'Tabs',
})

/** Emit tab change events */
const emit = defineEmits<{
  'tab-change': [tabId: string]
  'update:activeTab': [tabId: string]
}>()

// Internal state
const internalActiveTab = ref(props.tabs[0]?.id || '')

// Computed active tab with fallback to internal state
const activeTab = computed(() => {
  return props.activeTab || internalActiveTab.value
})

/**
 * Compute CSS classes for a tab button
 */
const tabClasses = (tab: Tab) => [
  'tab-button',
  {
    'tab-active': activeTab.value === tab.id,
    'tab-disabled': tab.disabled,
  },
]

/**
 * Handle tab click
 */
const handleTabClick = (tab: Tab) => {
  if (!tab.disabled && activeTab.value !== tab.id) {
    internalActiveTab.value = tab.id
    emit('tab-change', tab.id)
    emit('update:activeTab', tab.id)
  }
}

/**
 * Handle keyboard navigation
 */
const handleKeyNavigation = (event: KeyboardEvent, currentTab: Tab) => {
  const { key } = event

  // Only handle arrow keys and Home/End
  if (!['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(key)) {
    return
  }

  event.preventDefault()

  const enabledTabs = props.tabs.filter(tab => !tab.disabled)
  const currentIndex = enabledTabs.findIndex(tab => tab.id === currentTab.id)

  let nextTab: Tab | undefined

  switch (key) {
    case 'ArrowLeft':
      nextTab = enabledTabs[currentIndex - 1] || enabledTabs[enabledTabs.length - 1]
      break
    case 'ArrowRight':
      nextTab = enabledTabs[currentIndex + 1] || enabledTabs[0]
      break
    case 'Home':
      nextTab = enabledTabs[0]
      break
    case 'End':
      nextTab = enabledTabs[enabledTabs.length - 1]
      break
  }

  if (nextTab) {
    handleTabClick(nextTab)
    // Focus the button after state update
    nextTick(() => {
      const button = document.getElementById(`tab-${nextTab.id}`)
      button?.focus()
    })
  }
}

// Watch for external activeTab changes
watch(() => props.activeTab, (newTabId) => {
  if (newTabId && newTabId !== internalActiveTab.value) {
    internalActiveTab.value = newTabId
  }
})

// Provide active tab to child components
provide('activeTab', activeTab)

</script>

<style scoped lang="scss">
.tabs {
  width: 100%;
}

.tab-nav {
  display: flex;
  border-bottom: 1px solid var(--color-gray-200, #e5e7eb);
  overflow-x: auto;
  scrollbar-width: none; // Firefox
  -ms-overflow-style: none; // IE/Edge

  &::-webkit-scrollbar {
    display: none; // Chrome/Safari/Opera
  }
}

.tab-button {
  position: relative;
  padding: var(--spacing-md, 0.75rem) var(--spacing-lg, 1rem);
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  font-family: var(--font-family-sans, system-ui, sans-serif);
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-600, #4b5563);
  cursor: pointer;
  transition: all 0.2s ease-in-out;
  white-space: nowrap;
  user-select: none;

  &:hover:not(.tab-disabled) {
    color: var(--color-gray-900, #111827);
    background-color: var(--color-gray-50, #f9fafb);
  }

  &:focus-visible {
    outline: 2px solid var(--color-primary-500, #3b82f6);
    outline-offset: -2px;
  }

  &.tab-active {
    color: var(--color-primary-600, #2563eb);
    border-bottom-color: var(--color-primary-600, #2563eb);
  }

  &.tab-disabled {
    color: var(--color-gray-400, #9ca3af);
    cursor: not-allowed;
    opacity: 0.6;
  }
}

.tab-panels {
  padding-top: var(--spacing-lg, 1rem);
}

// Responsive design
@media (max-width: 640px) {
  .tab-button {
    padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
    font-size: var(--font-size-sm, 0.875rem);
  }
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'UiTabs'
}
</script>
