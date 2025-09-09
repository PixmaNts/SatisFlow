use crate::engine::calculator::ProductionTracker;
use crate::models::*;
use std::collections::HashMap;

/// Initialize a sample factory setup for testing and demonstration
/// Creates two factories: Northern Forest (iron-based) and Northern Cliff (copper-based)
/// with production lines and conveyor logistics between them
pub fn create_sample_factory_setup() -> ProductionTracker {
    let mut tracker = ProductionTracker::new();

    // Load static game data first
    tracker.load_static_data();
    let idx = tracker
        .index
        .as_ref()
        .expect("index available after static data load")
        .clone();

    // Create Northern Forest factory (iron-based production)
    let forest_factory = create_northern_forest_factory(&idx);
    tracker.add_factory(forest_factory);

    // Create Northern Cliff factory (copper-based production)
    let cliff_factory = create_northern_cliff_factory(&idx);
    tracker.add_factory(cliff_factory);

    // Set up conveyor logistics between factories
    setup_inter_factory_logistics(&mut tracker);

    tracker
}

/// Create the Northern Forest factory focused on iron production
fn create_northern_forest_factory(idx: &GameIndex) -> Factory {
    let factory_id = FactoryId("fac_northern_forest_1".to_string());

    let mut production_lines = Vec::new();

    // Iron rod production line
    let iron_rods = ProductionLine {
        id: ProductionLineId("line_northern_forest_iron_rod_1".to_string()),
        factory_id: factory_id.clone(),
        recipe_id: idx.recipe_id_by_name["Iron Rod"].clone(),
        machine_count: 2,
        clock_ratio: 1.0,
        group_name: Some("Basic Iron Processing".to_string()),
        output_routing: HashMap::new(),
        strange_matter_boosted: 0,
    };
    production_lines.push(iron_rods);

    // Iron plate production line
    let iron_plates = ProductionLine {
        id: ProductionLineId("line_northern_forest_iron_plate_1".to_string()),
        factory_id: factory_id.clone(),
        recipe_id: idx.recipe_id_by_name["Iron Plate"].clone(),
        machine_count: 2,
        clock_ratio: 1.0,
        group_name: Some("Basic Iron Processing".to_string()),
        output_routing: HashMap::new(),
        strange_matter_boosted: 0,
    };
    production_lines.push(iron_plates);

    // Reinforced iron plate production line
    let reinforced_plates = ProductionLine {
        id: ProductionLineId("line_northern_forest_reinforced_plate_1".to_string()),
        factory_id: factory_id.clone(),
        recipe_id: idx.recipe_id_by_name["Reinforced Iron Plate"].clone(),
        machine_count: 1,
        clock_ratio: 1.0,
        group_name: Some("Advanced Iron Processing".to_string()),
        output_routing: HashMap::new(),
        strange_matter_boosted: 0,
    };
    production_lines.push(reinforced_plates);

    // Raw inputs - iron ore mining
    let raw_inputs = vec![RawInput {
        id: RawInputId("raw_fac_northern_forest_1_iron_ore_1_input".to_string()),
        item: idx.item_id_by_name["Iron Ore"].clone(),
        quantity_per_min: 240.0, // 3 normal nodes * 80 per min
        source_type: "Miner Mk.1".to_string(),
        comment: Some("Three normal nodes clustered near main base".to_string()),
    }];

    Factory {
        id: factory_id,
        name: "Northern Forest".to_string(),
        raw_inputs,
        logistics_inputs: Vec::new(),
        production_lines,
    }
}

/// Create the Northern Cliff factory focused on copper production
fn create_northern_cliff_factory(idx: &GameIndex) -> Factory {
    let factory_id = FactoryId("fac_northern_cliff_1".to_string());

    let mut production_lines = Vec::new();

    // Wire production line
    let copper_wire = ProductionLine {
        id: ProductionLineId("line_northern_cliff_wire_1".to_string()),
        factory_id: factory_id.clone(),
        recipe_id: idx.recipe_id_by_name["Wire"].clone(),
        machine_count: 2,
        clock_ratio: 1.0,
        group_name: Some("Basic Copper Processing".to_string()),
        output_routing: HashMap::new(),
        strange_matter_boosted: 0,
    };
    production_lines.push(copper_wire);

    // Cable production line
    let cable = ProductionLine {
        id: ProductionLineId("line_northern_cliff_cable_1".to_string()),
        factory_id: factory_id.clone(),
        recipe_id: idx.recipe_id_by_name["Cable"].clone(),
        machine_count: 1,
        clock_ratio: 1.0,
        group_name: Some("Basic Copper Processing".to_string()),
        output_routing: HashMap::new(),
        strange_matter_boosted: 0,
    };
    production_lines.push(cable);

    // Copper sheet production line
    let copper_sheet = ProductionLine {
        id: ProductionLineId("line_northern_cliff_copper_sheet_1".to_string()),
        factory_id: factory_id.clone(),
        recipe_id: idx.recipe_id_by_name["Copper Sheet"].clone(),
        machine_count: 1,
        clock_ratio: 1.0,
        group_name: Some("Advanced Copper Processing".to_string()),
        output_routing: HashMap::new(),
        strange_matter_boosted: 0,
    };
    production_lines.push(copper_sheet);

    // Raw inputs - copper ore mining
    let raw_inputs = vec![RawInput {
        id: RawInputId("raw_fac_northern_cliff_1_copper_ore_1_input".to_string()),
        item: idx.item_id_by_name["Copper Ore"].clone(),
        quantity_per_min: 180.0, // 2 normal nodes * 90 per min
        source_type: "Miner Mk.1".to_string(),
        comment: Some("Two nodes on cliff, good for early game".to_string()),
    }];

    Factory {
        id: factory_id,
        name: "Northern Cliff".to_string(),
        raw_inputs,
        logistics_inputs: Vec::new(),
        production_lines,
    }
}

