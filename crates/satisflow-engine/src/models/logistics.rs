use serde::{Deserialize, Serialize};

use crate::models::{FactoryId, Item, LogisticsId};

pub trait ItemPerPin {
    fn item_per_min(&self) -> f32;
}

pub trait FluidPerMin {
    fn m3_per_min(&self) -> f32;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemFlow {
    pub item: Item,
    pub quantity_per_min: f32,
}

pub trait Transport {
    fn get_items(&self) -> Vec<ItemFlow>;
    fn get_transport_id(&self) -> String;
    fn get_transport_name(&self) -> Option<String>;
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
    pub id: LogisticsId,
    pub from_factory: FactoryId,
    pub to_factory: FactoryId,
    pub transport_type: TransportType,
    pub transport_details: String,
}

impl LogisticsFlux {
    pub fn get_items(&self) -> Vec<ItemFlow> {
        self.transport_type.get_items()
    }

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

    /// Find and remove a conveyor by its line_id (as string).
    /// Returns None if no conveyor with the given ID exists.
    pub fn remove_conveyor(&mut self, line_id: &str) -> Option<Conveyor> {
        let target_id = line_id.parse::<u64>().ok()?;
        let pos = self.lines.iter().position(|c| c.line_id == target_id)?;
        Some(self.lines.remove(pos))
    }

    /// Find and remove a pipeline by its pipeline_id (as string).
    /// Returns None if no pipeline with the given ID exists.
    pub fn remove_pipeline(&mut self, pipeline_id: &str) -> Option<Pipeline> {
        let target_id = pipeline_id.parse::<u64>().ok()?;
        let pos = self
            .pipelines
            .iter()
            .position(|p| p.pipeline_id == target_id)?;
        Some(self.pipelines.remove(pos))
    }

    /// Get a mutable reference to a conveyor by its line_id.
    /// Returns None if no conveyor with the given ID exists.
    pub fn get_conveyor_mut(&mut self, line_id: &str) -> Option<&mut Conveyor> {
        let target_id = line_id.parse::<u64>().ok()?;
        self.lines.iter_mut().find(|c| c.line_id == target_id)
    }

