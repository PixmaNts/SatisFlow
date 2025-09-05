use satisflow::*;
use std::collections::HashMap;

fn tracker_with_minimal_data() -> ProductionTracker {
    let mut t = ProductionTracker::new();
    for n in ["Iron Ore", "Iron Ingot"] {
        t.items.insert(n.to_string(), Item { name: n.to_string(), category: "Test".to_string(), description: String::new() });
    }
    t.recipes.insert(
        "Iron Ingot".to_string(),
        Recipe { name: "Iron Ingot".to_string(), machine_type: "Smelter".to_string(), base_duration: 2.0,
            inputs: vec![RecipeIngredient { item: "Iron Ore".to_string(), quantity_per_min: 30.0 }],
            outputs: vec![RecipeIngredient { item: "Iron Ingot".to_string(), quantity_per_min: 30.0 }],
        }
    );
    t.rebuild_index().unwrap();
    t
}

#[test]
fn add_production_line_validates_ranges() {
    let mut t = tracker_with_minimal_data();
    let fid = t.generate_factory_id("F");
    t.add_factory(Factory { id: fid.clone(), name: "F".into(), raw_inputs: vec![], logistics_inputs: vec![], production_lines: vec![] });
    let rid = t.index.as_ref().unwrap().recipe_id_by_name["Iron Ingot"].clone();
    // Bad machine count
    let bad_mc = ProductionLine { id: t.generate_line_id(&fid, "Iron Ingot"), factory_id: fid.clone(), recipe_id: rid.clone(), machine_count: 0, clock_ratio: 1.0, group_name: None, output_routing: HashMap::new(), strange_matter_boosted: 0 };
    assert!(t.add_production_line(bad_mc).is_err());
    // Bad clock ratio
    let bad_clock = ProductionLine { id: t.generate_line_id(&fid, "Iron Ingot"), factory_id: fid.clone(), recipe_id: rid, machine_count: 1, clock_ratio: 3.0, group_name: None, output_routing: HashMap::new(), strange_matter_boosted: 0 };
    assert!(t.add_production_line(bad_clock).is_err());
}

#[test]
fn add_logistics_flux_validates_factories_and_qty() {
    let mut t = tracker_with_minimal_data();
    let fid_a = t.generate_factory_id("A");
    let fid_b = t.generate_factory_id("B");
    t.add_factory(Factory { id: fid_a.clone(), name: "A".into(), raw_inputs: vec![], logistics_inputs: vec![], production_lines: vec![] });
    t.add_factory(Factory { id: fid_b.clone(), name: "B".into(), raw_inputs: vec![], logistics_inputs: vec![], production_lines: vec![] });
    let iron_ingot = t.index.as_ref().unwrap().item_id_by_name["Iron Ingot"].clone();
    // zero qty
    let flux_bad = LogisticsFlux { id: t.generate_logistics_id(&TransportType::Conveyor), from_factory: fid_a.clone(), to_factory: fid_b.clone(), item: iron_ingot.clone(), quantity_per_min: 0.0, transport_type: TransportType::Conveyor, transport_details: String::new() };
    assert!(t.add_logistics_flux(flux_bad).is_err());
    // same factory
    let flux_same = LogisticsFlux { id: t.generate_logistics_id(&TransportType::Conveyor), from_factory: fid_a.clone(), to_factory: fid_a.clone(), item: iron_ingot.clone(), quantity_per_min: 1.0, transport_type: TransportType::Conveyor, transport_details: String::new() };
    assert!(t.add_logistics_flux(flux_same).is_err());
    // ok
    let flux_ok = LogisticsFlux { id: t.generate_logistics_id(&TransportType::Conveyor), from_factory: fid_a.clone(), to_factory: fid_b.clone(), item: iron_ingot, quantity_per_min: 1.0, transport_type: TransportType::Conveyor, transport_details: String::new() };
    assert!(t.add_logistics_flux(flux_ok).is_ok());
}

