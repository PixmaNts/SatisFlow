<template>
  <div class="range-slider">
    <div class="slider-header">
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

    <div class="slider-container">
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

    <div v-if="showQuickPresets" class="quick-presets">
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
    <p v-if="hint" class="hint-text">{{ hint }}</p>
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
  @apply w-full space-y-3;
}

.slider-header {
  @apply flex items-center justify-between;
}

.slider-label {
  @apply text-sm font-medium text-gray-700 dark:text-gray-300;
}

.required-mark {
  @apply text-red-500 dark:text-red-400 ml-1;
}

.value-display {
  @apply flex items-center gap-1;
}

.value-input {
  @apply w-20 px-2 py-1 text-sm text-right
         bg-white dark:bg-gray-800
         border border-gray-300 dark:border-gray-600
         rounded text-gray-900 dark:text-gray-100
         focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500
         transition-colors;
}

.value-input--error {
  @apply border-red-500 dark:border-red-400;
}

.value-unit {
  @apply text-sm text-gray-600 dark:text-gray-400 font-medium min-w-[2ch];
}

.slider-container {
  @apply relative pt-2 pb-8;
}

.slider-track {
  @apply w-full h-2 appearance-none cursor-pointer
         bg-gray-200 dark:bg-gray-700 rounded-full
         disabled:opacity-50 disabled:cursor-not-allowed;
}

/* Webkit browsers (Chrome, Safari) */
.slider-track::-webkit-slider-thumb {
  @apply appearance-none w-5 h-5 rounded-full
         bg-blue-600 dark:bg-blue-500
         border-2 border-white dark:border-gray-800
         shadow-md cursor-pointer
         transition-transform hover:scale-110;
}

.slider-track:active::-webkit-slider-thumb {
  @apply scale-125;
}

/* Firefox */
.slider-track::-moz-range-thumb {
  @apply w-5 h-5 rounded-full
         bg-blue-600 dark:bg-blue-500
         border-2 border-white dark:border-gray-800
         shadow-md cursor-pointer
         transition-transform;
}

.slider-track::-moz-range-thumb:hover {
  @apply scale-110;
}

.slider-track:active::-moz-range-thumb {
  @apply scale-125;
}

/* Track fill effect */
.slider-track {
  background: linear-gradient(
    to right,
    rgb(37, 99, 235) 0%,
    rgb(37, 99, 235) var(--slider-percentage),
    rgb(229, 231, 235) var(--slider-percentage),
    rgb(229, 231, 235) 100%
  );
}

.dark .slider-track {
  background: linear-gradient(
    to right,
    rgb(59, 130, 246) 0%,
    rgb(59, 130, 246) var(--slider-percentage),
    rgb(55, 65, 81) var(--slider-percentage),
    rgb(55, 65, 81) 100%
  );
}

.slider-markers {
  @apply absolute left-0 right-0 top-0 h-2 pointer-events-none;
}

.marker-button {
  @apply absolute -translate-x-1/2 pointer-events-auto;
  top: -2px;
}

.marker-dot {
  @apply block w-2 h-2 rounded-full
         bg-gray-400 dark:bg-gray-500
         border border-white dark:border-gray-800
         transition-all;
}

.marker-button:hover .marker-dot {
  @apply bg-gray-600 dark:bg-gray-300 scale-125;
}

.marker-button--active .marker-dot {
  @apply bg-blue-600 dark:bg-blue-500 scale-150;
}

.marker-label {
  @apply absolute top-6 left-1/2 -translate-x-1/2
         text-xs text-gray-600 dark:text-gray-400
         whitespace-nowrap opacity-0 transition-opacity;
}

.marker-button:hover .marker-label,
.marker-button--active .marker-label {
  @apply opacity-100;
}

.quick-presets {
  @apply flex gap-2 flex-wrap;
}

.preset-button {
  @apply px-3 py-1.5 text-sm font-medium
         bg-white dark:bg-gray-800
         border border-gray-300 dark:border-gray-600
         rounded-md
         text-gray-700 dark:text-gray-300
         hover:bg-gray-50 dark:hover:bg-gray-700
         hover:border-gray-400 dark:hover:border-gray-500
         transition-all;
}

.preset-button--active {
  @apply bg-blue-50 dark:bg-blue-900/20
         border-blue-500 dark:border-blue-400
         text-blue-700 dark:text-blue-300
         ring-2 ring-blue-500/20;
}

.error-message {
  @apply text-sm text-red-600 dark:text-red-400;
}

.hint-text {
  @apply text-xs text-gray-500 dark:text-gray-400;
}
</style>
