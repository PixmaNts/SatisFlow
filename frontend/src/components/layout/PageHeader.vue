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

<style scoped lang="scss">
.page-header {
  margin-bottom: var(--spacing-8, 2rem);
  padding-bottom: var(--spacing-6, 1.5rem);
  padding-left: 0;
  border-bottom: 1px solid var(--color-border, #404040);
}

.header-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4, 1rem);
}

@media (min-width: 640px) {
  .header-content {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
  }
}

.header-text {
  flex: 1 1 0%;
}

.page-title {
  font-size: var(--font-size-3xl, 1.875rem);
  font-weight: var(--font-weight-bold, 700);
  color: var(--color-ficsit-orange, #f58b00);
  margin-bottom: var(--spacing-2, 0.5rem);
  letter-spacing: -0.02em;
}

.page-subtitle {
  font-size: var(--font-size-base, 1rem);
  color: var(--color-text-secondary, #b8b8b8);
  line-height: 1.5;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-3, 0.75rem);
  flex-shrink: 0;
}

/* Responsive adjustments */
@media (max-width: 640px) {
  .page-header {
    margin-bottom: var(--spacing-6, 1.5rem);
    padding-bottom: var(--spacing-4, 1rem);
  }

  .page-title {
    font-size: var(--font-size-2xl, 1.5rem);
  }

  .page-subtitle {
    font-size: var(--font-size-sm, 0.875rem);
  }
}
</style>
