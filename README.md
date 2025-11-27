# LF11A Project Frontend

A modern GTK4-based personnel management application built with Rust for managing employees, departments, and salary grades.

![Rust](https://img.shields.io/badge/rust-1.91.0-orange.svg)
![GTK](https://img.shields.io/badge/GTK-4.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

## Features

- 🏢 **Department Management** - Create, update, and delete departments with automatic head role management
- 👥 **Employee Management** - Comprehensive employee CRUD operations with role assignments
- 💰 **Salary Grade Management** - Define and manage salary grades with min/max ranges
- 🎨 **Modern Material Design UI** - Clean, responsive interface that respects GTK themes
- ⚙️ **Configurable** - Environment-based configuration for easy deployment
- 🔄 **Auto-refresh** - Automatic UI updates after CRUD operations
- 📊 **Relationship Management** - Automatic role promotion/demotion for department heads

## Screenshots

*(Add your screenshots here)*

## Installation

### Pre-built Packages

Download the latest release for your platform:

- **Debian/Ubuntu**: `lf11a-project-frontend_0.1.0_amd64.deb`
- **Fedora/RHEL/openSUSE**: `lf11a-project-frontend-0.1.0.rpm`
- **Arch Linux**: Use the PKGBUILD or AUR
- **Universal Linux**: `lf11a-project-frontend-0.1.0-x86_64.AppImage` (works on any distribution)
- **Windows**: `lf11a-project-frontend-0.1.0-setup.exe` (installer with shortcuts)

See [PACKAGING.md](docs/PACKAGING.md) for detailed installation instructions.

### From Source

#### Prerequisites
```bash
# Debian/Ubuntu
sudo apt install libgtk-4-dev build-essential

# Fedora/RHEL
sudo dnf install gtk4-devel gcc

# Arch Linux
sudo pacman -S gtk4 base-devel
```

#### Build and Run
```bash
git clone https://github.com/KyokoSpl/lf11a_project_frontend.git
cd lf11a_project_frontend
cargo build --release
./target/release/lf11a_project_frontend
```

## Configuration

Create a `.env` file from the example:

```bash
cp .env.example .env
nano .env  # Edit with your settings
```

### Key Configuration Options

```bash
# Backend API URL
API_BASE_URL=http://212.132.110.72:8082

# Logging level (TRACE, DEBUG, INFO, WARN, ERROR)
LOG_LEVEL=INFO

# Enable debug mode
DEBUG_MODE=false
```

See [CONFIGURATION.md](docs/CONFIGURATION.md) for all configuration options.

## Usage

1. Start the application
2. Configure your API connection in the `.env` file
3. Use the tabs to navigate between:
   - **Departments**: Manage departments and assign heads
   - **Employees**: Manage employee records and roles
   - **Salary Grades**: Define salary ranges

## Building Packages

To create distribution packages for all platforms:

```bash
./scripts/package.sh
```

This creates:
- DEB package (Debian/Ubuntu)
- RPM package (Fedora/RHEL/openSUSE)
- PKGBUILD (Arch Linux)
- AppImage (Universal Linux)
- Windows executable

See [PACKAGING.md](docs/PACKAGING.md) for detailed packaging instructions.

## Development

### Project Structure

```
lf11a_project_frontend/
├── src/                  # Source code
│   ├── main.rs           # Application entry point
│   ├── config.rs         # Configuration management
│   ├── api/              # Backend API client
│   │   ├── client.rs     # HTTP client implementation
│   │   └── models.rs     # Data models
│   └── gui/              # User interface
│       ├── main_window.rs
│       ├── views/        # Tab views
│       └── dialogs/      # CRUD dialogs
├── assets/               # Application assets
│   ├── style.css         # GTK CSS styling
│   └── icon.svg          # Application icon
├── docs/                 # Documentation
│   ├── CONFIGURATION.md
│   ├── PACKAGING.md
│   ├── QUICKSTART.md
│   └── WINDOWS-INSTALLER.md
├── scripts/              # Build and packaging scripts
│   ├── package.sh        # Multi-platform packaging
│   └── quick-build.sh    # Quick development build
├── .env.example          # Configuration template
└── README.md             # This file
```

### Technologies

- **Language**: Rust 1.91.0
- **UI Framework**: GTK4 4.0
- **HTTP Client**: reqwest
- **Async Runtime**: tokio
- **Configuration**: dotenvy

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [GTK](https://www.gtk.org/)
- Powered by [Rust](https://www.rust-lang.org/)
- Icon design inspired by the Rust logo

## Support

For issues, questions, or contributions, please visit:
- [GitHub Issues](https://github.com/KyokoSpl/lf11a_project_frontend/issues)
- [GitHub Discussions](https://github.com/KyokoSpl/lf11a_project_frontend/discussions)