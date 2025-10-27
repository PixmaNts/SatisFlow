<template>
  <div class="searchable-select" ref="containerRef">
    <div class="select-container">
      <div
        class="select-trigger"
        :class="{
          'select-trigger--active': isOpen,
          'select-trigger--error': error,
          'select-trigger--disabled': disabled
        }"
        @click="toggleDropdown"
        role="combobox"
        :aria-expanded="isOpen"
        :aria-haspopup="true"
        :aria-label="label"
      >
        <div class="select-value">
          <span v-if="selectedOption" class="selected-text">
            {{ selectedOption.label }}
          </span>
          <span v-else class="placeholder-text">
            {{ placeholder }}
          </span>
        </div>
        <div class="select-icons">
          <button
            v-if="modelValue && !disabled"
            type="button"
            class="clear-button"
            @click.stop="clearSelection"
            aria-label="Clear selection"
          >
            ×
          </button>
          <span class="chevron-icon" :class="{ 'chevron-icon--rotated': isOpen }">
            ▼
          </span>
        </div>
      </div>

      <Transition name="dropdown">
        <div v-if="isOpen" class="dropdown-panel">
          <div class="search-container">
            <div class="search-input-wrapper">
              <span class="search-icon">🔍</span>
              <input
                ref="searchInputRef"
                v-model="searchQuery"
                type="text"
                class="search-input"
                placeholder="Start typing to search..."
                @keydown.down.prevent="navigateDown"
                @keydown.up.prevent="navigateUp"
                @keydown.enter.prevent="selectHighlighted"
                @keydown.escape="closeDropdown"
              />
              <span v-if="searchQuery" class="result-count">
                {{ filteredOptions.length }} results
              </span>
            </div>
          </div>

          <div class="options-container" ref="optionsRef">
            <div
              v-for="(option, index) in filteredOptions"
              :key="option.value"
              class="option-item"
              :class="{
                'option-item--selected': option.value === modelValue,
                'option-item--highlighted': index === highlightedIndex
              }"
              @click="selectOption(option)"
              @mouseenter="highlightedIndex = index"
              role="option"
              :aria-selected="option.value === modelValue"
            >
              <div class="option-content">
                <span class="option-label" v-html="highlightMatch(option.label)"></span>
                <span v-if="option.category" class="option-category">{{ option.category }}</span>
              </div>
              <span v-if="option.value === modelValue" class="check-icon">✓</span>
            </div>
            <div v-if="filteredOptions.length === 0" class="no-results">
              <p class="no-results-text">No recipes found</p>
              <p class="no-results-hint">Try a different search term</p>
            </div>
          </div>
        </div>
      </Transition>
    </div>

    <p v-if="error" class="error-message">{{ error }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted, onBeforeUnmount } from 'vue';

interface Option {
  value: string;
  label: string;
  category?: string;
}

interface Props {
  modelValue?: string;
  options: Option[];
  placeholder?: string;
  label?: string;
  error?: string;
  disabled?: boolean;
  required?: boolean;
}

interface Emits {
  (e: 'update:modelValue', value: string): void;
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: 'Select an option...',
  label: 'Select',
  disabled: false,
  required: false,
});

const emit = defineEmits<Emits>();

// State
const isOpen = ref(false);
const searchQuery = ref('');
const highlightedIndex = ref(0);
const containerRef = ref<HTMLElement | null>(null);
const searchInputRef = ref<HTMLInputElement | null>(null);
const optionsRef = ref<HTMLElement | null>(null);

// Computed
const selectedOption = computed(() => {
  return props.options.find(opt => opt.value === props.modelValue);
});

const filteredOptions = computed(() => {
  if (!searchQuery.value) {
    // If there's a selected option, show just that one as a hint
    if (selectedOption.value) {
      return [selectedOption.value];
    }
    // Otherwise show nothing - user must type to search
    return [];
  }

  const query = searchQuery.value.toLowerCase();
  const filtered = props.options.filter(option =>
    option.label.toLowerCase().includes(query) ||
    (option.category && option.category.toLowerCase().includes(query))
  );

  // Limit to first 50 results to keep it manageable
  return filtered.slice(0, 50);
});

