# Satisflow Implementation Plan

## Project Overview

Satisflow is a production tracker for Satisfactory players, built with a Rust backend using Axum and a Vue.js frontend. This document provides a comprehensive implementation plan for transitioning from the current engine-only state to a full web application.

## Project Structure

```
Satisflow/
├── docs/                           # This documentation
│   ├── IMPLEMENTATION_PLAN.md     # This file
│   ├── BACKEND_GUIDE.md           # Backend implementation guide
│   └── FRONTEND_GUIDE.md          # Frontend implementation guide
├── crates/
│   ├── satisflow-engine/          # Core Rust engine (existing)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs             # SatisflowEngine main API
│   │       └── models/            # Data models
│   └── satisflow-server/          # Axum web server (new)
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs            # Server entry point
│           ├── state.rs           # Application state
│           ├── error.rs           # Error handling
│           └── handlers/          # API handlers
│               ├── mod.rs
│               ├── factory.rs      # Factory endpoints
│               ├── logistics.rs    # Logistics endpoints
│               └── dashboard.rs    # Dashboard endpoints
├── frontend/                       # Vue.js frontend (new)
│   ├── package.json
│   ├── vite.config.ts
│   ├── tsconfig.json
│   ├── index.html
│   └── src/
│       ├── main.ts                # App entry point
│       ├── App.vue                # Root component
│       ├── router/
│       │   └── index.ts           # Vue Router config
│       ├── components/
│       │   ├── MainNav.vue        # Navigation bar
│       │   ├── DashboardView.vue  # Dashboard view
│       │   ├── FactoryView.vue    # Factory view
│       │   └── LogisticsView.vue  # Logistics view
│       ├── api/
│       │   ├── client.ts          # API client
│       │   └── types.ts           # TypeScript types
│       └── styles/
│           └── main.css           # Global styles
├── Cargo.toml                      # Workspace configuration
└── README.md                       # Project README
```

## Implementation Timeline

### Phase 1: Backend Development (Week 1-2)

**Goal**: Complete Axum server with full CRUD API

**Week 1: Foundation**
- [ ] Set up `satisflow-server` crate with Axum dependencies
- [ ] Implement basic server structure with CORS and logging
- [ ] Create application state management
- [ ] Add error handling middleware
- [ ] Implement factory CRUD endpoints
- [ ] Add basic testing with curl

**Week 2: Complete API**
- [ ] Implement logistics endpoints
- [ ] Implement dashboard endpoints
- [ ] Add engine modifications (delete methods, get_all)
- [ ] Implement request validation
- [ ] Add comprehensive API testing
- [ ] Set up production configuration

### Phase 2: Frontend Development (Week 3-4)

**Goal**: Complete Vue.js frontend with all views

**Week 3: Foundation**
- [ ] Set up Vue.js + TypeScript + Vite project
- [ ] Configure API client and type definitions
- [ ] Implement router and navigation
- [ ] Create basic component structure
- [ ] Implement Dashboard view with filters
- [ ] Add responsive design

**Week 4: Complete Views**
- [ ] Implement Factory view with sub-tabs
- [ ] Implement Logistics view with filtering
- [ ] Add form validation and error handling
- [ ] Implement create/edit modals
- [ ] Add loading states and transitions
- [ ] Test all user workflows

### Phase 3: Polish & Production (Week 5+)

**Goal**: Production-ready application with deployment

**Week 5: Integration**
- [ ] End-to-end integration testing
- [ ] Performance optimization
- [ ] Error handling improvements
- [ ] Accessibility improvements
- [ ] Browser compatibility testing

**Week 6: Production**
- [ ] Docker containerization
- [ ] Deployment configuration
- [ ] Monitoring and logging setup
- [ ] Documentation updates
- [ ] User guide creation

## Code Review Findings (2025-02-17)

