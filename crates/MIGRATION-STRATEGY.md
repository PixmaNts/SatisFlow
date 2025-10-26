# Satisflow Save File Migration Strategy

**Document Version:** 1.0
**Date:** 2025-10-25
**Status:** Design Document

---

## Table of Contents

1. [Overview](#overview)
2. [Versioning Scheme](#versioning-scheme)
3. [Current Save File Format](#current-save-file-format)
4. [Migration Architecture](#migration-architecture)
5. [Migration Examples](#migration-examples)
6. [Implementation Plan](#implementation-plan)
7. [Testing Strategy](#testing-strategy)
8. [Best Practices](#best-practices)

---

## Overview

### Goals

1. **Backward Compatibility**: Old save files should always load in new versions
2. **Forward Detection**: New save files should be detected (and rejected) by old versions
3. **Automatic Migration**: Users shouldn't need to manually migrate files
4. **Data Integrity**: Migrations should never lose data
5. **Rollback Safety**: Users should be able to keep old save files as backups

### Non-Goals

- Forward compatibility (new saves in old versions)
- Manual migration scripts
- Database-style schema migrations

---

## Versioning Scheme

### Semantic Versioning

We use semantic versioning (MAJOR.MINOR.PATCH) with specific meanings:

```
Version: X.Y.Z

X (MAJOR): Breaking changes requiring migration
Y (MINOR): New features with backward-compatible defaults
Z (PATCH): Bug fixes, no schema changes
```

### Version Comparison

```rust
pub struct SaveVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SaveVersion {
    pub fn parse(version: &str) -> Result<Self, String> {
        // Parse "0.1.0" -> SaveVersion { major: 0, minor: 1, patch: 0 }
    }

    pub fn compatible_with(&self, engine_version: &SaveVersion) -> bool {
        // Same major version = compatible
        self.major == engine_version.major
    }

    pub fn needs_migration(&self, engine_version: &SaveVersion) -> bool {
        // Different major or minor = needs migration
        self.major != engine_version.major || self.minor != engine_version.minor
    }
}
```

### Version Strategy

| Change Type | Version Bump | Migration Required | Example |
|-------------|--------------|-------------------|---------|
| Add optional field with default | MINOR | No | Add `blueprint_library: Option<Vec<Blueprint>>` |
| Add required field | MAJOR | Yes | Add `production_group: String` (no default) |
| Rename field | MAJOR | Yes | `quantity_per_min` → `rate_per_minute` |
| Remove field | MAJOR | Yes | Remove deprecated field |
| Change type | MAJOR | Yes | `id: u64` → `id: Uuid` |
| Add new enum variant | MINOR | No | Add `TransportType::Drone` |
| Remove enum variant | MAJOR | Yes | Remove `ExtractorType::MinerMk1` |
| Change default value | PATCH | No | Bug fix in calculation |

---

## Current Save File Format

### Structure (v0.1.0)

```json
{
  "version": "0.1.0",
  "created_at": "2025-10-25T16:55:47.939216Z",
  "last_modified": "2025-10-25T16:55:47.939216Z",
  "game_version": null,
  "engine": {
    "factories": { /* HashMap<FactoryId, Factory> */ },
    "logistics_lines": { /* HashMap<LogisticsId, LogisticsFlux> */ }
  }
}
```

### Schema Documentation

**Top-Level Fields:**
- `version` (string): Save file format version (matches engine crate version)
- `created_at` (datetime): When save was first created
- `last_modified` (datetime): Last modification timestamp
- `game_version` (string?): Optional Satisfactory game version
- `engine` (object): The actual engine state

**Engine Fields:**
- `factories` (map): Factory ID → Factory data
- `logistics_lines` (map): Logistics ID → Logistics data

---

## Migration Architecture

### Design Principles

1. **Chain of Responsibility**: Migrations applied sequentially
2. **Immutable**: Each migration creates new data, doesn't modify in-place
3. **Idempotent**: Running same migration twice is safe
4. **Validated**: Each migration validates before/after
5. **Logged**: All migrations logged for debugging

### Migration Trait

```rust
pub trait Migration {
    /// The version this migration upgrades FROM
    fn from_version(&self) -> SaveVersion;

    /// The version this migration upgrades TO
    fn to_version(&self) -> SaveVersion;

    /// Apply the migration to a JSON value
    fn migrate(&self, data: serde_json::Value) -> Result<serde_json::Value, MigrationError>;

    /// Validate the migration was successful
    fn validate(&self, data: &serde_json::Value) -> Result<(), MigrationError>;

    /// Description of what this migration does
    fn description(&self) -> &'static str;
}
```

### Migration Registry

```rust
pub struct MigrationRegistry {
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationRegistry {
    pub fn new() -> Self {
        Self {
            migrations: vec![
                // Register all migrations in order
                Box::new(Migration_v0_1_to_v0_2::new()),
                Box::new(Migration_v0_2_to_v0_3::new()),
                // Future migrations...
            ],
        }
    }

    /// Find migration path from source to target version
    pub fn find_migration_path(
        &self,
        from: &SaveVersion,
        to: &SaveVersion,
    ) -> Result<Vec<&dyn Migration>, MigrationError> {
        // Find chain of migrations to apply
        let mut path = Vec::new();
        let mut current = from.clone();

        while current != *to {
            let migration = self.migrations.iter()
                .find(|m| m.from_version() == current)
                .ok_or(MigrationError::NoPathFound { from: from.clone(), to: to.clone() })?;

            path.push(migration.as_ref());
            current = migration.to_version();
        }

        Ok(path)
    }

    /// Apply all migrations in path
    pub fn migrate(
        &self,
        from_version: &SaveVersion,
        to_version: &SaveVersion,
        data: serde_json::Value,
    ) -> Result<serde_json::Value, MigrationError> {
        let path = self.find_migration_path(from_version, to_version)?;

        let mut current_data = data;
        for migration in path {
            println!("Applying migration: {}", migration.description());
            current_data = migration.migrate(current_data)?;
            migration.validate(&current_data)?;
        }

        Ok(current_data)
    }
}
```

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("No migration path found from {from} to {to}")]
    NoPathFound {
        from: SaveVersion,
        to: SaveVersion,
    },

    #[error("Migration failed: {message}")]
    MigrationFailed { message: String },

    #[error("Validation failed: {message}")]
    ValidationFailed { message: String },

    #[error("Unsupported save version: {version}")]
    UnsupportedVersion { version: String },

    #[error("Save file too new: file is {file_version}, engine is {engine_version}")]
    SaveTooNew {
        file_version: String,
        engine_version: String,
    },

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
```

---

## Migration Examples

### Example 1: Add Optional Field (v0.1.0 → v0.2.0)

**Change:** Add `production_group: Option<String>` to ProductionLine

**Migration:**
```rust
pub struct Migration_v0_1_to_v0_2;

impl Migration for Migration_v0_1_to_v0_2 {
    fn from_version(&self) -> SaveVersion {
        SaveVersion::new(0, 1, 0)
    }

    fn to_version(&self) -> SaveVersion {
        SaveVersion::new(0, 2, 0)
    }

    fn migrate(&self, mut data: serde_json::Value) -> Result<serde_json::Value, MigrationError> {
        // No actual migration needed - serde will use default value (None)
        // Just update version
        data["version"] = json!("0.2.0");
        data["last_modified"] = json!(Utc::now());
        Ok(data)
    }

    fn validate(&self, data: &serde_json::Value) -> Result<(), MigrationError> {
        // Verify version was updated
        let version = data["version"].as_str()
            .ok_or(MigrationError::ValidationFailed {
                message: "Missing version field".to_string(),
            })?;

        if version != "0.2.0" {
            return Err(MigrationError::ValidationFailed {
                message: format!("Expected version 0.2.0, got {}", version),
            });
        }

        Ok(())
    }

    fn description(&self) -> &'static str {
        "Add optional production_group field to ProductionLine"
    }
}
```

### Example 2: Rename Field (v0.2.0 → v0.3.0)

**Change:** Rename `quantity_per_min` to `rate_per_minute` in RawInput

**Migration:**
```rust
pub struct Migration_v0_2_to_v0_3;

impl Migration for Migration_v0_2_to_v0_3 {
    fn from_version(&self) -> SaveVersion {
        SaveVersion::new(0, 2, 0)
    }

    fn to_version(&self) -> SaveVersion {
        SaveVersion::new(0, 3, 0)
    }

    fn migrate(&self, mut data: serde_json::Value) -> Result<serde_json::Value, MigrationError> {
        // Navigate to raw_inputs in each factory
        if let Some(factories) = data["engine"]["factories"].as_object_mut() {
            for (_, factory) in factories.iter_mut() {
                if let Some(raw_inputs) = factory["raw_inputs"].as_object_mut() {
                    for (_, raw_input) in raw_inputs.iter_mut() {
                        // Rename field
                        if let Some(quantity) = raw_input.get_mut("quantity_per_min") {
                            let value = quantity.take();
                            raw_input["rate_per_minute"] = value;
                        }
                    }
                }
            }
        }

        // Update version
        data["version"] = json!("0.3.0");
        data["last_modified"] = json!(Utc::now());

        Ok(data)
    }

    fn validate(&self, data: &serde_json::Value) -> Result<(), MigrationError> {
        // Verify no old field names remain
        if let Some(factories) = data["engine"]["factories"].as_object() {
            for (factory_id, factory) in factories.iter() {
                if let Some(raw_inputs) = factory["raw_inputs"].as_object() {
                    for (input_id, raw_input) in raw_inputs.iter() {
                        if raw_input.get("quantity_per_min").is_some() {
                            return Err(MigrationError::ValidationFailed {
                                message: format!(
                                    "Found old field 'quantity_per_min' in factory {} raw_input {}",
                                    factory_id, input_id
                                ),
                            });
                        }

                        if raw_input.get("rate_per_minute").is_none() {
                            return Err(MigrationError::ValidationFailed {
                                message: format!(
                                    "Missing new field 'rate_per_minute' in factory {} raw_input {}",
                                    factory_id, input_id
                                ),
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn description(&self) -> &'static str {
        "Rename quantity_per_min to rate_per_minute in RawInput"
    }
}
```

### Example 3: Add Blueprint Library (v0.3.0 → v1.0.0)

**Change:** Add `blueprint_library: Vec<ProductionLineBlueprint>` to engine

**Migration:**
```rust
pub struct Migration_v0_3_to_v1_0;

impl Migration for Migration_v0_3_to_v1_0 {
    fn from_version(&self) -> SaveVersion {
        SaveVersion::new(0, 3, 0)
    }

    fn to_version(&self) -> SaveVersion {
        SaveVersion::new(1, 0, 0)
    }

    fn migrate(&self, mut data: serde_json::Value) -> Result<serde_json::Value, MigrationError> {
        // Add empty blueprint library
        data["engine"]["blueprint_library"] = json!([]);

        // Extract all existing blueprints from factories and add to library
        let mut blueprints = Vec::new();

        if let Some(factories) = data["engine"]["factories"].as_object() {
            for (_, factory) in factories.iter() {
                if let Some(production_lines) = factory["production_lines"].as_object() {
                    for (_, line) in production_lines.iter() {
                        if let Some(blueprint) = line.get("ProductionLineBlueprint") {
                            blueprints.push(blueprint.clone());
                        }
                    }
                }
            }
        }

        // Add extracted blueprints to library (deduplicated by ID)
        data["engine"]["blueprint_library"] = json!(blueprints);

        // Update version (MAJOR bump)
        data["version"] = json!("1.0.0");
        data["last_modified"] = json!(Utc::now());

        Ok(data)
    }

    fn validate(&self, data: &serde_json::Value) -> Result<(), MigrationError> {
        // Verify blueprint_library exists
        if data["engine"]["blueprint_library"].is_null() {
            return Err(MigrationError::ValidationFailed {
                message: "Missing blueprint_library field".to_string(),
            });
        }

        // Verify it's an array
        if !data["engine"]["blueprint_library"].is_array() {
            return Err(MigrationError::ValidationFailed {
                message: "blueprint_library must be an array".to_string(),
            });
        }

        Ok(())
    }

    fn description(&self) -> &'static str {
        "Add blueprint library to engine (v1.0.0 - blueprints feature)"
    }
}
```

### Example 4: Change ID Type (v1.0.0 → v2.0.0)

**Change:** Change RawInput ID from `u64` to `Uuid`

**Migration:**
```rust
pub struct Migration_v1_0_to_v2_0;

impl Migration for Migration_v1_0_to_v2_0 {
    fn from_version(&self) -> SaveVersion {
        SaveVersion::new(1, 0, 0)
    }

    fn to_version(&self) -> SaveVersion {
        SaveVersion::new(2, 0, 0)
    }

    fn migrate(&self, mut data: serde_json::Value) -> Result<serde_json::Value, MigrationError> {
        // Navigate to raw_inputs in each factory
        if let Some(factories) = data["engine"]["factories"].as_object_mut() {
            for (_, factory) in factories.iter_mut() {
                if let Some(raw_inputs) = factory["raw_inputs"].as_object() {
                    // Create new map with UUID keys
                    let mut new_raw_inputs = serde_json::Map::new();

                    for (old_id, raw_input_data) in raw_inputs.iter() {
                        // Generate UUID from u64
                        let uuid = self.u64_to_uuid(old_id.parse::<u64>().unwrap());
                        let uuid_str = uuid.to_string();

                        // Update the id field in the data
                        let mut new_data = raw_input_data.clone();
                        new_data["id"] = json!(uuid_str);

                        new_raw_inputs.insert(uuid_str, new_data);
                    }

                    factory["raw_inputs"] = json!(new_raw_inputs);
                }
            }
        }

        // Update version (MAJOR bump)
        data["version"] = json!("2.0.0");
        data["last_modified"] = json!(Utc::now());

        Ok(data)
    }

    fn validate(&self, data: &serde_json::Value) -> Result<(), MigrationError> {
        // Verify all raw_input IDs are valid UUIDs
        if let Some(factories) = data["engine"]["factories"].as_object() {
            for (factory_id, factory) in factories.iter() {
                if let Some(raw_inputs) = factory["raw_inputs"].as_object() {
                    for (input_id, raw_input) in raw_inputs.iter() {
                        // Verify ID is valid UUID
                        if Uuid::parse_str(input_id).is_err() {
                            return Err(MigrationError::ValidationFailed {
                                message: format!(
                                    "Invalid UUID for raw_input in factory {}: {}",
                                    factory_id, input_id
                                ),
                            });
                        }

                        // Verify id field matches key
                        let id_field = raw_input["id"].as_str()
                            .ok_or(MigrationError::ValidationFailed {
                                message: format!("Missing id field in raw_input"),
                            })?;

                        if id_field != input_id {
                            return Err(MigrationError::ValidationFailed {
                                message: format!(
                                    "ID mismatch: key={}, field={}",
                                    input_id, id_field
                                ),
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn description(&self) -> &'static str {
        "Change RawInput ID from u64 to Uuid"
    }

    fn u64_to_uuid(&self, value: u64) -> Uuid {
        Uuid::from_u128(value as u128)
    }
}
```

---

## Implementation Plan

### Phase 1: Core Migration System

**Files to Create:**
```
crates/satisflow-engine/src/
├── migrations/
│   ├── mod.rs              # Main migration module
│   ├── version.rs          # SaveVersion struct
│   ├── registry.rs         # MigrationRegistry
│   ├── error.rs            # MigrationError
│   └── migrations/         # Individual migrations
│       ├── mod.rs
│       └── v0_1_to_v0_2.rs (future)
```

**Implementation Steps:**

1. **Create migration module structure**
   ```rust
   // src/migrations/mod.rs
   pub mod version;
   pub mod registry;
   pub mod error;
   pub mod migrations;

   pub use version::SaveVersion;
   pub use registry::MigrationRegistry;
   pub use error::MigrationError;
   ```

2. **Implement SaveVersion**
   - Parse version strings
   - Compare versions
   - Check compatibility

3. **Implement Migration trait**
   - Define interface
   - Add helper methods

4. **Implement MigrationRegistry**
   - Find migration paths
   - Apply migrations
   - Validate results

5. **Update SaveFile**
   ```rust
   impl SaveFile {
       pub fn load_with_migration(json: &str) -> Result<Self, MigrationError> {
           // Parse JSON
           let value: serde_json::Value = serde_json::from_str(json)?;

           // Get version
           let file_version = SaveVersion::parse(
               value["version"].as_str().ok_or(...)?
           )?;

           // Get current engine version
           let engine_version = SaveVersion::parse(env!("CARGO_PKG_VERSION"))?;

           // Check if migration needed
           if file_version == engine_version {
               // No migration needed
               return Ok(serde_json::from_value(value)?);
           }

           if file_version > engine_version {
               return Err(MigrationError::SaveTooNew {
                   file_version: file_version.to_string(),
                   engine_version: engine_version.to_string(),
               });
           }

           // Apply migrations
           let registry = MigrationRegistry::new();
           let migrated = registry.migrate(&file_version, &engine_version, value)?;

           // Deserialize migrated data
           Ok(serde_json::from_value(migrated)?)
       }
   }
   ```

6. **Update load methods**
   ```rust
   impl SatisflowEngine {
       pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
           let json = std::fs::read_to_string(path)?;
           let save_file = SaveFile::load_with_migration(&json)?;
           Ok(save_file.engine)
       }

       pub fn load_from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
           let save_file = SaveFile::load_with_migration(json)?;
           Ok(save_file.engine)
       }
   }
   ```

### Phase 2: Testing Infrastructure

**Test Files:**
```
crates/satisflow-engine/tests/
└── migrations/
    ├── mod.rs
    ├── test_data/
    │   ├── v0_1_0_empty.json
    │   ├── v0_1_0_complex.json
    │   └── v0_2_0_complex.json (future)
    └── migration_tests.rs
```

**Test Strategy:**
1. Create reference save files for each version
2. Test loading old versions in new engine
3. Test migration validation
4. Test error cases (corrupted data, missing fields)
5. Test migration chains (v0.1 → v0.2 → v1.0)

### Phase 3: User-Facing Features

**Features:**
1. **Migration report:**
   ```rust
   pub struct MigrationReport {
       pub from_version: String,
       pub to_version: String,
       pub migrations_applied: Vec<String>,
       pub warnings: Vec<String>,
       pub duration_ms: u64,
   }
   ```

2. **Backup on migration:**
   ```rust
   impl SaveFile {
       pub fn load_with_backup(path: &Path) -> Result<Self, MigrationError> {
           // If migration needed, create backup first
           let json = std::fs::read_to_string(path)?;
           let value: serde_json::Value = serde_json::from_str(&json)?;

           let file_version = SaveVersion::parse(...)?;
           let engine_version = SaveVersion::parse(...)?;

           if file_version < engine_version {
               // Create backup
               let backup_path = path.with_extension("json.backup");
               std::fs::copy(path, backup_path)?;
           }

           SaveFile::load_with_migration(&json)
       }
   }
   ```

3. **CLI migration command:**
   ```bash
   save_load_demo migrate old_save.json --output migrated_save.json --report
   ```

---

## Testing Strategy

### Unit Tests

```rust
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
    fn test_version_compatibility() {
        let v1 = SaveVersion::new(1, 0, 0);
        let v2 = SaveVersion::new(1, 1, 0);
        let v3 = SaveVersion::new(2, 0, 0);

        assert!(v1.compatible_with(&v2));
        assert!(!v1.compatible_with(&v3));
    }

    #[test]
    fn test_migration_path() {
        let registry = MigrationRegistry::new();
        let from = SaveVersion::new(0, 1, 0);
        let to = SaveVersion::new(0, 3, 0);

        let path = registry.find_migration_path(&from, &to).unwrap();
        assert_eq!(path.len(), 2); // v0.1→v0.2, v0.2→v0.3
    }
}
```

### Integration Tests

```rust
#[test]
fn test_load_v0_1_0_save_in_v0_2_0() {
    let v0_1_json = include_str!("../test_data/v0_1_0_complex.json");
    let engine = SatisflowEngine::load_from_json(v0_1_json).unwrap();

    // Verify data was migrated correctly
    assert_eq!(engine.get_all_factories().len(), 2);
    // ... more assertions
}

#[test]
fn test_migration_chain() {
    // Start with v0.1.0 save
    let v0_1_json = include_str!("../test_data/v0_1_0_complex.json");

    // Migrate to v1.0.0 (skipping intermediate versions)
    let engine = SatisflowEngine::load_from_json(v0_1_json).unwrap();

    // Save in new format
    let v1_0_json = engine.save_to_json().unwrap();

    // Verify new version
    let save_file: SaveFile = serde_json::from_str(&v1_0_json).unwrap();
    assert_eq!(save_file.version, "1.0.0");
}
```

---

## Best Practices

### DO:

✅ **Always add optional fields with defaults**
```rust
#[derive(Serialize, Deserialize)]
pub struct ProductionLine {
    pub id: ProductionLineId,
    pub name: String,

    #[serde(default)]
    pub production_group: Option<String>,  // NEW: Optional with default
}
```

✅ **Use serde defaults for new fields**
```rust
#[serde(default = "default_blueprint_library")]
pub blueprint_library: Vec<ProductionLineBlueprint>,

fn default_blueprint_library() -> Vec<ProductionLineBlueprint> {
    Vec::new()
}
```

✅ **Version migrations, not data structures**
- Don't add `#[deprecated]` to struct fields
- Create migrations to handle old data

✅ **Test migrations with real data**
- Save actual test data files
- Don't generate test data programmatically

✅ **Document breaking changes**
- Update CHANGELOG.md
- Mention migration in release notes

### DON'T:

❌ **Don't remove fields without migration**
```rust
// BAD: Will fail to load old saves
pub struct RawInput {
    pub id: RawInputId,
    // pub old_field: String,  // REMOVED - breaks old saves!
}
```

❌ **Don't change field types without migration**
```rust
// BAD: Breaks deserialization
pub struct Factory {
    pub id: Uuid,  // Was: u64
}
```

❌ **Don't make optional fields required**
```rust
// BAD: Old saves won't have this field
pub struct ProductionLine {
    pub production_group: String,  // Was: Option<String>
}
```

❌ **Don't skip version numbers**
- Always increment sequentially
- 0.1.0 → 0.2.0 → 0.3.0 (not 0.1.0 → 0.3.0)

---

## Future Enhancements

### 1. Migration Dry Run

```rust
impl MigrationRegistry {
    pub fn dry_run(
        &self,
        from: &SaveVersion,
        to: &SaveVersion,
        data: &serde_json::Value,
    ) -> Result<MigrationReport, MigrationError> {
        // Show what would happen without actually migrating
    }
}
```

### 2. Partial Migrations

For very large save files, migrate in chunks:
```rust
pub fn migrate_streaming(
    &self,
    reader: impl Read,
    writer: impl Write,
) -> Result<(), MigrationError>
```

### 3. Custom Migration Hooks

Allow users to provide custom migration logic:
```rust
pub trait CustomMigration {
    fn before_migrate(&self, data: &mut serde_json::Value);
    fn after_migrate(&self, data: &mut serde_json::Value);
}
```

---

## Conclusion

This migration strategy provides:
- ✅ Automatic migration of old save files
- ✅ Safe handling of version incompatibilities
- ✅ Extensible system for future changes
- ✅ Data integrity guarantees
- ✅ User-friendly error messages

The key is to **always think about migration when making schema changes** and to maintain a registry of all migrations for testing and validation.

---

**Document End**
