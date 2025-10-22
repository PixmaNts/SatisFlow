# Satisflow Frontend Implementation Guide

## Overview

This guide provides detailed instructions for implementing the Vue.js frontend for the Satisflow project. The frontend provides a web interface for managing factories, production lines, and logistics.

## Prerequisites

- Node.js 18+ installed
- Basic understanding of Vue.js and TypeScript
- Familiarity with modern JavaScript (ES6+)

## Project Setup

### 1. Create the Frontend Project

Create a new Vue.js project in the root directory:

```bash
# Navigate to project root
cd Satisflow

# Create Vue.js project with TypeScript
npm create vue@latest frontend

# Answer the prompts:
# ✔ Project name: frontend
# ✔ Add TypeScript? Yes
# ✔ Add JSX Support? No
# ✔ Add Vue Router for Single Page Application development? Yes
# ✔ Add Pinia for state management? No (we'll use simple state)
# ✔ Add Vitest for Unit Testing? Yes
# ✔ Add an End-to-End Testing Solution? No
# ✔ Add ESLint for code quality? Yes
# ✔ Add Prettier for code formatting? Yes

# Navigate to the frontend directory
cd frontend

# Install dependencies
npm install
```

### 2. Install Additional Dependencies

```bash
# HTTP client for API calls
npm install axios

# Type definitions for better development experience
npm install -D @types/node

# UI components (optional, for better styling)
npm install @headlessui/vue @heroicons/vue

# CSS framework (optional)
npm install tailwindcss @tailwindcss/forms
npx tailwindcss init -p
```

### 3. Configure Vite

Update `frontend/vite.config.ts` to include API proxy:

```typescript
// frontend/vite.config.ts
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { fileURLToPath, URL } from 'node:url'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  server: {
    port: 5173,
    proxy: {
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true
      }
    }
  },
  build: {
    outDir: 'dist',
    sourcemap: true
  }
})
```

### 4. Configure TypeScript

Update `frontend/tsconfig.json` for better type checking:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "preserve",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "baseUrl": ".",
    "paths": {
      "@/*": ["src/*"]
    }
  },
  "include": ["src/**/*.ts", "src/**/*.tsx", "src/**/*.vue"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

### 5. Configure Tailwind CSS (if using)

Update `frontend/tailwind.config.js`:

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#eff6ff',
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8',
        }
      }
    },
  },
  plugins: [],
}
```

## Project Structure

```
frontend/src/
├── main.ts                 # Application entry point
├── App.vue                 # Root component
├── router/
│   └── index.ts            # Vue Router configuration
├── components/
│   ├── layout/
│   │   ├── MainNav.vue     # Navigation bar
│   │   └── PageHeader.vue  # Page headers
│   ├── factory/
│   │   ├── FactoryList.vue # Factory list component
│   │   ├── FactoryForm.vue # Factory creation/edit form
│   │   └── ProductionLineForm.vue # Production line form
│   ├── logistics/
│   │   ├── LogisticsList.vue # Logistics list
│   │   └── LogisticsForm.vue # Logistics form
│   └── ui/
│       ├── Button.vue      # Reusable button
│       ├── Modal.vue       # Modal component
│       └── Loading.vue     # Loading spinner
├── views/
│   ├── DashboardView.vue   # Dashboard view
│   ├── FactoryView.vue     # Factory view
│   └── LogisticsView.vue   # Logistics view
├── api/
│   ├── client.ts           # API client configuration
│   ├── types.ts            # TypeScript type definitions
│   └── endpoints.ts        # API endpoint functions
├── composables/
│   ├── useFactories.ts     # Factory state management
│   ├── useLogistics.ts     # Logistics state management
│   └── useDashboard.ts     # Dashboard state management
└── styles/
    ├── main.css            # Global styles
    └── components.css      # Component-specific styles
```

## Implementation Steps

### 1. API Client Setup (api/client.ts)

Create a configured HTTP client:

```typescript
// frontend/src/api/client.ts
import axios, { AxiosInstance, AxiosResponse } from 'axios'

