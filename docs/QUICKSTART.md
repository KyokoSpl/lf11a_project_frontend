# Quick Start Guide

## For End Users

### Installation

Choose your platform:

#### Linux (Debian/Ubuntu)
```bash
sudo dpkg -i lf11a-project-frontend_0.1.0_amd64.deb
```

#### Linux (Fedora/RHEL)
```bash
sudo dnf install lf11a-project-frontend-0.1.0*.rpm
```

#### Linux (Arch)
```bash
cd arch/
makepkg -si
```

#### Linux (Universal - AppImage)
```bash
chmod +x lf11a-project-frontend-0.1.0-x86_64.AppImage
./lf11a-project-frontend-0.1.0-x86_64.AppImage
```

#### Windows (Installer - Recommended)
1. Download and run `lf11a-project-frontend-0.1.0-setup.exe`
2. Configure `.env` in installation directory: `%LOCALAPPDATA%\LF11A Project Frontend`
3. Launch from Start Menu or Desktop shortcut

#### Windows (Portable)
1. Extract the portable folder
2. Edit `.env` with your API settings
3. Run `lf11a-project-frontend.exe`

### Configuration

1. Copy `.env.example` to `.env`
2. Edit the API URL:
   ```bash
   API_BASE_URL=http://your-backend-server:8082
   ```
3. Set log level (optional):
   ```bash
   LOG_LEVEL=INFO  # Options: TRACE, DEBUG, INFO, WARN, ERROR
   ```

### Running

After installation, launch from:
- **Linux**: Application menu or run `lf11a-project-frontend` in terminal
- **Windows**: Start Menu → "LF11A Project Frontend" or Desktop shortcut
- **Windows (Portable)**: Double-click the `.exe` file

---

## For Developers

### Quick Development Setup

```bash
# Clone the repository
git clone https://github.com/KyokoSpl/lf11a_project_frontend.git
cd lf11a_project_frontend

# Configure
cp .env.example .env
nano .env  # Edit with your settings

# Build and run
cargo run
```

### Development Build
```bash
cargo build          # Debug build
cargo run           # Run debug build
cargo build --release  # Release build
```

### Run Tests
```bash
cargo test           # Run all tests
```

### Create Distribution Packages
```bash
./scripts/package.sh         # Creates packages in build/ directory
```
**Important:** This only creates the package files. Users install them with:
- DEB: `sudo apt install ./lf11a-project-frontend_0.1.0_amd64.deb`
- RPM: `sudo dnf install ./lf11a-project-frontend-0.1.0*.rpm`
- Arch: `cd arch/ && makepkg -si`
- AppImage: `chmod +x *.AppImage && ./lf11a-project-frontend*.AppImage`

### Project Structure
```
src/
├── main.rs          # Entry point
├── config.rs        # Configuration
├── api/             # Backend API client
└── gui/             # User interface
    ├── views/       # Departments, Employees, Salary Grades
    └── dialogs/     # CRUD dialogs
```

### Useful Commands

```bash
# Quick build
./quick-build.sh

# Format code
cargo fmt

# Check for errors
cargo check

# Run tests
cargo test

# Clean build artifacts
cargo clean
rm -rf build/
```

---

## Troubleshooting

### Linux: Application doesn't start
```bash
# Check if GTK4 is installed
dpkg -l | grep libgtk-4  # Debian/Ubuntu
rpm -qa | grep gtk4      # Fedora/RHEL
pacman -Qs gtk4          # Arch

# Install if missing
sudo apt install libgtk-4-1        # Debian/Ubuntu
sudo dnf install gtk4              # Fedora/RHEL
sudo pacman -S gtk4                # Arch
```

### Windows: GTK not found
Download GTK4 runtime from:
- https://github.com/wingtk/gvsbuild/releases

### Can't connect to backend
1. Check `.env` file has correct `API_BASE_URL`
2. Verify backend is running: `curl http://your-backend:8082/api/departments`
3. Check firewall settings

### Debug mode
Enable verbose logging:
```bash
# In .env file
LOG_LEVEL=DEBUG
DEBUG_MODE=true
```

---

## Support

- **Issues**: https://github.com/KyokoSpl/lf11a_project_frontend/issues
- **Documentation**: See `docs/CONFIGURATION.md` and `docs/PACKAGING.md`
- **Source Code**: https://github.com/KyokoSpl/lf11a_project_frontend

---

## Building From Source

### Prerequisites

#### Debian/Ubuntu
```bash
sudo apt install libgtk-4-dev build-essential pkg-config
```

#### Fedora/RHEL
```bash
sudo dnf install gtk4-devel gcc pkg-config
```

#### Arch Linux
```bash
sudo pacman -S gtk4 base-devel
```

#### Windows (MSYS2)
```bash
pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-toolchain
```

### Build
```bash
cargo build --release
```

Binary will be in: `target/release/lf11a_project_frontend`

---

## License

MIT License - See LICENSE file for details
