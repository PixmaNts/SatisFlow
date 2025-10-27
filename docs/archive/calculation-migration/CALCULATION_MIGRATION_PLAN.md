# Calculation Migration Plan - Implementation Status

_Last updated: 2025-10-27_

## Executive Summary

- The migration objective of eliminating gameplay math from the Vue frontend and delegating every calculation to the Rust engine is **complete**.
- The backend exposes authoritative totals for production lines, power generators, raw inputs, and logistics, and provides `/preview` endpoints for live form feedback.
- Frontend components now consume those fields exclusively; guardrail tests and E2E coverage protect against regressions.
- Remaining work centres on incremental UX polish (for example wiring the raw-input preview endpoint) and documentation refreshes.

---

## Completed Milestones

### Backend
- **Calculated fields in API responses**  
  - `ProductionLineResponse` includes `total_power_consumption`, `total_machines`, `total_somersloop`, `input_rate`, and `output_rate`.  
  - `PowerGeneratorResponse` delivers `total_power_generation`, `total_fuel_consumption`, and nuclear waste metrics.  
  - `RawInputResponse` exposes `power_consumption` for both classic extractors and resource wells.  
  *(See `crates/satisflow-server/src/handlers/factory.rs` for serialization, with calculations sourced from `crates/satisflow-engine/src/models/production_line.rs`, `power_generator.rs`, and `raw_input.rs`.)*

- **Preview endpoints**  
  - `/factories/{id}/production-lines/preview`  
  - `/factories/{id}/power-generators/preview`  
  - `/factories/{id}/raw-inputs/preview`  
  *(Implemented in the factory handlers and surfaced in `frontend/src/api/endpoints.ts:186-235`.)*

- **Tests**  
  - Engine unit tests cover power, fuel, and rate calculations (`crates/satisflow-engine/src/models/**/*_tests.rs`).  
  - API integration tests assert calculated payloads (`crates/satisflow-server/tests/calculated_fields.rs`).

### Frontend
- **Lists use backend totals**  
  - `ProductionLineList.vue`, `PowerGeneratorList.vue`, and `RawInputList.vue` render the backend-provided totals with no local math.
- **Forms rely on previews**  
  - `ProductionLineForm.vue` and `PowerGeneratorForm.vue` call the preview endpoints to show live calculations.
- **Guardrails**  
  - `frontend/src/tests/calculation-violations.test.ts` scans for forbidden math constructs (`Math.pow`, `1.321928`, hardcoded power constants).  
  - `frontend/e2e/calculation-accuracy.spec.ts` verifies UI values against engine output.

---

## Remaining Work and Recommendations

| Priority | Item | Notes |
|----------|------|-------|
| Medium | Wire `factories.preview.rawInput` into `RawInputForm.vue` for live feedback parity. | Endpoint exists and is documented in `frontend/src/api/endpoints.ts`; UI currently waits for save. |
| Low | Refresh public API and architecture docs with the new response fields and preview endpoints. | Update `docs/api.md`, `docs/architecture.md`, and related guides. |
| Low | Continue monitoring for new UI surfaces that could accidentally reintroduce calculations. | Extend `calculation-violations.test.ts` patterns as the engine grows. |

---

## Rollout Status

- Backend and frontend changes landed together on `main` (Big Bang delivery).
- Save file and blueprint formats remain unchanged; no data migrations required.
- No regressions observed in user-facing power or rate displays following the switch to engine-calculated values.

---

## Quality Assurance

- Engine unit tests for production lines, power generators, and resource wells.
- Server integration tests covering calculated fields.
- Frontend unit/integration tests enforcing the "no math in UI" rule.
- Playwright E2E scenarios confirming displayed totals.
- Manual verification recommended whenever the engine gains new machine types or recipes.

---

## Completion Checklist

- [x] API surfaces expose all calculated fields required by the UI.
- [x] Preview endpoints implement production-line, power-generator, and raw-input calculations.
- [x] Vue components consume backend-calculated values exclusively.
- [x] Hardcoded gameplay constants removed from the frontend.
- [x] Automated tests detect any regression to frontend math.
- [ ] Raw input form uses live preview endpoint (optional UX enhancement).
- [ ] Documentation refreshed to reflect the completed migration.

The core migration effort is finished. Future iterations should prioritise keeping documentation current, expanding automated guardrails as new features land, and polishing UX parity across forms.
