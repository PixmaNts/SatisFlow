# DataTable Overhaul

## Objective
Deliver an accessible, performant DataTable component that sorts numeric data correctly, supports responsive layouts, and integrates with new filter requirements.

## Current Gaps
- Numeric values are stringified prior to sorting, producing incorrect order.
- Table headers lack appropriate semantics (`scope`, `aria-sort`), hindering accessibility.
- Large datasets stutter during scroll; no virtualization or batching exists.
- Styling does not consume the new design tokens, leading to visual drift between themes.

## Implementation Plan
1. **API redesign**: Update `frontend/src/components/ui/DataTable.vue` to accept column config objects with `id`, `header`, `accessor`, `sortType`, `sortAccessor`, and `isNumeric` flags. Provide type-safe definitions in a companion TypeScript file.
2. **Sorting engine**: Implement comparator functions for numeric, string, and custom sort types. Ensure numeric comparators operate on numbers and gracefully handle null/undefined values.
3. **Accessibility wiring**: Apply table semantics (`<th scope="col">`), toggle `aria-sort` based on state, and ensure headers are keyboard focusable with Enter/Space toggles.
4. **Performance guardrails**: Evaluate virtualization (e.g., `vue-virtual-scroll-list` or custom) for datasets above 500 rows. Provide graceful fallback for smaller sets to avoid complexity.
5. **Responsive behavior**: Add CSS strategies (stacking, column hiding) using tokens so tables remain usable on narrow screens.
6. **Filter integration**: Expose hooks for external filter components to consume table state (sorted column, direction, filtered rows) without coupling.
7. **Documentation + migration**: Create upgrade notes for views using the DataTable, including updated props and slot patterns.

## File Impact
- `frontend/src/components/ui/DataTable.vue`
- `frontend/src/components/ui/DataTable.types.ts` (new or updated)
- Any table-consuming views: `frontend/src/views/dashboard/`, `frontend/src/views/factory/`, `frontend/src/views/logistics/`
- Test utilities under `frontend/src/tests/`

## Testing Strategy
- **Unit**: Vitest coverage for sort accessors (numeric vs string vs custom), null handling, and keyboard interaction handlers.
- **E2E**: Playwright scenarios verifying sorting order, responsive collapse, and keyboard navigation across table headers.
- **Performance**: Profile renders with 1-2k rows to ensure <50ms sort latency; add regression benchmarks if available.

## Dependencies & Sequencing
- Consumes design tokens from the Design System modernization.
- Works in concert with Dashboard filter updates in the IA doc.
- Relies on centralized error/toast handling for consistent empty/error states.

## Architecture & Coding Standards Alignment
- Keep data shaping inside Pinia stores; the component renders already-prepared datasets, satisfying the separation-of-concerns rules.
- Type definitions must remain in `.ts` modules with explicit exports; avoid inline `any` types.
- Provide `data-test` hooks on headers and rows for stable E2E selectors.

## Risks & Mitigations
- **Virtualization complexity**: Treat virtualization as progressive enhancement. Fall back to windowed rendering only when necessary.
- **Regression volume**: Pair refactor with thorough component tests and targeted manual QA on key tables.
