<template>
  <div class="range-slider" :class="{ 'range-slider--compact': compact }">
    <div v-if="!compact" class="slider-header">
      <label v-if="label" :for="inputId" class="slider-label">
        {{ label }}
        <span v-if="required" class="required-mark">*</span>
      </label>
      <div class="value-display">
        <input
          :id="`${inputId}-input`"
          v-model.number="inputValue"
          type="number"
          :min="min"
          :max="max"
          :step="step"
          class="value-input"
          :class="{ 'value-input--error': error }"
          @blur="handleInputBlur"
          @keydown.enter="handleInputBlur"
        />
        <span class="value-unit">{{ unit }}</span>
      </div>
    </div>

    <div v-if="compact" class="slider-compact-row">
      <div class="slider-container slider-container--compact">
        <input
          :id="inputId"
          v-model.number="sliderValue"
          type="range"
          :min="min"
          :max="max"
          :step="step"
          :disabled="disabled"
          class="slider-track slider-track--compact"
          :style="sliderStyle"
          @input="handleSliderChange"
        />
      </div>
      <div class="value-display value-display--compact">
        <input
          :id="`${inputId}-input`"
          v-model.number="inputValue"
          type="number"
          :min="min"
          :max="max"
          :step="step"
          class="value-input value-input--compact"
          :class="{ 'value-input--error': error }"
          @blur="handleInputBlur"
          @keydown.enter="handleInputBlur"
        />
        <span class="value-unit value-unit--compact">{{ unit }}</span>
      </div>
    </div>

    <div v-if="!compact" class="slider-container">
      <input
        :id="inputId"
        v-model.number="sliderValue"
        type="range"
        :min="min"
        :max="max"
        :step="step"
        :disabled="disabled"
        class="slider-track"
        :style="sliderStyle"
        @input="handleSliderChange"
      />
      <div class="slider-markers">
        <button
          v-for="preset in presets"
          :key="preset.value"
          type="button"
          class="marker-button"
          :class="{ 'marker-button--active': Math.abs(modelValue - preset.value) < 0.01 }"
          :style="{ left: `${((preset.value - min) / (max - min)) * 100}%` }"
          @click="setPreset(preset.value)"
          :title="preset.label"
        >
          <span class="marker-dot"></span>
          <span class="marker-label">{{ preset.label }}</span>
        </button>
      </div>
    </div>

    <div v-if="showQuickPresets && !compact" class="quick-presets">
      <button
        v-for="preset in presets"
        :key="preset.value"
        type="button"
        class="preset-button"
        :class="{ 'preset-button--active': Math.abs(modelValue - preset.value) < 0.01 }"
        @click="setPreset(preset.value)"
      >
        {{ preset.label }}
      </button>
    </div>

    <p v-if="error" class="error-message">{{ error }}</p>
    <p v-if="hint && !compact" class="hint-text">{{ hint }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';

interface Preset {
  value: number;
  label: string;
}

interface Props {
  modelValue: number;
  label?: string;
  min?: number;
  max?: number;
  step?: number;
  unit?: string;
  presets?: Preset[];
  showQuickPresets?: boolean;
  disabled?: boolean;
  required?: boolean;
  error?: string;
  hint?: string;
  id?: string;
  compact?: boolean;
}

interface Emits {
  (e: 'update:modelValue', value: number): void;
}

const props = withDefaults(defineProps<Props>(), {
  min: 0,
  max: 250,
  step: 1,
  unit: '%',
  presets: () => [
    { value: 50, label: '50%' },
    { value: 100, label: '100%' },
    { value: 150, label: '150%' },
    { value: 200, label: '200%' },
    { value: 250, label: '250%' },
  ],
  showQuickPresets: true,
  disabled: false,
  required: false,
  compact: false,
});

const emit = defineEmits<Emits>();

// State
const inputValue = ref(props.modelValue);
const sliderValue = ref(props.modelValue);

// Computed
const inputId = computed(() => props.id || `range-slider-${Math.random().toString(36).substr(2, 9)}`);

const sliderStyle = computed(() => {
  const percentage = ((props.modelValue - props.min) / (props.max - props.min)) * 100;
  return {
    '--slider-percentage': `${percentage}%`,
  };
});

// Methods
const handleSliderChange = () => {
  const value = clampValue(sliderValue.value);
  inputValue.value = value;
  emit('update:modelValue', value);
};

const handleInputBlur = () => {
  const value = clampValue(inputValue.value);
  sliderValue.value = value;
  inputValue.value = value;
  emit('update:modelValue', value);
};

const setPreset = (value: number) => {
  if (props.disabled) return;

  inputValue.value = value;
  sliderValue.value = value;
  emit('update:modelValue', value);
};

const clampValue = (value: number): number => {
  return Math.max(props.min, Math.min(props.max, value));
};

// Watchers
watch(() => props.modelValue, (newValue) => {
  inputValue.value = newValue;
  sliderValue.value = newValue;
});
</script>

<style scoped>
.range-slider {
  width: 100%;
}

.range-slider:not(.range-slider--compact) {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.range-slider--compact {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.slider-compact-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;
  min-height: calc(0.625rem * 2 + 0.875rem * 1.5);
}

.slider-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.slider-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text-secondary, #b8b8b8);
}

.required-mark {
  color: var(--color-error, #ef4444);
  margin-left: 0.25rem;
}

.value-display {
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.value-input {
  width: 5rem;
  padding: 0.5rem 0.625rem;
  font-size: 0.875rem;
  text-align: right;
  background-color: var(--color-surface-inset, #1f1f1f);
  border: 1px solid var(--color-border, #404040);
  border-radius: 6px;
  color: var(--color-text-primary, #e5e5e5);
  outline: none;
  transition: border-color 0.2s, box-shadow 0.2s;
  box-sizing: border-box;
}

.value-input--compact {
  width: 6.5rem;
  padding: 0.625rem 0.875rem;
  font-size: 0.875rem;
  line-height: 1.5;
}

.value-display--compact {
  flex-shrink: 0;
}

.value-unit--compact {
  font-size: 0.875rem;
}

.value-input:hover:not(:disabled) {
  border-color: var(--color-border-light, #4a4a4a);
}

.value-input:focus {
  border-color: var(--color-ficsit-orange, #f58b00);
  box-shadow: 0 0 0 3px rgba(245, 139, 0, 0.1);
}

.value-input--error {
  border-color: var(--color-error, #ef4444);
}

.value-input--error:focus {
  border-color: var(--color-error, #ef4444);
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
}

.value-unit {
  font-size: 0.875rem;
  color: var(--color-text-secondary, #b8b8b8);
  font-weight: 500;
  min-width: 2ch;
}

.slider-container {
  position: relative;
  padding-top: 0.5rem;
  padding-bottom: 2rem;
}

.slider-container--compact {
  flex: 1;
  min-width: 0;
  padding: 0;
  margin: 0;
}

.slider-track {
  width: 100%;
  height: 0.5rem;
  appearance: none;
  cursor: pointer;
  border-radius: 9999px;
  background: linear-gradient(
    to right,
    rgb(59, 130, 246) 0%,
    rgb(59, 130, 246) var(--slider-percentage),
    rgb(55, 65, 81) var(--slider-percentage),
    rgb(55, 65, 81) 100%
  );
  outline: none;
  transition: background 0.2s;
}

.slider-track--compact {
  height: 0.375rem;
}

.slider-track:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Webkit browsers (Chrome, Safari) */
.slider-track::-webkit-slider-thumb {
  appearance: none;
  width: 1.25rem;
  height: 1.25rem;
  border-radius: 50%;
  background: rgb(59, 130, 246);
  border: 2px solid rgb(255, 255, 255);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  cursor: pointer;
  transition: transform 0.15s;
}

.slider-track::-webkit-slider-thumb:hover {
  transform: scale(1.1);
}

.slider-track:active::-webkit-slider-thumb {
  transform: scale(1.25);
}

.slider-track--compact::-webkit-slider-thumb {
  width: 1rem;
  height: 1rem;
  border-width: 1.5px;
}

.slider-track--compact::-webkit-slider-thumb:hover {
  transform: scale(1.15);
}

/* Firefox */
.slider-track::-moz-range-thumb {
  width: 1.25rem;
  height: 1.25rem;
  border-radius: 50%;
  background: rgb(59, 130, 246);
  border: 2px solid rgb(255, 255, 255);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  cursor: pointer;
  transition: transform 0.15s;
}

.slider-track::-moz-range-thumb:hover {
  transform: scale(1.1);
}

.slider-track:active::-moz-range-thumb {
  transform: scale(1.25);
}

.slider-track--compact::-moz-range-thumb {
  width: 1rem;
  height: 1rem;
  border-width: 1.5px;
}

.slider-track--compact::-moz-range-thumb:hover {
  transform: scale(1.15);
}

.slider-markers {
  position: absolute;
  left: 0;
  right: 0;
  top: 0;
  height: 0.5rem;
  pointer-events: none;
}

.marker-button {
  position: absolute;
  transform: translateX(-50%);
  pointer-events: auto;
  top: -2px;
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
}

.marker-dot {
  display: block;
  width: 0.5rem;
  height: 0.5rem;
  border-radius: 50%;
  background: rgb(156, 163, 175);
  border: 1px solid rgb(255, 255, 255);
  transition: all 0.2s;
}

.marker-button:hover .marker-dot {
  background: rgb(75, 85, 99);
  transform: scale(1.25);
}

.marker-button--active .marker-dot {
  background: rgb(59, 130, 246);
  transform: scale(1.5);
}

.marker-label {
  position: absolute;
  top: 1.5rem;
  left: 50%;
  transform: translateX(-50%);
  font-size: 0.75rem;
  color: var(--color-text-muted, #8a8a8a);
  white-space: nowrap;
  opacity: 0;
  transition: opacity 0.2s;
}

.marker-button:hover .marker-label,
.marker-button--active .marker-label {
  opacity: 1;
}

.quick-presets {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.preset-button {
  padding: 0.375rem 0.75rem;
  font-size: 0.875rem;
  font-weight: 500;
  background-color: var(--color-surface-inset, #1f1f1f);
  border: 1px solid var(--color-border, #404040);
  border-radius: 6px;
  color: var(--color-text-primary, #e5e5e5);
  cursor: pointer;
  transition: all 0.2s;
}

.preset-button:hover {
  background-color: var(--color-surface-hover, #2a2a2a);
  border-color: var(--color-border-light, #4a4a4a);
}

.preset-button--active {
  background-color: rgba(59, 130, 246, 0.1);
  border-color: rgb(59, 130, 246);
  color: rgb(147, 197, 253);
}

.error-message {
  font-size: 0.875rem;
  color: var(--color-error, #ef4444);
  margin-top: 0.25rem;
}

.hint-text {
  font-size: 0.75rem;
  color: var(--color-text-muted, #8a8a8a);
  margin-top: 0.25rem;
}
</style>
