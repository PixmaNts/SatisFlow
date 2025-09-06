#!/bin/bash
set -euo pipefail

echo "🔧 Rebuilding WASM bindings..."

cd crates/satisflow-wasm
if wasm-pack build --target web --out-dir ../../frontend/src/wasm; then
    echo "✅ WASM rebuild complete!"
    echo "💡 Refresh your browser to see changes"
else
    echo "❌ WASM rebuild failed!"
    exit 1
fi