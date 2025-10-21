<template>
  <div class="validation-demo p-6 max-w-2xl mx-auto">
    <h1 class="text-2xl font-bold text-gray-900 mb-6">Validation System Demo</h1>

    <div class="space-y-8">
      <!-- Factory Form Demo -->
      <div class="bg-white p-6 rounded-lg shadow-md">
        <h2 class="text-xl font-semibold text-gray-800 mb-4">Factory Form</h2>
        <FactoryForm
          :existing-factories="existingFactories"
          @submit="onFactorySubmit"
          @validation-change="onFactoryValidationChange"
        />
      </div>

      <!-- Production Line Form Demo -->
      <div class="bg-white p-6 rounded-lg shadow-md">
        <h2 class="text-xl font-semibold text-gray-800 mb-4">Production Line Form</h2>
        <ProductionLineForm
          :available-recipes="availableRecipes"
          :machine-types="machineTypes"
          @submit="onProductionLineSubmit"
          @validation-change="onProductionLineValidationChange"
        />
      </div>

      <!-- Validation Status -->
      <div class="bg-gray-50 p-4 rounded-lg">
        <h3 class="text-lg font-medium text-gray-900 mb-2">Validation Status</h3>
        <div class="space-y-2 text-sm">
          <div class="flex justify-between">
            <span>Factory Form Valid:</span>
            <span :class="factoryFormValid ? 'text-green-600' : 'text-red-600'">
              {{ factoryFormValid ? '✓ Valid' : '✗ Invalid' }}
            </span>
          </div>
          <div class="flex justify-between">
            <span>Production Line Form Valid:</span>
            <span :class="productionLineFormValid ? 'text-green-600' : 'text-red-600'">
              {{ productionLineFormValid ? '✓ Valid' : '✗ Invalid' }}
            </span>
          </div>
        </div>
      </div>

      <!-- Recent Submissions -->
      <div class="bg-gray-50 p-4 rounded-lg">
        <h3 class="text-lg font-medium text-gray-900 mb-2">Recent Submissions</h3>
        <div v-if="submissions.length === 0" class="text-gray-500 text-sm">
          No submissions yet
        </div>
        <div v-else class="space-y-2">
          <div
            v-for="(submission, index) in submissions"
            :key="index"
            class="bg-white p-3 rounded border border-gray-200"
          >
            <div class="font-medium text-gray-900">{{ submission.type }}</div>
            <div class="text-sm text-gray-600 mt-1">
              <pre class="whitespace-pre-wrap">{{ JSON.stringify(submission.data, null, 2) }}</pre>
            </div>
            <div class="text-xs text-gray-500 mt-2">
              {{ submission.timestamp }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import FactoryForm from './FactoryForm.vue';
import ProductionLineForm from './ProductionLineForm.vue';

// Demo data
const existingFactories = ref([
  { id: 1, name: 'Iron Processing' },
  { id: 2, name: 'Copper Processing' },
  { id: 3, name: 'Steel Production' }
]);

const availableRecipes = ref([
  { value: 'iron_ingot', label: 'Iron Ingot' },
  { value: 'copper_ingot', label: 'Copper Ingot' },
  { value: 'steel_ingot', label: 'Steel Ingot' },
  { value: 'iron_plate', label: 'Iron Plate' },
  { value: 'copper_sheet', label: 'Copper Sheet' }
]);

const machineTypes = ref({
  Constructor: { maxSommersloops: 1 },
  Smelter: { maxSommersloops: 0 },
  Assembler: { maxSommersloops: 2 },
  Manufacturer: { maxSommersloops: 4 },
  Refinery: { maxSommersloops: 0 },
  Blender: { maxSommersloops: 0 }
});

// Validation state
const factoryFormValid = ref(false);
const productionLineFormValid = ref(false);

// Submissions
const submissions = ref<Array<{
  type: string;
  data: Record<string, unknown>;
  timestamp: string;
}>>([]);

// Event handlers
const onFactorySubmit = (data: Record<string, unknown>) => {
  console.log('Factory submitted:', data);
  submissions.value.unshift({
    type: 'Factory',
    data,
    timestamp: new Date().toLocaleString()
  });

  // Keep only last 5 submissions
  if (submissions.value.length > 5) {
    submissions.value = submissions.value.slice(0, 5);
  }
};

const onFactoryValidationChange = (isValid: boolean, errors: Record<string, string[]>) => {
  factoryFormValid.value = isValid;
  console.log('Factory validation changed:', { isValid, errors });
};

const onProductionLineSubmit = (data: Record<string, unknown>) => {
  console.log('Production line submitted:', data);
  submissions.value.unshift({
    type: 'Production Line',
    data,
    timestamp: new Date().toLocaleString()
  });

  // Keep only last 5 submissions
  if (submissions.value.length > 5) {
    submissions.value = submissions.value.slice(0, 5);
  }
};

const onProductionLineValidationChange = (isValid: boolean, errors: Record<string, string[]>) => {
  productionLineFormValid.value = isValid;
  console.log('Production line validation changed:', { isValid, errors });
};
</script>

<style scoped>
.validation-demo {
  font-family: system-ui, -apple-system, sans-serif;
}

pre {
  font-family: 'Courier New', monospace;
  font-size: 0.75rem;
  line-height: 1.2;
}
</style>
