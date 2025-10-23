# Backend Implementation Plan

## Gaps to Close

### 1. Factory Sub-Resources

- No REST coverage for production lines, raw inputs, or power generators under `/api/factories/{id}/…`.
- Engine already exposes IDs and collections, but `src/handlers/factory.rs` never mutates them.
- **Required work**
  - Add request/response DTOs matching the contract in `.kilocode/rules/memory-bank/api-and-testing.md` (create/update/delete for each resource).
  - Extend `Router` definitions to include nested routes:
    - `POST /api/factories/{id}/production-lines`
    - `PUT /api/factories/{id}/production-lines/{line_id}`
    - `DELETE /api/factories/{id}/production-lines/{line_id}`
    - Repeat for `raw-inputs` and `power-generators`.
  - Implement handlers that:
    - Validate payloads (OC ranges, Somersloop caps, extractor compatibility, etc.).
    - Call into `SatisflowEngine` mutators (create, upsert, remove) and recalculate inventory/power on success.
  - Update error handling to keep `{status,error}` responses consistent.

### 2. Logistics Update Flow

- `src/handlers/logistics.rs` currently covers GET/POST/DELETE only; the UI needs to edit existing lines.
- **Required work**
  - Add `PUT /api/logistics/{id}` (or PATCH) that:
    - Loads the line from the engine, validates ownership (if factory IDs change) and updated transport configuration.
    - Reuses the existing builder helpers (`create_transport_from_request`) with minimal duplication.
  - Ensure transport identifiers (bus/train IDs, wagon IDs) remain stable where possible or emit new ones with both old/new IDs returned.

### 3. Persistence Hooks

- Engine serialization exists but the server never writes to disk; the brief calls for JSON save/load.
- **Required work**
  - Introduce `state::AppState` helpers to persist on mutate (`save(&self)`) and load during startup.
  - Wire CLI/env config to choose the save path (with tests using a temp dir).

### 4. Validation & UUIDs

- Sequential IDs were vulnerable to collisions; runtime identifiers now use UUIDs across factories, logistics, and nested resources.
- **Remaining work**
  - Extend validation: factory name uniqueness, circular logistics detection, overflow/underflow warnings.
  - Ensure ID persistence across save/load hooks once the storage layer lands.

### 5. Blueprint Import/Export

- API lacks endpoints for blueprint CRUD even though the engine supports it.
- **Required work**
  - Surface:
    - `POST /api/factories/{id}/blueprints` (import)
    - `GET /api/blueprints/:id` and `GET /api/blueprints` (catalog)
    - `DELETE /api/blueprints/:id`
  - Implement serialization/deserialization for nested production lines in handlers.

### 6. Error Surface & Telemetry

- Current errors bubble up as text only; we lack structured logging/metrics for new APIs.
- **Required work**
  - Standardise logging (`tracing`) inside each new handler and emit structured context.
  - Add rate-limited audit logs for create/update/delete actions.

## Testing Roadmap

- Extend `tests/common/test_data.rs` with builders for production lines, raw inputs, power generators, and blueprint payloads.
- Add integration modules mirroring existing suites:
  - `factories_production_lines.rs` (CRUD happy path + validation failures).
  - `factories_raw_inputs.rs` (pressurizer configs, extractor purity validation).
  - `factories_power_generators.rs` (overclock, waste tracking).
  - `logistics_update.rs` (edit flows per transport type).
  - `blueprints.rs` (import/export, nested validation).
- Ensure cascade tests cover deleting factories with attached production lines/raw inputs/generators/logistics.
- Re-run the full suite via `cargo test` in CI after each major addition.

## Sequencing Suggestions

1. Implement factory sub-resource endpoints (CRUD + tests) since the frontend blocks on them.
2. Add logistics update support to unlock editing UX.
3. Introduce persistence hooks so UUID assignments survive restarts before layering blueprints.
4. Layer in blueprint endpoints and validation enhancements.
5. Finalise with persistence integration tests and telemetry/nice-to-haves.
