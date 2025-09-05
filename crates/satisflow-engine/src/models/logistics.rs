use super::types::*;
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
    pub id: LogisticsFluxId,
    pub from_factory: FactoryId,
    pub to_factory: FactoryId,
    pub item: ItemId,
    pub quantity_per_min: f32,
    pub transport_type: TransportType,
    pub transport_details: String,
}
