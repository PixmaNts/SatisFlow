<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, h } from 'vue'
import { useFactoryStore } from '@/stores/factory'
import { useLogisticsStore } from '@/stores/logistics'
import LogisticsList from '@/components/logistics/LogisticsList.vue'
import LogisticsLineForm from '@/components/logistics/LogisticsLineForm.vue'
import Modal from '@/components/ui/Modal.vue'
import Button from '@/components/ui/Button.vue'
import FloatingActionButton, { type FabAction } from '@/components/ui/FloatingActionButton.vue'
import type { LogisticsResponse } from '@/api/types'

// Store
const factoryStore = useFactoryStore()
const logisticsStore = useLogisticsStore()

// State
const showCreateModal = ref(false)
const showEditModal = ref(false)
const selectedLogistics = ref<LogisticsResponse | null>(null)
const refreshTrigger = ref(0)

// Methods
const handleCreateLogistics = () => {
  selectedLogistics.value = null
  showCreateModal.value = true
}

const handleEditLogistics = (logistics: LogisticsResponse) => {
  selectedLogistics.value = logistics
  showEditModal.value = true
}

const handleSelectLogistics = (logistics: LogisticsResponse) => {
  selectedLogistics.value = logistics
}

const handleDeleteLogistics = (logistics: LogisticsResponse) => {
  // The list component already handles the deletion
  // Just refresh the list
  console.log(`Deleted logistics line: ${logistics.id}`)
  refreshTrigger.value++
}

const handleLogisticsCreated = (logistics: LogisticsResponse) => {
  console.log(`Created logistics line: ${logistics.id}`)
  showCreateModal.value = false
  refreshTrigger.value++
}

const handleLogisticsUpdated = (logistics: LogisticsResponse) => {
  console.log(`Updated logistics line: ${logistics.id}`)
  showEditModal.value = false
  selectedLogistics.value = null
  refreshTrigger.value++
}

// FAB actions - using h() for icon rendering
const fabActions = computed((): FabAction[] => [
  {
    id: 'create-logistics',
    label: 'Create Logistics Line',
    onClick: handleCreateLogistics,
    icon: h('svg', {
      width: '20',
      height: '20',
      viewBox: '0 0 24 24',
      fill: 'none',
      xmlns: 'http://www.w3.org/2000/svg'
    }, [
      h('path', {
        d: 'M8 7H16M8 12H16M8 17H13',
        stroke: 'currentColor',
        'stroke-width': '2',
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round'
      }),
      h('path', {
        d: 'M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z',
        stroke: 'currentColor',
        'stroke-width': '2'
      })
    ])
  }
])

const handleCloseCreateModal = () => {
  showCreateModal.value = false
}

const handleCloseEditModal = () => {
  showEditModal.value = false
  selectedLogistics.value = null
}

// Keyboard shortcuts
const handleCreateNew = () => {
  handleCreateLogistics()
}

// Lifecycle
onMounted(async () => {
  // Fetch initial data
  await Promise.all([
    factoryStore.fetchAll(),
    logisticsStore.fetchAll()
  ])

  // Register keyboard shortcuts
  document.addEventListener('app-create-new', handleCreateNew)
})

onUnmounted(() => {
  document.removeEventListener('app-create-new', handleCreateNew)
})
</script>

<template>
  <div class="logistics-view">
    <!-- Header -->
    <div class="logistics-header">
      <div class="header-content">
        <div class="header-title">
          <h1>Logistics Management</h1>
          <p class="header-subtitle">
            Configure and manage transport lines between factories
          </p>
        </div>

        <Button
          variant="primary"
          @click="handleCreateLogistics"
        >
          <svg
            width="16"
            height="16"
            viewBox="0 0 16 16"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            style="margin-right: 0.5rem;"
          >
            <path
              d="M8 2v6m0 0v6m0-6h6m-6 0H2"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
            />
          </svg>
          Create Logistics Line
        </Button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="logistics-content">
      <LogisticsList
        :refresh-trigger="refreshTrigger"
        @select-logistics="handleSelectLogistics"
        @edit-logistics="handleEditLogistics"
        @delete-logistics="handleDeleteLogistics"
        @create-logistics="handleCreateLogistics"
      />
    </div>

    <!-- Create Modal -->
    <Modal
      v-model:show="showCreateModal"
      title="Create Logistics Line"
      @close="handleCloseCreateModal"
    >
      <LogisticsLineForm
        @close="handleCloseCreateModal"
        @created="handleLogisticsCreated"
      />
    </Modal>

    <!-- Edit Modal -->
    <Modal
      v-model:show="showEditModal"
      title="Edit Logistics Line"
      @close="handleCloseEditModal"
    >
      <LogisticsLineForm
        :logistics-line="selectedLogistics"
        @close="handleCloseEditModal"
        @updated="handleLogisticsUpdated"
      />
    </Modal>

    <!-- Floating Action Button -->
    <FloatingActionButton
      :actions="fabActions"
      main-button-label="Quick actions"
      position="bottom-right"
    />
  </div>
</template>

<style scoped lang="scss">
.logistics-view {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg, 1rem);
  padding: var(--spacing-lg, 1rem);
  max-width: 1200px;
  margin: 0 auto;
  width: 100%;
}

.logistics-header {
  background-color: var(--color-surface, #252525);
  border-radius: var(--border-radius-sm, 3px);
  box-shadow: var(--shadow-inset);
  border: 1px solid var(--color-border, #404040);
  padding: var(--spacing-lg, 1rem);
  margin-bottom: var(--spacing-md, 0.75rem);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--spacing-md, 0.75rem);
}

.header-title {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
}

.header-title h1 {
  font-size: var(--font-size-2xl, 1.5rem);
  font-weight: var(--font-weight-bold, 700);
  color: var(--color-ficsit-orange, #f58b00);
  margin: 0;
  letter-spacing: -0.01em;
}

.header-subtitle {
  font-size: var(--font-size-base, 1rem);
  color: var(--color-text-secondary, #b8b8b8);
  margin: 0;
}

.logistics-content {
  flex: 1;
}

// Responsive design
@media (max-width: 768px) {
  .logistics-view {
    padding: var(--spacing-md, 0.75rem);
  }

  .header-content {
    flex-direction: column;
    align-items: stretch;
    gap: var(--spacing-md, 0.75rem);
  }

  .header-title h1 {
    font-size: var(--font-size-xl, 1.25rem);
  }
}

// Industrial theme is always dark - no separate dark theme needed
</style>
