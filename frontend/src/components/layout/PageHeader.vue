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
  @apply mb-6;
}

.header-content {
  @apply flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4;
}

.header-text {
  @apply flex-1;
}

.page-title {
  @apply text-2xl font-bold text-gray-900 dark:text-gray-100;
}

.page-subtitle {
  @apply mt-1 text-sm text-gray-600 dark:text-gray-400;
}

.header-actions {
  @apply flex items-center gap-3;
}
</style>
