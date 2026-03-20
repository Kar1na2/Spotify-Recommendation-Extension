#!/usr/bin/env bash
set -e

# Build WASM targeting the web (not Node.js bundlers)
wasm-pack build --target web --out-dir extension/pkg --release

echo "Build complete. Load the 'extension/' folder in chrome://extensions"