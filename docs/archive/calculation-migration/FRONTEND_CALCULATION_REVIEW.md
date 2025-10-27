# Frontend Calculation Review - Backend as Single Source of Truth

_Last verified: 2025-10-27_

## Summary
- All production, raw-input, and power generator UIs now consume the calculated fields returned by the Rust engine; no Vue component performs gameplay math locally.
- Live previews for production lines and power generators are delegated to the `/preview` API endpoints, keeping form feedback aligned with the engine.
- Repository safeguards (`frontend/src/tests/calculation-violations.test.ts`, E2E scenarios in `frontend/e2e/calculation-accuracy.spec.ts`) prevent accidental re-introduction of hardcoded constants or formulas.

## Verification Details

| Area | Frontend behaviour | Evidence |
|------|--------------------|----------|
| **Power generator form** | Uses backend preview response for all calculated values (`total_power_generation`, `total_fuel_consumption`). | `frontend/src/components/factory/PowerGeneratorForm.vue:252`, `frontend/src/components/factory/PowerGeneratorForm.vue:295` |
| **Power generator list** | Displays the `total_power_generation` and `total_fuel_consumption` fields returned by the API; no local math remains. | `frontend/src/components/factory/PowerGeneratorList.vue:206`?213 |
| **Production line list** | Reads `total_power_consumption` supplied by the backend for both recipe and blueprint lines. | `frontend/src/components/factory/ProductionLineList.vue:318`?320 |
| **Production line form** | Requests previews from `factories.preview.productionLine`, ensuring OC and Somersloop calculations originate in the engine. | `frontend/src/components/factory/ProductionLineForm.vue:330` |
| **Raw input list** | Relies on the engine-supplied `power_consumption` for every extractor/pressurizer configuration. | `frontend/src/components/factory/RawInputList.vue:222`?224 |
| **Raw input form** | No longer carries placeholder math; payload leaves `power_consumption` to be derived server-side after save. | `frontend/src/components/factory/RawInputForm.vue:441`?452 |

## Remaining Watchpoints
- **Raw input previews:** A parity improvement would be to surface the existing `factories.preview.rawInput` endpoint inside `RawInputForm.vue` to mirror the live feedback provided for production lines and power generators.
- **Display-only helpers:** `formatPower`/`formatFuelRate` utilities continue to limit themselves to rendering concerns; keep them free of gameplay formulas.

## Regression Safeguards
- **Static analysis:** `frontend/src/tests/calculation-violations.test.ts` scans the Vue source tree for `Math.pow`, `1.321928`, and other forbidden patterns tied to gameplay math.
- **Documentation guardrails:** `frontend/src/tests/README.md` documents the expectations for zero hardcoded constants in the UI layer.
- **End-to-end validation:** `frontend/e2e/calculation-accuracy.spec.ts` compares UI output with authoritative engine values for representative production lines, generators, and resource wells.

## Compliance Checklist
- [x] No hardcoded base power or fuel rates in frontend components.
- [x] No usage of `Math.pow(*, 1.321928)` or similar gameplay formulas in Vue/TypeScript code.
- [x] All calculation previews routed through backend endpoints.
- [x] Guardrail tests in place to detect regressions.

The frontend now behaves strictly as a presentation layer for engine-derived data. Ongoing verification should focus on keeping preview endpoints wired up for new forms and maintaining the automated checks that forbid gameplay math in the UI.

