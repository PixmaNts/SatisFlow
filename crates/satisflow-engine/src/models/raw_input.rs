use serde::{Deserialize, Serialize};

use crate::models::Item;

/// Purity levels for resource nodes in Satisfactory
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Purity {
    Impure, // 50% yield (0.5x multiplier)
    Normal, // 100% yield (1.0x multiplier)
    Pure,   // 200% yield (2.0x multiplier)
}

impl Purity {
    /// Get the yield multiplier for this purity level
    pub fn multiplier(&self) -> f32 {
        match self {
            Purity::Impure => 0.5,
            Purity::Normal => 1.0,
            Purity::Pure => 2.0,
        }
    }
}

/// Types of extractors available in Satisfactory
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ExtractorType {
    MinerMk1,
    MinerMk2,
    MinerMk3,
    WaterExtractor,
    OilExtractor,
    ResourceWellExtractor, // For gas/nitrogen
}

impl ExtractorType {
    /// Get the base extraction rate for this extractor type (items/min or m³/min)
    pub fn base_rate(&self) -> f32 {
        match self {
            ExtractorType::MinerMk1 => 60.0,
            ExtractorType::MinerMk2 => 120.0,
            ExtractorType::MinerMk3 => 240.0,
            ExtractorType::WaterExtractor => 120.0,
            ExtractorType::OilExtractor => 120.0, // Base for Normal purity
            ExtractorType::ResourceWellExtractor => 60.0, // Base for Normal purity
        }
    }

    /// Get the base power consumption for this extractor type in MW at 100% clock speed
    pub fn base_power_consumption(&self) -> f32 {
        match self {
            ExtractorType::MinerMk1 => 5.0,
            ExtractorType::MinerMk2 => 15.0,
            ExtractorType::MinerMk3 => 45.0,
            ExtractorType::WaterExtractor => 20.0,
            ExtractorType::OilExtractor => 40.0,
            ExtractorType::ResourceWellExtractor => 0.0, // No power consumption - powered by pressurizer
        }
    }

    /// Check if this extractor supports purity modifiers
    pub fn supports_purity(&self) -> bool {
        !matches!(self, ExtractorType::WaterExtractor)
    }

    /// Check if this extractor is compatible with the given item type
    pub fn is_compatible_with(&self, item: &Item) -> bool {
        match self {
            ExtractorType::MinerMk1 | ExtractorType::MinerMk2 | ExtractorType::MinerMk3 => {
                matches!(
                    item,
                    Item::IronOre
                        | Item::CopperOre
                        | Item::Limestone
                        | Item::Coal
                        | Item::CateriumOre
                        | Item::RawQuartz
                        | Item::Sulfur
                        | Item::Bauxite
                        | Item::Uranium
                        | Item::Sam
                )
            }
            ExtractorType::WaterExtractor => matches!(item, Item::Water),
            ExtractorType::OilExtractor => matches!(item, Item::CrudeOil),
            ExtractorType::ResourceWellExtractor => {
                matches!(item, Item::NitrogenGas | Item::CrudeOil | Item::Water)
            }
        }
    }
}

/// Resource Well Pressurizer - Main building that powers Resource Well Extractors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceWellPressurizer {
    pub id: u64,
    pub clock_speed: f32, // 0.000 to 250.000
}

impl ResourceWellPressurizer {
    /// Create a new Resource Well Pressurizer
    pub fn new(id: u64, clock_speed: f32) -> Result<Self, RawInputError> {
        if !(0.0..=250.0).contains(&clock_speed) {
            return Err(RawInputError::InvalidClockSpeed { clock_speed });
        }

        Ok(Self { id, clock_speed })
    }

    /// Calculate power consumption using the same formula as ProductionLine
    /// Power usage = Base power usage × (Clock speed/100)^1.321928
    pub fn power_consumption(&self) -> f32 {
        const BASE_POWER: f32 = 150.0; // 150MW base power consumption
        BASE_POWER * (self.clock_speed / 100.0).powf(1.321928)
    }

    /// Set the clock speed of the pressurizer
    pub fn set_clock_speed(&mut self, clock_speed: f32) -> Result<(), RawInputError> {
        if !(0.0..=250.0).contains(&clock_speed) {
            return Err(RawInputError::InvalidClockSpeed { clock_speed });
        }
        self.clock_speed = clock_speed;
        Ok(())
    }
}

/// Resource Well Extractor - Satellite node that extracts resources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceWellExtractor {
    pub id: u64,
    pub purity: Purity,
}

impl ResourceWellExtractor {
    /// Create a new Resource Well Extractor
    pub fn new(id: u64, purity: Purity) -> Self {
        Self { id, purity }
    }

    /// Calculate extraction rate based on purity and pressurizer clock speed
    /// Base rates: Impure 30, Normal 60, Pure 120 m³/min at 100% clock
    pub fn extraction_rate(&self, pressurizer_clock_speed: f32) -> f32 {
        let base_rate = match self.purity {
            Purity::Impure => 30.0,
            Purity::Normal => 60.0,
            Purity::Pure => 120.0,
        };
        base_rate * (pressurizer_clock_speed / 100.0)
    }
}

