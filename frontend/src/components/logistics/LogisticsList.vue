<template>
  <div class="logistics-list">
    <!-- Filters -->
    <div class="filters-section">
      <div class="filter-row">
        <div class="filter-field">
          <label for="transport-filter">Transport Type</label>
          <select
            id="transport-filter"
            v-model="filters.transportType"
            class="filter-select"
            @change="applyFilters"
          >
            <option value="">All Types</option>
            <option value="Bus">Bus</option>
            <option value="Train">Train</option>
            <option value="Truck">Truck</option>
            <option value="Drone">Drone</option>
          </select>
        </div>

        <div class="filter-field">
          <label for="from-factory-filter">From Factory</label>
          <select
            id="from-factory-filter"
            v-model="filters.fromFactory"
            class="filter-select"
            @change="applyFilters"
          >
            <option value="">All Factories</option>
            <option
              v-for="factory in factories"
              :key="`from-${factory.id}`"
              :value="factory.id"
            >
              {{ factory.name }}
            </option>
          </select>
        </div>

        <div class="filter-field">
          <label for="to-factory-filter">To Factory</label>
          <select
            id="to-factory-filter"
            v-model="filters.toFactory"
            class="filter-select"
            @change="applyFilters"
          >
            <option value="">All Factories</option>
            <option
              v-for="factory in factories"
              :key="`to-${factory.id}`"
              :value="factory.id"
            >
              {{ factory.name }}
            </option>
          </select>
        </div>

        <div class="filter-field">
          <label for="item-filter">Item</label>
          <select
            id="item-filter"
            v-model="filters.item"
            class="filter-select"
            @change="applyFilters"
          >
            <option value="">All Items</option>
            <option
              v-for="item in uniqueItems"
              :key="item"
              :value="item"
            >
              {{ formatItemName(item) }}
            </option>
          </select>
        </div>

        <div class="filter-field">
          <label for="search-filter">Search</label>
          <div class="search-input-wrapper">
            <input
              id="search-filter"
              v-model="filters.search"
              type="text"
              class="search-input"
              placeholder="Search logistics..."
              @input="applyFilters"
            />
            <svg
              class="search-icon"
              width="16"
              height="16"
              viewBox="0 0 16 16"
              fill="currentColor"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                fill-rule="evenodd"
                d="M10.442 10.442a1 1 0 011.415 0l3.85 3.85a1 1 0 01-1.414 1.415l-3.85-3.85a1 1 0 010-1.415z"
                clip-rule="evenodd"
              />
              <path
                fill-rule="evenodd"
                d="M6.5 2a4.5 4.5 0 100 9 4.5 4.5 0 000-9zM1 6.5a5.5 5.5 0 1111 0 5.5 5.5 0 01-11 0z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
        </div>
      </div>

      <div class="filter-actions">
        <button
          type="button"
          class="clear-filters-button"
          @click="clearFilters"
          :disabled="!hasActiveFilters"
        >
          Clear Filters
        </button>
      </div>
    </div>

    <!-- Logistics Lines -->
    <div class="logistics-content">
      <!-- Empty State -->
      <div v-if="filteredLogistics.length === 0 && !loading" class="empty-state">
        <svg
          class="empty-icon"
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
        <h3 class="empty-title">No logistics lines found</h3>
        <p class="empty-description">
          {{ hasActiveFilters ? 'Try adjusting your filters' : 'Create your first logistics line to get started' }}
        </p>
        <button
          v-if="!hasActiveFilters"
          type="button"
          class="create-button"
          @click="$emit('create-logistics')"
        >
          Create Logistics Line
        </button>
      </div>

      <!-- Grouped Logistics Lines -->
      <div v-else class="logistics-groups">
        <div
          v-for="(group, transportType) in groupedLogistics"
          :key="transportType"
          class="logistics-group"
        >
          <div class="group-header">
            <div class="group-icon">
              <component :is="getTransportIcon(transportType)" />
            </div>
            <h3 class="group-title">{{ transportType }} ({{ group.length }})</h3>
          </div>

          <div class="group-content">
            <div
              v-for="logistics in group"
              :key="logistics.id"
              class="logistics-item"
              @click="$emit('select-logistics', logistics)"
            >
              <div class="logistics-header">
                <div class="logistics-route">
                  <span class="factory-name">{{ getFactoryName(logistics.from_factory) }}</span>
                  <svg
                    class="route-arrow"
                    width="16"
                    height="16"
                    viewBox="0 0 16 16"
                    fill="currentColor"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      d="M1 8h14M8 1v14"
                      stroke="currentColor"
                      stroke-width="2"
                      stroke-linecap="round"
                    />
                  </svg>
                  <span class="factory-name">{{ getFactoryName(logistics.to_factory) }}</span>
                </div>
                <div class="logistics-actions">
                  <button
                    type="button"
                    class="action-button edit-button"
                    @click.stop="$emit('edit-logistics', logistics)"
                    title="Edit"
                  >
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                      <path d="M12.146.146a.5.5 0 01.708 0l3 3a.5.5 0 010 .708l-10 10a.5.5 0 01-.168.11l-5 2a.5.5 0 01-.65-.65l2-5a.5.5 0 01.11-.168l10-10zM11.207 2.5L13.5 4.793 14.793 3.5 12.5 1.207 11.207 2.5zm1.586 3L10.5 3.207 4 9.707V10a.5.5 0 00-.5.5v.007a.5.5 0 00.05.022L12.5 8.5a.5.5 0 00.5-.5v-.007a.5.5 0 00-.05-.022L12.793 5.5z"/>
                    </svg>
                  </button>
                  <button
                    type="button"
                    class="action-button delete-button"
                    @click.stop="handleDeleteLogistics(logistics)"
                    title="Delete"
                  >
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                      <path d="M5.5 5.5A.5.5 0 016 6v6a.5.5 0 01-1 0V6a.5.5 0 01.5-.5zm2.5 0a.5.5 0 01.5.5v6a.5.5 0 01-1 0V6a.5.5 0 01.5-.5zm3 .5a.5.5 0 00-1 0v6a.5.5 0 001 0V6z"/>
                      <path d="M14.5 3a1 1 0 01-1 1H13v9a2 2 0 01-2 2H5a2 2 0 01-2-2V4h-.5a1 1 0 01-1-1V2a1 1 0 011-1H6a1 1 0 011-1h2a1 1 0 011 1h14a1 1 0 011 1v1zM4.118 4L4 4.059V13a1 1 0 001 1h6a1 1 0 001-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
                    </svg>
                  </button>
                </div>
              </div>

              <div class="logistics-details">
                <div class="logistics-items">
                  <div
                    v-for="item in logistics.items"
                    :key="item.item"
                    class="item-flow"
                  >
                    <span class="item-name">{{ formatItemName(item.item) }}</span>
                    <span class="item-quantity">{{ item.quantity_per_min }}/min</span>
                  </div>
                </div>

                <div class="logistics-meta">
                  <div class="meta-item">
                    <span class="meta-label">ID:</span>
                    <span class="meta-value">{{ logistics.transport_id }}</span>
                  </div>
                  <div class="meta-item">
                    <span class="meta-label">Total:</span>
                    <span class="meta-value">{{ logistics.total_quantity_per_min }}/min</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Loading State -->
      <div v-if="loading" class="loading-state">
        <LoadingSpinner size="lg" />
        <p>Loading logistics lines...</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useFactoryStore } from '@/stores/factory'
