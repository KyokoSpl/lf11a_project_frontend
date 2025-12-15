# Project Structure# Project Structure# Project Structure



This document describes the organization of the LF11A Project Frontend codebase.



## Directory LayoutThis document describes the organization of the LF11A Project Frontend codebase.This document describes the organization of the LF11A Project Frontend codebase.



```

lf11a_project_frontend/

├── assets/                   # Application assets## Directory Layout## Directory Layout

│   └── icon.svg              # Application icon (SVG format)

│

├── docs/                     # Documentation

│   ├── CONFIGURATION.md      # Configuration guide``````

│   ├── PACKAGING.md          # Package building instructions

│   ├── PROJECT-STRUCTURE.md  # This filelf11a_project_frontend/lf11a_project_frontend/

│   ├── QUICKSTART.md         # Quick start guide

│   └── WINDOWS_BUILD.md      # Windows build guide├── assets/               # Application assets├── assets/               # Application assets

│

├── scripts/                  # Build and packaging scripts│   └── icon.svg          # Application icon (SVG format)│   ├── icon.svg          # Application icon (SVG format)

│   ├── package.sh            # Multi-platform package creation

│   ├── quick-build.sh        # Quick development build││   └── style.css         # GTK CSS styling

│   └── setup_windows.sh      # Windows cross-compilation setup

│├── docs/                 # Documentation│

├── src/                      # Source code

│   ├── main.rs               # Application entry point│   ├── CONFIGURATION.md      # Configuration guide├── docs/                 # Documentation

│   ├── config.rs             # Configuration management

│   ├── tests.rs              # Unit tests│   ├── PACKAGING.md          # Package building instructions│   ├── CONFIGURATION.md      # Configuration guide

│   │

│   ├── api/                  # Backend API client│   ├── PROJECT-STRUCTURE.md  # This file│   ├── PACKAGING.md          # Package building instructions

│   │   ├── mod.rs            # API module declarations

│   │   ├── client.rs         # HTTP client implementation│   ├── QUICKSTART.md         # Quick start guide│   ├── PROJECT-STRUCTURE.md  # This file

│   │   └── models.rs         # Data models (Department, Employee, SalaryGrade)

│   ││   └── WINDOWS_BUILD.md      # Windows build guide│   ├── QUICKSTART.md         # Quick start guide

│   └── gui/                  # User interface (egui)

│       ├── mod.rs            # GUI module with Tab enum││   └── WINDOWS-INSTALLER.md  # Windows installer documentation

│       ├── app.rs            # PersonnelApp struct and eframe::App impl

│       ├── colors.rs         # Material3Colors palette├── scripts/              # Build and packaging scripts│

│       ├── components.rs     # Reusable UI components

│       ││   ├── package.sh        # Multi-platform package creation├── scripts/              # Build and packaging scripts

│       └── views/            # Tab view implementations

│           ├── mod.rs        # Views module│   ├── quick-build.sh    # Quick development build│   ├── package.sh        # Multi-platform package creation

│           ├── departments.rs    # Department management view

│           ├── employees.rs      # Employee management view│   └── setup_windows.sh  # Windows cross-compilation setup│   └── quick-build.sh    # Quick development build

│           └── salary_grades.rs  # Salary grade management view

│││

├── target/                   # Build artifacts (ignored by git)

│├── src/                  # Source code├── src/                  # Source code

├── .env                      # Environment configuration (ignored by git)

├── .env.example              # Configuration template│   ├── main.rs           # Application entry point & egui UI│   ├── main.rs           # Application entry point

├── .gitignore                # Git ignore rules

├── Cargo.toml                # Rust project manifest│   ├── config.rs         # Configuration management│   ├── mod.rs            # Module declarations

├── Cargo.lock                # Dependency lock file

├── DESIGN_GUIDE.md           # Material 3 design reference│   ├── tests.rs          # Unit tests│   ├── config.rs         # Configuration management

├── LICENSE                   # MIT License

└── README.md                 # Main project documentation│   ││   │

```

│   └── api/              # Backend API client│   ├── api/              # Backend API client

## Key Components

│       ├── mod.rs        # API module│   │   ├── mod.rs        # API module

### Source Code (`src/`)

│       ├── client.rs     # HTTP client implementation│   │   ├── client.rs     # HTTP client implementation

- **main.rs**: Application entry point

  - Loads configuration│       └── models.rs     # Data models│   │   └── models.rs     # Data models

  - Sets up egui/eframe with custom dark theme

  - Initializes the `PersonnelApp`││   │

  - Loads application icon from SVG

├── target/               # Build artifacts (ignored by git)│   └── gui/              # User interface

- **config.rs**: Configuration management using environment variables

││       ├── mod.rs        # GUI module

- **tests.rs**: Unit tests for models, colors, config, and UI logic (58 tests)

├── .env                  # Environment configuration (ignored by git)│       ├── api.rs        # GUI API integration

