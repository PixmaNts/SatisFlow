import { ref, watch, onMounted, onUnmounted } from 'vue'

export interface FormDraftOptions {
  /** Unique key for storing the draft in localStorage */
  key: string
  /** Auto-save interval in milliseconds (default: 30000 = 30 seconds) */
  autoSaveInterval?: number
  /** Whether to restore draft on mount (default: true) */
  restoreOnMount?: boolean
  /** Whether to clear draft on successful submit (default: true) */
  clearOnSubmit?: boolean
}

/**
 * Composable for managing form drafts with auto-save functionality
 * 
 * @example
 * ```ts
 * const { formData, saveDraft, restoreDraft, clearDraft } = useFormDraft({
 *   key: 'factory-form-draft',
 *   autoSaveInterval: 30000
 * })
 * ```
 */
export function useFormDraft<T extends Record<string, unknown>>(
  initialData: T,
  options: FormDraftOptions
) {
  const {
    key,
    autoSaveInterval = 30000,
    restoreOnMount = true,
    clearOnSubmit = true
  } = options

  const formData = ref<T>({ ...initialData } as T)
  const hasDraft = ref(false)
  let autoSaveTimer: number | null = null

  /**
   * Save current form data to localStorage
   */
  const saveDraft = () => {
    try {
      const draftData = JSON.stringify(formData.value)
      localStorage.setItem(`form-draft-${key}`, draftData)
      hasDraft.value = true
    } catch (error) {
      console.error('Failed to save form draft:', error)
    }
  }

  /**
   * Restore draft from localStorage
   */
  const restoreDraft = (): boolean => {
    try {
      const draftData = localStorage.getItem(`form-draft-${key}`)
      if (draftData) {
        const parsed = JSON.parse(draftData) as T
        formData.value = { ...parsed }
        hasDraft.value = true
        return true
      }
    } catch (error) {
      console.error('Failed to restore form draft:', error)
    }
    return false
  }

  /**
   * Clear draft from localStorage
   */
  const clearDraft = () => {
    try {
      localStorage.removeItem(`form-draft-${key}`)
      hasDraft.value = false
    } catch (error) {
      console.error('Failed to clear form draft:', error)
    }
  }

  /**
   * Start auto-save timer
   */
  const startAutoSave = () => {
    stopAutoSave()
    autoSaveTimer = window.setInterval(() => {
      saveDraft()
    }, autoSaveInterval)
  }

  /**
   * Stop auto-save timer
   */
  const stopAutoSave = () => {
    if (autoSaveTimer !== null) {
      clearInterval(autoSaveTimer)
      autoSaveTimer = null
    }
  }

  /**
   * Reset form data to initial state
   */
  const resetForm = () => {
    formData.value = { ...initialData } as T
    if (clearOnSubmit) {
      clearDraft()
    }
  }

  // Auto-save on form data changes (debounced)
  let saveTimeout: number | null = null
  watch(
    () => formData.value,
    () => {
      if (saveTimeout !== null) {
        clearTimeout(saveTimeout)
      }
      saveTimeout = window.setTimeout(() => {
        saveDraft()
      }, 2000) // Debounce: save 2 seconds after last change
    },
    { deep: true }
  )

  // Restore draft on mount
  onMounted(() => {
    if (restoreOnMount) {
      const restored = restoreDraft()
      if (restored) {
        // Optionally show a notification that draft was restored
        console.log(`Draft restored for form: ${key}`)
      }
    }
    startAutoSave()
  })

  // Clean up on unmount
  onUnmounted(() => {
    stopAutoSave()
    if (saveTimeout !== null) {
      clearTimeout(saveTimeout)
    }
  })

  return {
    formData,
    hasDraft,
    saveDraft,
    restoreDraft,
    clearDraft,
    resetForm,
    startAutoSave,
    stopAutoSave
  }
}





