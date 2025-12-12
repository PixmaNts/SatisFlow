<template>
  <Modal
    :show="show"
    title="Import Blueprint"
    :closable="true"
    :close-on-overlay="true"
    @close="emit('close')"
  >
    <div v-if="metadata" class="blueprint-preview">
      <!-- Blueprint Name -->
      <div class="preview-section">
        <h3 class="section-title">Blueprint Information</h3>
        <div class="info-grid">
          <div class="info-item">
            <span class="info-label">Name:</span>
            <span class="info-value">{{ metadata.name }}</span>
          </div>
          <div v-if="metadata.description" class="info-item full-width">
            <span class="info-label">Description:</span>
            <span class="info-value">{{ metadata.description }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Exported:</span>
            <span class="info-value">{{ formatDate(metadata.exported_at) }}</span>
          </div>
        </div>
      </div>

      <!-- Stats -->
      <div class="preview-section">
        <h3 class="section-title">Statistics</h3>
        <div class="stats-grid">
          <div class="stat-card">
            <div class="stat-icon machines">⚙️</div>
            <div class="stat-content">
              <div class="stat-value">{{ metadata.total_machines }}</div>
              <div class="stat-label">Machines</div>
            </div>
          </div>
          <div class="stat-card">
            <div class="stat-icon power">⚡</div>
            <div class="stat-content">
              <div class="stat-value">{{ formatPower(metadata.total_power) }}</div>
              <div class="stat-label">Power</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Input Items -->
      <div v-if="metadata.input_items.length > 0" class="preview-section">
        <h3 class="section-title">Inputs</h3>
        <div class="items-list">
          <div v-for="[item, rate] in metadata.input_items" :key="item" class="item-row">
            <ItemDisplay :item="item" size="sm" />
            <span class="item-rate">{{ rate.toFixed(2) }}/min</span>
          </div>
        </div>
      </div>

      <!-- Output Items -->
      <div v-if="metadata.output_items.length > 0" class="preview-section">
        <h3 class="section-title">Outputs</h3>
        <div class="items-list">
          <div v-for="[item, rate] in metadata.output_items" :key="item" class="item-row">
            <ItemDisplay :item="item" size="sm" />
            <span class="item-rate">{{ rate.toFixed(2) }}/min</span>
          </div>
        </div>
      </div>

      <!-- Name Override -->
      <div class="preview-section">
        <h3 class="section-title">Import Options</h3>
        <div class="form-field">
          <label for="blueprint-name" class="field-label">
            Blueprint Name (optional override)
          </label>
          <input
            id="blueprint-name"
            v-model="customName"
            type="text"
            class="field-input"
            :placeholder="metadata.name"
          />
        </div>
      </div>
    </div>

    <template #footer>
      <Button variant="secondary" @click="emit('close')">
        Cancel
      </Button>
      <Button variant="primary" @click="handleImport">
        Import Blueprint
      </Button>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import Modal from '../ui/Modal.vue'
import Button from '../ui/Button.vue'
import ItemDisplay from '../ui/ItemDisplay.vue'
import type { BlueprintMetadata } from '@/api/types'

/**
 * Component props
 */
interface Props {
  /** Whether the modal is visible */
  show: boolean
  /** Blueprint metadata to display */
  metadata: BlueprintMetadata | null
}

defineProps<Props>()

/**
 * Component events
 */
const emit = defineEmits<{
  close: []
  import: [name?: string]
}>()

// Custom name override
const customName = ref<string>('')

/**
 * Handle import button click
 */
const handleImport = () => {
  emit('import', customName.value || undefined)
}

/**
 * Format power value with MW unit
 */
const formatPower = (power: number): string => {
  if (power >= 1000) {
    return `${(power / 1000).toFixed(2)} GW`
  }
  return `${power.toFixed(2)} MW`
}


/**
 * Format date string
 */
const formatDate = (dateString: string): string => {
  const date = new Date(dateString)
  return date.toLocaleString()
}
</script>

<style scoped lang="scss">
.blueprint-preview {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  max-width: 600px;
}

.preview-section {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.section-title {
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-gray-900, #111827);
  margin: 0;
  padding-bottom: 0.5rem;
  border-bottom: 2px solid var(--color-gray-200, #e5e7eb);
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 0.75rem;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;

  &.full-width {
    grid-column: 1 / -1;
  }
}

.info-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-gray-600, #4b5563);
}

.info-value {
  font-size: 1rem;
  color: var(--color-gray-900, #111827);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem;
  background-color: var(--color-gray-50, #f9fafb);
  border-radius: var(--border-radius-lg, 0.5rem);
  border: 1px solid var(--color-gray-200, #e5e7eb);
}

.stat-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 3rem;
  height: 3rem;
  font-size: 1.5rem;
  border-radius: var(--border-radius-full, 9999px);

  &.machines {
    background-color: #dbeafe;
  }

  &.power {
    background-color: #fef3c7;
  }
}

.stat-content {
  display: flex;
  flex-direction: column;
  gap: 0.125rem;
}

.stat-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--color-gray-900, #111827);
  line-height: 1;
}

.stat-label {
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--color-gray-600, #4b5563);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.items-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  max-height: 200px;
  overflow-y: auto;
}

.item-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.625rem;
  background-color: var(--color-gray-50, #f9fafb);
  border-radius: var(--border-radius-md, 0.375rem);
  border: 1px solid var(--color-gray-200, #e5e7eb);
}

.item-rate {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--color-gray-900, #111827);
  font-family: 'Monaco', 'Courier New', monospace;
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.field-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-gray-700, #374151);
}

.field-input {
  padding: 0.625rem;
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-md, 0.375rem);
  font-size: 0.875rem;
  color: var(--color-gray-900, #111827);
  transition: all 0.2s ease-in-out;

  &:focus {
    outline: none;
    border-color: var(--color-primary-500, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  &::placeholder {
    color: var(--color-gray-400, #9ca3af);
  }
}

// Responsive design
@media (max-width: 640px) {
  .stats-grid {
    grid-template-columns: 1fr;
  }

  .info-grid {
    grid-template-columns: 1fr;
  }
}
</style>

<script lang="ts">
export default {
  name: 'BlueprintPreviewModal'
}
</script>