### Frontend
- `frontend/src/components/factory/FactorySelector.vue`: the `<select>` binding leaves `selectedFactoryId` as a string, so `setCurrentFactory` stores `'id'` and `currentFactory` never resolves; the Factory view stays in its empty state.
- `frontend/src/views/FactoryView.vue`: two identical watchers on the active factory run immediately, issuing duplicate `fetchById` calls and re-saving preferences on every change.
- `frontend/src/stores/dashboard.ts`: `fetchAllData` awaits helper functions that each toggle `loading`/`error`, so one failure is cleared by the next request and the spinner flickers while other calls are still pending.

### Server
- `crates/satisflow-server/src/handlers/factory.rs`: the API accepts `notes` but never persists or returns them, so the value is dropped on create/update.
- `crates/satisflow-server/src/handlers/logistics.rs`: `create_logistics` ignores the request payload, hard-codes truck/drone transports with fixed IDs/items, and rejects buses/trains that the frontend exposes.

### Remediations (2025-02-17)
- Factory notes now persist end-to-end: the engine model exposes a `notes` field, handlers hydrate it on create/update, and MSW mocks plus API tests cover the behaviour.
- Duplicate factory fetches were eliminated and selector IDs stay numeric, so the UI correctly resolves the selected factory.
- Dashboard loading/error handling uses a shared pending-request counter, preventing oscillating spinners and swallowed errors during parallel fetches.
- Logistics creation accepts full payloads for trucks, drones, buses, and trains; both the backend handler and frontend request types were updated and documented, with tests covering the supported variants.

## Quick Start Commands

### Backend Development

```bash
# Navigate to project root
cd Satisflow

# Start backend server in development mode
cd crates/satisflow-server
cargo run

# Start backend with auto-reload
cargo watch -x run

# Run tests
cargo test

# Build for production
cargo build --release
```

### Frontend Development

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev

# Run type checking
npm run type-check

# Run tests
npm run test

# Build for production
npm run build
```

### Full Development Workflow

```bash
# Terminal 1: Backend
cd crates/satisflow-server
cargo run

# Terminal 2: Frontend
cd frontend
npm run dev

# Browser: http://localhost:5173 (Vite dev server)
# API: http://localhost:3000 (Axum server)
```

## API Endpoints Overview

### Factory Endpoints

```
GET    /api/factories              # Get all factories
GET    /api/factories/:id          # Get factory by ID
POST   /api/factories              # Create new factory
PUT    /api/factories/:id          # Update factory
DELETE /api/factories/:id          # Delete factory

GET    /api/factories/:id/production-lines     # Get production lines
POST   /api/factories/:id/production-lines     # Create production line
PUT    /api/factories/:id/production-lines/:pl # Update production line
DELETE /api/factories/:id/production-lines/:pl # Delete production line

GET    /api/factories/:id/raw-inputs          # Get raw inputs
POST   /api/factories/:id/raw-inputs          # Create raw input
PUT    /api/factories/:id/raw-inputs/:ri      # Update raw input
DELETE /api/factories/:id/raw-inputs/:ri      # Delete raw input

GET    /api/factories/:id/power-generators    # Get power generators
POST   /api/factories/:id/power-generators    # Create power generator
PUT    /api/factories/:id/power-generators/:pg # Update power generator
DELETE /api/factories/:id/power-generators/:pg # Delete power generator
```

### Logistics Endpoints

```
GET    /api/logistics              # Get all logistics lines
GET    /api/logistics/:id          # Get logistics line by ID
POST   /api/logistics              # Create logistics line
PUT    /api/logistics/:id          # Update logistics line
DELETE /api/logistics/:id          # Delete logistics line
```

### Dashboard Endpoints

```
GET    /api/dashboard/summary      # Global summary
GET    /api/dashboard/items        # Global item balances
GET    /api/dashboard/power        # Global power statistics
```

### Game Data Endpoints

```
GET    /api/game-data/recipes      # All available recipes
GET    /api/game-data/items        # All available items
GET    /api/game-data/machines     # All machine types
```

## Development Workflow Examples

### Example 1: Adding a New Factory

```bash
# Backend: Test with curl
curl -X POST http://localhost:3000/api/factories \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Iron Processing Plant",
    "description": "Main iron production facility"
  }'

