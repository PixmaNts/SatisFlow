# Satisfflow Vue.js UI Implementation Plan

## Executive Summary

Based on analysis of the backend API and engine models, this comprehensive implementation plan covers:

1. **Architecture Overview**: Component structure, data flow, and technology stack
2. **API Integration Layer**: TypeScript types, Axios client, and endpoint functions
3. **Component Hierarchy**: Reusable UI components, layouts, and views
4. **Feature Implementation**: Detailed breakdown of Dashboard, Factory, and Logistics views
5. **State Management**: Pinia stores and composables
6. **Validation & Error Handling**: Form validation, error boundaries, and user feedback
7. **Testing Strategy**: Vitest unit tests and Playwright E2E tests

## Architecture Overview

```mermaid
graph TB
    subgraph "Vue.js Application Layer"
        Router[Vue Router]
        Pinia[Pinia Store]
        
        subgraph "Views"
            Dashboard[Dashboard View]
            Factory[Factory View]
            Logistics[Logistics View]
        end
        
        subgraph "Components"
            Layout[Layout Components]
            UI[Reusable UI]
            Forms[Form Components]
        end
        
        subgraph "Composables"
            UseFactory[useFactory]
            UseLogistics[useLogistics]
            UseDashboard[useDashboard]
            UseGameData[useGameData]
        end
    end
    
    subgraph "API Layer"
        APIClient[Axios Client]
        Types[TypeScript Types]
        Endpoints[API Endpoints]
    end
    
    subgraph "Backend REST API"
        FactoryAPI[Factory Handlers]
        LogisticsAPI[Logistics Handlers]
        DashboardAPI[Dashboard Handlers]
        GameDataAPI[Game Data Handlers]
    end
    
    Router --> Views
    Views --> Components
    Views --> Composables
    Composables --> Pinia
    Composables --> APIClient
    APIClient --> Endpoints
    Endpoints --> Types
    Endpoints --> Backend REST API
```

## 1. Foundation Layer

### 1.1 Vite Configuration

**File**: [`frontend/vite.config.ts`](../frontend/vite.config.ts)

```typescript
import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

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
        changeOrigin: true,
        secure: false
      }
    }
  }
})
```

**Key Features**:
- API proxy to backend at `localhost:3000`
- Path alias `@/` for clean imports
- Default dev server port `5173`

### 1.2 TypeScript Type Definitions

**File**: `frontend/src/api/types.ts`

