# Modal Focus and Keyboard Control Standardization

## Objective
Guarantee every modal in the application provides reliable focus trapping, escape handling, and accessible labelling so keyboard and assistive technologies can operate without barriers.

## Current Gaps
- Some modals do not return focus to the invoking trigger.
- `aria-labelledby`/`aria-describedby` wiring is inconsistent or missing.
- Keyboard traps allow tabbing behind modals or fail to cycle through focusable elements.

## Implementation Plan
1. **Modal core review**: Refactor `frontend/src/components/ui/Modal.vue` to centralize focus management (initial focus, trapping, cleanup) and expose slots/props for headers, body, and footers.
2. **Focus trap utility**: Introduce a helper (e.g., `frontend/src/composables/useFocusTrap.ts`) abstracting focusable element detection, key handling, and restoration.
3. **Escape handling**: Ensure Escape closes the modal by default while allowing optional suppression for nested flows. Provide `onBeforeClose` hooks for validation.
4. **ARIA compliance**: Accept `aria-labelledby`, `aria-describedby`, and `role` props. Mirror IDs through slots to avoid duplicating markup in consumers.
5. **Integration sweep**: Update all modal consumers (Factory editors, Logistics flows, toasts if modal-based) to use the new API and supply accessible labels.
6. **Documentation**: Add usage notes highlighting required props and keyboard expectations.

## File Impact
- `frontend/src/components/ui/Modal.vue`
- `frontend/src/composables/useFocusTrap.ts` (new)
- Modal-consuming components under `frontend/src/components/` and `frontend/src/views/`
- Related unit tests in `frontend/src/tests/`

## Testing Strategy
- **Unit**: Vitest tests for the focus trap composable (tab cycling, shift+tab, escape).
- **Component**: Mount Modal with testing library to assert auto-focus, restoration, and ARIA attributes.
- **E2E**: Playwright scripts verifying modal workflows (create/edit factory assets) using keyboard-only navigation.

## Dependencies & Sequencing
- Draws on design tokens for visual focus rings (Design System doc).
- Complements the Accessibility Pass, which validates focus order across the app.

## Architecture & Coding Standards Alignment
- Encapsulates DOM manipulation in composables, keeping components declarative per development guidelines.
- Strong typing of modal props/events; no `any` usage.
- Provide `data-test` selectors on modal root and action buttons.

## Risks & Mitigations
- **Complex nested modals**: Document stacking rules and provide safeguards against simultaneous focus traps.
- **Legacy consumers**: Offer migration guidance and temporary compatibility props if needed during rollout.