import { useLogisticsStore } from '@/stores/logistics'
import LoadingSpinner from '@/components/ui/LoadingSpinner.vue'
import type { LogisticsResponse, Item } from '@/api/types'

interface Props {
  refreshTrigger?: number
}

interface Emits {
  'select-logistics': [logistics: LogisticsResponse]
  'edit-logistics': [logistics: LogisticsResponse]
  'delete-logistics': [logistics: LogisticsResponse]
  'create-logistics': []
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const factoryStore = useFactoryStore()
const logisticsStore = useLogisticsStore()

// State
const loading = ref(false)

// Filters
const filters = ref({
  transportType: '',
  fromFactory: '',
  toFactory: '',
  item: '',
  search: ''
})

// Computed properties
const factories = computed(() => factoryStore.factories)
const logistics = computed(() => logisticsStore.logistics)

const uniqueItems = computed(() => {
  const items = new Set<Item>()
  logistics.value.forEach(line => {
    line.items.forEach(itemFlow => {
      items.add(itemFlow.item)
    })
  })
  return Array.from(items).sort()
})

const hasActiveFilters = computed(() => {
  return filters.value.transportType !== '' ||
         filters.value.fromFactory !== '' ||
         filters.value.toFactory !== '' ||
         filters.value.item !== '' ||
         filters.value.search !== ''
})

const filteredLogistics = computed(() => {
  let filtered = [...logistics.value]

  // Transport type filter
  if (filters.value.transportType) {
    filtered = filtered.filter(line => line.transport_type === filters.value.transportType)
  }

  // From factory filter
  if (filters.value.fromFactory) {
    filtered = filtered.filter(line => line.from_factory === filters.value.fromFactory)
  }

  // To factory filter
  if (filters.value.toFactory) {
    filtered = filtered.filter(line => line.to_factory === filters.value.toFactory)
  }

  // Item filter
  if (filters.value.item) {
    filtered = filtered.filter(line =>
      line.items.some(itemFlow => itemFlow.item === filters.value.item)
    )
  }

  // Search filter
  if (filters.value.search) {
    const search = filters.value.search.toLowerCase()
    filtered = filtered.filter(line => {
      return (
        line.transport_id.toLowerCase().includes(search) ||
        line.transport_name?.toLowerCase().includes(search) ||
        getFactoryName(line.from_factory).toLowerCase().includes(search) ||
        getFactoryName(line.to_factory).toLowerCase().includes(search) ||
        line.items.some(itemFlow =>
          itemFlow.item.toLowerCase().includes(search)
        )
      )
    })
  }

  return filtered
})

const groupedLogistics = computed(() => {
  const groups: Record<string, LogisticsResponse[]> = {
    'Bus': [],
    'Train': [],
    'Truck': [],
    'Drone': []
  }

  filteredLogistics.value.forEach(line => {
    (groups[line.transport_type] as LogisticsResponse[]).push(line)
  })

  return groups
})

// Methods
const formatItemName = (item: Item): string => {
  return item.replace(/([A-Z])/g, ' $1').trim()
}

const getFactoryName = (factoryId: string): string => {
  const factory = factories.value.find(f => f.id === factoryId)
  return factory ? factory.name : `Factory ${factoryId}`
}

const getTransportIcon = (transportType: string) => {
  // Return appropriate icon component based on transport type
  switch (transportType) {
    case 'Bus':
      return 'BusIcon'
    case 'Train':
      return 'TrainIcon'
    case 'Truck':
      return 'TruckIcon'
    case 'Drone':
      return 'DroneIcon'
    default:
      return 'DefaultIcon'
  }
}

const applyFilters = () => {
  // Filters are reactive, so this just triggers the computed properties
}

const clearFilters = () => {
  filters.value = {
    transportType: '',
    fromFactory: '',
    toFactory: '',
    item: '',
    search: ''
  }
}

const handleDeleteLogistics = async (logistics: LogisticsResponse) => {
  if (confirm(`Are you sure you want to delete this logistics line?`)) {
    await logisticsStore.deleteLogistics(logistics.id)
    emit('delete-logistics', logistics)
  }
}

const fetchLogistics = async () => {
  loading.value = true
  try {
    await logisticsStore.fetchAll()
    await factoryStore.fetchAll()
  } catch (error) {
    console.error('Failed to fetch logistics:', error)
  } finally {
    loading.value = false
  }
}

// Lifecycle
onMounted(() => {
  fetchLogistics()
})

// Watch for refresh trigger
watch(() => props.refreshTrigger, () => {
  fetchLogistics()
})
</script>

<style scoped lang="scss">
.logistics-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg, 1rem);
}

