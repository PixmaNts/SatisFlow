# Satisflow Backend Implementation Guide

## Overview

This guide provides detailed instructions for implementing the Axum web server for the Satisflow project. The backend exposes a REST API that interfaces with the existing SatisflowEngine.

## Prerequisites

- Rust 1.70+ installed
- Understanding of async/await in Rust
- Familiarity with the existing SatisflowEngine codebase

## Project Setup

### 1. Create the Server Crate

Create a new crate in the `crates/` directory:

```bash
cd crates
cargo new satisflow-server
```

### 2. Configure Dependencies

Edit `crates/satisflow-server/Cargo.toml`:

```toml
[package]
name = "satisflow-server"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web framework
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# UUID generation
uuid = { version = "1.0", features = ["v4", "serde"] }

# Engine dependency
satisflow-engine = { path = "../satisflow-engine" }

# Async utilities
futures = "0.3"

# Environment variables
dotenv = "0.15"

[dev-dependencies]
reqwest = { version = "0.11", features = ["json"] }
```

### 3. Update Workspace Configuration

Edit the root `Cargo.toml` to include the new crate:

```toml
[workspace]
members = [
    "crates/satisflow-engine",
    "crates/satisflow-server",
]
```

## Project Structure

```
crates/satisflow-server/src/
├── main.rs              # Server entry point
├── state.rs             # Application state management
├── error.rs             # Error handling
├── lib.rs               # Library exports
└── handlers/            # API handlers
    ├── mod.rs
    ├── factory.rs       # Factory endpoints
    ├── logistics.rs     # Logistics endpoints
    ├── dashboard.rs     # Dashboard endpoints
    └── game_data.rs     # Game data endpoints
```

## Implementation Steps

### 1. Application State (state.rs)

Create the application state that will be shared across all handlers:

```rust
// crates/satisflow-server/src/state.rs
use std::sync::Arc;
use tokio::sync::RwLock;
use satisflow_engine::SatisflowEngine;

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<RwLock<SatisflowEngine>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            engine: Arc::new(RwLock::new(SatisflowEngine::new())),
        }
    }
}
```

### 2. Error Handling (error.rs)

Implement comprehensive error handling:

```rust
// crates/satisflow-server/src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    InternalError(#[from] anyhow::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Engine error: {0}")]
    EngineError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::SerializationError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Serialization error".to_string(),
            ),
            AppError::EngineError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
```

### 3. Main Server (main.rs)

Implement the main server setup:

```rust
// crates/satisflow-server/src/main.rs
use axum::{
    http::Method,
    routing::{get, post, put, delete},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod handlers;
mod state;

use error::Result;
use state::AppState;
use handlers::{factory, logistics, dashboard, game_data};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "satisflow_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Create application state
    let state = AppState::new();

    // Build the application router
    let app = Router::new()
        // API routes
        .nest("/api/factories", factory::routes())
        .nest("/api/logistics", logistics::routes())
        .nest("/api/dashboard", dashboard::routes())
        .nest("/api/game-data", game_data::routes())
        
        // Health check
        .route("/health", get(health_check))
        
        // Global middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                        .allow_headers(Any),
                ),
        )
        .with_state(state);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Satisflow server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
```

### 4. Factory Handlers (handlers/factory.rs)

Implement factory-related endpoints:

```rust
// crates/satisflow-server/src/handlers/factory.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{error::{AppError, Result}, state::AppState};
use satisflow_engine::{
    models::{Factory, ProductionLine, RawInput, PowerGenerator},
    SatisflowEngine,
};

// DTOs for API requests/responses
#[derive(Serialize, Deserialize)]
pub struct CreateFactoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateFactoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct FactoryResponse {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub production_lines: Vec<ProductionLineResponse>,
    pub raw_inputs: Vec<RawInputResponse>,
    pub power_generators: Vec<PowerGeneratorResponse>,
    pub items: HashMap<String, f32>,
}

#[derive(Serialize)]
pub struct ProductionLineResponse {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub production_type: String, // "recipe" or "blueprint"
    // Add other fields as needed
}

#[derive(Serialize)]
pub struct RawInputResponse {
    pub id: u64,
    pub extractor_type: String,
    pub item: String,
    pub purity: Option<String>,
    pub quantity_per_min: f32,
}

#[derive(Serialize)]
pub struct PowerGeneratorResponse {
    pub id: u64,
    pub generator_type: String,
    pub fuel_type: Option<String>,
    pub power_output: f32,
    // Add other fields as needed
}

// Convert engine models to response DTOs
impl From<&Factory> for FactoryResponse {
    fn from(factory: &Factory) -> Self {
        Self {
            id: factory.id,
            name: factory.name.clone(),
            description: factory.description.clone(),
            notes: factory.notes.clone(),
            production_lines: factory.production_lines.values()
                .map(|pl| ProductionLineResponse {
                    id: pl.id(),
                    name: "Production Line".to_string(), // Add name field to engine
                    description: None,
                    production_type: "recipe".to_string(), // Determine from actual type
                })
                .collect(),
            raw_inputs: factory.raw_inputs.values()
                .map(|ri| RawInputResponse {
                    id: ri.id,
                    extractor_type: format!("{:?}", ri.extractor_type),
                    item: format!("{:?}", ri.item),
                    purity: ri.purity.as_ref().map(|p| format!("{:?}", p)),
                    quantity_per_min: ri.quantity_per_min,
                })
                .collect(),
            power_generators: factory.power_generators.values()
                .map(|pg| PowerGeneratorResponse {
                    id: pg.id,
                    generator_type: format!("{:?}", pg.generator_type),
                    fuel_type: pg.fuel_type.as_ref().map(|f| format!("{:?}", f)),
                    power_output: pg.total_power_output(),
                })
                .collect(),
            items: factory.items.iter()
                .map(|(k, v)| (format!("{:?}", k), *v))
                .collect(),
        }
    }
}

// API handlers
pub async fn get_factories(
    State(state): State<AppState>,
) -> Result<Json<Vec<FactoryResponse>>> {
    let engine = state.engine.read().await;
    
    let factories: Vec<FactoryResponse> = engine.get_all_factories()
        .values()
        .map(FactoryResponse::from)
        .collect();
    
    Ok(Json(factories))
}

pub async fn get_factory(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<FactoryResponse>> {
    let engine = state.engine.read().await;
    
    let factory = engine.get_factory(id)
        .ok_or_else(|| AppError::NotFound(format!("Factory {} not found", id)))?;
    
    Ok(Json(FactoryResponse::from(factory)))
}

pub async fn create_factory(
    State(state): State<AppState>,
    Json(request): Json<CreateFactoryRequest>,
) -> Result<(StatusCode, Json<FactoryResponse>)> {
    let mut engine = state.engine.write().await;
    
    let factory_id = engine.create_factory(
        request.name,
        request.description,
    );
    
    let factory = engine.get_factory(factory_id)
        .expect("Factory should exist after creation");
    
    Ok((StatusCode::CREATED, Json(FactoryResponse::from(factory))))
}

pub async fn update_factory(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(request): Json<UpdateFactoryRequest>,
) -> Result<Json<FactoryResponse>> {
    let mut engine = state.engine.write().await;
    
    let factory = engine.get_factory_mut(id)
        .ok_or_else(|| AppError::NotFound(format!("Factory {} not found", id)))?;
    
    if let Some(name) = request.name {
        factory.name = name;
    }
    if let Some(description) = request.description {
        factory.description = Some(description);
    }
    if let Some(notes) = request.notes {
        factory.notes = Some(notes);
    }
    
    Ok(Json(FactoryResponse::from(factory)))
}

pub async fn delete_factory(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode> {
    let mut engine = state.engine.write().await;
    
    engine.delete_factory(id)
        .map_err(|_| AppError::NotFound(format!("Factory {} not found", id)))?;
    
    Ok(StatusCode::NO_CONTENT)
}

// Route configuration
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_factories).post(create_factory))
        .route("/:id", get(get_factory).put(update_factory).delete(delete_factory))
}
```

### 5. Logistics Handlers (handlers/logistics.rs)

Implement logistics-related endpoints:

```rust
// crates/satisflow-server/src/handlers/logistics.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{error::{AppError, Result}, state::AppState};
use satisflow_engine::models::{LogisticsFlux, TransportType};

#[derive(Serialize, Deserialize)]
pub struct CreateLogisticsRequest {
    pub from_factory: u64,
    pub to_factory: u64,
    pub transport_type: String, // "bus", "train", "truck", "drone"
    pub transport_details: String,
}

#[derive(Serialize)]
pub struct LogisticsResponse {
    pub id: u64,
    pub from_factory: u64,
    pub to_factory: u64,
    pub transport_type: String,
    pub transport_details: String,
    pub items: Vec<ItemFlowResponse>,
}

#[derive(Serialize)]
pub struct ItemFlowResponse {
    pub item: String,
    pub quantity_per_min: f32,
}

impl From<&LogisticsFlux> for LogisticsResponse {
    fn from(logistics: &LogisticsFlux) -> Self {
        Self {
            id: logistics.id,
            from_factory: logistics.from_factory,
            to_factory: logistics.to_factory,
            transport_type: format!("{:?}", logistics.transport_type),
            transport_details: logistics.transport_details.clone(),
            items: logistics.transport_type.get_items()
                .into_iter()
                .map(|item_flow| ItemFlowResponse {
                    item: format!("{:?}", item_flow.item),
                    quantity_per_min: item_flow.quantity_per_min,
                })
                .collect(),
        }
    }
}

pub async fn get_logistics(
    State(state): State<AppState>,
) -> Result<Json<Vec<LogisticsResponse>>> {
    let engine = state.engine.read().await;
    
    let logistics: Vec<LogisticsResponse> = engine.get_all_logistics()
        .values()
        .map(LogisticsResponse::from)
        .collect();
    
    Ok(Json(logistics))
}

pub async fn get_logistics_line(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<LogisticsResponse>> {
    let engine = state.engine.read().await;
    
    let logistics = engine.get_logistics_line(id)
        .ok_or_else(|| AppError::NotFound(format!("Logistics line {} not found", id)))?;
    
    let logistics = logistics.lock().unwrap();
    Ok(Json(LogisticsResponse::from(&*logistics)))
}

pub async fn create_logistics(
    State(state): State<AppState>,
    Json(request): Json<CreateLogisticsRequest>,
) -> Result<(StatusCode, Json<LogisticsResponse>)> {
    let mut engine = state.engine.write().await;
    
    // Validate factories exist
    if engine.get_factory(request.from_factory).is_none() {
        return Err(AppError::BadRequest(format!(
            "Source factory {} not found", request.from_factory
        )));
    }
    
    if engine.get_factory(request.to_factory).is_none() {
        return Err(AppError::BadRequest(format!(
            "Destination factory {} not found", request.to_factory
        )));
    }
    
    // Parse transport type (simplified - you'd need proper parsing logic)
    let transport_type = parse_transport_type(&request.transport_type, &request.transport_details)?;
    
    let logistics_id = engine.create_logistics_line(
        request.from_factory,
        request.to_factory,
        transport_type,
        request.transport_details,
    )?;
    
    let logistics = engine.get_logistics_line(logistics_id)
        .expect("Logistics should exist after creation");
    
    let logistics = logistics.lock().unwrap();
    Ok((StatusCode::CREATED, Json(LogisticsResponse::from(&*logistics))))
}

pub async fn delete_logistics(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode> {
    let mut engine = state.engine.write().await;
    
    engine.delete_logistics_line(id)
        .map_err(|_| AppError::NotFound(format!("Logistics line {} not found", id)))?;
    
    Ok(StatusCode::NO_CONTENT)
}

// Helper function to parse transport type (simplified)
fn parse_transport_type(transport_type: &str, details: &str) -> Result<TransportType> {
    match transport_type.to_lowercase().as_str() {
        "bus" => {
            // Parse bus details from JSON string
            // This is simplified - you'd need proper JSON parsing
            Ok(TransportType::Bus(
                // Parse bus from details
                todo!("Implement bus parsing")
            ))
        },
        "train" => {
            // Parse train details from JSON string
            Ok(TransportType::Train(
                // Parse train from details
                todo!("Implement train parsing")
            ))
        },
        "truck" => {
            // Parse truck details from JSON string
            Ok(TransportType::Truck(
                // Parse truck from details
                todo!("Implement truck parsing")
            ))
        },
        "drone" => {
            // Parse drone details from JSON string
            Ok(TransportType::Drone(
                // Parse drone from details
                todo!("Implement drone parsing")
            ))
        },
        _ => Err(AppError::BadRequest(format!(
            "Invalid transport type: {}", transport_type
        )))
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_logistics).post(create_logistics))
        .route("/:id", get(get_logistics_line).delete(delete_logistics))
}
```

