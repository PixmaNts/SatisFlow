<template>
  <div class="page-header">
    <div class="header-content">
      <div class="header-text">
        <h1 class="page-title">{{ title }}</h1>
        <p v-if="subtitle" class="page-subtitle">{{ subtitle }}</p>
      </div>

      <div v-if="$slots.actions" class="header-actions">
        <slot name="actions" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  /** Page title */
  title: string
  /** Optional page subtitle */
  subtitle?: string
}

defineProps<Props>()
</script>

<style scoped>
.page-header {
  @apply mb-8 pb-6 border-b border-gray-200 dark:border-gray-700;
  padding-left: 0;
}

.header-content {
  @apply flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4;
}

.header-text {
  @apply flex-1;
}

.page-title {
  @apply text-3xl font-bold text-gray-900 dark:text-gray-100 mb-2;
  color: var(--color-ficsit-orange, #f58b00);
  letter-spacing: -0.02em;
}

.page-subtitle {
  @apply text-base text-gray-600 dark:text-gray-400;
  line-height: 1.5;
}

.header-actions {
  @apply flex items-center gap-3 flex-shrink-0;
}

/* Responsive adjustments */
@media (max-width: 640px) {
  .page-header {
    @apply mb-6 pb-4;
  }
  
  .page-title {
    @apply text-2xl;
  }
  
  .page-subtitle {
    @apply text-sm;
  }
}
</style>