class ApiClient {
  private client: AxiosInstance

  constructor() {
    this.client = axios.create({
      baseURL: '/api',
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json',
      },
    })

    // Request interceptor
    this.client.interceptors.request.use(
      (config) => {
        // Add auth token if available
        // const token = localStorage.getItem('authToken')
        // if (token) {
        //   config.headers.Authorization = `Bearer ${token}`
        // }
        return config
      },
      (error) => {
        return Promise.reject(error)
      }
    )

    // Response interceptor
    this.client.interceptors.response.use(
      (response: AxiosResponse) => {
        return response
      },
      (error) => {
        if (error.response?.status === 401) {
          // Handle unauthorized
          console.error('Unauthorized access')
        } else if (error.response?.status >= 500) {
          // Handle server errors
          console.error('Server error:', error.response.data)
        }
        return Promise.reject(error)
      }
    )
  }

  async get<T>(url: string, params?: any): Promise<T> {
    const response = await this.client.get(url, { params })
    return response.data
  }

  async post<T>(url: string, data?: any): Promise<T> {
    const response = await this.client.post(url, data)
    return response.data
  }

  async put<T>(url: string, data?: any): Promise<T> {
    const response = await this.client.put(url, data)
    return response.data
  }

  async delete<T>(url: string): Promise<T> {
    const response = await this.client.delete(url)
    return response.data
  }
}

export const apiClient = new ApiClient()
```

### 2. TypeScript Types (api/types.ts)

Define TypeScript interfaces for API data:

```typescript
// frontend/src/api/types.ts

// Base types
export interface BaseEntity {
  id: number
}

// Factory types
export interface Factory extends BaseEntity {
  name: string
  description?: string
  notes?: string
  production_lines: ProductionLine[]
  raw_inputs: RawInput[]
  power_generators: PowerGenerator[]
  items: Record<string, number>
}

export interface CreateFactoryRequest {
  name: string
  description?: string
  notes?: string
}

export interface UpdateFactoryRequest {
  name?: string
  description?: string
  notes?: string
}

// Production Line types
export interface ProductionLine extends BaseEntity {
  name: string
  description?: string
  production_type: 'recipe' | 'blueprint'
  // Add other fields as needed
}

export interface CreateProductionLineRequest {
  name: string
  description?: string
  recipe: string
  machine_groups: MachineGroup[]
}

export interface MachineGroup {
  machine_type: string
  num_machines: number
  overclock: number
  sommersloop: number
}

// Raw Input types
export interface RawInput extends BaseEntity {
  extractor_type: string
  item: string
  purity?: string
  quantity_per_min: number
}

export interface CreateRawInputRequest {
  extractor_type: string
  item: string
  purity?: string
  quantity_per_min: number
}

// Power Generator types
export interface PowerGenerator extends BaseEntity {
  generator_type: string
  fuel_type?: string
  power_output: number
}

export interface CreatePowerGeneratorRequest {
  generator_type: string
  fuel_type?: string
  num_machines: number
  overclock: number
}

// Logistics response
export interface Logistics extends BaseEntity {
  from_factory: number
  to_factory: number
  transport_type: TransportType
  transport_id: string
  transport_name: string | null
  transport_details: string
  items: ItemFlow[]
  total_quantity_per_min: number
}

// Logistics payloads
export type ConveyorTier = 'Mk1' | 'Mk2' | 'Mk3' | 'Mk4' | 'Mk5' | 'Mk6'
export type PipelineTier = 'Mk1' | 'Mk2'
export type WagonCarType = 'Cargo' | 'Fluid'

export interface BusConveyorPayload {
  line_id?: string
  conveyor_type: ConveyorTier
  item: Item
  quantity_per_min: number
}

export interface BusPipelinePayload {
  pipeline_id?: string
  pipeline_type: PipelineTier
  item: Item
  quantity_per_min: number
}

export interface TrainWagonPayload {
  wagon_id?: string
  wagon_type: WagonCarType
  item: Item
  quantity_per_min: number
}