/// Set up logistics connections between the two factories
/// Creates 2 bus lines with 3 conveyors each (6 total conveyor connections)
fn setup_inter_factory_logistics(tracker: &mut ProductionTracker) {
    let forest_id = FactoryId("fac_northern_forest_1".to_string());
    let cliff_id = FactoryId("fac_northern_cliff_1".to_string());
    // Helper to append detail to a BUS id
    fn with_detail(base: &LogisticsFluxId, detail: &str) -> LogisticsFluxId {
        if detail.is_empty() {
            return base.clone();
        }
        LogisticsFluxId(format!("{}-{}", base.0, detail))
    }

    // Clone needed ItemIds from index first to avoid borrow conflicts
    let (iron_ingot, iron_rod, iron_plate, copper_ingot, wire, cable) = {
        let idx = tracker.index.as_ref().expect("index available");
        (
            idx.item_id_by_name["Iron Ingot"].clone(),
            idx.item_id_by_name["Iron Rod"].clone(),
            idx.item_id_by_name["Iron Plate"].clone(),
            idx.item_id_by_name["Copper Ingot"].clone(),
            idx.item_id_by_name["Wire"].clone(),
            idx.item_id_by_name["Cable"].clone(),
        )
    };

    // Bus 1: Forest -> Cliff (Iron products to Cliff)
    let bus1 = tracker.generate_logistics_id(&TransportType::Conveyor);
    // Conveyor 1: Iron Ingots
    let iron_transport = LogisticsFlux {
        id: with_detail(&bus1, "C01"),
        from_factory: forest_id.clone(),
        to_factory: cliff_id.clone(),
        item: iron_ingot,
        quantity_per_min: 120.0,
        transport_type: TransportType::Conveyor,
        transport_details: "Iron ingots from forest smelters to cliff for mixed production"
            .to_string(),
    };
    tracker
        .add_logistics_flux(iron_transport)
        .expect("Failed to add iron transport");

    // Conveyor 2: Iron Rods
    let iron_rod_transport = LogisticsFlux {
        id: with_detail(&bus1, "C02"),
        from_factory: forest_id.clone(),
        to_factory: cliff_id.clone(),
        item: iron_rod,
        quantity_per_min: 80.0,
        transport_type: TransportType::Conveyor,
        transport_details: "Iron rods for screws and other components at cliff".to_string(),
    };
    tracker
        .add_logistics_flux(iron_rod_transport)
        .expect("Failed to add iron rod transport");

    // Conveyor 3: Iron Plates
    let iron_plate_transport = LogisticsFlux {
        id: with_detail(&bus1, "C03"),
        from_factory: forest_id.clone(),
        to_factory: cliff_id.clone(),
        item: iron_plate,
        quantity_per_min: 60.0,
        transport_type: TransportType::Conveyor,
        transport_details: "Iron plates for advanced manufacturing at cliff".to_string(),
    };
    tracker
        .add_logistics_flux(iron_plate_transport)
        .expect("Failed to add iron plate transport");

    // Bus 2: Cliff -> Forest (Copper products to Forest)
    let bus2 = tracker.generate_logistics_id(&TransportType::Conveyor);
    // Conveyor 1: Copper Ingots
    let copper_transport = LogisticsFlux {
        id: with_detail(&bus2, "C01"),
        from_factory: cliff_id.clone(),
        to_factory: forest_id.clone(),
        item: copper_ingot,
        quantity_per_min: 90.0,
        transport_type: TransportType::Conveyor,
        transport_details: "Copper ingots from cliff smelters to forest".to_string(),
    };
    tracker
        .add_logistics_flux(copper_transport)
        .expect("Failed to add copper transport");

    // Conveyor 2: Wire
    let wire_transport = LogisticsFlux {
        id: with_detail(&bus2, "C02"),
        from_factory: cliff_id.clone(),
        to_factory: forest_id.clone(),
        item: wire,
        quantity_per_min: 150.0,
        transport_type: TransportType::Conveyor,
        transport_details: "Wire production for electronics at forest".to_string(),
    };
    tracker
        .add_logistics_flux(wire_transport)
        .expect("Failed to add wire transport");

    // Conveyor 3: Cable
    let cable_transport = LogisticsFlux {
        id: with_detail(&bus2, "C03"),
        from_factory: cliff_id.clone(),
        to_factory: forest_id.clone(),
        item: cable,
        quantity_per_min: 45.0,
        transport_type: TransportType::Conveyor,
        transport_details: "Cables for power and electronics at forest".to_string(),
    };
    tracker
        .add_logistics_flux(cable_transport)
        .expect("Failed to add cable transport");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_factory_creation() {
        let tracker = create_sample_factory_setup();

        // Verify both factories exist
        assert_eq!(tracker.factories.len(), 2);
        assert!(tracker
            .factories
            .contains_key(&FactoryId("fac_northern_forest_1".to_string())));
        assert!(tracker
            .factories
            .contains_key(&FactoryId("fac_northern_cliff_1".to_string())));

        // Verify production lines
        let forest = tracker
            .factories
            .get(&FactoryId("fac_northern_forest_1".to_string()))
            .unwrap();
        assert_eq!(forest.production_lines.len(), 3); // 3 iron-based production lines

        let cliff = tracker
            .factories
            .get(&FactoryId("fac_northern_cliff_1".to_string()))
            .unwrap();
        assert_eq!(cliff.production_lines.len(), 3); // 3 copper-based production lines

        // Verify logistics connections
        assert_eq!(tracker.logistics_fluxes.len(), 6); // 2 buses × 3 conveyors each
    }

    #[test]
    fn test_logistics_ids_follow_convention() {
        let tracker = create_sample_factory_setup();

        // Verify logistics IDs follow the LG-BUS-XXX-CXX pattern (Bus with conveyors)
        let expected_ids = vec![
            "LG-BUS-001-C01", // Iron ingot
            "LG-BUS-001-C02", // Iron rod
            "LG-BUS-001-C03", // Iron plate
            "LG-BUS-002-C01", // Copper ingot
            "LG-BUS-002-C02", // Wire
            "LG-BUS-002-C03", // Cable
        ];

        for expected_id in expected_ids {
            let logistics_id = LogisticsFluxId(expected_id.to_string());
            assert!(tracker.logistics_fluxes.contains_key(&logistics_id));
        }
    }

    #[test]
    fn test_production_overview_calculation() {
        let tracker = create_sample_factory_setup();
        let overview = tracker.calculate_overview();

        // Should have calculated production and consumption for various items
        assert!(!overview.is_empty());

        // Should track iron and copper items
        assert!(overview.iter().any(|item| item.item_name.contains("Iron")));
        assert!(overview
            .iter()
            .any(|item| item.item_name.contains("Copper")));
    }

    #[test]
    fn test_factory_raw_inputs() {
        let tracker = create_sample_factory_setup();

        let idx = tracker.index.as_ref().unwrap();
        let forest = tracker
            .factories
            .get(&FactoryId("fac_northern_forest_1".to_string()))
            .unwrap();
        assert_eq!(forest.raw_inputs.len(), 1);
        assert_eq!(forest.raw_inputs[0].item, idx.item_id_by_name["Iron Ore"]);

        let cliff = tracker
            .factories
            .get(&FactoryId("fac_northern_cliff_1".to_string()))
            .unwrap();
        assert_eq!(cliff.raw_inputs.len(), 1);
        assert_eq!(cliff.raw_inputs[0].item, idx.item_id_by_name["Copper Ore"]);
    }

    #[test]
    fn test_static_data_loading() {
        let mut tracker = ProductionTracker::new();
        tracker.load_static_data();

        // Print some items to debug
        println!(
            "Available items: {:?}",
            tracker.items.keys().take(10).collect::<Vec<_>>()
        );

        // Check if Iron Ingot exists
        assert!(
            tracker.items.contains_key("Iron Ingot"),
            "Iron Ingot should exist in static data"
        );
        assert!(
            tracker.items.contains_key("Iron Ore"),
            "Iron Ore should exist in static data"
        );
        assert!(
            tracker.items.contains_key("Wire"),
            "Wire should exist in static data"
        );
    }
}
