# Logistics Transport Editors Cleanup

## Objective
Replace placeholder logistics data with live game data, enforce compatibility checks, and ensure edit flows update existing entries instead of duplicating them.

## Current Gaps
- Editors use hard-coded sample item lists rather than sourcing from the `gameData` store, causing drift with backend updates.
- Solid/fluid compatibility validation is incomplete, allowing invalid configurations.
- Edit workflows duplicate logistics lines instead of invoking proper update endpoints.

## Implementation Plan
1. **Data sourcing**: Update logistics editors under `frontend/src/views/logistics/` (and related components) to consume items and transport metadata from `frontend/src/stores/gameData.ts`.
2. **Validation boundary**: Introduce compatibility helpers ensuring conveyor, pipeline, and wagon assignments match solid/fluid item categories. Display clear inline error messages when mismatched.
3. **API alignment**: Modify Pinia actions responsible for logistics CRUD to call the correct update endpoints defined in `frontend/src/api/endpoints.ts`.
4. **UI adjustments**: Clarify UI copy and grouping for "attach to existing" vs "new line" flows, aligning with IA guidance.
5. **Preview/feedback**: Optionally surface inline summaries (capacity, throughput) derived from backend responses post-update.
6. **Documentation**: Record supported logistics scenarios and validation approach for future contributors.

## File Impact
- Logistics editor components in `frontend/src/views/logistics/`
- `frontend/src/stores/logisticsStore.ts` (name placeholder)
- `frontend/src/stores/gameData.ts`
- API endpoint wrappers in `frontend/src/api/endpoints.ts`
- Shared validation helpers (potential new module under `frontend/src/utils/`)

## Testing Strategy
- **Unit**: Validate compatibility helpers and store actions (update vs create code paths).
- **E2E**: Playwright coverage for editing an existing logistics line, verifying no duplicate entries and correct error messaging for incompatible selections.
- **Regression**: Confirm sample data removal does not break seed flows or onboarding demos.

## Dependencies & Sequencing
- Requires game data cache guarantees from the State Management Hardening initiative.
- Error handling integration ensures backend failures show consistent toasts.

## Architecture & Coding Standards Alignment
- Maintains separation by keeping item catalogs in stores/composables, not components.
- TypeScript DTOs must reflect backend enums; no hard-coded strings for item IDs.
- Add `data-test` selectors for logistics forms to aid deterministic testing.

## Risks & Mitigations
- **Backend drift**: Create store-level assertions that warn if game data is unavailable before rendering editors.
- **User confusion**: Provide contextual hints and disable invalid actions instead of allowing submission.