export type CreateLogisticsRequest =
  | {
      from_factory: number
      to_factory: number
      transport_type: 'Truck'
      item: Item
      quantity_per_min: number
      truck_id?: string
    }
  | {
      from_factory: number
      to_factory: number
      transport_type: 'Drone'
      item: Item
      quantity_per_min: number
      drone_id?: string
    }
  | {
      from_factory: number
      to_factory: number
      transport_type: 'Bus'
      bus_name?: string
      conveyors: BusConveyorPayload[]
      pipelines: BusPipelinePayload[]
    }
  | {
      from_factory: number
      to_factory: number
      transport_type: 'Train'
      train_name?: string
      wagons: TrainWagonPayload[]
    }

export interface ItemFlow {
  item: string
  quantity_per_min: number
}

// Dashboard types
export interface DashboardSummary {
  total_factories: number
  total_production_lines: number
  total_logistics_lines: number
  total_power_consumption: number
  total_power_generation: number
  net_power: number
}

export interface ItemBalance {
  item: string
  balance: number
  state: 'overflow' | 'underflow' | 'balanced'
}

// Game Data types
export interface Recipe {
  name: string
  machine: string
  inputs: ItemQuantity[]
  outputs: ItemQuantity[]
}

export interface ItemQuantity {
  item: string
  quantity: number
}

export interface Machine {
  name: string
  base_power: number
  max_sommersloop: number
}

// API Response types
export interface ApiResponse<T> {
  data: T
  message?: string
}

export interface ApiError {
  error: string
  status: number
}
```

### 3. API Endpoints (api/endpoints.ts)

Create typed API endpoint functions:

```typescript
// frontend/src/api/endpoints.ts
import { apiClient } from './client'
import type {
  Factory,
  CreateFactoryRequest,
  UpdateFactoryRequest,
  ProductionLine,
  CreateProductionLineRequest,
  RawInput,
  CreateRawInputRequest,
  PowerGenerator,
  CreatePowerGeneratorRequest,
  Logistics,
  CreateLogisticsRequest,
  DashboardSummary,
  ItemBalance,
  Recipe,
  Machine
} from './types'

// Factory endpoints
export const factoryApi = {
  // Get all factories
  getFactories: (): Promise<Factory[]> => 
    apiClient.get('/factories'),
  
  // Get factory by ID
  getFactory: (id: number): Promise<Factory> => 
    apiClient.get(`/factories/${id}`),
  
  // Create factory
  createFactory: (data: CreateFactoryRequest): Promise<Factory> => 
    apiClient.post('/factories', data),
  
  // Update factory
  updateFactory: (id: number, data: UpdateFactoryRequest): Promise<Factory> => 
    apiClient.put(`/factories/${id}`, data),
  
  // Delete factory
  deleteFactory: (id: number): Promise<void> => 
    apiClient.delete(`/factories/${id}`),
  
  // Production lines
  createProductionLine: (factoryId: number, data: CreateProductionLineRequest): Promise<ProductionLine> => 
    apiClient.post(`/factories/${factoryId}/production-lines`, data),
  
  // Raw inputs
  createRawInput: (factoryId: number, data: CreateRawInputRequest): Promise<RawInput> => 
    apiClient.post(`/factories/${factoryId}/raw-inputs`, data),
  
  // Power generators
  createPowerGenerator: (factoryId: number, data: CreatePowerGeneratorRequest): Promise<PowerGenerator> => 
    apiClient.post(`/factories/${factoryId}/power-generators`, data),
}

// Logistics endpoints
export const logisticsApi = {
  // Get all logistics
  getLogistics: (): Promise<Logistics[]> => 
    apiClient.get('/logistics'),
  
  // Get logistics by ID
  getLogisticsLine: (id: number): Promise<Logistics> => 
    apiClient.get(`/logistics/${id}`),
  
  // Create logistics
  createLogistics: (data: CreateLogisticsRequest): Promise<Logistics> => 
    apiClient.post('/logistics', data),
  
  // Delete logistics
  deleteLogistics: (id: number): Promise<void> => 
    apiClient.delete(`/logistics/${id}`),
}

