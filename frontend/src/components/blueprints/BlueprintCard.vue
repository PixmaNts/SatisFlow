<template>
  <div class="blueprint-card">
    <!-- Card Header -->
    <div class="card-header">
      <h3 class="blueprint-name">{{ template.name }}</h3>
      <div class="card-actions">
        <Button
          variant="secondary"
          size="sm"
          @click="$emit('use-in-factory', template)"
          title="Use in Factory"
        >
          <template #icon>
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
            </svg>
          </template>
        </Button>

        <Button
          variant="secondary"
          size="sm"
          @click="$emit('export', template)"
          title="Export Blueprint"
        >
          <template #icon>
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
          </template>
        </Button>

        <Button
          variant="secondary"
          size="sm"
          @click="$emit('edit', template)"
          title="Edit Blueprint"
        >
          <template #icon>
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
            </svg>
          </template>
        </Button>

        <Button
          variant="secondary"
          size="sm"
          @click="$emit('delete', template)"
          title="Delete Blueprint"
          class="delete-button"
        >
          <template #icon>
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </template>
        </Button>
      </div>
    </div>

    <!-- Card Description -->
    <p v-if="template.description" class="blueprint-description">
      {{ template.description }}
    </p>

    <!-- Card Stats -->
    <div class="card-stats">
      <div class="stat-item">
        <svg class="stat-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
        </svg>
        <span class="stat-value">{{ template.total_machines }}</span>
        <span class="stat-label">Machines</span>
      </div>

      <div class="stat-item">
        <svg class="stat-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
        </svg>
        <span class="stat-value">{{ template.total_power.toFixed(1) }}</span>
        <span class="stat-label">MW</span>
      </div>

      <div class="stat-item">
        <svg class="stat-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" width="24" height="24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
        </svg>
        <span class="stat-value">{{ template.production_lines.length }}</span>
        <span class="stat-label">Lines</span>
      </div>
    </div>

    <!-- Input/Output Items -->
    <div class="items-section">
      <div v-if="template.input_items.length > 0" class="items-group">
        <h4 class="items-title">Inputs</h4>
        <div class="items-list">
          <div
            v-for="[item, quantity] in template.input_items"
            :key="item"
            class="item-tag input-item"
          >
            <ItemDisplay :item="item" size="sm" />
            <span>{{ quantity.toFixed(1) }}/min</span>
          </div>
        </div>
      </div>

      <div v-if="template.output_items.length > 0" class="items-group">
        <h4 class="items-title">Outputs</h4>
        <div class="items-list">
          <div
            v-for="[item, quantity] in template.output_items"
            :key="item"
            class="item-tag output-item"
          >
            <ItemDisplay :item="item" size="sm" />
            <span>{{ quantity.toFixed(1) }}/min</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { BlueprintTemplateResponse } from '@/api/types';
import Button from '@/components/ui/Button.vue';
import ItemDisplay from '@/components/ui/ItemDisplay.vue';

interface Props {
  /** Blueprint template data */
  template: BlueprintTemplateResponse;
}

defineProps<Props>();

defineEmits<{
  'use-in-factory': [template: BlueprintTemplateResponse];
  export: [template: BlueprintTemplateResponse];
  edit: [template: BlueprintTemplateResponse];
  delete: [template: BlueprintTemplateResponse];
}>();
</script>

<style scoped>
.blueprint-card {
  @apply bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-4 hover:shadow-md transition-shadow duration-200;
}

.card-header {
  @apply flex items-start justify-between mb-3;
}

.blueprint-name {
  @apply text-lg font-semibold text-gray-900 dark:text-gray-100 mr-2;
}

.card-actions {
  @apply flex items-center gap-1 opacity-0 transition-opacity duration-200;
}

.blueprint-card:hover .card-actions {
  @apply opacity-100;
}

.blueprint-description {
  @apply text-sm text-gray-600 dark:text-gray-400 mb-4 line-clamp-2;
}

.card-stats {
  @apply flex items-center gap-4 mb-4;
}

.stat-item {
  @apply flex flex-col items-center;
}

.stat-icon {
  @apply w-4 h-4 text-gray-500 dark:text-gray-400 mb-1;
}

.stat-value {
  @apply text-sm font-semibold text-gray-900 dark:text-gray-100;
}

.stat-label {
  @apply text-xs text-gray-500 dark:text-gray-400;
}

.items-section {
  @apply space-y-3;
}

.items-group {
  @apply border-t border-gray-100 dark:border-gray-700 pt-3;
}

.items-title {
  @apply text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-2;
}

.items-list {
  @apply flex flex-wrap gap-1;
}

.item-tag {
  @apply px-2 py-1 text-xs rounded-full font-medium flex items-center gap-1;
}

.input-item {
  @apply bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200;
}

.output-item {
  @apply bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200;
}

.delete-button:hover {
  @apply text-red-600 dark:text-red-400;
}

/* Line clamp utility */
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
