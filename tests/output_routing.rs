use satisflow::*;
use std::collections::HashMap;

#[test]
fn stored_routing_reduces_availability() {
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

    let fid = t.generate_factory_id("Smelting");
    let idx = t.index.as_ref().unwrap();
    let iron_ore = idx.item_id_by_name["Iron Ore"].clone();
    let iron_ingot = idx.item_id_by_name["Iron Ingot"].clone();
    let ingot_recipe = idx.recipe_id_by_name["Iron Ingot"].clone();
    t.add_factory(Factory { id: fid.clone(), name: "Smelting".into(), raw_inputs: vec![RawInput{ item: iron_ore, quantity_per_min: 30.0, source_type: "Test".into()}], logistics_inputs: vec![], production_lines: vec![] });

    let mut routing = HashMap::new();
    routing.insert(iron_ingot.clone(), ProductionOutput::Stored);
    t.add_production_line(ProductionLine { id: t.generate_line_id(&fid, "Iron Ingot"), factory_id: fid.clone(), recipe_id: ingot_recipe, machine_count: 1, clock_ratio: 1.0, group_name: None, output_routing: routing, strange_matter_boosted: 0 }).unwrap();

    let sums = t.calculate_overview();
    let ing = sums.iter().find(|s| s.item_name == "Iron Ingot").unwrap();
    // Produced 30, consumed 30 (stored), so available 0
    assert!((ing.available_per_min - 0.0).abs() < 1e-3);
}

