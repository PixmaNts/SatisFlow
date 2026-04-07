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

<style scoped lang="scss">
.validated-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md, 0.75rem);
}

.validated-form__global-error {
  padding: var(--spacing-sm, 0.5rem);
  margin-bottom: var(--spacing-sm, 0.5rem);
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-error-dark, #dc2626);
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid var(--color-error, #ef4444);
  border-radius: var(--border-radius-md, 0.375rem);
}

.validated-form__actions {
  margin-top: var(--spacing-lg, 1rem);
  padding-top: var(--spacing-sm, 0.5rem);
  border-top: 1px solid var(--color-border, #404040);
}

.validated-form__submit {
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary, #e5e5e5);
  background: var(--color-info-blue, #4a90a4);
  border: 1px solid transparent;
  border-radius: var(--border-radius-md, 0.375rem);
  box-shadow: var(--shadow-sm);
  cursor: pointer;
  transition: background-color var(--transition-fast, 150ms);

  &:hover:not(:disabled) {
    background: var(--color-info-blue-light, #5ba3b8);
  }

  &:focus {
    outline: none;
    box-shadow: 0 0 0 2px rgba(74, 144, 164, 0.4);
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background: var(--color-text-muted, #8a8a8a);
  }
}
</style>
