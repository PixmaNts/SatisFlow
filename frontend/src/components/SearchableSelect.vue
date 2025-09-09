<template>
  <div class="searchable-select" ref="container">
    <div class="search-input-container">
      <input
        ref="searchInput"
        v-model="searchTerm"
        @input="handleInput"
        @focus="showDropdown = true"
        @blur="handleBlur"
        @keydown.down.prevent="moveDown"
        @keydown.up.prevent="moveUp"
        @keydown.enter.prevent="selectCurrent"
        @keydown.escape="hideDropdown"
        :placeholder="placeholder"
        class="search-input"
        autocomplete="off"
      />
      <div class="search-icon">🔍</div>
    </div>
    
    <div v-if="showDropdown && filteredOptions.length > 0" class="dropdown">
      <div
        v-for="(option, index) in filteredOptions"
        :key="getOptionKey(option)"
        :class="{
          'dropdown-item': true,
          'highlighted': index === highlightedIndex
        }"
        @mouseenter="highlightedIndex = index"
        @mousedown.prevent="selectOption(option)"
      >
        <div class="option-main">{{ getOptionLabel(option) }}</div>
        <div v-if="getOptionMeta(option)" class="option-meta">{{ getOptionMeta(option) }}</div>
      </div>
    </div>
    
    <div v-if="showDropdown && filteredOptions.length === 0 && searchTerm" class="no-results">
      No recipes found for "{{ searchTerm }}"
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'

interface Props {
  options: any[]
  modelValue: any
  placeholder?: string
  labelKey?: string
  metaKey?: string
  keyKey?: string
  filterKeys?: string[]
}

interface Emits {
  (e: 'update:modelValue', value: any): void
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: 'Search...',
  labelKey: 'name',
  metaKey: '',
  keyKey: 'name',
  filterKeys: () => ['name']
})

const emit = defineEmits<Emits>()

const container = ref<HTMLElement>()
const searchInput = ref<HTMLInputElement>()
const searchTerm = ref('')
const showDropdown = ref(false)
const highlightedIndex = ref(0)

// Get display label for an option
const getOptionLabel = (option: any): string => {
  return option[props.labelKey] || String(option)
}

// Get meta text for an option (e.g., machine type)
const getOptionMeta = (option: any): string => {
  return props.metaKey ? (option[props.metaKey] || '') : ''
}

// Get unique key for an option
const getOptionKey = (option: any): string => {
  return option[props.keyKey] || String(option)
}

// Filter options based on search term
const filteredOptions = computed(() => {
  if (!searchTerm.value.trim()) {
    return props.options.slice(0, 50) // Limit to first 50 for performance
  }
  
  const term = searchTerm.value.toLowerCase()
  return props.options.filter(option => {
    return props.filterKeys.some(key => {
      const value = option[key]
      return value && String(value).toLowerCase().includes(term)
    })
  }).slice(0, 50) // Limit results to 50 for performance
})

// Update search term when model value changes
watch(() => props.modelValue, (newValue) => {
  if (newValue) {
    searchTerm.value = getOptionLabel(newValue)
  } else {
    searchTerm.value = ''
  }
}, { immediate: true })

// Reset highlighted index when filtered options change
watch(filteredOptions, () => {
  highlightedIndex.value = 0
})

const handleInput = () => {
  showDropdown.value = true
  highlightedIndex.value = 0
}

const handleBlur = () => {
  // Delay hiding to allow for option selection
  setTimeout(() => {
    showDropdown.value = false
  }, 150)
}

const hideDropdown = () => {
  showDropdown.value = false
  searchInput.value?.blur()
}

const moveDown = () => {
  if (highlightedIndex.value < filteredOptions.value.length - 1) {
    highlightedIndex.value++
  }
}

const moveUp = () => {
  if (highlightedIndex.value > 0) {
    highlightedIndex.value--
  }
}

const selectCurrent = () => {
  if (filteredOptions.value[highlightedIndex.value]) {
    selectOption(filteredOptions.value[highlightedIndex.value])
  }
}

const selectOption = (option: any) => {
  emit('update:modelValue', option)
  searchTerm.value = getOptionLabel(option)
  showDropdown.value = false
  
  nextTick(() => {
    searchInput.value?.blur()
  })
}
</script>

<style scoped>
.searchable-select {
  position: relative;
  width: 100%;
}

.search-input-container {
  position: relative;
}

.search-input {
  width: 100%;
  padding: 0.75rem 2.5rem 0.75rem 0.75rem;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 1rem;
  background: white;
}

.search-input:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.search-icon {
  position: absolute;
  right: 0.75rem;
  top: 50%;
  transform: translateY(-50%);
  color: #6b7280;
  pointer-events: none;
  font-size: 0.875rem;
}

.dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: white;
  border: 1px solid #d1d5db;
  border-top: none;
  border-radius: 0 0 6px 6px;
  max-height: 200px;
  overflow-y: auto;
  z-index: 1000;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
}

.dropdown-item {
  padding: 0.75rem;
  cursor: pointer;
  border-bottom: 1px solid #f3f4f6;
}

.dropdown-item:last-child {
  border-bottom: none;
}

.dropdown-item:hover,
.dropdown-item.highlighted {
  background-color: #f8fafc;
}

.option-main {
  font-weight: 500;
  color: #1f2937;
}

.option-meta {
  font-size: 0.875rem;
  color: #6b7280;
  margin-top: 0.125rem;
}

.no-results {
  padding: 0.75rem;
  color: #6b7280;
  font-style: italic;
  text-align: center;
}
</style>