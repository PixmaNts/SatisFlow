<template>
  <div
    v-show="isActive"
    :class="['tab-panel', { 'tab-panel-active': isActive }]"
    :id="`panel-${tabId}`"
    role="tabpanel"
    :aria-labelledby="`tab-${tabId}`"
    :tabindex="isActive ? 0 : -1"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed, inject, type Ref } from 'vue'

/**
 * TabPanel component props
 */
interface Props {
  /** Unique identifier for the tab panel */
  tabId: string
}

const props = defineProps<Props>()

// Inject active tab from parent Tabs component
const activeTab = inject<Ref<string>>('activeTab')

// Compute if this panel is active
const isActive = computed(() => {
  return activeTab?.value === props.tabId
})

</script>

<style scoped lang="scss">
.tab-panel {
  outline: none;

  &.tab-panel-active {
    animation: tab-panel-fade-in 0.3s ease-in-out;
  }
}

@keyframes tab-panel-fade-in {
  from {
    opacity: 0;
    transform: translateY(0.25rem);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'UiTabPanel'
}
</script>
