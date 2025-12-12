<template>
  <Modal
    :show="open"
    title="Settings"
    @close="$emit('close')"
    @update:show="$emit('close')"
  >
    <div class="settings-content">
      <div class="settings-section">
        <h3 class="section-title">Language</h3>

        <div class="setting-group">
          <label class="setting-label">Interface Language</label>
          <select
            v-model="currentLanguage"
            class="language-select"
            @change="handleLanguageChange"
          >
            <option value="en">English</option>
            <option value="fr">Français</option>
            <option value="de">Deutsch</option>
            <option value="es">Español</option>
            <option value="zh">中文</option>
          </select>
        </div>
      </div>

      <div class="settings-section">
        <h3 class="section-title">Dashboard</h3>

        <div class="setting-group">
          <label class="setting-label">Auto-refresh</label>
          <div class="toggle-container">
            <label class="toggle">
              <input
                v-model="autoRefresh"
                type="checkbox"
                @change="handleAutoRefreshChange"
              />
              <span class="toggle-slider"></span>
            </label>
            <span class="toggle-description">
              Automatically refresh dashboard data
            </span>
          </div>
        </div>

        <div class="setting-group" v-if="autoRefresh">
          <label class="setting-label">Refresh Interval</label>
          <div class="interval-container">
            <input
              v-model.number="refreshInterval"
              type="number"
              min="5"
              max="300"
              step="5"
              class="interval-input"
              @change="handleRefreshIntervalChange"
            />
            <span class="interval-unit">seconds</span>
          </div>
        </div>
      </div>

      <div class="settings-section">
        <h3 class="section-title">Data Management</h3>

        <div class="setting-group">
          <button
            class="button button-secondary"
            @click="handleExportPreferences"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M21 15V19C21 20.1046 20.1046 21 19 21H5C3.89543 21 3 20.1046 3 19V15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M7 10L12 15L17 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M12 15V3" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            Export Preferences
          </button>
        </div>

        <div class="setting-group">
          <button
            class="button button-secondary"
            @click="handleImportPreferences"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M21 15V19C21 20.1046 20.1046 21 19 21H5C3.89543 21 3 20.1046 3 19V15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M17 10L12 15L7 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M12 15V3" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            Import Preferences
          </button>
          <input
            ref="fileInput"
            type="file"
            accept=".json"
            style="display: none"
            @change="handleFileImport"
          />
        </div>

        <div class="setting-group">
          <button
            class="button button-danger"
            @click="handleResetPreferences"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M3 12C3 12 5 5 12 5C19 5 21 12 21 12C21 12 19 19 12 19C5 19 3 12 3 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M12 15C13.6569 15 15 13.6569 15 12C15 10.3431 13.6569 9 12 9C10.3431 9 9 10.3431 9 12C9 13.6569 10.3431 15 12 15Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            Reset to Defaults
          </button>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="settings-footer">
        <div class="footer-info">
          <span v-if="lastSaved" class="last-saved">
            Last saved: {{ formatTime(lastSaved) }}
          </span>
        </div>
        <div class="footer-actions">
          <button
            class="button button-secondary"
            @click="$emit('close')"
          >
            Close
          </button>
        </div>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { usePreferencesStore } from '@/stores/preferences'
import Modal from '@/components/ui/Modal.vue'

// Props
interface Props {
  open: boolean
}

defineProps<Props>()

// Emits
const emit = defineEmits<{
  close: []
}>()

// Store
const preferencesStore = usePreferencesStore()

// Refs
const fileInput = ref<HTMLInputElement>()
const lastSaved = ref<Date | null>(null)

// Current values
const currentLanguage = computed({
  get: () => preferencesStore.uiPreferences.language,
  set: (value) => preferencesStore.setLanguage(value)
})

const autoRefresh = computed({
  get: () => preferencesStore.uiPreferences.autoRefresh,
  set: (value) => preferencesStore.setAutoRefresh(value)
})

const refreshInterval = computed({
  get: () => preferencesStore.uiPreferences.refreshInterval,
  set: (value) => preferencesStore.setRefreshInterval(value)
})

// Methods
const handleLanguageChange = () => {
  updateLastSaved()
}

const handleAutoRefreshChange = () => {
  updateLastSaved()
}

const handleRefreshIntervalChange = () => {
  updateLastSaved()
}

const handleExportPreferences = () => {
  try {
    const data = preferencesStore.exportPreferences()
    const blob = new Blob([data], { type: 'application/json' })
    const url = URL.createObjectURL(blob)

    const a = document.createElement('a')
    a.href = url
    a.download = `satisflow-preferences-${new Date().toISOString().split('T')[0]}.json`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  } catch (error) {
    console.error('Failed to export preferences:', error)
  }
}

const handleImportPreferences = () => {
  fileInput.value?.click()
}

const handleFileImport = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]

  if (!file) return

  const reader = new FileReader()
  reader.onload = (e) => {
    try {
      const content = e.target?.result as string
      const success = preferencesStore.importPreferences(content)

      if (success) {
        updateLastSaved()
      } else {
        console.error('Failed to import preferences: Invalid format')
      }
    } catch (error) {
      console.error('Failed to import preferences:', error)
    }
  }

  reader.readAsText(file)

  // Reset file input
  if (target) {
    target.value = ''
  }
}

