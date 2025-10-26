<template>
  <div class="recipe-autocomplete-demo p-8 max-w-4xl mx-auto">
    <div class="space-y-8">
      <!-- Header -->
      <div>
        <h1 class="text-3xl font-bold text-gray-900 mb-2">Recipe Autocomplete Demo</h1>
        <p class="text-gray-600">
          Try typing to search for recipes. The autocomplete provides instant feedback with full recipe details including inputs and outputs.
        </p>
      </div>

      <!-- Demo Section 1: Basic Autocomplete -->
      <div class="bg-white rounded-lg shadow-md p-6">
        <h2 class="text-xl font-semibold text-gray-800 mb-4">Basic Recipe Search</h2>
        <p class="text-sm text-gray-600 mb-4">Start typing any recipe name or machine type:</p>

        <div class="space-y-2">
          <label for="recipe-search" class="block text-sm font-medium text-gray-700">
            Select Recipe
          </label>
          <RecipeAutocomplete
            id="recipe-search"
            v-model="selectedRecipe"
            :recipes="demoRecipes"
            placeholder="Type 'iron', 'smelter', or 'cable'..."
            clearable
            @selected="onRecipeSelected"
          />
        </div>

        <!-- Selected Recipe Info -->
        <div v-if="selectedRecipe && selectedRecipeInfo" class="mt-6 p-4 bg-blue-50 rounded-lg">
          <h3 class="font-semibold text-blue-900 mb-3">Selected Recipe Details:</h3>
          <div class="space-y-2 text-sm text-blue-800">
            <div>
              <span class="font-medium">Recipe:</span> {{ selectedRecipeInfo.name }}
            </div>
            <div>
              <span class="font-medium">Machine:</span> {{ selectedRecipeInfo.machine }}
            </div>
            <div>
              <span class="font-medium">Inputs:</span>
              <div class="ml-4 mt-1">
                <div v-for="input in selectedRecipeInfo.inputs" :key="input.item">
                  • {{ input.item }}: {{ input.quantity }} per minute
                </div>
              </div>
            </div>
            <div>
              <span class="font-medium">Outputs:</span>
              <div class="ml-4 mt-1">
                <div v-for="output in selectedRecipeInfo.outputs" :key="output.item">
                  • {{ output.item }}: {{ output.quantity }} per minute
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Demo Section 2: Search Examples -->
      <div class="bg-gray-50 rounded-lg p-6">
        <h2 class="text-xl font-semibold text-gray-800 mb-4">Try These Searches</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div
            v-for="example in searchExamples"
            :key="example.term"
            class="bg-white p-4 rounded border border-gray-200 cursor-pointer hover:border-blue-500 hover:bg-blue-50 transition"
            @click="trySearch(example.term)"
          >
            <div class="font-medium text-gray-900">{{ example.label }}</div>
            <div class="text-sm text-gray-600 mt-1">
              Try searching: <code class="bg-gray-100 px-2 py-1 rounded text-blue-600">{{ example.term }}</code>
            </div>
            <div class="text-xs text-gray-500 mt-2">Matches: {{ example.matches }}</div>
          </div>
        </div>
      </div>

      <!-- Demo Section 3: Features List -->
      <div class="bg-white rounded-lg shadow-md p-6">
        <h2 class="text-xl font-semibold text-gray-800 mb-4">Key Features</h2>
        <ul class="space-y-3">
          <li class="flex items-start gap-3">
            <span class="text-green-600 font-bold">✓</span>
            <div>
              <div class="font-medium text-gray-900">Search as You Type</div>
              <div class="text-sm text-gray-600">Find recipes instantly without scrolling huge lists</div>
            </div>
          </li>
          <li class="flex items-start gap-3">
            <span class="text-green-600 font-bold">✓</span>
            <div>
              <div class="font-medium text-gray-900">Rich Recipe Details</div>
              <div class="text-sm text-gray-600">See inputs, outputs, and machine requirements at a glance</div>
            </div>
          </li>
          <li class="flex items-start gap-3">
            <span class="text-green-600 font-bold">✓</span>
            <div>
              <div class="font-medium text-gray-900">Keyboard Navigation</div>
              <div class="text-sm text-gray-600">Use arrow keys and Enter to select without mouse</div>
            </div>
          </li>
          <li class="flex items-start gap-3">
            <span class="text-green-600 font-bold">✓</span>
            <div>
              <div class="font-medium text-gray-900">Accessible</div>
              <div class="text-sm text-gray-600">Full ARIA support for screen readers and assistive technology</div>
            </div>
          </li>
          <li class="flex items-start gap-3">
            <span class="text-green-600 font-bold">✓</span>
            <div>
              <div class="font-medium text-gray-900">Smart Search</div>
              <div class="text-sm text-gray-600">Searches both recipe names and machine types simultaneously</div>
            </div>
          </li>
        </ul>
      </div>

      <!-- Demo Section 4: Keyboard Shortcuts -->
      <div class="bg-gray-50 rounded-lg p-6">
        <h2 class="text-xl font-semibold text-gray-800 mb-4">Keyboard Shortcuts</h2>
        <div class="space-y-2 text-sm">
          <div class="flex justify-between items-center">
            <span class="text-gray-700">Arrow Up / Down</span>
            <kbd class="px-3 py-1 bg-white border border-gray-300 rounded font-mono text-xs">↑ ↓</kbd>
            <span class="text-gray-500">Navigate suggestions</span>
          </div>
          <div class="flex justify-between items-center">
            <span class="text-gray-700">Enter</span>
            <kbd class="px-3 py-1 bg-white border border-gray-300 rounded font-mono text-xs">⏎</kbd>
            <span class="text-gray-500">Select highlighted recipe</span>
          </div>
          <div class="flex justify-between items-center">
            <span class="text-gray-700">Escape</span>
            <kbd class="px-3 py-1 bg-white border border-gray-300 rounded font-mono text-xs">Esc</kbd>
            <span class="text-gray-500">Close dropdown or clear selection</span>
          </div>
          <div class="flex justify-between items-center">
            <span class="text-gray-700">Tab</span>
            <kbd class="px-3 py-1 bg-white border border-gray-300 rounded font-mono text-xs">Tab</kbd>
            <span class="text-gray-500">Move to next field</span>
          </div>
        </div>
      </div>

      <!-- Demo Section 5: Recipe Statistics -->
      <div class="bg-white rounded-lg shadow-md p-6">
        <h2 class="text-xl font-semibold text-gray-800 mb-4">Available Recipes</h2>
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div v-for="machine in machineStats" :key="machine.name" class="bg-gray-50 p-4 rounded text-center">
            <div class="text-2xl font-bold text-blue-600">{{ machine.count }}</div>
            <div class="text-sm text-gray-600">{{ machine.name }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import RecipeAutocomplete from './RecipeAutocomplete.vue'
import type { RecipeInfo } from '@/api/types'

// Demo recipes
const demoRecipes = ref<RecipeInfo[]>([
  {
    name: 'Iron Ingot',
    machine: 'Smelter',
    inputs: [{ item: 'IronOre', quantity: 30 }],
    outputs: [{ item: 'IronIngot', quantity: 30 }]
  },
  {
    name: 'Copper Ingot',
    machine: 'Smelter',
    inputs: [{ item: 'CopperOre', quantity: 30 }],
    outputs: [{ item: 'CopperIngot', quantity: 30 }]
  },
  {
    name: 'Steel Beam',
    machine: 'Smelter',
    inputs: [
      { item: 'IronOre', quantity: 30 },
      { item: 'Coal', quantity: 15 }
    ],
    outputs: [{ item: 'SteelBeam', quantity: 15 }]
  },
  {
    name: 'Iron Plate',
    machine: 'Assembler',
    inputs: [{ item: 'IronIngot', quantity: 30 }],
    outputs: [{ item: 'IronPlate', quantity: 20 }]
  },
  {
    name: 'Copper Sheet',
    machine: 'Assembler',
    inputs: [{ item: 'CopperIngot', quantity: 30 }],
    outputs: [{ item: 'CopperSheet', quantity: 20 }]
  },
  {
    name: 'Cable',
    machine: 'Assembler',
    inputs: [{ item: 'Wire', quantity: 60 }],
    outputs: [{ item: 'Cable', quantity: 30 }]
  },
  {
    name: 'Iron Rod',
    machine: 'Constructor',
    inputs: [{ item: 'IronIngot', quantity: 15 }],
    outputs: [{ item: 'IronRod', quantity: 15 }]
  },
  {
    name: 'Screw',
    machine: 'Constructor',
    inputs: [{ item: 'IronRod', quantity: 12 }],
    outputs: [{ item: 'Screw', quantity: 40 }]
  },
  {
    name: 'Wire',
    machine: 'Constructor',
    inputs: [{ item: 'CopperIngot', quantity: 30 }],
    outputs: [{ item: 'Wire', quantity: 30 }]
  },
  {
    name: 'Quickwire',
    machine: 'Constructor',
    inputs: [
      { item: 'CopperIngot', quantity: 60 },
      { item: 'Silica', quantity: 30 }
    ],
    outputs: [{ item: 'Quickwire', quantity: 60 }]
  }
])

// State
const selectedRecipe = ref('')
const selectedRecipeInfo = computed(() => {
  if (!selectedRecipe.value) return null
  return demoRecipes.value.find(r => r.name === selectedRecipe.value)
})

// Search examples
const searchExamples = [
  {
    term: 'iron',
    label: 'Search by Item',
    matches: 'Iron Ingot, Iron Plate, Iron Rod'
  },
  {
    term: 'smelter',
    label: 'Search by Machine',
    matches: 'All recipes using Smelter'
  },
  {
    term: 'assembler',
    label: 'Search by Machine Type',
    matches: 'Cable, Iron Plate, Copper Sheet'
  },
  {
    term: 'wire',
    label: 'Search by Name',
    matches: 'Wire, Quickwire'
  }
]

// Machine statistics
const machineStats = computed(() => {
  const stats = new Map<string, number>()
  for (const recipe of demoRecipes.value) {
    stats.set(recipe.machine, (stats.get(recipe.machine) || 0) + 1)
  }
  return Array.from(stats.entries())
    .map(([name, count]) => ({ name, count }))
    .sort((a, b) => b.count - a.count)
})

// Event handlers
const onRecipeSelected = (recipe: RecipeInfo) => {
  console.log('Recipe selected:', recipe)
}

const trySearch = (term: string) => {
  selectedRecipe.value = ''
  // Focus the input and set it (will be set by user typing)
  const input = document.querySelector('#recipe-search input') as HTMLInputElement
  if (input) {
    input.focus()
    input.value = term
    input.dispatchEvent(new Event('input', { bubbles: true }))
  }
}
</script>

<style scoped>
.recipe-autocomplete-demo {
  font-family: system-ui, -apple-system, sans-serif;
}

code {
  font-family: 'Courier New', monospace;
}

kbd {
  user-select: none;
}
</style>
