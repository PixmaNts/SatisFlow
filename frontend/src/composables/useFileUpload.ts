import { ref } from 'vue'

/**
 * File Upload Composable
 *
 * Provides functionality to handle file selection and JSON parsing
 */
export function useFileUpload() {
  const uploading = ref(false)
  const error = ref<string | null>(null)

  /**
   * Read a file and parse it as JSON
   * @param file - The file to read
   * @returns Promise resolving to the parsed JSON data
   */
  const readJsonFile = (file: File): Promise<unknown> => {
    return new Promise((resolve, reject) => {
      // Validate file type
      if (!file.name.endsWith('.json') && file.type !== 'application/json') {
        reject(new Error('Please select a valid JSON file'))
        return
      }

      // Validate file size (max 50MB)
      const maxSize = 50 * 1024 * 1024
      if (file.size > maxSize) {
        reject(new Error('File is too large (max 50MB)'))
        return
      }

      const reader = new FileReader()

      reader.onload = (event) => {
        try {
          const text = event.target?.result as string
          const data = JSON.parse(text)
          resolve(data)
        } catch (err) {
          reject(new Error('Invalid JSON file format'))
        }
      }

      reader.onerror = () => {
        reject(new Error('Failed to read file'))
      }

      reader.readAsText(file)
    })
  }

  /**
   * Handle file selection from input element
   * @param event - The file input change event
   * @returns Promise resolving to the parsed JSON data
   */
  const handleFileSelect = async (event: Event): Promise<unknown> => {
    const input = event.target as HTMLInputElement
    const file = input.files?.[0]

    if (!file) {
      throw new Error('No file selected')
    }

    try {
      uploading.value = true
      error.value = null

      const data = await readJsonFile(file)

      // Reset the input so the same file can be selected again
      input.value = ''

      return data
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to upload file'
      console.error('Upload error:', err)
      throw err
    } finally {
      uploading.value = false
    }
  }

  /**
   * Trigger file selection dialog
   * @returns Promise resolving to the parsed JSON data
   */
  const selectAndReadFile = (): Promise<unknown> => {
    return new Promise((resolve, reject) => {
      const input = document.createElement('input')
      input.type = 'file'
      input.accept = '.json,application/json'

      input.onchange = async (event) => {
        try {
          const data = await handleFileSelect(event)
          resolve(data)
        } catch (err) {
          reject(err)
        }
      }

      input.oncancel = () => {
        reject(new Error('File selection cancelled'))
      }

      input.click()
    })
  }

  /**
   * Clear any error state
   */
  const clearError = () => {
    error.value = null
  }

  return {
    uploading,
    error,
    readJsonFile,
    handleFileSelect,
    selectAndReadFile,
    clearError
  }
}
