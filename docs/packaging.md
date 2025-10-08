# KernelForge Packaging Guide

This document provides instructions for installing KernelForge from DEB and RPM packages, along with runtime dependencies for different Linux distributions.

## Installation

### Debian/Ubuntu

Download the `.deb` package from the [releases page](https://github.com/linuxiano85/KernelForge/releases) and install:

```bash
sudo dpkg -i kernelforge_*_amd64.deb
sudo apt-get install -f  # Install any missing dependencies
```

Alternatively, you can install dependencies first:

```bash
sudo apt-get install libwebkit2gtk-4.1-0 libgtk-3-0
sudo dpkg -i kernelforge_*_amd64.deb
```

### Fedora/RHEL/CentOS

Download the `.rpm` package from the [releases page](https://github.com/linuxiano85/KernelForge/releases) and install:

```bash
sudo rpm -i kernelforge-*.x86_64.rpm
```

Or using DNF:

```bash
sudo dnf install kernelforge-*.x86_64.rpm
```

## Runtime Dependencies

### Debian/Ubuntu Family

Required packages:
- `libwebkit2gtk-4.1-0` - WebKit2GTK for rendering the UI
- `libgtk-3-0` - GTK3 toolkit for the window system

Install all dependencies:
```bash
sudo apt-get install libwebkit2gtk-4.1-0 libgtk-3-0
```

### Fedora/RHEL/CentOS Family

Required packages:
- `webkit2gtk4.1` - WebKit2GTK for rendering the UI
- `gtk3` - GTK3 toolkit for the window system

Install all dependencies:
```bash
sudo dnf install webkit2gtk4.1 gtk3
```

### Arch Linux

For Arch users who want to install from the DEB/RPM packages, you can use tools like `debtap` to convert packages, though building from source or using the AUR (when available) is recommended.

Required packages for building/running:
- `webkit2gtk`
- `gtk3`

## Uninstalling

### Debian/Ubuntu

```bash
sudo apt-get remove kernelforge
```

### Fedora/RHEL/CentOS

```bash
sudo rpm -e kernelforge
```

Or using DNF:

```bash
sudo dnf remove kernelforge
```

## Desktop Integration

The packages install a desktop entry file that allows you to launch KernelForge from your application menu. The application will appear under the "System" or "Utilities" category depending on your desktop environment.

## Package Signing

**Note:** Current packages are not GPG signed. Future releases will include signed packages for enhanced security. Instructions for verifying signatures will be added when signing is implemented.

## Building from Source

If you prefer to build KernelForge from source instead of using pre-built packages:

1. Install Rust and the Tauri CLI:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   cargo install tauri-cli
   ```

2. Install build dependencies (same as runtime dependencies above)

3. Clone and build:
   ```bash
   git clone https://github.com/linuxiano85/KernelForge.git
   cd KernelForge
   cargo tauri build
   ```

4. The built packages will be in `src-tauri/target/release/bundle/`

## Troubleshooting

### Missing Dependencies

If you encounter errors about missing libraries, ensure all runtime dependencies are installed:

**Debian/Ubuntu:**
```bash
sudo apt-get update
sudo apt-get install libwebkit2gtk-4.1-0 libgtk-3-0
```

**Fedora/RHEL:**
```bash
sudo dnf install webkit2gtk4.1 gtk3
```

### Application Won't Start

1. Check if the dependencies are installed
2. Try running from terminal to see error messages: `kernelforge`
3. Check system logs: `journalctl -xe`

### Desktop Entry Not Showing

If the application doesn't appear in your menu:

1. Update the desktop database:
   ```bash
   sudo update-desktop-database
   ```

2. Check if the desktop file exists:
   ```bash
   ls -la /usr/share/applications/kernelforge.desktop
   ```

## Support

For issues with packaging or installation, please open an issue on the [GitHub repository](https://github.com/linuxiano85/KernelForge/issues).