// Example: create a bus logistics line
await logisticsApi.createLogistics({
  from_factory: 1,
  to_factory: 3,
  transport_type: 'Bus',
  bus_name: 'Main Conveyor Bus',
  conveyors: [
    {
      line_id: 'CV-001',
      conveyor_type: 'Mk3',
      item: 'IronPlate',
      quantity_per_min: 120
    }
  ],
  pipelines: [
    {
      pipeline_id: 'PL-001',
      pipeline_type: 'Mk1',
      item: 'Water',
      quantity_per_min: 240
    }
  ]
})

// Dashboard endpoints
export const dashboardApi = {
  // Get summary
  getSummary: (): Promise<DashboardSummary> => 
    apiClient.get('/dashboard/summary'),
  
  // Get item balances
  getItemBalances: (): Promise<ItemBalance[]> => 
    apiClient.get('/dashboard/items'),
  
  // Get power statistics
  getPowerStatistics: (): Promise<Record<string, number>> => 
    apiClient.get('/dashboard/power'),
}

// Game data endpoints
export const gameDataApi = {
  // Get recipes
  getRecipes: (): Promise<Recipe[]> => 
    apiClient.get('/game-data/recipes'),
  
  // Get items
  getItems: (): Promise<string[]> => 
    apiClient.get('/game-data/items'),
  
  // Get machines
  getMachines: (): Promise<Machine[]> => 
    apiClient.get('/game-data/machines'),
}
```

### 4. Router Configuration (router/index.ts)

Set up Vue Router:

```typescript
// frontend/src/router/index.ts
import { createRouter, createWebHistory } from 'vue-router'
import DashboardView from '@/views/DashboardView.vue'
import FactoryView from '@/views/FactoryView.vue'
import LogisticsView from '@/views/LogisticsView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: DashboardView,
      meta: { title: 'Dashboard' }
    },
    {
      path: '/factory',
      name: 'factory',
      component: FactoryView,
      meta: { title: 'Factory' }
    },
    {
      path: '/logistics',
      name: 'logistics',
      component: LogisticsView,
      meta: { title: 'Logistics' }
    }
  ]
})

// Update page title
router.beforeEach((to, from, next) => {
  document.title = `Satisflow - ${to.meta.title || 'Dashboard'}`
  next()
})

export default router
```

### 5. Main Application (main.ts)

Set up the Vue application:

```typescript
// frontend/src/main.ts
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'

// Import styles
import './styles/main.css'

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')
```

### 6. Root Component (App.vue)

Create the main application component:

```vue
<!-- frontend/src/App.vue -->
<template>
  <div id="app" class="min-h-screen bg-gray-50">
    <MainNav />
    <main class="container mx-auto px-4 py-8">
      <RouterView />
    </main>
  </div>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router'
import MainNav from '@/components/layout/MainNav.vue'
</script>

<style>
#app {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>
```

### 7. Navigation Component (components/layout/MainNav.vue)

Create the navigation bar:

```vue
<!-- frontend/src/components/layout/MainNav.vue -->
<template>
  <nav class="bg-white shadow-lg">
    <div class="container mx-auto px-4">
      <div class="flex justify-between items-center h-16">
        <div class="flex items-center">
          <h1 class="text-xl font-bold text-gray-800">Satisflow</h1>
        </div>
        
        <div class="flex space-x-8">
          <RouterLink
            v-for="route in routes"
            :key="route.name"
            :to="{ name: route.name }"
            class="px-3 py-2 rounded-md text-sm font-medium transition-colors"
            :class="[
              $route.name === route.name
                ? 'bg-primary-500 text-white'
                : 'text-gray-700 hover:bg-gray-100'
            ]"
          >
            {{ route.label }}
          </RouterLink>
        </div>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { RouterLink, useRoute } from 'vue-router'

const $route = useRoute()

