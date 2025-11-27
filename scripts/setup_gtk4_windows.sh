#!/bin/bash
# Setup GTK4 Windows libraries for cross-compilation
# This script downloads pre-built GTK4 Windows binaries

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
GTK_DIR="$PROJECT_ROOT/gtk4-windows"
GTK_VERSION="4.12.5"

echo "=== GTK4 Windows Cross-Compilation Setup ==="
echo ""

# Check if already set up
if [ -d "$GTK_DIR" ] && [ -f "$GTK_DIR/.setup_complete" ]; then
    echo "GTK4 Windows libraries already installed at: $GTK_DIR"
    echo "To reinstall, delete the directory and run again."
    exit 0
fi

echo "Note: This will download ~100MB of GTK4 Windows binaries"
echo ""

# Create directory
mkdir -p "$GTK_DIR"
cd "$GTK_DIR"

# Download pre-built GTK4 from gvsbuild releases
echo "Downloading GTK4 Windows binaries..."
GTK_URL="https://github.com/wingtk/gvsbuild/releases/download/2024.11.0/GTK4_Gvsbuild_2024.11.0_x64.zip"

if ! command -v wget &> /dev/null && ! command -v curl &> /dev/null; then
    echo "Error: Neither wget nor curl found. Please install one of them:"
    echo "  sudo dnf install wget"
    exit 1
fi

if command -v wget &> /dev/null; then
    wget -O gtk4.zip "$GTK_URL"
elif command -v curl &> /dev/null; then
    curl -L -o gtk4.zip "$GTK_URL"
fi

echo "Extracting GTK4 Windows binaries..."
unzip -q gtk4.zip
rm gtk4.zip

# The extracted files should be in a 'gtk' subdirectory or similar
# Let's verify the structure
echo ""
echo "Extracted contents:"
ls -la

# Fix pkg-config files to use correct paths
echo ""
echo "Fixing pkg-config file paths..."
GTK_UNIX_PATH="$(cd "$GTK_DIR" && pwd)"

# Replace Windows paths with Unix paths in all .pc files
find "$GTK_DIR" -name "*.pc" -type f -exec sed -i \
    -e "s|C:/gtk-build/gtk/x64/release|$GTK_UNIX_PATH|g" \
    -e "s|C:\\\\gtk-build\\\\gtk\\\\x64\\\\release|$GTK_UNIX_PATH|g" \
    -e "s|prefix=.*|prefix=$GTK_UNIX_PATH|" \
    {} \;

echo "Fixed $(find "$GTK_DIR" -name "*.pc" | wc -l) pkg-config files"

# Create pkg-config wrapper for cross-compilation
echo ""
echo "Creating pkg-config configuration..."

# Set up environment variables file
cat > "$GTK_DIR/gtk4-env.sh" << 'EOF'
# Source this file before cross-compiling
export GTK_WINDOWS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export PKG_CONFIG_PATH="$GTK_WINDOWS_DIR/lib/pkgconfig"
export PKG_CONFIG_SYSROOT_DIR="$GTK_WINDOWS_DIR"
export PKG_CONFIG="x86_64-w64-mingw32-pkg-config"
export PKG_CONFIG_ALLOW_CROSS=1

echo "GTK4 Windows environment configured:"
echo "  PKG_CONFIG_PATH=$PKG_CONFIG_PATH"
echo "  PKG_CONFIG_SYSROOT_DIR=$PKG_CONFIG_SYSROOT_DIR"
echo "  PKG_CONFIG=$PKG_CONFIG"
EOF

chmod +x "$GTK_DIR/gtk4-env.sh"

# Create marker file
touch "$GTK_DIR/.setup_complete"

echo ""
echo "=== Setup Complete ==="
echo ""
echo "GTK4 Windows libraries installed to: $GTK_DIR"
echo ""
echo "To use for cross-compilation:"
echo "  source $GTK_DIR/gtk4-env.sh"
echo "  cargo build --target x86_64-pc-windows-gnu --release"
echo ""
echo "Or use the package script which will do this automatically:"
echo "  ./scripts/package.sh"
