# Standardized Form Validation and Preview Flow

## Objective
Unify validation, error messaging, and backend-driven previews across production line, raw input, and power generator forms while removing duplicated logic.

## Current Gaps
- Validation logic is scattered in components, causing inconsistent messaging and missed rules.
- Frontend occasionally performs gameplay math that should come from backend preview endpoints.
- Preview requests lack debouncing and race-condition handling, leading to stale output.

## Implementation Plan
1. **Composable design**: Introduce `frontend/src/composables/useValidation.ts` exporting rule registries, shared validators, and helpers to map backend field errors to UI-friendly messages.
2. **Backend previews**: Wrap preview endpoints inside dedicated service functions in `frontend/src/api/endpoints.ts` and expose typed actions in relevant stores. Enforce backend as single source of truth.
3. **Debounced flow**: Use `useValidation` + a debounced preview helper (250-400ms) to trigger backend previews after valid field changes. Cancel in-flight requests when user edits again.
4. **Form architecture**: Update `ProductionLineForm`, `RawInputForm`, and `PowerGeneratorForm` under `frontend/src/components/factory/` to consume composables, show inline field messages, and surface a summary panel for form-level issues.
5. **Accessibility hooks**: Wire `aria-invalid`, `aria-describedby`, and `data-test` attributes into each form control based on validation state.
6. **Documentation**: Provide usage examples within component story notes or README to guide future forms.

## File Impact
- `frontend/src/composables/useValidation.ts`
- Factory form components under `frontend/src/components/factory/`
- Pinia stores handling form data and previews inside `frontend/src/stores/`
- API service wrappers in `frontend/src/api/endpoints.ts` and `frontend/src/api/types.ts`
- Test utilities for validation in `frontend/src/tests/`

## Testing Strategy
- **Unit**: Vitest specs for each validator, preview debouncer, and store action (success/error flows).
- **E2E**: Playwright tests that create/edit production lines, raw inputs, and power generators; assert preview values match backend responses and validation messages display correctly.
- **Contract**: Optional integration tests ensuring DTO types align with backend enums and nullable fields.

## Dependencies & Sequencing
- Depends on the API Layer work to normalize error responses and DTOs.
- Ties into centralized error notifications to surface backend failures consistently.
- Requires preferences/state updates for persisted drafts if planned in State Management doc.

## Architecture & Coding Standards Alignment
- Composables encapsulate validation logic, honoring the rule that components should remain declarative.
- All new code must use strict TypeScript types with explicit generics where needed.
- Backend preview endpoints remain the only location for calculation logic, respecting architecture constraints.

## Risks & Mitigations
- **Backend latency**: Provide optimistic UI states (skeleton preview, loading indicators) so debounced calls do not feel sluggish.
- **Error mapping drift**: Centralize mapping tables and share them across components to avoid divergence.
