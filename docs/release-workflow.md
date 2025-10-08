# Release Workflow Testing Guide

This document describes how to test the release workflow for KernelForge.

## Workflow Triggers

The release workflow (`.github/workflows/release.yml`) is triggered by:

1. **Tag Push**: Pushing a tag starting with `v` (e.g., `v1.0.0`, `v0.1.0`)
2. **Manual Dispatch**: Via GitHub Actions UI using `workflow_dispatch`

## Testing the Workflow

### Manual Testing (Recommended for Pre-Release)

1. Go to the Actions tab in the GitHub repository
2. Select "Release Build" workflow
3. Click "Run workflow" button
4. Select the branch to run from
5. Click "Run workflow"

This will build the packages without creating a release, useful for testing the build process.

### Testing with a Pre-Release Tag

To test the full release process including artifact upload:

```bash
git tag v0.1.0-alpha
git push origin v0.1.0-alpha
```

This will:
- Trigger the workflow
- Build DEB and RPM packages
- Create a draft release on GitHub
- Upload the packages as release assets

## Expected Artifacts

After a successful build, the following artifacts should be created:

1. **DEB Package**: `kernelforge_<version>_amd64.deb`
2. **RPM Package**: `kernelforge-<version>.x86_64.rpm`

## Workflow Steps

The workflow performs these steps:

1. **Checkout**: Clones the repository
2. **Install Rust**: Sets up the Rust toolchain
3. **Rust Cache**: Caches Rust dependencies for faster builds
4. **Install Dependencies**: Installs GTK and WebKit libraries
5. **Build Tauri App**: Uses `tauri-action` to build and package the app
6. **Upload Artifacts**: Uploads DEB and RPM to the GitHub Release

## Build Dependencies

The workflow installs these system dependencies:

- `libwebkit2gtk-4.1-dev`: WebKit2GTK development files
- `libgtk-3-dev`: GTK3 development files
- `libayatana-appindicator3-dev`: System tray support
- `librsvg2-dev`: SVG support
- `patchelf`: For binary patching

## Troubleshooting

### Build Fails at Dependency Installation

Check if the package names are correct for the Ubuntu version being used. The workflow uses `ubuntu-latest` which may change over time.

### Build Fails During Compilation

Check the build logs for specific Rust compilation errors. Common issues:
- Missing build dependencies
- Incorrect Tauri configuration
- Frontend build issues

### Artifacts Not Uploaded

Ensure:
- `GITHUB_TOKEN` has proper permissions
- Release is being created as draft
- Tag format is correct (starts with `v`)

## Future Enhancements

Items noted for future implementation:

1. **Package Signing**: Add GPG signing for packages
2. **Multi-Architecture**: Build for ARM64 in addition to x86_64
3. **Flatpak Support**: Add Flatpak bundling (separate PR)
4. **Testing**: Add automated package installation tests

## References

- [Tauri Action Documentation](https://github.com/tauri-apps/tauri-action)
- [Tauri Bundle Documentation](https://tauri.app/v1/guides/building/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
