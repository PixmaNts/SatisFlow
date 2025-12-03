# Design System and Theming Modernization

## Objective
Establish token-driven theming with light and dark modes while unifying foundational UI components so every screen consumes the same design language.

## Current Gaps
- Colors and spacing are duplicated across components and hard-coded, blocking dark mode.
- Base components (buttons, inputs, toasts, modals, tabs) do not consistently expose accessibility hooks or theme props.
- Animation timings, easing, and micro-interactions are scattered or missing.

## Implementation Plan
1. **Token sweep**: Consolidate all color, typography, spacing, radii, and shadow tokens inside `frontend/src/assets/styles/variables.css`. Add paired light/dark variables using HSL values for easier adjustments.
2. **Motion primitives**: Populate `frontend/src/assets/styles/transitions.css` and `frontend/src/assets/styles/micro-interactions.css` with standard durations, easing curves, and interaction patterns (focus, press, toast entry).
3. **Theme management**: Create a composable (e.g., `useTheme`) that reads the system preference, persists the selection in the preferences store/localStorage, and toggles a `data-theme` attribute on `document.documentElement`.
4. **UI kit extraction**: Refactor components in `frontend/src/components/ui/` to consume tokens only (no hard-coded colors) and surface required props/events for accessibility (`aria-*`, `data-test`). Prioritize `Button`, `Input`, `Select`, `Modal`, `Tabs`, `ToastContainer`, `Skeleton`.
5. **Global entry**: Ensure `frontend/src/App.vue` (or main layout shell) applies theme classes and provides CSS resets that leverage tokens.
6. **Docs + guardrails**: Update project documentation to explain token usage, theming rules, and how to request new tokens. Add lint/stylelint checks if available.

## File Impact
- `frontend/src/assets/styles/variables.css`
- `frontend/src/assets/styles/transitions.css`
- `frontend/src/assets/styles/micro-interactions.css`
- `frontend/src/composables/useTheme.ts` (new)
- `frontend/src/stores/preferencesStore.ts` (or equivalent)
- Core UI components under `frontend/src/components/ui/`
- `frontend/src/App.vue`, `frontend/src/router/index.ts` for theme persistence hooks

## Testing Strategy
- **Unit**: Vitest coverage for `useTheme` composable (initial preference, toggling, persistence) and any theme-aware store actions.
- **Visual/manual**: Verify token substitution on primary screens (Dashboard, Factory, Logistics) in both themes; run contrast checks (WCAG AA) on critical UI states.
- **E2E**: Playwright test toggling the theme, persisting across reloads, and ensuring data-test attributes remain stable.

## Dependencies & Sequencing
- Coordinates with the Accessibility Pass to ensure new tokens meet contrast requirements.
- Relies on Preferences store hardening (see State Management Hardening doc).

## Architecture & Coding Standards Alignment
- Tokens keep presentation concerns out of business logic per the architecture guidelines.
- Composables manage side-effects (DOM/class toggling) instead of components duplicating logic, aligning with the development guide.
- Maintain strict TypeScript types; avoid `any` when augmenting stores or composables.

## Risks & Mitigations
- **Regression risk**: Snapshot or Storybook visual diffs recommended during PR review.
- **Theme drift**: Document token usage and forbid direct color literals via lint rules.
