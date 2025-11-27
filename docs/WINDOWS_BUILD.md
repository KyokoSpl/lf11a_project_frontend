# Windows Build Guide

## Current Status

Cross-compiling GTK4 applications from Linux to Windows is **extremely complex** and not officially supported by the GTK project. The issue is that GTK4 requires GLib, Cairo, Pango, and many other C libraries that need to be cross-compiled with all their dependencies.

## Why Windows Builds Fail

The error you see:
```
error: failed to run custom build command for `glib-sys v0.21.2`
pkg-config has not been configured to support cross-compilation
```

This happens because:
1. GTK4's Rust bindings use `pkg-config` to find system libraries
2. Cross-compiling requires Windows versions of all GTK4 libraries
3. These libraries are not available in Fedora's MinGW packages (only GTK3 is available)

## Solutions

### Option 1: Native Windows Build (Recommended)

Build directly on Windows with:
1. Install [MSYS2](https://www.msys2.org/)
2. Install GTK4 and Rust:
   ```bash
   pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-rust
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

### Option 2: Wine for Testing

The package script installs Wine (already installed). You can test Linux builds under Wine:
```bash
wine target/release/lf11a_project_frontend.exe
```

**Note:** GTK4 applications may not work perfectly under Wine.

### Option 3: Docker/Podman Windows Container

Use a Windows container for building (requires Windows host or Windows Server):
```dockerfile
FROM mcr.microsoft.com/windows/servercore:ltsc2022
# Install MSYS2 and build tools
```

### Option 4: GitHub Actions

Use GitHub Actions with Windows runners:
```yaml
- name: Build Windows
  runs-on: windows-latest
  steps:
    - uses: actions/checkout@v4
    - uses: msys2/setup-msys2@v2
      with:
        update: true
        install: >-
          mingw-w64-x86_64-gtk4
          mingw-w64-x86_64-rust
    - run: cargo build --release
```

## Current Package Script Behavior

The `scripts/package.sh` now:
- ✅ Creates DEB packages (Debian/Ubuntu)
- ✅ Creates RPM packages (Fedora/RHEL)
- ✅ Creates AppImage (Universal Linux)
- ✅ Creates PKGBUILD (Arch Linux)
- ❌ Skips Windows (with helpful error message)

## Future Improvements

If Windows builds are critical, consider:
1. Setting up a Windows VM for builds
2. Using CI/CD with Windows runners
3. Providing installation instructions for MSYS2 instead
4. Creating a simple launcher script that installs dependencies

## Alternative: Distribution Without Building

You can also:
1. Provide MSYS2 installation instructions
2. Let users build from source on Windows
3. Focus on Linux distribution (where GTK4 works best)
