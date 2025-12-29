#!/bin/bash

# Setup script for Rust version

set -e

echo "🦀 Setting up Rust World Bank Analyzer..."
echo ""

# Check for Rust
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed"
    echo "Install from: https://rustup.rs/"
    exit 1
fi

echo "✅ Rust found: $(rustc --version)"

# Check for wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    echo ""
    echo "⚠️  wasm-pack is not installed"
    echo "Installing wasm-pack..."
    cargo install wasm-pack
fi

echo "✅ wasm-pack found: $(wasm-pack --version)"

# Update Rust
echo ""
echo "🔄 Updating Rust toolchain..."
rustup update

# Install wasm target
echo ""
echo "🔄 Installing WASM target..."
rustup target add wasm32-unknown-unknown

# Create directories if they don't exist
mkdir -p backend/src
mkdir -p frontend/src/{components,pages}

echo ""
echo "✅ Setup completed!"
echo ""
echo "Next steps:"
echo "  1. Build:  ./build.sh"
echo "  2. Run:    ./run.sh"
echo ""
echo "For more information, see RUST_README.md"
