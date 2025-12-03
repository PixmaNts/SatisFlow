# Centralized Error Notification Platform

## Objective
Create a reusable error handling pipeline that normalizes API failures, surfaces consistent toast notifications, and provides optional inline hints to affected components.

## Current Gaps
- Error states are handled ad hoc in components, duplicating message logic and missing edge cases.
- Toast presentation varies, and some failures silently fail without user feedback.
- Stores call API endpoints directly without funneling through a unified normalization layer.

## Implementation Plan
1. **Composable**: Implement `frontend/src/composables/useErrorNotification.ts` exposing helpers to normalize errors (`message`, `code`, `fieldErrors`), trigger toasts, and provide retry callbacks.
2. **Toast container**: Create or enhance `frontend/src/components/ui/ToastContainer.vue` to render queued notifications, ensuring accessibility (`role="status"`, focus handling).
3. **API interceptors**: Update `frontend/src/api/client.ts` to intercept responses, convert errors into normalized shapes, and optionally apply retry-with-jitter for transient failures.
4. **Store integration**: Refactor Pinia stores to use the composable for all async actions, ensuring `status` flags update in tandem (idle/loading/success/error).
5. **Inline messaging**: Components can request inline hints from the normalized error payload (e.g., map field errors back to forms).
6. **Telemetry hooks**: Optionally emit structured error events to logging infrastructure if available.

## File Impact
- `frontend/src/composables/useErrorNotification.ts`
- `frontend/src/components/ui/ToastContainer.vue`
- `frontend/src/api/client.ts`
- `frontend/src/api/errors.ts` (new utility module)
- Pinia stores under `frontend/src/stores/`
- Consumer components needing inline error hints

## Testing Strategy
- **Unit**: Vitest tests covering error normalization for Axios responses, network failures, and backend-provided validation errors.
- **Unit/UI**: Component tests for ToastContainer to ensure queue ordering, auto-dismiss, and keyboard accessibility.
- **E2E**: Playwright scenarios forcing API failures and asserting toast visibility plus inline messaging on forms.

## Dependencies & Sequencing
- Forms/validation refactor relies on normalized errors for backend field mapping.
- State management hardening ensures stores expose consistent status flags consumed here.

## Architecture & Coding Standards Alignment
- Keeps side-effects within composables/stores, matching architectural separation (components stay declarative).
- Avoids `any` by defining explicit error payload types aligned with backend DTOs.
- Ensures every new flow attaches `data-test` selectors to toast entries for deterministic testing.

## Risks & Mitigations
- **Double notifications**: Implement deduplication or suppression per request ID to avoid spam.
- **Unhandled errors**: Provide a final `window.onerror` safety net that routes to the same pipeline.