/// Represents a raw resource extraction source in a factory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawInput {
    pub id: u64,
    pub extractor_type: ExtractorType,
    pub item: Item,
    pub purity: Option<Purity>, // Some for ores/oil/gas, None for water
    pub quantity_per_min: f32,
    // Resource Well system fields
    pub pressurizer: Option<ResourceWellPressurizer>,
    pub extractors: Vec<ResourceWellExtractor>,
}

impl RawInput {
    /// Create a new RawInput with automatic quantity calculation
    pub fn new(
        id: u64,
        extractor_type: ExtractorType,
        item: Item,
        purity: Option<Purity>,
    ) -> Result<Self, RawInputError> {
        // Validate resource compatibility
        if !extractor_type.is_compatible_with(&item) {
            return Err(RawInputError::IncompatibleExtractor {
                extractor: extractor_type,
                item,
            });
        }

        // Validate purity based on extractor type
        if extractor_type.supports_purity() && purity.is_none() {
            return Err(RawInputError::MissingPurity {
                extractor: extractor_type,
            });
        }

        if !extractor_type.supports_purity() && purity.is_some() {
            return Err(RawInputError::UnexpectedPurity {
                extractor: extractor_type,
            });
        }

        let quantity_per_min = Self::calculate_extraction_rate(extractor_type, purity);

        Ok(Self {
            id,
            extractor_type,
            item,
            purity,
            quantity_per_min,
            pressurizer: None,
            extractors: Vec::new(),
        })
    }

    /// Create a new Resource Well system with pressurizer and extractors
    pub fn new_resource_well(
        id: u64,
        item: Item,
        pressurizer: ResourceWellPressurizer,
        extractors: Vec<ResourceWellExtractor>,
    ) -> Result<Self, RawInputError> {
        // Validate that the item is compatible with Resource Well Extractors
        if !ExtractorType::ResourceWellExtractor.is_compatible_with(&item) {
            return Err(RawInputError::IncompatibleExtractor {
                extractor: ExtractorType::ResourceWellExtractor,
                item,
            });
        }

        // Validate that extractors are not empty
        if extractors.is_empty() {
            return Err(RawInputError::NoExtractors);
        }

        // Calculate total extraction rate from all extractors
        let quantity_per_min = extractors
            .iter()
            .map(|e| e.extraction_rate(pressurizer.clock_speed))
            .sum();

        Ok(Self {
            id,
            extractor_type: ExtractorType::ResourceWellExtractor,
            item,
            purity: None, // Resource Well system doesn't use overall purity
            quantity_per_min,
            pressurizer: Some(pressurizer),
            extractors,
        })
    }

    /// Calculate extraction rate based on extractor type and purity
    pub fn calculate_extraction_rate(extractor_type: ExtractorType, purity: Option<Purity>) -> f32 {
        let base_rate = extractor_type.base_rate();

        if let Some(purity) = purity {
            base_rate * purity.multiplier()
        } else {
            // Water extractor has no purity
            base_rate
        }
    }

    /// Update extraction rates for Resource Well systems when clock speed changes
    pub fn update_extraction_rates(&mut self) {
        if let Some(pressurizer) = &self.pressurizer {
            self.quantity_per_min = self
                .extractors
                .iter()
                .map(|e| e.extraction_rate(pressurizer.clock_speed))
                .sum();
        }
    }

    /// Get the power consumption of this raw input
    /// Resource Well systems consume power only from the pressurizer
    /// Regular extractors consume their base power consumption
    pub fn power_consumption(&self) -> f32 {
        if let Some(pressurizer) = &self.pressurizer {
            pressurizer.power_consumption()
        } else {
            // Regular extractors consume their base power consumption
            self.extractor_type.base_power_consumption()
        }
    }

    /// Add an extractor to a Resource Well system
    pub fn add_extractor(&mut self, extractor: ResourceWellExtractor) -> Result<(), RawInputError> {
        if self.pressurizer.is_none() {
            return Err(RawInputError::ExtractorsWithoutPressurizer);
        }

        self.extractors.push(extractor);
        self.update_extraction_rates();
        Ok(())
    }

    /// Remove an extractor from a Resource Well system
    pub fn remove_extractor(&mut self, extractor_id: u64) -> Result<(), RawInputError> {
        if self.pressurizer.is_none() {
            return Err(RawInputError::ExtractorsWithoutPressurizer);
        }

        let initial_len = self.extractors.len();
        self.extractors.retain(|e| e.id != extractor_id);

        if self.extractors.len() == initial_len {
            return Err(RawInputError::ExtractorNotFound { id: extractor_id });
        }

        if self.extractors.is_empty() {
            return Err(RawInputError::NoExtractors);
        }

        self.update_extraction_rates();
        Ok(())
    }

