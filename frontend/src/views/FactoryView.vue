<template>
  <div class="factory-view">
    <!-- Factory Selector -->
    <FactorySelector />

    <!-- Factory Content (only show if a factory is selected) -->
    <div v-if="currentFactory" class="factory-content">
      <!-- Factory Overview -->
      <div class="factory-overview">
        <div class="overview-header">
          <div class="factory-name-row">
            <h2 class="factory-name">{{ currentFactory.name }}</h2>
            <Button
              variant="secondary"
              size="sm"
              @click="handleEditFactory"
              title="Edit this factory"
              class="edit-factory-btn"
            >
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
                <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
              </svg>
            </Button>
            <Button
              variant="danger"
              size="sm"
              @click="showDeleteDialog = true"
              title="Delete this factory"
              class="delete-factory-btn"
            >
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <polyline points="3 6 5 6 21 6" />
                <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
                <line x1="10" y1="11" x2="10" y2="17" />
                <line x1="14" y1="11" x2="14" y2="17" />
              </svg>
            </Button>
          </div>
          <div class="factory-stats">
            <div class="stat-item">
              <span class="stat-label">Power:</span>
              <span class="stat-value" :class="powerBalanceClass">
                {{ formatPower(currentFactory.power_balance) }}
              </span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Items:</span>
              <span class="stat-value">{{ currentFactory.items.length }}</span>
            </div>
          </div>
        </div>

        <p v-if="currentFactory.description" class="factory-description">
          {{ currentFactory.description }}
        </p>
      </div>

      <!-- Factory Tabs -->
      <Tabs
        v-model:active-tab="activeTab"
        :tabs="tabs"
        class="factory-tabs"
      >
        <template #actions>
          <!-- Production Lines Actions -->
          <template v-if="activeTab === 'production'">
            <Button
              variant="secondary"
              size="sm"
              @click="handleImportBlueprint"
              title="Import a blueprint from JSON file"
            >
              <span class="button-icon">📥</span>
              Import Blueprint
            </Button>
            <Button
              variant="primary"
              size="sm"
              @click="handleAddProductionLine"
            >
              Add Production Line
            </Button>
          </template>

          <!-- Raw Inputs Actions -->
          <template v-if="activeTab === 'raw-inputs'">
            <Button
              variant="primary"
              size="sm"
              @click="handleAddRawInput"
            >
              Add Raw Input
            </Button>
          </template>

          <!-- Power Generation Actions -->
          <template v-if="activeTab === 'power-generation'">
            <Button
              variant="primary"
              size="sm"
              @click="handleAddPowerGenerator"
            >
              Add Power Generator
            </Button>
          </template>
        </template>

        <!-- Production Lines Tab -->
        <TabPanel tab-id="production">
          <ProductionLineList ref="productionLineListRef" :factory-id="currentFactory.id" />
        </TabPanel>

        <!-- Raw Inputs Tab -->
        <TabPanel tab-id="raw-inputs">
          <RawInputList ref="rawInputListRef" :factory-id="currentFactory.id" />
        </TabPanel>

        <!-- Power Generation Tab -->
        <TabPanel tab-id="power-generation">
          <PowerGeneratorList ref="powerGeneratorListRef" :factory-id="currentFactory.id" />
        </TabPanel>
      </Tabs>
    </div>

    <!-- Empty State (no factory selected) -->
    <div v-else class="empty-state">
      <div class="empty-icon">
        <svg
          width="64"
          height="64"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M19 3H5C3.89543 3 3 3.89543 3 5V19C3 20.1046 3.89543 21 5 21H19C20.1046 21 21 20.1046 21 19V5C21 3.89543 20.1046 3 19 3Z"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <path
            d="M8 12H16M12 8V16"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </div>
      <h3 class="empty-title">No Factory Selected</h3>
      <p class="empty-description">
        Select a factory from the dropdown above or create a new factory to get started.
      </p>
    </div>

    <!-- Delete Factory Confirmation Dialog -->
    <ConfirmDialog
      v-if="currentFactory"
      :show="showDeleteDialog"
      title="Delete Factory"
      :message="`Are you sure you want to delete '${currentFactory.name}'? This action cannot be undone.`"
      confirm-text="Delete"
      cancel-text="Cancel"
      variant="danger"
      :loading="isDeleting"
      @confirm="handleDeleteFactory"
      @cancel="showDeleteDialog = false"
    >
      <div class="delete-impact">
        <h4 class="impact-title">This will also delete:</h4>
        <ul class="impact-list">
          <li v-if="currentFactory.production_lines?.length">
            {{ currentFactory.production_lines.length }} production line(s)
          </li>
          <li v-if="currentFactory.raw_inputs?.length">
            {{ currentFactory.raw_inputs.length }} raw input(s)
          </li>
          <li v-if="currentFactory.power_generators?.length">
            {{ currentFactory.power_generators.length }} power generator(s)
          </li>
          <li v-if="currentFactory.items?.length">
            {{ currentFactory.items.length }} item type(s)
          </li>
        </ul>
      </div>
    </ConfirmDialog>

    <!-- Edit Factory Modal -->
    <FactoryEditModal
      v-if="currentFactory"
      :show="showEditModal"
      :factory="currentFactory"
      @update:show="showEditModal = $event"
      @saved="factoryStore.fetchById(currentFactory.id)"
    />

  </div>