// Methods
const toggleDropdown = () => {
  if (props.disabled) return;

  if (isOpen.value) {
    closeDropdown();
  } else {
    openDropdown();
  }
};

const openDropdown = async () => {
  isOpen.value = true;
  searchQuery.value = '';
  highlightedIndex.value = 0;

  await nextTick();
  searchInputRef.value?.focus();

  // Scroll to selected option if exists
  if (selectedOption.value) {
    const index = filteredOptions.value.findIndex(opt => opt.value === props.modelValue);
    if (index >= 0) {
      highlightedIndex.value = index;
      scrollToHighlighted();
    }
  }
};

const closeDropdown = () => {
  isOpen.value = false;
  searchQuery.value = '';
  highlightedIndex.value = 0;
};

const selectOption = (option: Option) => {
  emit('update:modelValue', option.value);
  closeDropdown();
};

const clearSelection = () => {
  emit('update:modelValue', '');
};

const navigateDown = () => {
  if (highlightedIndex.value < filteredOptions.value.length - 1) {
    highlightedIndex.value++;
    scrollToHighlighted();
  }
};

const navigateUp = () => {
  if (highlightedIndex.value > 0) {
    highlightedIndex.value--;
    scrollToHighlighted();
  }
};

const selectHighlighted = () => {
  const option = filteredOptions.value[highlightedIndex.value];
  if (option) {
    selectOption(option);
  }
};

const scrollToHighlighted = () => {
  nextTick(() => {
    const container = optionsRef.value;
    if (!container) return;

    const highlighted = container.children[highlightedIndex.value] as HTMLElement;
    if (!highlighted) return;

    const containerRect = container.getBoundingClientRect();
    const highlightedRect = highlighted.getBoundingClientRect();

    if (highlightedRect.bottom > containerRect.bottom) {
      highlighted.scrollIntoView({ block: 'end', behavior: 'smooth' });
    } else if (highlightedRect.top < containerRect.top) {
      highlighted.scrollIntoView({ block: 'start', behavior: 'smooth' });
    }
  });
};

const highlightMatch = (text: string): string => {
  if (!searchQuery.value) return text;

  const regex = new RegExp(`(${searchQuery.value})`, 'gi');
  return text.replace(regex, '<mark class="search-highlight">$1</mark>');
};

const handleClickOutside = (event: MouseEvent) => {
  if (containerRef.value && !containerRef.value.contains(event.target as Node)) {
    closeDropdown();
  }
};

// Watchers
watch(() => props.options, () => {
  highlightedIndex.value = 0;
});

watch(searchQuery, () => {
  highlightedIndex.value = 0;
});

// Lifecycle
onMounted(() => {
  document.addEventListener('click', handleClickOutside);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside);
});
</script>

<style scoped>
.searchable-select {
  @apply relative w-full;
}

.select-container {
  @apply relative;
}

.select-trigger {
  @apply w-full flex items-center justify-between gap-2 px-3 py-2
         bg-white dark:bg-gray-800
         border border-gray-300 dark:border-gray-600
         rounded-lg cursor-pointer
         transition-all duration-200
         hover:border-gray-400 dark:hover:border-gray-500;
}

.select-trigger--active {
  @apply border-blue-500 dark:border-blue-400 ring-2 ring-blue-500/20;
}

.select-trigger--error {
  @apply border-red-500 dark:border-red-400;
}

.select-trigger--disabled {
  @apply opacity-50 cursor-not-allowed bg-gray-100 dark:bg-gray-700;
}

.select-value {
  @apply flex-1 min-w-0;
}

.selected-text {
  @apply text-sm text-gray-900 dark:text-gray-100 truncate block;
}

