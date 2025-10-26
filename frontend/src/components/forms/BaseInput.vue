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
  @apply mb-4;
}

.base-input__label {
  @apply block text-sm font-medium text-gray-700 mb-1;
}

.base-input__required {
  @apply text-red-500 ml-1;
}

.base-input__wrapper {
  @apply relative;
}

.base-input__field {
  @apply w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200;
}

.base-input__field--invalid {
  @apply border-red-300 focus:ring-red-500 focus:border-red-500;
}

.base-input__field--disabled {
  @apply bg-gray-50 text-gray-500 cursor-not-allowed;
}

.base-input__field--readonly {
  @apply bg-gray-50 text-gray-700;
}

.base-input__error {
  @apply mt-1 text-sm text-red-600;
}

.base-input__hint {
  @apply mt-1 text-sm text-gray-500;
}

.base-input--has-error .base-input__label {
  @apply text-red-700;
}

.base-input--disabled .base-input__label {
  @apply text-gray-500;
}
</style>
