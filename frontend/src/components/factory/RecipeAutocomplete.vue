<template>
  <div
    ref="rootElement"
    class="recipe-autocomplete"
    :class="{ 'is-disabled': disabled }"
  >
    <div class="input-wrapper">
      <input
        :id="inputId"
        ref="inputElement"
        :value="inputValue"
        class="autocomplete-input"
        type="text"
        :placeholder="placeholder"
        :disabled="disabled"
        autocomplete="off"
        role="combobox"
        :aria-expanded="isOpen"
        :aria-controls="listboxId"
        :aria-autocomplete="'list'"
        @focus="openDropdown"
        @keydown="handleKeydown"
        @input="handleInput"
        @blur="handleBlur"
      />

      <button
        v-if="clearable && searchTerm && !disabled"
        type="button"
        class="clear-button"
        aria-label="Clear selected recipe"
        @mousedown.prevent
        @click="clearSelection"
      >
        ×
      </button>
    </div>

    <transition name="fade">
      <ul
        v-if="isOpen"
        :id="listboxId"
        role="listbox"
        class="suggestions"
      >
        <li
          v-for="(recipe, index) in visibleRecipes"
          :key="recipe.name"
          role="option"
          :aria-selected="index === highlightedIndex"
          class="suggestion"
          :class="{ 'is-active': index === highlightedIndex }"
          @mousedown.prevent="selectRecipe(recipe)"
          @mouseenter="setHighlightedIndex(index)"
        >
          <div class="suggestion-header">
            <div class="suggestion-name">{{ recipe.name }}</div>
            <div class="suggestion-machine">{{ recipe.machine }}</div>
          </div>
          <div class="suggestion-io">
            <div class="suggestion-inputs">
              <span class="io-label">Inputs:</span>
              <span class="io-items">
                {{ formatItems(recipe.inputs) }}
              </span>
            </div>
            <div class="suggestion-outputs">
              <span class="io-label">Outputs:</span>
              <span class="io-items">
                {{ formatItems(recipe.outputs) }}
              </span>
            </div>
          </div>
        </li>
        <li v-if="!visibleRecipes.length" class="empty-state">
          No recipes found
        </li>
      </ul>
    </transition>
  </div>
</template>

<script setup lang="ts">
import {
  computed,
  nextTick,
  onBeforeUnmount,
  onMounted,
  ref,
  watch
} from 'vue'
import type { RecipeInfo } from '@/api/types'
import { formatItemName } from '@/composables/useItemIcon'

interface Props {
  modelValue: string
  recipes: RecipeInfo[]
  placeholder?: string
  id?: string
  maxSuggestions?: number
  disabled?: boolean
  clearable?: boolean
}

interface Emits {
  (e: 'update:modelValue', value: string): void
  (e: 'selected', recipe: RecipeInfo): void
  (e: 'cleared'): void
  (e: 'focus'): void
  (e: 'blur'): void
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: 'Search recipes...',
  recipes: () => [],
  id: undefined,
  maxSuggestions: 8,
  disabled: false,
  clearable: true
})

const emit = defineEmits<Emits>()

const rootElement = ref<HTMLElement | null>(null)
const inputElement = ref<HTMLInputElement | null>(null)
const searchTerm = ref('')
const inputValue = ref('') // The actual displayed value in the input
const isOpen = ref(false)
const highlightedIndex = ref(-1)
const lastEmittedRecipe = ref<string | null>(null) // Track last emitted recipe to avoid loops

// Update inputValue when searchTerm or isOpen changes
watch([searchTerm, isOpen], ([term, open]) => {
  if (!term) {
    inputValue.value = ''
    return
  }
  
  // When dropdown is open, show the raw search term (what user is typing)
  if (open) {
    inputValue.value = term
  } else {
    // When dropdown is closed, format the recipe identifier for display
    inputValue.value = formatItemName(term)
  }
}, { immediate: true })

const instanceId = Math.random().toString(36).slice(2, 9)
const inputId = computed(() => props.id ?? `recipe-autocomplete-${instanceId}`)
const listboxId = computed(() => `${inputId.value}-listbox`)

const normalizedRecipes = computed<RecipeInfo[]>(() => {
  if (!props.recipes?.length) return []
  return props.recipes
})