**Structure**:
```typescript
// ===== Core Engine Types =====
export type Item = 'IronOre' | 'CopperOre' | 'Coal' | /* ...150+ items */

export type Purity = 'Impure' | 'Normal' | 'Pure'

export type ExtractorType = 
  | 'MinerMk1' | 'MinerMk2' | 'MinerMk3'
  | 'WaterExtractor' | 'OilExtractor' | 'ResourceWellExtractor'

export type GeneratorType = 
  | 'Biomass' | 'Coal' | 'Fuel' | 'Nuclear' | 'Geothermal'

export type MachineType = 
  | 'Smelter' | 'Constructor' | 'Assembler' | 'Manufacturer' | /* etc */

export type ConveyorSpeed = 'Mk1' | 'Mk2' | 'Mk3' | 'Mk4' | 'Mk5' | 'Mk6'
export type PipelineCapacity = 'Mk1' | 'Mk2'
export type WagonType = 'Cargo' | 'Fluid'

// ===== Factory Types =====
export interface Factory {
  id: number
  name: string
  description?: string
  notes?: string
  production_lines: ProductionLine[]
  raw_inputs: RawInput[]
  power_generators: PowerGenerator[]
  items: Record<Item, number> // Calculated inventory
}

// ===== Production Line Types =====
export interface MachineGroup {
  number_of_machine: number
  oc_value: number // 0.000 - 250.000
  somersloop: number // 0, 1, 2, 4
}

export interface ProductionLineRecipe {
  id: number
  name: string
  description?: string
  recipe: string // Recipe enum name
  machine_groups: MachineGroup[]
}

export interface ProductionLineBlueprint {
  id: number
  name: string
  description?: string
  nested_lines: ProductionLineRecipe[]
}

export type ProductionLine = 
  | { type: 'Recipe'; data: ProductionLineRecipe }
  | { type: 'Blueprint'; data: ProductionLineBlueprint }

// ===== Raw Input Types =====
export interface ResourceWellPressurizer {
  id: number
  clock_speed: number // 0.000 - 250.000
}

export interface ResourceWellExtractor {
  id: number
  purity: Purity
}

export interface RawInput {
  id: number
  extractor_type: ExtractorType
  item: Item
  purity?: Purity
  quantity_per_min: number
  pressurizer?: ResourceWellPressurizer
  extractors: ResourceWellExtractor[]
}

// ===== Power Generator Types =====
export interface GeneratorGroup {
  number_of_generators: number
  clock_speed: number // 0.000 - 250.000
}

export interface PowerGenerator {
  id: number
  generator_type: GeneratorType
  fuel_type: Item
  groups: GeneratorGroup[]
}

// ===== Logistics Types =====
export interface Conveyor {
  line_id: number
  speed: ConveyorSpeed
  item: Item
  quantity_per_min: number
}

export interface Pipeline {
  pipeline_id: number
  capacity: PipelineCapacity
  item: Item
  quantity_per_min: number
}

export interface Bus {
  bus_id: number
  bus_name: string
  lines: Conveyor[]
  pipelines: Pipeline[]
}

export interface Wagon {
  wagon_id: number
  wagon_type: WagonType
  item: Item
  quantity_per_min: number
}

export interface Train {
  train_id: number
  train_name: string
  wagons: Wagon[]
}

export interface TruckTransport {
  truck_id: number
  item: Item
  quantity_per_min: number
}

export interface DroneTransport {
  drone_id: number
  item: Item
  quantity_per_min: number
}

export type TransportType =
  | { type: 'Bus'; data: Bus }
  | { type: 'Train'; data: Train }
  | { type: 'Truck'; data: TruckTransport }
  | { type: 'Drone'; data: DroneTransport }

export interface LogisticsFlux {
  id: number
  from_factory: number
  to_factory: number
  transport_type: TransportType
  transport_details: string
}

// ===== API Request/Response Types =====
export interface CreateFactoryRequest {
  name: string
  description?: string
}

export interface UpdateFactoryRequest {
  name?: string
  description?: string
}

export interface CreateLogisticsRequest {
  from_factory: number
  to_factory: number
  transport_type: string
  transport_details: string
}

// ===== Dashboard Types =====
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
  state: 'Overflow' | 'Underflow' | 'Balanced'
}

// ===== Game Data Types =====
export interface RecipeInfo {
  name: string
  machine: MachineType
  inputs: Array<{ item: Item; quantity: number }>
  outputs: Array<{ item: Item; quantity: number }>
}

export interface ItemInfo {
  name: string
  display_name: string
}

export interface MachineInfo {
  type: MachineType
  base_power: number
  max_sommersloop: number
}
```

**Total Size**: ~500 lines covering all backend DTOs and engine models

### 1.3 Axios API Client

**File**: `frontend/src/api/client.ts`

```typescript
import axios, { AxiosInstance, AxiosError } from 'axios'
import type { APIError } from './types'

class APIClient {
  private client: AxiosInstance

  constructor() {
    this.client = axios.create({
      baseURL: '/api',
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json'
      }
    })

    // Request interceptor
    this.client.interceptors.request.use(
      (config) => {
        // Add auth token if needed
        return config
      },
      (error) => Promise.reject(error)
    )

    // Response interceptor
    this.client.interceptors.response.use(
      (response) => response,
      (error: AxiosError<APIError>) => {
        // Global error handling
        console.error('API Error:', error.response?.data)
        return Promise.reject(error)
      }
    )
  }

  get<T>(url: string, params?: any) {
    return this.client.get<T>(url, { params })
  }

  post<T>(url: string, data?: any) {
    return this.client.post<T>(url, data)
  }

  put<T>(url: string, data?: any) {
    return this.client.put<T>(url, data)
  }

  delete<T>(url: string) {
    return this.client.delete<T>(url)
  }
}

export const apiClient = new APIClient()
```

### 1.4 API Endpoint Functions

**File**: `frontend/src/api/endpoints.ts`

