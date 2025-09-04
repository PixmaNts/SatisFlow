#!/bin/bash
set -e

echo "🏭 Building SatisFlow for WASM..."

# Install dx if not present
if ! command -v dx &> /dev/null; then
    echo "Installing dioxus-cli..."
    cargo install dioxus-cli
fi

# Build for web
echo "Building WASM..."
dx build --release --platform web

echo "✅ Build complete! Files are in dist/"
echo ""
echo "To serve locally:"
echo "  dx serve --platform web"
echo ""
echo "To deploy, upload the 'dist' folder to your web server."
