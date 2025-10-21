<template>
  <div class="base-select" :class="containerClasses">
    <label v-if="label" :for="selectId" class="base-select__label">
      {{ label }}
      <span v-if="required" class="base-select__required">*</span>
    </label>

    <div class="base-select__wrapper">
      <select
        :id="selectId"
        ref="selectRef"
        v-model="selectValue"
        :disabled="disabled"
        :class="selectClasses"
        @blur="handleBlur"
        @change="handleChange"
      >
        <option v-if="placeholder" value="" disabled>
          {{ placeholder }}
        </option>
        <option
          v-for="option in options"
          :key="option.value"
          :value="option.value"
          :disabled="option.disabled"
        >
          {{ option.label }}
        </option>
      </select>

      <div class="base-select__arrow">
        <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
        </svg>
      </div>

      <div v-if="showError && hasErrors" class="base-select__error">
        {{ errorMessage }}
      </div>
    </div>

    <div v-if="hint" class="base-select__hint">
      {{ hint }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, nextTick } from 'vue';
import { useId } from 'vue';

interface SelectOption {
  value: string | number;
  label: string;
  disabled?: boolean;
}

interface Props {
  modelValue: string | number | null;
  label?: string;
  options: SelectOption[];
  placeholder?: string;
  disabled?: boolean;
  required?: boolean;
  hint?: string;
  error?: string | string[];
  validateOnBlur?: boolean;
  showError?: boolean;
}

interface Emits {
  (e: 'update:modelValue', value: string | number | null): void;
  (e: 'blur', event: FocusEvent): void;
  (e: 'change', event: Event): void;
  (e: 'validate', value: string | number | null): void;
}

const props = withDefaults(defineProps<Props>(), {
  validateOnBlur: true,
  showError: true
});

const emit = defineEmits<Emits>();

const selectId = useId();
const selectRef = ref<HTMLSelectElement>();

const selectValue = computed({
  get: () => props.modelValue ?? '',
  set: (value: string) => {
    const parsedValue = value === '' ? null : value;
    emit('update:modelValue', parsedValue);
  }
});

const selectClasses = computed(() => ({
  'base-select__field': true,
  'base-select__field--invalid': hasErrors.value,
  'base-select__field--disabled': props.disabled,
  'base-select__field--placeholder': !props.modelValue
}));

const containerClasses = computed(() => ({
  'base-select--has-error': hasErrors.value,
  'base-select--disabled': props.disabled
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
    emit('validate', selectValue.value);
  }
};

const handleChange = (event: Event) => {
  emit('change', event);
};

const focus = () => {
  nextTick(() => {
    selectRef.value?.focus();
  });
};

const blur = () => {
  nextTick(() => {
    selectRef.value?.blur();
  });
};

defineExpose({
  focus,
  blur,
  selectRef
});
</script>

<style scoped>
.base-select {
  @apply mb-4;
}

.base-select__label {
  @apply block text-sm font-medium text-gray-700 mb-1;
}

.base-select__required {
  @apply text-red-500 ml-1;
}

.base-select__wrapper {
  @apply relative;
}

.base-select__field {
  @apply w-full px-3 py-2 pr-10 border border-gray-300 rounded-md shadow-sm appearance-none focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors duration-200 bg-white;
}

.base-select__field--invalid {
  @apply border-red-300 focus:ring-red-500 focus:border-red-500;
}

.base-select__field--disabled {
  @apply bg-gray-50 text-gray-500 cursor-not-allowed;
}

.base-select__field--placeholder {
  @apply text-gray-500;
}

.base-select__arrow {
  @apply absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none;
}

.base-select__error {
  @apply mt-1 text-sm text-red-600;
}

.base-select__hint {
  @apply mt-1 text-sm text-gray-500;
}

.base-select--has-error .base-select__label {
  @apply text-red-700;
}

.base-select--disabled .base-select__label {
  @apply text-gray-500;
}
</style>