- **api/**: Backend API client

  - `client.rs`: Async HTTP client using `reqwest`├── .env.example          # Configuration template│       ├── main_window.rs    # Main application window

  - `models.rs`: Data structures for departments, employees, salary grades

├── .gitignore            # Git ignore rules│       ├── models.rs     # GUI data models

- **gui/**: User interface module

  - `mod.rs`: Module declarations and `Tab` enum├── Cargo.toml            # Rust project manifest│       │

  - `app.rs`: Main `PersonnelApp` struct with state management

  - `colors.rs`: Material 3 dark theme color palette├── Cargo.lock            # Dependency lock file│       ├── views/        # Tab views for main window

  - `components.rs`: Reusable `material_button` and `material_card` functions

  - `views/`: Tab-specific view implementations├── DESIGN_GUIDE.md       # Material 3 design reference│       │   ├── mod.rs



### Assets (`assets/`)├── LICENSE               # MIT License│       │   ├── department_view.rs   # Department management



- **icon.svg**: Application icon in SVG format (embedded at compile time using `resvg`)└── README.md             # Main project documentation│       │   ├── employee_view.rs     # Employee management



### Documentation (`docs/`)```│       │   └── salary_grade_view.rs # Salary grade management



All project documentation is centralized here for easy access:│       │

- Configuration guide

- Packaging instructions## Key Components│       └── dialogs/      # CRUD dialogs

- Quick start guide

- Windows build guide│           ├── mod.rs

- Project structure (this file)

### Source Code (`src/`)│           ├── department_dialog.rs

### Scripts (`scripts/`)

- **main.rs**: Application entry point with complete egui UI implementation│           ├── employee_dialog.rs

Build and packaging automation:

- **package.sh**: Creates DEB, RPM, Arch, AppImage, and Windows packages  - Tab navigation (Departments, Employees, Salary Grades)│           └── salary_grade_dialog.rs

- **quick-build.sh**: Quick release build for development

- **setup_windows.sh**: Sets up Windows cross-compilation environment  - Material 3 color palette│



## Architecture  - Form components for CRUD operations├── target/               # Build artifacts (ignored by git)



### egui Framework  - List views with card-based design│



The application uses the `egui` immediate-mode GUI framework with `eframe` for native windowing.- **config.rs**: Configuration management using environment variables├── .env                  # Environment configuration (ignored by git)



Key benefits:- **tests.rs**: Unit tests for models, colors, config, and UI logic├── .env.example          # Configuration template

- Cross-platform (Linux, Windows, macOS)

- No external runtime dependencies- **api/**: Backend API client with async HTTP communication├── .gitignore            # Git ignore rules

- Fast iteration with hot reloading support

- Simple, Rust-native API├── Cargo.toml            # Rust project manifest



### Material 3 Design System### Assets (`assets/`)├── Cargo.lock            # Dependency lock file



The UI implements Material Design 3 (Material You) principles:- **icon.svg**: Application icon in SVG format (embedded at compile time)├── LICENSE               # MIT License



```rust└── README.md             # Main project documentation

pub struct Material3Colors {

    pub primary: Color32,           // Light purple### Documentation (`docs/`)```

    pub on_primary: Color32,        // Dark purple (for text on primary)

    pub surface: Color32,           // Very dark backgroundAll project documentation is centralized here for easy access:

    pub on_surface: Color32,        // Light text

    pub error: Color32,             // Error state- Configuration guide## Key Components

    pub success: Color32,           // Success state

    // ... more colors- Packaging instructions

}

```- Quick start guide### Source Code (`src/`)



### Tab System- Windows build guide- **main.rs**: Application entry point, initializes GTK and loads configuration



```rust- Project structure (this file)- **config.rs**: Configuration management using environment variables

pub enum Tab {

    Departments,- **api/**: Backend API client with HTTP communication

    Employees,

    SalaryGrades,### Scripts (`scripts/`)- **gui/**: GTK4-based user interface components

}

```Build and packaging automation:



Simple enum-based tab navigation rendered in the top panel.- **package.sh**: Creates DEB, RPM, Arch, AppImage, and Windows packages### Assets (`assets/`)



### API Client- **quick-build.sh**: Quick release build for development- **icon.svg**: Application icon in SVG format (converted to PNG/ICO during packaging)



```rust- **setup_windows.sh**: Sets up Windows cross-compilation environment- **style.css**: GTK CSS for custom styling (Material Design inspired)

pub struct ApiClient {

    client: reqwest::Client,

    base_url: String,

}## Architecture### Documentation (`docs/`)

```

All project documentation is centralized here for easy access:

Handles all async HTTP communication with the backend API using `reqwest` and `tokio`.

### egui Framework- Configuration guide

## Module Organization

The application uses the `egui` immediate-mode GUI framework with `eframe` for native windowing.- Packaging instructions

### GUI Module (`src/gui/`)

- Quick start guide

Organized into:

- **mod.rs**: Top-level module with `Tab` enum and re-exportsKey benefits:- Windows installer documentation

- **app.rs**: `PersonnelApp` struct with all state and `eframe::App` implementation

- **colors.rs**: `Material3Colors` struct with dark theme colors- Cross-platform (Linux, Windows, macOS)- Project structure (this file)

- **components.rs**: Reusable UI helper functions

- **views/**: Tab view implementations (each view is an `impl` block on `PersonnelApp`)- No external runtime dependencies



### API Module (`src/api/`)- Fast iteration with hot reloading support### Scripts (`scripts/`)



```rust- Simple, Rust-native APIBuild and packaging automation:

pub struct ApiClient {

    client: reqwest::Client,- **package.sh**: Creates DEB, RPM, Arch, AppImage, and Windows packages

    base_url: String,

}### Material 3 Design System- **quick-build.sh**: Quick release build for development

```

The UI implements Material Design 3 (Material You) principles:

Handles all HTTP communication with the backend API.

## Module Organization

### Configuration (`src/config.rs`)

```rust

Singleton configuration loaded from `.env` file:

```ruststruct Material3Colors {### API Module (`src/api/`)

Config::get().api_url()  // Access configuration

```    primary: Color32,           // Light purple```rust



## Building    on_primary: Color32,        // Dark purple (for text on primary)pub struct ApiClient {



### Debug Build    surface: Color32,           // Very dark background    client: reqwest::Client,

```bash

cargo build    on_surface: Color32,        // Light text    base_url: String,

cargo run

```    error: Color32,             // Error state}



### Release Build    // ... more colors```

```bash

cargo build --release}Handles all HTTP communication with the backend API.

./target/release/lf11a_project_frontend_egui

``````



### Running Tests### GUI Module (`src/gui/`)

```bash

cargo test### API ClientOrganized into:

```

```rust- **views/**: Main tab views (departments, employees, salary grades)

### Windows Cross-Compilation

```bashpub struct ApiClient {- **dialogs/**: Create/edit dialogs for CRUD operations

rustup target add x86_64-pc-windows-gnu

cargo build --release --target x86_64-pc-windows-gnu    client: reqwest::Client,- **main_window.rs**: Application window with tab layout

```

}

## Dependencies

```### Configuration (`src/config.rs`)

Core dependencies:

- `eframe` / `egui`: GUI frameworkHandles all async HTTP communication with the backend API using `reqwest` and `tokio`.Singleton configuration loaded from `.env` file:

- `reqwest`: HTTP client

- `tokio`: Async runtime```rust

- `serde` / `serde_json`: Serialization

- `dotenvy`: Environment file loading### Tab SystemConfig::get().api_url()  // Access configuration

- `resvg`: SVG rendering for icon loading

- `image`: Image processing```rust```



## File Naming Conventionsenum Tab {



- **Rust files**: snake_case (e.g., `salary_grades.rs`)    Departments,## File Naming Conventions

- **Directories**: lowercase (e.g., `docs/`, `scripts/`)

- **Documentation**: UPPERCASE.md (e.g., `README.md`, `CONFIGURATION.md`)    Employees,

- **Assets**: lowercase (e.g., `icon.svg`)

    SalaryGrades,- **Rust files**: snake_case (e.g., `main_window.rs`)

## Development Workflow

}- **Directories**: lowercase (e.g., `docs/`, `scripts/`)

1. Edit source code in `src/`

2. Update assets in `assets/` if needed```- **Documentation**: UPPERCASE.md (e.g., `README.md`, `CONFIGURATION.md`)

3. Test with `cargo build && cargo run`

4. Run tests with `cargo test`Simple enum-based tab navigation rendered in the top panel.- **Assets**: lowercase (e.g., `icon.svg`, `style.css`)

5. Update documentation in `docs/` if needed

6. Build packages with `./scripts/package.sh`



## References## Building## Build Artifacts



- Main README: `README.md`

- Configuration: `docs/CONFIGURATION.md`

- Packaging: `docs/PACKAGING.md`### Debug Build### Development Build

- Quick Start: `docs/QUICKSTART.md`

```bash```bash

cargo buildcargo build

cargo run# Binary: target/debug/lf11a_project_frontend

``````



### Release Build### Release Build

```bash```bash

cargo build --releasecargo build --release

./target/release/lf11a_project_frontend_egui# Binary: target/release/lf11a_project_frontend

``````



### Running Tests### Packages

```bashCreated by `./scripts/package.sh` in `build/` directory:

cargo test- `*.deb` - Debian/Ubuntu packages

```- `*.rpm` - Fedora/RHEL/openSUSE packages

- `PKGBUILD` - Arch Linux package

### Windows Cross-Compilation- `*.AppImage` - Universal Linux binary

```bash- `*-setup.exe` - Windows installer

rustup target add x86_64-pc-windows-gnu

cargo build --release --target x86_64-pc-windows-gnu## Path References

```

All path references in the code use relative paths from the project root:

## Dependencies- CSS: `assets/style.css`

- Icon: `assets/icon.svg`

Core dependencies:- Documentation: `docs/*.md`

- `eframe` / `egui`: GUI framework- Scripts: `scripts/*.sh`

- `reqwest`: HTTP client

- `tokio`: Async runtime## Best Practices

- `serde` / `serde_json`: Serialization

- `dotenvy`: Environment file loading1. **Configuration**: Use `.env` file for all configurable values

- `resvg`: SVG rendering for icon loading2. **Assets**: Store all images, icons, and CSS in `assets/`

- `image`: Image processing3. **Documentation**: Add new documentation to `docs/`

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
