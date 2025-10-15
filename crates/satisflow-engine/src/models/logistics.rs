use serde::{Deserialize, Serialize};

use crate::models::Item;

pub trait ItemPerPin {
    /// Returns the number of items that can be transported per minute (Max throughput).
    fn item_per_min(&self) -> f32;
}

pub trait FluidPerMin {
    /// Returns the volume of fluid that can be transported per minute (Max throughput in m³/min).
    fn m3_per_min(&self) -> f32;
}

/// Common item flow information returned by all transport types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemFlow {
    pub item: Item,
    pub quantity_per_min: f32,
}

/// Trait for all transport types that can carry items
pub trait Transport {
    /// Returns all item flows for this transport
    fn get_items(&self) -> Vec<ItemFlow>;

    /// Returns the transport ID (e.g., "TRN-1", "BUS-5")
    fn get_transport_id(&self) -> String;

    /// Returns the transport name if available
    fn get_transport_name(&self) -> Option<String>;

    /// Returns the transport type as a string
    fn get_transport_type_name(&self) -> &'static str;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TruckTransport {
    pub truck_id: u64,
    pub item: Item,
    pub quantity_per_min: f32,
}

impl TruckTransport {
    pub fn new(truck_id: u64, item: Item, quantity_per_min: f32) -> Self {
        Self {
            truck_id,
            item,
            quantity_per_min,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DroneTransport {
    pub drone_id: u64,
    pub item: Item,
    pub quantity_per_min: f32,
}

impl DroneTransport {
    pub fn new(drone_id: u64, item: Item, quantity_per_min: f32) -> Self {
        Self {
            drone_id,
            item,
            quantity_per_min,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransportType {
    Bus(Bus),
    Train(Train),
    Truck(TruckTransport),
    Drone(DroneTransport),
}

impl TransportType {
    pub fn id_prefix(&self) -> &'static str {
        match self {
            TransportType::Bus(_) => "BUS",
            TransportType::Train(_) => "TRN",
            TransportType::Truck(_) => "TRK",
            TransportType::Drone(_) => "DRN",
        }
    }
}

impl std::fmt::Display for TransportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            TransportType::Bus(_) => "Bus",
            TransportType::Train(_) => "Train",
            TransportType::Truck(_) => "Truck",
            TransportType::Drone(_) => "Drone",
        };
        write!(f, "{}", type_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogisticsFlux {
    pub id: u64,
    pub from_factory: u64,
    pub to_factory: u64,
    pub transport_type: TransportType,
    pub transport_details: String,
}

impl LogisticsFlux {
    /// Get all items transported by this logistics flux
    pub fn get_items(&self) -> Vec<ItemFlow> {
        self.transport_type.get_items()
    }

    /// Get total quantity across all items
    pub fn total_quantity_per_min(&self) -> f32 {
        self.get_items().iter().map(|i| i.quantity_per_min).sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bus {
    pub bus_id: u64,
    pub bus_name: String,
    pub lines: Vec<Conveyor>,
    pub pipelines: Vec<Pipeline>,
}

impl Bus {
    pub fn new(bus_id: u64, bus_name: impl Into<String>) -> Self {
        Self {
            bus_id,
            bus_name: bus_name.into(),
            lines: Vec::new(),
            pipelines: Vec::new(),
        }
    }

    pub fn with_conveyor(mut self, conveyor: Conveyor) -> Self {
        self.lines.push(conveyor);
        self
    }

    pub fn with_pipeline(mut self, pipeline: Pipeline) -> Self {
        self.pipelines.push(pipeline);
        self
    }

    pub fn add_conveyor(&mut self, conveyor: Conveyor) {
        self.lines.push(conveyor);
    }

    pub fn add_pipeline(&mut self, pipeline: Pipeline) {
        self.pipelines.push(pipeline);
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conveyor {
    pub line_id: u64,
    pub speed: ConveyorSpeed,
    pub item: Item,
    pub quantity_per_min: f32,
}

impl Conveyor {
    pub fn new(line_id: u64, speed: ConveyorSpeed, item: Item, quantity_per_min: f32) -> Self {
        Self {
            line_id,
            speed,
            item,
            quantity_per_min,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pipeline {
    pub pipeline_id: u64,
    pub capacity: PipelineCapacity,
    pub item: Item,
    pub quantity_per_min: f32,
}

impl Pipeline {
    pub fn new(
        pipeline_id: u64,
        capacity: PipelineCapacity,
        item: Item,
        quantity_per_min: f32,
    ) -> Self {
        Self {
            pipeline_id,
            capacity,
            item,
            quantity_per_min,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PipelineCapacity {
    Mk1,
    Mk2,
}

impl PipelineCapacity {
    pub const MK1_CAPACITY: f32 = 300.0;
    pub const MK2_CAPACITY: f32 = 600.0;

    pub fn m3_per_min(&self) -> f32 {
        match self {
            PipelineCapacity::Mk1 => Self::MK1_CAPACITY,
            PipelineCapacity::Mk2 => Self::MK2_CAPACITY,
        }
    }
}
impl FluidPerMin for PipelineCapacity {
    fn m3_per_min(&self) -> f32 {
        match self {
            PipelineCapacity::Mk1 => Self::MK1_CAPACITY,
            PipelineCapacity::Mk2 => Self::MK2_CAPACITY,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConveyorSpeed {
    Mk1,
    Mk2,
    Mk3,
    Mk4,
    Mk5,
    Mk6,
}

impl ConveyorSpeed {
    pub const MK1_SPEED: f32 = 60.0;
    pub const MK2_SPEED: f32 = 120.0;
    pub const MK3_SPEED: f32 = 270.0;
    pub const MK4_SPEED: f32 = 480.0;
    pub const MK5_SPEED: f32 = 780.0;
    pub const MK6_SPEED: f32 = 1200.0;
}
impl ItemPerPin for ConveyorSpeed {
    fn item_per_min(&self) -> f32 {
        match self {
            ConveyorSpeed::Mk1 => Self::MK1_SPEED,
            ConveyorSpeed::Mk2 => Self::MK2_SPEED,
            ConveyorSpeed::Mk3 => Self::MK3_SPEED,
            ConveyorSpeed::Mk4 => Self::MK4_SPEED,
            ConveyorSpeed::Mk5 => Self::MK5_SPEED,
            ConveyorSpeed::Mk6 => Self::MK6_SPEED,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Train {
    pub train_id: u64,
    pub train_name: String,
    pub wagons: Vec<Wagon>,
}

impl Train {
    pub fn new(train_id: u64, train_name: impl Into<String>) -> Self {
        Self {
            train_id,
            train_name: train_name.into(),
            wagons: Vec::new(),
        }
    }

    pub fn with_wagon(mut self, wagon: Wagon) -> Self {
        self.wagons.push(wagon);
        self
    }

    pub fn add_wagon(&mut self, wagon: Wagon) {
        self.wagons.push(wagon);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Wagon {
    pub wagon_id: u64,
    pub wagon_type: WagonType,
    pub item: Item,
    pub quantity_per_min: f32,
}

impl Wagon {
    pub fn new(wagon_id: u64, wagon_type: WagonType, item: Item, quantity_per_min: f32) -> Self {
        Self {
            wagon_id,
            wagon_type,
            item,
            quantity_per_min,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WagonType {
    Cargo,
    Fluid,
}

// ===== Transport Trait Implementations =====

impl Transport for Train {
    fn get_items(&self) -> Vec<ItemFlow> {
        self.wagons
            .iter()
            .map(|w| ItemFlow {
                item: w.item,
                quantity_per_min: w.quantity_per_min,
            })
            .collect()
    }

    fn get_transport_id(&self) -> String {
        format!("TRN-{}", self.train_id)
    }

    fn get_transport_name(&self) -> Option<String> {
        Some(self.train_name.clone())
    }

    fn get_transport_type_name(&self) -> &'static str {
        "Train"
    }
}

impl Transport for Bus {
    fn get_items(&self) -> Vec<ItemFlow> {
        let mut items = Vec::new();

        // Add conveyor items
        items.extend(self.lines.iter().map(|c| ItemFlow {
            item: c.item,
            quantity_per_min: c.quantity_per_min,
        }));

        // Add pipeline items
        items.extend(self.pipelines.iter().map(|p| ItemFlow {
            item: p.item,
            quantity_per_min: p.quantity_per_min,
        }));

        items
    }

    fn get_transport_id(&self) -> String {
        format!("BUS-{}", self.bus_id)
    }

    fn get_transport_name(&self) -> Option<String> {
        Some(self.bus_name.clone())
    }

    fn get_transport_type_name(&self) -> &'static str {
        "Bus"
    }
}

impl Transport for TruckTransport {
    fn get_items(&self) -> Vec<ItemFlow> {
        vec![ItemFlow {
            item: self.item,
            quantity_per_min: self.quantity_per_min,
        }]
    }

    fn get_transport_id(&self) -> String {
        format!("TRK-{}", self.truck_id)
    }

    fn get_transport_name(&self) -> Option<String> {
        None
    }

    fn get_transport_type_name(&self) -> &'static str {
        "Truck"
    }
}

impl Transport for DroneTransport {
    fn get_items(&self) -> Vec<ItemFlow> {
        vec![ItemFlow {
            item: self.item,
            quantity_per_min: self.quantity_per_min,
        }]
    }

    fn get_transport_id(&self) -> String {
        format!("DRN-{}", self.drone_id)
    }

    fn get_transport_name(&self) -> Option<String> {
        None
    }

    fn get_transport_type_name(&self) -> &'static str {
        "Drone"
    }
}

// Delegate from enum to implementations
impl Transport for TransportType {
    fn get_items(&self) -> Vec<ItemFlow> {
        match self {
            TransportType::Bus(bus) => bus.get_items(),
            TransportType::Train(train) => train.get_items(),
            TransportType::Truck(truck) => truck.get_items(),
            TransportType::Drone(drone) => drone.get_items(),
        }
    }

    fn get_transport_id(&self) -> String {
        match self {
            TransportType::Bus(bus) => bus.get_transport_id(),
            TransportType::Train(train) => train.get_transport_id(),
            TransportType::Truck(truck) => truck.get_transport_id(),
            TransportType::Drone(drone) => drone.get_transport_id(),
        }
    }

    fn get_transport_name(&self) -> Option<String> {
        match self {
            TransportType::Bus(bus) => bus.get_transport_name(),
            TransportType::Train(train) => train.get_transport_name(),
            TransportType::Truck(truck) => truck.get_transport_name(),
            TransportType::Drone(drone) => drone.get_transport_name(),
        }
    }

    fn get_transport_type_name(&self) -> &'static str {
        match self {
            TransportType::Bus(bus) => bus.get_transport_type_name(),
            TransportType::Train(train) => train.get_transport_type_name(),
            TransportType::Truck(truck) => truck.get_transport_type_name(),
            TransportType::Drone(drone) => drone.get_transport_type_name(),
        }
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Item;
    #[test]
    fn test_conveyor_speed_constants() {
        assert_eq!(ConveyorSpeed::MK1_SPEED, 60.0);
        assert_eq!(ConveyorSpeed::MK2_SPEED, 120.0);
        assert_eq!(ConveyorSpeed::MK3_SPEED, 270.0);
        assert_eq!(ConveyorSpeed::MK4_SPEED, 480.0);
        assert_eq!(ConveyorSpeed::MK5_SPEED, 780.0);
        assert_eq!(ConveyorSpeed::MK6_SPEED, 1200.0);
    }

    #[test]
    fn test_conveyor_speed_item_per_min() {
        assert_eq!(ConveyorSpeed::Mk1.item_per_min(), 60.0);
        assert_eq!(ConveyorSpeed::Mk2.item_per_min(), 120.0);
        assert_eq!(ConveyorSpeed::Mk3.item_per_min(), 270.0);
        assert_eq!(ConveyorSpeed::Mk4.item_per_min(), 480.0);
        assert_eq!(ConveyorSpeed::Mk5.item_per_min(), 780.0);
        assert_eq!(ConveyorSpeed::Mk6.item_per_min(), 1200.0);
    }

    #[test]
    fn test_conveyor_constants_match_methods() {
        assert_eq!(ConveyorSpeed::MK1_SPEED, ConveyorSpeed::Mk1.item_per_min());
        assert_eq!(ConveyorSpeed::MK2_SPEED, ConveyorSpeed::Mk2.item_per_min());
        assert_eq!(ConveyorSpeed::MK3_SPEED, ConveyorSpeed::Mk3.item_per_min());
        assert_eq!(ConveyorSpeed::MK4_SPEED, ConveyorSpeed::Mk4.item_per_min());
        assert_eq!(ConveyorSpeed::MK5_SPEED, ConveyorSpeed::Mk5.item_per_min());
        assert_eq!(ConveyorSpeed::MK6_SPEED, ConveyorSpeed::Mk6.item_per_min());
    }

    #[test]
    fn test_pipeline_capacity_constants() {
        assert_eq!(PipelineCapacity::MK1_CAPACITY, 300.0);
        assert_eq!(PipelineCapacity::MK2_CAPACITY, 600.0);
    }

    #[test]
    fn test_pipeline_capacity_m3_per_min() {
        assert_eq!(PipelineCapacity::Mk1.m3_per_min(), 300.0);
        assert_eq!(PipelineCapacity::Mk2.m3_per_min(), 600.0);
    }

    #[test]
    fn test_pipeline_constants_match_methods() {
        assert_eq!(
            PipelineCapacity::MK1_CAPACITY,
            PipelineCapacity::Mk1.m3_per_min()
        );
        assert_eq!(
            PipelineCapacity::MK2_CAPACITY,
            PipelineCapacity::Mk2.m3_per_min()
        );
    }

    #[test]
    fn test_conveyor_speed_progression() {
        // Test that each tier is faster than the previous
        assert!(ConveyorSpeed::Mk2.item_per_min() > ConveyorSpeed::Mk1.item_per_min());
        assert!(ConveyorSpeed::Mk3.item_per_min() > ConveyorSpeed::Mk2.item_per_min());
        assert!(ConveyorSpeed::Mk4.item_per_min() > ConveyorSpeed::Mk3.item_per_min());
        assert!(ConveyorSpeed::Mk5.item_per_min() > ConveyorSpeed::Mk4.item_per_min());
        assert!(ConveyorSpeed::Mk6.item_per_min() > ConveyorSpeed::Mk5.item_per_min());
    }

    #[test]
    fn test_pipeline_capacity_progression() {
        // Test that Mk2 has higher capacity than Mk1
        assert!(PipelineCapacity::Mk2.m3_per_min() > PipelineCapacity::Mk1.m3_per_min());
    }

    // ===== NEW TESTS FOR PHASE 0 REFACTOR =====

    #[test]
    fn test_wagon_with_item() {
        let wagon = Wagon::new(1, WagonType::Cargo, Item::IronOre, 120.0);
        assert_eq!(wagon.wagon_id, 1);
        assert_eq!(wagon.wagon_type, WagonType::Cargo);
        assert_eq!(wagon.item, Item::IronOre);
        assert_eq!(wagon.quantity_per_min, 120.0);
    }

    #[test]
    fn test_conveyor_with_item() {
        let conveyor = Conveyor::new(1, ConveyorSpeed::Mk3, Item::CopperOre, 90.0);
        assert_eq!(conveyor.line_id, 1);
        assert_eq!(conveyor.speed, ConveyorSpeed::Mk3);
        assert_eq!(conveyor.item, Item::CopperOre);
        assert_eq!(conveyor.quantity_per_min, 90.0);
    }

    #[test]
    fn test_pipeline_with_item() {
        let pipeline = Pipeline::new(1, PipelineCapacity::Mk2, Item::Water, 450.0);
        assert_eq!(pipeline.pipeline_id, 1);
        assert_eq!(pipeline.capacity, PipelineCapacity::Mk2);
        assert_eq!(pipeline.item, Item::Water);
        assert_eq!(pipeline.quantity_per_min, 450.0);
    }

    #[test]
    fn test_train_get_items() {
        let train = Train {
            train_id: 1,
            train_name: "Iron Express".into(),
            wagons: vec![
                Wagon::new(1, WagonType::Cargo, Item::IronOre, 120.0),
                Wagon::new(2, WagonType::Cargo, Item::Coal, 60.0),
            ],
        };

        let items = train.get_items();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].item, Item::IronOre);
        assert_eq!(items[0].quantity_per_min, 120.0);
        assert_eq!(items[1].item, Item::Coal);
        assert_eq!(items[1].quantity_per_min, 60.0);
        assert_eq!(train.get_transport_id(), "TRN-1");
        assert_eq!(train.get_transport_name(), Some("Iron Express".to_string()));
        assert_eq!(train.get_transport_type_name(), "Train");
    }

    #[test]
    fn test_bus_get_items_conveyors_only() {
        let bus = Bus {
            bus_id: 1,
            bus_name: "Main Bus".into(),
            lines: vec![
                Conveyor::new(1, ConveyorSpeed::Mk3, Item::CopperOre, 90.0),
                Conveyor::new(2, ConveyorSpeed::Mk4, Item::IronOre, 180.0),
            ],
            pipelines: vec![],
        };

        let items = bus.get_items();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].item, Item::CopperOre);
        assert_eq!(items[1].item, Item::IronOre);
    }

    #[test]
    fn test_bus_get_items_mixed() {
        let bus = Bus {
            bus_id: 1,
            bus_name: "Mixed Bus".into(),
            lines: vec![Conveyor::new(1, ConveyorSpeed::Mk3, Item::CopperOre, 90.0)],
            pipelines: vec![Pipeline::new(1, PipelineCapacity::Mk2, Item::Water, 450.0)],
        };

        let items = bus.get_items();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].item, Item::CopperOre);
        assert_eq!(items[1].item, Item::Water);
    }

    #[test]
    fn test_truck_transport_get_items() {
        let truck = TruckTransport {
            truck_id: 1,
            item: Item::Concrete,
            quantity_per_min: 30.0,
        };

        let items = truck.get_items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].item, Item::Concrete);
        assert_eq!(items[0].quantity_per_min, 30.0);
    }

    #[test]
    fn test_drone_transport_get_items() {
        let drone = DroneTransport {
            drone_id: 1,
            item: Item::Computer,
            quantity_per_min: 15.0,
        };

        let items = drone.get_items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].item, Item::Computer);
        assert_eq!(items[0].quantity_per_min, 15.0);
    }

    #[test]
    fn test_transport_type_train_polymorphism() {
        let train = Train {
            train_id: 1,
            train_name: "Express".into(),
            wagons: vec![Wagon::new(1, WagonType::Cargo, Item::IronOre, 120.0)],
        };

        let transport = TransportType::Train(train);

        // Test polymorphic methods
        let items = transport.get_items();
        assert_eq!(items.len(), 1);
        assert_eq!(transport.get_transport_type_name(), "Train");
        assert_eq!(transport.get_transport_id(), "TRN-1");
        assert_eq!(transport.get_transport_name(), Some("Express".to_string()));
        assert_eq!(items[0].item, Item::IronOre);
        assert_eq!(items[0].quantity_per_min, 120.0);
    }

    #[test]
    fn test_transport_type_bus_polymorphism() {
        let bus = Bus {
            bus_id: 5,
            bus_name: "Main Line".into(),
            lines: vec![Conveyor::new(1, ConveyorSpeed::Mk3, Item::CopperOre, 90.0)],
            pipelines: vec![],
        };

        let transport = TransportType::Bus(bus);

        assert_eq!(transport.get_transport_type_name(), "Bus");
        assert_eq!(transport.get_transport_id(), "BUS-5");
        assert_eq!(
            transport.get_transport_name(),
            Some("Main Line".to_string())
        );
    }

    #[test]
    fn test_transport_type_truck_polymorphism() {
        let truck = TruckTransport {
            truck_id: 3,
            item: Item::Concrete,
            quantity_per_min: 30.0,
        };

        let transport = TransportType::Truck(truck);

        assert_eq!(transport.get_transport_type_name(), "Truck");
        assert_eq!(transport.get_transport_id(), "TRK-3");
        assert_eq!(transport.get_transport_name(), None);
    }

    #[test]
    fn test_transport_type_drone_polymorphism() {
        let drone = DroneTransport {
            drone_id: 7,
            item: Item::Computer,
            quantity_per_min: 15.0,
        };

        let transport = TransportType::Drone(drone);

        assert_eq!(transport.get_transport_type_name(), "Drone");
        assert_eq!(transport.get_transport_id(), "DRN-7");
        assert_eq!(transport.get_transport_name(), None);
    }

    #[test]
    fn test_logistics_flux_get_items() {
        let train = Train {
            train_id: 1,
            train_name: "Express".into(),
            wagons: vec![
                Wagon::new(1, WagonType::Cargo, Item::IronOre, 120.0),
                Wagon::new(2, WagonType::Cargo, Item::Coal, 60.0),
            ],
        };

        let flux = LogisticsFlux {
            id: 1,
            from_factory: 1,
            to_factory: 2,
            transport_type: TransportType::Train(train),
            transport_details: "Main line".into(),
        };

        let items = flux.get_items();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].item, Item::IronOre);
        assert_eq!(items[0].quantity_per_min, 120.0);
        assert_eq!(items[1].item, Item::Coal);
        assert_eq!(items[1].quantity_per_min, 60.0);
    }

    #[test]
    fn test_logistics_flux_total_quantity() {
        let train = Train {
            train_id: 1,
            train_name: "Express".into(),
            wagons: vec![
                Wagon::new(1, WagonType::Cargo, Item::IronOre, 120.0),
                Wagon::new(2, WagonType::Cargo, Item::Coal, 60.0),
            ],
        };

        let flux = LogisticsFlux {
            id: 1,
            from_factory: 1,
            to_factory: 2,
            transport_type: TransportType::Train(train),
            transport_details: "".into(),
        };
        assert_eq!(flux.total_quantity_per_min(), 180.0);
    }

    #[test]
    fn test_item_flow_creation() {
        let item_flow = ItemFlow {
            item: Item::IronOre,
            quantity_per_min: 120.0,
        };

        assert_eq!(item_flow.item, Item::IronOre);
        assert_eq!(item_flow.quantity_per_min, 120.0);
    }
}
