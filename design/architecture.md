# SatisFlow - Architecture

## Overview
SatisFlow is a **web application** built with Dioxus (Rust) and compiled to WebAssembly (WASM) that helps players track and optimize their factory production chains and logistics flows. It runs directly in web browsers with near-native performance.

## Architecture Principles
- **Separation of Concerns**: Clear boundaries between UI, business logic, and data
- **External Configuration**: Game data stored in editable JSON files
- **Modular Design**: Components can be developed and tested independently
- **Reactive UI**: State-driven interface that updates automatically

## Module Structure

### `/src/main.rs`
- Application entry point
- Main App component with navigation and state management
- Demo data initialization

### `/src/models/`
Core data structures representing game entities:
- **`factory.rs`**: Factory, ProductionLine, RawInput, ItemSummary
- **`logistics.rs`**: LogisticsFlux, TransportType
- **`recipe.rs`**: Recipe, Item, RecipeIngredient

### `/src/gui/`
UI components for each main view:
- **`overview.rs`**: Production summary table with status indicators
- **`logistics.rs`**: Logistics flux management interface
- **`factory.rs`**: Factory and production line management

### `/src/engine/`
Business logic and data processing:
- **`calculator.rs`**: ProductionTracker with production calculations
- **`persistence.rs`**: Save/load functionality for user data

### `/src/data/`
External data management:
- **`mod.rs`**: JSON file loading for items and recipes

## Data Flow

```
JSON Files (HTTP) → Async Data Loader → ProductionTracker → GUI Components
                                              ↕
                                    Browser localStorage
```

1. **Initialization**: Fetch game data via HTTP from public/ directory
2. **User Input**: UI components update tracker state via Dioxus signals
3. **Calculation**: Engine recalculates production summaries
4. **Rendering**: UI displays updated state with reactive updates
5. **Persistence**: Save user data to browser localStorage

## State Management
- **Dioxus Signals**: Reactive state for UI updates
- **ProductionTracker**: Central data store for all game entities
- **Immutable Operations**: State changes through well-defined methods

## External Data Format

### `/data/items.json`
```json
{
  "items": [
    {
      "name": "Iron Ore",
      "category": "Raw Material", 
      "description": "Basic iron ore..."
    }
  ]
}
```

### `/data/recipes.json`
```json
{
  "recipes": [
    {
      "name": "Iron Ingot",
      "machine_type": "Smelter",
      "base_duration": 2.0,
      "inputs": [{"item": "Iron Ore", "quantity_per_min": 30.0}],
      "outputs": [{"item": "Iron Ingot", "quantity_per_min": 30.0}]
    }
  ]
}
```

## Technology Stack
- **Frontend**: Dioxus 0.6 with web/WASM target
- **Runtime**: WebAssembly (WASM) for near-native performance
- **Styling**: Tailwind CSS classes (CDN-based)
- **HTTP Client**: gloo-net for fetching game data
- **Storage**: Browser localStorage API
- **Serialization**: Serde with JSON
- **Language**: Rust (edition 2024)

## Build System & Deployment
- **Build Tool**: Dioxus CLI (`dx`) for WASM compilation
- **Target**: `wasm32-unknown-unknown`
- **Deployment**: Static files (WASM + JS + HTML)
- **Hosting**: Any static file server (GitHub Pages, Netlify, etc.)
- **PWA Support**: Manifest.json for installable web app

## Performance Characteristics
- **Bundle Size**: ~500KB WASM + minimal JS glue
- **Load Time**: ~2-3 seconds on first visit, <1s cached
- **Runtime Performance**: Near-native speed for calculations
- **Memory Usage**: ~10-20MB typical usage
- **Offline Capability**: Full functionality after initial load