const visibleRecipes = computed<RecipeInfo[]>(() => {
  if (!normalizedRecipes.value.length) return []

  const term = searchTerm.value.trim().toLowerCase()
  const suggestions = term
    ? normalizedRecipes.value.filter(recipe => {
        // Search by recipe identifier (name) and formatted name for better matching
        const recipeNameLower = recipe.name.toLowerCase()
        const formattedNameLower = formatItemName(recipe.name).toLowerCase()
        return (
          recipeNameLower.includes(term) ||
          formattedNameLower.includes(term) ||
          recipe.machine.toLowerCase().includes(term)
        )
      })
    : normalizedRecipes.value

  return suggestions.slice(0, props.maxSuggestions)
})

const openDropdown = () => {
  if (props.disabled) return
  // When opening, ensure searchTerm is set to the modelValue (recipe identifier)
  // This ensures search works correctly with the identifier format
  if (props.modelValue && searchTerm.value !== props.modelValue) {
    searchTerm.value = props.modelValue
  }
  isOpen.value = true
  highlightedIndex.value = visibleRecipes.value.length ? 0 : -1
  emit('focus')
}

const closeDropdown = () => {
  isOpen.value = false
  highlightedIndex.value = -1
}

const setHighlightedIndex = (index: number) => {
  highlightedIndex.value = index
}

const selectRecipe = (recipe: RecipeInfo) => {
  searchTerm.value = recipe.name
  inputValue.value = formatItemName(recipe.name) // Format immediately for display
  lastEmittedRecipe.value = recipe.name // Track that we're emitting this
  emit('update:modelValue', recipe.name)
  emit('selected', recipe)
  nextTick(() => {
    closeDropdown()
    inputElement.value?.blur()
  })
}

const clearSelection = () => {
  searchTerm.value = ''
  inputValue.value = ''
  highlightedIndex.value = -1
  emit('update:modelValue', '')
  emit('cleared')
  nextTick(() => {
    inputElement.value?.focus()
  })
}

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  const newValue = target.value
  
  // Update searchTerm with what user typed
  searchTerm.value = newValue
  // Also update inputValue directly (watch will handle formatting when closed)
  inputValue.value = newValue
  
  if (!isOpen.value) {
    openDropdown()
  }

  if (!searchTerm.value) {
    emit('update:modelValue', '')
    emit('cleared')
  }
}

const handleKeydown = (event: KeyboardEvent) => {
  if (!isOpen.value && (event.key === 'ArrowDown' || event.key === 'ArrowUp')) {
    openDropdown()
    event.preventDefault()
    return
  }

  switch (event.key) {
    case 'ArrowDown':
      event.preventDefault()
      if (!visibleRecipes.value.length) return
      highlightedIndex.value = (highlightedIndex.value + 1) % visibleRecipes.value.length
      break
    case 'ArrowUp':
      event.preventDefault()
      if (!visibleRecipes.value.length) return
      highlightedIndex.value =
        (highlightedIndex.value - 1 + visibleRecipes.value.length) % visibleRecipes.value.length
      break
    case 'Enter':
      if (isOpen.value && highlightedIndex.value >= 0) {
        event.preventDefault()
        const recipe = visibleRecipes.value[highlightedIndex.value]
        if (recipe) {
          selectRecipe(recipe)
        }
      }
      break
    case 'Escape':
      if (isOpen.value) {
        event.preventDefault()
        closeDropdown()
        // Reset to modelValue when closing
        searchTerm.value = props.modelValue ?? ''
      } else {
        searchTerm.value = props.modelValue ?? ''
      }
      break
    case 'Tab':
      closeDropdown()
      break
    default:
      break
  }
}

const handleBlur = () => {
  // Delay closing to allow click selection
  requestAnimationFrame(() => {
    closeDropdown()
    emit('blur')
    // Reset to modelValue (recipe identifier) - the watch will format it for display
    if (props.modelValue) {
      searchTerm.value = props.modelValue
    } else {
      searchTerm.value = ''
    }
  })
}

const handleFocusOutside = (event: MouseEvent) => {
  if (!rootElement.value) return
  if (rootElement.value.contains(event.target as Node)) return
  closeDropdown()
}

const formatItems = (items?: Array<{ item: string; quantity: number }>) => {
  if (!items || items.length === 0) return 'None'
  return items.map(i => {
    const qty = typeof i.quantity === 'number' ? i.quantity.toFixed(0) : i.quantity
    return `${i.item} (${qty})`
  }).join(', ')
}

// Watch for modelValue changes
watch(
  () => props.modelValue,
  (value) => {
    // When modelValue changes, update searchTerm to the recipe identifier
    if (value !== searchTerm.value) {
      searchTerm.value = value ?? ''
      // Reset lastEmittedRecipe when modelValue changes externally (e.g., when editing)
      if (value !== lastEmittedRecipe.value) {
        lastEmittedRecipe.value = null
      }
    }
  },
  { immediate: true }
)

