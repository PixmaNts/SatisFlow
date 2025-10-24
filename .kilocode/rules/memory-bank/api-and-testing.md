# API Contract and Testing Guide

**Last Updated**: 2025-10-22

This document provides the complete API contract, testing infrastructure, and development workflow for the Satisfflow backend server. All information is self-contained in this document.

## API Overview

### Base Configuration

- **Base URL**: `http://localhost:3000/api`
- **Content-Type**: `application/json`
- **CORS**: Configurable (development: all origins, production: restricted)
- **Authentication**: None (planned for future)
- **Health Check**: `GET /health`

### Error Response Format

All API errors follow a consistent structure:

```json
{
  "error": "Human-readable error message",
  "status": 400
}
```

### HTTP Status Codes

- `200 OK` - Successful request
- `201 Created` - Resource successfully created
- `204 No Content` - Successful deletion
- `400 Bad Request` - Invalid request data
- `404 Not Found` - Resource not found
- `409 Conflict` - Resource conflict
- `500 Internal Server Error` - Server error

## Complete Type Definitions

### Item Types

Items are represented as strings matching Rust enum variants:

```
"IronOre", "CopperOre", "IronIngot", "CopperIngot", "IronPlate", "CopperSheet",
"Wire", "Cable", "Concrete", "Coal", "Biomass", "Fuel", "Turbofuel",
"CrudeOil", "Water", "NitrogenGas", "UraniumFuelRod", "UraniumWaste"
```

### Factory Response

```json
{
  "id": 1,
  "name": "Iron Processing",
  "description": "Main iron processing facility",
  "notes": null,
  "production_lines": [
    {
      "ProductionLineRecipe": {
        "id": 1,
        "name": "Iron Ingot Production",
        "description": "Basic iron ingot production",
        "recipe": "IronIngot",
        "machine_groups": [
          {
            "number_of_machine": 4,
            "oc_value": 100.0,
            "somersloop": 0
          }
        ]
      }
    }
  ],
  "raw_inputs": [
    {
      "id": 1,
      "extractor_type": "MinerMk2",
      "item": "IronOre",
      "purity": "Normal",
      "quantity_per_min": 120.0,
      "pressurizer": null,
      "extractors": []
    }
  ],
  "power_generators": [
    {
      "id": 1,
      "generator_type": "Coal",
      "fuel_type": "Coal",
      "groups": [
        {
          "number_of_generators": 5,
          "clock_speed": 150.0
        }
      ]
    }
  ],
  "items": [
    {
      "item": "IronIngot",
      "quantity": 120.0
    },
    {
      "item": "IronOre",
      "quantity": -120.0
    }
  ],
  "total_power_consumption": 16.0,
  "total_power_generation": 562.5,
  "power_balance": 546.5
}
```

### Production Line Response (Tagged Union)

Production lines use a tagged union format with `#[serde(tag)]`:

**Recipe Variant**:

```json
{
  "ProductionLineRecipe": {
    "id": 1,
    "name": "Iron Ingot Production",
    "description": "Basic iron ingot production",
    "recipe": "IronIngot",
    "machine_groups": [
      {
        "number_of_machine": 4,
        "oc_value": 100.0,
        "somersloop": 0
      }
    ]
  }
}
```

**Blueprint Variant**:

```json
{
  "ProductionLineBlueprint": {
    "id": 2,
    "name": "Heavy Modular Frame Production",
    "description": "Complete heavy frame production",
    "production_lines": [
      {
        "id": 3,
        "name": "Modular Frame Production",
        "description": "Basic modular frame",
        "recipe": "ModularFrame",
        "machine_groups": [
          {
            "number_of_machine": 2,
            "oc_value": 100.0,
            "somersloop": 2
          }
        ]
      }
    ]
  }
}
```

### Logistics Response