# Frontend: Use API client
import { apiClient } from '@/api/client';

const factory = await apiClient.post('/api/factories', {
  name: 'Iron Processing Plant',
  description: 'Main iron production facility'
});
```

### Example 2: Adding Production Line

```bash
# Backend: Test with curl
curl -X POST http://localhost:3000/api/factories/1/production-lines \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Iron Ingot Production",
    "recipe": "IronIngot",
    "machine_groups": [
      {
        "machine_type": "Smelter",
        "num_machines": 3,
        "overclock": 100.0,
        "sommersloop": 0
      }
    ]
  }'

# Frontend: Use API client
const productionLine = await apiClient.post(`/api/factories/${factoryId}/production-lines`, {
  name: 'Iron Ingot Production',
  recipe: 'IronIngot',
  machine_groups: [
    {
      machine_type: 'Smelter',
      num_machines: 3,
      overclock: 100.0,
      sommersloop: 0
    }
  ]
});
```

### Example 3: Real-time Dashboard Updates

```typescript
// Frontend: Polling for updates
import { ref, onMounted, onUnmounted } from 'vue';
import { apiClient } from '@/api/client';

const dashboardData = ref(null);
let pollingInterval: number;

onMounted(async () => {
  // Initial load
  dashboardData.value = await apiClient.get('/api/dashboard/summary');
  
  // Set up polling every 5 seconds
  pollingInterval = setInterval(async () => {
    dashboardData.value = await apiClient.get('/api/dashboard/summary');
  }, 5000);
});

onUnmounted(() => {
  clearInterval(pollingInterval);
});
```

## Success Criteria

### Phase 1 Success Criteria

- [ ] All API endpoints implemented and tested
- [ ] Server can handle concurrent requests
- [ ] Proper error handling and validation
- [ ] CORS configured for frontend access
- [ ] Production-ready logging and configuration
- [ ] 95%+ test coverage for API endpoints

### Phase 2 Success Criteria

- [ ] All three main views implemented (Dashboard, Factory, Logistics)
- [ ] Full CRUD operations for all entities
- [ ] Responsive design works on mobile and desktop
- [ ] Form validation prevents invalid data
- [ ] Loading states and error handling
- [ ] TypeScript type safety throughout

### Phase 3 Success Criteria

- [ ] Application deployed and accessible
- [ ] Performance metrics meet targets (< 200ms API response)
- [ ] Error monitoring and logging in place
- [ ] Documentation complete and accurate
- [ ] User guide for common operations
- [ ] Browser compatibility (Chrome, Firefox, Safari, Edge)

## Technology Stack

### Backend

- **Rust**: Systems programming language
- **Axum**: Web framework built on Tokio
- **Tokio**: Asynchronous runtime
- **Serde**: Serialization/deserialization
- **Tracing**: Structured logging
- **Tower**: Middleware ecosystem

### Frontend

- **Vue.js 3**: Progressive JavaScript framework
- **TypeScript**: Type-safe JavaScript
- **Vite**: Build tool and dev server
- **Vue Router**: Client-side routing
- **Axios**: HTTP client
- **CSS3**: Styling with custom properties

### Development Tools

- **Cargo**: Rust package manager
- **npm**: Node.js package manager
- **Docker**: Containerization
- **Git**: Version control
- **VS Code**: Recommended IDE

## Next Steps

1. **Read BACKEND_GUIDE.md** for detailed backend implementation
2. **Read FRONTEND_GUIDE.md** for detailed frontend implementation
3. **Set up development environment** with Rust and Node.js
4. **Start with Phase 1** backend implementation
5. **Progress through phases** systematically
6. **Test thoroughly** at each milestone

## Additional Resources

- [Axum Documentation](https://docs.rs/axum/)
- [Vue.js Documentation](https://vuejs.org/)
- [TypeScript Documentation](https://www.typescriptlang.org/)
- [Vite Documentation](https://vitejs.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