```typescript
import { apiClient } from './client'
import type {
  Factory, CreateFactoryRequest, UpdateFactoryRequest,
  LogisticsFlux, CreateLogisticsRequest,
  DashboardSummary, ItemBalance,
  RecipeInfo, ItemInfo, MachineInfo
} from './types'

// ===== Factory Endpoints =====
export const factoryAPI = {
  getAll: () => apiClient.get<Factory[]>('/factories'),
  getById: (id: number) => apiClient.get<Factory>(`/factories/${id}`),
  create: (data: CreateFactoryRequest) => 
    apiClient.post<Factory>('/factories', data),
  update: (id: number, data: UpdateFactoryRequest) =>
    apiClient.put<Factory>(`/factories/${id}`, data),
  delete: (id: number) => apiClient.delete(`/factories/${id}`)
}

// ===== Logistics Endpoints =====
export const logisticsAPI = {
  getAll: () => apiClient.get<LogisticsFlux[]>('/logistics'),
  getById: (id: number) => apiClient.get<LogisticsFlux>(`/logistics/${id}`),
  create: (data: CreateLogisticsRequest) =>
    apiClient.post<LogisticsFlux>('/logistics', data),
  delete: (id: number) => apiClient.delete(`/logistics/${id}`)
}

// ===== Dashboard Endpoints =====
export const dashboardAPI = {
  getSummary: () => apiClient.get<DashboardSummary>('/dashboard/summary'),
  getItemBalances: () => apiClient.get<ItemBalance[]>('/dashboard/items'),
  getPowerStats: () => apiClient.get<any>('/dashboard/power')
}

// ===== Game Data Endpoints =====
export const gameDataAPI = {
  getRecipes: () => apiClient.get<RecipeInfo[]>('/game-data/recipes'),
  getItems: () => apiClient.get<ItemInfo[]>('/game-data/items'),
  getMachines: () => apiClient.get<MachineInfo[]>('/game-data/machines')
}
```

## 2. Component Architecture

### 2.1 Component Hierarchy

```
src/
├── components/
│   ├── layout/
│   │   ├── MainNav.vue          # Top navigation bar
│   │   ├── PageHeader.vue       # Page title and actions
│   │   └── AppLayout.vue        # Main layout wrapper
│   │
│   ├── ui/
│   │   ├── Button.vue           # Reusable button component
│   │   ├── Modal.vue            # Modal dialog
│   │   ├── LoadingSpinner.vue   # Loading indicator
│   │   ├── Tabs.vue             # Tab navigation
│   │   ├── DataTable.vue        # Sortable/filterable table
│   │   ├── ConfirmDialog.vue    # Confirmation dialog
│   │   └── Alert.vue            # Alert/notification
│   │
│   ├── forms/
│   │   ├── FormInput.vue        # Text input with validation
│   │   ├── FormSelect.vue       # Dropdown select
│   │   ├── FormNumber.vue       # Number input (OC, Sommersloop)
│   │   ├── ItemSelector.vue     # Item type selector
│   │   ├── RecipeSelector.vue   # Recipe selector
│   │   └── ValidationMessage.vue # Error message display
│   │
│   ├── factory/
│   │   ├── FactorySelector.vue       # Factory dropdown
│   │   ├── ProductionLineList.vue    # Production line table
│   │   ├── ProductionLineForm.vue    # Create/edit production line
│   │   ├── MachineGroupEditor.vue    # Machine group configuration
│   │   ├── RawInputList.vue          # Raw input table
│   │   ├── RawInputForm.vue          # Create/edit raw input
│   │   ├── PowerGeneratorList.vue    # Power generator table
│   │   └── PowerGeneratorForm.vue    # Create/edit generator
│   │
│   ├── logistics/
│   │   ├── LogisticsLineList.vue    # Logistics table
│   │   ├── LogisticsLineForm.vue    # Create/edit logistics
│   │   ├── TransportSelector.vue    # Transport type selector
│   │   ├── BusEditor.vue            # Bus configuration
│   │   └── TrainEditor.vue          # Train configuration
│   │
│   └── dashboard/
│       ├── SummaryCards.vue         # Summary statistics
│       ├── ItemBalanceTable.vue     # Item balance table
│       ├── PowerStatsCard.vue       # Power statistics
│       └── ItemBalanceFilters.vue   # Filter controls
│
├── views/
│   ├── DashboardView.vue         # Dashboard main view
│   ├── FactoryView.vue           # Factory main view
│   └── LogisticsView.vue         # Logistics main view
│
├── composables/
│   ├── useFactory.ts             # Factory CRUD operations
│   ├── useLogistics.ts           # Logistics CRUD operations
│   ├── useDashboard.ts           # Dashboard data fetching
│   ├── useGameData.ts            # Game data (recipes, items)
│   ├── useValidation.ts          # Form validation logic
│   └── useLocalStorage.ts        # Local storage persistence
│
└── stores/
    ├── factory.ts                # Pinia store for factories
    ├── logistics.ts              # Pinia store for logistics
    ├── gameData.ts               # Pinia store for game data
    └── preferences.ts            # Pinia store for user prefs
```

