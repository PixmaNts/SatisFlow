# SatisFlow - Requirements

## Project Overview
A desktop application to help Satisfactory players track their factory production, logistics flows, and resource management across complex industrial setups.

## Core User Stories

### As a Satisfactory player, I want to...

1. **Track Production Overview**
   - See all items being produced/consumed across my entire factory network
   - Monitor production status (balanced, overflow, underflow)
   - Identify bottlenecks and resource shortages quickly

2. **Manage Logistics Networks**
   - Document transport routes between factories
   - Track logistics flux with proper ID system (LG-TRN-XX-WNN, LG-BUS-001-02)
   - Support different transport types (conveyor, train, truck, drone)
   - Record transport details (wagon numbers, conveyor groups)

3. **Organize Factory Operations**  
   - Define multiple factories with descriptive names
   - Configure raw input sources (miners, extractors)
   - Set up production lines with machine counts and clock speeds
   - Group related production lines for better organization

4. **Maintain Accurate Data**
   - Save and load factory configurations
   - Update recipes and items without code changes
   - Handle complex production chains with multiple inputs/outputs

## Functional Requirements

### Overview Page
- Display production summary table with columns:
  - Item name
  - Total produced per minute
  - Total consumed per minute  
  - Available per minute
  - Status indicator (✅ Balanced, ⚠️ Overflow, ❌ Underflow)

### Logistics Management
- Add/edit/delete logistics flux entries
- Searchable item selection with autocomplete
- Support transport type selection with visual icons
- Validate origin and destination factories
- Generate unique logistics IDs automatically

### Factory Management
- Create new factories with custom names
- Configure raw input sources with quantities and comments
- Support multiple raw inputs of the same item type
- Add production lines with:
  - Searchable recipe selection with autocomplete
  - Machine count (1-999)
  - Clock ratio (0.0-2.5)
  - Factory-specific grouping names with dropdown selection
  - Strange Matter booster support
- Edit and delete existing factories
- Pre-selection of current factory in production line creation

### Data Persistence
- Save user configurations to JSON file (clean format without static data)
- Load previous sessions automatically with backward compatibility
- Import/export functionality for sharing setups
- Legacy data migration for older save formats

## Non-Functional Requirements

### Performance
- Instant UI updates for production calculations
- Handle 200+ production lines per factory
- Support 50+ factories without performance degradation

### Usability  
- Clean, intuitive interface following modern UI patterns
- Keyboard shortcuts for power users
- Responsive design that works on different screen sizes

### Maintainability
- External game data files for easy updates
- Modular code structure for feature additions
- Clear separation between UI and business logic
 - JSON Schemas for data validation in CI

### Reliability
- Graceful error handling for invalid data
- Data validation to prevent corruption
- Automatic backup of user configurations
 - Versioned save file format with migrations

## Acceptance Criteria (Phase 1) ✅ COMPLETED

- ✅ Create factory via dialog; persists across reloads
- ✅ Add production line with searchable recipe selection; overview numbers update correctly
- ✅ Add multiple raw inputs of same item type with unique IDs and comments
- ✅ Factory-specific production line grouping with reusable group names
- ✅ Add logistics flux with searchable items; ID auto-generated; validated endpoints
- ✅ Save/Load works with clean JSON format and backward compatibility
- ✅ Context-aware factory preselection in production line creation

## Game-Specific Requirements

### Satisfactory Recipe Support
- All base game recipes included
- Alternative recipes marked clearly  
- Machine type specifications (Constructor, Assembler, etc.)
- Accurate production rates and timing

### ID Conventions
- Logistics flux IDs follow pattern: `LG-{TYPE}-{ID}[-{DETAIL}]`
  - Train: `LG-TRN-01-W02` (Train 01, Wagon 02)
  - Bus: `LG-BUS-001` is the bus identifier; each conveyor on the bus may add a detail like `-C03` (Bus 001, Conveyor 03)
  - Truck: `LG-TRK-05` (Truck route 05)
  - Drone: `LG-DRN-12` (Drone route 12)

### Production Line Organization
- Support for grouping related production lines
- Common groups: "Smelting", "Parts", "Assembly", "Refinement"
- Visual indicators for different production stages

## Technical Constraints

### Platform Support
- Primary: Desktop (Windows, macOS, Linux)
- UI Framework: Dioxus with desktop backend
- No web browser dependencies required

### Data Formats
- User data: JSON for human readability
- Game data: External JSON files for easy modification
- Save files: Pretty-printed JSON for debugging

### External Dependencies
- Minimal dependencies to reduce maintenance burden  
- No network requirements for core functionality
- Standard Rust ecosystem tools only