    /// Set or update the pressurizer for this Resource Well system
    pub fn set_pressurizer(
        &mut self,
        pressurizer: ResourceWellPressurizer,
    ) -> Result<(), RawInputError> {
        if self.extractor_type != ExtractorType::ResourceWellExtractor {
            return Err(RawInputError::PressurizerOnNonResourceWell);
        }

        self.pressurizer = Some(pressurizer);
        self.update_extraction_rates();
        Ok(())
    }

    /// Validate that this raw input configuration is correct
    pub fn validate(&self) -> Result<(), RawInputError> {
        // Check resource compatibility
        if !self.extractor_type.is_compatible_with(&self.item) {
            return Err(RawInputError::IncompatibleExtractor {
                extractor: self.extractor_type,
                item: self.item,
            });
        }

        // Check purity requirements (except for Resource Well systems which use individual extractor purities)
        if self.extractor_type.supports_purity()
            && self.purity.is_none()
            && self.extractor_type != ExtractorType::ResourceWellExtractor
        {
            return Err(RawInputError::MissingPurity {
                extractor: self.extractor_type,
            });
        }

        if !self.extractor_type.supports_purity() && self.purity.is_some() {
            return Err(RawInputError::UnexpectedPurity {
                extractor: self.extractor_type,
            });
        }

        // Validate Resource Well system
        if self.extractor_type == ExtractorType::ResourceWellExtractor {
            if self.pressurizer.is_none() {
                return Err(RawInputError::ExtractorsWithoutPressurizer);
            }

            if self.extractors.is_empty() {
                return Err(RawInputError::NoExtractors);
            }
        }

        Ok(())
    }
}

/// Errors that can occur when working with raw inputs
#[derive(Debug, Clone, PartialEq)]
pub enum RawInputError {
    IncompatibleExtractor {
        extractor: ExtractorType,
        item: Item,
    },
    MissingPurity {
        extractor: ExtractorType,
    },
    UnexpectedPurity {
        extractor: ExtractorType,
    },
    InvalidClockSpeed {
        clock_speed: f32,
    },
    NoExtractors,
    ExtractorsWithoutPressurizer,
    PressurizerOnNonResourceWell,
    ExtractorNotFound {
        id: u64,
    },
}

impl std::fmt::Display for RawInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawInputError::IncompatibleExtractor { extractor, item } => {
                write!(
                    f,
                    "Extractor {:?} cannot extract item {:?}",
                    extractor, item
                )
            }
            RawInputError::MissingPurity { extractor } => {
                write!(f, "Extractor {:?} requires a purity level", extractor)
            }
            RawInputError::UnexpectedPurity { extractor } => {
                write!(f, "Extractor {:?} does not use purity levels", extractor)
            }
            RawInputError::InvalidClockSpeed { clock_speed } => {
                write!(
                    f,
                    "Clock speed {} is invalid. Must be between 0.000 and 250.000",
                    clock_speed
                )
            }
            RawInputError::NoExtractors => {
                write!(f, "Resource Well system must have at least one extractor")
            }
            RawInputError::ExtractorsWithoutPressurizer => {
                write!(f, "Resource Well Extractors require a pressurizer")
            }
            RawInputError::PressurizerOnNonResourceWell => {
                write!(
                    f,
                    "Pressurizer can only be used with Resource Well Extractors"
                )
            }
            RawInputError::ExtractorNotFound { id } => {
                write!(f, "Extractor with ID {} not found", id)
            }
        }
    }
}