const routes = [
  { name: 'dashboard', label: 'Dashboard' },
  { name: 'factory', label: 'Factory' },
  { name: 'logistics', label: 'Logistics' }
]
</script>
```

### 8. Dashboard View (views/DashboardView.vue)

Implement the dashboard view:

```vue
<!-- frontend/src/views/DashboardView.vue -->
<template>
  <div>
    <PageHeader title="Dashboard" subtitle="Global production overview" />
    
    <!-- Summary Cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      <SummaryCard
        title="Factories"
        :value="summary?.total_factories || 0"
        icon="building"
        color="blue"
      />
      <SummaryCard
        title="Production Lines"
        :value="summary?.total_production_lines || 0"
        icon="cog"
        color="green"
      />
      <SummaryCard
        title="Logistics Lines"
        :value="summary?.total_logistics_lines || 0"
        icon="truck"
        color="purple"
      />
      <SummaryCard
        title="Net Power"
        :value="formatPower(summary?.net_power || 0)"
        icon="bolt"
        :color="powerColor"
      />
    </div>

    <!-- Item Balances -->
    <div class="bg-white rounded-lg shadow p-6">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-lg font-semibold text-gray-900">Item Balances</h2>
        <div class="flex space-x-2">
          <FilterButton
            v-for="filter in balanceFilters"
            :key="filter.value"
            :label="filter.label"
            :active="activeFilter === filter.value"
            @click="activeFilter = filter.value"
          />
        </div>
      </div>
      
      <div v-if="loading" class="flex justify-center py-8">
        <LoadingSpinner />
      </div>
      
      <div v-else-if="filteredItems.length === 0" class="text-center py-8 text-gray-500">
        No items found for the selected filter
      </div>
      
      <div v-else class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Item
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Balance
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                State
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="item in filteredItems" :key="item.item">
              <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                {{ item.item }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                {{ formatNumber(item.balance) }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <span
                  class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full"
                  :class="getStateClass(item.state)"
                >
                  {{ item.state }}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { dashboardApi } from '@/api/endpoints'
import type { DashboardSummary, ItemBalance } from '@/api/types'
import PageHeader from '@/components/layout/PageHeader.vue'
import SummaryCard from '@/components/ui/SummaryCard.vue'
import FilterButton from '@/components/ui/FilterButton.vue'
import LoadingSpinner from '@/components/ui/LoadingSpinner.vue'

// State
const summary = ref<DashboardSummary | null>(null)
const itemBalances = ref<ItemBalance[]>([])
const loading = ref(true)
const activeFilter = ref<string>('all')

// Filters
const balanceFilters = [
  { value: 'all', label: 'All' },
  { value: 'overflow', label: 'Overflow' },
  { value: 'underflow', label: 'Underflow' },
  { value: 'balanced', label: 'Balanced' }
]

// Computed
const filteredItems = computed(() => {
  if (activeFilter.value === 'all') {
    return itemBalances.value
  }
  return itemBalances.value.filter(item => item.state === activeFilter.value)
})

const powerColor = computed(() => {
  if (!summary.value) return 'gray'
  return summary.value.net_power >= 0 ? 'green' : 'red'
})

// Methods
const formatNumber = (num: number): string => {
  return num.toFixed(2)
}

const formatPower = (power: number): string => {
  return `${power.toFixed(1)} MW`
}

const getStateClass = (state: string): string => {
  switch (state) {
    case 'overflow':
      return 'bg-green-100 text-green-800'
    case 'underflow':
      return 'bg-red-100 text-red-800'
    case 'balanced':
      return 'bg-blue-100 text-blue-800'
    default:
      return 'bg-gray-100 text-gray-800'
  }
}

const loadData = async () => {
  try {
    loading.value = true
    const [summaryData, itemsData] = await Promise.all([
      dashboardApi.getSummary(),
      dashboardApi.getItemBalances()
    ])
    
    summary.value = summaryData
    itemBalances.value = itemsData
  } catch (error) {
    console.error('Failed to load dashboard data:', error)
  } finally {
    loading.value = false
  }
}

// Lifecycle
onMounted(() => {
  loadData()
  
  // Set up polling for real-time updates
  const interval = setInterval(loadData, 5000)
  
  // Cleanup on unmount
  onUnmounted(() => {
    clearInterval(interval)
  })
})
</script>
```

### 9. Factory View (views/FactoryView.vue)

Implement the factory management view:

```vue
<!-- frontend/src/views/FactoryView.vue -->
<template>
  <div>
    <PageHeader 
      title="Factory Management" 
      subtitle="Manage your factories and production"
    >
      <template #actions>
        <Button @click="showCreateFactoryModal = true">
          Create Factory
        </Button>
      </template>
    </PageHeader>

    <!-- Factory Selector -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
      <label class="block text-sm font-medium text-gray-700 mb-2">
        Select Factory
      </label>
      <select 
        v-model="selectedFactoryId" 
        class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
      >
        <option value="">Select a factory...</option>
        <option v-for="factory in factories" :key="factory.id" :value="factory.id">
          {{ factory.name }}
        </option>
      </select>
    </div>

    <!-- Factory Details -->
    <div v-if="selectedFactory">
      <Tabs>
        <Tab title="Production Lines">
          <ProductionLinesTab :factory="selectedFactory" @refresh="loadFactories" />
        </Tab>
        <Tab title="Raw Inputs">
          <RawInputsTab :factory="selectedFactory" @refresh="loadFactories" />
        </Tab>
        <Tab title="Power Generation">
          <PowerGenerationTab :factory="selectedFactory" @refresh="loadFactories" />
        </Tab>
      </Tabs>
    </div>

    <!-- Create Factory Modal -->
    <Modal 
      v-if="showCreateFactoryModal" 
      @close="showCreateFactoryModal = false"
      title="Create New Factory"
    >
      <FactoryForm 
        @submit="handleCreateFactory" 
        @cancel="showCreateFactoryModal = false" 
      />
    </Modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { factoryApi } from '@/api/endpoints'
import type { Factory } from '@/api/types'
import PageHeader from '@/components/layout/PageHeader.vue'
import Button from '@/components/ui/Button.vue'
import Modal from '@/components/ui/Modal.vue'
import FactoryForm from '@/components/factory/FactoryForm.vue'
import ProductionLinesTab from '@/components/factory/ProductionLinesTab.vue'
import RawInputsTab from '@/components/factory/RawInputsTab.vue'
import PowerGenerationTab from '@/components/factory/PowerGenerationTab.vue'
import Tabs from '@/components/ui/Tabs.vue'
import Tab from '@/components/ui/Tab.vue'

// State
const factories = ref<Factory[]>([])
const selectedFactoryId = ref<number | null>(null)
const showCreateFactoryModal = ref(false)
const loading = ref(false)

// Computed
const selectedFactory = computed(() => {
  if (!selectedFactoryId.value) return null
  return factories.value.find(f => f.id === selectedFactoryId.value) || null
})

// Methods
const loadFactories = async () => {
  try {
    loading.value = true
    factories.value = await factoryApi.getFactories()
  } catch (error) {
    console.error('Failed to load factories:', error)
  } finally {
    loading.value = false
  }
}

const handleCreateFactory = async (factoryData: any) => {
  try {
    await factoryApi.createFactory(factoryData)
    showCreateFactoryModal.value = false
    await loadFactories()
  } catch (error) {
    console.error('Failed to create factory:', error)
  }
}

// Lifecycle
onMounted(() => {
  loadFactories()
})
</script>
```

### 10. Composables for State Management (composables/useFactories.ts)

Create reusable state management:

```typescript
// frontend/src/composables/useFactories.ts
import { ref, computed } from 'vue'
import { factoryApi } from '@/api/endpoints'
import type { Factory, CreateFactoryRequest, UpdateFactoryRequest } from '@/api/types'

export function useFactories() {
  const factories = ref<Factory[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const loadFactories = async () => {
    try {
      loading.value = true
      error.value = null
      factories.value = await factoryApi.getFactories()
    } catch (err) {
      error.value = 'Failed to load factories'
      console.error('Error loading factories:', err)
    } finally {
      loading.value = false
    }
  }

  const getFactory = async (id: number): Promise<Factory | null> => {
    try {
      return await factoryApi.getFactory(id)
    } catch (err) {
      error.value = 'Failed to load factory'
      console.error('Error loading factory:', err)
      return null
    }
  }

  const createFactory = async (data: CreateFactoryRequest): Promise<Factory | null> => {
    try {
      const newFactory = await factoryApi.createFactory(data)
      factories.value.push(newFactory)
      return newFactory
    } catch (err) {
      error.value = 'Failed to create factory'
      console.error('Error creating factory:', err)
      return null
    }
  }

  const updateFactory = async (id: number, data: UpdateFactoryRequest): Promise<Factory | null> => {
    try {
      const updatedFactory = await factoryApi.updateFactory(id, data)
      const index = factories.value.findIndex(f => f.id === id)
      if (index !== -1) {
        factories.value[index] = updatedFactory
      }
      return updatedFactory
    } catch (err) {
      error.value = 'Failed to update factory'
      console.error('Error updating factory:', err)
      return null
    }
  }

  const deleteFactory = async (id: number): Promise<boolean> => {
    try {
      await factoryApi.deleteFactory(id)
      factories.value = factories.value.filter(f => f.id !== id)
      return true
    } catch (err) {
      error.value = 'Failed to delete factory'
      console.error('Error deleting factory:', err)
      return false
    }
  }

  return {
    factories,
    loading,
    error,
    loadFactories,
    getFactory,
    createFactory,
    updateFactory,
    deleteFactory
  }
}
```

### 11. Reusable UI Components

Create reusable UI components:

```vue
<!-- frontend/src/components/ui/Button.vue -->
<template>
  <button
    :type="type"
    :disabled="disabled || loading"
    :class="buttonClasses"
    @click="handleClick"
  >
    <LoadingSpinner v-if="loading" class="mr-2" />
    <slot />
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import LoadingSpinner from './LoadingSpinner.vue'

interface Props {
  type?: 'button' | 'submit' | 'reset'
  variant?: 'primary' | 'secondary' | 'danger'
  size?: 'sm' | 'md' | 'lg'
  disabled?: boolean
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  type: 'button',
  variant: 'primary',
  size: 'md',
  disabled: false,
  loading: false
})

const emit = defineEmits<{
  click: [event: MouseEvent]
}>()

const buttonClasses = computed(() => {
  const baseClasses = 'inline-flex items-center justify-center font-medium rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2 transition-colors'
  
  const sizeClasses = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-sm',
    lg: 'px-6 py-3 text-base'
  }
  
  const variantClasses = {
    primary: 'bg-primary-600 text-white hover:bg-primary-700 focus:ring-primary-500',
    secondary: 'bg-gray-200 text-gray-900 hover:bg-gray-300 focus:ring-gray-500',
    danger: 'bg-red-600 text-white hover:bg-red-700 focus:ring-red-500'
  }
  
  const disabledClasses = props.disabled || props.loading
    ? 'opacity-50 cursor-not-allowed'
    : ''
  
  return [
    baseClasses,
    sizeClasses[props.size],
    variantClasses[props.variant],
    disabledClasses
  ].join(' ')
})

const handleClick = (event: MouseEvent) => {
  if (!props.disabled && !props.loading) {
    emit('click', event)
  }
}
</script>
```

```vue
<!-- frontend/src/components/ui/LoadingSpinner.vue -->
<template>
  <div class="animate-spin rounded-full border-2 border-gray-300 border-t-primary-600" :class="sizeClasses"></div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  size?: 'sm' | 'md' | 'lg'
}

