<template>
  <Modal
    :show="show"
    :title="title"
    :size="size"
    @close="$emit('cancel')"
  >
    <div class="confirm-dialog">
      <p v-if="message" class="confirm-message">
        {{ message }}
      </p>

      <div v-if="$slots.default" class="confirm-content">
        <slot />
      </div>

      <div v-if="details" class="confirm-details">
        <details>
          <summary>Details</summary>
          <pre>{{ details }}</pre>
        </details>
      </div>
    </div>

    <template #footer>
      <div class="confirm-actions">
        <Button
          variant="secondary"
          @click="$emit('cancel')"
          :disabled="loading"
        >
          {{ cancelText }}
        </Button>
        <Button
          :variant="variant"
          @click="$emit('confirm')"
          :disabled="loading"
          :loading="loading"
        >
          {{ confirmText }}
        </Button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import Button from './Button.vue'
import Modal from './Modal.vue'

interface Props {
  /** Whether dialog is shown */
  show: boolean
  /** Dialog title */
  title: string
  /** Confirmation message */
  message?: string
  /** Confirm button text */
  confirmText?: string
  /** Cancel button text */
  cancelText?: string
  /** Button variant (affects confirm button) */
  variant?: 'primary' | 'secondary' | 'danger'
  /** Modal size */
  size?: 'sm' | 'md' | 'lg' | 'xl'
  /** Loading state */
  loading?: boolean
  /** Optional details to show in collapsible section */
  details?: string
}

withDefaults(defineProps<Props>(), {
  confirmText: 'Confirm',
  cancelText: 'Cancel',
  variant: 'primary',
  size: 'md',
  loading: false
})

defineEmits<{
  confirm: []
  cancel: []
}>()
</script>

<style scoped>
.confirm-dialog {
  @apply space-y-4;
}

.confirm-message {
  @apply text-gray-700 dark:text-gray-300;
}

.confirm-content {
  @apply text-gray-700 dark:text-gray-300;
}

.confirm-details {
  @apply mt-4;
}

.confirm-details details {
  @apply border border-gray-200 dark:border-gray-700 rounded-md p-2;
}

.confirm-details summary {
  @apply cursor-pointer font-medium text-gray-700 dark:text-gray-300 mb-2;
}

.confirm-details pre {
  @apply text-xs text-gray-600 dark:text-gray-400 whitespace-pre-wrap;
  max-height: 200px;
  overflow-y: auto;
}

.confirm-actions {
  @apply flex gap-3 justify-end;
}
</style>