```json
{
  "id": 1,
  "from_factory": 1,
  "to_factory": 2,
  "transport_type": "Truck",
  "transport_id": "TRK-1",
  "transport_name": null,
  "transport_details": "TruckTransport { truck_id: 1, item: IronOre, quantity_per_min: 60.0 }",
  "items": [
    {
      "item": "IronOre",
      "quantity_per_min": 60.0
    }
  ],
  "total_quantity_per_min": 60.0
}
```

### Dashboard Responses

#### Summary Response

```json
{
  "total_factories": 3,
  "total_production_lines": 8,
  "total_logistics_lines": 5,
  "total_power_consumption": 245.5,
  "total_power_generation": 1200.0,
  "net_power": 954.5
}
```

#### Item Balance Response

```json
[
  {
    "item": "IronIngot",
    "balance": 120.0,
    "state": "overflow"
  },
  {
    "item": "IronOre",
    "balance": -120.0,
    "state": "underflow"
  },
  {
    "item": "CopperIngot",
    "balance": 0.0,
    "state": "balanced"
  }
]
```

#### Power Statistics Response

```json
{
  "total_generation": 1200.0,
  "total_consumption": 245.5,
  "power_balance": 954.5,
  "has_surplus": true,
  "has_deficit": false,
  "is_balanced": false,
  "factory_stats": [
    {
      "factory_id": 1,
      "factory_name": "Iron Processing",
      "generation": 562.5,
      "consumption": 16.0,
      "balance": 546.5,
      "generator_count": 5,
      "generator_types": ["Coal"]
    }
  ]
}
```

### Game Data Responses

#### Recipe Response

```json
[
  {
    "name": "Iron Ingot",
    "machine": "Smelter",
    "inputs": [
      {
        "item": "IronOre",
        "quantity": 30.0
      }
    ],
    "outputs": [
      {
        "item": "IronIngot",
        "quantity": 30.0
      }
    ]
  }
]
```

#### Item Response

```json
[
  "IronOre",
  "CopperOre",
  "IronIngot",
  "CopperIngot",
  "IronPlate",
  "CopperSheet",
  "Wire",
  "Cable",
  "Concrete",
  "Coal",
  "Biomass",
  "Fuel",
  "Turbofuel",
  "CrudeOil",
  "Water",
  "NitrogenGas",
  "UraniumFuelRod",
  "UraniumWaste"
]
```

#### Machine Response

```json
[
  {
    "name": "Constructor",
    "base_power": 4.0,
    "max_somersloop": 1
  },
  {
    "name": "Smelter",
    "base_power": 4.0,
    "max_somersloop": 1
  },
  {
    "name": "Assembler",
    "base_power": 16.0,
    "max_somersloop": 2
  }
]
```

## Complete API Endpoints

### Factory Endpoints

#### GET /api/factories

Get all factories with calculated item balances and power statistics.

**Response**: Array of `Factory` objects

**Example**:

```bash
curl http://localhost:3000/api/factories
```

#### GET /api/factories/:id

Get a specific factory by ID.

**Parameters**:

- `id` (path): Factory ID

**Response**: `Factory` object

**Example**:

```bash
curl http://localhost:3000/api/factories/1
```

**Error Response** (404):

```json
{
  "error": "Factory with id 999 not found",
  "status": 404
}
```

#### POST /api/factories

Create a new factory.

**Request Body**:

```json
{
  "name": "New Factory",
  "description": "Factory description",
  "notes": "Optional notes"
}
```

**Response**: `Factory` object with HTTP 201

**Error Response** (400):

```json
{
  "error": "Factory name cannot be empty",
  "status": 400
}
```

#### PUT /api/factories/:id

Update an existing factory.

**Parameters**:

- `id` (path): Factory ID

**Request Body**:

```json
{
  "name": "Updated Factory Name",
  "description": "Updated description",
  "notes": "Updated notes"
}
```

**Response**: `Factory` object

**Error Response** (404):

```json
{
  "error": "Factory with id 999 not found",
  "status": 404
}
```

#### DELETE /api/factories/:id

Delete a factory and all associated logistics lines.

**Parameters**:

- `id` (path): Factory ID

**Response**: HTTP 204 with no body

**Error Response** (404):

```json
{
  "error": "Factory with id 999 not found",
  "status": 404
}
```

### Logistics Endpoints

#### GET /api/logistics

Get all logistics lines.

**Response**: Array of `LogisticsLine` objects

**Example**:

```bash
curl http://localhost:3000/api/logistics
```

#### GET /api/logistics/:id

Get a specific logistics line by ID.

**Parameters**:

- `id` (path): Logistics line ID

**Response**: `LogisticsLine` object

**Error Response** (404):

```json
{
  "error": "Logistics line with id 999 not found",
  "status": 404
}
```

#### POST /api/logistics

Create a new logistics line. Supports four transport types with different payload structures.

**Truck Payload**:

```json
{
  "from_factory": 1,
  "to_factory": 2,
  "transport_type": "Truck",
  "item": "IronOre",
  "quantity_per_min": 60.0,
  "truck_id": "TRK-001"
}
```

**Drone Payload**:

```json
{
  "from_factory": 1,
  "to_factory": 2,
  "transport_type": "Drone",
  "item": "IronPlate",
  "quantity_per_min": 30.0,
  "drone_id": "DRN-003"
}
```

**Bus Payload**:

```json
{
  "from_factory": 1,
  "to_factory": 3,
  "transport_type": "Bus",
  "bus_name": "Main Conveyor Bus",
  "conveyors": [
    {
      "line_id": "CV-001",
      "conveyor_type": "Mk3",
      "item": "IronPlate",
      "quantity_per_min": 120.0
    }
  ],
  "pipelines": [
    {
      "pipeline_id": "PL-001",
      "pipeline_type": "Mk1",
      "item": "Water",
      "quantity_per_min": 240.0
    }
  ]
}
```

**Train Payload**:

```json
{
  "from_factory": 2,
  "to_factory": 4,
  "transport_type": "Train",
  "train_name": "Northern Line",
  "wagons": [
    {
      "wagon_id": "WG-001",
      "wagon_type": "Cargo",
      "item": "CopperSheet",
      "quantity_per_min": 200.0
    },
    {
      "wagon_id": "WG-002",
      "wagon_type": "Fluid",
      "item": "Water",
      "quantity_per_min": 300.0
    }
  ]
}
```

**Supported transport types**: `"Truck"`, `"Drone"`, `"Bus"`, `"Train"`

**Response**: `LogisticsLine` object with HTTP 201

**Error Response** (400):

```json
{
  "error": "Source factory with id 999 does not exist",
  "status": 400
}
```

#### DELETE /api/logistics/:id

Delete a logistics line.

**Parameters**:

- `id` (path): Logistics line ID

**Response**: HTTP 204 with no body

**Error Response** (404):

```json
{
  "error": "Logistics line with id 999 not found",
  "status": 404
}
```

### Dashboard Endpoints

#### GET /api/dashboard/summary

Get global summary statistics.

**Response**: `DashboardSummary` object

**Example**:

```bash
curl http://localhost:3000/api/dashboard/summary
```

#### GET /api/dashboard/items

Get global item balances across all factories.

**Response**: Array of `ItemBalance` objects

**States**: "overflow" (>0), "underflow" (<0), "balanced" (=0)

**Example**:

```bash
curl http://localhost:3000/api/dashboard/items
```

#### GET /api/dashboard/power

Get global power statistics with per-factory breakdown.

**Response**: `PowerStatisticsResponse` object

**Example**:

```bash
curl http://localhost:3000/api/dashboard/power
```

### Game Data Endpoints

#### GET /api/game-data/recipes

Get all available recipes.

**Response**: Array of `RecipeInfo` objects

**Example**:

```bash
curl http://localhost:3000/api/game-data/recipes
```

#### GET /api/game-data/items

Get all available items.

**Response**: Array of item names (strings)

**Example**:

```bash
curl http://localhost:3000/api/game-data/items
```

