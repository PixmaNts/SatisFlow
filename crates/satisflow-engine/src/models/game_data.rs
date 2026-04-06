use core::f32;

use serde::{Deserialize, Serialize};

/// Game version these constants are verified against
pub const GAME_VERSION: &str = "1.2";

/// Overclock exponent: log₂(2.5) ≈ 1.321928
/// Power consumption scales with (clock_speed/100)^OVERCLOCK_EXPONENT
pub const OVERCLOCK_EXPONENT: f32 = 1.321928;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MachineType {
    Constructor,
    Assembler,
    Manufacturer,
    Smelter,
    Foundry,
    Refinery,
    Blender,
    Packager,
    ParticleAccelerator,
    QuantumEncoder,
    Converter,
    Manual, // For buildings, equipment, vehicles - built manually, not in production lines
}

impl MachineType {
    pub fn max_somersloop(&self) -> u8 {
        match self {
            MachineType::Constructor => 1,
            MachineType::Assembler => 2,
            MachineType::Manufacturer => 4,
            MachineType::Smelter => 1,
            MachineType::Foundry => 2,
            MachineType::Refinery => 2,
            MachineType::Blender => 4,
            MachineType::Packager => 1,
            MachineType::ParticleAccelerator => 4,
            MachineType::QuantumEncoder => 4,
            MachineType::Converter => 2,
            MachineType::Manual => 0,
        }
    }
    pub fn base_power_mw(&self) -> f32 {
        match self {
            MachineType::Constructor => 4.0,
            MachineType::Assembler => 16.0,
            MachineType::Manufacturer => 32.0,
            MachineType::Smelter => 4.0,
            MachineType::Foundry => 16.0,
            MachineType::Refinery => 16.0,
            MachineType::Blender => 32.0,
            MachineType::Packager => 4.0,
            MachineType::ParticleAccelerator => 64.0,
            MachineType::QuantumEncoder => 1000.0,
            MachineType::Converter => 250.0,
            MachineType::Manual => 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConveyorType {
    Mk1,
    Mk2,
    Mk3,
    Mk4,
    Mk5,
    Mk6,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineType {
    Mk1,
    Mk2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinerType {
    Mk1,
    Mk2,
    Mk3,
}

/// Purity multipliers for resource extraction
pub mod purity {
    /// Impure node multiplier: 0.5x (50% yield)
    pub const IMPURE_MULTIPLIER: f32 = 0.5;
    /// Normal node multiplier: 1.0x (100% yield)
    pub const NORMAL_MULTIPLIER: f32 = 1.0;
    /// Pure node multiplier: 2.0x (200% yield)
    pub const PURE_MULTIPLIER: f32 = 2.0;
}

/// Conveyor belt speeds (items per minute)
pub mod conveyor {
    /// Conveyor Belt Mk.1: 60 items/min
    pub const MK1_SPEED: f32 = 60.0;
    /// Conveyor Belt Mk.2: 120 items/min
    pub const MK2_SPEED: f32 = 120.0;
    /// Conveyor Belt Mk.3: 270 items/min
    pub const MK3_SPEED: f32 = 270.0;
    /// Conveyor Belt Mk.4: 480 items/min
    pub const MK4_SPEED: f32 = 480.0;
    /// Conveyor Belt Mk.5: 780 items/min
    pub const MK5_SPEED: f32 = 780.0;
    /// Conveyor Belt Mk.6: 1200 items/min
    pub const MK6_SPEED: f32 = 1200.0;
}

/// Pipeline capacities (m³ per minute)
pub mod pipeline {
    /// Pipeline Mk.1: 300 m³/min
    pub const MK1_CAPACITY: f32 = 300.0;
    /// Pipeline Mk.2: 600 m³/min
    pub const MK2_CAPACITY: f32 = 600.0;
}

/// Machine base power consumption (MW at 100% clock speed)
pub mod machine_power {
    /// Constructor: 4 MW
    pub const CONSTRUCTOR: f32 = 4.0;
    /// Assembler: 16 MW
    pub const ASSEMBLER: f32 = 16.0;
    /// Manufacturer: 32 MW
    pub const MANUFACTURER: f32 = 32.0;
    /// Smelter: 4 MW
    pub const SMELTER: f32 = 4.0;
    /// Foundry: 16 MW
    pub const FOUNDRY: f32 = 16.0;
    /// Refinery: 16 MW
    pub const REFINERY: f32 = 16.0;
    /// Blender: 32 MW
    pub const BLENDER: f32 = 32.0;
    /// Packager: 4 MW
    pub const PACKAGER: f32 = 4.0;
    /// Particle Accelerator: 64 MW
    pub const PARTICLE_ACCELERATOR: f32 = 64.0;
    /// Quantum Encoder: 1000 MW (average, fluctuates 0-2000 MW)
    pub const QUANTUM_ENCODER: f32 = 1000.0;
    /// Converter: 250 MW (average, fluctuates 100-400 MW)
    pub const CONVERTER: f32 = 250.0;
}

/// Miner power consumption (MW at 100% clock speed)
pub mod miner_power {
    /// Miner Mk.1: 5 MW
    pub const MK1: f32 = 5.0;
    /// Miner Mk.2: 15 MW
    pub const MK2: f32 = 15.0;
    /// Miner Mk.3: 45 MW
    pub const MK3: f32 = 45.0;
    /// Water Extractor: 20 MW
    pub const WATER_EXTRACTOR: f32 = 20.0;
    /// Oil Extractor: 40 MW
    pub const OIL_EXTRACTOR: f32 = 40.0;
}

/// Somersloop power formula multiplier
/// Power multiplier = (1 + somersloop_count / max_somersloop)^2
pub fn somersloop_power_multiplier(somersloop_count: u8, max_somersloop: u8) -> f32 {
    let ratio = somersloop_count as f32 / max_somersloop as f32;
    (1.0 + ratio).powi(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Game Version Verification Tests
    // =========================================================================

    #[test]
    fn test_game_version_is_1_2() {
        assert_eq!(GAME_VERSION, "1.2", "Game version must be 1.2");
    }

    // =========================================================================
    // Overclock Exponent Verification Tests
    // =========================================================================

    #[test]
    fn test_overclock_exponent_is_log2_2_5() {
        // log₂(2.5) ≈ 1.321928094887362
        const EXPECTED: f32 = 1.321928;
        assert!(
            (OVERCLOCK_EXPONENT - EXPECTED).abs() < 0.000001,
            "Overclock exponent should be log₂(2.5) ≈ 1.321928, got {}",
            OVERCLOCK_EXPONENT
        );
    }

    #[test]
    fn test_overclock_exponent_250_percent() {
        // At 250% clock speed, power multiplier = 2.5^1.321928 ≈ 3.357
        // This is because 1.321928 ≈ log₂(2.5), so 2.5^log₂(2.5) ≈ 3.357
        let multiplier = 2.5_f32.powf(OVERCLOCK_EXPONENT);
        let expected = 3.357_f32;
        assert!(
            (multiplier - expected).abs() < 0.01,
            "250% OC power multiplier should be approximately {}, got {}",
            expected,
            multiplier
        );
    }

    // =========================================================================
    // Purity Multiplier Verification Tests
    // =========================================================================

    #[test]
    fn test_purity_impure_multiplier() {
        assert_eq!(
            purity::IMPURE_MULTIPLIER,
            0.5,
            "Impure purity multiplier must be 0.5"
        );
    }

    #[test]
    fn test_purity_normal_multiplier() {
        assert_eq!(
            purity::NORMAL_MULTIPLIER,
            1.0,
            "Normal purity multiplier must be 1.0"
        );
    }

    #[test]
    fn test_purity_pure_multiplier() {
        assert_eq!(
            purity::PURE_MULTIPLIER,
            2.0,
            "Pure purity multiplier must be 2.0"
        );
    }

    // =========================================================================
    // Conveyor Speed Verification Tests
    // =========================================================================

    #[test]
    fn test_conveyor_mk1_speed() {
        assert_eq!(
            conveyor::MK1_SPEED,
            60.0,
            "Conveyor Mk.1 speed must be 60 items/min"
        );
    }

    #[test]
    fn test_conveyor_mk2_speed() {
        assert_eq!(
            conveyor::MK2_SPEED,
            120.0,
            "Conveyor Mk.2 speed must be 120 items/min"
        );
    }

    #[test]
    fn test_conveyor_mk3_speed() {
        assert_eq!(
            conveyor::MK3_SPEED,
            270.0,
            "Conveyor Mk.3 speed must be 270 items/min"
        );
    }

    #[test]
    fn test_conveyor_mk4_speed() {
        assert_eq!(
            conveyor::MK4_SPEED,
            480.0,
            "Conveyor Mk.4 speed must be 480 items/min"
        );
    }

    #[test]
    fn test_conveyor_mk5_speed() {
        assert_eq!(
            conveyor::MK5_SPEED,
            780.0,
            "Conveyor Mk.5 speed must be 780 items/min"
        );
    }

    #[test]
    fn test_conveyor_mk6_speed() {
        assert_eq!(
            conveyor::MK6_SPEED,
            1200.0,
            "Conveyor Mk.6 speed must be 1200 items/min"
        );
    }

    #[test]
    fn test_conveyor_speed_progression() {
        // Each tier should be faster than the previous
        assert!(conveyor::MK2_SPEED > conveyor::MK1_SPEED);
        assert!(conveyor::MK3_SPEED > conveyor::MK2_SPEED);
        assert!(conveyor::MK4_SPEED > conveyor::MK3_SPEED);
        assert!(conveyor::MK5_SPEED > conveyor::MK4_SPEED);
        assert!(conveyor::MK6_SPEED > conveyor::MK5_SPEED);
    }

    // =========================================================================
    // Pipeline Capacity Verification Tests
    // =========================================================================

    #[test]
    fn test_pipeline_mk1_capacity() {
        assert_eq!(
            pipeline::MK1_CAPACITY,
            300.0,
            "Pipeline Mk.1 capacity must be 300 m³/min"
        );
    }

    #[test]
    fn test_pipeline_mk2_capacity() {
        assert_eq!(
            pipeline::MK2_CAPACITY,
            600.0,
            "Pipeline Mk.2 capacity must be 600 m³/min"
        );
    }

    #[test]
    fn test_pipeline_mk2_double_mk1() {
        assert_eq!(
            pipeline::MK2_CAPACITY,
            pipeline::MK1_CAPACITY * 2.0,
            "Pipeline Mk.2 should be exactly 2x Mk.1 capacity"
        );
    }

    // =========================================================================
    // Machine Base Power Verification Tests
    // =========================================================================

    #[test]
    fn test_machine_constructor_power() {
        assert_eq!(
            MachineType::Constructor.base_power_mw(),
            4.0,
            "Constructor base power must be 4 MW"
        );
        assert_eq!(
            machine_power::CONSTRUCTOR,
            4.0,
            "machine_power::CONSTRUCTOR constant must be 4 MW"
        );
    }

    #[test]
    fn test_machine_assembler_power() {
        assert_eq!(
            MachineType::Assembler.base_power_mw(),
            16.0,
            "Assembler base power must be 16 MW"
        );
        assert_eq!(
            machine_power::ASSEMBLER,
            16.0,
            "machine_power::ASSEMBLER constant must be 16 MW"
        );
    }

    #[test]
    fn test_machine_manufacturer_power() {
        assert_eq!(
            MachineType::Manufacturer.base_power_mw(),
            32.0,
            "Manufacturer base power must be 32 MW"
        );
        assert_eq!(
            machine_power::MANUFACTURER,
            32.0,
            "machine_power::MANUFACTURER constant must be 32 MW"
        );
    }

    #[test]
    fn test_machine_smelter_power() {
        assert_eq!(
            MachineType::Smelter.base_power_mw(),
            4.0,
            "Smelter base power must be 4 MW"
        );
        assert_eq!(
            machine_power::SMELTER,
            4.0,
            "machine_power::SMELTER constant must be 4 MW"
        );
    }

    #[test]
    fn test_machine_foundry_power() {
        assert_eq!(
            MachineType::Foundry.base_power_mw(),
            16.0,
            "Foundry base power must be 16 MW"
        );
        assert_eq!(
            machine_power::FOUNDRY,
            16.0,
            "machine_power::FOUNDRY constant must be 16 MW"
        );
    }

    #[test]
    fn test_machine_refinery_power() {
        assert_eq!(
            MachineType::Refinery.base_power_mw(),
            16.0,
            "Refinery base power must be 16 MW"
        );
        assert_eq!(
            machine_power::REFINERY,
            16.0,
            "machine_power::REFINERY constant must be 16 MW"
        );
    }

    #[test]
    fn test_machine_blender_power() {
        assert_eq!(
            MachineType::Blender.base_power_mw(),
            32.0,
            "Blender base power must be 32 MW"
        );
        assert_eq!(
            machine_power::BLENDER,
            32.0,
            "machine_power::BLENDER constant must be 32 MW"
        );
    }

    #[test]
    fn test_machine_packager_power() {
        assert_eq!(
            MachineType::Packager.base_power_mw(),
            4.0,
            "Packager base power must be 4 MW"
        );
        assert_eq!(
            machine_power::PACKAGER,
            4.0,
            "machine_power::PACKAGER constant must be 4 MW"
        );
    }

    #[test]
    fn test_machine_particle_accelerator_power() {
        assert_eq!(
            MachineType::ParticleAccelerator.base_power_mw(),
            64.0,
            "Particle Accelerator base power must be 64 MW"
        );
        assert_eq!(
            machine_power::PARTICLE_ACCELERATOR,
            64.0,
            "machine_power::PARTICLE_ACCELERATOR constant must be 64 MW"
        );
    }

    #[test]
    fn test_machine_quantum_encoder_power() {
        assert_eq!(
            MachineType::QuantumEncoder.base_power_mw(),
            1000.0,
            "Quantum Encoder base power must be 1000 MW (average)"
        );
        assert_eq!(
            machine_power::QUANTUM_ENCODER,
            1000.0,
            "machine_power::QUANTUM_ENCODER constant must be 1000 MW"
        );
    }

    #[test]
    fn test_machine_converter_power() {
        assert_eq!(
            MachineType::Converter.base_power_mw(),
            250.0,
            "Converter base power must be 250 MW (average)"
        );
        assert_eq!(
            machine_power::CONVERTER,
            250.0,
            "machine_power::CONVERTER constant must be 250 MW"
        );
    }

    #[test]
    fn test_machine_manual_power() {
        assert_eq!(
            MachineType::Manual.base_power_mw(),
            0.0,
            "Manual machine base power must be 0 MW"
        );
    }

    // =========================================================================
    // Max Somersloop Verification Tests
    // =========================================================================

    #[test]
    fn test_max_somersloop_constructor() {
        assert_eq!(MachineType::Constructor.max_somersloop(), 1);
    }

    #[test]
    fn test_max_somersloop_assembler() {
        assert_eq!(MachineType::Assembler.max_somersloop(), 2);
    }

    #[test]
    fn test_max_somersloop_manufacturer() {
        assert_eq!(MachineType::Manufacturer.max_somersloop(), 4);
    }

    #[test]
    fn test_max_somersloop_smelter() {
        assert_eq!(MachineType::Smelter.max_somersloop(), 1);
    }

    #[test]
    fn test_max_somersloop_foundry() {
        assert_eq!(MachineType::Foundry.max_somersloop(), 2);
    }

    #[test]
    fn test_max_somersloop_refinery() {
        assert_eq!(MachineType::Refinery.max_somersloop(), 2);
    }

    #[test]
    fn test_max_somersloop_blender() {
        assert_eq!(MachineType::Blender.max_somersloop(), 4);
    }

    #[test]
    fn test_max_somersloop_packager() {
        assert_eq!(MachineType::Packager.max_somersloop(), 1);
    }

    #[test]
    fn test_max_somersloop_particle_accelerator() {
        assert_eq!(MachineType::ParticleAccelerator.max_somersloop(), 4);
    }

    #[test]
    fn test_max_somersloop_quantum_encoder() {
        assert_eq!(MachineType::QuantumEncoder.max_somersloop(), 4);
    }

    #[test]
    fn test_max_somersloop_converter() {
        assert_eq!(MachineType::Converter.max_somersloop(), 2);
    }

    #[test]
    fn test_max_somersloop_manual() {
        assert_eq!(MachineType::Manual.max_somersloop(), 0);
    }

    // =========================================================================
    // Miner Power Verification Tests
    // =========================================================================

    #[test]
    fn test_miner_mk1_power() {
        assert_eq!(miner_power::MK1, 5.0, "Miner Mk.1 power must be 5 MW");
    }

    #[test]
    fn test_miner_mk2_power() {
        assert_eq!(miner_power::MK2, 15.0, "Miner Mk.2 power must be 15 MW");
    }

    #[test]
    fn test_miner_mk3_power() {
        assert_eq!(miner_power::MK3, 45.0, "Miner Mk.3 power must be 45 MW");
    }

    #[test]
    fn test_water_extractor_power() {
        assert_eq!(
            miner_power::WATER_EXTRACTOR,
            20.0,
            "Water Extractor power must be 20 MW"
        );
    }

    #[test]
    fn test_oil_extractor_power() {
        assert_eq!(
            miner_power::OIL_EXTRACTOR,
            40.0,
            "Oil Extractor power must be 40 MW"
        );
    }

    // =========================================================================
    // Somersloop Power Formula Verification Tests
    // =========================================================================

    #[test]
    fn test_somersloop_power_formula_no_somersloop() {
        // (1 + 0/max)^2 = 1^2 = 1
        let multiplier = somersloop_power_multiplier(0, 4);
        assert!(
            (multiplier - 1.0).abs() < 0.0001,
            "No somersloop should give 1.0x power multiplier, got {}",
            multiplier
        );
    }

    #[test]
    fn test_somersloop_power_formula_full_somersloop() {
        // (1 + max/max)^2 = (1 + 1)^2 = 4
        let multiplier = somersloop_power_multiplier(4, 4);
        assert!(
            (multiplier - 4.0).abs() < 0.0001,
            "Full somersloop should give 4.0x power multiplier, got {}",
            multiplier
        );
    }

    #[test]
    fn test_somersloop_power_formula_half_somersloop() {
        // (1 + 2/4)^2 = (1 + 0.5)^2 = 2.25
        let multiplier = somersloop_power_multiplier(2, 4);
        assert!(
            (multiplier - 2.25).abs() < 0.0001,
            "Half somersloop should give 2.25x power multiplier, got {}",
            multiplier
        );
    }

    #[test]
    fn test_somersloop_power_formula_quarter_somersloop() {
        // (1 + 1/4)^2 = (1 + 0.25)^2 = 1.5625
        let multiplier = somersloop_power_multiplier(1, 4);
        assert!(
            (multiplier - 1.5625).abs() < 0.0001,
            "Quarter somersloop should give 1.5625x power multiplier, got {}",
            multiplier
        );
    }

    // =========================================================================
    // Cross-Reference Verification Tests
    // =========================================================================

    #[test]
    fn test_machine_power_constants_match_enum() {
        // Verify that the constants match the enum method values
        assert_eq!(
            MachineType::Constructor.base_power_mw(),
            machine_power::CONSTRUCTOR
        );
        assert_eq!(
            MachineType::Assembler.base_power_mw(),
            machine_power::ASSEMBLER
        );
        assert_eq!(
            MachineType::Manufacturer.base_power_mw(),
            machine_power::MANUFACTURER
        );
        assert_eq!(MachineType::Smelter.base_power_mw(), machine_power::SMELTER);
        assert_eq!(MachineType::Foundry.base_power_mw(), machine_power::FOUNDRY);
        assert_eq!(
            MachineType::Refinery.base_power_mw(),
            machine_power::REFINERY
        );
        assert_eq!(MachineType::Blender.base_power_mw(), machine_power::BLENDER);
        assert_eq!(
            MachineType::Packager.base_power_mw(),
            machine_power::PACKAGER
        );
        assert_eq!(
            MachineType::ParticleAccelerator.base_power_mw(),
            machine_power::PARTICLE_ACCELERATOR
        );
        assert_eq!(
            MachineType::QuantumEncoder.base_power_mw(),
            machine_power::QUANTUM_ENCODER
        );
        assert_eq!(
            MachineType::Converter.base_power_mw(),
            machine_power::CONVERTER
        );
    }
}
