<template>
  <div class="data-table-container">
    <!-- Table controls -->
    <div v-if="filterable || $slots.controls" class="table-controls">
      <div v-if="filterable" class="filter-container">
        <div class="filter-input-wrapper">
          <svg
            class="filter-icon"
            width="16"
            height="16"
            viewBox="0 0 16 16"
            fill="currentColor"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              fill-rule="evenodd"
              d="M7.14 8.584a1 1 0 01-1.414 0l-3-3a1 1 0 011.414-1.414L6.5 6.67l2.36-2.36a1 1 0 111.414 1.414l-3 3z"
              clip-rule="evenodd"
            />
          </svg>
          <input
            v-model="filterQuery"
            type="text"
            :placeholder="filterPlaceholder"
            class="filter-input"
            @input="handleFilter"
          />
        </div>
      </div>

      <div class="controls-extra">
        <slot name="controls" />
      </div>
    </div>

    <!-- Table wrapper -->
    <div class="table-wrapper">
      <table class="data-table">
        <!-- Table header -->
        <thead>
          <tr>
            <th
              v-for="column in columns"
              :key="column.key"
              :class="headerClasses(column)"
              :style="column.width ? { width: column.width } : {}"
              @click="handleSort(column)"
            >
              <span class="header-content">
                {{ column.label }}
                <span
                  v-if="sortable && column.sortable !== false && sortState.key === column.key"
                  class="sort-indicator"
                >
                  <svg
                    v-if="sortState.direction === 'asc'"
                    width="12"
                    height="12"
                    viewBox="0 0 12 12"
                    fill="currentColor"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      fill-rule="evenodd"
                      d="M3.293 9.707a1 1 0 010-1.414l2.586-2.586a1 1 0 011.414 0l2.586 2.586a1 1 0 01-1.414 1.414L6 7.414l-2.293 2.293a1 1 0 01-1.414 0z"
                      clip-rule="evenodd"
                    />
                  </svg>
                  <svg
                    v-else
                    width="12"
                    height="12"
                    viewBox="0 0 12 12"
                    fill="currentColor"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      fill-rule="evenodd"
                      d="M8.707 2.293a1 1 0 010 1.414L6.121 6.293a1 1 0 01-1.414 0L2.121 3.707a1 1 0 011.414-1.414L5.414 4.586 8.707 2.293z"
                      clip-rule="evenodd"
                    />
                  </svg>
                </span>
              </span>
            </th>
          </tr>
        </thead>

        <!-- Table body -->
        <tbody>
          <tr
            v-for="(row, index) in paginatedData"
            :key="getRowKey(row, index)"
            :class="rowClasses(row, index)"
            @click="handleRowClick(row, index)"
          >
            <td
              v-for="column in columns"
              :key="column.key"
              :class="cellClasses(column)"
            >
              <slot
                :name="`cell-${column.key}`"
                :row="row"
                :value="getCellValue(row, column)"
                :column="column"
              >
                {{ formatCellValue(getCellValue(row, column), column) }}
              </slot>
            </td>
          </tr>
        </tbody>
      </table>

      <!-- Loading overlay -->
      <div v-if="loading" class="loading-overlay">
        <LoadingSpinner size="lg" />
        <span class="loading-text">Loading...</span>
      </div>

      <!-- Empty state -->
      <div v-else-if="paginatedData.length === 0" class="empty-state">
        <div class="empty-icon">
          <svg
            width="48"
            height="48"
            viewBox="0 0 48 48"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M24 14C18.477 14 14 18.477 14 24C14 29.523 18.477 34 24 34C29.523 34 34 29.523 34 24C34 18.477 29.523 14 24 14Z"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <path
              d="M24 20V28M20 24H28"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
        </div>
        <p class="empty-text">{{ emptyText }}</p>
      </div>
    </div>

    <!-- Pagination -->
    <div v-if="paginated && totalPages > 1" class="pagination">
      <button
        type="button"
        class="pagination-button"
        :disabled="currentPage === 1"
        @click="goToPage(currentPage - 1)"
      >
        Previous
      </button>

      <div class="pagination-info">
        Page {{ currentPage }} of {{ totalPages }}
      </div>

      <button
        type="button"
        class="pagination-button"
        :disabled="currentPage === totalPages"
        @click="goToPage(currentPage + 1)"
      >
        Next
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import LoadingSpinner from './LoadingSpinner.vue'

/**
 * Column definition interface
 */
export interface Column {
  /** Unique key for the column */
  key: string
  /** Display label for the column */
  label: string
  /** Whether the column can be sorted */
  sortable?: boolean
  /** Fixed width for the column */
  width?: string
  /** Custom formatter function */
  formatter?: (value: unknown) => string
  /** CSS class for cells in this column */
  cellClass?: string | ((row: Record<string, unknown>) => string)
}