### 2.2 Key Component Specifications

#### MainNav.vue
```vue
<template>
  <nav class="main-nav">
    <router-link to="/">Dashboard</router-link>
    <router-link to="/factory">Factory</router-link>
    <router-link to="/logistics">Logistics</router-link>
  </nav>
</template>
```

#### DataTable.vue
- Props: `columns`, `data`, `sortable`, `filterable`
- Features: Column sorting, row filtering, pagination
- Emits: `row-click`, `sort-change`

#### FactorySelector.vue
```vue
<template>
  <select v-model="selectedFactoryId" @change="handleChange">
    <option value="">Select Factory...</option>
    <option v-for="factory in factories" :key="factory.id" :value="factory.id">
      {{ factory.name }}
    </option>
  </select>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useFactoryStore } from '@/stores/factory'

const store = useFactoryStore()
const factories = computed(() => store.factories)

const selectedFactoryId = defineModel<number>()
</script>
```

## 3. View Implementation Details

### 3.1 Dashboard View

**Route**: `/`

**Components Used**:
- [`SummaryCards.vue`](../frontend/src/components/dashboard/SummaryCards.vue) - 6 cards (factories, production lines, logistics lines, power consumption, generation, net)
- [`ItemBalanceTable.vue`](../frontend/src/components/dashboard/ItemBalanceTable.vue) - Sortable/filterable table with columns: Item, Balance, State, Actions
- [`ItemBalanceFilters.vue`](../frontend/src/components/dashboard/ItemBalanceFilters.vue) - Filters: Balance state (Overflow/Underflow/Balanced), Factory, Item type

**Data Flow**:
```typescript
// useDashboard.ts
export function useDashboard() {
  const summary = ref<DashboardSummary | null>(null)
  const itemBalances = ref<ItemBalance[]>([])
  const loading = ref(false)
  const error = ref<Error | null>(null)

  async function fetchSummary() {
    loading.value = true
    try {
      const { data } = await dashboardAPI.getSummary()
      summary.value = data
    } catch (e) {
      error.value = e as Error
    } finally {
      loading.value = false
    }
  }

  async function fetchItemBalances() {
    const { data } = await dashboardAPI.getItemBalances()
    itemBalances.value = data
  }

  return { summary, itemBalances, loading, error, fetchSummary, fetchItemBalances }
}
```

### 3.2 Factory View

**Route**: `/factory`

**Layout**:
```
+------------------------------------------+
| FactorySelector [Dropdown]               |
+------------------------------------------+
| Tabs: [Production Lines] [Raw Inputs]   |
|       [Power Generation]                 |
+------------------------------------------+
| Tab Content (dynamic based on selection) |
+------------------------------------------+
```

**Tab 1: Production Lines**
- **Table Columns**: ID, Name, Recipe/Blueprint, Machines, OC Range, Power, Actions
- **Actions**: Edit, Delete, Duplicate
- **Create Button**: Opens modal with choice: "Standard Recipe" or "Blueprint"

