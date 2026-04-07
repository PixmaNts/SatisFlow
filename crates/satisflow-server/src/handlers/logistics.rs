// crates/satisflow-server/src/handlers/logistics.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use satisflow_engine::models::logistics::{
    Bus, Conveyor, ConveyorSpeed, DroneTransport, LogisticsFlux, Pipeline, PipelineCapacity, Train,
    Transport, TransportType, TruckTransport, Wagon, WagonType,
};
use satisflow_engine::models::Item;
use satisflow_engine::SatisflowEngine;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    state::AppState,
};

#[derive(Serialize, Deserialize)]
pub struct CreateLogisticsRequest {
    pub from_factory: Uuid,
    pub to_factory: Uuid,
    #[serde(flatten)]
    pub transport: CreateLogisticsTransport,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "transport_type")]
pub enum CreateLogisticsTransport {
    #[serde(rename = "Truck", alias = "truck")]
    Truck {
        item: Item,
        quantity_per_min: f32,
        #[serde(default)]
        truck_id: Option<String>,
    },
    #[serde(rename = "Drone", alias = "drone")]
    Drone {
        item: Item,
        quantity_per_min: f32,
        #[serde(default)]
        drone_id: Option<String>,
    },
    #[serde(rename = "Bus", alias = "bus")]
    Bus {
        #[serde(default)]
        bus_name: Option<String>,
        #[serde(default)]
        conveyors: Vec<BusConveyorRequest>,
        #[serde(default)]
        pipelines: Vec<BusPipelineRequest>,
    },
    #[serde(rename = "Train", alias = "train")]
    Train {
        #[serde(default)]
        train_name: Option<String>,
        #[serde(default)]
        wagons: Vec<TrainWagonRequest>,
    },
}

#[derive(Serialize, Deserialize)]
pub struct BusConveyorRequest {
    pub line_id: Option<String>,
    pub conveyor_type: String,
    pub item: Item,
    pub quantity_per_min: f32,
}

#[derive(Serialize, Deserialize)]
pub struct BusPipelineRequest {
    pub pipeline_id: Option<String>,
    pub pipeline_type: String,
    pub item: Item,
    pub quantity_per_min: f32,
}

#[derive(Serialize, Deserialize)]
pub struct TrainWagonRequest {
    pub wagon_id: Option<String>,
    pub wagon_type: String,
    pub item: Item,
    pub quantity_per_min: f32,
}

// Request DTOs for child entity mutations

