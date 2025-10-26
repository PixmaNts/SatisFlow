<template>
  <div class="save-load-controls">
    <!-- Save Button -->
    <Button
      variant="secondary"
      size="sm"
      :loading="saving"
      :disabled="saving || loading || resetting"
      @click="handleSave"
      class="control-button"
    >
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M21 15V19C21 19.5304 20.7893 20.0391 20.4142 20.4142C20.0391 20.7893 19.5304 21 19 21H5C4.46957 21 3.96086 20.7893 3.58579 20.4142C3.21071 20.0391 3 19.5304 3 19V15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M7 10L12 15L17 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M12 15V3" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      Save
    </Button>

    <!-- Load Button -->
    <Button
      variant="secondary"
      size="sm"
      :loading="loading"
      :disabled="saving || loading || resetting"
      @click="handleLoad"
      class="control-button"
    >
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M21 15V19C21 19.5304 20.7893 20.0391 20.4142 20.4142C20.0391 20.7893 19.5304 21 19 21H5C4.46957 21 3.96086 20.7893 3.58579 20.4142C3.21071 20.0391 3 19.5304 3 19V15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M17 8L12 3L7 8" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M12 3V15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      Load
    </Button>

    <!-- Reset Button -->
    <Button
      variant="danger"
      size="sm"
      :loading="resetting"
      :disabled="saving || loading || resetting"
      @click="showResetDialog = true"
      class="control-button"
    >
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M3 6H21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M8 6V4C8 3.46957 8.21071 2.96086 8.58579 2.58579C8.96086 2.21071 9.46957 2 10 2H14C14.5304 2 15.0391 2.21071 15.4142 2.58579C15.7893 2.96086 16 3.46957 16 4V6M19 6V20C19 20.5304 18.7893 21.0391 18.4142 21.4142C18.0391 21.7893 17.5304 22 17 22H7C6.46957 22 5.96086 21.7893 5.58579 21.4142C5.21071 21.0391 5 20.5304 5 20V6H19Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      Reset
    </Button>

    <!-- Success/Error Messages -->
    <transition name="fade">
      <div v-if="successMessage" class="status-message success">
        {{ successMessage }}
      </div>
    </transition>

    <transition name="fade">
      <div v-if="errorMessage" class="status-message error">
        {{ errorMessage }}
      </div>
    </transition>

    <!-- Reset Confirmation Dialog -->
    <Modal
      :show="showResetDialog"
      title="Confirm Reset"
      :closable="true"
      :close-on-overlay="true"
      @close="showResetDialog = false"
    >
      <div class="reset-dialog-content">
        <p class="warning-text">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" class="warning-icon">
            <path d="M12 9V13M12 17H12.01M10.29 3.86L1.82 18C1.64537 18.3024 1.55296 18.6453 1.55199 18.9945C1.55101 19.3437 1.64151 19.6871 1.81445 19.9905C1.98738 20.2939 2.23675 20.5467 2.53773 20.7239C2.83871 20.9011 3.18082 20.9961 3.53 21H20.47C20.8192 20.9961 21.1613 20.9011 21.4623 20.7239C21.7633 20.5467 22.0126 20.2939 22.1856 19.9905C22.3585 19.6871 22.449 19.3437 22.448 18.9945C22.447 18.6453 22.3546 18.3024 22.18 18L13.71 3.86C13.5317 3.56611 13.2807 3.32312 12.9812 3.15448C12.6817 2.98585 12.3437 2.89725 12 2.89725C11.6563 2.89725 11.3183 2.98585 11.0188 3.15448C10.7193 3.32312 10.4683 3.56611 10.29 3.86Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </p>
        <p><strong>Warning: This action cannot be undone!</strong></p>
        <p>Resetting will permanently delete:</p>
        <ul>
          <li>All factories and their configurations</li>
          <li>All production lines</li>
          <li>All raw inputs and power generators</li>
          <li>All logistics lines</li>
        </ul>
        <p>Consider saving your current state before resetting.</p>
      </div>

      <template #footer>
        <div class="dialog-footer">
          <Button variant="secondary" size="md" @click="showResetDialog = false" :disabled="resetting">
            Cancel
          </Button>
          <Button variant="danger" size="md" @click="handleReset" :loading="resetting">
            Reset All Data
          </Button>
        </div>
      </template>
    </Modal>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { Button, Modal } from '@/components/ui'
import { useFileDownload } from '@/composables/useFileDownload'
import { useFileUpload } from '@/composables/useFileUpload'
import { useErrorHandler } from '@/composables/useErrorHandler'
import { endpoints } from '@/api'

// Router
const router = useRouter()

/**
 * SaveLoadControls component
 *
 * Provides save, load, and reset functionality for the engine state
 */

// Composables
const { downloadJson, generateFilename } = useFileDownload()
const { selectAndReadFile } = useFileUpload()
const { handleError: handleGlobalError } = useErrorHandler()

// Component state
const saving = ref(false)
const loading = ref(false)
const resetting = ref(false)
const showResetDialog = ref(false)
const successMessage = ref<string | null>(null)
const errorMessage = ref<string | null>(null)

// Emits
const emit = defineEmits<{
  saved: []
  loaded: []
  reset: []
}>()

/**
 * Clear status messages after a delay
 */
