use core::f32;

use serde::{Deserialize, Serialize};

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