impl std::error::Error for RawInputError {}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Purity Tests =====

    #[test]
    fn test_purity_multipliers() {
        assert_eq!(Purity::Impure.multiplier(), 0.5);
        assert_eq!(Purity::Normal.multiplier(), 1.0);
        assert_eq!(Purity::Pure.multiplier(), 2.0);
    }

    // ===== ExtractorType Tests =====

    #[test]
    fn test_extractor_base_rates() {
        assert_eq!(ExtractorType::MinerMk1.base_rate(), 60.0);
        assert_eq!(ExtractorType::MinerMk2.base_rate(), 120.0);
        assert_eq!(ExtractorType::MinerMk3.base_rate(), 240.0);
        assert_eq!(ExtractorType::WaterExtractor.base_rate(), 120.0);
        assert_eq!(ExtractorType::OilExtractor.base_rate(), 120.0);
        assert_eq!(ExtractorType::ResourceWellExtractor.base_rate(), 60.0);
    }

    #[test]
    fn test_extractor_purity_support() {
        assert!(ExtractorType::MinerMk1.supports_purity());
        assert!(ExtractorType::MinerMk2.supports_purity());
        assert!(ExtractorType::MinerMk3.supports_purity());
        assert!(!ExtractorType::WaterExtractor.supports_purity());
        assert!(ExtractorType::OilExtractor.supports_purity());
        assert!(ExtractorType::ResourceWellExtractor.supports_purity());
    }

    #[test]
    fn test_miner_compatibility() {
        // Test solid resources
        assert!(ExtractorType::MinerMk1.is_compatible_with(&Item::IronOre));
        assert!(ExtractorType::MinerMk2.is_compatible_with(&Item::CopperOre));
        assert!(ExtractorType::MinerMk3.is_compatible_with(&Item::Coal));
        assert!(ExtractorType::MinerMk1.is_compatible_with(&Item::Limestone));
        assert!(ExtractorType::MinerMk2.is_compatible_with(&Item::CateriumOre));
        assert!(ExtractorType::MinerMk3.is_compatible_with(&Item::RawQuartz));
        assert!(ExtractorType::MinerMk1.is_compatible_with(&Item::Sulfur));
        assert!(ExtractorType::MinerMk2.is_compatible_with(&Item::Bauxite));
        assert!(ExtractorType::MinerMk3.is_compatible_with(&Item::Uranium));
        assert!(ExtractorType::MinerMk3.is_compatible_with(&Item::Sam));
        // Test incompatible items
        assert!(!ExtractorType::MinerMk1.is_compatible_with(&Item::Water));
        assert!(!ExtractorType::MinerMk2.is_compatible_with(&Item::CrudeOil));
    }

    #[test]
    fn test_water_extractor_compatibility() {
        assert!(ExtractorType::WaterExtractor.is_compatible_with(&Item::Water));
        assert!(!ExtractorType::WaterExtractor.is_compatible_with(&Item::IronOre));
        assert!(!ExtractorType::WaterExtractor.is_compatible_with(&Item::CrudeOil));
    }

    #[test]
    fn test_oil_extractor_compatibility() {
        assert!(ExtractorType::OilExtractor.is_compatible_with(&Item::CrudeOil));
        assert!(!ExtractorType::OilExtractor.is_compatible_with(&Item::Water));
        assert!(!ExtractorType::OilExtractor.is_compatible_with(&Item::IronOre));
    }

    #[test]
    fn test_resource_well_compatibility() {
        assert!(ExtractorType::ResourceWellExtractor.is_compatible_with(&Item::NitrogenGas));
        assert!(ExtractorType::ResourceWellExtractor.is_compatible_with(&Item::CrudeOil));
        assert!(ExtractorType::ResourceWellExtractor.is_compatible_with(&Item::Water));
    }

    #[test]
    fn test_resource_well_with_water() {
        assert!(ExtractorType::ResourceWellExtractor.is_compatible_with(&Item::Water));
        let input = RawInput::new(
            1,
            ExtractorType::ResourceWellExtractor,
            Item::Water,
            Some(Purity::Normal),
        )
        .expect("Should create valid water input with resource well");

        assert_eq!(input.quantity_per_min, 60.0);
    }

    #[test]
    fn test_resource_well_with_oil() {
        assert!(ExtractorType::ResourceWellExtractor.is_compatible_with(&Item::CrudeOil));
        let input = RawInput::new(
            1,
            ExtractorType::ResourceWellExtractor,
            Item::CrudeOil,
            Some(Purity::Pure),
        )
        .expect("Should create valid oil input with resource well");

        assert_eq!(input.quantity_per_min, 120.0);
    }

    #[test]
    fn test_resource_well_with_nitrogen() {
        assert!(ExtractorType::ResourceWellExtractor.is_compatible_with(&Item::NitrogenGas));
        let input = RawInput::new(
            1,
            ExtractorType::ResourceWellExtractor,
            Item::NitrogenGas,
            Some(Purity::Normal),
        )
        .expect("Should create valid nitrogen input with resource well");

        assert_eq!(input.quantity_per_min, 60.0);
    }

    // ===== Extraction Rate Calculation Tests =====

    #[test]
    fn test_miner_mk1_extraction_rates() {
        assert_eq!(
            RawInput::calculate_extraction_rate(ExtractorType::MinerMk1, Some(Purity::Impure)),
            30.0
        );
        assert_eq!(
            RawInput::calculate_extraction_rate(ExtractorType::MinerMk1, Some(Purity::Normal)),
            60.0
        );
        assert_eq!(
            RawInput::calculate_extraction_rate(ExtractorType::MinerMk1, Some(Purity::Pure)),
            120.0
        );
    }

    #[test]
    fn test_miner_mk2_extraction_rates() {
        assert_eq!(
            RawInput::calculate_extraction_rate(ExtractorType::MinerMk2, Some(Purity::Impure)),
            60.0
        );
        assert_eq!(
            RawInput::calculate_extraction_rate(ExtractorType::MinerMk2, Some(Purity::Normal)),
            120.0
        );
        assert_eq!(
            RawInput::calculate_extraction_rate(ExtractorType::MinerMk2, Some(Purity::Pure)),
            240.0
        );
    }

    #[test]
    fn test_miner_mk3_extraction_rates() {
        assert_eq!(
            RawInput::calculate_extraction_rate(ExtractorType::MinerMk3, Some(Purity::Impure)),
            120.0
        );
        assert_eq!(
            RawInput::calculate_extraction_rate(ExtractorType::MinerMk3, Some(Purity::Normal)),
            240.0
        );
        assert_eq!(
            RawInput::calculate_extraction_rate(ExtractorType::MinerMk3, Some(Purity::Pure)),
            480.0
        );
    }

    #[test]
    fn test_water_extractor_rate() {
        // Water extractor has fixed rate, no purity
        assert_eq!(
            RawInput::calculate_extraction_rate(ExtractorType::WaterExtractor, None),
            120.0
        );
    }

    #[test]
    fn test_oil_extractor_rates() {
        assert_eq!(
            RawInput::calculate_extraction_rate(ExtractorType::OilExtractor, Some(Purity::Impure)),
            60.0
        );
        assert_eq!(
            RawInput::calculate_extraction_rate(ExtractorType::OilExtractor, Some(Purity::Normal)),
            120.0
        );
        assert_eq!(
            RawInput::calculate_extraction_rate(ExtractorType::OilExtractor, Some(Purity::Pure)),
            240.0
        );
    }

    #[test]
    fn test_resource_well_rates() {
        assert_eq!(
            RawInput::calculate_extraction_rate(
                ExtractorType::ResourceWellExtractor,
                Some(Purity::Impure)
            ),
            30.0
        );
        assert_eq!(
            RawInput::calculate_extraction_rate(
                ExtractorType::ResourceWellExtractor,
                Some(Purity::Normal)
            ),
            60.0
        );
        assert_eq!(
            RawInput::calculate_extraction_rate(
                ExtractorType::ResourceWellExtractor,
                Some(Purity::Pure)
            ),
            120.0
        );
    }

    // ===== RawInput Creation Tests =====

    #[test]
    fn test_create_valid_iron_ore_input() {
        let input = RawInput::new(
            1,
            ExtractorType::MinerMk2,
            Item::IronOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid iron ore input");

        assert_eq!(input.id, 1);
        assert_eq!(input.extractor_type, ExtractorType::MinerMk2);
        assert_eq!(input.item, Item::IronOre);
        assert_eq!(input.purity, Some(Purity::Normal));
        assert_eq!(input.quantity_per_min, 120.0);
    }

    #[test]
    fn test_create_valid_water_input() {
        let input = RawInput::new(1, ExtractorType::WaterExtractor, Item::Water, None)
            .expect("Should create valid water input");

        assert_eq!(input.id, 1);
        assert_eq!(input.extractor_type, ExtractorType::WaterExtractor);
        assert_eq!(input.item, Item::Water);
        assert_eq!(input.purity, None);
        assert_eq!(input.quantity_per_min, 120.0);
    }

    #[test]
    fn test_create_valid_oil_input() {
        let input = RawInput::new(
            1,
            ExtractorType::OilExtractor,
            Item::CrudeOil,
            Some(Purity::Pure),
        )
        .expect("Should create valid oil input");

        assert_eq!(input.quantity_per_min, 240.0);
    }

    #[test]
    fn test_create_invalid_extractor_item_mismatch() {
        let result = RawInput::new(
            1,
            ExtractorType::MinerMk1,
            Item::Water,
            Some(Purity::Normal),
        );

        assert!(result.is_err());
        match result {
            Err(RawInputError::IncompatibleExtractor { extractor, item }) => {
                assert_eq!(extractor, ExtractorType::MinerMk1);
                assert_eq!(item, Item::Water);
            }
            _ => panic!("Expected IncompatibleExtractor error"),
        }
    }

    #[test]
    fn test_create_invalid_missing_purity() {
        let result = RawInput::new(1, ExtractorType::MinerMk1, Item::IronOre, None);

        assert!(result.is_err());
        match result {
            Err(RawInputError::MissingPurity { extractor }) => {
                assert_eq!(extractor, ExtractorType::MinerMk1);
            }
            _ => panic!("Expected MissingPurity error"),
        }
    }

    #[test]
    fn test_create_invalid_unexpected_purity() {
        let result = RawInput::new(
            1,
            ExtractorType::WaterExtractor,
            Item::Water,
            Some(Purity::Normal),
        );

        assert!(result.is_err());
        match result {
            Err(RawInputError::UnexpectedPurity { extractor }) => {
                assert_eq!(extractor, ExtractorType::WaterExtractor);
            }
            _ => panic!("Expected UnexpectedPurity error"),
        }
    }

    // ===== Validation Tests =====

    #[test]
    fn test_validate_correct_input() {
        let input = RawInput::new(
            1,
            ExtractorType::MinerMk2,
            Item::CopperOre,
            Some(Purity::Pure),
        )
        .expect("Should create valid input");

        assert!(input.validate().is_ok());
    }

    #[test]
    fn test_validate_water_without_purity() {
        let input = RawInput::new(1, ExtractorType::WaterExtractor, Item::Water, None)
            .expect("Should create valid input");

        assert!(input.validate().is_ok());
    }

    // ===== Edge Case Tests =====

    #[test]
    fn test_all_ore_types_with_mk3_miner() {
        let ores = vec![
            Item::IronOre,
            Item::CopperOre,
            Item::Limestone,
            Item::Coal,
            Item::CateriumOre,
            Item::RawQuartz,
            Item::Sulfur,
            Item::Bauxite,
            Item::Uranium,
        ];

        for ore in ores {
            let result = RawInput::new(1, ExtractorType::MinerMk3, ore, Some(Purity::Normal));
            assert!(result.is_ok(), "Mk3 miner should work with {:?}", ore);
            assert_eq!(result.unwrap().quantity_per_min, 240.0);
        }
    }

    #[test]
    fn test_pure_uranium_with_mk3() {
        let input = RawInput::new(
            1,
            ExtractorType::MinerMk3,
            Item::Uranium,
            Some(Purity::Pure),
        )
        .expect("Should create valid uranium input");

        assert_eq!(input.quantity_per_min, 480.0);
    }

    #[test]
    fn test_impure_coal_with_mk1() {
        let input = RawInput::new(1, ExtractorType::MinerMk1, Item::Coal, Some(Purity::Impure))
            .expect("Should create valid coal input");

        assert_eq!(input.quantity_per_min, 30.0);
    }

    // ===== Resource Well Pressurizer Tests =====

    #[test]
    fn test_pressurizer_creation() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 150.0).expect("Should create valid pressurizer");
        assert_eq!(pressurizer.id, 1);
        assert_eq!(pressurizer.clock_speed, 150.0);
    }

    #[test]
    fn test_pressurizer_invalid_clock_speed_too_low() {
        let result = ResourceWellPressurizer::new(1, -10.0);
        assert!(result.is_err());
        match result {
            Err(RawInputError::InvalidClockSpeed { clock_speed }) => {
                assert_eq!(clock_speed, -10.0);
            }
            _ => panic!("Expected InvalidClockSpeed error"),
        }
    }

    #[test]
    fn test_pressurizer_invalid_clock_speed_too_high() {
        let result = ResourceWellPressurizer::new(1, 300.0);
        assert!(result.is_err());
        match result {
            Err(RawInputError::InvalidClockSpeed { clock_speed }) => {
                assert_eq!(clock_speed, 300.0);
            }
            _ => panic!("Expected InvalidClockSpeed error"),
        }
    }

    #[test]
    fn test_pressurizer_power_consumption() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create valid pressurizer");
        // At 100% clock speed, power consumption should be exactly 150MW
        assert_eq!(pressurizer.power_consumption(), 150.0);
    }

    #[test]
    fn test_pressurizer_power_consumption_at_50_percent() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 50.0).expect("Should create valid pressurizer");
        // At 50% clock speed: 150 * (0.5)^1.321928 = 150 * 0.4 = 60MW (approximately)
        let power = pressurizer.power_consumption();
        assert!(
            (power - 60.0).abs() < 1.0,
            "Power should be approximately 60MW, got {}",
            power
        );
    }

    #[test]
    fn test_pressurizer_power_consumption_at_200_percent() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 200.0).expect("Should create valid pressurizer");
        // At 200% clock speed: 150 * (2.0)^1.321928 = 150 * 2.5 = 375MW (approximately)
        let power = pressurizer.power_consumption();
        assert!(
            (power - 375.0).abs() < 1.0,
            "Power should be approximately 375MW, got {}",
            power
        );
    }

    #[test]
    fn test_pressurizer_set_clock_speed() {
        let mut pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create valid pressurizer");
        pressurizer
            .set_clock_speed(175.0)
            .expect("Should update clock speed");
        assert_eq!(pressurizer.clock_speed, 175.0);
    }

    #[test]
    fn test_pressurizer_set_invalid_clock_speed() {
        let mut pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create valid pressurizer");
        let result = pressurizer.set_clock_speed(300.0);
        assert!(result.is_err());
        assert_eq!(pressurizer.clock_speed, 100.0); // Should remain unchanged
    }

    // ===== Resource Well Extractor Tests =====

    #[test]
    fn test_extractor_creation() {
        let extractor = ResourceWellExtractor::new(1, Purity::Pure);
        assert_eq!(extractor.id, 1);
        assert_eq!(extractor.purity, Purity::Pure);
    }

    #[test]
    fn test_extractor_extraction_rate() {
        let extractor = ResourceWellExtractor::new(1, Purity::Normal);
        assert_eq!(extractor.extraction_rate(100.0), 60.0); // 60 * 1.0
        assert_eq!(extractor.extraction_rate(50.0), 30.0); // 60 * 0.5
        assert_eq!(extractor.extraction_rate(200.0), 120.0); // 60 * 2.0
    }

    #[test]
    fn test_extractor_extraction_rate_by_purity() {
        let impure = ResourceWellExtractor::new(1, Purity::Impure);
        let normal = ResourceWellExtractor::new(2, Purity::Normal);
        let pure = ResourceWellExtractor::new(3, Purity::Pure);

        assert_eq!(impure.extraction_rate(100.0), 30.0); // 30 * 1.0
        assert_eq!(normal.extraction_rate(100.0), 60.0); // 60 * 1.0
        assert_eq!(pure.extraction_rate(100.0), 120.0); // 120 * 1.0
    }

    // ===== Resource Well System Tests =====

    #[test]
    fn test_create_resource_well_system() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create pressurizer");
        let extractors = vec![
            ResourceWellExtractor::new(1, Purity::Normal),
            ResourceWellExtractor::new(2, Purity::Pure),
        ];

        let raw_input = RawInput::new_resource_well(1, Item::CrudeOil, pressurizer, extractors)
            .expect("Should create valid resource well system");

        assert_eq!(raw_input.id, 1);
        assert_eq!(
            raw_input.extractor_type,
            ExtractorType::ResourceWellExtractor
        );
        assert_eq!(raw_input.item, Item::CrudeOil);
        assert!(raw_input.pressurizer.is_some());
        assert_eq!(raw_input.extractors.len(), 2);
        assert_eq!(raw_input.quantity_per_min, 180.0); // 60 + 120
        assert_eq!(raw_input.power_consumption(), 150.0); // Pressurizer at 100%
    }

    #[test]
    fn test_create_resource_well_system_invalid_item() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create pressurizer");
        let extractors = vec![ResourceWellExtractor::new(1, Purity::Normal)];

        let result = RawInput::new_resource_well(1, Item::IronOre, pressurizer, extractors);
        assert!(result.is_err());
        match result {
            Err(RawInputError::IncompatibleExtractor { extractor, item }) => {
                assert_eq!(extractor, ExtractorType::ResourceWellExtractor);
                assert_eq!(item, Item::IronOre);
            }
            _ => panic!("Expected IncompatibleExtractor error"),
        }
    }

    #[test]
    fn test_create_resource_well_system_no_extractors() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create pressurizer");
        let extractors = vec![];

        let result = RawInput::new_resource_well(1, Item::CrudeOil, pressurizer, extractors);
        assert!(result.is_err());
        match result {
            Err(RawInputError::NoExtractors) => {
                // Expected error
            }
            _ => panic!("Expected NoExtractors error"),
        }
    }

    #[test]
    fn test_resource_well_add_extractor() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create pressurizer");
        let extractors = vec![ResourceWellExtractor::new(1, Purity::Normal)];

        let mut raw_input = RawInput::new_resource_well(1, Item::CrudeOil, pressurizer, extractors)
            .expect("Should create valid resource well system");

        assert_eq!(raw_input.quantity_per_min, 60.0);

        let new_extractor = ResourceWellExtractor::new(2, Purity::Pure);
        raw_input
            .add_extractor(new_extractor)
            .expect("Should add extractor");

        assert_eq!(raw_input.extractors.len(), 2);
        assert_eq!(raw_input.quantity_per_min, 180.0); // 60 + 120
    }

    #[test]
    fn test_resource_well_remove_extractor() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create pressurizer");
        let extractors = vec![
            ResourceWellExtractor::new(1, Purity::Normal),
            ResourceWellExtractor::new(2, Purity::Pure),
        ];

        let mut raw_input = RawInput::new_resource_well(1, Item::CrudeOil, pressurizer, extractors)
            .expect("Should create valid resource well system");

        assert_eq!(raw_input.quantity_per_min, 180.0); // 60 + 120

        raw_input
            .remove_extractor(1)
            .expect("Should remove extractor");

        assert_eq!(raw_input.extractors.len(), 1);
        assert_eq!(raw_input.quantity_per_min, 120.0); // Only the pure one remains
    }

    #[test]
    fn test_resource_well_remove_nonexistent_extractor() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create pressurizer");
        let extractors = vec![ResourceWellExtractor::new(1, Purity::Normal)];

        let mut raw_input = RawInput::new_resource_well(1, Item::CrudeOil, pressurizer, extractors)
            .expect("Should create valid resource well system");

        let result = raw_input.remove_extractor(999);
        assert!(result.is_err());
        match result {
            Err(RawInputError::ExtractorNotFound { id }) => {
                assert_eq!(id, 999);
            }
            _ => panic!("Expected ExtractorNotFound error"),
        }
    }

    #[test]
    fn test_resource_well_remove_last_extractor() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create pressurizer");
        let extractors = vec![ResourceWellExtractor::new(1, Purity::Normal)];

        let mut raw_input = RawInput::new_resource_well(1, Item::CrudeOil, pressurizer, extractors)
            .expect("Should create valid resource well system");

        let result = raw_input.remove_extractor(1);
        assert!(result.is_err());
        match result {
            Err(RawInputError::NoExtractors) => {
                // Expected error
            }
            _ => panic!("Expected NoExtractors error"),
        }
    }

    #[test]
    fn test_resource_well_set_pressurizer() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create pressurizer");
        let extractors = vec![ResourceWellExtractor::new(1, Purity::Normal)];

        let mut raw_input = RawInput::new_resource_well(1, Item::CrudeOil, pressurizer, extractors)
            .expect("Should create valid resource well system");

        assert_eq!(raw_input.quantity_per_min, 60.0);

        let new_pressurizer =
            ResourceWellPressurizer::new(2, 200.0).expect("Should create pressurizer");
        raw_input
            .set_pressurizer(new_pressurizer)
            .expect("Should set pressurizer");

        assert_eq!(raw_input.quantity_per_min, 120.0); // 60 * 2.0
        let power = raw_input.power_consumption();
        assert!(
            (power - 375.0).abs() < 0.1,
            "Power should be approximately 375MW, got {}",
            power
        ); // Pressurizer at 200%
    }

    #[test]
    fn test_resource_well_update_extraction_rates() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create pressurizer");
        let extractors = vec![
            ResourceWellExtractor::new(1, Purity::Normal),
            ResourceWellExtractor::new(2, Purity::Pure),
        ];

        let mut raw_input = RawInput::new_resource_well(1, Item::CrudeOil, pressurizer, extractors)
            .expect("Should create valid resource well system");

        assert_eq!(raw_input.quantity_per_min, 180.0); // 60 + 120

        // Manually change clock speed to test update
        if let Some(pressurizer) = &mut raw_input.pressurizer {
            pressurizer.clock_speed = 50.0;
        }

        raw_input.update_extraction_rates();
        assert_eq!(raw_input.quantity_per_min, 90.0); // 30 + 60
    }

    #[test]
    fn test_resource_well_power_consumption() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 150.0).expect("Should create pressurizer");
        let extractors = vec![ResourceWellExtractor::new(1, Purity::Normal)];

        let raw_input = RawInput::new_resource_well(1, Item::CrudeOil, pressurizer, extractors)
            .expect("Should create valid resource well system");

        // Power should come only from pressurizer, not extractors
        let power = raw_input.power_consumption();
        let expected = 150.0_f32 * (1.5_f32).powf(1.321928_f32);
        assert!(
            (power - expected).abs() < 1.0,
            "Power should be approximately {}, got {}",
            expected,
            power
        );
    }

    #[test]
    fn test_regular_extractor_power_consumption() {
        let input = RawInput::new(
            1,
            ExtractorType::MinerMk2,
            Item::IronOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid input");

        assert_eq!(input.power_consumption(), 15.0); // Miner Mk2 consumes 15MW
    }

    #[test]
    fn test_all_extractor_power_consumption() {
        // Test all extractor types have correct power consumption
        let mk1 = RawInput::new(
            1,
            ExtractorType::MinerMk1,
            Item::IronOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid input");
        let mk2 = RawInput::new(
            2,
            ExtractorType::MinerMk2,
            Item::IronOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid input");
        let mk3 = RawInput::new(
            3,
            ExtractorType::MinerMk3,
            Item::IronOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid input");
        let water = RawInput::new(4, ExtractorType::WaterExtractor, Item::Water, None)
            .expect("Should create valid input");
        let oil = RawInput::new(
            5,
            ExtractorType::OilExtractor,
            Item::CrudeOil,
            Some(Purity::Normal),
        )
        .expect("Should create valid input");

        assert_eq!(mk1.power_consumption(), 5.0); // Miner Mk1: 5MW
        assert_eq!(mk2.power_consumption(), 15.0); // Miner Mk2: 15MW
        assert_eq!(mk3.power_consumption(), 45.0); // Miner Mk3: 45MW
        assert_eq!(water.power_consumption(), 20.0); // Water Extractor: 20MW
        assert_eq!(oil.power_consumption(), 40.0); // Oil Extractor: 40MW
    }

    #[test]
    fn test_validate_resource_well_system() {
        let pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create pressurizer");
        let extractors = vec![ResourceWellExtractor::new(1, Purity::Normal)];

        let raw_input = RawInput::new_resource_well(1, Item::CrudeOil, pressurizer, extractors)
            .expect("Should create valid resource well system");

        assert!(raw_input.validate().is_ok());
    }

    #[test]
    fn test_add_extractor_to_non_resource_well() {
        let mut input = RawInput::new(
            1,
            ExtractorType::MinerMk2,
            Item::IronOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid input");

        let extractor = ResourceWellExtractor::new(1, Purity::Normal);
        let result = input.add_extractor(extractor);
        assert!(result.is_err());
        match result {
            Err(RawInputError::ExtractorsWithoutPressurizer) => {
                // Expected error
            }
            _ => panic!("Expected ExtractorsWithoutPressurizer error"),
        }
    }

    #[test]
    fn test_set_pressurizer_on_non_resource_well() {
        let mut input = RawInput::new(
            1,
            ExtractorType::MinerMk2,
            Item::IronOre,
            Some(Purity::Normal),
        )
        .expect("Should create valid input");

        let pressurizer =
            ResourceWellPressurizer::new(1, 100.0).expect("Should create pressurizer");
        let result = input.set_pressurizer(pressurizer);
        assert!(result.is_err());
        match result {
            Err(RawInputError::PressurizerOnNonResourceWell) => {
                // Expected error
            }
            _ => panic!("Expected PressurizerOnNonResourceWell error"),
        }
    }
}