</template>

<script setup lang="ts">
import { computed, watch, ref } from 'vue'
import { useFactoryStore } from '@/stores/factory'
import { usePreferencesStore } from '@/stores/preferences'
import FactorySelector from '@/components/factory/FactorySelector.vue'
import ProductionLineList from '@/components/factory/ProductionLineList.vue'
import RawInputList from '@/components/factory/RawInputList.vue'
import PowerGeneratorList from '@/components/factory/PowerGeneratorList.vue'
import Tabs from '@/components/ui/Tabs.vue'
import TabPanel from '@/components/ui/TabPanel.vue'
import Button from '@/components/ui/Button.vue'
import ConfirmDialog from '@/components/ui/ConfirmDialog.vue'
import FactoryEditModal from '@/components/factory/FactoryEditModal.vue'
import { useToast } from '@/composables/useToast'

const factoryStore = useFactoryStore()
const preferencesStore = usePreferencesStore()
const toast = useToast()

// Edit state
const showEditModal = ref(false)

// Delete state
const showDeleteDialog = ref(false)
const isDeleting = ref(false)

// Component refs
const productionLineListRef = ref<InstanceType<typeof ProductionLineList> | null>(null)
const rawInputListRef = ref<InstanceType<typeof RawInputList> | null>(null)
const powerGeneratorListRef = ref<InstanceType<typeof PowerGeneratorList> | null>(null)

// State
const activeTab = computed({
  get: () => preferencesStore.factoryViewTab,
  set: (value) => preferencesStore.setFactoryViewTab(value)
})

// Tab configuration
const tabs = [
  { id: 'production', label: 'Production Lines' },
  { id: 'raw-inputs', label: 'Raw Inputs' },
  { id: 'power-generation', label: 'Power Generation' }
]

// Computed
const currentFactory = computed(() => factoryStore.currentFactory)

const powerBalanceClass = computed(() => {
  if (!currentFactory.value) return ''

  const balance = currentFactory.value.power_balance
  if (balance > 0) return 'power-surplus'
  if (balance < 0) return 'power-deficit'
  return 'power-balanced'
})


// Methods
const handleEditFactory = () => {
  showEditModal.value = true
}

const formatPower = (power: number): string => {
  if (power < 0) {
    return `-${formatPower(Math.abs(power))}`
  }
  if (power < 1) {
    return `${(power * 1000).toFixed(0)} kW`
  }
  return `${power.toFixed(1)} MW`
}

const handleImportBlueprint = () => {
  if (productionLineListRef.value) {
    productionLineListRef.value.handleImportButtonClick()
  }
}

const handleAddProductionLine = () => {
  if (productionLineListRef.value) {
    productionLineListRef.value.openCreateModal()
  }
}

const handleAddRawInput = () => {
  if (rawInputListRef.value) {
    rawInputListRef.value.openCreateModal()
  }
}

const handleAddPowerGenerator = () => {
  if (powerGeneratorListRef.value) {
    powerGeneratorListRef.value.openCreateModal()
  }
}

const handleDeleteFactory = async () => {
  if (!currentFactory.value) return

  isDeleting.value = true
  try {
    const success = await factoryStore.deleteFactory(currentFactory.value.id)
    if (success) {
      toast.showSuccess(`Factory '${currentFactory.value.name}' deleted successfully`)
      showDeleteDialog.value = false
    } else {
      toast.showError('Failed to delete factory')
    }
  } catch (err) {
    toast.showError('An unexpected error occurred while deleting the factory')
  } finally {
    isDeleting.value = false
  }
}

