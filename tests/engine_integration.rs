use satisflow::*;
use std::collections::HashMap;

fn tracker_with_minimal_data() -> ProductionTracker {
    let mut t = ProductionTracker::new();
    for n in ["Iron Ore", "Iron Ingot", "Iron Plate"] {
        t.items.insert(n.to_string(), Item { name: n.to_string(), category: "Test".to_string(), description: String::new() });
    }
    t.recipes.insert(
        "Iron Ingot".to_string(),
        Recipe { name: "Iron Ingot".to_string(), machine_type: "Smelter".to_string(), base_duration: 2.0,
            inputs: vec![RecipeIngredient { item: "Iron Ore".to_string(), quantity_per_min: 30.0 }],
            outputs: vec![RecipeIngredient { item: "Iron Ingot".to_string(), quantity_per_min: 30.0 }],
        }
    );
    t.recipes.insert(
        "Iron Plate".to_string(),
        Recipe { name: "Iron Plate".to_string(), machine_type: "Constructor".to_string(), base_duration: 6.0,
            inputs: vec![RecipeIngredient { item: "Iron Ingot".to_string(), quantity_per_min: 30.0 }],
            outputs: vec![RecipeIngredient { item: "Iron Plate".to_string(), quantity_per_min: 20.0 }],
        }
    );
    t.rebuild_index().unwrap();
    t
}

#[test]
fn end_to_end_add_factory_line_and_logistics() {
    let mut t = tracker_with_minimal_data();

    // Add factory
    let fid = t.generate_factory_id("Assembly A");
    let idx = t.index.as_ref().unwrap();
    let iron_ore = idx.item_id_by_name["Iron Ore"].clone();
    let iron_ingot_recipe = idx.recipe_id_by_name["Iron Ingot"].clone();
    t.add_factory(Factory { id: fid.clone(), name: "Assembly A".to_string(), raw_inputs: vec![RawInput { item: iron_ore, quantity_per_min: 60.0, source_type: "Test".to_string() }], logistics_inputs: vec![], production_lines: vec![] });

    // Add line
    let line = ProductionLine { id: t.generate_line_id(&fid, "Iron Ingot"), factory_id: fid.clone(), recipe_id: iron_ingot_recipe, machine_count: 2, clock_ratio: 1.0, group_name: None, output_routing: HashMap::new(), strange_matter_boosted: 0 };
    t.add_production_line(line).expect("line insert");

    // Add logistics flux (conveyor)
    let id = t.generate_logistics_id(&TransportType::Conveyor);
    let iron_plate = t.index.as_ref().unwrap().item_id_by_name["Iron Plate"].clone();
    // Prepare destination factory per validation rules
    let fid_dest = t.generate_factory_id("Dest");
    t.add_factory(Factory { id: fid_dest.clone(), name: "Dest".into(), raw_inputs: vec![], logistics_inputs: vec![], production_lines: vec![] });
    let flux = LogisticsFlux { id: id.clone(), from_factory: fid.clone(), to_factory: fid_dest.clone(), item: iron_plate, quantity_per_min: 10.0, transport_type: TransportType::Conveyor, transport_details: "Bus 001, Conveyor 01".to_string() };
    t.add_logistics_flux(flux).expect("flux insert");

    assert!(t.factories.contains_key(&fid));
    assert!(t.logistics_fluxes.contains_key(&id));

    // Overview sanity
    let sums = t.calculate_overview();
    assert!(sums.iter().any(|s| s.item_name == "Iron Ingot"));
}
