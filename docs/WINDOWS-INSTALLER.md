# Windows Installer Guide

## Overview

The Windows installer (`lf11a-project-frontend-0.1.0-setup.exe`) is an NSIS-based installer that provides a complete installation experience for Windows users.

## What the Installer Does

### Installation
- Installs the application to `%LOCALAPPDATA%\LF11A Project Frontend`
- Creates a Start Menu folder with shortcuts
- Creates a Desktop shortcut
- Includes all required files (executable, CSS, icon, example config)
- Registers the application with Windows
- Creates an uninstaller

### Files Installed
```
%LOCALAPPDATA%\LF11A Project Frontend\
├── lf11a-project-frontend.exe    # Main application
├── style.css                      # UI stylesheet
├── .env.example                   # Configuration template
├── README.txt                     # Quick start guide
├── icon.png                       # Application icon
├── icon.ico                       # Windows icon
└── uninstall.exe                  # Uninstaller
```

### Shortcuts Created
- Start Menu: `LF11A Project Frontend`
- Desktop: `LF11A Project Frontend`
- Uninstaller in Windows Programs & Features

## Installation Steps

1. **Download the installer**
   - Get `lf11a-project-frontend-0.1.0-setup.exe` from releases

2. **Run the installer**
   - Double-click the `.exe` file
   - Click "Next" through the installation wizard
   - Choose installation directory (default is recommended)
   - Click "Install"

3. **Install GTK4 Runtime** (Required)
   - Download from: https://www.gtk.org/docs/installations/windows
   - Or: https://github.com/wingtk/gvsbuild/releases
   - Install the GTK4 runtime for Windows

4. **Configure the application**
   - Navigate to: `%LOCALAPPDATA%\LF11A Project Frontend`
   - Copy `.env.example` to `.env`
   - Edit `.env` with your API settings:
     ```ini
     API_BASE_URL=http://your-backend-server:8082
     LOG_LEVEL=INFO
     ```

5. **Launch**
   - Use Start Menu shortcut or Desktop shortcut
   - Or run directly: `%LOCALAPPDATA%\LF11A Project Frontend\lf11a-project-frontend.exe`

## Uninstallation

### Method 1: Windows Settings
1. Open Windows Settings
2. Go to Apps → Installed Apps
3. Find "LF11A Project Frontend"
4. Click Uninstall

### Method 2: Start Menu
1. Open Start Menu
2. Find "LF11A Project Frontend" folder
3. Click "Uninstall"

### Method 3: Direct Uninstaller
Run: `%LOCALAPPDATA%\LF11A Project Frontend\uninstall.exe`

### What Gets Removed
- Application executable and files
- Start Menu shortcuts
- Desktop shortcut
- Registry entries
- Installation directory

**Note:** Your `.env` configuration file will also be removed.

## Silent Installation (Advanced)

For automated deployments:

```cmd
REM Silent install
lf11a-project-frontend-0.1.0-setup.exe /S

REM Silent install to custom directory
lf11a-project-frontend-0.1.0-setup.exe /S /D=C:\MyApps\LF11A

REM Silent uninstall
"%LOCALAPPDATA%\LF11A Project Frontend\uninstall.exe" /S
```

## Troubleshooting

### Installer won't run
- **Windows SmartScreen**: Click "More info" → "Run anyway"
- **Antivirus**: Add exception for the installer
- **User permissions**: Run as Administrator if needed

### Application won't start
1. **GTK4 not installed**: Install GTK4 runtime first
2. **Missing DLLs**: Ensure GTK4 bin directory is in PATH
3. **Check logs**: Look for error messages in terminal output

### Can't find installation
- Default location: `%LOCALAPPDATA%\LF11A Project Frontend`
- Full path: `C:\Users\YourUsername\AppData\Local\LF11A Project Frontend`

### Configuration not found
1. Copy `.env.example` to `.env` in installation directory
2. Edit with your settings
3. Restart the application

## Building the Installer (For Developers)

### Prerequisites
```bash
# On Linux (cross-compilation)
sudo apt install mingw-w64 nsis
rustup target add x86_64-pc-windows-gnu
```

### Build
```bash
./package.sh
```

This creates: `build/lf11a-project-frontend-0.1.0-setup.exe`

### Testing
- Test on Windows 10 and Windows 11
- Test with and without GTK4 pre-installed
- Test installation and uninstallation
- Verify shortcuts work correctly
- Check file associations and registry entries

## Customization

To modify the installer, edit the NSIS script in `package.sh` or create a separate `.nsi` file:

```nsis
!define APP_NAME "Your App Name"
!define VERSION "1.0.0"
InstallDir "$PROGRAMFILES64\YourApp"
```

## Distribution

### GitHub Releases
Upload the installer as a release asset:
```
lf11a-project-frontend-0.1.0-setup.exe
```

### Checksums
Generate checksums for verification:
```bash
sha256sum lf11a-project-frontend-0.1.0-setup.exe > checksums.txt
```

### Download Size
- Installer: ~5-10 MB (depends on included resources)
- GTK4 Runtime: ~50-100 MB (separate download)

## Security

- The installer is unsigned (code signing certificate required for production)
- Users may see SmartScreen warnings
- For production: Consider code signing certificate from Sectigo, DigiCert, etc.

## Support

For issues with the installer:
- Check GTK4 installation
- Verify Windows version compatibility (Windows 10+)
- Report issues on GitHub with installation logs
