# Accessibility Compliance Pass

## Objective
Conduct a focused accessibility audit and remediation covering focus management, ARIA labelling, contrast ratios, and keyboard-only workflows across the refactored frontend.

## Current Gaps
- Inconsistent focus indicators and keyboard navigation patterns in tables, forms, and modals.
- Missing or incorrect ARIA attributes on interactive UI components.
- Dark mode theming has not been validated against WCAG AA contrast requirements.

## Implementation Plan
1. **Checklist alignment**: Use the provided A11y quick check as the baseline for all components in `frontend/src/components/ui/`, `frontend/src/components/factory/`, and `frontend/src/views/`.
2. **Focus audit**: Validate focus order and visibility for primary flows (Dashboard, Factory, Logistics). Ensure custom components forward `tabindex` and keyboard events properly.
3. **ARIA labelling**: Add or correct `aria-label`, `aria-labelledby`, `aria-describedby`, and `role` attributes on inputs, buttons, modals, and status regions.
4. **Contrast validation**: Run automated tools (axe, lighthouse) and manual checks to verify both light and dark themes meet WCAG AA. Adjust tokens if necessary.
5. **Keyboard trapping**: Confirm modals/tabs utilize the standardized focus trap (see Modal Focus doc) and there are no keyboard traps.
6. **Documentation**: Record findings and remediation steps; update contributor guidelines with accessibility expectations.

## File Impact
- UI components under `frontend/src/components/ui/`
- Factory/logistics forms under `frontend/src/components/factory/` and `frontend/src/views/`
- Theme tokens in `frontend/src/assets/styles/`
- Testing scripts/config for axe integrations

## Testing Strategy
- **Automated**: Integrate axe-core or similar into component/E2E tests to catch regressions.
- **Manual**: Keyboard-only walkthroughs of critical flows; screen reader spot checks (NVDA/VoiceOver) for modals and tables.
- **Regression**: Capture before/after contrast measurements for tokens.

## Dependencies & Sequencing
- Builds on Design System tokens (especially focus and color variables).
- Relies on Modal Focus standardization and DataTable overhaul for structural improvements.

## Architecture & Coding Standards Alignment
- Accessibility attributes are part of UI layer responsibilities, maintaining separation from business logic.
- Enforce `data-test` selectors while ensuring they do not conflict with ARIA usage.

## Risks & Mitigations
- **Token adjustments**: Coordinate with design to tweak colors without breaking brand alignment.
- **Ongoing compliance**: Add accessibility checks to CI so regressions are caught automatically.
