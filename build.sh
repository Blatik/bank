#!/bin/bash

# Build Rust backend
echo "🔨 Building Rust backend..."
cd backend
cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ Backend build failed"
    exit 1
fi
cd ..

# Build Rust frontend to WASM
echo "🔨 Building Rust frontend to WASM..."
cd frontend
wasm-pack build --release --target web
if [ $? -ne 0 ]; then
    echo "❌ Frontend build failed"
    exit 1
fi
cd ..

echo "✅ Build completed successfully!"
echo ""
echo "📦 Build artifacts:"
echo "  Backend:  ./backend/target/release/bank-api"
echo "  Frontend: ./frontend/pkg/"
