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

<style scoped lang="scss">
.confirm-dialog {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4, 1rem);
}

.confirm-message {
  color: var(--color-text-secondary, #b8b8b8);
}

.confirm-content {
  color: var(--color-text-secondary, #b8b8b8);
}

.confirm-details {
  margin-top: var(--spacing-4, 1rem);
}

.confirm-details details {
  border: 1px solid var(--color-border, #404040);
  border-radius: var(--border-radius-md, 0.375rem);
  padding: var(--spacing-2, 0.5rem);
  background-color: var(--color-surface-inset, #1f1f1f);
}

.confirm-details summary {
  cursor: pointer;
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary, #e5e5e5);
  margin-bottom: var(--spacing-2, 0.5rem);
}

.confirm-details pre {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-text-muted, #8a8a8a);
  white-space: pre-wrap;
  max-height: 200px;
  overflow-y: auto;
}

.confirm-actions {
  display: flex;
  gap: var(--spacing-3, 0.75rem);
  justify-content: flex-end;
}
</style>
