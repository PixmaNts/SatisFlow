# FRONTEND SOURCE

Vue 3 + TypeScript SPA. Vite 7 build. Pinia stores. Composition API only.

## STRUCTURE

```
src/
├── main.ts              # Bootstrap: Pinia, Router, global CSS, dark theme
├── App.vue              # Root: navbar + router-view + ToastContainer + ErrorBoundary
├── router/index.ts      # Routes: /, /factory, /logistics, /blueprints
├── api/
│   ├── index.ts         # Barrel: api, endpoints, types
│   ├── client.ts        # Axios instance + interceptors + error handling
│   ├── endpoints.ts     # Domain-grouped API functions (574 lines)
│   ├── types.ts         # TypeScript types matching backend DTOs (830 lines)
│   └── logistics-types.ts  # Transport form types + conveyor/pipeline constants
├── stores/
│   ├── index.ts         # Barrel: useFactoryStore, useLogisticsStore, etc.
│   ├── factory.ts       # Factory CRUD + production lines + raw inputs + power
│   ├── logistics.ts     # Logistics line management
│   ├── dashboard.ts     # Summary stats, item balances, power stats
│   ├── gameData.ts      # Static game data (5-min cache TTL)
│   └── preferences.ts   # User prefs with localStorage auto-persist via watch()
├── composables/         # 11 composables (useXxx naming)
│   ├── useToast.ts          # Global toast notification system
│   ├── useValidation.ts     # Form validation (overclock, somersloop, machine count)
│   ├── useErrorHandler.ts   # Centralized error handling
│   ├── useErrorNotification.ts  # API error → toast pipeline
│   ├── useTheme.ts          # Dark/light theme toggle
│   ├── useLocalStorage.ts   # Persistent storage abstraction
│   ├── useFormDraft.ts      # Form draft persistence
│   ├── useFileUpload.ts     # File upload helper (blueprints)
│   ├── useFileDownload.ts   # File download helper (saves)
│   ├── useItemIcon.ts       # Item icon path resolution
│   └── useKeyboardShortcuts.ts  # Global keyboard shortcuts
├── components/
│   ├── ui/              # 21 reusable UI components (barrel export via index.ts)
│   │   ├── Button, Modal, Alert, Tabs, DataTable, ConfirmDialog
│   │   ├── LoadingSpinner, ProgressIndicator, SuccessConfirmation
│   │   ├── ErrorBoundary, EmptyState, Collapsible, ToastContainer
│   │   ├── SkeletonLoader, SkeletonCard, SkeletonTable
│   │   ├── CommandPalette, FloatingActionButton, ItemDisplay
│   │   └── KeyboardShortcutsModal
│   ├── forms/           # Base + domain forms
│   │   ├── BaseInput, BaseSelect, FormNumber, RangeSlider, SearchableSelect
│   │   ├── ValidatedForm, FactoryForm, ValidationDemo
│   │   └── ProductionLineForm, RawInputForm, PowerGeneratorForm, LogisticsLineForm
│   ├── factory/         # Factory feature components (barrel export)
│   ├── logistics/       # Transport editors: TruckEditor, TrainEditor, DroneEditor, BusEditor
│   ├── blueprints/      # BlueprintCard, DetailsModal, ImportModal, FormModal
│   ├── dashboard/       # SaveLoadControls, PowerStatsChart
│   └── layout/          # PageHeader
├── views/               # Page-level components (4 views)
│   ├── DashboardView.vue       # / (summary, balances, power, save/load)
│   ├── FactoryView.vue         # /factory (tabbed: production, raw inputs, power)
│   ├── LogisticsView.vue       # /logistics (transport list + forms)
│   └── BlueprintLibraryView.vue # /blueprints (template CRUD)
├── assets/styles/       # CSS custom properties (variables.css, typography.css, industrial-overrides.css)
├── types/               # validation.ts (validation types + rule definitions)
├── test-utils/          # setup.ts (mocks), test-helpers.ts (mountWithStore, mock factories)
└── tests/               # calculation-violations.test.ts (forbidden pattern scanner)
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add a page/route | `views/` + `router/index.ts` | Import view, add route object |
| Add a reusable component | `components/ui/` | Add to `index.ts` barrel |
| Add a domain component | `components/{factory,logistics,blueprints,dashboard}/` | Barrel export if >3 files |
| Add a composable | `composables/useXxx.ts` | Module-level refs for shared state |
| Add a store | `stores/xxx.ts` | Composition API style, add to `index.ts` barrel |
| Add an API endpoint call | `api/endpoints.ts` + `api/types.ts` | Endpoint fn + request/response types |
| Fix a display calculation | Use backend response values | NEVER recalculate in frontend |
| Add form validation | `composables/useValidation.ts` | Domain-specific validators already exist |
| Add a toast notification | `composables/useToast.ts` | `toast.success()` / `toast.error()` |
| Style/theming | `assets/styles/variables.css` | CSS custom properties, dark-theme class |
| Add a test | `__tests__/` co-located with source | MSW handlers in `test-utils/mocks/` |

## DATA FLOW

```
User action → Component → Store action → API endpoint → Axios client → Backend
                ↑                                              |
                └─────── Store state update ←── Response ──────┘
```

- **Components NEVER call API directly** — always through store actions
- **Stores hold typed API responses** — `FactoryResponse[]`, `DashboardSummary`, etc.
- **gameData store** caches static data (recipes, items, machines) for 5 minutes
- **preferences store** auto-persists to localStorage via `watch()`

## CONVENTIONS

- **`<script setup lang="ts">`** — Composition API only, no Options API
- **Barrel exports** — `index.ts` files for stores, components/ui, api, components/factory
- **Import style**: `import { X } from '@/stores'` (not individual files)
- **CSS**: Scoped styles + CSS custom properties from `variables.css`
- **No semicolons, single quotes, 2-space indent** — enforced by Prettier/ESLint
- **Test files**: co-located in `__tests__/` directories, `.test.ts` or `.spec.ts` suffix

## ANTI-PATTERNS

- **NEVER calculate game values in frontend** — no `Math.pow(oc, 1.321928)`, no hardcoded power/fuel/extraction values. Use backend responses only.
- **NEVER call API from components** — route through Pinia store actions.
- **NEVER add `@ts-ignore` or `as any`** — fix the types properly.
- **NEVER use semicolons** — Prettier enforces `semi: false`.

## COMPLEXITY HOTSPOTS

| File | Lines | Why large |
|------|-------|-----------|
| `api/types.ts` | 830 | Full TypeScript API contract (all request/response types) |
| `api/endpoints.ts` | 574 | All API endpoint wrappers |
| `components/factory/RawInputForm.vue` | 1234 | Complex form: extractors, purity, overclocking, preview |
| `components/factory/ProductionLineForm.vue` | 822 | Recipe autocomplete, machine groups, somersloop |
| `components/ui/DataTable.vue` | 687 | Reusable table: sort, filter, pagination |
| `views/DashboardView.vue` | 625 | Multiple data sections, save/load integration |
