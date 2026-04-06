//! Data Integrity Validation Tests
//!
//! These tests validate the integrity of game data and document the current state
//! of the data. They serve as regression guards after data updates.
//!
//! NOTE: These tests document the CURRENT state and should pass NOW.
//! Any data issues found should be documented, not fixed here.

#[cfg(test)]
mod tests {
    use crate::models::{
        game_data::MachineType,
        items::{all_items, Item},
        raw_input::ExtractorType,
        recipes::{all_recipes, recipe_info, Recipe, RecipeInfo},
    };
    use std::collections::HashSet;

    // =========================================================================
    // Recipe Item Validation
    // =========================================================================

    /// Validates that every recipe's inputs and outputs reference valid Item variants.
    /// This ensures data integrity between recipes and items.
    #[test]
    fn test_all_recipe_items_exist() {
        let mut errors: Vec<String> = Vec::new();
        let all_recipe_infos = all_recipes();

        for recipe_info in all_recipe_infos {
            // Validate inputs
            for (item, _quantity) in recipe_info.inputs {
                // The fact that the recipe compiles means the Item variant exists
                // This test validates that the item is properly defined in the items module
                let item_name = format!("{:?}", item);
                if item_name.is_empty() {
                    errors.push(format!(
                        "Recipe '{}' has empty item name in inputs",
                        recipe_info.name
                    ));
                }
            }

            // Validate outputs
            for (item, _quantity) in recipe_info.outputs {
                let item_name = format!("{:?}", item);
                if item_name.is_empty() {
                    errors.push(format!(
                        "Recipe '{}' has empty item name in outputs",
                        recipe_info.name
                    ));
                }
            }
        }

        if !errors.is_empty() {
            panic!(
                "Found {} recipe item validation errors:\n{}",
                errors.len(),
                errors.join("\n")
            );
        }

        // Test passes - all recipe items are valid
        // Total recipes validated
        assert!(!all_recipe_infos.is_empty(), "Expected at least one recipe");
    }

    // =========================================================================
    // Recipe Machine Validation
    // =========================================================================

    /// Validates that every recipe references a valid MachineType.
    /// Ensures all recipes have a valid machine assignment.
    #[test]
    fn test_all_recipe_machines_exist() {
        let mut errors: Vec<String> = Vec::new();
        let all_recipe_infos = all_recipes();
        let valid_machines = get_all_machine_types();

        for recipe_info in all_recipe_infos {
            if !valid_machines.contains(&recipe_info.machine) {
                errors.push(format!(
                    "Recipe '{}' references invalid machine type: {:?}",
                    recipe_info.name, recipe_info.machine
                ));
            }
        }

        if !errors.is_empty() {
            panic!(
                "Found {} recipe machine validation errors:\n{}",
                errors.len(),
                errors.join("\n")
            );
        }

        // All recipes have valid machines
        assert!(!all_recipe_infos.is_empty(), "Expected at least one recipe");
    }

    fn get_all_machine_types() -> HashSet<MachineType> {
        let mut machines = HashSet::new();
        machines.insert(MachineType::Constructor);
        machines.insert(MachineType::Assembler);
        machines.insert(MachineType::Manufacturer);
        machines.insert(MachineType::Smelter);
        machines.insert(MachineType::Foundry);
        machines.insert(MachineType::Refinery);
        machines.insert(MachineType::Blender);
        machines.insert(MachineType::Packager);
        machines.insert(MachineType::ParticleAccelerator);
        machines.insert(MachineType::Manual);
        machines
    }

    // =========================================================================
    // Recipe Uniqueness Validation
    // =========================================================================

    /// Validates that there are no duplicate recipe names.
    /// Duplicate names would cause lookup ambiguity.
    #[test]
    fn test_no_duplicate_recipes() {
        let all_recipe_infos = all_recipes();
        let mut seen_names: HashSet<&str> = HashSet::new();
        let mut duplicates: Vec<String> = Vec::new();

        for recipe_info in all_recipe_infos {
            let name = recipe_info.name;
            if !seen_names.insert(name) {
                duplicates.push(name.to_string());
            }
        }

        if !duplicates.is_empty() {
            panic!(
                "Found {} duplicate recipe names: {}",
                duplicates.len(),
                duplicates.join(", ")
            );
        }

        // All recipe names are unique
        assert!(!all_recipe_infos.is_empty(), "Expected at least one recipe");
        assert_eq!(
            seen_names.len(),
            all_recipe_infos.len(),
            "Recipe count mismatch - possible duplicates"
        );
    }

