<template>
  <div class="blueprint-card" @click="$emit('click', template)">
    <!-- Centered Title -->
    <h3 class="blueprint-name">{{ template.name }}</h3>

    <!-- Machines and Power on one line -->
    <div class="stats-line">
      <div class="stat-item">
        <img src="/icons/Constructor.webp" alt="Machines" class="stat-icon" />
        <span class="stat-value">{{ template.total_machines }}</span>
      </div>
      <span class="stat-separator">|</span>
      <div class="stat-item">
        <img src="/icons/Power_Line.png" alt="Power" class="stat-icon" />
        <span class="stat-value">{{ formatNumber(template.total_power) }} MW</span>
      </div>
    </div>

    <!-- Inputs and Outputs side by side -->
    <div class="items-container">
      <!-- Inputs Column -->
      <div v-if="externalInputs.length > 0" class="items-column">
        <h4 class="items-title">Inputs</h4>
        <div class="items-list">
          <div
            v-for="[item, quantity] in externalInputs"
            :key="item"
            class="item-row input-item"
            :title="formatItemName(item)"
          >
            <ItemDisplay :item="item" size="sm" />
            <span class="item-quantity">{{ formatNumber(quantity) }}</span>
          </div>
        </div>
      </div>

      <!-- Outputs Column -->
      <div v-if="externalOutputs.length > 0" class="items-column">
        <h4 class="items-title">Outputs</h4>
        <div class="items-list">
          <div
            v-for="[item, quantity] in externalOutputs"
            :key="item"
            class="item-row output-item"
            :title="formatItemName(item)"
          >
            <ItemDisplay :item="item" size="sm" />
            <span class="item-quantity">{{ formatNumber(quantity) }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { BlueprintTemplateResponse } from '@/api/types';
import ItemDisplay from '@/components/ui/ItemDisplay.vue';
import { useItemIcon } from '@/composables/useItemIcon';

interface Props {
  /** Blueprint template data */
  template: BlueprintTemplateResponse;
}

const props = defineProps<Props>();

defineEmits<{
  click: [template: BlueprintTemplateResponse];
}>();

const { formatItemName } = useItemIcon();

// Filter out intermediate items - only show external inputs and outputs
const externalInputs = computed(() => {
  const outputItemSet = new Set(props.template.output_items.map(([item]) => item));
  return props.template.input_items.filter(([item]) => !outputItemSet.has(item));
});

const externalOutputs = computed(() => {
  const inputItemSet = new Set(props.template.input_items.map(([item]) => item));
  return props.template.output_items.filter(([item]) => !inputItemSet.has(item));
});

// Format number: remove .0 for integers
const formatNumber = (num: number): string => {
  return num % 1 === 0 ? num.toString() : num.toFixed(1);
};
</script>

<style scoped>
.blueprint-card {
  background: var(--color-surface, #252525);
  border: 2px solid var(--color-border, #404040);
  border-radius: 8px;
  padding: 1rem;
  cursor: pointer;
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  height: 100%;
  transition: all 0.2s ease;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.3);
}

.blueprint-card:hover {
  border-color: var(--color-ficsit-orange, #f58b00);
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.4), 0 2px 4px -1px rgba(0, 0, 0, 0.2);
  background: var(--color-surface-hover, #2a2a2a);
}

.blueprint-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: linear-gradient(90deg, #f58b00, transparent);
  opacity: 0;
  transition: opacity 0.2s;
}

.blueprint-card:hover::before {
  opacity: 1;
}

.blueprint-name {
  text-align: center;
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--color-text-primary, #e5e5e5);
  margin: 0 0 0.75rem 0;
  line-height: 1.3;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.stats-line {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  margin-bottom: 0.75rem;
  padding-bottom: 0.75rem;
  border-bottom: 1px solid var(--color-border, #404040);
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.stat-icon {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
  object-fit: contain;
}

.stat-value {
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--color-text-secondary, #b8b8b8);
  white-space: nowrap;
}

.stat-separator {
  color: var(--color-text-muted, #8a8a8a);
  font-size: 0.75rem;
  margin: 0 0.25rem;
}

.items-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.5rem;
  flex: 1;
}

.items-column {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.items-title {
  font-size: 0.625rem;
  font-weight: 600;
  color: var(--color-text-muted, #8a8a8a);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 0.5rem;
  text-align: center;
}

.items-list {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.item-row {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.25rem 0.375rem;
  border-radius: 4px;
  font-size: 0.75rem;
  min-width: 0;
}

.item-row :deep(.item-display) {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 0.25rem;
  overflow: hidden;
}

.item-row :deep(.item-icon) {
  flex-shrink: 0;
  width: 16px;
  height: 16px;
}

.item-row :deep(.item-name) {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
  min-width: 0;
}

.item-quantity {
  font-weight: 600;
  flex-shrink: 0;
  margin-left: auto;
}

.input-item {
  background-color: rgba(239, 68, 68, 0.15);
  color: #fca5a5;
  border: 1px solid rgba(239, 68, 68, 0.3);
}

.output-item {
  background-color: rgba(34, 197, 94, 0.15);
  color: #86efac;
  border: 1px solid rgba(34, 197, 94, 0.3);
}

.item-quantity {
  font-weight: 600;
  flex-shrink: 0;
  white-space: nowrap;
}
</style>