/**
 * DataTable component props
 */
interface Props {
  /** Array of column definitions */
  columns: Column[]
  /** Array of data rows */
  data: Record<string, unknown>[]
  /** Whether the table is sortable */
  sortable?: boolean
  /** Whether the table is filterable */
  filterable?: boolean
  /** Whether the table is loading */
  loading?: boolean
  /** Placeholder text for filter input */
  filterPlaceholder?: string
  /** Text to display when no data */
  emptyText?: string
  /** Whether to enable pagination */
  paginated?: boolean
  /** Number of rows per page */
  pageSize?: number
  /** Function to get unique row key */
  rowKey?: (row: Record<string, unknown>, index: number) => string | number
  /** CSS class for rows */
  rowClass?: string | ((row: Record<string, unknown>, index: number) => string)
}

const props = withDefaults(defineProps<Props>(), {
  sortable: true,
  filterable: false,
  loading: false,
  filterPlaceholder: 'Filter...',
  emptyText: 'No data available',
  paginated: false,
  pageSize: 10,
  rowKey: (row, index) => index,
})

/** Emit table events */
const emit = defineEmits<{
  'row-click': [row: Record<string, unknown>, index: number]
  'sort-change': [{ key: string, direction: 'asc' | 'desc' }]
}>()

// Internal state
const filterQuery = ref('')
const currentPage = ref(1)
const sortState = ref<{ key: string; direction: 'asc' | 'desc' }>({
  key: '',
  direction: 'asc',
})

/**
 * Compute filtered data
 */
const filteredData = computed(() => {
  if (!filterQuery.value.trim()) {
    return props.data
  }

  const query = filterQuery.value.toLowerCase()
  return props.data.filter(row => {
    return props.columns.some(column => {
      const value = getCellValue(row, column)
      return String(value).toLowerCase().includes(query)
    })
  })
})

/**
 * Compute sorted data
 */
const sortedData = computed(() => {
  if (!sortState.value.key) {
    return filteredData.value
  }

  const { key, direction } = sortState.value
  const column = props.columns.find(col => col.key === key)

  return [...filteredData.value].sort((a, b) => {
    const aValue = getCellValue(a, column!)
    const bValue = getCellValue(b, column!)

    let comparison = 0
    const aStr = String(aValue || '')
    const bStr = String(bValue || '')

    if (aStr < bStr) comparison = -1
    if (aStr > bStr) comparison = 1

    return direction === 'desc' ? -comparison : comparison
  })
})

/**
 * Compute paginated data
 */
const paginatedData = computed(() => {
  if (!props.paginated) {
    return sortedData.value
  }

  const startIndex = (currentPage.value - 1) * props.pageSize
  const endIndex = startIndex + props.pageSize
  return sortedData.value.slice(startIndex, endIndex)
})

/**
 * Compute total pages
 */
const totalPages = computed(() => {
  return Math.ceil(sortedData.value.length / props.pageSize)
})

/**
 * Get cell value for a row and column
 */
const getCellValue = (row: Record<string, unknown>, column: Column) => {
  return row[column.key]
}

/**
 * Format cell value
 */
const formatCellValue = (value: unknown, column: Column) => {
  if (column.formatter) {
    return column.formatter(value)
  }
  return value
}

/**
 * Get unique row key
 */
const getRowKey = (row: Record<string, unknown>, index: number) => {
  return props.rowKey(row, index)
}

/**
 * Compute CSS classes for header
 */
const headerClasses = (column: Column) => [
  'table-header',
  {
    'table-header-sortable': props.sortable && column.sortable !== false,
    'table-header-sorted': sortState.value.key === column.key,
  },
]

/**
 * Compute CSS classes for row
 */
const rowClasses = (row: Record<string, unknown>, index: number) => {
  const classes = ['table-row']

  if (props.rowClass) {
    if (typeof props.rowClass === 'function') {
      classes.push(props.rowClass(row, index))
    } else {
      classes.push(props.rowClass)
    }
  }

  return classes
}

/**
 * Compute CSS classes for cell
 */
const cellClasses = (column: Column) => {
  const classes = ['table-cell']

  if (column.cellClass) {
    if (typeof column.cellClass === 'function') {
      // We can't determine the class without the row data here
      // This will be handled in the template
    } else {
      classes.push(column.cellClass)
    }
  }

  return classes
}

/**
 * Handle sort click
 */
const handleSort = (column: Column) => {
  if (!props.sortable || column.sortable === false) {
    return
  }

  let direction: 'asc' | 'desc' = 'asc'

  if (sortState.value.key === column.key) {
    direction = sortState.value.direction === 'asc' ? 'desc' : 'asc'
  }

  sortState.value = { key: column.key, direction }
  emit('sort-change', { key: column.key, direction })
}