#[derive(Serialize, Deserialize)]
pub struct AddConveyorRequest {
    pub line_id: Option<String>,
    pub conveyor_type: String,
    pub item: Item,
    pub quantity_per_min: f32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateConveyorRequest {
    pub conveyor_type: Option<String>,
    pub item: Option<Item>,
    pub quantity_per_min: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct AddPipelineRequest {
    pub pipeline_id: Option<String>,
    pub pipeline_type: String,
    pub item: Item,
    pub quantity_per_min: f32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePipelineRequest {
    pub pipeline_type: Option<String>,
    pub item: Option<Item>,
    pub quantity_per_min: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct AddWagonRequest {
    pub wagon_id: Option<String>,
    pub wagon_type: String,
    pub item: Item,
    pub quantity_per_min: f32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateWagonRequest {
    pub wagon_type: Option<String>,
    pub item: Option<Item>,
    pub quantity_per_min: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTruckRequest {
    pub item: Option<Item>,
    pub quantity_per_min: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateDroneRequest {
    pub item: Option<Item>,
    pub quantity_per_min: Option<f32>,
}

#[derive(Serialize)]
pub struct ItemFlowResponse {
    pub item: Item,
    pub quantity_per_min: f32,
}

#[derive(Serialize)]
pub struct LogisticsResponse {
    pub id: Uuid,
    pub from_factory: Uuid,
    pub to_factory: Uuid,
    pub transport_type: String,
    pub transport_id: String,
    pub transport_name: Option<String>,
    pub transport_details: String,
    pub items: Vec<ItemFlowResponse>,
    pub total_quantity_per_min: f32,
}

fn logistics_to_response(logistics: &LogisticsFlux) -> LogisticsResponse {
    let items = convert_item_flows(logistics.get_items());
    let total_quantity = logistics.total_quantity_per_min();

    LogisticsResponse {
        id: logistics.id,
        from_factory: logistics.from_factory,
        to_factory: logistics.to_factory,
        transport_type: logistics
            .transport_type
            .get_transport_type_name()
            .to_string(),
        transport_id: logistics.transport_type.get_transport_id(),
        transport_name: logistics.transport_type.get_transport_name(),
        transport_details: logistics.transport_details.clone(),
        items,
        total_quantity_per_min: total_quantity,
    }
}

// Helper function to parse conveyor speed
fn parse_conveyor_speed(speed_str: &str) -> Result<ConveyorSpeed> {
    match speed_str.to_uppercase().as_str() {
        "MK1" => Ok(ConveyorSpeed::Mk1),
        "MK2" => Ok(ConveyorSpeed::Mk2),
        "MK3" => Ok(ConveyorSpeed::Mk3),
        "MK4" => Ok(ConveyorSpeed::Mk4),
        "MK5" => Ok(ConveyorSpeed::Mk5),
        "MK6" => Ok(ConveyorSpeed::Mk6),
        _ => Err(AppError::BadRequest(format!(
            "Unknown conveyor speed: {}",
            speed_str
        ))),
    }
}

// Helper function to parse pipeline capacity
fn parse_pipeline_capacity(capacity_str: &str) -> Result<PipelineCapacity> {
    match capacity_str.to_uppercase().as_str() {
        "MK1" => Ok(PipelineCapacity::Mk1),
        "MK2" => Ok(PipelineCapacity::Mk2),
        _ => Err(AppError::BadRequest(format!(
            "Unknown pipeline capacity: {}",
            capacity_str
        ))),
    }
}

// Helper function to parse wagon type
fn parse_wagon_type(wagon_type_str: &str) -> Result<WagonType> {
    match wagon_type_str.to_uppercase().as_str() {
        "CARGO" => Ok(WagonType::Cargo),
        "FLUID" => Ok(WagonType::Fluid),
        _ => Err(AppError::BadRequest(format!(
            "Unknown wagon type: {}",
            wagon_type_str
        ))),
    }
}

// Helper function to convert ItemFlow to response
fn convert_item_flows(
    item_flows: Vec<satisflow_engine::models::logistics::ItemFlow>,
) -> Vec<ItemFlowResponse> {
    item_flows
        .into_iter()
        .map(|flow| ItemFlowResponse {
            item: flow.item,
            quantity_per_min: flow.quantity_per_min,
        })
        .collect()
}

pub async fn get_logistics(State(state): State<AppState>) -> Result<Json<Vec<LogisticsResponse>>> {
    let engine = state.engine.read().await;
    let logistics_lines = engine.get_all_logistics();

    let responses = logistics_lines
        .values()
        .map(logistics_to_response)
        .collect();

    Ok(Json(responses))
}

pub async fn get_logistics_line(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<LogisticsResponse>> {
    let engine = state.engine.read().await;

    let logistics = engine
        .get_logistics_line(id)
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    Ok(Json(logistics_to_response(logistics)))
}

pub async fn create_logistics(
    State(state): State<AppState>,
    Json(request): Json<CreateLogisticsRequest>,
) -> Result<(StatusCode, Json<LogisticsResponse>)> {
    let mut engine = state.engine.write().await;

    let from_factory = request.from_factory;
    let to_factory = request.to_factory;

    // Validate that factories exist
    if engine.get_factory(from_factory).is_none() {
        return Err(AppError::BadRequest(format!(
            "Source factory with id {} does not exist",
            from_factory
        )));
    }

    if engine.get_factory(to_factory).is_none() {
        return Err(AppError::BadRequest(format!(
            "Destination factory with id {} does not exist",
            to_factory
        )));
    }

    let (transport_type, transport_details) = build_transport(&engine, request.transport, None)
        .map_err(|err| match err {
            AppError::SerializationError(_) => {
                AppError::BadRequest("Failed to serialize transport details".to_string())
            }
            other => other,
        })?;

    let logistics_id = engine
        .create_logistics_line(from_factory, to_factory, transport_type, transport_details)
        .map_err(|e| AppError::BadRequest(format!("Failed to create logistics line: {}", e)))?;

    let logistics = engine
        .get_logistics_line(logistics_id)
        .ok_or_else(|| AppError::InternalError(anyhow::anyhow!("Failed to retrieve created logistics line")))?;
    let response = logistics_to_response(logistics);

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn update_logistics(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<CreateLogisticsRequest>,
) -> Result<Json<LogisticsResponse>> {
    let mut engine = state.engine.write().await;

    let existing = engine
        .get_logistics_line(id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    let (transport_type, transport_details) =
        build_transport(&engine, request.transport, Some(&existing)).map_err(|err| match err {
            AppError::SerializationError(_) => {
                AppError::BadRequest("Failed to serialize transport details".to_string())
            }
            other => other,
        })?;

    engine
        .update_logistics_line(
            id,
            request.from_factory,
            request.to_factory,
            transport_type,
            transport_details,
        )
        .map_err(|e| AppError::BadRequest(format!("Failed to update logistics line: {}", e)))?;

    let updated = engine
        .get_logistics_line(id)
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    Ok(Json(logistics_to_response(updated)))
}

fn build_transport(
    engine: &SatisflowEngine,
    transport: CreateLogisticsTransport,
    existing: Option<&LogisticsFlux>,
) -> std::result::Result<(TransportType, String), AppError> {
    match transport {
        CreateLogisticsTransport::Truck {
            item,
            quantity_per_min,
            truck_id,
        } => {
            let quantity = ensure_positive(quantity_per_min, "Truck quantity_per_min")?;
            let item_enum = item;
            let fallback_id = existing
                .and_then(|flux| match &flux.transport_type {
                    TransportType::Truck(truck) => Some(truck.truck_id),
                    _ => None,
                })
                .unwrap_or_else(|| next_transport_identifier(engine));
            let numeric_id = parse_numeric_identifier(truck_id.as_deref(), fallback_id);
            let display_id = truck_id
                .as_ref()
                .map(|val| val.trim())
                .filter(|val| !val.is_empty())
                .map(|val| val.to_string())
                .unwrap_or_else(|| format!("TRK-{numeric_id:03}"));

            let transport =
                TransportType::Truck(TruckTransport::new(numeric_id, item_enum, quantity));

            let details = serde_json::to_string(&json!({
                "truck_id": display_id,
                "item": item_enum,
                "quantity_per_min": quantity,
            }))?;

            Ok((transport, details))
        }
        CreateLogisticsTransport::Drone {
            item,
            quantity_per_min,
            drone_id,
        } => {
            let quantity = ensure_positive(quantity_per_min, "Drone quantity_per_min")?;
            let item_enum = item;
            let fallback_id = existing
                .and_then(|flux| match &flux.transport_type {
                    TransportType::Drone(drone) => Some(drone.drone_id),
                    _ => None,
                })
                .unwrap_or_else(|| next_transport_identifier(engine));
            let numeric_id = parse_numeric_identifier(drone_id.as_deref(), fallback_id);
            let display_id = drone_id
                .as_ref()
                .map(|val| val.trim())
                .filter(|val| !val.is_empty())
                .map(|val| val.to_string())
                .unwrap_or_else(|| format!("DRN-{numeric_id:03}"));

            let transport =
                TransportType::Drone(DroneTransport::new(numeric_id, item_enum, quantity));

            let details = serde_json::to_string(&json!({
                "drone_id": display_id,
                "item": item_enum,
                "quantity_per_min": quantity,
            }))?;

            Ok((transport, details))
        }
        CreateLogisticsTransport::Bus {
            bus_name,
            conveyors,
            pipelines,
        } => {
            let existing_bus = existing.and_then(|flux| match &flux.transport_type {
                TransportType::Bus(bus) => Some(bus),
                _ => None,
            });
            let bus_id = existing_bus
                .map(|bus| bus.bus_id)
                .unwrap_or_else(|| next_transport_identifier(engine));
            let provided_name = bus_name
                .as_ref()
                .map(|val| val.trim())
                .filter(|val| !val.is_empty())
                .map(|val| val.to_string());
            let name = provided_name
                .or_else(|| existing_bus.map(|bus| bus.bus_name.clone()))
                .unwrap_or_else(|| sanitize_name(None, "Bus", bus_id));
            let mut bus = Bus::new(bus_id, name.clone());

            let mut conveyor_details = Vec::new();
            for (index, conveyor) in conveyors.into_iter().enumerate() {
                let BusConveyorRequest {
                    line_id,
                    conveyor_type,
                    item,
                    quantity_per_min,
                } = conveyor;

                let quantity = ensure_positive(quantity_per_min, "Bus conveyor quantity_per_min")?;
                let item_enum = item;
                let speed = parse_conveyor_speed(&conveyor_type)?;
                let numeric_line_id =
                    parse_numeric_identifier(line_id.as_deref(), (index + 1) as u64);
                let line_label = line_id
                    .as_ref()
                    .map(|val| val.trim())
                    .filter(|val| !val.is_empty())
                    .map(|val| val.to_string())
                    .unwrap_or_else(|| format!("CV-{numeric_line_id:03}"));

                bus.add_conveyor(Conveyor::new(
                    numeric_line_id,
                    speed.clone(),
                    item_enum,
                    quantity,
                ));

                conveyor_details.push(json!({
                    "line_id": line_label,
                    "conveyor_type": speed,
                    "item": item_enum,
                    "quantity_per_min": quantity,
                }));
            }

            let mut pipeline_details = Vec::new();
            for (index, pipeline) in pipelines.into_iter().enumerate() {
                let BusPipelineRequest {
                    pipeline_id,
                    pipeline_type,
                    item,
                    quantity_per_min,
                } = pipeline;

                let quantity = ensure_positive(quantity_per_min, "Bus pipeline quantity_per_min")?;
                let item_enum = item;
                let capacity = parse_pipeline_capacity(&pipeline_type)?;
                let numeric_pipeline_id =
                    parse_numeric_identifier(pipeline_id.as_deref(), (index + 1) as u64);
                let pipeline_label = pipeline_id
                    .as_ref()
                    .map(|val| val.trim())
                    .filter(|val| !val.is_empty())
                    .map(|val| val.to_string())
                    .unwrap_or_else(|| format!("PL-{numeric_pipeline_id:03}"));

                bus.add_pipeline(Pipeline::new(
                    numeric_pipeline_id,
                    capacity.clone(),
                    item_enum,
                    quantity,
                ));

                pipeline_details.push(json!({
                    "pipeline_id": pipeline_label,
                    "pipeline_type": capacity,
                    "item": item_enum,
                    "quantity_per_min": quantity,
                }));
            }

            if bus.lines.is_empty() && bus.pipelines.is_empty() {
                return Err(AppError::BadRequest(
                    "Bus transport requires at least one conveyor or pipeline".to_string(),
                ));
            }

            let details = serde_json::to_string(&json!({
                "bus_id": bus_id,
                "bus_name": name,
                "conveyors": conveyor_details,
                "pipelines": pipeline_details,
            }))?;

            Ok((TransportType::Bus(bus), details))
        }
        CreateLogisticsTransport::Train { train_name, wagons } => {
            let existing_train = existing.and_then(|flux| match &flux.transport_type {
                TransportType::Train(train) => Some(train),
                _ => None,
            });
            let train_id = existing_train
                .map(|train| train.train_id)
                .unwrap_or_else(|| next_transport_identifier(engine));
            let provided_name = train_name
                .as_ref()
                .map(|val| val.trim())
                .filter(|val| !val.is_empty())
                .map(|val| val.to_string());
            let name = provided_name
                .or_else(|| existing_train.map(|train| train.train_name.clone()))
                .unwrap_or_else(|| sanitize_name(None, "Train", train_id));
            let mut train = Train::new(train_id, name.clone());

            let mut wagon_details = Vec::new();
            for (index, wagon) in wagons.into_iter().enumerate() {
                let TrainWagonRequest {
                    wagon_id,
                    wagon_type,
                    item,
                    quantity_per_min,
                } = wagon;

                let quantity = ensure_positive(quantity_per_min, "Train wagon quantity_per_min")?;
                let item_enum = item;
                let wagon_type_enum = parse_wagon_type(&wagon_type)?;
                let numeric_wagon_id =
                    parse_numeric_identifier(wagon_id.as_deref(), (index + 1) as u64);
                let wagon_label = wagon_id
                    .as_ref()
                    .map(|val| val.trim())
                    .filter(|val| !val.is_empty())
                    .map(|val| val.to_string())
                    .unwrap_or_else(|| format!("WG-{numeric_wagon_id:03}"));

                train.add_wagon(Wagon::new(
                    numeric_wagon_id,
                    wagon_type_enum.clone(),
                    item_enum,
                    quantity,
                ));

                wagon_details.push(json!({
                    "wagon_id": wagon_label,
                    "wagon_type": wagon_type_enum,
                    "item": item_enum,
                    "quantity_per_min": quantity,
                }));
            }

            if wagon_details.is_empty() {
                return Err(AppError::BadRequest(
                    "Train transport requires at least one wagon".to_string(),
                ));
            }

            let details = serde_json::to_string(&json!({
                "train_id": train_id,
                "train_name": name,
                "wagons": wagon_details,
            }))?;

            Ok((TransportType::Train(train), details))
        }
    }
}

fn next_transport_identifier(engine: &SatisflowEngine) -> u64 {
    engine.get_all_logistics().len() as u64 + 1
}

fn parse_numeric_identifier(value: Option<&str>, fallback: u64) -> u64 {
    value
        .and_then(|raw| {
            let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if digits.is_empty() {
                None
            } else {
                digits.parse::<u64>().ok()
            }
        })
        .unwrap_or(fallback)
}

fn sanitize_name(value: Option<&str>, label: &str, id: u64) -> String {
    value
        .map(|val| val.trim())
        .filter(|val| !val.is_empty())
        .map(|val| val.to_string())
        .unwrap_or_else(|| format!("{} {}", label, id))
}

fn ensure_positive(value: f32, context: &str) -> std::result::Result<f32, AppError> {
    if value > 0.0 {
        Ok(value)
    } else {
        Err(AppError::BadRequest(format!(
            "{context} must be greater than zero"
        )))
    }
}

pub async fn delete_logistics(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    let mut engine = state.engine.write().await;

    engine
        .delete_logistics_line(id)
        .map_err(|_| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    Ok(StatusCode::NO_CONTENT)
}

// Child entity mutation handlers for Bus conveyors

pub async fn add_bus_conveyor(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<AddConveyorRequest>,
) -> Result<(StatusCode, Json<LogisticsResponse>)> {
    let mut engine = state.engine.write().await;

    let logistics = engine
        .get_logistics_line(id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    let TransportType::Bus(mut bus) = logistics.transport_type else {
        return Err(AppError::BadRequest(
            "Logistics line is not a Bus transport".to_string(),
        ));
    };

    let quantity = ensure_positive(request.quantity_per_min, "Conveyor quantity_per_min")?;
    let speed = parse_conveyor_speed(&request.conveyor_type)?;
    let line_index = bus.lines.len() + bus.pipelines.len() + 1;
    let numeric_line_id = parse_numeric_identifier(request.line_id.as_deref(), line_index as u64);

    bus.add_conveyor(Conveyor::new(
        numeric_line_id,
        speed,
        request.item,
        quantity,
    ));

    // Update the logistics line with modified bus
    let transport_type = TransportType::Bus(bus);
    let transport_details = build_transport_details(&transport_type)?;

    engine
        .update_logistics_line(
            id,
            logistics.from_factory,
            logistics.to_factory,
            transport_type,
            transport_details,
        )
        .map_err(|e| AppError::BadRequest(format!("Failed to update logistics line: {}", e)))?;

    let updated = engine
        .get_logistics_line(id)
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    Ok((StatusCode::CREATED, Json(logistics_to_response(updated))))
}

pub async fn update_bus_conveyor(
    State(state): State<AppState>,
    Path((id, conveyor_id)): Path<(Uuid, String)>,
    Json(request): Json<UpdateConveyorRequest>,
) -> Result<Json<LogisticsResponse>> {
    let mut engine = state.engine.write().await;

    let logistics = engine
        .get_logistics_line(id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    let TransportType::Bus(mut bus) = logistics.transport_type else {
        return Err(AppError::BadRequest(
            "Logistics line is not a Bus transport".to_string(),
        ));
    };

    let conveyor = bus.get_conveyor_mut(&conveyor_id).ok_or_else(|| {
        AppError::NotFound(format!(
            "Conveyor with id {} not found in bus",
            conveyor_id
        ))
    })?;

    // Apply updates using field setters (via struct field mutation)
    if let Some(speed_str) = request.conveyor_type {
        conveyor.speed = parse_conveyor_speed(&speed_str)?;
    }
    if let Some(item) = request.item {
        conveyor.item = item;
    }
    if let Some(quantity) = request.quantity_per_min {
        conveyor.quantity_per_min = ensure_positive(quantity, "Conveyor quantity_per_min")?;
    }

    let transport_type = TransportType::Bus(bus);
    let transport_details = build_transport_details(&transport_type)?;

    engine
        .update_logistics_line(
            id,
            logistics.from_factory,
            logistics.to_factory,
            transport_type,
            transport_details,
        )
        .map_err(|e| AppError::BadRequest(format!("Failed to update logistics line: {}", e)))?;

    let updated = engine
        .get_logistics_line(id)
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    Ok(Json(logistics_to_response(updated)))
}

pub async fn remove_bus_conveyor(
    State(state): State<AppState>,
    Path((id, conveyor_id)): Path<(Uuid, String)>,
) -> Result<StatusCode> {
    let mut engine = state.engine.write().await;

    let logistics = engine
        .get_logistics_line(id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    let TransportType::Bus(mut bus) = logistics.transport_type else {
        return Err(AppError::BadRequest(
            "Logistics line is not a Bus transport".to_string(),
        ));
    };

    bus.remove_conveyor(&conveyor_id).ok_or_else(|| {
        AppError::NotFound(format!(
            "Conveyor with id {} not found in bus",
            conveyor_id
        ))
    })?;

    // Ensure bus still has at least one conveyor or pipeline
    if bus.lines.is_empty() && bus.pipelines.is_empty() {
        return Err(AppError::BadRequest(
            "Bus transport requires at least one conveyor or pipeline".to_string(),
        ));
    }

    let transport_type = TransportType::Bus(bus);
    let transport_details = build_transport_details(&transport_type)?;

    engine
        .update_logistics_line(
            id,
            logistics.from_factory,
            logistics.to_factory,
            transport_type,
            transport_details,
        )
        .map_err(|e| AppError::BadRequest(format!("Failed to update logistics line: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}

// Child entity mutation handlers for Bus pipelines

pub async fn add_bus_pipeline(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<AddPipelineRequest>,
) -> Result<(StatusCode, Json<LogisticsResponse>)> {
    let mut engine = state.engine.write().await;

    let logistics = engine
        .get_logistics_line(id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    let TransportType::Bus(mut bus) = logistics.transport_type else {
        return Err(AppError::BadRequest(
            "Logistics line is not a Bus transport".to_string(),
        ));
    };

    let quantity = ensure_positive(request.quantity_per_min, "Pipeline quantity_per_min")?;
    let capacity = parse_pipeline_capacity(&request.pipeline_type)?;
    let pipeline_index = bus.lines.len() + bus.pipelines.len() + 1;
    let numeric_pipeline_id = parse_numeric_identifier(request.pipeline_id.as_deref(), pipeline_index as u64);

    bus.add_pipeline(Pipeline::new(
        numeric_pipeline_id,
        capacity,
        request.item,
        quantity,
    ));

    let transport_type = TransportType::Bus(bus);
    let transport_details = build_transport_details(&transport_type)?;

    engine
        .update_logistics_line(
            id,
            logistics.from_factory,
            logistics.to_factory,
            transport_type,
            transport_details,
        )
        .map_err(|e| AppError::BadRequest(format!("Failed to update logistics line: {}", e)))?;

    let updated = engine
        .get_logistics_line(id)
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    Ok((StatusCode::CREATED, Json(logistics_to_response(updated))))
}

pub async fn update_bus_pipeline(
    State(state): State<AppState>,
    Path((id, pipeline_id)): Path<(Uuid, String)>,
    Json(request): Json<UpdatePipelineRequest>,
) -> Result<Json<LogisticsResponse>> {
    let mut engine = state.engine.write().await;

    let logistics = engine
        .get_logistics_line(id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    let TransportType::Bus(mut bus) = logistics.transport_type else {
        return Err(AppError::BadRequest(
            "Logistics line is not a Bus transport".to_string(),
        ));
    };

    let pipeline = bus.get_pipeline_mut(&pipeline_id).ok_or_else(|| {
        AppError::NotFound(format!(
            "Pipeline with id {} not found in bus",
            pipeline_id
        ))
    })?;

    if let Some(capacity_str) = request.pipeline_type {
        pipeline.capacity = parse_pipeline_capacity(&capacity_str)?;
    }
    if let Some(item) = request.item {
        pipeline.item = item;
    }
    if let Some(quantity) = request.quantity_per_min {
        pipeline.quantity_per_min = ensure_positive(quantity, "Pipeline quantity_per_min")?;
    }

    let transport_type = TransportType::Bus(bus);
    let transport_details = build_transport_details(&transport_type)?;

    engine
        .update_logistics_line(
            id,
            logistics.from_factory,
            logistics.to_factory,
            transport_type,
            transport_details,
        )
        .map_err(|e| AppError::BadRequest(format!("Failed to update logistics line: {}", e)))?;

    let updated = engine
        .get_logistics_line(id)
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    Ok(Json(logistics_to_response(updated)))
}

pub async fn remove_bus_pipeline(
    State(state): State<AppState>,
    Path((id, pipeline_id)): Path<(Uuid, String)>,
) -> Result<StatusCode> {
    let mut engine = state.engine.write().await;

    let logistics = engine
        .get_logistics_line(id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    let TransportType::Bus(mut bus) = logistics.transport_type else {
        return Err(AppError::BadRequest(
            "Logistics line is not a Bus transport".to_string(),
        ));
    };

    bus.remove_pipeline(&pipeline_id).ok_or_else(|| {
        AppError::NotFound(format!(
            "Pipeline with id {} not found in bus",
            pipeline_id
        ))
    })?;

    if bus.lines.is_empty() && bus.pipelines.is_empty() {
        return Err(AppError::BadRequest(
            "Bus transport requires at least one conveyor or pipeline".to_string(),
        ));
    }

    let transport_type = TransportType::Bus(bus);
    let transport_details = build_transport_details(&transport_type)?;

    engine
        .update_logistics_line(
            id,
            logistics.from_factory,
            logistics.to_factory,
            transport_type,
            transport_details,
        )
        .map_err(|e| AppError::BadRequest(format!("Failed to update logistics line: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}

// Child entity mutation handlers for Train wagons

pub async fn add_train_wagon(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<AddWagonRequest>,
) -> Result<(StatusCode, Json<LogisticsResponse>)> {
    let mut engine = state.engine.write().await;

    let logistics = engine
        .get_logistics_line(id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    let TransportType::Train(mut train) = logistics.transport_type else {
        return Err(AppError::BadRequest(
            "Logistics line is not a Train transport".to_string(),
        ));
    };

    let quantity = ensure_positive(request.quantity_per_min, "Wagon quantity_per_min")?;
    let wagon_type = parse_wagon_type(&request.wagon_type)?;
    let wagon_index = train.wagons.len() + 1;
    let numeric_wagon_id = parse_numeric_identifier(request.wagon_id.as_deref(), wagon_index as u64);

    train.add_wagon(Wagon::new(
        numeric_wagon_id,
        wagon_type,
        request.item,
        quantity,
    ));

    let transport_type = TransportType::Train(train);
    let transport_details = build_transport_details(&transport_type)?;

    engine
        .update_logistics_line(
            id,
            logistics.from_factory,
            logistics.to_factory,
            transport_type,
            transport_details,
        )
        .map_err(|e| AppError::BadRequest(format!("Failed to update logistics line: {}", e)))?;

    let updated = engine
        .get_logistics_line(id)
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    Ok((StatusCode::CREATED, Json(logistics_to_response(updated))))
}

pub async fn update_train_wagon(
    State(state): State<AppState>,
    Path((id, wagon_id)): Path<(Uuid, String)>,
    Json(request): Json<UpdateWagonRequest>,
) -> Result<Json<LogisticsResponse>> {
    let mut engine = state.engine.write().await;

    let logistics = engine
        .get_logistics_line(id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    let TransportType::Train(mut train) = logistics.transport_type else {
        return Err(AppError::BadRequest(
            "Logistics line is not a Train transport".to_string(),
        ));
    };

    let wagon = train.get_wagon_mut(&wagon_id).ok_or_else(|| {
        AppError::NotFound(format!("Wagon with id {} not found in train", wagon_id))
    })?;

    if let Some(wagon_type_str) = request.wagon_type {
        wagon.wagon_type = parse_wagon_type(&wagon_type_str)?;
    }
    if let Some(item) = request.item {
        wagon.item = item;
    }
    if let Some(quantity) = request.quantity_per_min {
        wagon.quantity_per_min = ensure_positive(quantity, "Wagon quantity_per_min")?;
    }

    let transport_type = TransportType::Train(train);
    let transport_details = build_transport_details(&transport_type)?;

    engine
        .update_logistics_line(
            id,
            logistics.from_factory,
            logistics.to_factory,
            transport_type,
            transport_details,
        )
        .map_err(|e| AppError::BadRequest(format!("Failed to update logistics line: {}", e)))?;

    let updated = engine
        .get_logistics_line(id)
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    Ok(Json(logistics_to_response(updated)))
}

pub async fn remove_train_wagon(
    State(state): State<AppState>,
    Path((id, wagon_id)): Path<(Uuid, String)>,
) -> Result<StatusCode> {
    let mut engine = state.engine.write().await;

    let logistics = engine
        .get_logistics_line(id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    let TransportType::Train(mut train) = logistics.transport_type else {
        return Err(AppError::BadRequest(
            "Logistics line is not a Train transport".to_string(),
        ));
    };

    train.remove_wagon(&wagon_id).ok_or_else(|| {
        AppError::NotFound(format!("Wagon with id {} not found in train", wagon_id))
    })?;

    if train.wagons.is_empty() {
        return Err(AppError::BadRequest(
            "Train transport requires at least one wagon".to_string(),
        ));
    }

    let transport_type = TransportType::Train(train);
    let transport_details = build_transport_details(&transport_type)?;

    engine
        .update_logistics_line(
            id,
            logistics.from_factory,
            logistics.to_factory,
            transport_type,
            transport_details,
        )
        .map_err(|e| AppError::BadRequest(format!("Failed to update logistics line: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}

// Field setter handlers for Truck

pub async fn update_truck_fields(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateTruckRequest>,
) -> Result<Json<LogisticsResponse>> {
    let mut engine = state.engine.write().await;

    let logistics = engine
        .get_logistics_line(id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    let TransportType::Truck(mut truck) = logistics.transport_type else {
        return Err(AppError::BadRequest(
            "Logistics line is not a Truck transport".to_string(),
        ));
    };

    // Apply field updates
    if let Some(item) = request.item {
        truck.item = item;
    }
    if let Some(quantity) = request.quantity_per_min {
        truck.quantity_per_min = ensure_positive(quantity, "Truck quantity_per_min")?;
    }

    let transport_type = TransportType::Truck(truck);
    let transport_details = build_transport_details(&transport_type)?;

    engine
        .update_logistics_line(
            id,
            logistics.from_factory,
            logistics.to_factory,
            transport_type,
            transport_details,
        )
        .map_err(|e| AppError::BadRequest(format!("Failed to update logistics line: {}", e)))?;

    let updated = engine
        .get_logistics_line(id)
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    Ok(Json(logistics_to_response(updated)))
}

// Field setter handlers for Drone

pub async fn update_drone_fields(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateDroneRequest>,
) -> Result<Json<LogisticsResponse>> {
    let mut engine = state.engine.write().await;

    let logistics = engine
        .get_logistics_line(id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    let TransportType::Drone(mut drone) = logistics.transport_type else {
        return Err(AppError::BadRequest(
            "Logistics line is not a Drone transport".to_string(),
        ));
    };

    // Apply field updates
    if let Some(item) = request.item {
        drone.item = item;
    }
    if let Some(quantity) = request.quantity_per_min {
        drone.quantity_per_min = ensure_positive(quantity, "Drone quantity_per_min")?;
    }

    let transport_type = TransportType::Drone(drone);
    let transport_details = build_transport_details(&transport_type)?;

    engine
        .update_logistics_line(
            id,
            logistics.from_factory,
            logistics.to_factory,
            transport_type,
            transport_details,
        )
        .map_err(|e| AppError::BadRequest(format!("Failed to update logistics line: {}", e)))?;

    let updated = engine
        .get_logistics_line(id)
        .ok_or_else(|| AppError::NotFound(format!("Logistics line with id {} not found", id)))?;

    Ok(Json(logistics_to_response(updated)))
}

// Helper function to build transport details JSON for child mutations
fn build_transport_details(transport_type: &TransportType) -> Result<String> {
    match transport_type {
        TransportType::Bus(bus) => {
            let conveyor_details: Vec<_> = bus
                .lines
                .iter()
                .map(|c| {
                    json!({
                        "line_id": format!("CV-{}", c.line_id),
                        "conveyor_type": c.speed,
                        "item": c.item,
                        "quantity_per_min": c.quantity_per_min,
                    })
                })
                .collect();
            let pipeline_details: Vec<_> = bus
                .pipelines
                .iter()
                .map(|p| {
                    json!({
                        "pipeline_id": format!("PL-{}", p.pipeline_id),
                        "pipeline_type": p.capacity,
                        "item": p.item,
                        "quantity_per_min": p.quantity_per_min,
                    })
                })
                .collect();
            serde_json::to_string(&json!({
                "bus_id": bus.bus_id,
                "bus_name": bus.bus_name,
                "conveyors": conveyor_details,
                "pipelines": pipeline_details,
            }))
            .map_err(AppError::from)
        }
        TransportType::Train(train) => {
            let wagon_details: Vec<_> = train
                .wagons
                .iter()
                .map(|w| {
                    json!({
                        "wagon_id": format!("WG-{}", w.wagon_id),
                        "wagon_type": w.wagon_type,
                        "item": w.item,
                        "quantity_per_min": w.quantity_per_min,
                    })
                })
                .collect();
            serde_json::to_string(&json!({
                "train_id": train.train_id,
                "train_name": train.train_name,
                "wagons": wagon_details,
            }))
            .map_err(AppError::from)
        }
        TransportType::Truck(truck) => {
            serde_json::to_string(&json!({
                "truck_id": format!("TRK-{}", truck.truck_id),
                "item": truck.item,
                "quantity_per_min": truck.quantity_per_min,
            }))
            .map_err(AppError::from)
        }
        TransportType::Drone(drone) => {
            serde_json::to_string(&json!({
                "drone_id": format!("DRN-{}", drone.drone_id),
                "item": drone.item,
                "quantity_per_min": drone.quantity_per_min,
            }))
            .map_err(AppError::from)
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_logistics).post(create_logistics))
        .route(
            "/:id",
            get(get_logistics_line)
                .put(update_logistics)
                .delete(delete_logistics),
        )
        // Bus child entity routes
        .route("/:id/conveyors", post(add_bus_conveyor))
        .route("/:id/conveyors/:conveyor_id", put(update_bus_conveyor).delete(remove_bus_conveyor))
        .route("/:id/pipelines", post(add_bus_pipeline))
        .route("/:id/pipelines/:pipeline_id", put(update_bus_pipeline).delete(remove_bus_pipeline))
        // Train child entity routes
        .route("/:id/wagons", post(add_train_wagon))
        .route("/:id/wagons/:wagon_id", put(update_train_wagon).delete(remove_train_wagon))
        // Truck/Drone field setter routes
        .route("/:id/truck", put(update_truck_fields))
        .route("/:id/drone", put(update_drone_fields))
}
