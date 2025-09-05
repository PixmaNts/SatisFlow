#!/bin/bash
set -euo pipefail

echo "🏭 Building SatisFlow (WASM + Vue) ..."

# 1) Build WASM bindings
pushd crates/satisflow-wasm >/dev/null
echo "➡️  Building WASM bindings with wasm-pack"
wasm-pack build --target web --out-dir ../../frontend/src/wasm
popd >/dev/null

# 2) Build frontend with Vite
pushd frontend >/dev/null
echo "➡️  Installing frontend deps"
npm install
echo "➡️  Building frontend"
npm run build
popd >/dev/null

echo "✅ Build complete! Files are in frontend/dist/"
echo "To serve locally:"
echo "  (in frontend/) npm run dev"
