<template>
  <ValidatedForm
    :field-configs="fieldConfigs"
    :initial-data="initialData"
    :validation-options="validationOptions"
    :validation-context="validationContext"
    submit-text="Create Production Line"
    @submit="handleSubmit"
    @validation-change="onValidationChange"
    @field-change="onFieldChange"
  >
    <template #default="{ formData, errors, setFieldValue, handleFieldBlur }">
      <div class="space-y-6">
        <!-- Basic Information -->
        <div class="bg-gray-50 p-4 rounded-lg">
          <h3 class="text-lg font-medium text-gray-900 mb-4">Basic Information</h3>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <BaseInput
              :model-value="formData.name as string"
              @update:model-value="setFieldValue('name', $event)"
              label="Production Line Name"
              placeholder="Enter production line name"
              :error="errors.name"
              :required="true"
              hint="A descriptive name for this production line"
              @validate="setFieldValue('name', $event)"
              @blur="handleFieldBlur('name', ($event.target as HTMLInputElement).value)"
            />

            <div class="form-group">
              <label for="recipe-autocomplete" class="form-label">
                Recipe
                <span class="text-red-500">*</span>
              </label>
              <RecipeAutocomplete
                id="recipe-autocomplete"
                :model-value="formData.recipe as string"
                :recipes="availableRecipes"
                placeholder="Start typing to search recipes..."
                clearable
                @update:model-value="setFieldValue('recipe', $event)"
                @selected="onRecipeSelected"
                @focus="handleFieldBlur('recipe', formData.recipe as string)"
              />
              <p v-if="errors.recipe" class="text-red-500 text-sm mt-1">
                {{ errors.recipe?.[0] }}
              </p>
              <p class="text-gray-500 text-sm mt-1">The recipe to produce</p>
            </div>
          </div>

          <BaseInput
            :model-value="formData.description as string"
            @update:model-value="setFieldValue('description', $event)"
            label="Description"
            placeholder="Optional description"
            :error="errors.description"
            hint="Brief description of this production line"
            @validate="setFieldValue('description', $event)"
            @blur="handleFieldBlur('description', ($event.target as HTMLInputElement).value)"
          />
        </div>

        <!-- Machine Groups -->
        <div class="bg-gray-50 p-4 rounded-lg">
          <h3 class="text-lg font-medium text-gray-900 mb-4">Machine Groups</h3>

          <div v-for="(group, index) in machineGroups" :key="index" class="mb-6 p-4 border border-gray-200 rounded-lg">
            <div class="flex justify-between items-center mb-4">
              <h4 class="text-md font-medium text-gray-800">Machine Group {{ index + 1 }}</h4>
              <button
                v-if="machineGroups.length > 1"
                type="button"
                @click="removeMachineGroup(index)"
                class="text-red-600 hover:text-red-800 text-sm"
              >
                Remove Group
              </button>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <BaseInput
                :model-value="group.machineCount"
                @update:model-value="updateMachineGroup(index, 'machineCount', $event)"
                label="Number of Machines"
                type="number"
                placeholder="1"
                :error="errors[`machineGroups.${index}.machineCount`]"
                :required="true"
                hint="How many machines in this group"
                @validate="setFieldValue(`machineGroups.${index}.machineCount`, $event)"
                @blur="handleFieldBlur(`machineGroups.${index}.machineCount`, ($event.target as HTMLInputElement).value)"
              />

              <BaseInput
                :model-value="group.overclock"
                @update:model-value="updateMachineGroup(index, 'overclock', $event)"
                label="Overclock (%)"
                type="number"
                step="0.001"
                placeholder="100.000"
                :error="errors[`machineGroups.${index}.overclock`]"
                hint="Overclock percentage (0.000-250.000)"
                @validate="setFieldValue(`machineGroups.${index}.overclock`, $event)"
                @blur="handleFieldBlur(`machineGroups.${index}.overclock`, ($event.target as HTMLInputElement).value)"
              />

              <BaseInput
                :model-value="group.somersloops"
                @update:model-value="updateMachineGroup(index, 'somersloops', $event)"
                label="Somersloops"
                type="number"
                placeholder="0"
                :error="errors[`machineGroups.${index}.somersloops`]"
                hint="Number of somersloops per machine"
                @validate="setFieldValue(`machineGroups.${index}.somersloops`, $event)"
                @blur="handleFieldBlur(`machineGroups.${index}.somersloops`, ($event.target as HTMLInputElement).value)"
              />
            </div>
          </div>

          <button
            type="button"
            @click="addMachineGroup"
            class="w-full py-2 px-4 border border-gray-300 rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          >
            + Add Machine Group
          </button>
        </div>
      </div>
    </template>
  </ValidatedForm>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import ValidatedForm from './ValidatedForm.vue';
import BaseInput from './BaseInput.vue';
import RecipeAutocomplete from '../factory/RecipeAutocomplete.vue';
import {
  required,
  maxLength,
  validateMachineCount,
  validateOverclock,
  validateSomersloop,
  custom,
  number,
  integer,
  min
} from '@/composables/useValidation';
import type { FieldValidationConfig, ValidationContext } from '@/types/validation';
import type { RecipeInfo } from '@/api/types';