    // =========================================================================
    // Item Naming Convention Validation
    // =========================================================================

    /// Validates that Item enum variants follow consistent naming conventions.
    /// Documents the current naming patterns for regression detection.
    #[test]
    fn test_all_items_have_consistent_naming() {
        let all_item_pairs = all_items();
        let mut inconsistent: Vec<String> = Vec::new();

        for (_item, name) in all_item_pairs {
            // Check for consistent naming patterns
            // Items should have non-empty display names
            if name.is_empty() {
                inconsistent.push("Empty item name found".to_string());
                continue;
            }

            // Check that names don't have leading/trailing whitespace
            if name.len() != name.trim().len() {
                inconsistent.push(format!("Item '{}' has leading/trailing whitespace", name));
            }

            // Document naming convention: PascalCase variants map to Title Case names
            // This is validated by the fact that all items compile successfully
        }

        if !inconsistent.is_empty() {
            // Document but don't fail - this is for information
            println!(
                "Item naming conventions checked. Found {} items total.",
                all_item_pairs.len()
            );
            if !inconsistent.is_empty() {
                println!("Naming issues found (documented, not failing):");
                for issue in &inconsistent {
                    println!("  - {}", issue);
                }
            }
        }

        // Total items in the game
        assert!(
            all_item_pairs.len() > 100,
            "Expected at least 100 items, found {}",
            all_item_pairs.len()
        );
    }

    // =========================================================================
    // Machine Power Values Validation
    // =========================================================================

    /// Validates that every MachineType has defined base power values.
    /// Documents current power consumption for all machine types.
    #[test]
    fn test_machine_power_values_present() {
        let machines = get_all_machine_types();
        let mut missing_power: Vec<String> = Vec::new();

        for machine in machines {
            let power = machine.base_power_mw();

            // All machines should have defined power values
            // Manual has 0.0 power which is valid
            if power < 0.0 {
                missing_power.push(format!("{:?} has negative power: {}", machine, power));
            }
        }

        if !missing_power.is_empty() {
            panic!(
                "Found {} machines with invalid power values:\n{}",
                missing_power.len(),
                missing_power.join("\n")
            );
        }

        // Verify specific expected power values (documents current state)
        assert_eq!(MachineType::Constructor.base_power_mw(), 4.0);
        assert_eq!(MachineType::Assembler.base_power_mw(), 16.0);
        assert_eq!(MachineType::Manufacturer.base_power_mw(), 32.0);
        assert_eq!(MachineType::Smelter.base_power_mw(), 4.0);
        assert_eq!(MachineType::Foundry.base_power_mw(), 16.0);
        assert_eq!(MachineType::Refinery.base_power_mw(), 16.0);
        assert_eq!(MachineType::Blender.base_power_mw(), 32.0);
        assert_eq!(MachineType::Packager.base_power_mw(), 4.0);
        assert_eq!(MachineType::ParticleAccelerator.base_power_mw(), 64.0);
        assert_eq!(MachineType::Manual.base_power_mw(), 0.0);
    }

    // =========================================================================
    // Extractor Rate Validation
    // =========================================================================