const clearStatusMessages = () => {
  setTimeout(() => {
    successMessage.value = null
    errorMessage.value = null
  }, 3000)
}

/**
 * Handle save button click
 */
const handleSave = async () => {
  try {
    saving.value = true
    errorMessage.value = null
    successMessage.value = null

    // Call save API
    const response = await endpoints.saveLoad.save()

    // Parse the save data to get the actual JSON object
    const saveData = JSON.parse(response.save_data)

    // Generate filename with timestamp
    const filename = generateFilename('satisflow-save')

    // Trigger download
    downloadJson(saveData, filename)

    // Show success message
    successMessage.value = `Saved successfully! (${response.summary.factory_count} factories, ${response.summary.logistics_count} logistics lines)`
    clearStatusMessages()

    // Emit saved event
    emit('saved')
  } catch (err) {
    errorMessage.value = err instanceof Error ? err.message : 'Failed to save'
    clearStatusMessages()

    // Handle error globally
    handleGlobalError(err, { action: 'save' }, {
      userMessage: 'Failed to save engine state. Please try again.'
    })
  } finally {
    saving.value = false
  }
}

/**
 * Handle load button click
 */
const handleLoad = async () => {
  try {
    loading.value = true
    errorMessage.value = null
    successMessage.value = null

    // Open file picker and read JSON
    const fileData = await selectAndReadFile()

    // Convert to JSON string for API
    const saveDataString = JSON.stringify(fileData)

    // Call load API
    const response = await endpoints.saveLoad.load(saveDataString)

    // Navigate to dashboard if we're on a detail page (to avoid 404 errors)
    if (router.currentRoute.value.name !== 'dashboard') {
      await router.push({ name: 'dashboard' })
    }

    // Show success message
    successMessage.value = response.message
    clearStatusMessages()

    // Emit loaded event
    emit('loaded')
  } catch (err) {
    // Check if it's a cancellation
    const errorMsg = err instanceof Error ? err.message : 'Failed to load'
    if (errorMsg.includes('cancelled')) {
      // Don't show error for user cancellation
      return
    }

    errorMessage.value = errorMsg
    clearStatusMessages()

    // Handle error globally
    handleGlobalError(err, { action: 'load' }, {
      userMessage: 'Failed to load engine state. Please check the file and try again.'
    })
  } finally {
    loading.value = false
  }
}

/**
 * Handle reset button click
 */
const handleReset = async () => {
  try {
    resetting.value = true
    errorMessage.value = null
    successMessage.value = null

    // Call reset API
    const response = await endpoints.saveLoad.reset()

    // Close the dialog
    showResetDialog.value = false

    // Navigate to dashboard if we're on a detail page (to avoid 404 errors)
    if (router.currentRoute.value.name !== 'dashboard') {
      await router.push({ name: 'dashboard' })
    }

    // Show success message
    successMessage.value = response.message
    clearStatusMessages()

    // Emit reset event
    emit('reset')
  } catch (err) {
    errorMessage.value = err instanceof Error ? err.message : 'Failed to reset'
    clearStatusMessages()

    // Handle error globally
    handleGlobalError(err, { action: 'reset' }, {
      userMessage: 'Failed to reset engine state. Please try again.'
    })
  } finally {
    resetting.value = false
  }
}
</script>

<style scoped lang="scss">
.save-load-controls {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 0.5rem);
  position: relative;
}

.control-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
}

.status-message {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: var(--spacing-xs, 0.25rem);
  padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
  border-radius: var(--border-radius-md, 0.375rem);
  font-size: var(--font-size-xs, 0.75rem);
  font-weight: var(--font-weight-medium, 500);
  white-space: nowrap;
  z-index: 10;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);

  &.success {
    background-color: var(--color-success-50, #ecfdf5);
    color: var(--color-success-700, #047857);
    border: 1px solid var(--color-success-200, #a7f3d0);
  }

  &.error {
    background-color: var(--color-error-50, #fef2f2);
    color: var(--color-error-700, #b91c1c);
    border: 1px solid var(--color-error-200, #fecaca);
  }
}

// Fade transition
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

// Reset dialog content
.reset-dialog-content {
  p {
    margin: var(--spacing-sm, 0.5rem) 0;
    line-height: 1.5;
  }

  strong {
    color: var(--color-error-700, #b91c1c);
  }

  ul {
    margin: var(--spacing-sm, 0.5rem) 0;
    padding-left: var(--spacing-lg, 1.5rem);

    li {
      margin: var(--spacing-xs, 0.25rem) 0;
    }
  }

  .warning-text {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm, 0.5rem);
    margin-bottom: var(--spacing-md, 1rem);
  }

  .warning-icon {
    color: var(--color-warning-500, #f59e0b);
    flex-shrink: 0;
  }
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm, 0.5rem);
}

// Responsive design
@media (max-width: 768px) {
  .save-load-controls {
    flex-direction: column;
    align-items: stretch;
    gap: var(--spacing-xs, 0.25rem);
  }

  .control-button {
    width: 100%;
    justify-content: center;
  }

  .status-message {
    position: static;
    margin-top: var(--spacing-sm, 0.5rem);
    text-align: center;
  }

  .dialog-footer {
    flex-direction: column-reverse;

    button {
      width: 100%;
    }
  }
}
</style>
