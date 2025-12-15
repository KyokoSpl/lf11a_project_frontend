#!/usr/bin/env bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

APP_NAME="lf11a-project-frontend"
APP_DESCRIPTION="LF11A Project Frontend - egui Personnel Management Application"
AUTHOR="kyoko"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$PROJECT_ROOT/build"
RELEASES_DIR="$PROJECT_ROOT/releases"
BINARY_NAME="lf11a_project_frontend_egui"

# Extract version from Cargo.toml
APP_VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
echo -e "${BLUE}Detected version: ${APP_VERSION}${NC}"

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --release    Build all packages and create a GitHub release"
    echo "  --help       Show this help message"
    echo ""
    echo "Without options, builds all packages for Linux and Windows."
}

# Function to create GitHub release
create_github_release() {
    echo -e "${GREEN}=== Creating GitHub Release v${APP_VERSION} ===${NC}"
    
    # Check if gh CLI is installed
    if ! command -v gh &> /dev/null; then
        echo -e "${RED}Error: GitHub CLI (gh) is not installed.${NC}"
        echo -e "Install it with: sudo dnf install gh  (Fedora)"
        echo -e "                 sudo apt install gh  (Debian/Ubuntu)"
        echo -e "Then authenticate with: gh auth login"
        exit 1
    fi
    
    # Check if authenticated
    if ! gh auth status &> /dev/null; then
        echo -e "${RED}Error: Not authenticated with GitHub CLI.${NC}"
        echo -e "Run: gh auth login"
        exit 1
    fi
    
    # Create releases directory
    mkdir -p "$RELEASES_DIR"
    
    # Copy all packages to releases directory
    echo -e "${YELLOW}Collecting release artifacts...${NC}"
    
    # Copy DEB package
    if [ -f "$BUILD_DIR/${APP_NAME}_${APP_VERSION}_amd64.deb" ]; then
        cp "$BUILD_DIR/${APP_NAME}_${APP_VERSION}_amd64.deb" "$RELEASES_DIR/"
        echo -e "${GREEN}✓${NC} DEB package"
    fi
    
    # Copy RPM package
    if [ -f "$HOME/rpmbuild/RPMS/x86_64/${APP_NAME}-${APP_VERSION}-1.x86_64.rpm" ]; then
        cp "$HOME/rpmbuild/RPMS/x86_64/${APP_NAME}-${APP_VERSION}-1.x86_64.rpm" "$RELEASES_DIR/"
        echo -e "${GREEN}✓${NC} RPM package"
    fi
    
    # Copy AppImage
    if [ -f "$BUILD_DIR/${APP_NAME}-${APP_VERSION}-x86_64.AppImage" ]; then
        cp "$BUILD_DIR/${APP_NAME}-${APP_VERSION}-x86_64.AppImage" "$RELEASES_DIR/"
        echo -e "${GREEN}✓${NC} AppImage"
    fi
    
    # Copy Windows installer
    if [ -f "$BUILD_DIR/${APP_NAME}-${APP_VERSION}-setup.exe" ]; then
        cp "$BUILD_DIR/${APP_NAME}-${APP_VERSION}-setup.exe" "$RELEASES_DIR/"
        echo -e "${GREEN}✓${NC} Windows installer"
    fi
    
    # Copy Arch PKGBUILD
    if [ -f "$BUILD_DIR/arch/PKGBUILD" ]; then
        cp "$BUILD_DIR/arch/PKGBUILD" "$RELEASES_DIR/"
        echo -e "${GREEN}✓${NC} Arch PKGBUILD"
    fi
    
    # Generate release notes from recent commits
    echo -e "${YELLOW}Generating release notes...${NC}"
    
    RELEASE_NOTES_FILE="$RELEASES_DIR/RELEASE_NOTES.md"
    
    # Get the last tag or use initial commit if no tags
    LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
    
    cat > "$RELEASE_NOTES_FILE" << EOF
# LF11A Project Frontend v${APP_VERSION}

## What's New

EOF
    
    # Add commits since last tag (or last 20 commits if no tag)
    if [ -n "$LAST_TAG" ]; then
        echo "### Changes since ${LAST_TAG}" >> "$RELEASE_NOTES_FILE"
        echo "" >> "$RELEASE_NOTES_FILE"
        git log "${LAST_TAG}..HEAD" --pretty=format:"- %s" --no-merges >> "$RELEASE_NOTES_FILE" 2>/dev/null || true
    else
        echo "### Recent Changes" >> "$RELEASE_NOTES_FILE"
        echo "" >> "$RELEASE_NOTES_FILE"
        git log -20 --pretty=format:"- %s" --no-merges >> "$RELEASE_NOTES_FILE" 2>/dev/null || true
    fi
    
    cat >> "$RELEASE_NOTES_FILE" << EOF


## Downloads

| Platform | Package | Description |
|----------|---------|-------------|
| Debian/Ubuntu | \`${APP_NAME}_${APP_VERSION}_amd64.deb\` | Install with \`sudo dpkg -i\` |
| Fedora/RHEL | \`${APP_NAME}-${APP_VERSION}-1.x86_64.rpm\` | Install with \`sudo rpm -i\` |
| Linux (Universal) | \`${APP_NAME}-${APP_VERSION}-x86_64.AppImage\` | Make executable and run |
| Windows | \`${APP_NAME}-${APP_VERSION}-setup.exe\` | Run the installer |
| Arch Linux | \`PKGBUILD\` | Build with \`makepkg -si\` |

## System Requirements

- **Linux**: X11/Wayland with OpenGL support
- **Windows**: Windows 10/11 with OpenGL support

## Installation

### Linux (DEB)
\`\`\`bash
sudo dpkg -i ${APP_NAME}_${APP_VERSION}_amd64.deb
sudo apt-get install -f  # Install dependencies if needed
\`\`\`

### Linux (RPM)
\`\`\`bash
sudo rpm -i ${APP_NAME}-${APP_VERSION}-1.x86_64.rpm
\`\`\`

### Linux (AppImage)
\`\`\`bash
chmod +x ${APP_NAME}-${APP_VERSION}-x86_64.AppImage
./${APP_NAME}-${APP_VERSION}-x86_64.AppImage
\`\`\`

### Windows
Run the installer and follow the prompts. Desktop and Start Menu shortcuts will be created.
EOF
    
    echo -e "${GREEN}Release notes generated: ${RELEASE_NOTES_FILE}${NC}"
    
    # Generate SHA256 checksums
    echo -e "${YELLOW}Generating checksums...${NC}"
    cd "$RELEASES_DIR"
    sha256sum *.deb *.rpm *.AppImage *.exe PKGBUILD 2>/dev/null > SHA256SUMS.txt || true
    cd "$PROJECT_ROOT"
    echo -e "${GREEN}✓${NC} SHA256SUMS.txt generated"
    
    # List files to be uploaded
    echo -e "${YELLOW}Files to be released:${NC}"
    ls -lh "$RELEASES_DIR"
    
    # Ask for confirmation
    echo ""
    echo -e "${YELLOW}Ready to create GitHub release v${APP_VERSION}${NC}"
    read -p "Do you want to proceed? (y/N) " -n 1 -r
    echo
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${YELLOW}Release cancelled.${NC}"
        exit 0
    fi
    
    # Create the GitHub release
    echo -e "${GREEN}Creating GitHub release...${NC}"
    
    # Collect all release files
    RELEASE_FILES=()
    for file in "$RELEASES_DIR"/*.deb "$RELEASES_DIR"/*.rpm "$RELEASES_DIR"/*.AppImage "$RELEASES_DIR"/*.exe "$RELEASES_DIR"/PKGBUILD "$RELEASES_DIR"/SHA256SUMS.txt; do
        if [ -f "$file" ]; then
            RELEASE_FILES+=("$file")
        fi
    done
    
    # Create release with gh CLI
    gh release create "v${APP_VERSION}" \
        --title "LF11A Project Frontend v${APP_VERSION}" \
        --notes-file "$RELEASE_NOTES_FILE" \
        "${RELEASE_FILES[@]}"
    
    echo -e "${GREEN}=== GitHub Release v${APP_VERSION} created successfully! ===${NC}"
    echo -e "View at: https://github.com/$(gh repo view --json nameWithOwner -q .nameWithOwner)/releases/tag/v${APP_VERSION}"
}

# Parse command line arguments
RELEASE_MODE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            RELEASE_MODE=true
            shift
            ;;
        --help|-h)
            show_usage
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            show_usage
            exit 1
            ;;
    esac
done

echo -e "${GREEN}=== Building LF11A Project Frontend Packages ===${NC}"

# Clean previous builds
echo -e "${YELLOW}Cleaning previous builds...${NC}"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# Change to project root for cargo commands
cd "$PROJECT_ROOT"

# Build the application in release mode
echo -e "${YELLOW}Building application in release mode...${NC}"
cargo build --release

# Convert SVG icon to PNG (requires inkscape or imagemagick)
echo -e "${YELLOW}Converting icon to PNG...${NC}"
if command -v inkscape &> /dev/null; then
    inkscape "$PROJECT_ROOT/assets/icon.svg" --export-filename="$BUILD_DIR/icon.png" --export-width=512 --export-height=512
elif command -v convert &> /dev/null; then
    convert -background none "$PROJECT_ROOT/assets/icon.svg" -resize 512x512 "$BUILD_DIR/icon.png"
else
    echo -e "${RED}Warning: Neither inkscape nor imagemagick found. Icon conversion skipped.${NC}"
    cp "$PROJECT_ROOT/assets/icon.svg" "$BUILD_DIR/icon.png" 2>/dev/null || true
fi

# Create additional icon sizes for Linux
if [ -f "$BUILD_DIR/icon.png" ]; then
    for size in 16 32 48 64 128 256; do
        if command -v convert &> /dev/null; then
            convert "$BUILD_DIR/icon.png" -resize ${size}x${size} "$BUILD_DIR/icon-${size}.png"
        fi
    done
fi

# Package for DEB (Debian/Ubuntu)
echo -e "${GREEN}=== Creating DEB package ===${NC}"
DEB_DIR="$BUILD_DIR/deb-build"
mkdir -p "$DEB_DIR/DEBIAN"
mkdir -p "$DEB_DIR/usr/bin"
mkdir -p "$DEB_DIR/usr/share/applications"
mkdir -p "$DEB_DIR/usr/share/icons/hicolor/512x512/apps"
mkdir -p "$DEB_DIR/usr/share/pixmaps"
mkdir -p "$DEB_DIR/usr/share/$APP_NAME"

# Copy binary
cp "target/release/$BINARY_NAME" "$DEB_DIR/usr/bin/$APP_NAME"
chmod +x "$DEB_DIR/usr/bin/$APP_NAME"

# Copy icon
if [ -f "$BUILD_DIR/icon.png" ]; then
    cp "$BUILD_DIR/icon.png" "$DEB_DIR/usr/share/icons/hicolor/512x512/apps/$APP_NAME.png"
    cp "$BUILD_DIR/icon.png" "$DEB_DIR/usr/share/pixmaps/$APP_NAME.png"
fi

# Copy .env file with backend configuration
cp .env "$DEB_DIR/usr/share/$APP_NAME/" 2>/dev/null || cp .env.example "$DEB_DIR/usr/share/$APP_NAME/.env" 2>/dev/null || true

# Create desktop entry
cat > "$DEB_DIR/usr/share/applications/$APP_NAME.desktop" << EOF
[Desktop Entry]
Version=${APP_VERSION}
Type=Application
Name=LF11A Project Frontend
Comment=$APP_DESCRIPTION
Exec=$APP_NAME
Icon=$APP_NAME
Categories=Office;Database;
Terminal=false
EOF

# Create control file
cat > "$DEB_DIR/DEBIAN/control" << EOF
Package: $APP_NAME
Version: ${APP_VERSION}
Section: utils
Priority: optional
Architecture: amd64
Depends: libgl1, libxcb1, libxkbcommon0
Maintainer: $AUTHOR
Description: $APP_DESCRIPTION
 A modern egui-based personnel management application
 built with Rust for managing employees, departments,
 and salary grades.
EOF

# Build DEB package
if command -v dpkg-deb &> /dev/null; then
    dpkg-deb --build "$DEB_DIR" "$BUILD_DIR/${APP_NAME}_${APP_VERSION}_amd64.deb"
    echo -e "${GREEN}✓ DEB package created: ${APP_NAME}_${APP_VERSION}_amd64.deb${NC}"
else
    echo -e "${YELLOW}dpkg-deb not found. Skipping DEB package.${NC}"
fi

# Package for RPM (Fedora/RHEL/openSUSE)
echo -e "${GREEN}=== Creating RPM package ===${NC}"
RPM_BUILD_DIR="$BUILD_DIR/rpmbuild"
mkdir -p "$RPM_BUILD_DIR"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Create tarball for sources
TARBALL="$RPM_BUILD_DIR/SOURCES/${APP_NAME}-${APP_VERSION}.tar.gz"
mkdir -p "/tmp/${APP_NAME}-${APP_VERSION}"
cp "target/release/$BINARY_NAME" "/tmp/${APP_NAME}-${APP_VERSION}/"
[ -f "$BUILD_DIR/icon.png" ] && cp "$BUILD_DIR/icon.png" "/tmp/${APP_NAME}-${APP_VERSION}/"
tar -czf "$TARBALL" -C /tmp "${APP_NAME}-${APP_VERSION}"
rm -rf "/tmp/${APP_NAME}-${APP_VERSION}"

# Create RPM spec file
cat > "$RPM_BUILD_DIR/SPECS/${APP_NAME}.spec" << EOF
%global debug_package %{nil}

Name:           lf11a-project-frontend
Version:        ${APP_VERSION}
Release:        1%{?dist}
Summary:        LF11A Project Frontend - Personnel Management Application

License:        MIT
URL:            https://github.com/KyokoSpl/lf11a_project_frontend
Source0:        %{name}-%{version}.tar.gz

Requires:       mesa-libGL, libxcb, libxkbcommon

%description
A modern egui-based personnel management application built with Rust
for managing employees, departments, and salary grades.

%prep
%setup -q

%install
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/applications
mkdir -p %{buildroot}%{_datadir}/icons/hicolor/512x512/apps
mkdir -p %{buildroot}%{_datadir}/%{name}

install -m 755 lf11a_project_frontend_egui %{buildroot}%{_bindir}/%{name}
install -m 644 icon.png %{buildroot}%{_datadir}/icons/hicolor/512x512/apps/%{name}.png || true

cat > %{buildroot}%{_datadir}/applications/%{name}.desktop << DESKTOP
[Desktop Entry]
Version=1.0
Type=Application
Name=LF11A Project Frontend
Comment=Personnel Management Application
Exec=%{name}
Icon=%{name}
Categories=Office;Database;
Terminal=false
DESKTOP

%files
%{_bindir}/%{name}
%{_datadir}/applications/%{name}.desktop
%{_datadir}/icons/hicolor/512x512/apps/%{name}.png

%changelog
* Wed Nov 27 2024 kyoko <kyoko@example.com> - ${APP_VERSION}-1
- Initial package
EOF

# Build RPM
if command -v rpmbuild &> /dev/null; then
    if rpmbuild --define "_topdir $RPM_BUILD_DIR" -bb "$RPM_BUILD_DIR/SPECS/${APP_NAME}.spec" 2>&1; then
        cp "$RPM_BUILD_DIR/RPMS/x86_64/${APP_NAME}-${APP_VERSION}"*.rpm "$BUILD_DIR/" 2>/dev/null && \
            echo -e "${GREEN}✓ RPM package created${NC}" || \
            echo -e "${YELLOW}Warning: RPM build completed but package not found${NC}"
    else
        echo -e "${RED}✗ RPM build failed${NC}"
    fi
else
    echo -e "${YELLOW}rpmbuild not found. Skipping RPM package.${NC}"
fi

# Package for Arch Linux (PKGBUILD)
echo -e "${GREEN}=== Creating Arch Linux package ===${NC}"
ARCH_DIR="$BUILD_DIR/arch"
mkdir -p "$ARCH_DIR"

# Copy the binary and icon
cp "$PROJECT_ROOT/target/release/lf11a_project_frontend_egui" "$ARCH_DIR/"
if [ -f "$BUILD_DIR/icon.png" ]; then
    cp "$BUILD_DIR/icon.png" "$ARCH_DIR/"
fi

cat > "$ARCH_DIR/PKGBUILD" << EOF
# Maintainer: kyoko <kyoko@example.com>
pkgname=lf11a-project-frontend
pkgver=${APP_VERSION}
pkgrel=1
pkgdesc="LF11A Project Frontend - Personnel Management Application"
arch=('x86_64')
url="https://github.com/KyokoSpl/lf11a_project_frontend"
license=('MIT')
depends=('libgl' 'libxcb' 'libxkbcommon')
source=('lf11a_project_frontend_egui' 'icon.png')
md5sums=('SKIP' 'SKIP')

package() {
    install -Dm755 "\${srcdir}/lf11a_project_frontend_egui" "\${pkgdir}/usr/bin/\${pkgname}"
    install -Dm644 "\${srcdir}/icon.png" "\${pkgdir}/usr/share/icons/hicolor/512x512/apps/\${pkgname}.png"
    
    install -Dm644 /dev/stdin "\${pkgdir}/usr/share/applications/\${pkgname}.desktop" << DESKTOP
[Desktop Entry]
Version=1.0
Type=Application
Name=LF11A Project Frontend
Comment=Personnel Management Application
Exec=\${pkgname}
Icon=\${pkgname}
Categories=Office;Database;
Terminal=false
DESKTOP
}
EOF

echo -e "${GREEN}✓ PKGBUILD created in $ARCH_DIR${NC}"
echo -e "${YELLOW}To build Arch package, run: cd $ARCH_DIR && makepkg -si${NC}"

# Create AppImage
echo -e "${GREEN}=== Creating AppImage ===${NC}"
APPIMAGE_DIR="$BUILD_DIR/appimage-build"
mkdir -p "$APPIMAGE_DIR/usr/bin"
mkdir -p "$APPIMAGE_DIR/usr/share/applications"
mkdir -p "$APPIMAGE_DIR/usr/share/icons/hicolor/512x512/apps"
mkdir -p "$APPIMAGE_DIR/usr/share/$APP_NAME"

cp "target/release/$BINARY_NAME" "$APPIMAGE_DIR/usr/bin/$APP_NAME"
chmod +x "$APPIMAGE_DIR/usr/bin/$APP_NAME"

# Copy resources
cp .env "$APPIMAGE_DIR/usr/share/$APP_NAME/" 2>/dev/null || cp .env.example "$APPIMAGE_DIR/usr/share/$APP_NAME/.env" 2>/dev/null || true

if [ -f "$BUILD_DIR/icon.png" ]; then
    cp "$BUILD_DIR/icon.png" "$APPIMAGE_DIR/usr/share/icons/hicolor/512x512/apps/$APP_NAME.png"
    cp "$BUILD_DIR/icon.png" "$APPIMAGE_DIR/$APP_NAME.png"
fi

cat > "$APPIMAGE_DIR/$APP_NAME.desktop" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=LF11A Project Frontend
Comment=$APP_DESCRIPTION
Exec=$APP_NAME
Icon=$APP_NAME
Categories=Office;Database;
Terminal=false
EOF

cp "$APPIMAGE_DIR/$APP_NAME.desktop" "$APPIMAGE_DIR/usr/share/applications/"

# Create AppRun script
cat > "$APPIMAGE_DIR/AppRun" << 'APPRUN'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"
export XDG_DATA_DIRS="${HERE}/usr/share:${XDG_DATA_DIRS}"
exec "${HERE}/usr/bin/lf11a-project-frontend" "$@"
APPRUN

chmod +x "$APPIMAGE_DIR/AppRun"

# Check for appimagetool (system or local)
APPIMAGETOOL=""
if command -v appimagetool &> /dev/null; then
    APPIMAGETOOL="appimagetool"
elif [ -f "$PROJECT_ROOT/tools/appimagetool" ]; then
    APPIMAGETOOL="$PROJECT_ROOT/tools/appimagetool"
fi

if [ -n "$APPIMAGETOOL" ]; then
    if ARCH=x86_64 "$APPIMAGETOOL" "$APPIMAGE_DIR" "$BUILD_DIR/${APP_NAME}-${APP_VERSION}-x86_64.AppImage" 2>&1; then
        chmod +x "$BUILD_DIR/${APP_NAME}-${APP_VERSION}-x86_64.AppImage"
        echo -e "${GREEN}✓ AppImage created: ${APP_NAME}-${APP_VERSION}-x86_64.AppImage${NC}"
    else
        echo -e "${RED}✗ AppImage creation failed${NC}"
    fi
else
    echo -e "${YELLOW}appimagetool not found. Skipping AppImage.${NC}"
    echo -e "${YELLOW}Install from: https://github.com/AppImage/AppImageKit/releases${NC}"
fi

# Windows EXE (cross-compilation)
echo -e "${GREEN}=== Creating Windows executable ===${NC}"

# Check if Windows target is installed
if ! rustup target list | grep -q "x86_64-pc-windows-gnu (installed)"; then
    echo -e "${YELLOW}Windows target not installed. Installing...${NC}"
    if ! rustup target add x86_64-pc-windows-gnu; then
        echo -e "${RED}✗ Failed to install Windows target. Skipping Windows build.${NC}"
        echo -e "${YELLOW}You can manually install later with: rustup target add x86_64-pc-windows-gnu${NC}"
        exit 0  # Continue with summary instead of failing
    fi
fi

# Check for MinGW cross-compiler
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo -e "${YELLOW}MinGW-w64 cross-compiler not found. Skipping Windows build.${NC}"
    echo -e "${YELLOW}Install with: sudo dnf install mingw64-gcc mingw64-gcc-c++ (Fedora)${NC}"
    exit 0  # Continue with summary
fi

echo -e "${YELLOW}Building for Windows (x86_64-pc-windows-gnu)...${NC}"
if cargo build --release --target x86_64-pc-windows-gnu 2>&1; then
    WIN_DIR="$BUILD_DIR/windows"
    mkdir -p "$WIN_DIR"
    
    cp "target/x86_64-pc-windows-gnu/release/${BINARY_NAME}.exe" "$WIN_DIR/${APP_NAME}.exe"
    cp .env "$WIN_DIR/" 2>/dev/null || cp .env.example "$WIN_DIR/.env" 2>/dev/null || true
    
    # Convert icon to ICO if possible
    if [ -f "$BUILD_DIR/icon.png" ]; then
        if command -v convert &> /dev/null; then
            convert "$BUILD_DIR/icon.png" -define icon:auto-resize=256,128,96,64,48,32,16 "$WIN_DIR/icon.ico"
        fi
        cp "$BUILD_DIR/icon.png" "$WIN_DIR/" 2>/dev/null || true
    fi
    
    # Create README for Windows
    cat > "$WIN_DIR/README.txt" << 'WINREADME'
LF11A Project Frontend - Windows Installation

Requirements:
- Windows 10/11 with OpenGL support

Installation:
1. Copy .env.example to .env and configure your API settings
2. Run lf11a-project-frontend.exe

Configuration:
Edit the .env file to configure:
- API_BASE_URL: Your backend API URL
- LOG_LEVEL: DEBUG, INFO, WARN, or ERROR

For more information, see docs/CONFIGURATION.md
WINREADME
    
    # Create NSIS installer if available
    if command -v makensis &> /dev/null; then
        echo -e "${YELLOW}Creating Windows installer with NSIS...${NC}"
        
        cat > "$WIN_DIR/installer.nsi" << 'NSIS'
!define APP_NAME "LF11A Project Frontend"
!define COMP_NAME "kyoko"
!define VERSION "${APP_VERSION}"
!define COPYRIGHT "kyoko © 2025"
!define DESCRIPTION "Personnel Management Application"
!define INSTALLER_NAME "lf11a-project-frontend-${APP_VERSION}-setup.exe"
!define MAIN_APP_EXE "lf11a-project-frontend.exe"
!define INSTALL_TYPE "SetShellVarContext current"
!define REG_ROOT "HKCU"
!define REG_APP_PATH "Software\Microsoft\Windows\CurrentVersion\App Paths\${MAIN_APP_EXE}"
!define UNINSTALL_PATH "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}"

!include "MUI2.nsh"

Name "${APP_NAME}"
OutFile "${INSTALLER_NAME}"
InstallDir "$LOCALAPPDATA\${APP_NAME}"
InstallDirRegKey "${REG_ROOT}" "${REG_APP_PATH}" ""

!define MUI_ABORTWARNING
!define MUI_UNABORTWARNING
!define MUI_ICON "icon.ico"
!define MUI_UNICON "icon.ico"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!define MUI_FINISHPAGE_RUN "$INSTDIR\${MAIN_APP_EXE}"
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

Section "MainSection" SEC01
    ${INSTALL_TYPE}
    SetOutPath "$INSTDIR"
    SetOverwrite ifnewer
    File "lf11a-project-frontend.exe"
    File ".env"
    File "README.txt"
    File /nonfatal "icon.png"
    File /nonfatal "icon.ico"
SectionEnd

Section -AdditionalIcons
    ${INSTALL_TYPE}
    SetOutPath "$INSTDIR"
    CreateDirectory "$SMPROGRAMS\${APP_NAME}"
    
    ; Create Start Menu shortcut with icon
    CreateShortCut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "$INSTDIR\${MAIN_APP_EXE}" "" "$INSTDIR\icon.ico" 0 SW_SHOWNORMAL "" "Personnel Management Application"
    
    ; Create Desktop shortcut with icon
    CreateShortCut "$DESKTOP\${APP_NAME}.lnk" "$INSTDIR\${MAIN_APP_EXE}" "" "$INSTDIR\icon.ico" 0 SW_SHOWNORMAL "" "Personnel Management Application"
    
    ; Create Uninstall shortcut
    CreateShortCut "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk" "$INSTDIR\uninstall.exe"
SectionEnd

Section -Post
    ${INSTALL_TYPE}
    WriteUninstaller "$INSTDIR\uninstall.exe"
    WriteRegStr ${REG_ROOT} "${REG_APP_PATH}" "" "$INSTDIR\${MAIN_APP_EXE}"
    WriteRegStr ${REG_ROOT} "${UNINSTALL_PATH}" "DisplayName" "${APP_NAME}"
    WriteRegStr ${REG_ROOT} "${UNINSTALL_PATH}" "UninstallString" "$INSTDIR\uninstall.exe"
    WriteRegStr ${REG_ROOT} "${UNINSTALL_PATH}" "DisplayIcon" "$INSTDIR\${MAIN_APP_EXE}"
    WriteRegStr ${REG_ROOT} "${UNINSTALL_PATH}" "DisplayVersion" "${VERSION}"
    WriteRegStr ${REG_ROOT} "${UNINSTALL_PATH}" "Publisher" "${COMP_NAME}"
SectionEnd

Section Uninstall
    ${INSTALL_TYPE}
    Delete "$INSTDIR\${MAIN_APP_EXE}"
    Delete "$INSTDIR\.env.example"
    Delete "$INSTDIR\.env"
    Delete "$INSTDIR\README.txt"
    Delete "$INSTDIR\icon.png"
    Delete "$INSTDIR\icon.ico"
    Delete "$INSTDIR\uninstall.exe"
    
    ; Remove shortcuts
    Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
    Delete "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk"
    Delete "$DESKTOP\${APP_NAME}.lnk"
    
    RMDir "$SMPROGRAMS\${APP_NAME}"
    RMDir "$INSTDIR"
    
    DeleteRegKey ${REG_ROOT} "${REG_APP_PATH}"
    DeleteRegKey ${REG_ROOT} "${UNINSTALL_PATH}"
SectionEnd
NSIS
        
        # Build the installer
        cd "$WIN_DIR"
        makensis installer.nsi
        cd - > /dev/null
        
        if [ -f "$WIN_DIR/lf11a-project-frontend-${APP_VERSION}-setup.exe" ]; then
            mv "$WIN_DIR/lf11a-project-frontend-${APP_VERSION}-setup.exe" "$BUILD_DIR/"
            echo -e "${GREEN}✓ Windows installer created: lf11a-project-frontend-${APP_VERSION}-setup.exe${NC}"
        fi
    else
        echo -e "${YELLOW}NSIS not found. Only portable Windows build created.${NC}"
        echo -e "${YELLOW}Install NSIS to create installer: sudo apt install nsis${NC}"
    fi
    
    echo -e "${GREEN}✓ Windows executable created in $WIN_DIR${NC}"
else
    echo -e "${RED}✗ Windows build failed${NC}"
fi

# Create summary
echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Package Build Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "Build artifacts in: ${YELLOW}$BUILD_DIR${NC}"
echo ""

# List created packages
PACKAGES_CREATED=0

if [ -f "$BUILD_DIR/${APP_NAME}_${APP_VERSION}_amd64.deb" ]; then
    echo -e "${GREEN}✓ DEB Package:${NC}"
    ls -lh "$BUILD_DIR"/*.deb
    echo -e "  Install: ${YELLOW}sudo apt install ./build/${APP_NAME}_${APP_VERSION}_amd64.deb${NC}"
    echo ""
    PACKAGES_CREATED=$((PACKAGES_CREATED + 1))
fi

if ls "$BUILD_DIR"/*.rpm 1> /dev/null 2>&1; then
    echo -e "${GREEN}✓ RPM Package:${NC}"
    ls -lh "$BUILD_DIR"/*.rpm
    echo -e "  Install: ${YELLOW}sudo dnf install ./build/${APP_NAME}-${APP_VERSION}*.rpm${NC}"
    echo -e "       or: ${YELLOW}sudo rpm -i ./build/${APP_NAME}-${APP_VERSION}*.rpm${NC}"
    echo ""
    PACKAGES_CREATED=$((PACKAGES_CREATED + 1))
fi

if [ -f "$ARCH_DIR/PKGBUILD" ]; then
    echo -e "${GREEN}✓ Arch Linux Package:${NC}"
    echo -e "  Location: ${YELLOW}$ARCH_DIR/PKGBUILD${NC}"
    echo -e "  Build & Install: ${YELLOW}cd $ARCH_DIR && makepkg -si${NC}"
    echo ""
    PACKAGES_CREATED=$((PACKAGES_CREATED + 1))
fi

if [ -f "$BUILD_DIR/${APP_NAME}-${APP_VERSION}-x86_64.AppImage" ]; then
    echo -e "${GREEN}✓ AppImage (Universal Linux):${NC}"
    ls -lh "$BUILD_DIR"/*.AppImage
    echo -e "  Run: ${YELLOW}./$BUILD_DIR/${APP_NAME}-${APP_VERSION}-x86_64.AppImage${NC}"
    echo ""
    PACKAGES_CREATED=$((PACKAGES_CREATED + 1))
fi

if [ -f "$BUILD_DIR/lf11a-project-frontend-${APP_VERSION}-setup.exe" ]; then
    echo -e "${GREEN}✓ Windows Installer:${NC}"
    ls -lh "$BUILD_DIR"/*-setup.exe
    echo -e "  Run the installer on Windows to install the application"
    echo ""
    PACKAGES_CREATED=$((PACKAGES_CREATED + 1))
elif [ -f "$WIN_DIR/${APP_NAME}.exe" ]; then
    echo -e "${GREEN}✓ Windows Portable Executable:${NC}"
    ls -lh "$WIN_DIR"/*.exe
    echo -e "  Location: ${YELLOW}$WIN_DIR/${NC}"
    echo -e "  Note: Install GTK4 runtime first"
    echo ""
    PACKAGES_CREATED=$((PACKAGES_CREATED + 1))
fi

if [ $PACKAGES_CREATED -eq 0 ]; then
    echo -e "${RED}No packages were created. Check error messages above.${NC}"
    echo ""
fi

echo -e "${YELLOW}Note:${NC} Packages are ready for distribution."
echo -e "      Do NOT run this script with sudo - it only creates packages."
echo ""

# If release mode, create GitHub release
if [ "$RELEASE_MODE" = true ]; then
    create_github_release
fi