interface MachineGroup {
  machineCount: number | null;
  overclock: number | null;
  somersloops: number | null;
}

interface Props {
  initialData?: {
    name?: string;
    description?: string;
    recipe?: string;
    machineGroups?: MachineGroup[];
  };
  availableRecipes?: RecipeInfo[];
  machineTypes?: Record<string, { maxSomersloops: number }>;
}

interface Emits {
  (e: 'submit', data: {
    name: string;
    description?: string;
    recipe: string;
    machineGroups: MachineGroup[];
  }): void;
  (e: 'validation-change', isValid: boolean, errors: Record<string, string[]>): void;
  (e: 'field-change', field: string, value: unknown): void;
}

const props = withDefaults(defineProps<Props>(), {
  initialData: () => ({}),
  availableRecipes: () => [],
  machineTypes: () => ({})
});

const emit = defineEmits<Emits>();

// Machine groups state
const machineGroups = ref<MachineGroup[]>(
  props.initialData.machineGroups || [
    { machineCount: null, overclock: null, somersloops: null }
  ]
);

// Recipe selected handler
const onRecipeSelected = (recipe: RecipeInfo) => {
  // Optionally update other fields based on selected recipe
  // For example, you could auto-populate machine type based on recipe
  emit('field-change', 'recipe', recipe.name);
};

// Get machine type for current recipe
const currentMachineType = computed(() => {
  // This would normally come from the recipe data
  // For now, we'll use a default
  return 'Assembler'; // Default machine type
});

// Field validation configuration
const fieldConfigs: FieldValidationConfig = {
  name: [
    required('Production line name is required'),
    maxLength(100, 'Name must be less than 100 characters')
  ],
  recipe: [
    required('Recipe is required')
  ],
  description: [
    maxLength(500, 'Description must be less than 500 characters')
  ]
};

// Add validation for each machine group
machineGroups.value.forEach((_, index) => {
  fieldConfigs[`machineGroups.${index}.machineCount`] = [
    required('Machine count is required'),
    number(),
    integer(),
    min(1, 'Must have at least 1 machine'),
    custom(validateMachineCount())
  ];

  fieldConfigs[`machineGroups.${index}.overclock`] = [
    number(),
    custom(validateOverclock())
  ];

  fieldConfigs[`machineGroups.${index}.somersloops`] = [
    number(),
    integer(),
    min(0, 'Cannot have negative somersloops'),
    custom(validateSomersloop({
      machineType: currentMachineType.value,
      maxSomersloops: Object.fromEntries(
        Object.entries(props.machineTypes).map(([key, value]) => [key, value.maxSomersloops])
      )
    }))
  ];
});

// Validation options
const validationOptions = {
  validateOnBlur: true,
  validateOnChange: false,
  validateOnSubmit: true,
  stopOnFirstError: false
};

// Validation context
const validationContext: ValidationContext = {
  formData: {},
  machineTypes: props.machineTypes
};

// Machine group management
const addMachineGroup = () => {
  machineGroups.value.push({
    machineCount: null,
    overclock: null,
    somersloops: null
  });

  // Add validation for the new group
  const index = machineGroups.value.length - 1;
  fieldConfigs[`machineGroups.${index}.machineCount`] = [
    required('Machine count is required'),
    number(),
    integer(),
    min(1, 'Must have at least 1 machine'),
    custom(validateMachineCount())
  ];

  fieldConfigs[`machineGroups.${index}.overclock`] = [
    number(),
    custom(validateOverclock())
  ];

  fieldConfigs[`machineGroups.${index}.somersloops`] = [
    number(),
    integer(),
    min(0, 'Cannot have negative somersloops'),
    custom(validateSomersloop({
      machineType: currentMachineType.value,
      maxSomersloops: Object.fromEntries(
        Object.entries(props.machineTypes).map(([key, value]) => [key, value.maxSomersloops])
      )
    }))
  ];
};

const removeMachineGroup = (index: number) => {
  machineGroups.value.splice(index, 1);

  // Remove validation for the removed group
  delete fieldConfigs[`machineGroups.${index}.machineCount`];
  delete fieldConfigs[`machineGroups.${index}.overclock`];
  delete fieldConfigs[`machineGroups.${index}.somersloops`];
};

const updateMachineGroup = (index: number, field: keyof MachineGroup, value: unknown) => {
  if (machineGroups.value[index]) {
    machineGroups.value[index][field] = Number(value) || null;
  }
};

// Handle form submission
const handleSubmit = (data: Record<string, unknown>) => {
  emit('submit', {
    name: String(data.name || ''),
    description: data.description ? String(data.description) : undefined,
    recipe: String(data.recipe || ''),
    machineGroups: machineGroups.value
  });
};

// Handle validation changes
const onValidationChange = (isValid: boolean, errors: Record<string, string[]>) => {
  emit('validation-change', isValid, errors);
};

// Handle field changes
const onFieldChange = (field: string, value: unknown) => {
  emit('field-change', field, value);
};
</script>