**Production Line Form (Modal)**:
```
+-------------------------------------------+
| Create Production Line                     |
+-------------------------------------------+
| Type: (*) Recipe  ( ) Blueprint           |
|                                           |
| Recipe: [Dropdown]                        |
| Name: [Input]                             |
| Description: [Textarea]                   |
|                                           |
| Machine Groups:                           |
| +---------------------------------------+ |
| | Group 1                               | |
| | Machines: [10]  OC: [150.0%]          | |
| | Sommersloop: [2]  [Remove]            | |
| +---------------------------------------+ |
| [+ Add Group]                             |
|                                           |
| Power Consumption: 245.6 MW (calculated)  |
|                                           |
| [Cancel] [Create]                         |
+-------------------------------------------+
```

**Validation Rules**:
- OC: 0.000 - 250.000 (3 decimal places)
- Sommersloop: 0, 1, 2, 4 (based on machine type)
- At least 1 machine group required
- Machine count > 0

**Tab 2: Raw Inputs**
- **Table Columns**: ID, Extractor Type, Item, Purity, Quantity/min, Power, Actions
- **Create Button**: Opens raw input form

**Raw Input Form**:
```
+-------------------------------------------+
| Create Raw Input                          |
+-------------------------------------------+
| Extractor Type: [Dropdown]                |
| Item: [Dropdown] (filtered by extractor)  |
| Purity: [Dropdown] (if applicable)        |
|                                           |
| --- Resource Well (if selected) ---       |
| Pressurizer Clock: [100.0%]               |
| Extractors:                               |
| +---------------------------------------+ |
| | ID: 1  Purity: Pure  [Remove]         | |
| +---------------------------------------+ |
| [+ Add Extractor]                         |
|                                           |
| Quantity: 120.0 /min (calculated)         |
| Power: 150.0 MW (calculated)              |
|                                           |
| [Cancel] [Create]                         |
+-------------------------------------------+
```

**Tab 3: Power Generation**
- **Table Columns**: ID, Generator Type, Fuel, Groups, Power Output, Fuel Consumption, Actions
- **Special**: Nuclear shows waste production

**Power Generator Form**:
```
+-------------------------------------------+
| Create Power Generator                    |
+-------------------------------------------+
| Generator Type: [Dropdown]                |
| Fuel Type: [Dropdown] (filtered)          |
|                                           |
| Generator Groups:                         |
| +---------------------------------------+ |
| | Group 1                               | |
| | Generators: [5]  Clock: [100.0%]      | |
| | [Remove]                              | |
| +---------------------------------------+ |
| [+ Add Group]                             |
|                                           |
| Power Output: 375.0 MW (calculated)       |
| Fuel Consumption: 75.0 items/min          |
|                                           |
| [Cancel] [Create]                         |
+-------------------------------------------+
```

### 3.3 Logistics View

**Route**: `/logistics`

**Layout**:
```
+------------------------------------------+
| Filters: [Transport Type] [From] [To]    |
|          [Item]                          |
+------------------------------------------+
| Grouped by Transport Type                |
+------------------------------------------+
| ▼ Buses (3)                              |
|   Bus-1: Factory A -> Factory B          |
|   - Conveyor Mk5: Iron Ore (780/min)     |
|   - Pipeline Mk2: Water (450/min)        |
|                                          |
| ▼ Trains (2)                             |
|   Train-1: Iron Express (Factory A -> B) |
|   - Wagon 1: Iron Ore (500/min)          |
|   - Wagon 2: Coal (300/min)              |
+------------------------------------------+
| [+ Create Logistics Line]                |
+------------------------------------------+
```

**Create Logistics Form**:
```
+-------------------------------------------+
| Create Logistics Line                     |
+-------------------------------------------+
| Transport Type: (*) Bus ( ) Train         |
|                 ( ) Truck ( ) Drone       |
|                                           |
| From Factory: [Dropdown]                  |
| To Factory: [Dropdown]                    |
|                                           |
| --- Bus Configuration ---                 |
| Name: [Input]                             |
|                                           |
| Conveyors:                                |
| +---------------------------------------+ |
| | Mk5 | Iron Ore | 780 /min | [Remove] | |
| +---------------------------------------+ |
| [+ Add Conveyor]                          |
|                                           |
| Pipelines:                                |
| +---------------------------------------+ |
| | Mk2 | Water | 450 /min | [Remove]    | |
| +---------------------------------------+ |
| [+ Add Pipeline]                          |
|                                           |
| [Cancel] [Create]                         |
+-------------------------------------------+
```

