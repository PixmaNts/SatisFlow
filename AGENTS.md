# PROJECT KNOWLEDGE BASE

**Generated:** 2026-04-06
**Commit:** aaadc82
**Branch:** main

## OVERVIEW

Satisfactory factory production chain tracker and optimizer. Rust workspace (engine lib + Axum REST server) with a Vue 3 + TypeScript SPA frontend. Backend is the single source of truth for ALL game calculations.

## STRUCTURE

```
./
├── crates/
│   ├── satisflow-engine/    # Core domain logic (pure lib, no I/O deps)
│   └── satisflow-server/    # Axum REST API + Docker deployment
├── frontend/                # Vue 3 + Vite SPA (pnpm)
├── features/                # Feature planning docs (13 .md)
└── icons/                   # Game item icons (duplicated in frontend/public/icons)
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add a game item/recipe | `crates/satisflow-engine/src/models/items.rs`, `recipes.rs` | Also update `frontend/src/api/types.ts` Item union |
| Add/edit API endpoint | `crates/satisflow-server/src/handlers/` | Register route in `main.rs` |
| Add/edit frontend page | `frontend/src/views/` | Add route in `frontend/src/router/index.ts` |
| Add UI component | `frontend/src/components/ui/` | Export from `index.ts` barrel |
| Add composable | `frontend/src/composables/` | `useXxx` naming convention |
| Add Pinia store | `frontend/src/stores/` | Export from `index.ts` barrel |
| Fix calculation bug | `crates/satisflow-engine/src/models/` | NEVER fix in frontend |
| Save/load format | `crates/satisflow-engine/src/lib.rs` | Version-gated via `version.rs` |
| Blueprint templates | `crates/satisflow-engine/src/models/production_line.rs` | ProductionLineBlueprint |
| Docker/deploy config | `crates/satisflow-server/` | Dockerfile, docker-compose.yml, deploy.sh |
| Test a handler | `crates/satisflow-server/tests/` | Shared helpers in `tests/common/mod.rs` |
| Test frontend | `frontend/src/**/__tests__/` | MSW for API mocking |

## CONVENTIONS

- **No semicolons** in frontend (Prettier config: `semi: false`)
- **Single quotes** for JS/TS strings
- **2-space indent**, 100 char max line width
- **`@` alias** → `frontend/src/` in both TS and Vite configs
- **Path alias**: `import { X } from '@/stores'` (barrel exports via `index.ts`)
- **Vue components**: `<script setup lang="ts">` Composition API only
- **Pinia stores**: Composition API style `defineStore('name', () => {...})`
- **Rust edition 2021**, workspace shared deps
- **pnpm** for frontend, **cargo** for backend
- **Node** `^20.19.0 || >=22.12.0`

## ANTI-PATTERNS (THIS PROJECT)

- **NEVER calculate game values in frontend** — no `Math.pow(oc, 1.321928)`, no hardcoded base power values, no extraction rate formulas. All calculations must come from backend API responses.
- **NEVER remove/change Rust struct fields without migration** — breaks save file compatibility. See `crates/MIGRATION-STRATEGY.md`.
- **NEVER skip save version numbers** — always increment sequentially.
- **NEVER use `.unwrap()` in production Rust handlers** — use `?` or `.ok_or()`.
- **Components must NOT call API directly** — route through Pinia store actions.

## COMMANDS

```bash
# Frontend (from frontend/)
pnpm install              # Install deps
pnpm dev                  # Dev server (proxies /api → localhost:3000)
pnpm build                # Type-check + production build
pnpm test                 # Vitest unit tests
pnpm test:coverage        # Tests with 70% coverage thresholds
pnpm lint                 # ESLint --fix
pnpm format               # Prettier

# Backend (from repo root)
cargo build                          # Build all workspace members
cargo build --release                # Release build (LTO enabled)
cargo test                           # Run all tests
cargo run --package satisflow-server # Start API server (port 3000)

# Docker (from crates/satisflow-server/)
docker-compose up -d       # Production deploy
./deploy.sh run            # Alternative deploy script
```

## NOTES

- **No CI/CD pipelines** — manual deployment via `deploy.sh`
- **Icons duplicated** — `./icons/` and `frontend/public/icons/` contain same files
- **E2E tests removed** — backend integration tests verify calculations instead
- **Frontend currently dark-theme only** — applied in `main.ts` before mount
- **Vitest coverage threshold**: 70% for branches/functions/lines/statements
- **Game data cache**: `gameData` store caches for 5 minutes
- **API proxy**: Vite dev server proxies `/api` → `http://localhost:3000`
