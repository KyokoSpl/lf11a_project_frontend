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
- DEB package (Debian/Ubuntu)
- RPM package (Fedora/RHEL)
- PKGBUILD (Arch Linux)
- AppImage (Universal Linux)
- Windows installer (EXE)

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
