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

<style scoped lang="scss">
.searchable-select {
  position: relative;
  width: 100%;
}

.select-container {
  position: relative;
}

.select-trigger {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-sm, 0.5rem);
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  background: var(--color-surface, #252525);
  border: 1px solid var(--color-border, #404040);
  border-radius: var(--border-radius-lg, 0.5rem);
  cursor: pointer;
  transition: all var(--transition-normal, 200ms);

  &:hover:not(.select-trigger--disabled) {
    border-color: var(--color-border-light, #4a4a4a);
  }
}

.select-trigger--active {
  border-color: var(--color-info-blue, #4a90a4);
  box-shadow: 0 0 0 2px rgba(74, 144, 164, 0.2);
}

.select-trigger--error {
  border-color: var(--color-error, #ef4444);
}

.select-trigger--disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background: var(--color-surface-inset, #1f1f1f);
}

.select-value {
  flex: 1;
  min-width: 0;
}

.selected-text {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-primary, #e5e5e5);
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.placeholder-text {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-secondary, #b8b8b8);
}

.select-icons {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
  flex-shrink: 0;
}

.clear-button {
  padding: var(--spacing-xs, 0.25rem);
  border-radius: var(--border-radius-sm, 3px);
  background: transparent;
  border: none;
  color: var(--color-text-secondary, #b8b8b8);
  cursor: pointer;
  transition: background-color var(--transition-fast, 150ms);
  font-size: 1.25rem;
  line-height: 1;

  &:hover {
    background: var(--color-surface-hover, #2a2a2a);
  }
}

.chevron-icon {
  color: var(--color-text-secondary, #b8b8b8);
  transition: transform var(--transition-normal, 200ms);
  font-size: 0.625rem;
  line-height: 1;
}

.chevron-icon--rotated {
  transform: rotate(180deg);
}

.dropdown-panel {
  position: absolute;
  z-index: 50;
  width: 100%;
  margin-top: var(--spacing-xs, 0.25rem);
  background: var(--color-surface, #252525);
  border: 1px solid var(--color-border, #404040);
  border-radius: var(--border-radius-lg, 0.5rem);
  overflow: hidden;
  box-shadow: var(--shadow-md);
}

.search-container {
  padding: var(--spacing-sm, 0.5rem);
  border-bottom: 1px solid var(--color-border, #404040);
  background: var(--color-surface-inset, #1f1f1f);
}

.search-input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: var(--spacing-sm, 0.5rem);
  color: var(--color-text-secondary, #b8b8b8);
  font-size: 0.875rem;
  line-height: 1;
}

.search-input {
  width: 100%;
  padding: var(--spacing-sm, 0.5rem) 5rem var(--spacing-sm, 0.5rem) 2.25rem;
  font-size: var(--font-size-sm, 0.875rem);
  background: var(--color-surface, #252525);
  border: 1px solid var(--color-border, #404040);
  border-radius: var(--border-radius-sm, 3px);
  color: var(--color-text-primary, #e5e5e5);
  transition: border-color var(--transition-fast, 150ms), box-shadow var(--transition-fast, 150ms);

  &::placeholder {
    color: var(--color-text-muted, #8a8a8a);
  }

  &:focus {
    outline: none;
    border-color: var(--color-info-blue, #4a90a4);
    box-shadow: 0 0 0 2px rgba(74, 144, 164, 0.2);
  }
}

.result-count {
  position: absolute;
  right: var(--spacing-sm, 0.5rem);
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-text-secondary, #b8b8b8);
}

.options-container {
  max-height: 16rem;
  overflow-y: auto;
}

.option-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  cursor: pointer;
  transition: background-color var(--transition-fast, 150ms);
  border-bottom: 1px solid var(--color-border, #404040);
  background: var(--color-surface, #252525);

  &:last-child {
    border-bottom: none;
  }

  &:hover,
  &.option-item--highlighted {
    background: rgba(74, 144, 164, 0.15);
  }

  &.option-item--selected {
    background: rgba(74, 144, 164, 0.25);
    font-weight: var(--font-weight-medium, 500);
  }
}

.option-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2xs, 0.125rem);
  flex: 1;
  min-width: 0;
}

.option-label {
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-text-primary, #e5e5e5);
}

.option-category {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-text-secondary, #b8b8b8);
}

.check-icon {
  color: var(--color-info-blue, #4a90a4);
  flex-shrink: 0;
  font-size: 0.875rem;
  line-height: 1;
  font-weight: var(--font-weight-bold, 700);
}

.no-results {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-lg, 1rem) var(--spacing-sm, 0.5rem);
}

.no-results-text {
  font-size: var(--font-size-sm, 0.875rem);
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-text-primary, #e5e5e5);
  margin-bottom: var(--spacing-xs, 0.25rem);
}

.no-results-hint {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-text-secondary, #b8b8b8);
}

.error-message {
  margin-top: var(--spacing-xs, 0.25rem);
  font-size: var(--font-size-sm, 0.875rem);
  color: var(--color-error, #ef4444);
}

/* Dropdown transition */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all var(--transition-normal, 200ms);
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-2px);
}

/* Search highlight */
:deep(.search-highlight) {
  background: rgba(245, 158, 11, 0.3);
  font-weight: var(--font-weight-medium, 500);
}

/* Custom scrollbar */
.options-container::-webkit-scrollbar {
  width: 0.5rem;
}

.options-container::-webkit-scrollbar-track {
  background: var(--color-surface-hover, #2a2a2a);
}

.options-container::-webkit-scrollbar-thumb {
  background: var(--color-border, #404040);
  border-radius: var(--border-radius-full, 9999px);

  &:hover {
    background: var(--color-border-light, #4a4a4a);
  }
}
</style>
