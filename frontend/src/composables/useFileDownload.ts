import { ref } from 'vue'

/**
 * File Download Composable
 *
 * Provides functionality to trigger browser downloads for JSON data
 */
export function useFileDownload() {
  const downloading = ref(false)
  const error = ref<string | null>(null)

  /**
   * Trigger a file download in the browser
   * @param data - The data to download (will be JSON stringified)
   * @param filename - The name of the file to download
   */
  const downloadJson = (data: unknown, filename: string) => {
    try {
      downloading.value = true
      error.value = null

      // Convert data to JSON string with pretty formatting
      const jsonString = JSON.stringify(data, null, 2)

      // Create a Blob from the JSON string
      const blob = new Blob([jsonString], { type: 'application/json' })

      // Create a temporary URL for the blob
      const url = URL.createObjectURL(blob)

      // Create a temporary anchor element and trigger download
      const link = document.createElement('a')
      link.href = url
      link.download = filename

      // Append to body, click, and remove
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)

      // Clean up the URL object
      URL.revokeObjectURL(url)
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to download file'
      console.error('Download error:', err)
      throw err
    } finally {
      downloading.value = false
    }
  }

  /**
   * Generate a timestamped filename
   * @param prefix - Prefix for the filename (default: 'satisflow')
   * @param extension - File extension (default: 'json')
   * @returns Filename with timestamp
   */
  const generateFilename = (prefix = 'satisflow', extension = 'json'): string => {
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, -5)
    return `${prefix}_${timestamp}.${extension}`
  }

  /**
   * Clear any error state
   */
  const clearError = () => {
    error.value = null
  }

  return {
    downloading,
    error,
    downloadJson,
    generateFilename,
    clearError
  }
}