const handleResetPreferences = () => {
  if (confirm('Are you sure you want to reset all preferences to their default values? This action cannot be undone.')) {
    preferencesStore.resetPreferences()
    updateLastSaved()
  }
}

const updateLastSaved = () => {
  lastSaved.value = new Date()
}

const formatTime = (date: Date): string => {
  return date.toLocaleTimeString()
}

// Watch for preference changes to update last saved time
watch(
  () => preferencesStore.preferences,
  () => {
    updateLastSaved()
  },
  { deep: true }
)
</script>

<style scoped lang="scss">
.settings-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xl, 1.25rem);
  max-height: 70vh;
  overflow-y: auto;
  color: var(--color-text-primary, #e5e5e5);
}

.settings-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg, 1rem);
}

.section-title {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-text-primary, #e5e5e5);
  margin: 0;
  padding-bottom: var(--spacing-sm, 0.5rem);
  border-bottom: 1px solid var(--color-border, #404040);
}

.setting-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm, 0.5rem);
}

.setting-label {
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-secondary, #b8b8b8);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}


.language-select {
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  border: 1px solid var(--color-border, #404040);
  border-radius: var(--border-radius-sm, 3px);
  background-color: var(--color-surface-inset, #1f1f1f);
  color: var(--color-text-primary, #e5e5e5);
  font-size: var(--font-size-sm, 0.875rem);
  transition: all var(--transition-normal, 200ms);
  box-shadow: var(--shadow-inset-light);

  &:focus {
    outline: none;
    border-color: var(--color-ficsit-orange, #f58b00);
    box-shadow: var(--shadow-glow-orange);
    background-color: var(--color-surface, #252525);
  }

  &:hover {
    border-color: var(--color-border-light, #4a4a4a);
    background-color: var(--color-surface, #252525);
  }
}

.toggle-container {
  display: flex;
  align-items: center;
  gap: var(--spacing-md, 0.75rem);
}

.toggle {
  position: relative;
  display: inline-block;
  width: 48px;
  height: 24px;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--color-gray-300, #d1d5db);
  transition: var(--transition-normal, 300ms ease-in-out);
  border-radius: var(--border-radius-full, 9999px);

  &:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 3px;
    bottom: 3px;
    background-color: var(--color-text-primary, #e5e5e5);
    transition: var(--transition-normal, 200ms);
    border-radius: 50%;
  }
}

.toggle input:checked + .toggle-slider {
  background-color: var(--color-ficsit-orange, #f58b00);
  border-color: var(--color-ficsit-orange, #f58b00);
}

.toggle input:checked + .toggle-slider:before {
  transform: translateX(24px);
  background-color: var(--color-text-primary, #e5e5e5);
}

.toggle-description {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-secondary, #b8b8b8);
}

.interval-container {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 0.5rem);
}

.interval-input {
  width: 80px;
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  border: 1px solid var(--color-border, #404040);
  border-radius: var(--border-radius-sm, 3px);
  background-color: var(--color-surface-inset, #1f1f1f);
  color: var(--color-text-primary, #e5e5e5);
  font-size: var(--font-size-sm, 0.875rem);
  font-family: var(--font-family-mono);
  transition: all var(--transition-normal, 200ms);
  box-shadow: var(--shadow-inset-light);

  &:focus {
    outline: none;
    border-color: var(--color-ficsit-orange, #f58b00);
    box-shadow: var(--shadow-glow-orange);
    background-color: var(--color-surface, #252525);
  }

  &:hover {
    border-color: var(--color-border-light, #4a4a4a);
    background-color: var(--color-surface, #252525);
  }
}

.interval-unit {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-secondary, #b8b8b8);
  font-family: var(--font-family-mono);
}

.button {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  border: 1px solid transparent;
  border-radius: var(--border-radius-sm, 3px);
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  cursor: pointer;
  transition: all var(--transition-normal, 200ms);

  &.button-secondary {
    background-color: var(--color-surface, #252525);
    color: var(--color-text-primary, #e5e5e5);
    border-color: var(--color-border, #404040);
    box-shadow: var(--shadow-inset-light);

    &:hover {
      background-color: var(--color-surface-hover, #2a2a2a);
      border-color: var(--color-ficsit-orange, #f58b00);
      color: var(--color-ficsit-orange, #f58b00);
    }
  }

  &.button-danger {
    background-color: var(--color-error, #ef4444);
    color: var(--color-text-primary, #e5e5e5);
    border-color: var(--color-error-dark, #dc2626);

    &:hover {
      background-color: var(--color-error-dark, #dc2626);
      box-shadow: 0 0 8px rgba(239, 68, 68, 0.4);
    }
  }
}

.settings-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-md, 0.75rem);
}

.footer-info {
  flex: 1;
}

.last-saved {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-text-muted, #9ca3af);
}

.footer-actions {
  display: flex;
  gap: var(--spacing-sm, 0.5rem);
}

// Industrial theme is always dark - no theme switching needed
</style>
