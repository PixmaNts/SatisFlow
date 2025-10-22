# Satisflow API Contract

This document provides a comprehensive API contract for the Satisflow backend server, accurately reflecting the implemented handlers and response formats.

## API Overview

- **Base URL**: `http://localhost:3000/api`
- **Authentication**: None (for now)
- **Content-Type**: `application/json`
- **Error Format**: Consistent error response structure

### Error Response Format

All API errors follow this consistent format:

```json
{
  "error": "Human-readable error message",
  "status": 400
}
```

### HTTP Status Codes

- `200 OK` - Successful request
- `201 Created` - Resource successfully created
- `204 No Content` - Successful deletion (no response body)
- `400 Bad Request` - Invalid request data
- `404 Not Found` - Resource not found
- `409 Conflict` - Resource conflict
- `500 Internal Server Error` - Server error

## Type Definitions

### Item Types

Items are represented as strings matching Rust enum variants:

```json
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

OR

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
    "max_sommersloop": 1
  },
  {
    "name": "Smelter",
    "base_power": 4.0,
    "max_sommersloop": 1
  },
  {
    "name": "Assembler",
    "base_power": 16.0,
    "max_sommersloop": 2
  }
]
```

## API Endpoints

### Factory Endpoints

#### GET /api/factories

Get all factories with their calculated item balances and power statistics.

**Response**: Array of `FactoryResponse`

**Example**:
```bash
curl -X GET http://localhost:3000/api/factories
```

#### GET /api/factories/:id

Get a specific factory by ID.

**Parameters**:
- `id` (path): Factory ID

**Response**: `FactoryResponse`

**Example**:
```bash
curl -X GET http://localhost:3000/api/factories/1
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

**Response**: `FactoryResponse` with HTTP 201

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

**Response**: `FactoryResponse`

**Error Response** (404):
```json
{
  "error": "Factory with id 999 not found",
  "status": 404
}
```

#### DELETE /api/factories/:id

Delete a factory.

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

**Response**: Array of `LogisticsResponse`

**Example**:
```bash
curl -X GET http://localhost:3000/api/logistics
```

#### GET /api/logistics/:id

Get a specific logistics line by ID.

**Parameters**:
- `id` (path): Logistics line ID

**Response**: `LogisticsResponse`

**Error Response** (404):
```json
{
  "error": "Logistics line with id 999 not found",
  "status": 404
}
```

#### POST /api/logistics

Create a new logistics line.

**Request Body Examples**:

*Truck*
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

*Drone*
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

*Bus*
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

*Train*
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

**Response**: `LogisticsResponse` with HTTP 201

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

**Response**: `DashboardSummary`

**Example**:
```bash
curl -X GET http://localhost:3000/api/dashboard/summary
```

#### GET /api/dashboard/items

Get global item balances across all factories.

**Response**: Array of `ItemBalance`

**Example**:
```bash
curl -X GET http://localhost:3000/api/dashboard/items
```

#### GET /api/dashboard/power

Get global power statistics.

**Response**: `PowerStatisticsResponse`

**Example**:
```bash
curl -X GET http://localhost:3000/api/dashboard/power
```

### Game Data Endpoints

#### GET /api/game-data/recipes

Get all available recipes.

**Response**: Array of `RecipeInfo`

**Example**:
```bash
curl -X GET http://localhost:3000/api/game-data/recipes
```

#### GET /api/game-data/items

Get all available items.

**Response**: Array of item names (strings)

**Example**:
```bash
curl -X GET http://localhost:3000/api/game-data/items
```

#### GET /api/game-data/machines

Get all available machine types.

**Response**: Array of `MachineInfo`

**Example**:
```bash
curl -X GET http://localhost:3000/api/game-data/machines
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

### Date/Time Formats

- Timestamps use ISO 8601 format: `"2023-10-20T15:30:45.123Z"`
- No date/time fields are currently used in the main data models

## Transport Type Details

### Transport Types

The API supports four transport types, each with different serialization:

1. **Truck**: Single item transport
2. **Drone**: Single item transport
3. **Bus**: Multi-item transport (conveyors + pipelines)
4. **Train**: Multi-item transport (cargo + fluid wagons)

### Transport ID Format

Transport IDs follow these patterns:
- Trucks: `"TRK-{id}"`
- Drones: `"DRN-{id}"`
- Buses: `"BUS-{id}"`
- Trains: `"TRN-{id}"`

### Transport Details

The `transport_details` field contains a JSON snapshot of the submitted transport payload and is primarily returned for debugging or auditing purposes.

## Power System Details

### Power Calculation

Power calculations follow these formulas:

**Production Line Power**:
```
Power = Base Power × (1 + sommersloop/max_sommersloop)² × (clock_speed/100)^1.321928 × number_of_machines
```

**Generator Power**:
```
Power = Base Power × (clock_speed/100) × number_of_generators
```

**Generator Fuel Consumption**:
```
Fuel = Base Fuel × fuel_multiplier × (clock_speed/100) × number_of_generators
```

### Power Balance

- **Positive**: Power surplus (generation > consumption)
- **Negative**: Power deficit (consumption > generation)
- **Zero**: Power balanced

## Item Balance States

Item balances have three states:

- **Overflow** (`> 0`): More items produced than consumed
- **Underflow** (`< 0`): More items consumed than produced
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
