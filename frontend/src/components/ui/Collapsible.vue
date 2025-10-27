<template>
  <div class="collapsible" :class="{ 'collapsible--open': isOpen }">
    <div
      class="collapsible-header"
      :class="headerClass"
      @click="toggle"
      role="button"
      :aria-expanded="isOpen"
      tabindex="0"
      @keydown.enter="toggle"
      @keydown.space.prevent="toggle"
    >
      <div class="header-content">
        <div class="header-left">
          <span class="chevron-icon" :class="{ 'chevron-icon--rotated': isOpen }">
            ▶
          </span>
          <slot name="header">
            <h4 class="header-title">{{ title }}</h4>
          </slot>
        </div>

        <div class="header-right" @click.stop>
          <slot name="badge"></slot>
          <slot name="actions"></slot>
        </div>
      </div>

      <div v-if="$slots.summary && !isOpen" class="summary-content">
        <slot name="summary"></slot>
      </div>
    </div>

    <Transition
      name="collapse"
      @enter="onEnter"
      @after-enter="onAfterEnter"
      @leave="onLeave"
      @after-leave="onAfterLeave"
    >
      <div v-show="isOpen" class="collapsible-body">
        <div class="body-content">
          <slot></slot>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

interface Props {
  title?: string;
  defaultOpen?: boolean;
  headerClass?: string;
}

interface Emits {
  (e: 'toggle', isOpen: boolean): void;
}

const props = withDefaults(defineProps<Props>(), {
  defaultOpen: false,
  headerClass: '',
});

const emit = defineEmits<Emits>();

// State
const isOpen = ref(props.defaultOpen);

// Methods
const toggle = () => {
  isOpen.value = !isOpen.value;
  emit('toggle', isOpen.value);
};

const open = () => {
  isOpen.value = true;
  emit('toggle', true);
};

const close = () => {
  isOpen.value = false;
  emit('toggle', false);
};

// Animation handlers
const onEnter = (el: Element) => {
  const element = el as HTMLElement;
  element.style.height = '0';
};

const onAfterEnter = (el: Element) => {
  const element = el as HTMLElement;
  element.style.height = 'auto';
};

const onLeave = (el: Element) => {
  const element = el as HTMLElement;
  element.style.height = `${element.scrollHeight}px`;
  // Force reflow
  element.offsetHeight;
  element.style.height = '0';
};

const onAfterLeave = (el: Element) => {
  const element = el as HTMLElement;
  element.style.height = 'auto';
};

// Expose methods
defineExpose({
  toggle,
  open,
  close,
  isOpen,
});
</script>

<style scoped>
.collapsible {
  @apply border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden
         transition-all duration-200;
}

.collapsible--open {
  @apply ring-1 ring-blue-500/20 border-blue-200 dark:border-blue-800;
}

.collapsible-header {
  @apply w-full px-3 py-2
         bg-white dark:bg-gray-800
         hover:bg-gray-50 dark:hover:bg-gray-750
         cursor-pointer select-none
         transition-colors duration-150;
}

.collapsible--open .collapsible-header {
  @apply bg-gray-50 dark:bg-gray-750 border-b border-gray-200 dark:border-gray-700;
}

.header-content {
  @apply flex items-center justify-between gap-3;
}

.header-left {
  @apply flex items-center gap-2 flex-1 min-w-0;
}

.chevron-icon {
  @apply text-gray-500 dark:text-gray-400
         transition-transform duration-200 flex-shrink-0;
  font-size: 0.625rem;
  display: inline-block;
  line-height: 1;
}

.chevron-icon--rotated {
  transform: rotate(90deg);
}

.header-title {
  @apply text-base font-medium text-gray-900 dark:text-gray-100;
}

.header-right {
  @apply flex items-center gap-2 flex-shrink-0;
}

.summary-content {
  @apply mt-2 text-sm text-gray-600 dark:text-gray-400;
}

.collapsible-body {
  @apply overflow-hidden;
}

.body-content {
  @apply p-3 bg-white dark:bg-gray-800;
}

/* Collapse animation */
.collapse-enter-active,
.collapse-leave-active {
  @apply transition-all duration-300 ease-in-out;
  overflow: hidden;
}

.collapse-enter-from,
.collapse-leave-to {
  opacity: 0;
  height: 0 !important;
}
</style>
