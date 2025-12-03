# State Management Hardening (Pinia)

## Objective
Enforce consistent Pinia patterns for async flows, error handling, and persisted preferences to support the refactored UI.

## Current Gaps
- Stores expose inconsistent shapes (some mixing state/getters/actions) and occasionally call API endpoints directly from components.
- Status flags (`loading`, `error`) are not standardized, complicating UI feedback states.
- Preferences (selected factory, tabs, theme) are scattered or missing versioning.

## Implementation Plan
1. **Store blueprint**: Define a shared pattern template (state/getters/actions) and document it in `frontend/src/stores/README.md` or similar.
2. **Async status helper**: Create utility functions or mixins to manage `status` enums: `idle | loading | success | error`. Ensure actions update status before/after API calls.
3. **API integration**: Move all API interactions into store actions, keeping components declarative. Import typed endpoint wrappers from `frontend/src/api/endpoints.ts`.
4. **Preferences store**: Formalize a dedicated preferences store (`frontend/src/stores/preferencesStore.ts`) with schema versioning and migration helpers. Persist theme, selected factory, dashboard filters, tab state.
5. **Error funnel**: Integrate the centralized error notification composable so every store action routes failures through the shared pipeline.
6. **Testing + tooling**: Add unit tests validating store behavior (success/error flows) and confirm types remain strict (`defineStore<StoreState, StoreGetters, StoreActions>` patterns).

## File Impact
- All Pinia stores under `frontend/src/stores/`
- New or updated utilities for status management (e.g., `frontend/src/stores/helpers/status.ts`)
- Preferences persistence module(s)
- Store documentation/readme

## Testing Strategy
- **Unit**: Vitest tests for store actions, ensuring status transitions and persistence round-trips behave predictably.
- **Integration**: Optional component tests verifying stores provide the right data to DataTable/forms.
- **E2E**: Validate persisted preferences survive reloads and apply to navigation/filter flows.

## Dependencies & Sequencing
- Depends on API Layer standardization for consistent endpoints and DTOs.
- Feeds into Design System (theme persistence), IA (filters/tabs), and Error Notification pipeline.

## Architecture & Coding Standards Alignment
- Pinia stores remain the single source of truth for client state per architecture rules.
- Strong typing and no direct API calls in components align with the development guide.
- All stores expose `data-test` friendly selectors via computed properties used by the UI.

## Risks & Mitigations
- **Migration complexity**: Plan staged refactors to avoid breaking in-flight features; consider adapter layers if needed.
- **Persistence schema drift**: Include version keys and upgrade scripts to migrate old preference payloads.
