//! Version management and compatibility checking for save files

use serde::{Deserialize, Serialize};
use std::fmt;

/// Semantic version for save files
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SaveVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SaveVersion {
    /// Create a new version
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parse a version string like "0.1.0"
    pub fn parse(version: &str) -> Result<Self, VersionError> {
        let parts: Vec<&str> = version.split('.').collect();

        if parts.len() != 3 {
            return Err(VersionError::InvalidFormat {
                version: version.to_string(),
            });
        }

        let major = parts[0].parse().map_err(|_| VersionError::InvalidFormat {
            version: version.to_string(),
        })?;
        let minor = parts[1].parse().map_err(|_| VersionError::InvalidFormat {
            version: version.to_string(),
        })?;
        let patch = parts[2].parse().map_err(|_| VersionError::InvalidFormat {
            version: version.to_string(),
        })?;

        Ok(Self::new(major, minor, patch))
    }

    /// Get the current engine version
    pub fn current() -> Self {
        Self::parse(env!("CARGO_PKG_VERSION")).expect("Engine version should always be valid")
    }

    /// Check if this version is compatible with another version
    ///
    /// Compatibility rules:
    /// - Same major version = compatible
    /// - Different major version = incompatible
    pub fn is_compatible_with(&self, other: &SaveVersion) -> bool {
        self.major == other.major
    }

    /// Check if this save file is newer than the engine
    pub fn is_newer_than(&self, other: &SaveVersion) -> bool {
        self > other
    }

    /// Check if this save file is older than the engine
    pub fn is_older_than(&self, other: &SaveVersion) -> bool {
        self < other
    }
}

impl fmt::Display for SaveVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Errors related to version handling
#[derive(Debug, thiserror::Error)]
pub enum VersionError {
    #[error(
        "Invalid version format: '{version}'. Expected format: MAJOR.MINOR.PATCH (e.g., '0.1.0')"
    )]
    InvalidFormat { version: String },

    #[error(
        "Save file version {save_version} is incompatible with engine version {engine_version}"
    )]
    Incompatible {
        save_version: String,
        engine_version: String,
    },

    #[error("Save file is too new: file version {save_version}, engine version {engine_version}. Please update Satisflow to load this save file.")]
    SaveTooNew {
        save_version: String,
        engine_version: String,
    },

    #[error("Save file is too old: file version {save_version}, engine version {engine_version}. Migration not yet implemented.")]
    SaveTooOld {
        save_version: String,
        engine_version: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        let v = SaveVersion::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_version_parsing_invalid() {
        assert!(SaveVersion::parse("1.2").is_err());
        assert!(SaveVersion::parse("1.2.3.4").is_err());
        assert!(SaveVersion::parse("a.b.c").is_err());
        assert!(SaveVersion::parse("").is_err());
    }

    #[test]
    fn test_version_display() {
        let v = SaveVersion::new(1, 2, 3);
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn test_version_comparison() {
        let v1 = SaveVersion::new(1, 0, 0);
        let v2 = SaveVersion::new(1, 1, 0);
        let v3 = SaveVersion::new(2, 0, 0);

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v1 < v3);

        assert!(v2.is_newer_than(&v1));
        assert!(v1.is_older_than(&v2));
    }

    #[test]
    fn test_version_compatibility() {
        let v1_0_0 = SaveVersion::new(1, 0, 0);
        let v1_1_0 = SaveVersion::new(1, 1, 0);
        let v1_2_5 = SaveVersion::new(1, 2, 5);
        let v2_0_0 = SaveVersion::new(2, 0, 0);

        // Same major version = compatible
        assert!(v1_0_0.is_compatible_with(&v1_1_0));
        assert!(v1_1_0.is_compatible_with(&v1_2_5));

        // Different major version = incompatible
        assert!(!v1_0_0.is_compatible_with(&v2_0_0));
        assert!(!v1_2_5.is_compatible_with(&v2_0_0));
    }

    #[test]
    fn test_current_version() {
        let current = SaveVersion::current();
        // Should match Cargo.toml version
        assert_eq!(
            current,
            SaveVersion::parse(env!("CARGO_PKG_VERSION")).unwrap()
        );
    }

    #[test]
    fn test_version_equality() {
        let v1 = SaveVersion::new(1, 2, 3);
        let v2 = SaveVersion::new(1, 2, 3);
        let v3 = SaveVersion::new(1, 2, 4);

        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }
}
