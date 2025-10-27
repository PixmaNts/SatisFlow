// crates/satisflow-server/src/handlers/logistics.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
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

    let logistics = engine.get_logistics_line(logistics_id).unwrap();
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

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_logistics).post(create_logistics))
        .route(
            "/:id",
            get(get_logistics_line)
                .put(update_logistics)
                .delete(delete_logistics),
        )
}
