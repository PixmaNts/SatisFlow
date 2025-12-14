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
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  width: 100%;
}

.number-input {
  width: 100%;
  box-sizing: border-box;
  padding: 0.625rem 0.875rem;
  background-color: var(--color-surface-inset, #1f1f1f);
  border: 1px solid var(--color-border, #404040);
  border-radius: 6px;
  color: var(--color-text-primary, #e5e5e5);
  font-size: 0.875rem;
  line-height: 1.5;
  transition: border-color 0.2s, box-shadow 0.2s;
  outline: none;
}

.number-input::placeholder {
  color: var(--color-text-muted, #8a8a8a);
}

.number-input:hover:not(:disabled) {
  border-color: var(--color-border-light, #4a4a4a);
}

.number-input:focus {
  border-color: var(--color-ficsit-orange, #f58b00);
  box-shadow: 0 0 0 3px rgba(245, 139, 0, 0.1);
}

.number-input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.number-input.has-error {
  border-color: var(--color-error, #ef4444);
}

.number-input.has-error:focus {
  border-color: var(--color-error, #ef4444);
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
}

.error-message {
  font-size: 0.75rem;
  color: var(--color-error, #ef4444);
}
</style>
