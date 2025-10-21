<script setup lang="ts">
import SkeletonLoader from './SkeletonLoader.vue'

interface Props {
  rows?: number
  columns?: number
  showHeader?: boolean
  showFooter?: boolean
  headerHeight?: string | number
  rowHeight?: string | number
  cellSpacing?: string
}

withDefaults(defineProps<Props>(), {
  rows: 5,
  columns: 4,
  showHeader: true,
  showFooter: false,
  headerHeight: '2.5rem',
  rowHeight: '2.5rem',
  cellSpacing: '1rem'
})
</script>

<template>
  <div class="skeleton-table">
    <!-- Header -->
    <div v-if="showHeader" class="skeleton-table-header">
      <SkeletonLoader
        v-for="i in columns"
        :key="`header-${i}`"
        :width="i === columns ? '60%' : '100%'"
        :height="headerHeight"
        variant="rectangular"
        class="skeleton-header-cell"
      />
    </div>

    <!-- Body rows -->
    <div class="skeleton-table-body">
      <div
        v-for="row in rows"
        :key="`row-${row}`"
        class="skeleton-table-row"
      >
        <SkeletonLoader
          v-for="col in columns"
          :key="`cell-${row}-${col}`"
          :width="col === 1 ? '80%' : col === columns ? '60%' : '100%'"
          :height="rowHeight"
          variant="text"
          class="skeleton-table-cell"
        />
      </div>
    </div>

    <!-- Footer -->
    <div v-if="showFooter" class="skeleton-table-footer">
      <SkeletonLoader
        :width="'40%'"
        :height="rowHeight"
        variant="rectangular"
        class="skeleton-footer-cell"
      />
    </div>
  </div>
</template>

<style scoped>
.skeleton-table {
  width: 100%;
  border-collapse: collapse;
}

.skeleton-table-header {
  display: flex;
  gap: v-bind(cellSpacing);
  margin-bottom: 0.5rem;
  padding: 0.5rem;
  background-color: var(--color-table-header-bg, #f9fafb);
  border-radius: 4px;
}

.skeleton-table-body {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.skeleton-table-row {
  display: flex;
  gap: v-bind(cellSpacing);
  padding: 0.5rem;
  border-bottom: 1px solid var(--color-border, #e5e7eb);
}

.skeleton-table-footer {
  display: flex;
  gap: v-bind(cellSpacing);
  margin-top: 0.5rem;
  padding: 0.5rem;
  background-color: var(--color-table-footer-bg, #f9fafb);
  border-radius: 4px;
}

.skeleton-header-cell {
  flex: 1;
  min-width: 0;
}

.skeleton-table-cell {
  flex: 1;
  min-width: 0;
}

.skeleton-footer-cell {
  margin: 0 auto;
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .skeleton-table-header {
    background-color: var(--color-table-header-bg-dark, #374151);
  }

  .skeleton-table-row {
    border-bottom-color: var(--color-border-dark, #4b5563);
  }

  .skeleton-table-footer {
    background-color: var(--color-table-footer-bg-dark, #374151);
  }
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .skeleton-table-header,
  .skeleton-table-row,
  .skeleton-table-footer {
    padding: 0.25rem;
  }
}
</style>
