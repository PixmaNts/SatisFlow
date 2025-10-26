<template>
  <div class="save-load-controls">
    <!-- Save Button -->
    <Button
      variant="secondary"
      size="sm"
      :loading="saving"
      :disabled="saving || loading"
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
      :disabled="saving || loading"
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
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Button } from '@/components/ui'
import { useFileDownload } from '@/composables/useFileDownload'
import { useFileUpload } from '@/composables/useFileUpload'
import { useErrorHandler } from '@/composables/useErrorHandler'
import { endpoints } from '@/api'

/**
 * SaveLoadControls component
 *
 * Provides save and load functionality for the engine state
 */

// Composables
const { downloadJson, generateFilename } = useFileDownload()
const { selectAndReadFile } = useFileUpload()
const { handleError: handleGlobalError } = useErrorHandler()

// Component state
const saving = ref(false)
const loading = ref(false)
const successMessage = ref<string | null>(null)
const errorMessage = ref<string | null>(null)

// Emits
const emit = defineEmits<{
  saved: []
  loaded: []
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
}
</style>
