<template>
  <ValidatedForm
    :field-configs="fieldConfigs"
    :initial-data="initialData"
    :validation-options="validationOptions"
    :validation-context="validationContext"
    submit-text="Create Factory"
    @submit="handleSubmit"
    @validation-change="onValidationChange"
    @field-change="onFieldChange"
  >
    <template #default="{ formData, errors, setFieldValue, handleFieldBlur }">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <BaseInput
          :model-value="formData.name as string"
          @update:model-value="setFieldValue('name', $event)"
          label="Factory Name"
          placeholder="Enter factory name"
          :error="errors.name"
          :required="true"
          hint="A unique name to identify this factory"
          @validate="setFieldValue('name', $event)"
          @blur="handleFieldBlur('name', ($event.target as HTMLInputElement).value)"
        />

        <BaseInput
          :model-value="formData.description as string"
          @update:model-value="setFieldValue('description', $event)"
          label="Description"
          placeholder="Optional description"
          :error="errors.description"
          hint="Brief description of this factory's purpose"
          @validate="setFieldValue('description', $event)"
          @blur="handleFieldBlur('description', ($event.target as HTMLInputElement).value)"
        />
      </div>

      <div class="mt-6">
        <BaseInput
          :model-value="formData.notes as string"
          @update:model-value="setFieldValue('notes', $event)"
          label="Notes"
          type="text"
          placeholder="Additional notes"
          :error="errors.notes"
          hint="Any additional information about this factory"
          @validate="setFieldValue('notes', $event)"
          @blur="handleFieldBlur('notes', ($event.target as HTMLInputElement).value)"
        />
      </div>
    </template>
  </ValidatedForm>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import ValidatedForm from './ValidatedForm.vue';
import BaseInput from './BaseInput.vue';
import { required, maxLength, validateFactoryName, custom } from '@/composables/useValidation';
import type { FieldValidationConfig, ValidationContext } from '@/types/validation';

interface Props {
  initialData?: {
    name?: string;
    description?: string;
    notes?: string;
  };
  existingFactories?: Array<{ id: string | number; name: string }>;
  isEditing?: boolean;
}

interface Emits {
  (e: 'submit', data: { name: string; description?: string; notes?: string }): void;
  (e: 'validation-change', isValid: boolean, errors: Record<string, string[]>): void;
  (e: 'field-change', field: string, value: unknown): void;
}

const props = withDefaults(defineProps<Props>(), {
  initialData: () => ({}),
  existingFactories: () => [],
  isEditing: false
});

const emit = defineEmits<Emits>();

// Get existing factory names for validation
const existingFactoryNames = computed(() =>
  props.existingFactories.map(f => f.name)
);

// Field validation configuration
const fieldConfigs: FieldValidationConfig = {
  name: [
    required('Factory name is required'),
    maxLength(100, 'Factory name must be less than 100 characters'),
    custom(validateFactoryName({
      existingNames: existingFactoryNames.value,
      currentName: props.isEditing ? props.initialData.name : undefined
    }))
  ],
  description: [
    maxLength(500, 'Description must be less than 500 characters')
  ],
  notes: [
    maxLength(1000, 'Notes must be less than 1000 characters')
  ]
};

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
};

// Handle form submission
const handleSubmit = (data: Record<string, unknown>) => {
  emit('submit', {
    name: String(data.name || ''),
    description: data.description ? String(data.description) : undefined,
    notes: data.notes ? String(data.notes) : undefined
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