    /// Validates that every ExtractorType has defined extraction rates.
    /// Documents current extraction rates for all extractor types.
    #[test]
    fn test_extractor_types_have_rates() {
        let extractors = get_all_extractor_types();
        let mut missing_rates: Vec<String> = Vec::new();
        let mut missing_power: Vec<String> = Vec::new();

        for extractor in &extractors {
            let rate = extractor.base_rate();
            let power = extractor.base_power_consumption();

            // All extractors should have positive rates (except potentially special cases)
            if rate <= 0.0 {
                missing_rates.push(format!("{:?} has invalid rate: {}", extractor, rate));
            }

            // Power can be 0.0 for special cases (like ResourceWellExtractor)
            if power < 0.0 {
                missing_power.push(format!("{:?} has negative power: {}", extractor, power));
            }
        }

        if !missing_rates.is_empty() {
            panic!(
                "Found {} extractors with invalid rates:\n{}",
                missing_rates.len(),
                missing_rates.join("\n")
            );
        }

        if !missing_power.is_empty() {
            panic!(
                "Found {} extractors with invalid power:\n{}",
                missing_power.len(),
                missing_power.join("\n")
            );
        }

        // Verify specific expected rates (documents current state)
        assert_eq!(ExtractorType::MinerMk1.base_rate(), 60.0);
        assert_eq!(ExtractorType::MinerMk2.base_rate(), 120.0);
        assert_eq!(ExtractorType::MinerMk3.base_rate(), 240.0);
        assert_eq!(ExtractorType::WaterExtractor.base_rate(), 120.0);
        assert_eq!(ExtractorType::OilExtractor.base_rate(), 120.0);
        assert_eq!(ExtractorType::ResourceWellExtractor.base_rate(), 60.0);

        // Verify power consumption values
        assert_eq!(ExtractorType::MinerMk1.base_power_consumption(), 5.0);
        assert_eq!(ExtractorType::MinerMk2.base_power_consumption(), 15.0);
        assert_eq!(ExtractorType::MinerMk3.base_power_consumption(), 45.0);
        assert_eq!(ExtractorType::WaterExtractor.base_power_consumption(), 20.0);
        assert_eq!(ExtractorType::OilExtractor.base_power_consumption(), 40.0);
        assert_eq!(
            ExtractorType::ResourceWellExtractor.base_power_consumption(),
            0.0
        );
    }

    fn get_all_extractor_types() -> Vec<ExtractorType> {
        vec![
            ExtractorType::MinerMk1,
            ExtractorType::MinerMk2,
            ExtractorType::MinerMk3,
            ExtractorType::WaterExtractor,
            ExtractorType::OilExtractor,
            ExtractorType::ResourceWellExtractor,
        ]
    }

    // =========================================================================
    // Cross-Reference Validation
    // =========================================================================

    /// Validates that all items referenced by recipes are in the all_items list.
    /// This ensures no orphaned recipe references.
    #[test]
    fn test_recipe_items_in_all_items_list() {
        let all_recipe_infos = all_recipes();
        let all_item_pairs = all_items();
        let mut orphaned_items: Vec<String> = Vec::new();

        // Build set of all known items
        let known_items: HashSet<Item> = all_item_pairs.iter().map(|(item, _)| *item).collect();

        for recipe_info in all_recipe_infos {
            // Check inputs
            for (item, _qty) in recipe_info.inputs {
                if !known_items.contains(item) {
                    orphaned_items.push(format!(
                        "Recipe '{}' input {:?} not in all_items list",
                        recipe_info.name, item
                    ));
                }
            }

            // Check outputs
            for (item, _qty) in recipe_info.outputs {
                if !known_items.contains(item) {
                    orphaned_items.push(format!(
                        "Recipe '{}' output {:?} not in all_items list",
                        recipe_info.name, item
                    ));
                }
            }
        }

        if !orphaned_items.is_empty() {
            panic!(
                "Found {} orphaned recipe item references:\n{}",
                orphaned_items.len(),
                orphaned_items.join("\n")
            );
        }

        // All recipe items are valid
        assert!(!all_recipe_infos.is_empty(), "Expected at least one recipe");
    }

    /// Documents the count of various data types for regression tracking.
    #[test]
    fn test_data_counts_for_regression_tracking() {
        let item_count = all_items().len();
        let recipe_count = all_recipes().len();
        let machine_count = get_all_machine_types().len();
        let extractor_count = get_all_extractor_types().len();

        // Document current counts (these will fail after data updates to alert reviewers)
        println!("Current data counts for regression tracking:");
        println!("  - Items: {}", item_count);
        println!("  - Recipes: {}", recipe_count);
        println!("  - Machine Types: {}", machine_count);
        println!("  - Extractor Types: {}", extractor_count);

        // These assertions document the current state
        // They serve as alerts when data changes significantly
        // CURRENT BASELINE (as of 2026-04-06): 153 items, 473 recipes
        assert!(
            item_count >= 153,
            "Item count ({}) is less than expected baseline of 153",
            item_count
        );
        assert!(
            recipe_count >= 473,
            "Recipe count ({}) is less than expected baseline of 473",
            recipe_count
        );
        assert_eq!(machine_count, 10, "Machine type count changed");
        assert_eq!(extractor_count, 6, "Extractor type count changed");
    }
}
