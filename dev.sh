#!/bin/bash
set -euo pipefail

echo "🚀 Starting SatisFlow Development Environment"
echo "============================================="

# Function to clean up background processes
cleanup() {
    echo -e "\n🧹 Cleaning up background processes..."
    kill $(jobs -p) 2>/dev/null || true
    wait
    echo "✅ Development server stopped"
}

# Set up cleanup trap
trap cleanup EXIT INT TERM

# 1) Build WASM bindings once for development
echo "📦 Building WASM bindings..."
cd crates/satisflow-wasm
if ! wasm-pack build --target web --out-dir ../../frontend/src/wasm; then
    echo "❌ WASM build failed!"
    exit 1
fi
cd ../..

# 2) Install frontend dependencies
echo "📋 Installing frontend dependencies..."
cd frontend
if ! npm install; then
    echo "❌ npm install failed!"
    exit 1
fi

# 3) Start the Vue development server
echo "🎯 Starting Vue dev server (with hot reload)..."
echo ""
echo "🌐 Your SatisFlow app will be available at:"
echo "   → http://localhost:5173"
echo ""
echo "💡 Features available in dev mode:"
echo "   • Hot reload for Vue components"
echo "   • TypeScript checking"
echo "   • Fast refresh"
echo "   • Vue DevTools support"
echo ""
echo "🔧 To rebuild WASM manually (if needed):"
echo "   → npm run build-wasm"
echo ""
echo "⏹️  Press Ctrl+C to stop the development server"
echo "============================================="
echo ""

# Start Vue dev server (this will keep running)
npm run dev

# The cleanup function will be called when the script exits