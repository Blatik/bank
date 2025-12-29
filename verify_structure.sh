#!/bin/bash

# Verify Rust project structure

echo "🔍 Verifying Rust project structure..."
echo ""

check_file() {
    if [ -f "$1" ]; then
        echo "✅ $1"
    else
        echo "❌ $1"
    fi
}

check_dir() {
    if [ -d "$1" ]; then
        echo "✅ $1/"
    else
        echo "❌ $1/"
    fi
}

echo "📁 Checking directory structure..."
check_dir "backend"
check_dir "backend/src"
check_dir "frontend"
check_dir "frontend/src"
check_dir "frontend/src/components"
check_dir "frontend/src/pages"

echo ""
echo "📄 Checking backend files..."
check_file "backend/Cargo.toml"
check_file "backend/src/main.rs"
check_file "backend/src/api.rs"
check_file "backend/src/handlers.rs"
check_file "backend/src/models.rs"
check_file "backend/src/errors.rs"
check_file "backend/src/data.rs"

echo ""
echo "📄 Checking frontend files..."
check_file "frontend/Cargo.toml"
check_file "frontend/src/main.rs"
check_file "frontend/src/models.rs"
check_file "frontend/src/api.rs"
check_file "frontend/src/storage.rs"
check_file "frontend/src/components/mod.rs"
check_file "frontend/src/components/country_selector.rs"
check_file "frontend/src/components/indicator_selector.rs"
check_file "frontend/src/components/chart_viewer.rs"
check_file "frontend/src/pages/mod.rs"
check_file "frontend/src/pages/search_page.rs"
check_file "frontend/src/pages/comparison_page.rs"
check_file "frontend/src/pages/favorites_page.rs"
check_file "frontend/index.html"

echo ""
echo "📄 Checking configuration files..."
check_file "Cargo.toml"
check_file "build.sh"
check_file "run.sh"
check_file "setup_rust.sh"

echo ""
echo "📚 Checking documentation..."
check_file "RUST_README.md"
check_file "RUST_QUICK_START.md"
check_file "MIGRATION_GUIDE.md"
check_file "MIGRATION_SUMMARY.md"

echo ""
echo "✅ Structure verification complete!"