### 6. Dashboard Handlers (handlers/dashboard.rs)

Implement dashboard endpoints:

```rust
// crates/satisflow-server/src/handlers/dashboard.rs
use axum::{
    extract::State,
    Json,
};
use serde::Serialize;
use std::collections::HashMap;

use crate::{error::Result, state::AppState};
use satisflow_engine::models::Item;

#[derive(Serialize)]
pub struct DashboardSummary {
    pub total_factories: usize,
    pub total_production_lines: usize,
    pub total_logistics_lines: usize,
    pub total_power_consumption: f32,
    pub total_power_generation: f32,
    pub net_power: f32,
}

#[derive(Serialize)]
pub struct ItemBalance {
    pub item: String,
    pub balance: f32,
    pub state: String, // "overflow", "underflow", "balanced"
}

pub async fn get_summary(
    State(state): State<AppState>,
) -> Result<Json<DashboardSummary>> {
    let engine = state.engine.read().await;
    
    let factories = engine.get_all_factories();
    let logistics = engine.get_all_logistics();
    
    let mut total_production_lines = 0;
    let mut total_power_consumption = 0.0;
    let mut total_power_generation = 0.0;
    
    for factory in factories.values() {
        total_production_lines += factory.production_lines.len();
        total_power_consumption += factory.total_power_consumption();
        total_power_generation += factory.total_power_generation();
    }
    
    let summary = DashboardSummary {
        total_factories: factories.len(),
        total_production_lines,
        total_logistics_lines: logistics.len(),
        total_power_consumption,
        total_power_generation,
        net_power: total_power_generation - total_power_consumption,
    };
    
    Ok(Json(summary))
}

pub async fn get_item_balances(
    State(state): State<AppState>,
) -> Result<Json<Vec<ItemBalance>>> {
    let engine = state.engine.read().await;
    
    let global_items = engine.update();
    
    let item_balances: Vec<ItemBalance> = global_items
        .into_iter()
        .map(|(item, balance)| {
            let state = if balance > 0.0 {
                "overflow".to_string()
            } else if balance < 0.0 {
                "underflow".to_string()
            } else {
                "balanced".to_string()
            };
            
            ItemBalance {
                item: format!("{:?}", item),
                balance,
                state,
            }
        })
        .collect();
    
    Ok(Json(item_balances))
}

pub async fn get_power_statistics(
    State(state): State<AppState>,
) -> Result<Json<HashMap<String, f32>>> {
    let engine = state.engine.read().await;
    
    let factories = engine.get_all_factories();
    
    let mut power_stats = HashMap::new();
    let mut total_consumption = 0.0;
    let mut total_generation = 0.0;
    
    for factory in factories.values() {
        total_consumption += factory.total_power_consumption();
        total_generation += factory.total_power_generation();
    }
    
    power_stats.insert("consumption".to_string(), total_consumption);
    power_stats.insert("generation".to_string(), total_generation);
    power_stats.insert("net".to_string(), total_generation - total_consumption);
    
    Ok(Json(power_stats))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/summary", get(get_summary))
        .route("/items", get(get_item_balances))
        .route("/power", get(get_power_statistics))
}
```

### 7. Game Data Handlers (handlers/game_data.rs)

Implement game data endpoints:

