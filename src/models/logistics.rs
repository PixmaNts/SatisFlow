use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransportType {
    Conveyor,
    Train,
    Truck,
    Drone,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogisticsFlux {
    pub id: String,
    pub from_factory: String,
    pub to_factory: String,
    pub item: String,
    pub quantity_per_min: f32,
    pub transport_type: TransportType,
    pub transport_details: String,
}