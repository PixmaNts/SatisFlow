<template>
  <div class="form-number">
    <input
      :id="id"
      v-model="inputValue"
      type="number"
      :placeholder="placeholder"
      :min="min"
      :max="max"
      :step="step"
      :disabled="disabled"
      :required="required"
      class="number-input"
      :class="{ 'has-error': error }"
      @blur="onBlur"
      @focus="onFocus"
    />
    <div v-if="error" class="error-message">{{ error }}</div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  /** Input ID */
  id?: string;
  /** Input value */
  modelValue: number | null;
  /** Placeholder text */
  placeholder?: string;
  /** Minimum value */
  min?: number;
  /** Maximum value */
  max?: number;
  /** Step increment */
  step?: number;
  /** Whether input is disabled */
  disabled?: boolean;
  /** Whether input is required */
  required?: boolean;
  /** Error message */
  error?: string;
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: '',
  disabled: false,
  required: false,
});

const emit = defineEmits<{
  'update:modelValue': [value: number | null];
  blur: [event: FocusEvent];
  focus: [event: FocusEvent];
}>();

const inputValue = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value === null || value === undefined ? null : Number(value)),
});

const onBlur = (event: FocusEvent) => {
  emit('blur', event);
};

const onFocus = (event: FocusEvent) => {
  emit('focus', event);
};
</script>

<style scoped>
.form-number {
  @apply space-y-1;
}

.number-input {
  @apply w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:text-white;
}

.number-input.has-error {
  @apply border-red-300 dark:border-red-600 focus:ring-red-500 focus:border-red-500;
}

.error-message {
  @apply text-sm text-red-600 dark:text-red-400 mt-1;
}
</style>
