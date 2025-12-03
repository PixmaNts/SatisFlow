# Satisflow Frontend Refactoring Plan

Audience: Senior Vue.js + TypeScript engineers, UI/UX designers, and QA  
Context: Vue 3 + TypeScript + Vite frontend with Pinia, Axios, Playwright, Vitest

This plan defines what to improve, why, and exactly how to implement changes safely and iteratively. It aligns with the project’s established principles:

- Backend is the single source of truth for all game calculations.
- Strong TypeScript types aligned with backend enums and nullable fields.
- Production-grade UX: accessible, fast, resilient, and consistent.

Quick links to relevant areas:

- API/types and client: [frontend/src/api/types.ts](frontend/src/api/types.ts), [frontend/src/api/endpoints.ts](frontend/src/api/endpoints.ts), [frontend/src/api/client.ts](frontend/src/api/client.ts)
- State stores: [frontend/src/stores/](frontend/src/stores/)
- Core components: [frontend/src/components/](frontend/src/components/)
- Views: [frontend/src/views/](frontend/src/views/)
- Styles: [frontend/src/assets/styles/](frontend/src/assets/styles/)
- E2E: [frontend/e2e/](frontend/e2e/)
- Unit tests: [frontend/src/tests/](frontend/src/tests/) (if present)

--------------------------------------------------------------------------------

1) Executive Summary

Primary goals

- UX: Improve clarity, speed, and trust. Eliminate inconsistent patterns, improve keyboard support, and enhance feedback (errors, progress, success).
- Design: Establish a small but robust design system with tokens, components, and layout guidelines. Enable dark mode.
- Engineering: Harden state management, error handling, data tables, and form validation. Reduce duplication and drift from backend rules.
- Testing: Increase unit and E2E coverage with stable selectors and regression guardrails. Prepare CI for reliable automation.

Key initiatives

- DataTable overhaul for correct numeric sorting, accessibility, and large-data performance.
- Centralized error and toast notification handling integrated with API client and stores.
- Validation and preview patterns for all forms aligned with backend preview endpoints.
- Theming via CSS variables and a minimal design system extracted from existing styles.
- Accessibility pass for focus management, ARIA, contrast, and keyboard traps.
- DX improvements: lint rules, TS configs, stable test IDs, and testing utilities.

--------------------------------------------------------------------------------

2) Current State Assessment (from project docs and structure)

Strengths

- Clear separation of concerns with a complete view/component/store/API architecture.
- Strong type definitions and alignment with backend enums and null handling.
- Backend-driven calculations already integrated across key components.
- E2E tests exist for critical flows (save/load, calculation accuracy).

Gaps and risks

- DataTable limitations (stringifying values, broken numeric sorting, limited accessibility).
- Error handling inconsistent across components; missing centralized toast/error helper.
- Logistics edit flow may duplicate entries instead of calling update endpoints.
- Transport editors contain sample/hard-coded item lists instead of using game data.
- Dashboard filters missing parity (factory dropdown, production group taxonomy, numeric sorting).
- Accessibility needs systematic focus/ARIA/contrast audit across components.
- Performance: tables may need virtualization; forms should provide debounced previews.
- Testing: Additional E2E coverage, shared test utilities, and stable data-test attributes required.

--------------------------------------------------------------------------------

3) UX Objectives and Success Criteria

3.1 Objectives

- Predictable list and table interactions: correct sorting, column semantics, keyboard navigation, and screen reader labels.
- Clear, non-intrusive feedback for network states: loading, success, error, retry.
- Form previews use backend endpoints; no gameplay math in the frontend.
- Consistent dialogs, toasts, and inline messages.

3.2 Success criteria

- All sortable numeric columns sort numerically by default and via explicit sort accessors.
- Error handling centralized: API errors always produce a toast and optional inline hints.
- All create/edit forms support debounced backend previews and show accurate power/flow data.
- Keyboard-only workflows are fully supported on all major forms and tables.

--------------------------------------------------------------------------------

4) Design System and Theming

4.1 Design tokens

- Consolidate tokens in [frontend/src/assets/styles/variables.css](frontend/src/assets/styles/variables.css)
  - Colors (with WCAG AA contrast), spacing scale, font sizes, radii, shadows.
  - Light and dark mode variable pairs: prefer HSL for easier theming.
