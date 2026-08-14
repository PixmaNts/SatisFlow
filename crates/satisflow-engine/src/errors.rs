//! Error types for the Satisflow engine

use thiserror::Error;

/// Main error type for Satisflow engine operations
#[derive(Debug, Clone, Error)]
pub enum SatisflowError {
    #[error("Factory not found: {id}")]
    FactoryNotFound { id: String },

    #[error("Logistics line not found: {id}")]
    LogisticsNotFound { id: String },

    #[error("Blueprint not found: {id}")]
    BlueprintNotFound { id: String },

    #[error("Production line not found: {id}")]
    ProductionLineNotFound { id: String },

    #[error("Raw input not found: {id}")]
    RawInputNotFound { id: String },

    #[error("Power generator not found: {id}")]
    PowerGeneratorNotFound { id: String },

    #[error("Invalid input: {message}")]
    InvalidInput { message: String },

    #[error("Serialization error: {message}")]
    Serialization { message: String },

    #[error("Version error: {message}")]
    Version { message: String },
}

impl From<crate::version::VersionError> for SatisflowError {
    fn from(e: crate::version::VersionError) -> Self {
        SatisflowError::Version {
            message: e.to_string(),
        }
    }
}

impl From<serde_json::Error> for SatisflowError {
    fn from(e: serde_json::Error) -> Self {
        SatisflowError::Serialization {
            message: e.to_string(),
        }
    }
}