const props = withDefaults(defineProps<Props>(), {
  size: 'md'
})

const sizeClasses = computed(() => {
  const sizes = {
    sm: 'h-4 w-4',
    md: 'h-6 w-6',
    lg: 'h-8 w-8'
  }
  return sizes[props.size]
})
</script>
```

### 12. Global Styles (styles/main.css)

Set up global styles:

```css
/* frontend/src/styles/main.css */
@import 'tailwindcss/base';
@import 'tailwindcss/components';
@import 'tailwindcss/utilities';

/* Custom styles */
.container {
  max-width: 1200px;
}

/* Form styles */
.form-group {
  @apply mb-4;
}

.form-label {
  @apply block text-sm font-medium text-gray-700 mb-1;
}

.form-input {
  @apply block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500;
}

.form-error {
  @apply mt-1 text-sm text-red-600;
}

/* Table styles */
.table-container {
  @apply overflow-x-auto shadow ring-1 ring-black ring-opacity-5 md:rounded-lg;
}

.table {
  @apply min-w-full divide-y divide-gray-300;
}

.table-header {
  @apply bg-gray-50;
}

.table-header-cell {
  @apply px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider;
}

.table-body {
  @apply bg-white divide-y divide-gray-200;
}

.table-row {
  @apply hover:bg-gray-50;
}

