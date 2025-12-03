# API Layer and DTO Alignment

## Objective
Harden the Axios client, endpoint wrappers, and TypeScript DTOs so all frontend data flows remain aligned with backend schemas and benefit from consistent error handling.

## Current Gaps
- Axios client lacks interceptors for standard headers, timeouts, and error normalization.
- DTO definitions drift from backend enums/nullable fields, causing runtime casting.
- Components occasionally reach into endpoints directly instead of using typed wrappers.

## Implementation Plan
1. **Client configuration**: Enhance `frontend/src/api/client.ts` with request/response interceptors adding request IDs, authorization, and timeout handling. Integrate retry logic for idempotent GETs.
2. **Error normalization**: Share utilities with the Centralized Error Notification platform to convert Axios errors into typed payloads.
3. **Endpoint wrappers**: Review `frontend/src/api/endpoints.ts` to expose narrow functions returning strongly typed responses. Group by domain (Dashboard, Factory, Logistics) to improve discoverability.
4. **Types audit**: Synchronize `frontend/src/api/types.ts` with backend contracts (enums, nullable fields, numbers vs strings). Remove client-side derived fields in favor of backend-provided values.
5. **Consumers refactor**: Update Pinia stores and composables to use the typed endpoint wrappers exclusively. Eliminate direct Axios usage elsewhere.
6. **Documentation**: Provide guidelines for adding new endpoints, emphasizing type safety and error normalization.

## File Impact
- `frontend/src/api/client.ts`
- `frontend/src/api/endpoints.ts`
- `frontend/src/api/types.ts`
- Shared error utilities (`frontend/src/api/errors.ts`)
- Pinia stores and composables consuming API data

## Testing Strategy
- **Unit**: Vitest tests for client interceptors (headers set, retries triggered), error normalization, and endpoint wrapper typing.
- **Integration**: Mock backend responses to ensure DTO parsing works end-to-end.
- **E2E**: Regression checks to confirm API failures surface to toasts and inline errors as expected.

## Dependencies & Sequencing
- Provides the foundation for Error Notification, Form Validation, and State Management initiatives.
- Coordinate with backend team if schema updates are required; update shared typings if using codegen.

## Architecture & Coding Standards Alignment
- Keeps API interactions encapsulated in dedicated modules, supporting the tiered architecture described in project docs.
- Maintains strict TypeScript types with zero `any` leakage; prefer discriminated unions for error states where useful.
- Enforce consistent `data-test` selectors on components that surface backend data for reliable testing.

## Risks & Mitigations
- **Breaking DTO changes**: Introduce compile-time guards or type tests (e.g., expectTypeOf) to catch mismatches early.
- **Retry storms**: Implement exponential backoff and limit retry attempts to avoid overwhelming backend services.
