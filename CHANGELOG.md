# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Multiple raw inputs support**: Support for multiple raw inputs of the same item type with unique IDs and comments
- **Searchable autocomplete**: Searchable dropdown components for recipes, items, and factory selection across all modals
- **Factory-specific production groups**: Production line grouping system with factory-scoped group management
- **Context-aware factory preselection**: Current factory automatically preselected when adding production lines
- **Raw input comments**: Comment system for documenting raw input sources and locations
- **Legacy data migration**: Automatic migration system for backward compatibility with old save files
- **Clean JSON exports**: Optimized JSON export format excluding static game data (recipes/items)

### Changed
- **Vue 3 + TypeScript frontend**: Migrated from Dioxus to Vue 3 with TypeScript for better developer experience
- **Enhanced UI components**: Replaced basic dropdowns with SearchableSelect component featuring keyboard navigation
- **Improved factory management**: Factory creation and editing with comprehensive validation and error handling
- **Robust data persistence**: Enhanced save/load system with versioned format and automatic migrations

### Fixed
- **Critical import bug**: Fixed game index not being rebuilt after JSON import, preventing recipe validation
- **Raw input ID conflicts**: Resolved issue preventing multiple raw inputs of the same item type
- **Frontend integration**: Fixed WASM bindings and TypeScript integration issues
- **Data validation**: Comprehensive error handling for corrupted or invalid save files

### Technical
- **Rust/WASM engine**: Core calculations performed in Rust compiled to WebAssembly for performance
- **Serde serialization**: Advanced JSON handling with selective field serialization and custom migration logic  
- **Unique ID generation**: Database-style unique ID system for all entities (factories, raw inputs, production lines)
- **Component architecture**: Reusable Vue components with TypeScript interfaces and proper prop validation
- **Game data integration**: Static compilation of game data into Rust code via build.rs for performance

## [0.1.0] - 2025-01-04 (Planning)

### Planning
- Core architecture and requirements definition
- Technology stack selection (Dioxus + WASM)
- Design documentation and roadmap creation
- OSS project setup with proper licensing

---

### Legend
- **Added**: New features
- **Changed**: Changes in existing functionality  
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Vulnerability fixes
- **Technical**: Internal/developer-facing changes