.table-cell {
  @apply px-6 py-4 whitespace-nowrap text-sm text-gray-900;
}

/* Animation classes */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.slide-up-enter-active,
.slide-up-leave-active {
  transition: all 0.3s ease;
}

.slide-up-enter-from {
  transform: translateY(10px);
  opacity: 0;
}

.slide-up-leave-to {
  transform: translateY(-10px);
  opacity: 0;
}
```

## Development Workflow

### 1. Development Server

Start the development server:

```bash
cd frontend
npm run dev
```

The server will start at `http://localhost:5173` with API proxy to the backend.

### 2. Type Checking

Run TypeScript type checking:

```bash
npm run type-check
```

### 3. Testing

Run unit tests:

```bash
npm run test
```

Run tests in watch mode:

```bash
npm run test:watch
```

### 4. Building for Production

Build the application for production:

```bash
npm run build
```

Preview the production build:

```bash
npm run preview
```

### 5. Linting and Formatting

Check for linting issues:

```bash
npm run lint
```

Fix linting issues:

```bash
npm run lint:fix
```

Format code:

```bash
npm run format
```

## Production Build Instructions

### 1. Environment Configuration

Create a `.env.production` file:

```env
VITE_API_BASE_URL=https://api.yourdomain.com
VITE_APP_TITLE=Satisflow
```

