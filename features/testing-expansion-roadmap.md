# Testing Expansion Roadmap

## Objective
Increase automated coverage across unit, component, and end-to-end layers while establishing stable selectors and shared test utilities.

## Current Gaps
- Incomplete unit coverage for composables, stores, and UI components.
- E2E scenarios miss critical workflows (logistics updates, dashboard filters).
- Lacks shared utilities/data builders leading to brittle tests.

## Implementation Plan
1. **Coverage targets**: Enforce goals (Composables 90%+, Stores 85%+, Critical UI 80%+) through Vitest configurations and coverage thresholds in `package.json` or CI scripts.
2. **Test utilities**: Create reusable helpers and fixtures under `frontend/src/tests/` (factories, store mocks, API stubs). Include data builders for production lines/logistics.
3. **Unit focus**: Add Vitest specs for `useValidation`, `useErrorNotification`, theme composables, and Pinia stores. Use dependency injection/mocking for API calls.
4. **Component tests**: Utilize Vue Testing Library for DataTable sorting, Modal focus trapping, and Toast rendering.
5. **E2E expansion**: Author Playwright suites covering factory CRUD (including previews), logistics edit vs new flows, dashboard filters persistence, and error-to-toast scenarios.
6. **Tooling**: Integrate axe accessibility checks and Playwright trace artifacts into CI for debugging.

## File Impact
- `frontend/src/tests/` (new utilities, fixtures)
- `frontend/e2e/` (expanded suites, selectors)
- CI workflows triggering test jobs
- Component/stores under test may need `data-test` attributes or dependency injection hooks

## Testing Strategy
- **Automated**: Run Vitest (unit/component) and Playwright (E2E) in CI with fail-fast configuration.
- **Manual**: QA smoke tests to validate high-risk paths after major refactors.

## Dependencies & Sequencing
- Requires stable `data-test` selectors added during Design System and Error Notification initiatives.
- Works closely with Developer Experience & CI updates to ensure tests run automatically.

## Architecture & Coding Standards Alignment
- Tests respect module boundaries by mocking API layers rather than reaching into backend logic.
- Encourage type-safe test helpers to avoid `any` usage.

## Risks & Mitigations
- **Flaky E2E**: Use network mocking or seed data to stabilize flows.
- **Maintenance overhead**: Document test patterns and naming conventions to keep suites approachable.
