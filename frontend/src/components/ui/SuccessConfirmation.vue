<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'

interface Props {
  show: boolean
  title: string
  message?: string
  confirmText?: string
  cancelText?: string
  type?: 'success' | 'info' | 'warning'
  autoClose?: boolean
  autoCloseDelay?: number
  showIcon?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  message: '',
  confirmText: 'OK',
  cancelText: 'Cancel',
  type: 'success',
  autoClose: false,
  autoCloseDelay: 3000,
  showIcon: true
})

const emit = defineEmits<{
  confirm: []
  cancel: []
  close: []
}>()

// Local state
const isVisible = ref(false)
const autoCloseTimer = ref<number | null>(null)

// Computed icon based on type
const icon = computed(() => {
  const icons = {
    success: '<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M9 12L11 14L15 10M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>',
    info: '<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12 9V13M12 17H12.01M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>',
    warning: '<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12 9V13M12 17H12.01M5.07183 19H18.9282C19.558 19 20.0848 18.5041 20.1233 17.8759C20.1618 17.2477 19.6986 16.6945 19.0784 16.5908L12.1502 15.4759C12.0506 15.4592 11.9494 15.4592 11.8498 15.4759L4.9216 16.5908C4.3014 16.6945 3.8382 17.2477 3.8767 17.8759C3.9152 18.5041 4.442 19 5.07183 19Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>'
  }
  return icons[props.type]
})

// Computed color classes based on type
const typeClasses = computed(() => {
  const classes = {
    success: {
      bg: 'bg-green-50 dark:bg-green-900/20',
      border: 'border-green-200 dark:border-green-800',
      text: 'text-green-800 dark:text-green-200',
      icon: 'text-green-600 dark:text-green-400',
      button: 'bg-green-600 hover:bg-green-700 text-white'
    },
    info: {
      bg: 'bg-blue-50 dark:bg-blue-900/20',
      border: 'border-blue-200 dark:border-blue-800',
      text: 'text-blue-800 dark:text-blue-200',
      icon: 'text-blue-600 dark:text-blue-400',
      button: 'bg-blue-600 hover:bg-blue-700 text-white'
    },
    warning: {
      bg: 'bg-yellow-50 dark:bg-yellow-900/20',
      border: 'border-yellow-200 dark:border-yellow-800',
      text: 'text-yellow-800 dark:text-yellow-200',
      icon: 'text-yellow-600 dark:text-yellow-400',
      button: 'bg-yellow-600 hover:bg-yellow-700 text-white'
    }
  }
  return classes[props.type]
})

// Handle confirm action
const handleConfirm = () => {
  emit('confirm')
  closeDialog()
}

// Handle cancel action
const handleCancel = () => {
  emit('cancel')
  closeDialog()
}

// Close dialog
const closeDialog = () => {
  isVisible.value = false
  clearAutoCloseTimer()
  emit('close')
}

// Setup auto close timer
const setupAutoCloseTimer = () => {
  if (props.autoClose && props.autoCloseDelay > 0) {
    autoCloseTimer.value = window.setTimeout(() => {
      closeDialog()
    }, props.autoCloseDelay)
  }
}

// Clear auto close timer
const clearAutoCloseTimer = () => {
  if (autoCloseTimer.value) {
    clearTimeout(autoCloseTimer.value)
    autoCloseTimer.value = null
  }
}

// Handle escape key
const handleEscapeKey = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && isVisible.value) {
    handleCancel()
  }
}

// Watch for show prop changes
onMounted(() => {
  isVisible.value = props.show
  if (isVisible.value) {
    setupAutoCloseTimer()
  }
  document.addEventListener('keydown', handleEscapeKey)
})

onUnmounted(() => {
  clearAutoCloseTimer()
  document.removeEventListener('keydown', handleEscapeKey)
})

// Update visibility when show prop changes
watch(() => props.show, (newValue: boolean) => {
  isVisible.value = newValue
  if (newValue) {
    setupAutoCloseTimer()
  } else {
    clearAutoCloseTimer()
  }
})
</script>

<template>
  <Teleport to="body">
    <Transition
      name="confirmation"
      enter-active-class="transition-all duration-300 ease-out"
      enter-from-class="opacity-0 scale-90"
      enter-to-class="opacity-100 scale-100"
      leave-active-class="transition-all duration-200 ease-in"
      leave-from-class="opacity-100 scale-100"
      leave-to-class="opacity-0 scale-90"
    >
      <div
        v-if="isVisible"
        class="fixed inset-0 z-50 flex items-center justify-center p-4"
        role="dialog"
        aria-modal="true"
        :aria-labelledby="`confirmation-title-${Math.random().toString(36).substr(2, 9)}`"
      >
        <!-- Backdrop -->
        <div
          class="absolute inset-0 bg-black/50 backdrop-blur-sm"
          @click="handleCancel"
        />

        <!-- Dialog -->
        <div
          :class="[
            'relative max-w-md w-full rounded-lg shadow-lg p-6',
            typeClasses.bg,
            typeClasses.border,
            'border'
          ]"
        >
          <!-- Icon and Title -->
          <div class="flex items-start">
            <div
              v-if="showIcon"
              :class="['flex-shrink-0 mr-3', typeClasses.icon]"
              v-html="icon"
            />

            <div class="flex-1">
              <h3
                :id="`confirmation-title-${Math.random().toString(36).substr(2, 9)}`"
                :class="['text-lg font-medium', typeClasses.text]"
              >
                {{ title }}
              </h3>

              <p
                v-if="message"
                :class="['mt-1 text-sm', typeClasses.text]"
              >
                {{ message }}
              </p>
            </div>
          </div>

          <!-- Actions -->
          <div class="mt-6 flex justify-end space-x-3">
            <button
              v-if="type !== 'success'"
              @click="handleCancel"
              class="px-4 py-2 text-sm font-medium border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 dark:border-gray-600 dark:hover:bg-gray-700"
            >
              {{ cancelText }}
            </button>

            <button
              @click="handleConfirm"
              :class="[
                'px-4 py-2 text-sm font-medium rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2',
                typeClasses.button
              ]"
            >
              {{ confirmText }}
            </button>
          </div>

          <!-- Auto-close indicator -->
          <div
            v-if="autoClose"
            class="absolute bottom-0 left-0 h-1 bg-gray-200 dark:bg-gray-700 rounded-b-lg overflow-hidden"
          >
            <div
              class="h-full bg-blue-600"
              :style="{ animation: `countdown ${autoCloseDelay}ms linear forwards` }"
            />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
/* Countdown animation for auto-close indicator */
@keyframes countdown {
  from {
    width: 100%;
  }
  to {
    width: 0%;
  }
}

/* Transition styles */
.confirmation-enter-active,
.confirmation-leave-active {
  transition: all 0.3s ease;
}

.confirmation-enter-from,
.confirmation-leave-to {
  opacity: 0;
  transform: scale(0.9);
}

/* Focus styles for accessibility */
button:focus-visible {
  outline: 2px solid currentColor;
  outline-offset: 2px;
}

/* Respect prefers-reduced-motion */
@media (prefers-reduced-motion: reduce) {
  .confirmation-enter-active,
  .confirmation-leave-active {
    transition: none;
  }

  @keyframes countdown {
    from, to {
      width: 100%;
    }
  }
}
</style>
