<template>
  <form @submit.prevent="onFormSubmit" class="validated-form">
    <slot
      :formData="formData"
      :errors="validationState.errors"
      :hasErrors="hasErrors"
      :isValid="isValid"
      :isDirty="isDirty"
      :isValidating="isValidating"
      :getFieldErrors="getFieldErrors"
      :fieldHasErrors="fieldHasErrors"
      :isFieldValid="isFieldValid"
      :isFieldInvalid="isFieldInvalid"
      :validateField="validateField"
      :setFieldValue="setFieldValue"
      :handleFieldBlur="handleFieldBlur"
      :resetValidation="resetValidation"
    />

    <div v-if="showGlobalError && hasErrors" class="validated-form__global-error">
      Please correct the errors above before submitting.
    </div>

    <div class="validated-form__actions">
      <slot name="actions"
        :handleSubmit="handleSubmit"
        :isSubmitting="isSubmitting"
        :isValid="isValid"
        :hasErrors="hasErrors"
      >
        <button
          type="submit"
          :disabled="isSubmitting || (validateOnSubmit && hasErrors)"
          class="validated-form__submit"
        >
          <span v-if="isSubmitting">Submitting...</span>
          <span v-else>{{ submitText }}</span>
        </button>
      </slot>
    </div>
  </form>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useValidation } from '@/composables/useValidation';
import type { FieldValidationConfig, FormValidationOptions, ValidationContext } from '@/types/validation';

interface Props {
  fieldConfigs: FieldValidationConfig;
  initialData?: Record<string, unknown>;
  validationOptions?: FormValidationOptions;
  validationContext?: ValidationContext;
  submitText?: string;
  showGlobalError?: boolean;
  validateOnSubmit?: boolean;
}

interface Emits {
  (e: 'submit', formData: Record<string, unknown>): void;
  (e: 'validation-change', isValid: boolean, errors: Record<string, string[]>): void;
  (e: 'field-change', field: string, value: unknown): void;
}

const props = withDefaults(defineProps<Props>(), {
  submitText: 'Submit',
  showGlobalError: true,
  validateOnSubmit: true
});

const emit = defineEmits<Emits>();

// Form data
const formData = ref<Record<string, unknown>>({ ...props.initialData });

// Initialize validation
const {
  validationState,
  hasErrors,
  isValid,
  isDirty,
  isValidating,
  getFieldErrors,
  fieldHasErrors,
  isFieldValid,
  isFieldInvalid,
  validateField,
  validateForm,
  resetValidation,
  setFieldValue,
  handleFieldBlur,
  handleSubmit
} = useValidation(
  props.fieldConfigs,
  props.validationOptions,
  { ...props.validationContext, formData: formData.value }
);

// Submitting state
const isSubmitting = ref(false);

// Watch for validation changes and emit events
watch(
  () => validationState.isValid,
  (newIsValid) => {
    emit('validation-change', newIsValid, validationState.errors);
  },
  { immediate: true }
);

// Watch for field changes and emit events
const handleFieldChange = (field: string, value: unknown) => {
  formData.value[field] = value;
  emit('field-change', field, value);
};

// Wrap setFieldValue to also update formData
const wrappedSetFieldValue = async (field: string, value: unknown) => {
  handleFieldChange(field, value);
  await setFieldValue(field, value);
};

// Wrap handleFieldBlur to also update formData
const wrappedHandleFieldBlur = async (field: string, value: unknown) => {
  handleFieldChange(field, value);
  await handleFieldBlur(field, value);
};

// Handle form submission
const onFormSubmit = async () => {
  if (isSubmitting.value) return;

  isSubmitting.value = true;

  try {
    const success = await handleSubmit(formData.value, async () => {
      emit('submit', { ...formData.value });
    });

    if (success) {
      // Optionally reset form after successful submission
      // resetValidation();
      // formData.value = { ...props.initialData };
    }
  } finally {
    isSubmitting.value = false;
  }
};


// Expose methods for parent components
defineExpose({
  formData,
  validateForm,
  resetValidation,
  validateField,
  setFieldValue: wrappedSetFieldValue,
  handleFieldBlur: wrappedHandleFieldBlur,
  handleSubmit: onFormSubmit,
  isSubmitting,
  hasErrors,
  isValid,
  isDirty
});
</script>

<style scoped>
.validated-form {
  @apply space-y-4;
}

.validated-form__global-error {
  @apply p-3 mb-4 text-sm text-red-700 bg-red-100 rounded-md border border-red-200;
}

.validated-form__actions {
  @apply mt-6 pt-4 border-t border-gray-200;
}

.validated-form__submit {
  @apply px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200;
}

.validated-form__submit:disabled {
  @apply bg-gray-400 hover:bg-gray-400 cursor-not-allowed;
}
</style>