### 2. Build Configuration

Update `vite.config.ts` for production:

```typescript
export default defineConfig({
  // ... existing config
  build: {
    outDir: 'dist',
    sourcemap: false,
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['vue', 'vue-router', 'axios'],
          ui: ['@headlessui/vue', '@heroicons/vue']
        }
      }
    }
  }
})
```

### 3. Deployment

Deploy the built files to a web server. The `dist` directory contains all the static files.

#### Example Nginx Configuration:

```nginx
server {
    listen 80;
    server_name yourdomain.com;
    root /var/www/satisflow/dist;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    location /api {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Best Practices

### 1. Component Organization

- Keep components focused on a single responsibility
- Use composition API with `<script setup>` syntax
- Extract reusable logic into composables
- Use TypeScript interfaces for props and emits

### 2. State Management

- Use local state for component-specific data
- Create composables for shared state
- Keep API calls separate from components
- Handle loading and error states consistently

### 3. Performance Optimization

- Use `v-memo` for expensive computations
- Implement virtual scrolling for large lists
- Lazy load components with `defineAsyncComponent`
- Optimize bundle size with code splitting

### 4. Accessibility

- Use semantic HTML elements
- Provide proper ARIA labels
- Ensure keyboard navigation
- Test with screen readers

### 5. Error Handling

- Implement global error handlers
- Show user-friendly error messages
- Log errors for debugging
- Provide recovery options

## Next Steps

1. **Complete component implementations** for all views
2. **Add comprehensive testing** for components and API calls
3. **Implement advanced features** like real-time updates
4. **Add user preferences** and local storage
5. **Optimize for mobile** devices
6. **Add internationalization** support

This frontend implementation provides a solid foundation for the Satisflow web application, with proper TypeScript support, component organization, and development workflow.