- Add animation timings and easing in [frontend/src/assets/styles/transitions.css](frontend/src/assets/styles/transitions.css)
- Add micro-interactions in [frontend/src/assets/styles/micro-interactions.css](frontend/src/assets/styles/micro-interactions.css)

4.2 Theming

- Implement a root data-theme attribute on the html element and toggle in a composable:
  - Preferred theme in localStorage, default to system preference.
- Verify contrast across key components (nav, tables, forms, modals).

4.3 Component foundations

- Promote a minimal design kit derived from existing components:
  - Buttons, Inputs, Selects, Modals, Tabs, Alerts/Toasts, Skeletons/Spinners.
- Ensure all base components expose aria-* attributes and forward native events.

Deliverables

- Tokenized colors, spacing, and typography.
- Theme switcher and persisted preference.
- Updated base UI components in [frontend/src/components/ui/](frontend/src/components/ui/).

--------------------------------------------------------------------------------

5) Information Architecture and Navigation

5.1 Main nav

- Validate labels and routes in [frontend/src/router/index.ts](frontend/src/router/index.ts)
- Keep the three primary areas: Dashboard, Factory, Logistics. Ensure selected state and keyboard focus ring are visible.

5.2 In-view IA

- Dashboard: Expose factory filter and production group filters above the table.
- Factory: Confirm tabs structure (Production, Raw Input, Power). Persist active tab per factory in preferences store.
- Logistics: Provide clear affordances for “attach to existing” vs “new line”.

Deliverables

- Nav active state standardized in [frontend/src/components/layout/MainNav.vue](frontend/src/components/layout/MainNav.vue)
- Filter sections with consistent spacing and semantics.

--------------------------------------------------------------------------------

6) Component Refactoring Program

6.1 DataTable overhaul [frontend/src/components/ui/DataTable.vue](frontend/src/components/ui/DataTable.vue)

- Add column-level options: id, header, accessor, sortType (number, string, custom), sortAccessor function, isNumeric flag.
- Numeric sorting: no string coercion; use dedicated comparators.
- Accessibility:
  - Table semantics with scope, aria-sort, keyboard navigable headers.
  - Focus outlines and logical tab order.
- Performance:
  - Row virtualization for large datasets (optional progressive enhancement).
- Styling:
  - Integrate tokens and theming; responsive column stacking for narrow screens.

6.2 Form and validation patterns

- Centralize validation rules into a composable in [frontend/src/composables/useValidation.ts](frontend/src/composables/useValidation.ts)
- Standardize form patterns:
  - Debounced backend preview requests for production lines, raw inputs, power generators.
  - Inline field messages and a top-level form summary area.
  - Form components must accept aria-invalid, aria-describedby props and expose data-test attributes.

6.3 Error handling and toasts

- Create [frontend/src/composables/useErrorNotification.ts](frontend/src/composables/useErrorNotification.ts) for:
  - Normalized error objects (message, code, fieldErrors).
  - Toast notifications (success/error/info) via a shared [frontend/src/components/ui/ToastContainer.vue](frontend/src/components/ui/ToastContainer.vue).
- Update API client in [frontend/src/api/client.ts](frontend/src/api/client.ts) to:
  - Intercept errors, map to normalized shape, and expose consistent messaging.
  - Support optional retry-with-jitter strategy for transient failures.

6.4 Modal and keyboard control

- Standardize modal focus trapping and escape handling in [frontend/src/components/ui/Modal.vue](frontend/src/components/ui/Modal.vue).
- Ensure all modals have labelledby/ describedby wiring.

6.5 Transport editors cleanup

- Replace all sample/hard-coded item lists with live game data from [frontend/src/stores/gameData.ts](frontend/src/stores/gameData.ts).
- Validate solid vs fluid compatibility for conveyors/pipelines/wagons at the component boundary.
- Provide user-friendly inline validation messages.

--------------------------------------------------------------------------------

7) State Management Hardening (Pinia)

7.1 Store structure

- Ensure each store in [frontend/src/stores/](frontend/src/stores/) exports:
  - state, getters (pure), actions (async/side-effects), and typed DTOs.