.filters-section {
  background-color: var(--color-white, #ffffff);
  border: 1px solid var(--color-gray-200, #e5e7eb);
  border-radius: var(--border-radius-md, 0.375rem);
  padding: var(--spacing-md, 0.75rem);
}

.filter-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: var(--spacing-md, 0.75rem);
  margin-bottom: var(--spacing-md, 0.75rem);
}

.filter-field {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);

  label {
    font-size: var(--font-size-sm, 0.875rem);
    font-weight: var(--font-weight-medium, 500);
    color: var(--color-gray-700, #374151);
  }
}

.filter-select {
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-sm, 0.25rem);
  font-size: var(--font-size-sm, 0.875rem);
  background-color: var(--color-white, #ffffff);
  transition: border-color 0.2s ease-in-out;

  &:focus {
    outline: none;
    border-color: var(--color-primary-500, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }
}

.search-input-wrapper {
  position: relative;
}

.search-input {
  width: 100%;
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem) var(--spacing-sm, 0.5rem) 2.5rem;
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-sm, 0.25rem);
  font-size: var(--font-size-sm, 0.875rem);
  background-color: var(--color-white, #ffffff);
  transition: border-color 0.2s ease-in-out;

  &:focus {
    outline: none;
    border-color: var(--color-primary-500, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }
}

.search-icon {
  position: absolute;
  left: var(--spacing-sm, 0.5rem);
  top: 50%;
  transform: translateY(-50%);
  color: var(--color-gray-400, #9ca3af);
  pointer-events: none;
}

.filter-actions {
  display: flex;
  justify-content: flex-end;
}

.clear-filters-button {
  padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
  border: 1px solid var(--color-gray-300, #d1d5db);
  background-color: var(--color-white, #ffffff);
  color: var(--color-gray-700, #374151);
  border-radius: var(--border-radius-sm, 0.25rem);
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

.logistics-content {
  min-height: 200px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl, 1.25rem);
  text-align: center;
  color: var(--color-gray-500, #6b7280);
}

.empty-icon {
  width: 4rem;
  height: 4rem;
  margin-bottom: var(--spacing-md, 0.75rem);
  opacity: 0.5;
}

.empty-title {
  font-size: var(--font-size-lg, 1.125rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-gray-900, #111827);
  margin: 0 0 var(--spacing-sm, 0.5rem) 0;
}

.empty-description {
  font-size: var(--font-size-base, 1rem);
  margin: 0 0 var(--spacing-lg, 1rem) 0;
}

.create-button {
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  background-color: var(--color-primary-600, #2563eb);
  color: var(--color-white, #ffffff);
  border: none;
  border-radius: var(--border-radius-sm, 0.25rem);
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  cursor: pointer;
  transition: background-color 0.2s ease-in-out;

  &:hover {
    background-color: var(--color-primary-700, #1d4ed8);
  }
}

.logistics-groups {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg, 1rem);
}

.logistics-group {
  background-color: var(--color-white, #ffffff);
  border: 1px solid var(--color-gray-200, #e5e7eb);
  border-radius: var(--border-radius-md, 0.375rem);
  overflow: hidden;
}

.group-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 0.5rem);
  padding: var(--spacing-md, 0.75rem);
  background-color: var(--color-gray-50, #f9fafb);
  border-bottom: 1px solid var(--color-gray-200, #e5e7eb);
}

.group-icon {
  width: 1.5rem;
  height: 1.5rem;
  color: var(--color-gray-600, #4b5563);
}

.group-title {
  font-size: var(--font-size-base, 1rem);
  font-weight: var(--font-weight-semibold, 600);
  color: var(--color-gray-900, #111827);
  margin: 0;
}

.group-content {
  padding: var(--spacing-sm, 0.5rem);
}

.logistics-item {
  padding: var(--spacing-md, 0.75rem);
  border-radius: var(--border-radius-sm, 0.25rem);
  border: 1px solid var(--color-gray-100, #f3f4f6);
  cursor: pointer;
  transition: all 0.2s ease-in-out;

  &:hover {
    background-color: var(--color-gray-50, #f9fafb);
    border-color: var(--color-gray-200, #e5e7eb);
  }

  &:not(:last-child) {
    margin-bottom: var(--spacing-sm, 0.5rem);
  }
}

.logistics-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-sm, 0.5rem);
}

.logistics-route {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 0.5rem);
}

.factory-name {
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-900, #111827);
}

.route-arrow {
  color: var(--color-gray-400, #9ca3af);
}

.logistics-actions {
  display: flex;
  gap: var(--spacing-xs, 0.25rem);
}

.action-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 1.75rem;
  height: 1.75rem;
  padding: 0;
  border: none;
  border-radius: var(--border-radius-sm, 0.25rem);
  cursor: pointer;
  transition: all 0.2s ease-in-out;

  &.edit-button {
    background-color: var(--color-blue-100, #dbeafe);
    color: var(--color-blue-600, #2563eb);

    &:hover {
      background-color: var(--color-blue-200, #bfdbfe);
    }
  }

  &.delete-button {
    background-color: var(--color-red-100, #fee2e2);
    color: var(--color-red-600, #dc2626);

    &:hover {
      background-color: var(--color-red-200, #fecaca);
    }
  }
}

.logistics-details {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm, 0.5rem);
}

.logistics-items {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-sm, 0.5rem);
}

.item-flow {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
  padding: var(--spacing-xs, 0.25rem) var(--spacing-sm, 0.5rem);
  background-color: var(--color-gray-100, #f3f4f6);
  border-radius: var(--border-radius-sm, 0.25rem);
}

.item-name {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-700, #374151);
}

.item-quantity {
  font-size: var(--font-size-xs, 0.75rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-900, #111827);
}

.logistics-meta {
  display: flex;
  gap: var(--spacing-md, 0.75rem);
}

.meta-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
}

.meta-label {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-500, #6b7280);
}

.meta-value {
  font-size: var(--font-size-xs, 0.75rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-700, #374151);
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl, 1.25rem);
  gap: var(--spacing-md, 0.75rem);
  color: var(--color-gray-500, #6b7280);
}

// Responsive design
@media (max-width: 768px) {
  .filter-row {
    grid-template-columns: 1fr;
  }

  .logistics-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-sm, 0.5rem);
  }

  .logistics-route {
    flex-wrap: wrap;
  }

  .logistics-meta {
    flex-direction: column;
    gap: var(--spacing-xs, 0.25rem);
  }
}
</style>

<script lang="ts">
// Component name fix for ESLint multi-word requirement
export default {
  name: 'LogisticsList'
}
</script>