## 4. State Management (Pinia)

### 4.1 Factory Store

```typescript
// stores/factory.ts
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { factoryAPI } from '@/api/endpoints'
import type { Factory } from '@/api/types'

export const useFactoryStore = defineStore('factory', () => {
  const factories = ref<Factory[]>([])
  const currentFactoryId = ref<number | null>(null)
  const loading = ref(false)
  const error = ref<Error | null>(null)

  const currentFactory = computed(() => 
    factories.value.find(f => f.id === currentFactoryId.value)
  )

  async function fetchAll() {
    loading.value = true
    try {
      const { data } = await factoryAPI.getAll()
      factories.value = data
    } catch (e) {
      error.value = e as Error
    } finally {
      loading.value = false
    }
  }

  async function fetchById(id: number) {
    const { data } = await factoryAPI.getById(id)
    const index = factories.value.findIndex(f => f.id === id)
    if (index >= 0) {
      factories.value[index] = data
    } else {
      factories.value.push(data)
    }
  }

  async function create(request: CreateFactoryRequest) {
    const { data } = await factoryAPI.create(request)
    factories.value.push(data)
    return data
  }

  async function update(id: number, request: UpdateFactoryRequest) {
    const { data } = await factoryAPI.update(id, request)
    const index = factories.value.findIndex(f => f.id === id)
    if (index >= 0) {
      factories.value[index] = data
    }
    return data
  }

  async function remove(id: number) {
    await factoryAPI.delete(id)
    factories.value = factories.value.filter(f => f.id !== id)
  }

  return {
    factories,
    currentFactoryId,
    currentFactory,
    loading,
    error,
    fetchAll,
    fetchById,
    create,
    update,
    remove
  }
})
```

## 5. Validation System

### 5.1 Validation Composable

```typescript
// composables/useValidation.ts
export function useValidation() {
  function validateOverclock(value: number): string | null {
    if (value < 0 || value > 250) {
      return 'Overclock must be between 0.000 and 250.000'
    }
    return null
  }

  function validateSommersloop(value: number, machineType: MachineType): string | null {
    const limits = {
      Constructor: 1,
      Assembler: 2,
      Manufacturer: 4,
      // ... etc
    }
    const max = limits[machineType] || 0
    if (value > max) {
      return `${machineType} supports max ${max} Sommersloop`
    }
    return null
  }

  function validateMachineCount(value: number): string | null {
    if (value <= 0) {
      return 'Machine count must be greater than 0'
    }
    return null
  }

  return {
    validateOverclock,
    validateSommersloop,
    validateMachineCount
  }
}
```

## 6. Local Storage Integration

### 6.1 Preferences Store

```typescript
// stores/preferences.ts
import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export const usePreferencesStore = defineStore('preferences', () => {
  const selectedFactoryId = ref<number | null>(null)
  const dashboardFilters = ref({
    balanceState: null,
    factoryId: null,
    itemType: null
  })
  const factoryViewTab = ref<'production' | 'raw' | 'power'>('production')

  // Load from localStorage on init
  function loadFromStorage() {
    const saved = localStorage.getItem('satisflow-preferences')
    if (saved) {
      const data = JSON.parse(saved)
      selectedFactoryId.value = data.selectedFactoryId
      dashboardFilters.value = data.dashboardFilters
      factoryViewTab.value = data.factoryViewTab
    }
  }

  // Watch for changes and save to localStorage
  watch(
    [selectedFactoryId, dashboardFilters, factoryViewTab],
    () => {
      localStorage.setItem('satisflow-preferences', JSON.stringify({
        selectedFactoryId: selectedFactoryId.value,
        dashboardFilters: dashboardFilters.value,
        factoryViewTab: factoryViewTab.value
      }))
    },
    { deep: true }
  )

  loadFromStorage()

  return {
    selectedFactoryId,
    dashboardFilters,
    factoryViewTab
  }
})
```

## 7. Testing Strategy

### 7.1 Vitest Unit Tests

**Coverage Targets**:
- Composables: 90%+
- Stores: 85%+
- Validation functions: 100%
- Utilities: 90%+