- Collocate API calls in actions; components should not call [frontend/src/api/endpoints.ts](frontend/src/api/endpoints.ts) directly.

7.2 Error and loading states

- Standardize “status flags”: idle | loading | success | error per async operation.
- Centralize error funneling to the error notification composable.
- Use abort controllers or a simple request token to avoid race conditions in rapid updates.

7.3 Preferences and persistence

- Use a dedicated preferences store with stable schema and versioning for localStorage keys.
- Persist: selected factory, dashboard filters, active factory tab, theme.

Deliverables

- Consistent Pinia patterns across factory, logistics, dashboard, gameData, preferences stores.

--------------------------------------------------------------------------------

8) API Layer and DTOs

8.1 API client [frontend/src/api/client.ts](frontend/src/api/client.ts)

- Add interceptors for:
  - Request: inject standard headers, request IDs.
  - Response: normalize errors into a shared shape.
- Timeouts and retries for GET endpoints (configurable).

8.2 Types and endpoints

- Keep types in [frontend/src/api/types.ts](frontend/src/api/types.ts) and endpoints in [frontend/src/api/endpoints.ts](frontend/src/api/endpoints.ts) aligned with backend schema.
- Do not reintroduce client-side calculation fields. Use backend-calculated fields (power, machines, item rates).

Deliverables

- Error normalization utilities
- Typed endpoint wrappers with narrow function signatures per view need.

--------------------------------------------------------------------------------

9) Accessibility Plan

9.1 Focus management

- Ensure visible focus rings on all interactive elements (buttons, links, inputs, table headers).
- Focus trap and initial focus in modals; return focus on close.

9.2 ARIA and labeling

- aria-label/aria-labelledby/aria-describedby on inputs, buttons, modals, and tabs.
- role="status" regions for async feedback.

9.3 Color and contrast

- Validate token contrast against WCAG AA in both themes.
- Avoid color-only signalling; include icons or text.

Deliverables

- A11y checklist applied to [frontend/src/components/ui/](frontend/src/components/ui/), [frontend/src/components/factory/](frontend/src/components/factory/), [frontend/src/views/](frontend/src/views/)

--------------------------------------------------------------------------------

10) Performance Strategy

10.1 Rendering

- Virtualize large tables if row count warrants it.
- Debounce expensive operations (search, previews) at 250–400ms.

10.2 Network

- Cache stable lists (game data, templates) in memory per session with refresh controls.
- Use conditional refetch on view activation.

10.3 Bundling

- Route-level code splitting in [frontend/src/router/index.ts](frontend/src/router/index.ts)
- Verify production sourcemaps, preloading hints, and CSS extraction.

Deliverables

- Measurable interaction latency under 100ms for common actions.
- Table interactions (sort/filter) under 50ms for 1–2k rows or virtualized.

--------------------------------------------------------------------------------

11) Testing Expansion

11.1 Unit tests (Vitest)

- Composables: validation, error handling, theme management.
- Stores: action flows (success, error), status flags, and persistence round-trips.
- UI: DataTable sort behavior (numeric vs string), Modal focus trapping.

11.2 E2E tests (Playwright) [frontend/e2e/](frontend/e2e/)

- Factory CRUD including raw inputs and power generators end-to-end.
- Logistics update flows: edit vs create new, attach-to-existing scenarios.
- Dashboard filters parity and persistence.
- Error-to-toast assertions with stable data-test selectors.

11.3 Test ergonomics

- Introduce shared test utilities under [frontend/src/tests/](frontend/src/tests/) and strengthen data-test attributes across components.

Deliverables

- Coverage targets: Composables 90%+, Stores 85%+, Critical UI components 80%+
- Stable and fast E2E suite with deterministic selectors.

--------------------------------------------------------------------------------

12) Developer Experience and CI

12.1 Linting and formatting

- Enforce ESLint + Prettier configs: no console logs in production builds, explicit return types in stores and composables.

12.2 Type safety

- Strict TS across the repo; forbid any except in typed boundary modules with comments.

12.3 CI

- Add workflows:
  - Type check
  - Lint
  - Unit tests
  - E2E (with server spin-up if needed)