    /// Get a mutable reference to a pipeline by its pipeline_id.
    /// Returns None if no pipeline with the given ID exists.
    pub fn get_pipeline_mut(&mut self, pipeline_id: &str) -> Option<&mut Pipeline> {
        let target_id = pipeline_id.parse::<u64>().ok()?;
        self.pipelines
            .iter_mut()
            .find(|p| p.pipeline_id == target_id)
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

    pub fn remove_wagon(&mut self, wagon_id: &str) -> Option<Wagon> {
        let id = wagon_id.parse::<u64>().ok()?;
        let pos = self.wagons.iter().position(|w| w.wagon_id == id)?;
        Some(self.wagons.swap_remove(pos))
    }

    pub fn get_wagon_mut(&mut self, wagon_id: &str) -> Option<&mut Wagon> {
        let id = wagon_id.parse::<u64>().ok()?;
        self.wagons.iter_mut().find(|w| w.wagon_id == id)
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
        items.extend(self.lines.iter().map(|c| ItemFlow {
            item: c.item,
            quantity_per_min: c.quantity_per_min,
        }));
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

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn uuid_from_u64(value: u64) -> Uuid {
        Uuid::from_u128(value as u128)
    }

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
    fn test_pipeline_capacity_constants() {
        assert_eq!(PipelineCapacity::MK1_CAPACITY, 300.0);
        assert_eq!(PipelineCapacity::MK2_CAPACITY, 600.0);
    }

    #[test]
    fn test_wagon_with_item() {
        let wagon = Wagon::new(1, WagonType::Cargo, Item::IronOre, 120.0);
        assert_eq!(wagon.wagon_id, 1);
        assert_eq!(wagon.wagon_type, WagonType::Cargo);
        assert_eq!(wagon.item, Item::IronOre);
        assert_eq!(wagon.quantity_per_min, 120.0);
    }

    #[test]
    fn test_train_remove_wagon_success() {
        let mut train = Train::new(1, "Test Train");
        train.add_wagon(Wagon::new(1, WagonType::Cargo, Item::IronOre, 120.0));
        train.add_wagon(Wagon::new(2, WagonType::Cargo, Item::Coal, 60.0));
        train.add_wagon(Wagon::new(3, WagonType::Fluid, Item::Water, 300.0));

        let removed = train.remove_wagon("2");
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().wagon_id, 2);
        assert_eq!(train.wagons.len(), 2);

        let ids: Vec<u64> = train.wagons.iter().map(|w| w.wagon_id).collect();
        assert!(ids.contains(&1));
        assert!(ids.contains(&3));
        assert!(!ids.contains(&2));
    }

    #[test]
    fn test_train_remove_wagon_not_found() {
        let mut train = Train::new(1, "Test Train");
        train.add_wagon(Wagon::new(1, WagonType::Cargo, Item::IronOre, 120.0));

        let removed = train.remove_wagon("99");
        assert!(removed.is_none());
        assert_eq!(train.wagons.len(), 1);
    }

    #[test]
    fn test_train_remove_wagon_invalid_id() {
        let mut train = Train::new(1, "Test Train");
        train.add_wagon(Wagon::new(1, WagonType::Cargo, Item::IronOre, 120.0));

        let removed = train.remove_wagon("not-a-number");
        assert!(removed.is_none());
        assert_eq!(train.wagons.len(), 1);
    }

    #[test]
    fn test_train_remove_wagon_empty_train() {
        let mut train = Train::new(1, "Empty Train");

        let removed = train.remove_wagon("1");
        assert!(removed.is_none());
        assert!(train.wagons.is_empty());
    }

    #[test]
    fn test_train_get_wagon_mut_success() {
        let mut train = Train::new(1, "Test Train");
        train.add_wagon(Wagon::new(1, WagonType::Cargo, Item::IronOre, 120.0));
        train.add_wagon(Wagon::new(2, WagonType::Cargo, Item::Coal, 60.0));

        if let Some(wagon) = train.get_wagon_mut("2") {
            wagon.quantity_per_min = 100.0;
            wagon.item = Item::CopperOre;
        }

        let wagon = train.wagons.iter().find(|w| w.wagon_id == 2).unwrap();
        assert_eq!(wagon.quantity_per_min, 100.0);
        assert_eq!(wagon.item, Item::CopperOre);
    }

    #[test]
    fn test_train_get_wagon_mut_not_found() {
        let mut train = Train::new(1, "Test Train");
        train.add_wagon(Wagon::new(1, WagonType::Cargo, Item::IronOre, 120.0));

        let wagon = train.get_wagon_mut("99");
        assert!(wagon.is_none());
    }

    #[test]
    fn test_train_get_wagon_mut_invalid_id() {
        let mut train = Train::new(1, "Test Train");
        train.add_wagon(Wagon::new(1, WagonType::Cargo, Item::IronOre, 120.0));

        let wagon = train.get_wagon_mut("invalid");
        assert!(wagon.is_none());
    }

    #[test]
    fn test_train_wagon_mutations_idempotent() {
        let mut train = Train::new(1, "Test Train");
        train.add_wagon(Wagon::new(10, WagonType::Cargo, Item::IronOre, 120.0));

        assert!(train.get_wagon_mut("10").is_some());
        assert!(train.get_wagon_mut("10").is_some());

        let removed = train.remove_wagon("10");
        assert!(removed.is_some());

        assert!(train.get_wagon_mut("10").is_none());
        assert!(train.remove_wagon("10").is_none());
    }

    // ===== BUS CHILD ENTITY MUTATION TESTS =====

    #[test]
    fn test_bus_remove_conveyor_success() {
        let mut bus = Bus::new(1, "Test Bus");
        bus.add_conveyor(Conveyor::new(1, ConveyorSpeed::Mk3, Item::CopperOre, 90.0));
        bus.add_conveyor(Conveyor::new(2, ConveyorSpeed::Mk4, Item::IronOre, 180.0));
        bus.add_conveyor(Conveyor::new(3, ConveyorSpeed::Mk5, Item::Coal, 120.0));

        // Remove the middle conveyor
        let removed = bus.remove_conveyor("2");

        assert!(removed.is_some());
        let removed = removed.unwrap();
        assert_eq!(removed.line_id, 2);
        assert_eq!(removed.item, Item::IronOre);
        assert_eq!(bus.lines.len(), 2);

        // Verify remaining conveyors are correct
        assert_eq!(bus.lines[0].line_id, 1);
        assert_eq!(bus.lines[1].line_id, 3);
    }

    #[test]
    fn test_bus_remove_pipeline_success() {
        let mut bus = Bus::new(1, "Test Bus");
        bus.add_pipeline(Pipeline::new(10, PipelineCapacity::Mk1, Item::Water, 200.0));
        bus.add_pipeline(Pipeline::new(
            20,
            PipelineCapacity::Mk2,
            Item::CrudeOil,
            400.0,
        ));
        bus.add_pipeline(Pipeline::new(
            30,
            PipelineCapacity::Mk1,
            Item::HeavyOilResidue,
            150.0,
        ));

        // Remove the last pipeline
        let removed = bus.remove_pipeline("30");

        assert!(removed.is_some());
        let removed = removed.unwrap();
        assert_eq!(removed.pipeline_id, 30);
        assert_eq!(removed.item, Item::HeavyOilResidue);
        assert_eq!(bus.pipelines.len(), 2);

        // Verify remaining pipelines are correct
        assert_eq!(bus.pipelines[0].pipeline_id, 10);
        assert_eq!(bus.pipelines[1].pipeline_id, 20);
    }

    #[test]
    fn test_bus_remove_nonexistent() {
        let mut bus = Bus::new(1, "Test Bus");
        bus.add_conveyor(Conveyor::new(1, ConveyorSpeed::Mk3, Item::CopperOre, 90.0));
        bus.add_pipeline(Pipeline::new(10, PipelineCapacity::Mk1, Item::Water, 200.0));

        // Try to remove non-existent conveyor
        let result = bus.remove_conveyor("999");
        assert!(result.is_none());
        assert_eq!(bus.lines.len(), 1); // Should still have original

        // Try to remove non-existent pipeline
        let result = bus.remove_pipeline("999");
        assert!(result.is_none());
        assert_eq!(bus.pipelines.len(), 1); // Should still have original

        // Try to remove with invalid ID string
        let result = bus.remove_conveyor("not-a-number");
        assert!(result.is_none());
    }

    #[test]
    fn test_bus_get_conveyor_mut_success() {
        let mut bus = Bus::new(1, "Test Bus");
        bus.add_conveyor(Conveyor::new(1, ConveyorSpeed::Mk3, Item::CopperOre, 90.0));
        bus.add_conveyor(Conveyor::new(2, ConveyorSpeed::Mk4, Item::IronOre, 180.0));

        // Get mutable reference and modify
        let conveyor = bus.get_conveyor_mut("2");
        assert!(conveyor.is_some());

        let conveyor = conveyor.unwrap();
        conveyor.quantity_per_min = 250.0;

        // Verify the modification persisted
        assert_eq!(bus.lines[1].quantity_per_min, 250.0);
    }

    #[test]
    fn test_bus_get_pipeline_mut_success() {
        let mut bus = Bus::new(1, "Test Bus");
        bus.add_pipeline(Pipeline::new(10, PipelineCapacity::Mk1, Item::Water, 200.0));
        bus.add_pipeline(Pipeline::new(
            20,
            PipelineCapacity::Mk2,
            Item::CrudeOil,
            400.0,
        ));

        // Get mutable reference and modify
        let pipeline = bus.get_pipeline_mut("20");
        assert!(pipeline.is_some());

        let pipeline = pipeline.unwrap();
        pipeline.quantity_per_min = 500.0;

        // Verify the modification persisted
        assert_eq!(bus.pipelines[1].quantity_per_min, 500.0);
    }

    #[test]
    fn test_bus_get_conveyor_mut_nonexistent() {
        let mut bus = Bus::new(1, "Test Bus");
        bus.add_conveyor(Conveyor::new(1, ConveyorSpeed::Mk3, Item::CopperOre, 90.0));

        // Try to get non-existent conveyor
        let result = bus.get_conveyor_mut("999");
        assert!(result.is_none());

        // Try with invalid ID string
        let result = bus.get_conveyor_mut("invalid");
        assert!(result.is_none());
    }

    #[test]
    fn test_bus_get_pipeline_mut_nonexistent() {
        let mut bus = Bus::new(1, "Test Bus");
        bus.add_pipeline(Pipeline::new(10, PipelineCapacity::Mk1, Item::Water, 200.0));

        // Try to get non-existent pipeline
        let result = bus.get_pipeline_mut("999");
        assert!(result.is_none());

        // Try with invalid ID string
        let result = bus.get_pipeline_mut("invalid");
        assert!(result.is_none());
    }

    #[test]
    fn test_bus_child_mutations_multiple_operations() {
        let mut bus = Bus::new(1, "Test Bus");

        // Add multiple conveyors and pipelines
        bus.add_conveyor(Conveyor::new(1, ConveyorSpeed::Mk3, Item::CopperOre, 90.0));
        bus.add_conveyor(Conveyor::new(2, ConveyorSpeed::Mk4, Item::IronOre, 180.0));
        bus.add_pipeline(Pipeline::new(10, PipelineCapacity::Mk1, Item::Water, 200.0));
        bus.add_pipeline(Pipeline::new(
            20,
            PipelineCapacity::Mk2,
            Item::CrudeOil,
            400.0,
        ));

        // Modify via mutable reference
        if let Some(conv) = bus.get_conveyor_mut("1") {
            conv.quantity_per_min = 100.0;
        }
        if let Some(pipe) = bus.get_pipeline_mut("10") {
            pipe.quantity_per_min = 250.0;
        }

        // Remove some items
        bus.remove_conveyor("2");
        bus.remove_pipeline("20");

        // Verify final state
        assert_eq!(bus.lines.len(), 1);
        assert_eq!(bus.pipelines.len(), 1);
        assert_eq!(bus.lines[0].quantity_per_min, 100.0);
        assert_eq!(bus.pipelines[0].quantity_per_min, 250.0);
    }
}
