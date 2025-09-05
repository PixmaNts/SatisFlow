use crate::models::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductionTracker {
    pub factories: HashMap<FactoryId, Factory>,
    pub logistics_fluxes: HashMap<LogisticsFluxId, LogisticsFlux>,
    pub recipes: HashMap<String, Recipe>,
    pub items: HashMap<String, Item>,
    // Typed indices for internal logic
    #[serde(skip)]
    pub index: Option<GameIndex>,
}

impl ProductionTracker {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
            logistics_fluxes: HashMap::new(),
            recipes: HashMap::new(),
            items: HashMap::new(),
            index: None,
        }
    }

    // Load from build-time generated static data (no serde at runtime)
    pub fn load_static_data(&mut self) {
        // Convert generated items/recipes to our serde-compatible structs, then build typed index
        // Items: gather unique names from generated item enum
        // We rely on recipe infos to infer the set; item_name() gives canonical name
        use crate::static_data as game_data;
        // Build items map from all names seen in recipe IO and from item variants
        let mut items: std::collections::BTreeSet<&'static str> = std::collections::BTreeSet::new();
        // include every item variant by name
        for &i in game_data::RECIPE_INFOS
            .iter()
            .flat_map(|ri| ri.inputs.iter().map(|(it, _)| it))
            .chain(
                game_data::RECIPE_INFOS
                    .iter()
                    .flat_map(|ri| ri.outputs.iter().map(|(it, _)| it)),
            )
        {
            items.insert(game_data::item_name(i));
        }
        // fill items map
        self.items.clear();
        for name in items {
            self.items.insert(
                name.to_string(),
                Item {
                    name: name.to_string(),
                    category: "Item".to_string(),
                    description: String::new(),
                },
            );
        }

        // Recipes
        self.recipes.clear();
        for ri in game_data::RECIPE_INFOS {
            let name = ri.name.to_string();
            let machine = match ri.machine {
                crate::models::types::MachineType::Constructor => "Constructor",
                crate::models::types::MachineType::Assembler => "Assembler",
                crate::models::types::MachineType::Manufacturer => "Manufacturer",
                crate::models::types::MachineType::Smelter => "Smelter",
                crate::models::types::MachineType::Foundry => "Foundry",
                crate::models::types::MachineType::Refinery => "Refinery",
                crate::models::types::MachineType::Blender => "Blender",
                crate::models::types::MachineType::Packager => "Packager",
                crate::models::types::MachineType::ParticleAccelerator => "Particle Accelerator",
            }
            .to_string();
            let inputs = ri
                .inputs
                .iter()
                .map(|(it, q)| RecipeIngredient {
                    item: game_data::item_name(*it).to_string(),
                    quantity_per_min: *q as f32,
                })
                .collect();
            let outputs = ri
                .outputs
                .iter()
                .map(|(it, q)| RecipeIngredient {
                    item: game_data::item_name(*it).to_string(),
                    quantity_per_min: *q as f32,
                })
                .collect();
            self.recipes.insert(
                name.clone(),
                Recipe {
                    name,
                    machine_type: machine,
                    base_duration: 0.0,
                    inputs,
                    outputs,
                },
            );
        }
        // Build typed index
        let _ = self.rebuild_index();
        let _ = self.validate_game_data();
    }

    fn build_index(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create stable IDs (slugs) from names and build typed domain models
        let mut idx = GameIndex::default();

        // Items
        for (name, it) in &self.items {
            let mut slug = slugify(name);
            // ensure uniqueness by suffixing if needed
            let original = slug.clone();
            let mut n = 2;
            while idx.item_id_by_name.contains_key(name)
                || idx.items_by_id.contains_key(&ItemId(slug.clone()))
            {
                slug = format!("{}-{}", original, n);
                n += 1;
            }
            let id = ItemId(slug);
            let category = ItemCategory::from_str(&it.category).unwrap_or(ItemCategory::Item);
            let d = DomainItem {
                id: id.clone(),
                name: name.clone(),
                category,
                description: it.description.clone(),
                availability: Availability::Always,
            };
            idx.item_id_by_name.insert(name.clone(), id.clone());
            idx.item_name_by_id.insert(id.clone(), name.clone());
            idx.items_by_id.insert(id, d);
        }

        // Recipes
        for (rname, r) in &self.recipes {
            let mtype = MachineType::from_str(&r.machine_type).ok_or_else(|| {
                format!(
                    "Unknown machine type '{}' in recipe '{}'",
                    r.machine_type, rname
                )
            })?;
            let mut slug = slugify(rname);
            let original = slug.clone();
            let mut n = 2;
            while idx.recipe_id_by_name.contains_key(rname)
                || idx.recipes_by_id.contains_key(&RecipeId(slug.clone()))
            {
                slug = format!("{}-{}", original, n);
                n += 1;
            }
            let rid = RecipeId(slug);
            let mut inputs = Vec::new();
            for ing in &r.inputs {
                let iid = idx
                    .item_id_by_name
                    .get(&ing.item)
                    .ok_or_else(|| {
                        format!("Recipe '{}' references unknown input '{}'", rname, ing.item)
                    })?
                    .clone();
                inputs.push(DomainRecipeIngredient {
                    item_id: iid,
                    quantity_per_min: ing.quantity_per_min,
                });
            }
            let mut outputs = Vec::new();
            for out in &r.outputs {
                let oid = idx
                    .item_id_by_name
                    .get(&out.item)
                    .ok_or_else(|| {
                        format!(
                            "Recipe '{}' references unknown output '{}'",
                            rname, out.item
                        )
                    })?
                    .clone();
                outputs.push(DomainRecipeIngredient {
                    item_id: oid,
                    quantity_per_min: out.quantity_per_min,
                });
            }
            let d = DomainRecipe {
                id: rid.clone(),
                name: rname.clone(),
                machine_type: mtype,
                base_duration: r.base_duration,
                inputs,
                outputs,
            };
            idx.recipe_id_by_name.insert(rname.clone(), rid.clone());
            idx.recipe_name_by_id.insert(rid.clone(), rname.clone());
            idx.recipes_by_id.insert(rid, d);
        }

        self.index = Some(idx);
        Ok(())
    }

    // Public helper for tests and non-file loaders
    pub fn rebuild_index(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.build_index()
    }

    // Validate that recipes reference known items, names are non-empty, etc.
    pub fn validate_game_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check items
        for (name, item) in &self.items {
            if name.trim().is_empty() {
                return Err(format!("Item with empty name: {:?}", item).into());
            }
        }
        // Check recipes exist and reference valid items
        for (rname, recipe) in &self.recipes {
            if rname.trim().is_empty() {
                return Err(format!("Recipe with empty name").into());
            }
            // Machine type must be recognized (indexing enforces it already)
            if MachineType::from_str(&recipe.machine_type).is_none() {
                return Err(format!(
                    "Unknown machine type: {} in recipe {}",
                    recipe.machine_type, rname
                )
                .into());
            }
            for ing in &recipe.inputs {
                if !self.items.contains_key(&ing.item) {
                    return Err(format!(
                        "Recipe '{}' references unknown input item '{}'",
                        rname, ing.item
                    )
                    .into());
                }
                if ing.quantity_per_min < 0.0 {
                    return Err(format!(
                        "Negative input amount in '{}': {}",
                        rname, ing.quantity_per_min
                    )
                    .into());
                }
            }
            for out in &recipe.outputs {
                if !self.items.contains_key(&out.item) {
                    return Err(format!(
                        "Recipe '{}' references unknown output item '{}'",
                        rname, out.item
                    )
                    .into());
                }
                if out.quantity_per_min < 0.0 {
                    return Err(format!(
                        "Negative output amount in '{}': {}",
                        rname, out.quantity_per_min
                    )
                    .into());
                }
            }
        }
        Ok(())
    }

    // Calculate per-factory item balances, applying logistics transfers
    // Returns: factory_id -> (item_name -> (prod, cons, net))
    pub fn calculate_factory_balances(
        &self,
    ) -> HashMap<FactoryId, HashMap<ItemId, (f32, f32, f32)>> {
        let mut balances: HashMap<FactoryId, HashMap<ItemId, (f32, f32, f32)>> = HashMap::new();
        // Prefer typed index for correctness
        let idx = match &self.index {
            Some(i) => i,
            None => return balances, // no data yet
        };

        // Initialize from factories: raw inputs and production lines
        for (fid, factory) in &self.factories {
            let entry = balances.entry(fid.clone()).or_default();
            // Raw inputs count as production (resolve item name to id->name stable)
            for ri in &factory.raw_inputs {
                let e = entry.entry(ri.item.clone()).or_insert((0.0, 0.0, 0.0));
                e.0 += ri.quantity_per_min;
            }
            // Production lines
            for pl in &factory.production_lines {
                if let Some(recipe) = idx.recipes_by_id.get(&pl.recipe_id) {
                    let m = pl.machine_count.max(1);
                    let boosted = pl.strange_matter_boosted.min(m);
                    let base_machines = (m - boosted) as f32;
                    let boosted_machines = boosted as f32;
                    // Inputs scale with all machines * clock
                    let input_mult = (m as f32) * pl.clock_ratio;
                    // Outputs: boosted machines double their output
                    let output_mult = (base_machines + boosted_machines * 2.0) * pl.clock_ratio;
                    for o in &recipe.outputs {
                        let e = entry.entry(o.item_id.clone()).or_insert((0.0, 0.0, 0.0));
                        e.0 += o.quantity_per_min * output_mult;
                        // Stored routing: treat stored outputs as consumed (not available)
                        if let Some(route) = pl.output_routing.get(&o.item_id) {
                            if matches!(route, ProductionOutput::Stored) {
                                e.1 += o.quantity_per_min * output_mult;
                            }
                        }
                    }
                    for i in &recipe.inputs {
                        let e = entry.entry(i.item_id.clone()).or_insert((0.0, 0.0, 0.0));
                        e.1 += i.quantity_per_min * input_mult;
                    }
                }
            }
        }

        // Apply logistics transfers
        for flux in self.logistics_fluxes.values() {
            // Source factory consumption
            if let Some(src) = balances.get_mut(&flux.from_factory) {
                let e = src.entry(flux.item.clone()).or_insert((0.0, 0.0, 0.0));
                e.1 += flux.quantity_per_min;
            }
            // Destination factory production
            if let Some(dst) = balances.get_mut(&flux.to_factory) {
                let e = dst.entry(flux.item.clone()).or_insert((0.0, 0.0, 0.0));
                e.0 += flux.quantity_per_min;
            }
        }

        // Compute net for each entry
        for factory_items in balances.values_mut() {
            for (_iname, (p, c, n)) in factory_items.iter_mut() {
                *n = *p - *c;
            }
        }

        balances
    }

    pub fn calculate_overview(&self) -> Vec<ItemSummary> {
        let balances = self.calculate_factory_balances();
        let mut production: HashMap<ItemId, f32> = HashMap::new();
        let mut consumption: HashMap<ItemId, f32> = HashMap::new();
        for items in balances.values() {
            for (iid, (p, c, _)) in items {
                *production.entry(iid.clone()).or_insert(0.0) += *p;
                *consumption.entry(iid.clone()).or_insert(0.0) += *c;
            }
        }
        let mut all: std::collections::HashSet<ItemId> = std::collections::HashSet::new();
        all.extend(production.keys().cloned());
        all.extend(consumption.keys().cloned());
        let idx = match &self.index {
            Some(i) => i,
            None => return Vec::new(),
        };
        let mut out = Vec::new();
        for iid in all {
            let p = production.get(&iid).copied().unwrap_or(0.0);
            let c = consumption.get(&iid).copied().unwrap_or(0.0);
            let avail = p - c;
            let status = if avail.abs() < 0.1 {
                ProductionStatus::Balanced
            } else if avail > 0.0 {
                ProductionStatus::Overflow
            } else {
                ProductionStatus::Underflow
            };
            let name = idx
                .item_name_by_id
                .get(&iid)
                .cloned()
                .unwrap_or_else(|| iid.0.clone());
            out.push(ItemSummary {
                item_id: iid,
                item_name: name,
                total_produced_per_min: p,
                total_consumed_per_min: c,
                available_per_min: avail,
                status,
            });
        }
        out.sort_by(|a, b| a.item_name.cmp(&b.item_name));
        out
    }

    pub fn add_factory(&mut self, factory: Factory) {
        self.factories.insert(factory.id.clone(), factory);
    }

    pub fn factory_name_exists(&self, name: &str) -> bool {
        self.factories
            .values()
            .any(|f| f.name.eq_ignore_ascii_case(name))
    }

    pub fn generate_factory_id(&self, name: &str) -> FactoryId {
        let mut slug = name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect::<String>();
        // collapse multiple underscores
        while slug.contains("__") {
            slug = slug.replace("__", "_");
        }
        slug = slug.trim_matches('_').to_string();
        if slug.is_empty() {
            slug = "factory".to_string();
        }

        let mut n: u32 = 1;
        loop {
            let id = FactoryId(format!("fac_{}_{}", slug, n));
            if !self.factories.contains_key(&id) {
                break id;
            }
            n += 1;
        }
    }

    pub fn generate_line_id(&self, factory_id: &FactoryId, recipe_name: &str) -> ProductionLineId {
        let mut slug = recipe_name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect::<String>();
        while slug.contains("__") {
            slug = slug.replace("__", "_");
        }
        slug = slug.trim_matches('_').to_string();
        if slug.is_empty() {
            slug = "line".to_string();
        }
        let mut n: u32 = 1;
        loop {
            let id = ProductionLineId(format!("line_{}_{}_{}", factory_id.0, slug, n));
            let exists = self
                .factories
                .values()
                .any(|f| f.production_lines.iter().any(|pl| pl.id == id));
            if !exists {
                break id;
            }
            n += 1;
        }
    }

    pub fn add_production_line(&mut self, line: ProductionLine) -> Result<(), String> {
        if self
            .index
            .as_ref()
            .map_or(true, |idx| !idx.recipes_by_id.contains_key(&line.recipe_id))
        {
            return Err(format!("Unknown recipe id: {}", line.recipe_id.0));
        }
        if line.machine_count == 0 || line.machine_count > 999 {
            return Err(format!(
                "Invalid machine_count: {} (allowed 1..=999)",
                line.machine_count
            ));
        }
        if !(0.1..=2.5).contains(&line.clock_ratio) {
            return Err(format!(
                "Invalid clock_ratio: {} (allowed 0.1..=2.5)",
                line.clock_ratio
            ));
        }
        match self.factories.get_mut(&line.factory_id) {
            Some(f) => {
                f.production_lines.push(line);
                Ok(())
            }
            None => Err(format!("Factory not found: {}", line.factory_id.0)),
        }
    }

    pub fn add_logistics_flux(&mut self, flux: LogisticsFlux) -> Result<(), String> {
        if !self.factories.contains_key(&flux.from_factory) {
            return Err(format!("Unknown from_factory: {}", flux.from_factory.0));
        }
        if !self.factories.contains_key(&flux.to_factory) {
            return Err(format!("Unknown to_factory: {}", flux.to_factory.0));
        }
        if flux.from_factory == flux.to_factory {
            return Err("from_factory and to_factory must differ".to_string());
        }
        if flux.quantity_per_min <= 0.0 {
            return Err(format!(
                "quantity_per_min must be > 0, got {}",
                flux.quantity_per_min
            ));
        }
        if let Some(idx) = &self.index {
            if !idx.items_by_id.contains_key(&flux.item) {
                return Err(format!("Unknown item id in flux: {}", flux.item.0));
            }
        }
        self.logistics_fluxes.insert(flux.id.clone(), flux);
        Ok(())
    }

    pub fn generate_logistics_id(&self, t: &TransportType) -> LogisticsFluxId {
        let prefix = match t {
            TransportType::Conveyor => "BUS",
            TransportType::Train => "TRN",
            TransportType::Truck => "TRK",
            TransportType::Drone => "DRN",
        };
        let mut n: u32 = 1;
        loop {
            let base = format!("LG-{}-{:03}", prefix, n);
            // Consider both exact base and any suffixed variants (e.g., -C01) as used
            let used = self
                .logistics_fluxes
                .keys()
                .any(|lid| lid.0 == base || lid.0.starts_with(&(base.clone() + "-")));
            if !used {
                break LogisticsFluxId(base);
            }
            n += 1;
        }
    }

    pub fn generate_logistics_id_with_detail(
        &self,
        t: &TransportType,
        detail: Option<&str>,
    ) -> LogisticsFluxId {
        let base = self.generate_logistics_id(t);
        if let Some(d) = detail {
            if d.is_empty() {
                return base;
            }
            LogisticsFluxId(format!("{}-{}", base.0, d))
        } else {
            base
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tracker_with_data() -> ProductionTracker {
        let mut t = ProductionTracker::new();
        // Minimal items
        for n in ["Iron Ore", "Iron Ingot", "Iron Plate"] {
            t.items.insert(
                n.to_string(),
                Item {
                    name: n.to_string(),
                    category: "Test".to_string(),
                    description: String::new(),
                },
            );
        }
        // Minimal recipes
        t.recipes.insert(
            "Iron Ingot".to_string(),
            Recipe {
                name: "Iron Ingot".to_string(),
                machine_type: "Smelter".to_string(),
                base_duration: 2.0,
                inputs: vec![RecipeIngredient {
                    item: "Iron Ore".to_string(),
                    quantity_per_min: 30.0,
                }],
                outputs: vec![RecipeIngredient {
                    item: "Iron Ingot".to_string(),
                    quantity_per_min: 30.0,
                }],
            },
        );
        t.recipes.insert(
            "Iron Plate".to_string(),
            Recipe {
                name: "Iron Plate".to_string(),
                machine_type: "Constructor".to_string(),
                base_duration: 6.0,
                inputs: vec![RecipeIngredient {
                    item: "Iron Ingot".to_string(),
                    quantity_per_min: 30.0,
                }],
                outputs: vec![RecipeIngredient {
                    item: "Iron Plate".to_string(),
                    quantity_per_min: 20.0,
                }],
            },
        );
        // Build typed index for engine operations
        t.rebuild_index().unwrap();
        t
    }

    #[test]
    fn factory_id_and_name_uniqueness() {
        let mut t = tracker_with_data();
        let name = "Iron Processing Plant";
        let id1 = t.generate_factory_id(name);
        t.add_factory(Factory {
            id: id1.clone(),
            name: name.to_string(),
            raw_inputs: vec![],
            logistics_inputs: vec![],
            production_lines: vec![],
        });
        assert!(t.factory_name_exists(name));
        let id2 = t.generate_factory_id(name);
        assert_ne!(id1, id2);
        assert!(id1.0.starts_with("fac_iron_processing_plant_"));
        assert!(id2.0.starts_with("fac_iron_processing_plant_"));
    }

    #[test]
    fn overview_calculations_basic() {
        let mut t = tracker_with_data();
        // Factory with ingot production 2x -> 60 ingots/min
        let fid = t.generate_factory_id("Smelting");
        let (iron_ore, iron_ingot_recipe, iron_plate_recipe) = {
            let idx = t.index.as_ref().unwrap();
            (
                idx.item_id_by_name["Iron Ore"].clone(),
                idx.recipe_id_by_name["Iron Ingot"].clone(),
                idx.recipe_id_by_name["Iron Plate"].clone(),
            )
        };
        t.add_factory(Factory {
            id: fid.clone(),
            name: "Smelting".to_string(),
            raw_inputs: vec![RawInput {
                item: iron_ore.clone(),
                quantity_per_min: 60.0,
                source_type: "Test".to_string(),
            }],
            logistics_inputs: vec![],
            production_lines: vec![],
        });
        t.add_production_line(ProductionLine {
            id: t.generate_line_id(&fid, "Iron Ingot"),
            factory_id: fid.clone(),
            recipe_id: iron_ingot_recipe,
            machine_count: 2,
            clock_ratio: 1.0,
            group_name: None,
            output_routing: HashMap::new(),
            strange_matter_boosted: 0,
        })
        .unwrap();

        let sums = t.calculate_overview();
        let ingot = sums.iter().find(|s| s.item_name == "Iron Ingot").unwrap();
        assert!((ingot.total_produced_per_min - 60.0).abs() < 1e-3);
        assert!((ingot.total_consumed_per_min - 0.0).abs() < 1e-3);

        // Add plate consumer 1x -> consumes 30 ingots/min
        t.add_production_line(ProductionLine {
            id: t.generate_line_id(&fid, "Iron Plate"),
            factory_id: fid.clone(),
            recipe_id: iron_plate_recipe,
            machine_count: 1,
            clock_ratio: 1.0,
            group_name: None,
            output_routing: HashMap::new(),
            strange_matter_boosted: 0,
        })
        .unwrap();
        let sums2 = t.calculate_overview();
        let ingot2 = sums2.iter().find(|s| s.item_name == "Iron Ingot").unwrap();
        assert!((ingot2.total_produced_per_min - 60.0).abs() < 1e-3);
        assert!((ingot2.total_consumed_per_min - 30.0).abs() < 1e-3);
        assert!((ingot2.available_per_min - 30.0).abs() < 1e-3);
    }

    #[test]
    fn logistics_id_generation() {
        let t = tracker_with_data();
        let first = t.generate_logistics_id(&TransportType::Conveyor);
        assert_eq!(first.0, "LG-BUS-001");
    }

    #[test]
    fn logistics_transfer_affects_factory_balances() {
        let mut t = tracker_with_data();
        let fid_a = t.generate_factory_id("A");
        let fid_b = t.generate_factory_id("B");
        let idx = t.index.as_ref().unwrap();
        let iron_ingot = idx.item_id_by_name["Iron Ingot"].clone();
        t.add_factory(Factory {
            id: fid_a.clone(),
            name: "A".into(),
            raw_inputs: vec![RawInput {
                item: iron_ingot.clone(),
                quantity_per_min: 50.0,
                source_type: "Test".into(),
            }],
            logistics_inputs: vec![],
            production_lines: vec![],
        });
        t.add_factory(Factory {
            id: fid_b.clone(),
            name: "B".into(),
            raw_inputs: vec![],
            logistics_inputs: vec![],
            production_lines: vec![],
        });
        t.add_logistics_flux(LogisticsFlux {
            id: t.generate_logistics_id(&TransportType::Conveyor),
            from_factory: fid_a.clone(),
            to_factory: fid_b.clone(),
            item: iron_ingot.clone(),
            quantity_per_min: 30.0,
            transport_type: TransportType::Conveyor,
            transport_details: String::new(),
        });

        let balances = t.calculate_factory_balances();
        let a = balances.get(&fid_a).unwrap();
        let b = balances.get(&fid_b).unwrap();
        let (pa, ca, na) = a.get(&iron_ingot).copied().unwrap_or((0.0, 0.0, 0.0));
        let (pb, cb, nb) = b.get(&iron_ingot).copied().unwrap_or((0.0, 0.0, 0.0));
        assert!((pa - 50.0).abs() < 1e-3);
        assert!((ca - 30.0).abs() < 1e-3);
        assert!((na - 20.0).abs() < 1e-3);
        assert!((pb - 30.0).abs() < 1e-3);
        assert!((cb - 0.0).abs() < 1e-3);
        assert!((nb - 30.0).abs() < 1e-3);
    }

    #[test]
    fn validation_catches_unknown_item() {
        let mut t = ProductionTracker::new();
        t.items.insert(
            "Iron Ore".into(),
            Item {
                name: "Iron Ore".into(),
                category: "Test".into(),
                description: String::new(),
            },
        );
        t.recipes.insert(
            "Bad".into(),
            Recipe {
                name: "Bad".into(),
                machine_type: "Smelter".into(),
                base_duration: 2.0,
                inputs: vec![RecipeIngredient {
                    item: "Unknown".into(),
                    quantity_per_min: 1.0,
                }],
                outputs: vec![RecipeIngredient {
                    item: "Iron Ore".into(),
                    quantity_per_min: 1.0,
                }],
            },
        );
        let err = t.validate_game_data().unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("Unknown input item") || msg.contains("references unknown"));
    }
}