```rust
// crates/satisflow-server/src/handlers/game_data.rs
use axum::{
    extract::State,
    Json,
};
use serde::Serialize;
use std::collections::HashMap;

use crate::{error::Result, state::AppState};
use satisflow_engine::models::{Recipe, Item, MachineType};

#[derive(Serialize)]
pub struct RecipeInfo {
    pub name: String,
    pub machine: String,
    pub inputs: Vec<ItemQuantity>,
    pub outputs: Vec<ItemQuantity>,
}

#[derive(Serialize)]
pub struct ItemQuantity {
    pub item: String,
    pub quantity: f32,
}

#[derive(Serialize)]
pub struct MachineInfo {
    pub name: String,
    pub base_power: f32,
    pub max_sommersloop: u8,
}

pub async fn get_recipes(
    State(_state): State<AppState>,
) -> Result<Json<Vec<RecipeInfo>>> {
    let recipes: Vec<RecipeInfo> = Recipe::all_recipes()
        .into_iter()
        .map(|(recipe, details)| RecipeInfo {
            name: details.name.clone(),
            machine: format!("{:?}", details.machine),
            inputs: details.inputs
                .into_iter()
                .map(|(item, qty)| ItemQuantity {
                    item: format!("{:?}", item),
                    quantity: qty,
                })
                .collect(),
            outputs: details.outputs
                .into_iter()
                .map(|(item, qty)| ItemQuantity {
                    item: format!("{:?}", item),
                    quantity: qty,
                })
                .collect(),
        })
        .collect();
    
    Ok(Json(recipes))
}

pub async fn get_items(
    State(_state): State<AppState>,
) -> Result<Json<Vec<String>>> {
    let items: Vec<String> = Item::all_items()
        .keys()
        .map(|item| format!("{:?}", item))
        .collect();
    
    Ok(Json(items))
}

pub async fn get_machines(
    State(_state): State<AppState>,
) -> Result<Json<Vec<MachineInfo>>> {
    let machines: Vec<MachineInfo> = [
        MachineType::Constructor,
        MachineType::Smelter,
        MachineType::Assembler,
        MachineType::Manufacturer,
        MachineType::Refinery,
        MachineType::Blender,
        MachineType::Packager,
        MachineType::Unpackager,
    ]
    .iter()
    .map(|machine| {
        let details = machine.get_details();
        MachineInfo {
            name: format!("{:?}", machine),
            base_power: details.base_power,
            max_sommersloop: details.max_sommersloop,
        }
    })
    .collect();
    
    Ok(Json(machines))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/recipes", get(get_recipes))
        .route("/items", get(get_items))
        .route("/machines", get(get_machines))
}
```

### 8. Handler Module (handlers/mod.rs)

Create the handler module:

```rust
// crates/satisflow-server/src/handlers/mod.rs
pub mod factory;
pub mod logistics;
pub mod dashboard;
pub mod game_data;

pub use factory::*;
pub use logistics::*;
pub use dashboard::*;
pub use game_data::*;
```

### 9. Library Export (lib.rs)

Create the library file:

```rust
// crates/satisflow-server/src/lib.rs
pub mod error;
pub mod handlers;
pub mod state;

pub use error::{AppError, Result};
pub use state::AppState;
```

## Engine Modifications

The current engine needs some additions to support the web API:

### 1. Add get_all_factories method

Add to [`SatisflowEngine`](../crates/satisflow-engine/src/lib.rs):

```rust
impl SatisflowEngine {
    // ... existing methods
    
    pub fn get_all_factories(&self) -> &HashMap<u64, Factory> {
        &self.factories
    }
    
    pub fn get_all_logistics(&self) -> &HashMap<u64, Arc<Mutex<LogisticsFlux>>> {
        &self.logistics_lines
    }
    
    pub fn delete_factory(&mut self, id: u64) -> Result<(), Box<dyn std::error::Error>> {
        // Remove all logistics lines connected to this factory
        self.logistics_lines.retain(|_, logistics| {
            let logistics = logistics.lock().unwrap();
            logistics.from_factory != id && logistics.to_factory != id
        });
        
        // Remove the factory
        self.factories.remove(&id)
            .ok_or("Factory not found")?;
        
        Ok(())
    }
    
    pub fn delete_logistics_line(&mut self, id: u64) -> Result<(), Box<dyn std::error::Error>> {
        self.logistics_lines.remove(&id)
            .ok_or("Logistics line not found")?;
        
        Ok(())
    }
    
    // Add similar methods for other entities as needed
}
```

### 2. Add name field to ProductionLine

