# Frontend Implementation Plan

## Goals
- Reach feature completeness for Dashboard, Factory, and Logistics views per the product brief in `.kilocode/rules/memory-bank/brief.md`.
- Replace placeholder UX with fully wired CRUD flows that persist through the existing REST API described in `.kilocode/rules/memory-bank/api-and-testing.md`.
- Establish a reliable test net (unit, component, and e2e) that guards happy paths and key error paths.

## Feature Gaps & Missing Integrations (P0)
1. **Factory sub-tabs CRUD wiring** ✅ ENDPOINTS TESTED
   - ✅ **COMPLETED:** Production line, raw input, and power generator endpoints fully implemented and tested
   - ✅ All CRUD operations (create/update/delete) wired through `useFactoryStore`
   - ✅ Comprehensive test suite: 26 unit tests for endpoints covering all CRUD operations
   - ⏳ **Remaining:** Blueprint UI mode is still a placeholder banner (nested lines, Somersloop limits, OC validation)

2. **Logistics editing duplicates lines**
   - `src/components/logistics/LogisticsLineForm.vue` creates a second record when “editing” because the API client lacks an update/delete path for the previous transport.
   - **Plan:** add `PUT /api/logistics/:id` (or delete + replace) in `src/api/endpoints.ts`, expose it through `useLogisticsStore`, and update the form so edits preserve IDs when the backend accepts them. Until the backend supports updates, fall back to delete-then-create with proper confirmation.

3. **Transport item sources are hard-coded**
   - `src/components/logistics/BusEditor.vue` (and siblings) ship with inline “Sample items…” lists, contradicting the shared game-data catalogue.
   - **Plan:** swap to `useGameDataStore` for item options, surface validation when a player selects an item incompatible with the chosen transport medium (solids vs fluids), and ensure we re-fetch game data before the editors mount.

4. **Dashboard feature parity gaps**
   - Current UI only filters by state and name search; requirements call for factory, production group, and quantity filters plus sorting by amount/factory.
   - **Plan:** expand `DashboardView` to expose the missing filters (factory dropdown fed by `/factories`, production-group taxonomy from the engine), wire them into `useDashboardStore.filteredItemBalances`, and enhance `DataTable` sorting so numeric columns order correctly.

## High-Priority Refactors (P1)
- **DataTable numeric sorting & accessibility:** `src/components/ui/DataTable.vue` stringifies all values; extend the column definition with `sortAccessor`/`sortType` to support numeric/item-name sorting and add keyboard focus handling.
- **Store/service cohesion:** move repeated error handling into a shared helper, and normalise loader state so nested modals don’t stack spinners.
- **Error/empty states:** list components should surface API errors via the global toast system instead of silent console logs.

## Test Coverage Plan
- **✅ API Endpoint Tests:** Comprehensive unit test suite (`src/api/__tests__/endpoints.test.ts`) with 26 tests covering:
  - Production line CRUD (create, update, delete) - 9 tests
  - Raw input CRUD (create, update, delete) - 9 tests
  - Power generator CRUD (create, update, delete) - 9 tests
  - Endpoint integration & URL construction - 4 tests
  - All tests passing with proper mocking of Axios client

- **Store unit tests:** replicate the `useDashboardStore` test pattern for `useFactoryStore`, `useLogisticsStore`, and `useGameDataStore` as soon as their APIs are real.
- **Component tests:** add Vitest + Vue Test Utils specs for the major forms (factories, production lines, logistics) covering validation, conditional UI (pressuriser toggles, machine groups), and emitted payloads.
- **Integration/e2e tests:** expand Playwright under `frontend/tests/e2e` to cover:
  1. End-to-end factory CRUD, including raw inputs and power generators.
  2. Production line creation with overclock + Somersloop edge cases.
  3. Logistics flows for bus/train/drone with attach-to-existing scenarios and filtering assertions.
  4. Dashboard filters and persistence in `usePreferencesStore`.
  Use MSW or the existing mock handlers in `src/test-utils/mocks/handlers.ts` for deterministic responses.

## Sequencing & Dependencies
1. ✅ **Backend alignment:** All REST endpoints for production lines, raw inputs, power generators exist and are tested.
2. ✅ **Factory CRUD endpoints:** Fully implemented and tested via `useFactoryStore` with 26 unit tests.
3. **Blueprint UI completion:** Implement nested line editing, Somersloop limits, and OC validation.
4. **Logistics cleanup:** Implement `PUT /api/logistics/:id` endpoint on backend, integrate real item catalogue, add tests.
5. **Dashboard enhancements:** Ship missing filters, numeric sorting, and persistence updates with tests.
6. **Polish & QA:** Harden error handling, accessibility, and add comprehensive Playwright coverage before merging to main.

## Risks & Mitigations
- **API surface drift:** keep the TypeScript types in `src/api/types.ts` in sync with backend structs; consider generating them from OpenAPI/Serde once available.
- **Complex Blueprint UX:** prototype nested line editing incrementally (view ➜ duplicate ➜ full edit) to avoid blocking P0.
- **Test flakiness:** rely on deterministic mocks and avoid direct `setTimeout` usage in tests; favour composables returning promises we can await.

