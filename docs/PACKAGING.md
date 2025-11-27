# Packaging Guide

## Prerequisites

Before running the packaging script, ensure you have the following tools installed:

### For All Packages
```bash
cargo install cargo-deb cargo-generate-rpm
```

### For Icon Conversion
```bash
# Debian/Ubuntu
sudo apt install inkscape imagemagick

# Fedora/RHEL
sudo dnf install inkscape ImageMagick

# Arch Linux
sudo pacman -S inkscape imagemagick
```

### For Specific Package Types

#### DEB (Debian/Ubuntu)
```bash
sudo apt install dpkg-dev
```

#### RPM (Fedora/RHEL/openSUSE)
```bash
# Fedora/RHEL
sudo dnf install rpm-build

# openSUSE
sudo zypper install rpm-build
```

#### Arch Linux
```bash
sudo pacman -S base-devel
```

#### AppImage
```bash
# Download appimagetool
wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
chmod +x appimagetool-x86_64.AppImage
sudo mv appimagetool-x86_64.AppImage /usr/local/bin/appimagetool
```

#### Windows (Cross-compilation from Linux)
```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Debian/Ubuntu
sudo apt install mingw-w64 nsis

# Fedora/RHEL
sudo dnf install mingw64-gcc nsis

# Arch Linux
sudo pacman -S mingw-w64-gcc nsis
```

**Note:** NSIS is needed to create the Windows installer (.exe). Without it, only a portable executable will be created.

## Building Packages

**Important:** The `package.sh` script only **creates** the package files. It does NOT install anything on your system. Installation is done separately using your package manager.

### Build All Packages
```bash
./package.sh
```

This creates package files in the `build/` directory:
- `build/*.deb` - Debian/Ubuntu package (installable with apt)
- `build/*.rpm` - Fedora/RHEL/openSUSE package (installable with dnf/rpm)
- `build/arch/PKGBUILD` - Arch Linux build script (use with makepkg)
- `build/*.AppImage` - Universal Linux AppImage (portable, no install needed)
- `build/*-setup.exe` - Windows installer (NSIS) - **Recommended for Windows**
- `build/windows/*.exe` - Windows portable executable (if NSIS not available)

### Build Specific Packages

The script automatically detects available tools and builds what it can. If some packages are skipped, install the required dependencies and run again.

**Note:** Do NOT run `package.sh` with sudo. It only needs write access to the project directory.

## Installation

### Debian/Ubuntu (.deb)
```bash
# Using apt (recommended - handles dependencies automatically)
sudo apt install ./build/lf11a-project-frontend_0.1.0_amd64.deb

# Or using dpkg (manual dependency resolution)
sudo dpkg -i build/lf11a-project-frontend_0.1.0_amd64.deb
sudo apt --fix-broken install  # If dependencies are missing
```

### Fedora/RHEL/openSUSE (.rpm)
```bash
# Fedora/RHEL (dnf handles dependencies)
sudo dnf install ./build/lf11a-project-frontend-0.1.0*.rpm

# Or using rpm directly
sudo rpm -i build/lf11a-project-frontend-0.1.0*.rpm

# openSUSE
sudo zypper install ./build/lf11a-project-frontend-0.1.0*.rpm
```

### Arch Linux
```bash
cd build/arch
makepkg -si  # Builds and installs the package
```

### AppImage (Universal Linux - No installation required)
```bash
chmod +x build/lf11a-project-frontend-0.1.0-x86_64.AppImage
./build/lf11a-project-frontend-0.1.0-x86_64.AppImage
```
AppImage is portable - just make it executable and run it. No installation needed.

### Windows

#### Option 1: Using the Installer (Recommended)
1. Download `lf11a-project-frontend-0.1.0-setup.exe`
2. Run the installer
3. Install GTK4 runtime from https://www.gtk.org/docs/installations/windows
4. Configure `.env` file in the installation directory (default: `%LOCALAPPDATA%\LF11A Project Frontend`)
5. Launch from Start Menu or Desktop shortcut

The installer will:
- Install the application to `%LOCALAPPDATA%\LF11A Project Frontend`
- Create Start Menu shortcuts
- Create Desktop shortcut
- Add uninstaller to Windows Programs

#### Option 2: Portable Executable
1. Extract `build/windows/` folder contents
2. Install GTK4 runtime for Windows from https://www.gtk.org/docs/installations/windows
3. Copy `.env.example` to `.env` and configure your API settings
4. Run `lf11a-project-frontend.exe`

## Package Contents

All packages include:
- The compiled binary
- Application icon
- CSS stylesheet
- Desktop entry (Linux only)
- Example configuration file

## Distribution

### Upload to GitHub Releases
```bash
# Create a release on GitHub and upload:
build/lf11a-project-frontend_0.1.0_amd64.deb              # Debian/Ubuntu
build/lf11a-project-frontend-0.1.0*.rpm                  # Fedora/RHEL
build/lf11a-project-frontend-0.1.0-x86_64.AppImage       # Universal Linux
build/lf11a-project-frontend-0.1.0-setup.exe             # Windows Installer
```

**Windows Note:** Upload the installer (`*-setup.exe`), not the portable executable. The installer provides a better user experience.

### Create AUR Package (Arch Linux)
1. Create an AUR repository
2. Upload the PKGBUILD from `build/arch/PKGBUILD`
3. Include the source tarball or link to GitHub release

## Troubleshooting

### Missing Dependencies
If the application doesn't start, ensure GTK4 is installed:

```bash
# Debian/Ubuntu
sudo apt install libgtk-4-1

# Fedora/RHEL
sudo dnf install gtk4

# Arch Linux
sudo pacman -S gtk4
```

### Icon Not Displaying
The icon is installed to standard system locations. If it doesn't appear:
```bash
# Update icon cache (Linux)
sudo gtk-update-icon-cache /usr/share/icons/hicolor/
```

### Windows: GTK Not Found
Download and install GTK4 runtime from:
https://github.com/wingtk/gvsbuild/releases

## Customization

To customize the packages:

1. **Change version**: Edit `APP_VERSION` in `package.sh` and `version` in `Cargo.toml`
2. **Change icon**: Replace `icon.svg` with your own
3. **Add dependencies**: Edit the package control files in `package.sh`
4. **Modify desktop entry**: Edit the `.desktop` file creation in `package.sh`

## Clean Build
```bash
rm -rf build/
cargo clean
```
