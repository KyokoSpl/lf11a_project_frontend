# Windows Build Guide

## Overview

Building the egui-based application for Windows is straightforward since egui uses OpenGL and doesn't require external UI framework libraries like GTK4.

## Cross-Compilation from Linux

### Prerequisites

1. Install the Windows target:
   ```bash
   rustup target add x86_64-pc-windows-gnu
   ```

2. Install MinGW-w64 cross-compiler:
   - **Fedora/RHEL**: `sudo dnf install mingw64-gcc mingw64-gcc-c++`
   - **Debian/Ubuntu**: `sudo apt install gcc-mingw-w64-x86-64 g++-mingw-w64-x86-64`
   - **Arch Linux**: `sudo pacman -S mingw-w64-gcc`

### Building

```bash
# Simple build
cargo build --release --target x86_64-pc-windows-gnu

# Or use the setup script
./scripts/setup_windows.sh

# Or use the full packaging script
./scripts/package.sh
```

### Output

The Windows executable will be at:
```
target/x86_64-pc-windows-gnu/release/lf11a_project_frontend_egui.exe
```

## Native Windows Build

If you prefer to build natively on Windows:

### Prerequisites

1. Install [Rust](https://rustup.rs/)
2. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) or Visual Studio with C++ development tools

### Building

```powershell
# Clone the repository
git clone https://github.com/KyokoSpl/lf11a_project_frontend.git
cd lf11a_project_frontend

# Build
cargo build --release
```

The executable will be at:
```
target\release\lf11a_project_frontend_egui.exe
```

## Distribution

The application is self-contained and doesn't require any runtime dependencies like GTK4. Just distribute the `.exe` file along with the `.env` configuration file.

## Creating an Installer

The `package.sh` script will create an NSIS installer if NSIS is installed:

```bash
# Install NSIS
sudo apt install nsis  # Debian/Ubuntu
sudo dnf install nsis  # Fedora

# Run the packaging script
./scripts/package.sh
```

This creates a Windows installer at:
```
build/lf11a-project-frontend-0.1.0-setup.exe
```
