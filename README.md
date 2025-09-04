# SatisFlow 🏭

> A blazingly fast web application for tracking and optimizing your Satisfactory factory production chains and logistics flows.

[![Build Status](https://github.com/PixmaNts/SatisFlow/workflows/CI/badge.svg)](https://github.com/PixmaNts/SatisFlow/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://rustup.rs)

## ✨ Features

- **🌐 Web-First**: Runs in any modern browser, no installation required
- **⚡ Near-Native Performance**: Built with Rust and WebAssembly (WASM)
- **📊 Production Overview**: Track all items produced/consumed across your factory network
- **🚛 Logistics Management**: Organize transport routes with proper ID conventions
- **🏗️ Factory Organization**: Manage multiple factories with production line grouping
- **💾 Persistent Storage**: Save your configurations in browser localStorage
- **📱 Progressive Web App**: Install as a native-feeling app from your browser
- **🎨 Modern UI**: Clean, responsive interface with Tailwind CSS

## 🚀 Quick Start

### Try Online
**[🌐 Open SatisFlow](https://pixmanits.github.io/SatisFlow)** *(coming soon)*

### Local Development

1. **Install Rust and Dioxus CLI**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   cargo install dioxus-cli
   ```

2. **Clone and serve**:
   ```bash
   git clone https://github.com/PixmaNts/SatisFlow.git
   cd SatisFlow
   dx serve --platform web
   ```

3. **Open your browser** to `http://localhost:8080`

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

- **Frontend**: [Dioxus](https://dioxuslabs.com/) (React-like for Rust)
- **Runtime**: WebAssembly for near-native performance
- **Styling**: [Tailwind CSS](https://tailwindcss.com/) for responsive design
- **Storage**: Browser localStorage for persistence
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

# Run in development mode with hot reload
dx serve --platform web --hot-reload

# Build for production
dx build --release --platform web
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