/**
 * Handle row click
 */
const handleRowClick = (row: Record<string, unknown>, index: number) => {
  emit('row-click', row, index)
}

/**
 * Handle filter input
 */
const handleFilter = () => {
  currentPage.value = 1 // Reset to first page when filtering
}

/**
 * Go to specific page
 */
const goToPage = (page: number) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
  }
}

// Reset pagination when data changes
watch(() => props.data, () => {
  currentPage.value = 1
})
</script>

<style scoped lang="scss">
.data-table-container {
  width: 100%;
  background-color: var(--color-white, #ffffff);
  border-radius: var(--border-radius-lg, 0.5rem);
  box-shadow: var(--shadow-sm, 0 1px 2px 0 rgba(0, 0, 0, 0.05));
  overflow: hidden;
}

.table-controls {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md, 0.75rem) var(--spacing-lg, 1rem);
  border-bottom: 1px solid var(--color-gray-200, #e5e7eb);
  gap: var(--spacing-md, 0.75rem);
}

.filter-container {
  flex: 1;
  max-width: 20rem;
}

.filter-input-wrapper {
  position: relative;
}

.filter-icon {
  position: absolute;
  left: var(--spacing-sm, 0.5rem);
  top: 50%;
  transform: translateY(-50%);
  color: var(--color-gray-400, #9ca3af);
  pointer-events: none;
}

.filter-input {
  width: 100%;
  padding: var(--spacing-sm, 0.5rem) var(--spacing-sm, 0.5rem) var(--spacing-sm, 0.5rem) 2rem;
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-md, 0.375rem);
  font-size: var(--font-size-sm, 0.875rem);
  transition: border-color 0.2s ease-in-out;

  &:focus {
    outline: none;
    border-color: var(--color-primary-500, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }
}

.controls-extra {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 0.5rem);
}

.table-wrapper {
  position: relative;
  overflow-x: auto;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
  font-size: var(--font-size-sm, 0.875rem);
}

.table-header {
  background-color: var(--color-gray-50, #f9fafb);
  padding: var(--spacing-md, 0.75rem) var(--spacing-lg, 1rem);
  text-align: left;
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-gray-900, #111827);
  border-bottom: 1px solid var(--color-gray-200, #e5e7eb);
  white-space: nowrap;

  &.table-header-sortable {
    cursor: pointer;
    user-select: none;
    transition: background-color 0.2s ease-in-out;

    &:hover {
      background-color: var(--color-gray-100, #f3f4f6);
    }
  }

  &.table-header-sorted {
    color: var(--color-primary-600, #2563eb);
  }
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-xs, 0.25rem);
}

.sort-indicator {
  display: flex;
  align-items: center;
  color: currentColor;
}

.table-row {
  border-bottom: 1px solid var(--color-gray-100, #f3f4f6);
  transition: background-color 0.2s ease-in-out;

  &:hover {
    background-color: var(--color-gray-50, #f9fafb);
  }

  &:last-child {
    border-bottom: none;
  }
}

.table-cell {
  padding: var(--spacing-md, 0.75rem) var(--spacing-lg, 1rem);
  color: var(--color-gray-700, #374151);
  vertical-align: middle;
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background-color: rgba(255, 255, 255, 0.8);
  gap: var(--spacing-md, 0.75rem);
}

.loading-text {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-gray-600, #4b5563);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl, 1.25rem) var(--spacing-lg, 1rem);
  gap: var(--spacing-md, 0.75rem);
  color: var(--color-gray-500, #6b7280);
}

.empty-icon {
  opacity: 0.5;
}

.empty-text {
  font-size: var(--font-size-base, 1rem);
  margin: 0;
}

.pagination {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md, 0.75rem) var(--spacing-lg, 1rem);
  border-top: 1px solid var(--color-gray-200, #e5e7eb);
}

.pagination-button {
  padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-md, 0.375rem);
  background-color: var(--color-white, #ffffff);
  color: var(--color-gray-700, #374151);
  font-size: var(--font-size-sm, 0.875rem);
  cursor: pointer;
  transition: all 0.2s ease-in-out;

  &:hover:not(:disabled) {
    background-color: var(--color-gray-50, #f9fafb);
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.pagination-info {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-gray-600, #4b5563);
}

// Responsive design
@media (max-width: 768px) {
  .table-controls {
    flex-direction: column;
    align-items: stretch;
    gap: var(--spacing-sm, 0.5rem);
  }

  .filter-container {
    max-width: none;
  }

  .table-header,
  .table-cell {
    padding: var(--spacing-sm, 0.5rem);
    font-size: var(--font-size-xs, 0.75rem);
  }

  .pagination {
    flex-direction: column;
    gap: var(--spacing-sm, 0.5rem);
  }
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'UiDataTable'
}
</script>
