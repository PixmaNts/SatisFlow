# Information Architecture and Navigation Refresh

## Objective
Align primary navigation, in-view filters, and layout affordances with the refactoring goals so users can reach key workflows quickly and persist their context.

## Current Gaps
- Main navigation lacks consistent active/focus styles, confusing keyboard users.
- Dashboard filters miss factory and production-group controls, forcing manual scanning.
- Factory and Logistics views do not persist tab/filter state between visits.

## Implementation Plan
1. **Audit routes**: Confirm route meta definitions in `frontend/src/router/index.ts` align with Dashboard, Factory, and Logistics sections. Add lazy-loaded route chunks where missing.
2. **MainNav overhaul**: Update `frontend/src/components/layout/MainNav.vue` to consume design tokens, expose visible focus states, and emit navigation analytics if required. Ensure active route detection uses `useRoute` + `useRouter` for accuracy.
3. **Filter architecture**:
   - Dashboard: Introduce factory selector and production-group taxonomy filters above the DataTable. Persist selections via the preferences store.
   - Factory view: Validate tab grouping (Production, Raw Input, Power). Persist active tab per factory using store-level state keyed by factory ID.
   - Logistics: Clarify "attach to existing" vs "new line" flows with descriptive headers and action buttons.
4. **State persistence**: Extend the preferences store to remember the last active navigation section, dashboard filters, and per-factory tab state.
5. **Responsive layout**: Ensure filter bars collapse gracefully on smaller breakpoints without losing functionality; leverage CSS grid/flex utilities from the design system.
6. **Documentation**: Update any onboarding or UX docs to highlight new navigation behaviors and shortcuts.

## File Impact
- `frontend/src/router/index.ts`
- `frontend/src/components/layout/MainNav.vue`
- Dashboard filter components under `frontend/src/views/dashboard/`
- Factory tabs and related components in `frontend/src/views/factory/`
- Logistics editors in `frontend/src/views/logistics/`
- `frontend/src/stores/preferencesStore.ts`

## Testing Strategy
- **Unit**: Verify preferences store actions for persisting filters/tabs using Vitest.
- **E2E**: Playwright scenarios covering navigation via keyboard, filter persistence after reload, and Logistics branching flows.
- **Accessibility**: Manual focus order validation and screen reader announcements for navigation landmarks.

## Dependencies & Sequencing
- Requires design tokens/focus styles from the Design System modernization.
- Preferences persistence depends on State Management Hardening updates.
- Dashboard filter parity ties into the DataTable overhaul for consistent sorting/filter behavior.

## Architecture & Coding Standards Alignment
- Navigation and filter logic remains within views/stores; no direct API access from components, matching the architectural separation rules.
- Persisted state uses typed Pinia stores with strict TypeScript definitions.
- All new components must expose `data-test` selectors per testing guidelines.

## Risks & Mitigations
- **State drift**: Implement store versioning/migration to avoid stale preferences.
- **Keyboard regressions**: Include accessibility smoke tests in CI to catch focus issues early.
