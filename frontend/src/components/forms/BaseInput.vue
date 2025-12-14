<template>
  <div class="base-input" :class="containerClasses">
    <label v-if="label" :for="inputId" class="base-input__label">
      {{ label }}
      <span v-if="required" class="base-input__required">*</span>
    </label>

    <div class="base-input__wrapper">
      <input
        :id="inputId"
        ref="inputRef"
        v-model="inputValue"
        :type="type"
        :placeholder="placeholder"
        :disabled="disabled"
        :readonly="readonly"
        :class="inputClasses"
        @blur="handleBlur"
        @input="handleInput"
        @change="handleChange"
      />

      <div v-if="showError && hasErrors" class="base-input__error">
        {{ errorMessage }}
      </div>
    </div>

    <div v-if="hint" class="base-input__hint">
      {{ hint }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, nextTick } from 'vue';
import { useId } from 'vue';

interface Props {
  modelValue: string | number | null;
  label?: string;
  type?: 'text' | 'number' | 'email' | 'password' | 'tel';
  placeholder?: string;
  disabled?: boolean;
  readonly?: boolean;
  required?: boolean;
  hint?: string;
  error?: string | string[];
  validateOnBlur?: boolean;
  validateOnChange?: boolean;
  showError?: boolean;
}

interface Emits {
  (e: 'update:modelValue', value: string | number | null): void;
  (e: 'blur', event: FocusEvent): void;
  (e: 'input', event: Event): void;
  (e: 'change', event: Event): void;
  (e: 'validate', value: string | number | null): void;
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  validateOnBlur: true,
  validateOnChange: false,
  showError: true
});

const emit = defineEmits<Emits>();

const inputId = useId();
const inputRef = ref<HTMLInputElement>();

const inputValue = computed({
  get: () => props.modelValue?.toString() ?? '',
  set: (value: string) => {
    const parsedValue = props.type === 'number'
      ? (value === '' ? null : Number(value))
      : value;

    emit('update:modelValue', parsedValue);
  }
});

const inputClasses = computed(() => ({
  'base-input__field': true,
  'base-input__field--invalid': hasErrors.value,
  'base-input__field--disabled': props.disabled,
  'base-input__field--readonly': props.readonly
}));

const containerClasses = computed(() => ({
  'base-input--has-error': hasErrors.value,
  'base-input--disabled': props.disabled,
  'base-input--readonly': props.readonly
}));

const hasErrors = computed(() => {
  if (!props.error) return false;
  return Array.isArray(props.error) ? props.error.length > 0 : !!props.error;
});

const errorMessage = computed(() => {
  if (!props.error) return '';
  return Array.isArray(props.error) ? props.error[0] : props.error;
});

const handleBlur = (event: FocusEvent) => {
  emit('blur', event);
  if (props.validateOnBlur) {
    emit('validate', inputValue.value);
  }
};

const handleInput = (event: Event) => {
  emit('input', event);
  if (props.validateOnChange) {
    emit('validate', inputValue.value);
  }
};

const handleChange = (event: Event) => {
  emit('change', event);
};

const focus = () => {
  nextTick(() => {
    inputRef.value?.focus();
  });
};

const blur = () => {
  nextTick(() => {
    inputRef.value?.blur();
  });
};

defineExpose({
  focus,
  blur,
  inputRef
});
</script>

<style scoped>
.base-input {
  margin-bottom: 0;
  width: 100%;
  box-sizing: border-box;
}

.base-input__label {
  display: block;
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--color-text-secondary, #b8b8b8);
  margin-bottom: 0.5rem;
}

.base-input__required {
  color: var(--color-error, #ef4444);
  margin-left: 0.25rem;
}

.base-input__wrapper {
  position: relative;
  width: 100%;
  box-sizing: border-box;
}

.base-input__field {
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

.base-input__field::placeholder {
  color: var(--color-text-muted, #8a8a8a);
}

.base-input__field:hover:not(:disabled):not(:readonly) {
  border-color: var(--color-border-light, #4a4a4a);
}

.base-input__field:focus {
  border-color: var(--color-ficsit-orange, #f58b00);
  box-shadow: 0 0 0 3px rgba(245, 139, 0, 0.1);
}

.base-input__field--invalid {
  border-color: var(--color-error, #ef4444);
}

.base-input__field--invalid:focus {
  border-color: var(--color-error, #ef4444);
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
}

.base-input__field--disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.base-input__field--readonly {
  cursor: default;
}

.base-input__error {
  margin-top: 0.25rem;
  font-size: 0.75rem;
  color: var(--color-error, #ef4444);
}

.base-input__hint {
  margin-top: 0.25rem;
  font-size: 0.75rem;
  color: var(--color-text-muted, #8a8a8a);
}

.base-input--has-error .base-input__label {
  color: var(--color-error, #ef4444);
}

.base-input--disabled .base-input__label {
  color: var(--color-text-disabled, #5a5a5a);
}
</style>