#### GET /api/game-data/machines

Get all available machine types.

**Response**: Array of `MachineInfo` objects

**Example**:

```bash
curl http://localhost:3000/api/game-data/machines
```

### Health Check

#### GET /health

Health check endpoint.

**Response**:

```json
{
  "status": "healthy",
  "timestamp": "2023-10-20T15:30:45.123Z",
  "service": "satisflow-server"
}
```

## Serialization Notes

### HashMap to Array Conversion

The backend converts HashMaps to Arrays for JSON serialization:

1. **Factory Items**: `HashMap<Item, f32>` → `Vec<ItemBalanceResponse>`
2. **Production Lines**: `HashMap<u64, ProductionLine>` → `Vec<ProductionLineResponse>`
3. **Raw Inputs**: `HashMap<u64, RawInput>` → `Vec<RawInputResponse>`
4. **Power Generators**: `HashMap<u64, PowerGenerator>` → `Vec<PowerGeneratorResponse>`

### Tagged Enum Format

Production lines use Rust's tagged enum serialization:

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ProductionLine {
    ProductionLineRecipe(ProductionLineRecipe),
    ProductionLineBlueprint(ProductionLineBlueprint),
}
```

This results in JSON with the variant name as the key and the variant data as the value.

### Item Serialization

Items are serialized as their Rust enum variant names (string representation):

```rust
// Rust enum
pub enum Item {
    IronOre,
    CopperOre,
    IronIngot,
    // ...
}

// JSON representation
"IronOre"
```

## Transport Type Details

### Transport Capacity Constants

**Conveyors**:

- Mk1: 60 items/min
- Mk2: 120 items/min
- Mk3: 270 items/min
- Mk4: 480 items/min
- Mk5: 780 items/min
- Mk6: 1200 items/min

**Pipelines**:

- Mk1: 300 m³/min
- Mk2: 600 m³/min

### Transport ID Format

Transport IDs follow these patterns:

- Trucks: `"TRK-{id}"`
- Drones: `"DRN-{id}"`
- Buses: `"BUS-{id}"`
- Trains: `"TRN-{id}"`

### Transport Details

The `transport_details` field contains a JSON snapshot of the submitted transport payload and is primarily returned for debugging or auditing purposes.

## Power Calculation Formulas

### Production Line Power

```
Power = Base Power × (1 + somersloop/max_somersloop)² × (clock_speed/100)^1.321928 × number_of_machines
```

### Generator Power

```
Power = Base Power × (clock_speed/100) × number_of_generators
```

### Generator Fuel Consumption

```
Fuel = Base Fuel × fuel_multiplier × (clock_speed/100) × number_of_generators
```

**Important Note**: Power generators overclock differently from power consumers. Their fuel consumption rate is always proportional to the power production of the building. Hence, overclocking a power generator will neither increase nor decrease fuel efficiency.

### Power Balance

- **Positive**: Power surplus (generation > consumption)
- **Negative**: Power deficit (consumption > generation)
- **Zero**: Power balanced

## Item Balance States

Item balances have three states:

- **Overflow** (`> 0`): More items produced than consumed
- **Underflow** (`< 0`): More items consumed than produced (production deficit)
- **Balanced** (`= 0`): Production exactly matches consumption

## Error Handling

All errors follow the consistent format:

```json
{
  "error": "Human-readable error message",
  "status": HTTP_STATUS_CODE
}
```

Common error scenarios:

- Invalid factory/logistics IDs (404)
- Invalid request data (400)
- Empty factory names (400)
- Non-existent source/destination factories (400)
- Internal server errors (500)

## CORS Configuration

The API supports CORS with configurable origins. In development mode, all origins are allowed. In production, origins are restricted based on the `CORS_ORIGINS` environment variable.

## Environment Variables

- `PORT`: Server port (default: 3000)
- `HOST`: Server host (default: 127.0.0.1)
- `ENVIRONMENT`: Environment mode (development/production)
- `RUST_LOG`: Log level
- `CORS_ORIGINS`: Comma-separated list of allowed origins

## Backend Testing Infrastructure

### Test Structure

**Location**: `crates/satisflow-server/tests/`

```
tests/
├── common/
│   └── mod.rs              # Test utilities and helpers
├── api_tests.rs            # Main API test suite
├── factories_lifecycle.rs  # Factory CRUD operation tests
├── logistics_transports.rs # All transport type tests
├── test_runner.rs          # Test execution utility
└── README.md               # Test documentation
```

### Test Coverage

**Total Lines**: 484 lines of comprehensive integration tests

**Coverage Areas**:

- ✅ Factory CRUD operations (create, read, update, delete)
- ✅ Logistics management (all 4 transport types)
- ✅ Dashboard endpoint validation (summary, items, power)
- ✅ Game data endpoint tests (recipes, items, machines)
- ✅ CORS functionality tests (preflight, headers)
- ✅ Error handling tests (404, 400, validation)
- ✅ Concurrent request handling tests

### Test Utilities

**Common Module** (`tests/common/mod.rs`):

- `create_test_server()` - Isolated test server on random port
- `create_test_client()` - Configured reqwest client
- `assert_json_response()` - Parse and validate JSON responses
- `assert_created_response()` - Validate 201 Created responses
- `assert_no_content()` - Validate 204 No Content responses
- Helper functions for test data generation

### Running Tests

**Run All Tests**:

```bash
cd crates/satisflow-server
cargo test
```

**Run Tests with Verbose Output**:

```bash
cargo test -- --nocapture
```

**Run Specific Test Modules**:

```bash
# Factory tests
cargo test --package satisflow-server -- --exact test_factory_crud_operations
cargo test --package satisflow-server -- --exact test_factory_error_cases

