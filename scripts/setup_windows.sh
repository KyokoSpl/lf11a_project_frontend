#!/bin/bash
# Windows cross-compilation setup for egui-based application
# Note: Since egui doesn't require GTK4, this script just helps set up
# the basic Windows cross-compilation environment

set -e

echo "=== Windows Cross-Compilation Setup for egui ==="
echo ""

# Check if Windows target is installed
if ! rustup target list | grep -q "x86_64-pc-windows-gnu (installed)"; then
    echo "Installing Windows target..."
    rustup target add x86_64-pc-windows-gnu
    echo "✓ Windows target installed"
else
    echo "✓ Windows target already installed"
fi

# Check for MinGW cross-compiler
if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "✓ MinGW-w64 cross-compiler found"
else
    echo ""
    echo "MinGW-w64 cross-compiler not found."
    echo "Install it with:"
    echo "  Fedora/RHEL: sudo dnf install mingw64-gcc mingw64-gcc-c++"
    echo "  Debian/Ubuntu: sudo apt install gcc-mingw-w64-x86-64 g++-mingw-w64-x86-64"
    echo "  Arch Linux: sudo pacman -S mingw-w64-gcc"
    echo ""
    exit 1
fi

echo ""
echo "=== Setup Complete ==="
echo ""
echo "You can now build for Windows with:"
echo "  cargo build --release --target x86_64-pc-windows-gnu"
echo ""
echo "Or use the packaging script:"
echo "  ./scripts/package.sh"