Deliverables

- A green CI that blocks merges on failing checks.
- Pre-commit hooks optional for local speed-ups.

--------------------------------------------------------------------------------

13) Step-by-Step Execution Plan (Sprints)

Sprint 0: Foundations (1–2 days)

- Introduce useErrorNotification composable and ToastContainer.
- Wire API client error interceptor to the composable.
- Add stable data-test attributes to key components.

Sprint 1: DataTable and Filters (2–3 days)

- Refactor DataTable with numeric sorting, sortAccessors, and a11y.
- Add factory and production-group filters to Dashboard.
- Update routes and views for responsive table layouts.

Sprint 2: Forms and Previews (2–3 days)

- Standardize validation via useValidation.
- Implement debounced backend previews across ProductionLineForm, RawInputForm, PowerGeneratorForm in [frontend/src/components/factory/](frontend/src/components/factory/).
- Ensure preview values come only from backend.

Sprint 3: Logistics Editors and Game Data (2 days)

- Replace sample item lists with gameData store data.
- Validate solid vs fluid compatibility with helpful messages.
- Fix edit flow to call update endpoints instead of duplicating lines.

Sprint 4: Accessibility Pass (2 days)

- Focus management and ARIA audit on Modals, Tabs, Tables, Forms.
- Contrast validation for tokens and themes.
- Keyboard nav patterns validated in E2E.

Sprint 5: Theming and Design System (2 days)

- Finalize tokens and dark mode toggle + persistence.
- Apply tokens to base UI components.
- Update docs/readme for design tokens and usage.

Sprint 6: Performance and Testing (2–3 days)

- Virtualize large tables if necessary, debounce inputs, cache stable endpoints.
- Expand unit/E2E coverage to targets.
- Integrate CI workflows.

--------------------------------------------------------------------------------

14) Risks and Mitigations

- Scope creep in DataTable: Keep MVP focused on sorting, a11y, basic virtualization.
- Preview API reliance: Debounce and guard against race conditions; show skeletons.
- Theming regressions: Use token-driven approach and visual snapshots in E2E.
- Test flakiness: Adopt deterministic data-test selectors and test utilities.

--------------------------------------------------------------------------------

15) Acceptance Criteria per Initiative

DataTable

- Numeric columns sort correctly; aria-sort updates; keyboard accessible.

Error handling

- Any API failure shows a toast with a human-readable message and optional inline hints.

Forms

- All previewed calculations match backend responses; zero client-side math.

Transport editors

- Items sourced from gameData; incompatible selections blocked with clear messaging.

Accessibility

- Keyboard-only flows possible; modals trap focus; color contrast AA pass.

Theming

- Theme persisted; all base components read from tokens; no hard-coded colors.

Testing/CI

- Unit/E2E thresholds met; CI blocks failing builds.

--------------------------------------------------------------------------------

16) Non-Goals (for this refactor)

- Introducing complex design frameworks or CSS-in-JS.
- Rewriting the router or store framework.
- Implementing i18n in this phase (can be planned later).
- WASM integration (backend REST suffices).

--------------------------------------------------------------------------------

17) Working Agreements

- No reintroduction of gameplay calculations in the frontend.
- Every new or refactored component has stable data-test selectors.
- Error flows must exercise the centralized notification path.
- All PRs include before/after screenshots or video for UX changes and pass a11y checks.

--------------------------------------------------------------------------------
Appendix A: Checklists

A11y quick check

- Focus visible on all interactives
- aria-label/labelledby/describedby present
- aria-sort on table headers
- Escape closes modals, returns focus
- No keyboard traps

Testing quick check

- data-test attributes present
- Unit tests for composables and stores
- E2E for critical flows and filters
- Network error cases covered

Design tokens quick check

- No hard-coded colors in refactored components
- Spacing and typography use scales
- Dark mode verified for contrast and legibility

--------------------------------------------------------------------------------

Implementation Notes

- Keep references to filenames and directories in this document to guide work. Do not include code blocks or language constructs requiring line-number syntax links.
- All calculations must remain backend-driven and surfaced via API fields and preview endpoints.

--------------------------------------------------------------------------------
