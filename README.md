# SatisFlow

A blazingly fast web application for tracking and optimizing Satisfactory factory production chains.

[![Build Status](https://github.com/PixmaNts/SatisFlow/workflows/CI/badge.svg)](https://github.com/PixmaNts/SatisFlow/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://rustup.rs)

## Features

- **Production Tracking**: Monitor production rates across multiple factories
- **Resource Management**: Identify resource surpluses and shortages
- **Logistics Optimization**: Track material flows between factories  
- **Cloud Storage**: Save your factory configurations (premium feature)
- **File Import/Export**: Local save files for offline usage
- **WASM Performance**: Near-native calculation speed in the browser

## Project Structure

This project uses a Cargo workspace with multiple crates:

```
SatisFlow/
├── Cargo.toml                   # Workspace configuration
├── crates/
│   ├── satisflow-engine/        # Core calculation engine (Rust library)
│   ├── satisflow-wasm/          # WASM bindings for browser integration  
│   └── satisflow-server/        # SaaS backend server (optional)
├── frontend/                    # Vue.js + TypeScript frontend
└── design/                      # Architecture and design documents
```

## Development Setup

### Prerequisites
- Rust 1.70+ with `wasm-pack` installed
- Node.js 18+ with npm
- PostgreSQL (for server development)

### Quick Start

1. **Clone and setup:**
   ```bash
   git clone https://github.com/PixmaNts/SatisFlow.git
   cd SatisFlow
   ```

2. **Build the WASM module:**
   ```bash
   cd crates/satisflow-wasm
   wasm-pack build --target web --out-dir ../../frontend/src/wasm
   ```

3. **Start the frontend:**
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

4. **Optional - Start the backend server:**
   ```bash
   cargo run -p satisflow-server
   ```

## 📖 Usage

### Overview Page
Track your entire production network at a glance:
- See all items being produced and consumed
- Monitor production status (✅ Balanced, ⚠️ Overflow, ❌ Underflow)
- Identify bottlenecks quickly

### Factory Management
Organize your industrial operations:
- Create factories with descriptive names
- Configure raw input sources (miners, extractors)
- Set up production lines with machine counts and clock speeds
- Group related production lines for better organization

### Logistics Planning
Document your transport networks:
- Track logistics flux between factories
- Support for trains, conveyors, trucks, and drones
- Proper ID conventions: `LG-TRN-01-W02`, `LG-BUS-001-03`
- Record transport details and capacities

## 🏗️ Architecture

Built with modern web technologies for optimal performance:

- **Frontend**: Vue 3 + TypeScript (Vite)
- **Engine**: Rust compiled to WebAssembly via `wasm-bindgen`
- **Styling**: Tailwind CSS or scoped CSS
- **Storage**: Browser localStorage for persistence (web); JSON files for desktop
- **Data**: External JSON files for easy recipe management

See [design/architecture.md](design/architecture.md) for detailed technical documentation.

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Clone repository
git clone https://github.com/PixmaNts/SatisFlow.git
cd SatisFlow

# Build WASM bindings
cd crates/satisflow-wasm
wasm-pack build --target web --out-dir ../../frontend/src/wasm

# Start Vue dev server
cd ../../frontend
npm install
npm run dev
```

## 📋 Roadmap

- [ ] **Factory Creation Dialogs**: Easy factory setup with guided UI
- [ ] **Production Line Editor**: Visual recipe selection and configuration
- [ ] **Enhanced Recipe Database**: Complete Satisfactory recipe coverage
- [ ] **Power Consumption Tracking**: Monitor your electrical grid
- [ ] **Blueprint Sharing**: Export and import factory configurations
- [ ] **Production Optimization Engine**: Automated bottleneck detection

See [design/next.md](design/next.md) for the complete roadmap.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Coffee Stain Studios](https://www.coffeestain.com/) for creating the amazing game [Satisfactory](https://satisfactorygame.com/)
- [Dioxus Labs](https://dioxuslabs.com/) for the excellent Rust web framework
- The Rust and WebAssembly communities for making high-performance web applications possible

---

**Built with ❤️ and ☕ by [PixmaNts](https://github.com/PixmaNts)**

*Satisfactory is a trademark of Coffee Stain Studios. This project is not affiliated with Coffee Stain Studios.*
