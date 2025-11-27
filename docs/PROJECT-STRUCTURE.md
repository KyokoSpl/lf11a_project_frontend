# Project Structure

This document describes the organization of the LF11A Project Frontend codebase.

## Directory Layout

```
lf11a_project_frontend/
├── assets/               # Application assets
│   ├── icon.svg          # Application icon (SVG format)
│   └── style.css         # GTK CSS styling
│
├── docs/                 # Documentation
│   ├── CONFIGURATION.md      # Configuration guide
│   ├── PACKAGING.md          # Package building instructions
│   ├── PROJECT-STRUCTURE.md  # This file
│   ├── QUICKSTART.md         # Quick start guide
│   └── WINDOWS-INSTALLER.md  # Windows installer documentation
│
├── scripts/              # Build and packaging scripts
│   ├── package.sh        # Multi-platform package creation
│   └── quick-build.sh    # Quick development build
│
├── src/                  # Source code
│   ├── main.rs           # Application entry point
│   ├── mod.rs            # Module declarations
│   ├── config.rs         # Configuration management
│   │
│   ├── api/              # Backend API client
│   │   ├── mod.rs        # API module
│   │   ├── client.rs     # HTTP client implementation
│   │   └── models.rs     # Data models
│   │
│   └── gui/              # User interface
│       ├── mod.rs        # GUI module
│       ├── api.rs        # GUI API integration
│       ├── main_window.rs    # Main application window
│       ├── models.rs     # GUI data models
│       │
│       ├── views/        # Tab views for main window
│       │   ├── mod.rs
│       │   ├── department_view.rs   # Department management
│       │   ├── employee_view.rs     # Employee management
│       │   └── salary_grade_view.rs # Salary grade management
│       │
│       └── dialogs/      # CRUD dialogs
│           ├── mod.rs
│           ├── department_dialog.rs
│           ├── employee_dialog.rs
│           └── salary_grade_dialog.rs
│
├── target/               # Build artifacts (ignored by git)
│
├── .env                  # Environment configuration (ignored by git)
├── .env.example          # Configuration template
├── .gitignore            # Git ignore rules
├── Cargo.toml            # Rust project manifest
├── Cargo.lock            # Dependency lock file
├── LICENSE               # MIT License
└── README.md             # Main project documentation
```

## Key Components

### Source Code (`src/`)
- **main.rs**: Application entry point, initializes GTK and loads configuration
- **config.rs**: Configuration management using environment variables
- **api/**: Backend API client with HTTP communication
- **gui/**: GTK4-based user interface components

### Assets (`assets/`)
- **icon.svg**: Application icon in SVG format (converted to PNG/ICO during packaging)
- **style.css**: GTK CSS for custom styling (Material Design inspired)

### Documentation (`docs/`)
All project documentation is centralized here for easy access:
- Configuration guide
- Packaging instructions
- Quick start guide
- Windows installer documentation
- Project structure (this file)

### Scripts (`scripts/`)
Build and packaging automation:
- **package.sh**: Creates DEB, RPM, Arch, AppImage, and Windows packages
- **quick-build.sh**: Quick release build for development

## Module Organization

### API Module (`src/api/`)
```rust
pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}
```
Handles all HTTP communication with the backend API.

### GUI Module (`src/gui/`)
Organized into:
- **views/**: Main tab views (departments, employees, salary grades)
- **dialogs/**: Create/edit dialogs for CRUD operations
- **main_window.rs**: Application window with tab layout

### Configuration (`src/config.rs`)
Singleton configuration loaded from `.env` file:
```rust
Config::get().api_url()  // Access configuration
```

## File Naming Conventions

- **Rust files**: snake_case (e.g., `main_window.rs`)
- **Directories**: lowercase (e.g., `docs/`, `scripts/`)
- **Documentation**: UPPERCASE.md (e.g., `README.md`, `CONFIGURATION.md`)
- **Assets**: lowercase (e.g., `icon.svg`, `style.css`)

## Build Artifacts

### Development Build
```bash
cargo build
# Binary: target/debug/lf11a_project_frontend
```

### Release Build
```bash
cargo build --release
# Binary: target/release/lf11a_project_frontend
```

### Packages
Created by `./scripts/package.sh` in `build/` directory:
- `*.deb` - Debian/Ubuntu packages
- `*.rpm` - Fedora/RHEL/openSUSE packages
- `PKGBUILD` - Arch Linux package
- `*.AppImage` - Universal Linux binary
- `*-setup.exe` - Windows installer

## Path References

All path references in the code use relative paths from the project root:
- CSS: `assets/style.css`
- Icon: `assets/icon.svg`
- Documentation: `docs/*.md`
- Scripts: `scripts/*.sh`

## Best Practices

1. **Configuration**: Use `.env` file for all configurable values
2. **Assets**: Store all images, icons, and CSS in `assets/`
3. **Documentation**: Add new documentation to `docs/`
4. **Scripts**: Add build/packaging scripts to `scripts/`
5. **Git**: Never commit `.env`, build artifacts, or binaries

## Development Workflow

1. Edit source code in `src/`
2. Update assets in `assets/` if needed
3. Test with `cargo build && cargo run`
4. Update documentation in `docs/` if needed
5. Build packages with `./scripts/package.sh`

## References

- Main README: `README.md`
- Configuration: `docs/CONFIGURATION.md`
- Packaging: `docs/PACKAGING.md`
- Quick Start: `docs/QUICKSTART.md`