// Watch for factory changes to refresh data
watch(() => currentFactory.value?.id, (factoryId) => {
  if (factoryId) {
    factoryStore.fetchById(factoryId)
    // Save selected factory to preferences
    preferencesStore.setSelectedFactoryId(factoryId)
  }
}, { immediate: true })

// Initialize selected factory from preferences
if (preferencesStore.selectedFactoryId && !currentFactory.value) {
  factoryStore.fetchById(preferencesStore.selectedFactoryId)
}

</script>

<style scoped lang="scss">
.factory-view {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.factory-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.factory-overview {
  background-color: var(--color-surface);
  border-radius: var(--border-radius-md);
  box-shadow: var(--shadow-inset);
  border: 1px solid var(--color-border);
  padding: var(--spacing-xl);
}

.overview-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-md);
  flex-wrap: wrap;
  gap: var(--spacing-md);
}

.factory-name {
  font-size: var(--font-size-2xl);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
  margin: 0;
  font-family: var(--font-family-sans);
  letter-spacing: -0.01em;
}

.factory-name-row {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.edit-factory-btn,
.delete-factory-btn {
  flex-shrink: 0;
  transition: all var(--transition-normal);
  
  &:hover {
    transform: translateY(-1px);
  }
}

.factory-stats {
  display: flex;
  gap: var(--spacing-xl);
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--color-surface-inset);
  border-radius: var(--border-radius-sm);
  border: 1px solid var(--color-border-dark);
  min-width: 100px;
}

.stat-label {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-weight: var(--font-weight-medium);
}

.stat-value {
  font-size: var(--font-size-xl);
  font-weight: var(--font-weight-bold);
  font-family: var(--font-family-mono);

  &.power-surplus {
    color: var(--color-success);
    text-shadow: 0 0 8px rgba(34, 197, 94, 0.3);
  }

  &.power-deficit {
    color: var(--color-error);
    text-shadow: 0 0 8px rgba(239, 68, 68, 0.3);
  }

  &.power-balanced {
    color: var(--color-info-blue);
    text-shadow: 0 0 8px rgba(74, 144, 164, 0.3);
  }
}

.factory-description {
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.6;
  font-size: var(--font-size-base);
}

.factory-tabs {
  background-color: var(--color-surface);
  border-radius: var(--border-radius-md);
  box-shadow: var(--shadow-inset);
  border: 1px solid var(--color-border);
  overflow: hidden;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-3xl);
  text-align: center;
  background-color: var(--color-surface);
  border-radius: var(--border-radius-md);
  box-shadow: var(--shadow-inset);
  border: 1px solid var(--color-border);
  min-height: 400px;
}

.empty-icon {
  color: var(--color-text-muted);
  margin-bottom: var(--spacing-lg);
  opacity: 0.5;
  transition: opacity var(--transition-normal);
  
  &:hover {
    opacity: 0.7;
  }
}

.empty-title {
  font-size: var(--font-size-2xl);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  margin: 0 0 var(--spacing-md) 0;
  font-family: var(--font-family-sans);
}

.empty-description {
  color: var(--color-text-secondary);
  margin: 0;
  max-width: 500px;
  font-size: var(--font-size-base);
  line-height: 1.6;
}

.button-icon {
  margin-right: var(--spacing-xs);
}

.delete-impact {
  background-color: var(--color-surface-inset);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-md);
  padding: var(--spacing-lg);
}

.impact-title {
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-semibold);
  color: var(--color-error);
  margin: 0 0 var(--spacing-md) 0;
}

.impact-list {
  margin: 0;
  padding-left: var(--spacing-lg);
  color: var(--color-text-secondary);
  font-size: var(--font-size-sm);

  li {
    margin-bottom: var(--spacing-xs);
    line-height: 1.5;
  }
}

@media (max-width: 768px) {
  .button-icon {
    display: none;
  }
  
  .factory-overview {
    padding: var(--spacing-lg);
  }
  
  .factory-name {
    font-size: var(--font-size-xl);
  }
}

// Responsive design
@media (max-width: 768px) {
  .overview-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .factory-stats {
    width: 100%;
    justify-content: space-around;
    flex-wrap: wrap;
    gap: var(--spacing-sm);
  }

  .stat-item {
    min-width: 80px;
    flex: 1;
  }
}
</style>