.placeholder-text {
  @apply text-sm text-gray-500 dark:text-gray-400;
}

.select-icons {
  @apply flex items-center gap-1 flex-shrink-0;
}

.clear-button {
  @apply p-0.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700
         text-gray-500 dark:text-gray-400
         transition-colors;
  font-size: 1.25rem;
  line-height: 1;
}

.chevron-icon {
  @apply text-gray-500 dark:text-gray-400
         transition-transform duration-200;
  font-size: 0.625rem;
  line-height: 1;
}

.chevron-icon--rotated {
  transform: rotate(180deg);
}

.dropdown-panel {
  @apply absolute z-50 w-full mt-1
         bg-white dark:bg-gray-800
         border border-gray-300 dark:border-gray-600
         rounded-lg
         overflow-hidden;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.search-container {
  @apply p-2 border-b border-gray-200 dark:border-gray-700
         bg-gray-50 dark:bg-gray-750;
}

.search-input-wrapper {
  @apply relative flex items-center;
}

.search-icon {
  @apply absolute left-3 text-gray-400;
  font-size: 0.875rem;
  line-height: 1;
}

.search-input {
  @apply w-full pl-9 pr-20 py-2 text-sm
         bg-white dark:bg-gray-800
         border border-gray-300 dark:border-gray-600
         rounded-md
         text-gray-900 dark:text-gray-100
         placeholder-gray-400 dark:placeholder-gray-500
         focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500;
}

.result-count {
  @apply absolute right-3 text-xs text-gray-500 dark:text-gray-400;
}

.options-container {
  @apply max-h-64 overflow-y-auto;
}

.option-item {
  @apply flex items-center justify-between px-3 py-2
         cursor-pointer
         transition-colors duration-150
         border-b border-gray-100 dark:border-gray-700;
  background: white;
}

:global(.dark) .option-item {
  background: #1f2937;
}

.option-item:last-child {
  border-bottom: none;
}

.option-item:hover,
.option-item--highlighted {
  @apply bg-blue-50 dark:bg-blue-900/30;
}

.option-item--selected {
  @apply bg-blue-100 dark:bg-blue-900/40;
  font-weight: 500;
}

.option-content {
  @apply flex flex-col gap-0.5 flex-1 min-w-0;
}

.option-label {
  @apply text-sm text-gray-900 dark:text-gray-100;
}

.option-category {
  @apply text-xs text-gray-500 dark:text-gray-400;
}

.check-icon {
  @apply text-blue-600 dark:text-blue-400 flex-shrink-0;
  font-size: 0.875rem;
  line-height: 1;
  font-weight: 700;
}

.no-results {
  @apply flex flex-col items-center justify-center py-6 px-4;
}

.no-results-text {
  @apply text-sm font-medium text-gray-900 dark:text-gray-100 mb-1;
}

.no-results-hint {
  @apply text-xs text-gray-500 dark:text-gray-400;
}

.error-message {
  @apply mt-1 text-sm text-red-600 dark:text-red-400;
}

/* Dropdown transition */
.dropdown-enter-active,
.dropdown-leave-active {
  @apply transition-all duration-200;
}

.dropdown-enter-from,
.dropdown-leave-to {
  @apply opacity-0 -translate-y-1;
}

/* Search highlight */
:deep(.search-highlight) {
  @apply bg-yellow-200 dark:bg-yellow-900/50 font-medium;
}

/* Custom scrollbar */
.options-container::-webkit-scrollbar {
  @apply w-2;
}

.options-container::-webkit-scrollbar-track {
  @apply bg-gray-100 dark:bg-gray-700;
}

.options-container::-webkit-scrollbar-thumb {
  @apply bg-gray-300 dark:bg-gray-600 rounded-full;
}

.options-container::-webkit-scrollbar-thumb:hover {
  @apply bg-gray-400 dark:bg-gray-500;
}
</style>
