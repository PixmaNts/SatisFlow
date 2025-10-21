<template>
  <Teleport to="body">
    <Transition name="modal" appear>
      <div
        v-if="show"
        class="modal-overlay"
        @click="handleOverlayClick"
        @keydown.esc="handleEscapeKey"
      >
        <div
          ref="modalContent"
          class="modal-container"
          role="dialog"
          :aria-modal="show"
          :aria-labelledby="titleId"
          @click.stop
        >
          <div v-if="title || closable" class="modal-header">
            <h2 v-if="title" :id="titleId" class="modal-title">
              {{ title }}
            </h2>
            <button
              v-if="closable"
              type="button"
              class="modal-close"
              aria-label="Close modal"
              @click="handleClose"
            >
              <svg
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  d="M18 6L6 18M6 6L18 18"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                />
              </svg>
            </button>
          </div>

          <div class="modal-body">
            <slot />
          </div>

          <div v-if="$slots.footer" class="modal-footer">
            <slot name="footer" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, nextTick, onUnmounted, watch } from 'vue'

/**
 * Modal component props
 */
interface Props {
  /** Whether the modal is visible */
  show: boolean
  /** Modal title */
  title?: string
  /** Whether the modal can be closed */
  closable?: boolean
  /** Whether clicking outside closes the modal */
  closeOnOverlay?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  title: '',
  closable: true,
  closeOnOverlay: true,
})

/** Emit modal events */
const emit = defineEmits<{
  close: []
  confirm: []
  'update:show': [value: boolean]
}>()

// Template refs
const modalContent = ref<HTMLElement>()

// Generate unique ID for title
const titleId = computed(() => `modal-title-${Math.random().toString(36).substr(2, 9)}`)

// Focus management
let previousFocus: Element | null = null

/**
 * Handle overlay click
 */
const handleOverlayClick = () => {
  if (props.closeOnOverlay && props.closable) {
    handleClose()
  }
}

/**
 * Handle close button click
 */
const handleClose = () => {
  if (props.closable) {
    emit('close')
    emit('update:show', false)
  }
}

/**
 * Handle ESC key press
 */
const handleEscapeKey = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && props.closable) {
    handleClose()
  }
}

/**
 * Focus trap implementation
 */
const focusTrap = (event: KeyboardEvent) => {
  if (!modalContent.value || event.key !== 'Tab') return

  const focusableElements = modalContent.value.querySelectorAll(
    'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
  )
  const firstElement = focusableElements[0] as HTMLElement
  const lastElement = focusableElements[focusableElements.length - 1] as HTMLElement

  if (event.shiftKey) {
    if (document.activeElement === firstElement) {
      lastElement.focus()
      event.preventDefault()
    }
  } else {
    if (document.activeElement === lastElement) {
      firstElement.focus()
      event.preventDefault()
    }
  }
}

/**
 * Set up focus management when modal opens
 */
const setupFocus = async () => {
  if (props.show) {
    // Store current focus
    previousFocus = document.activeElement

    // Wait for DOM update
    await nextTick()

    // Focus first focusable element
    if (modalContent.value) {
      const focusableElement = modalContent.value.querySelector(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
      ) as HTMLElement
      focusableElement?.focus()
    }

    // Add focus trap listener
    document.addEventListener('keydown', focusTrap)
  } else {
    // Remove focus trap listener
    document.removeEventListener('keydown', focusTrap)

    // Restore previous focus
    if (previousFocus && previousFocus instanceof HTMLElement) {
      previousFocus.focus()
    }
  }
}

// Watch for show prop changes
watch(() => props.show, setupFocus)

// Clean up on unmount
onUnmounted(() => {
  document.removeEventListener('keydown', focusTrap)
})
</script>

<style scoped lang="scss">
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: var(--spacing-md, 0.75rem);
}

.modal-container {
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-lg, 0.5rem);
  box-shadow: var(--shadow-xl, 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04));
  max-width: 90vw;
  max-height: 90vh;
  width: 100%;
  max-width: 32rem;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg, 1rem) var(--spacing-xl, 1.25rem);
  border-bottom: 1px solid var(--color-gray-200, #e5e7eb);
}

.modal-title {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-gray-900, #111827);
  margin: 0;
}

.modal-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2rem;
  height: 2rem;
  border: none;
  background: none;
  color: var(--color-gray-500, #6b7280);
  border-radius: var(--border-radius-md, 0.375rem);
  cursor: pointer;
  transition: all 0.2s ease-in-out;

  &:hover {
    background-color: var(--color-gray-100, #f3f4f6);
    color: var(--color-gray-700, #374151);
  }

  &:focus-visible {
    outline: 2px solid var(--color-primary-500, #3b82f6);
    outline-offset: 2px;
  }
}

.modal-body {
  padding: var(--spacing-lg, 1rem) var(--spacing-xl, 1.25rem);
  overflow-y: auto;
  flex: 1;
}

.modal-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--spacing-sm, 0.5rem);
  padding: var(--spacing-lg, 1rem) var(--spacing-xl, 1.25rem);
  border-top: 1px solid var(--color-gray-200, #e5e7eb);
}

// Transition animations
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-active .modal-container,
.modal-leave-active .modal-container {
  transition: transform 0.3s ease, opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
  transform: scale(0.9);
  opacity: 0;
}

// Responsive design
@media (max-width: 640px) {
  .modal-overlay {
    padding: var(--spacing-sm, 0.5rem);
  }

  .modal-container {
    max-width: 100vw;
    max-height: 100vh;
    border-radius: var(--border-radius-lg, 0.5rem) var(--border-radius-lg, 0.5rem) 0 0;
    margin: auto 0;
  }

  .modal-header,
  .modal-body,
  .modal-footer {
    padding-left: var(--spacing-md, 0.75rem);
    padding-right: var(--spacing-md, 0.75rem);
  }
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'UiModal'
}
</script>
