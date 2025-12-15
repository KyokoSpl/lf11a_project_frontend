#!/usr/bin/env bash

# Quick Package Build Script
# This is a simplified version that builds only the most common formats

set -e

echo "🚀 Quick Package Builder"
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Run this script from the project root directory"
    exit 1
fi

# Build release binary
echo "📦 Building release binary..."
cargo build --release

echo ""
echo "✅ Build complete!"
echo ""
echo "📍 Binary location: target/release/lf11a_project_frontend_egui"
echo ""
echo "To create distribution packages, run: ./scripts/package.sh"
echo ""
echo "Quick test:"
echo "  ./target/release/lf11a_project_frontend_egui"
echo ""