// Watch for when recipes become available and modelValue is set
// This ensures we emit the selected event when editing a production line
watch(
  () => [props.modelValue, normalizedRecipes.value.length],
  ([value, recipeCount]) => {
    if (value && recipeCount > 0 && value !== lastEmittedRecipe.value) {
      const matchingRecipe = normalizedRecipes.value.find(r => r.name === value)
      if (matchingRecipe) {
        lastEmittedRecipe.value = value
        // Emit selected event so parent knows recipe is selected
        // Use nextTick to avoid infinite loops
        nextTick(() => {
          emit('selected', matchingRecipe)
        })
      }
    }
  },
  { immediate: true }
)

watch(
  () => visibleRecipes.value.length,
  length => {
    if (!length) {
      highlightedIndex.value = -1
    } else if (highlightedIndex.value === -1) {
      highlightedIndex.value = 0
    }
  }
)

onMounted(() => {
  document.addEventListener('mousedown', handleFocusOutside)
})

onBeforeUnmount(() => {
  document.removeEventListener('mousedown', handleFocusOutside)
})
</script>

<style scoped lang="scss">
.recipe-autocomplete {
  position: relative;
  width: 100%;

  &.is-disabled {
    opacity: 0.6;
    pointer-events: none;
  }
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.autocomplete-input {
  width: 100%;
  padding: var(--spacing-sm, 0.5rem) calc(var(--spacing-lg, 1rem));
  padding-right: 2.25rem;
  border: 1px solid var(--color-gray-300, #d1d5db);
  border-radius: var(--border-radius-md, 0.375rem);
  font-size: var(--font-size-base, 1rem);
  background-color: var(--color-white, #ffffff);
  transition: border-color 0.2s ease-in-out, box-shadow 0.2s ease-in-out;

  &:focus {
    outline: none;
    border-color: var(--color-primary-500, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  &::placeholder {
    color: var(--color-gray-400, #9ca3af);
  }
}

.clear-button {
  position: absolute;
  right: 0.5rem;
  background: transparent;
  border: none;
  font-size: 1.25rem;
  line-height: 1;
  color: var(--color-gray-400, #9ca3af);
  cursor: pointer;
  padding: 0;

  &:hover {
    color: var(--color-gray-600, #4b5563);
  }
}

.suggestions {
  position: absolute;
  z-index: 10;
  top: calc(100% + 0.25rem);
  left: 0;
  right: 0;
  max-height: 16rem;
  overflow-y: auto;
  background-color: var(--color-white, #ffffff);
  border: 1px solid var(--color-gray-200, #e5e7eb);
  border-radius: var(--border-radius-md, 0.375rem);
  box-shadow: 0 10px 30px rgba(15, 23, 42, 0.12);
  margin: 0;
  padding: var(--spacing-xs, 0.25rem) 0;
  list-style: none;
}

.suggestion {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 0.25rem);
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  cursor: pointer;
  border-bottom: 1px solid var(--color-gray-100, #f3f4f6);

  &:last-child {
    border-bottom: none;
  }

  &:hover,
  &.is-active {
    background-color: var(--color-primary-50, #eff6ff);
  }
}

.suggestion-header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  gap: var(--spacing-md, 0.75rem);
}

.suggestion-name {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-900, #111827);
  flex: 1;
}

.suggestion-machine {
  font-size: var(--font-size-xs, 0.75rem);
  color: var(--color-gray-500, #6b7280);
  background-color: var(--color-gray-100, #f3f4f6);
  padding: 0.125rem 0.5rem;
  border-radius: 0.25rem;
  white-space: nowrap;
}

.suggestion-io {
  display: flex;
  gap: var(--spacing-md, 0.75rem);
  margin-top: var(--spacing-2xs, 0.125rem);
}

.suggestion-inputs,
.suggestion-outputs {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 0.25rem);
  font-size: var(--font-size-xs, 0.75rem);
  flex: 1;
}

.io-label {
  font-weight: var(--font-weight-medium, 500);
  color: var(--color-gray-600, #4b5563);
}

.io-items {
  color: var(--color-gray-500, #6b7280);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.empty-state {
  padding: var(--spacing-sm, 0.5rem) var(--spacing-md, 0.75rem);
  color: var(--color-gray-500, #6b7280);
  font-size: var(--font-size-sm, 0.875rem);
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>

