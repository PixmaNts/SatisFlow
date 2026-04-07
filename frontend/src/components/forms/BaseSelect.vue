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

<style scoped lang="scss">
.base-select {
  margin-bottom: var(--spacing-md, 0.75rem);
}

.base-select__label {
  display: block;
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-secondary, #b8b8b8);
  margin-bottom: var(--spacing-xs, 0.25rem);
}

.base-select__required {
  color: var(--color-error, #ef4444);
  margin-left: var(--spacing-xs, 0.25rem);
}

.base-select__wrapper {
  position: relative;
}

.base-select__field {
  width: 100%;
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  padding-right: 2.5rem;
  border: 1px solid var(--color-border, #404040);
  border-radius: var(--border-radius-md, 0.375rem);
  box-shadow: var(--shadow-sm);
  appearance: none;
  background: var(--color-surface, #252525);
  color: var(--color-text-primary, #e5e5e5);
  font-size: var(--font-size-sm, 0.875rem);
  transition: border-color var(--transition-fast, 150ms), box-shadow var(--transition-fast, 150ms);
  cursor: pointer;

  &:focus {
    outline: none;
    border-color: var(--color-info-blue, #4a90a4);
    box-shadow: 0 0 0 2px rgba(74, 144, 164, 0.2);
  }

  &--invalid {
    border-color: var(--color-error, #ef4444);

    &:focus {
      border-color: var(--color-error, #ef4444);
      box-shadow: 0 0 0 2px rgba(239, 68, 68, 0.2);
    }
  }

  &--disabled {
    background: var(--color-surface-inset, #1f1f1f);
    color: var(--color-text-muted, #8a8a8a);
    cursor: not-allowed;
  }

  &--placeholder {
    color: var(--color-text-muted, #8a8a8a);
  }
}

.base-select__arrow {
  position: absolute;
  top: 0;
  bottom: 0;
  right: 0;
  display: flex;
  align-items: center;
  padding-right: var(--spacing-sm, 0.5rem);
  pointer-events: none;
}

.base-select__error {
  margin-top: var(--spacing-xs, 0.25rem);
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-error, #ef4444);
}

.base-select__hint {
  margin-top: var(--spacing-xs, 0.25rem);
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-secondary, #b8b8b8);
}

.base-select--has-error .base-select__label {
  color: var(--color-error, #ef4444);
}

.base-select--disabled .base-select__label {
  color: var(--color-text-muted, #8a8a8a);
}
</style>