Modify the ProductionLine trait to include a name:

```rust
// In crates/satisflow-engine/src/models/production_line.rs
pub trait ProductionLine {
    fn id(&self) -> u64;
    fn name(&self) -> &str; // Add this method
    fn total_machines(&self) -> u32;
    // ... other existing methods
}
```

## Testing the Backend

### 1. Run the Server

```bash
cd crates/satisflow-server
cargo run
```

### 2. Test with curl

```bash
# Health check
curl http://localhost:3000/health

# Get all factories
curl http://localhost:3000/api/factories

# Create a factory
curl -X POST http://localhost:3000/api/factories \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Factory",
    "description": "A test factory"
  }'

# Get the created factory (replace 1 with actual ID)
curl http://localhost:3000/api/factories/1

# Get dashboard summary
curl http://localhost:3000/api/dashboard/summary

# Get game data
curl http://localhost:3000/api/game-data/recipes
```

### 3. Automated Tests

Create integration tests in `tests/` directory:

```rust
// crates/satisflow-server/tests/api_tests.rs
use reqwest;
use serde_json::json;
use tokio;

#[tokio::test]
async fn test_factory_crud() {
    let client = reqwest::Client::new();
    let base_url = "http://localhost:3000";
    
    // Create factory
    let create_response = client
        .post(&format!("{}/api/factories", base_url))
        .json(&json!({
            "name": "Test Factory",
            "description": "Test description"
        }))
        .send()
        .await
        .expect("Failed to create factory");
    
    assert_eq!(create_response.status(), 201);
    
    let factory: serde_json::Value = create_response.json().await.unwrap();
    let factory_id = factory["id"].as_u64().unwrap();
    
    // Get factory
    let get_response = client
        .get(&format!("{}/api/factories/{}", base_url, factory_id))
        .send()
        .await
        .expect("Failed to get factory");
    
    assert_eq!(get_response.status(), 200);
    
    let retrieved_factory: serde_json::Value = get_response.json().await.unwrap();
    assert_eq!(retrieved_factory["name"], "Test Factory");
    
    // Delete factory
    let delete_response = client
        .delete(&format!("{}/api/factories/{}", base_url, factory_id))
        .send()
        .await
        .expect("Failed to delete factory");
    
    assert_eq!(delete_response.status(), 204);
}
```

## Production Considerations

### 1. Environment Variables

Create a `.env` file for configuration:

```env
# Server configuration
PORT=3000
HOST=127.0.0.1

# Logging
RUST_LOG=info

# CORS (in production, specify actual origins)
CORS_ORIGINS=http://localhost:5173,https://yourdomain.com
```

### 2. CORS Configuration

Update the CORS layer in `main.rs` for production:

```rust
use std::env;

// In main function
let cors_origins = env::var("CORS_ORIGINS")
    .unwrap_or_else(|_| "http://localhost:5173".to_string())
    .split(',')
    .map(|s| s.trim().to_string())
    .collect::<Vec<_>>();

let cors = CorsLayer::new()
    .allow_origins(cors_origins)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers(Any);
```

### 3. Logging Configuration

Configure structured logging for production:

```rust
// In main.rs
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

let env_filter = EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| EnvFilter::new("info"));

tracing_subscriber::registry()
    .with(env_filter)
    .with(tracing_subscriber::fmt::layer().json()) // JSON format for production
    .init();
```

### 4. Graceful Shutdown

Implement graceful shutdown:

```rust
// In main.rs
use tokio::signal;
use std::sync::Arc;

// Replace the final serve call with:
let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Received shutdown signal, shutting down gracefully");
}
```

### 5. Docker Configuration

Create `Dockerfile`:

```dockerfile
# Dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/satisflow-server /usr/local/bin/

EXPOSE 3000

CMD ["satisflow-server"]
```

## Next Steps

1. **Complete the implementation** by filling in the TODOs in transport parsing
2. **Add comprehensive tests** for all endpoints
3. **Set up CI/CD** with automated testing
4. **Configure monitoring** and alerting
5. **Deploy to production** with proper infrastructure

This backend implementation provides a solid foundation for the Satisflow web application, with proper error handling, logging, and production-ready configuration.