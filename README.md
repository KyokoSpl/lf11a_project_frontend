# Personnel Management System - egui Frontend with Material 3 Design

A modern personnel management application built with the egui framework featuring Material 3 design principles for excellent cross-platform compatibility.

## Features

- Department management (CRUD operations)
- Employee management (CRUD operations)
- Salary grade management (CRUD operations)
- Relationship tracking between entities
- Material 3 design system with modern color schemes
- Cross-platform support (Linux, Windows, macOS)
- No external runtime dependencies required
- Embedded application icon

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Configuration

Copy `.env.example` to `.env` and configure your API endpoint:

```bash
API_BASE_URL=http://localhost:8082
API_PREFIX=/api
```

**Configuration file location (after installation):**
- Development: `.env` in project root
- Windows Installer: In installation directory (`%LOCALAPPDATA%\LF11A Project Frontend`)
- Windows Portable: In the same directory as the .exe
- Linux packages: Environment variables or create `.env` in working directory

## Cross-Platform Builds

### Windows

```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

### Packaging

Use the packaging script to create distribution packages:

```bash
./scripts/package.sh
```

This creates:
- **DEB package** (Debian/Ubuntu)
- **RPM package** (Fedora/RHEL/openSUSE)
- **PKGBUILD** (Arch Linux)
- **Windows NSIS installer** (Setup.exe)
- **Windows portable** (Standalone .exe)

## Installation

### Debian/Ubuntu

```bash
sudo apt install ./build/lf11a-project-frontend_0.2.0_amd64.deb
```

### Fedora/RHEL/CentOS

```bash
sudo dnf install ./build/lf11a-project-frontend-0.2.0-1.x86_64.rpm
# or
sudo rpm -i ./build/lf11a-project-frontend-0.2.0-1.x86_64.rpm
```

### Arch Linux

```bash
cd build/arch
makepkg -si
```

Or install from the built package:

```bash
sudo pacman -U lf11a-project-frontend-0.2.0-1-x86_64.pkg.tar.zst
```

### Windows

**Option 1: Installer (Recommended)**
- Download `lf11a-project-frontend-0.2.0-setup.exe`
- Double-click to run the installer
- Follow the installation wizard
- Application will be added to Start Menu and Desktop

**Option 2: Portable**
- Download `lf11a-project-frontend.exe`
- No installation required - just run the executable
- Note: Requires Windows 10/11 with OpenGL support

## Material 3 Design

This application follows Material 3 design principles with:
- Dynamic color theming (dark mode)
- Rounded corners and shadow effects
- Consistent spacing and typography
- Modern purple color palette
- Card-based list items

## Project Structure

See [docs/PROJECT-STRUCTURE.md](docs/PROJECT-STRUCTURE.md) for detailed information.

## License

MIT
