<template>
  <Modal
    :show="show"
    :title="template?.name || 'Blueprint Details'"
    size="xl"
    @close="$emit('close')"
  >
    <div v-if="template" class="blueprint-details">
      <!-- Description -->
      <div v-if="template.description" class="detail-section">
        <p class="description">{{ template.description }}</p>
      </div>

      <!-- Statistics -->
      <div class="detail-section">
        <h3 class="section-title">Statistics</h3>
        <div class="stats-grid">
          <div class="stat-card">
            <svg class="stat-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
            </svg>
            <div class="stat-content">
              <div class="stat-value">{{ template.total_machines }}</div>
              <div class="stat-label">Machines</div>
            </div>
          </div>

          <div class="stat-card">
            <svg class="stat-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
            <div class="stat-content">
              <div class="stat-value">{{ template.total_power.toFixed(1) }}</div>
              <div class="stat-label">MW</div>
            </div>
          </div>

          <div class="stat-card">
            <svg class="stat-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
            </svg>
            <div class="stat-content">
              <div class="stat-value">{{ template.production_lines.length }}</div>
              <div class="stat-label">Production Lines</div>
            </div>
          </div>
        </div>
      </div>

      <!-- External Input Items (not produced internally) -->
      <div v-if="externalInputs.length > 0" class="detail-section">
        <h3 class="section-title">Inputs</h3>
        <div class="items-grid">
          <div
            v-for="[item, quantity] in externalInputs"
            :key="item"
            class="item-card input-item"
          >
            <ItemDisplay :item="item" size="md" />
            <div class="item-info">
              <div class="item-name">{{ formatItemName(item) }}</div>
              <div class="item-rate">{{ quantity.toFixed(1) }}/min</div>
            </div>
          </div>
        </div>
      </div>

      <!-- External Output Items (not consumed internally) -->
      <div v-if="externalOutputs.length > 0" class="detail-section">
        <h3 class="section-title">Outputs</h3>
        <div class="items-grid">
          <div
            v-for="[item, quantity] in externalOutputs"
            :key="item"
            class="item-card output-item"
          >
            <ItemDisplay :item="item" size="md" />
            <div class="item-info">
              <div class="item-name">{{ formatItemName(item) }}</div>
              <div class="item-rate">{{ quantity.toFixed(1) }}/min</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Production Lines -->
      <div v-if="template.production_lines.length > 0" class="detail-section">
        <h3 class="section-title">Production Lines ({{ template.production_lines.length }})</h3>
        <div class="production-lines-list">
          <div
            v-for="(line, index) in template.production_lines"
            :key="line.id || index"
            class="production-line-item"
          >
            <div class="line-header">
              <span class="line-number">Line {{ index + 1 }}</span>
              <span class="line-recipe">{{ line.recipe }}</span>
            </div>
            <div class="line-machines">
              {{ line.machine_groups.length }} machine group(s)
            </div>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="modal-actions">
        <Button variant="secondary" @click="$emit('use-in-factory')">
          <template #icon>
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
            </svg>
          </template>
          Use in Factory
        </Button>
        <Button variant="secondary" @click="$emit('export')">
          <template #icon>
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
          </template>
          Export
        </Button>
        <Button variant="secondary" @click="$emit('edit')">
          <template #icon>
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
            </svg>
          </template>
          Edit
        </Button>
        <Button variant="danger" @click="$emit('delete')">
          <template #icon>
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </template>
          Delete
        </Button>
        <Button variant="primary" @click="$emit('close')">
          Close
        </Button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { BlueprintTemplateResponse } from '@/api/types';
import Modal from '@/components/ui/Modal.vue';
import Button from '@/components/ui/Button.vue';
import ItemDisplay from '@/components/ui/ItemDisplay.vue';

interface Props {
  show: boolean;
  template: BlueprintTemplateResponse | null;
}

const props = defineProps<Props>();

defineEmits<{
  close: [];
  'use-in-factory': [];
  export: [];
  edit: [];
  delete: [];
}>();

// Filter out intermediate items - only show external inputs and outputs
const externalInputs = computed(() => {
  if (!props.template) return [];
  const outputItemSet = new Set(props.template.output_items.map(([item]) => item));
  return props.template.input_items.filter(([item]) => !outputItemSet.has(item));
});

const externalOutputs = computed(() => {
  if (!props.template) return [];
  const inputItemSet = new Set(props.template.input_items.map(([item]) => item));
  return props.template.output_items.filter(([item]) => !inputItemSet.has(item));
});

const formatItemName = (item: string): string => {
  return item.replace(/([A-Z])/g, ' $1').trim();
};
</script>

<style scoped>
.blueprint-details {
  @apply space-y-6;
}

.detail-section {
  @apply space-y-4;
}

.description {
  @apply text-base text-gray-700 dark:text-gray-300 leading-relaxed;
}

.section-title {
  @apply text-lg font-semibold text-gray-900 dark:text-gray-100;
  color: var(--color-ficsit-orange, #f58b00);
}

.stats-grid {
  @apply grid grid-cols-3 gap-4;
}

.stat-card {
  @apply bg-gray-50 dark:bg-gray-700/50 rounded-lg p-4 flex items-center gap-3;
}

.stat-icon {
  @apply w-8 h-8 text-gray-500 dark:text-gray-400 flex-shrink-0;
}

.stat-content {
  @apply flex flex-col;
}

.stat-value {
  @apply text-xl font-bold text-gray-900 dark:text-gray-100;
}

.stat-label {
  @apply text-sm text-gray-500 dark:text-gray-400;
}

.items-grid {
  @apply grid grid-cols-2 md:grid-cols-3 gap-3;
}

.item-card {
  @apply rounded-lg p-3 flex items-center gap-3;
}

.input-item {
  @apply bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800;
}

.output-item {
  @apply bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800;
}

.item-info {
  @apply flex flex-col;
}

.item-name {
  @apply text-sm font-medium text-gray-900 dark:text-gray-100;
}

.item-rate {
  @apply text-xs text-gray-500 dark:text-gray-400;
}

.production-lines-list {
  @apply space-y-2;
}

.production-line-item {
  @apply bg-gray-50 dark:bg-gray-700/50 rounded-lg p-3;
}

.line-header {
  @apply flex items-center justify-between mb-1;
}

.line-number {
  @apply text-sm font-semibold text-gray-700 dark:text-gray-300;
}

.line-recipe {
  @apply text-sm text-gray-600 dark:text-gray-400;
}

.line-machines {
  @apply text-xs text-gray-500 dark:text-gray-400;
}

.modal-actions {
  @apply flex items-center justify-end gap-2 flex-wrap;
}
</style>