**Example Test**:
```typescript
// composables/__tests__/useValidation.spec.ts
import { describe, it, expect } from 'vitest'
import { useValidation } from '../useValidation'

describe('useValidation', () => {
  const { validateOverclock } = useValidation()

  it('accepts valid overclock values', () => {
    expect(validateOverclock(100)).toBeNull()
    expect(validateOverclock(0)).toBeNull()
    expect(validateOverclock(250)).toBeNull()
  })

  it('rejects invalid overclock values', () => {
    expect(validateOverclock(-1)).toBeTruthy()
    expect(validateOverclock(251)).toBeTruthy()
  })
})
```

### 7.2 Playwright E2E Tests

**Test Scenarios**:
1. **Factory Creation Flow**: Create factory → Add production line → Verify in list
2. **Logistics Creation**: Create bus with multiple conveyors → Verify transport details
3. **Dashboard Filtering**: Filter by balance state → Verify correct items shown
4. **Power Generation**: Create coal generator → Verify power calculation
5. **Validation**: Submit invalid OC value → Verify error message

**Example Test**:
```typescript
// e2e/factory.spec.ts
import { test, expect } from '@playwright/test'

test('create production line', async ({ page }) => {
  await page.goto('/factory')
  
  // Select factory
  await page.selectOption('[data-test="factory-selector"]', '1')
  
  // Click create production line
  await page.click('[data-test="create-production-line"]')
  
  // Fill form
  await page.selectOption('[data-test="recipe-select"]', 'IronIngot')
  await page.fill('[data-test="name-input"]', 'Iron Smelting')
  await page.fill('[data-test="machines-input"]', '10')
  await page.fill('[data-test="oc-input"]', '150')
  
  // Submit
  await page.click('[data-test="submit-btn"]')
  
  // Verify in list
  await expect(page.locator('text=Iron Smelting')).toBeVisible()
})
```

## 8. Implementation Roadmap

### Phase 1: Foundation (Week 1)
- [x] Vite configuration with API proxy
- [ ] TypeScript type definitions (complete)
- [ ] Axios API client with interceptors
- [ ] API endpoint functions
- [ ] Router setup with views
- [ ] Base UI components (Button, Modal, LoadingSpinner)

### Phase 2: Core Features (Week 2-3)
- [ ] Factory store and composables
- [ ] Dashboard view implementation
- [ ] Factory view with tabs
- [ ] Production line CRUD
- [ ] Raw input CRUD
- [ ] Power generator CRUD

### Phase 3: Logistics (Week 4)
- [ ] Logistics store and composables
- [ ] Logistics view implementation
- [ ] Transport type forms (Bus, Train, Truck, Drone)
- [ ] Validation system

### Phase 4: Polish (Week 5)
- [ ] Error handling and user feedback
- [ ] Local storage integration
- [ ] UI/UX improvements
- [ ] Accessibility (ARIA labels, keyboard navigation)

### Phase 5: Testing (Week 6)
- [ ] Vitest unit tests
- [ ] Playwright E2E tests
- [ ] Performance optimization
- [ ] Documentation

## 9. Estimated File Counts and Sizes

| Category | Files | Est. Lines |
|----------|-------|------------|
| Types | 1 | 500 |
| API Layer | 2 | 300 |
| Stores | 4 | 800 |
| Composables | 6 | 600 |
| UI Components | 10 | 1500 |
| Form Components | 6 | 1200 |
| Feature Components | 15 | 3000 |
| Views | 3 | 1500 |
| Tests | 30 | 2000 |
| **Total** | **77** | **~11,400** |

## 10. Next Steps

Once this plan is approved:

1. **Review and Feedback**: Confirm the architecture aligns with your vision
2. **Priority Clarification**: Which view should be implemented first?
3. **UI Framework**: Should we add a CSS framework (Tailwind, Vuetify) or custom CSS?
4. **Icon Library**: Prefer Font Awesome, Material Icons, or custom SVGs?
5. **Mode Switch**: Switch to Code mode to begin implementation

**Questions**:
- Does this architecture match your expectations?
- Any specific UI/UX requirements (colors, layout preferences)?
- Should we start with Dashboard or Factory view first?
- Need any additional features not covered in the brief?