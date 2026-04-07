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

<style scoped lang="scss">
.collapsible {
  border: 1px solid var(--color-border, #404040);
  border-radius: var(--border-radius-lg, 0.5rem);
  overflow: hidden;
  transition: all var(--transition-normal, 200ms) cubic-bezier(0.4, 0, 0.2, 1);
  background-color: var(--color-surface, #252525);
}

.collapsible--open {
  box-shadow: var(--shadow-md, 0 4px 6px rgba(0, 0, 0, 0.35));
  border-color: var(--color-ficsit-orange, #f58b00);
}

.collapsible-header {
  width: 100%;
  padding: var(--spacing-2, 0.5rem) var(--spacing-3, 0.75rem);
  background-color: var(--color-surface, #252525);
  cursor: pointer;
  user-select: none;
  transition: background-color var(--transition-fast, 150ms) ease;
}

.collapsible-header:hover {
  background-color: var(--color-surface-hover, #2a2a2a);
}

.collapsible--open .collapsible-header {
  background-color: var(--color-surface-active, #303030);
  border-bottom: 1px solid var(--color-border, #404040);
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  width: 100%;
  flex-wrap: nowrap;
}

.header-left {
  display: flex;
  align-items: center;
  gap: var(--spacing-2, 0.5rem);
  flex: 1 1 0;
  min-width: 0;
  overflow: hidden;
}

.chevron-icon {
  color: var(--color-text-muted, #8a8a8a);
  transition: transform var(--transition-fast, 150ms) ease;
  flex-shrink: 0;
  font-size: 0.625rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  line-height: 1;
}

.chevron-icon--rotated {
  transform: rotate(90deg);
}

.header-title {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary, #e5e5e5);
}

.header-right {
  display: flex !important;
  align-items: center !important;
  gap: var(--spacing-2, 0.5rem);
  flex-shrink: 0 !important;
  flex-wrap: nowrap !important;
  margin-left: auto;
}

.summary-content {
  margin-top: var(--spacing-2, 0.5rem);
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-muted, #8a8a8a);
}

.collapsible-body {
  overflow: hidden;
}

.body-content {
  padding: var(--spacing-3, 0.75rem);
  background-color: var(--color-surface-inset, #1f1f1f);
}

/* Collapse animation */
.collapse-enter-active,
.collapse-leave-active {
  transition: all var(--transition-slow, 300ms) ease-in-out;
  overflow: hidden;
}

.collapse-enter-from,
.collapse-leave-to {
  opacity: 0;
  height: 0 !important;
}
</style>
