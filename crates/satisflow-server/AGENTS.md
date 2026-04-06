# SATISFLOW SERVER

Axum-based REST API server exposing the engine. Runs on port 3000 by default.

## STRUCTURE

```
src/
├── main.rs        # Server bootstrap (routes, CORS, tracing, graceful shutdown)
├── lib.rs         # Re-exports AppState, AppError, Result
├── state.rs       # AppState wraps Arc<RwLock<SatisflowEngine>>
├── error.rs       # AppError enum (NotFound, BadRequest, Internal, Serialization, Engine, Validation)
└── handlers/
    ├── mod.rs              # Handler module declarations
    ├── factory.rs          # Factory CRUD + production lines + raw inputs + power generators
    ├── logistics.rs        # Logistics CRUD (transport dispatch by type)
    ├── dashboard.rs        # Summary, item balances, power stats
    ├── game_data.rs        # Static data: recipes, items, machines, extractor compatibility
    ├── save_load.rs        # Save/load engine state as JSON
    ├── blueprint.rs        # Blueprint import/export
    └── blueprint_templates.rs  # Template library CRUD + instantiate into factory

tests/
├── common/mod.rs          # Test server spawn, HTTP client, data builders, assertions
├── api_tests.rs           # Full API integration tests (1365 lines)
└── calculated_fields.rs   # Preview/calculation endpoint tests (1631 lines)

Dockerfile, docker-compose.yml, deploy.sh, .env.example  # Deployment
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add an API endpoint | `handlers/` + `main.rs` | Add handler fn, register `.nest()` route |
| Change API response shape | `handlers/*.rs` | Response structs are defined inline in handlers |
| Change error handling | `error.rs` | AppError → HTTP status mapping |
| Add middleware | `main.rs` | Tower layers (CORS, tracing already configured) |
| Fix a handler bug | `handlers/factory.rs` | Largest handler at 1052 lines |
| Add integration test | `tests/` | Use `common/mod.rs` helpers: `create_test_server()`, `create_test_client()` |
| Configure deployment | `.env.example`, `docker-compose.yml` | PORT, HOST, RUST_LOG, CORS_ORIGINS, ENVIRONMENT |

## API ROUTES

```
GET  /health                                    # Health check
/api/factories          → factory::routes()     # Factory CRUD
/api/logistics          → logistics::routes()    # Logistics CRUD
/api/dashboard          → dashboard::routes()    # Summary & stats
/api/game-data          → game_data::routes()    # Static game data
/api                    → save_load::routes()    # Save/load state
/api                    → blueprint_templates    # Template library
/api                    → blueprint              # Blueprint import/export
```

## CONVENTIONS

- **AppState** = `Arc<RwLock<SatisflowEngine>>` — all handlers share one engine instance
- **Handler pattern**: extract `State`, JSON path/query params, call engine method, return `Json` response
- **Error mapping**: `AppError` implements `IntoResponse` — handlers return `Result<Json<T>, AppError>`
- **Logging**: `tracing` crate — structured JSON in production, pretty-print in dev
- **Environment vars**: loaded via `dotenv` — see `.env.example` for all options

## ANTI-PATTERNS

- **NEVER** use `.unwrap()` in handlers — use `?` or `.ok_or(AppError::...)`
- **NEVER** mutate engine state without acquiring write lock
- **NEVER** hardcode game constants — delegate to engine crate
- **NEVER** add I/O dependencies — keep that in server crate only