# Logistics tests
cargo test --package satisflow-server -- --exact test_logistics_crud_operations
cargo test --package satisflow-server -- --exact test_logistics_error_cases

# Dashboard tests
cargo test --package satisflow-server -- --exact test_dashboard_endpoints

# Game data tests
cargo test --package satisflow-server -- --exact test_game_data_endpoints

# CORS tests
cargo test --package satisflow-server -- --exact test_cors_headers

# Error handling tests
cargo test --package satisflow-server -- --exact test_error_response_format
```

**Using Test Runner**:

```bash
# Run all tests with verbose output
cargo run --bin test_runner verbose

# Run specific test categories
cargo run --bin test_runner factory
cargo run --bin test_runner logistics
cargo run --bin test_runner dashboard
cargo run --bin test_runner game_data
cargo run --bin test_runner cors
cargo run --bin test_runner errors

# Run a specific test
cargo run --bin test_runner single test_health_check
```

### Test Implementation Pattern

```rust
#[tokio::test]
async fn test_new_endpoint() {
    let server = create_test_server().await;
    let client = create_test_client();
    
    // Test success case
    let response = client
        .post(&format!("{}/api/new-endpoint", server.base_url))
        .json(&test_data())
        .send()
        .await
        .expect("Failed to send request");
    
    assert_eq!(response.status(), 201);
    let result: Value = assert_created_response(response).await;
    assert!(result.get("id").is_some());
    
    // Test error cases
    // ...
}
```

### Adding New Tests

When adding new endpoints:

1. Add test data helpers to `tests/common/mod.rs`
2. Create test functions following existing patterns
3. Test both success and error paths
4. Validate response structure and status codes
5. Update test documentation

## Development Workflow

### Starting Development Servers

```bash
# Terminal 1: Backend (Port 3000)
cd crates/satisflow-server
cargo run

# Terminal 2: Frontend (Port 5173)
cd frontend
npm run dev

# Browser: http://localhost:5173
```

### Testing Commands

```bash
# Backend tests
cd crates/satisflow-server
cargo test

# Frontend unit tests
cd frontend
npm run test
npm run test:watch

# Frontend E2E tests
npm run test:e2e
