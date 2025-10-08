# Kernel Version Catalog API

This module provides dynamic kernel version discovery and management, fetching available versions from kernel.org.

## Features

- **Dynamic Version Discovery**: Fetches kernel versions from kernel.org's official releases.json API
- **Caching**: Stores versions locally in `$XDG_CACHE_HOME/kernelforge/versions.json` with 24-hour TTL
- **Offline Fallback**: Works offline with a predefined set of known kernel versions
- **Semver Support**: Parses and normalizes version strings for comparison
- **Metadata**: Includes channel (stable/mainline/longterm), release date, and EOL status

## API Usage

### Async API

```rust
use kernelforge::version_catalog;

// List available versions (uses cache if valid)
let versions = version_catalog::list_available_versions(false).await?;

// Force refresh (bypasses cache)
let versions = version_catalog::list_available_versions(true).await?;

// Access version metadata
for version in versions {
    println!("Version: {}", version.version);
    println!("Channel: {}", version.channel);
    println!("EOL: {}", version.eol);
    if let Some(released) = version.released {
        println!("Released: {}", released);
    }
}
```

### Blocking API

```rust
use kernelforge::version_catalog;

// Blocking version (uses tokio runtime internally)
let versions = version_catalog::list_available_versions_blocking(false)?;
```

## Data Structure

```rust
pub struct KernelVersion {
    /// Semantic version (e.g., "6.6.0", "6.17.3")
    pub version: String,
    /// Parsed semver for comparison
    pub semver: Option<Version>,
    /// Channel: stable, mainline, longterm
    pub channel: String,
    /// Release date (ISO format)
    pub released: Option<String>,
    /// End of life status
    pub eol: bool,
}
```

## Version Sources

1. **Primary**: https://www.kernel.org/releases.json
2. **Fallback**: Hardcoded set of known versions (6.6.0 - 6.17.0)

## Cache Location

- Linux/BSD: `$XDG_CACHE_HOME/kernelforge/versions.json` (usually `~/.cache/kernelforge/`)
- macOS: `~/Library/Caches/kernelforge/versions.json`
- Windows: `%LOCALAPPDATA%\kernelforge\cache\versions.json`

## Cache Invalidation

The cache is automatically invalidated after 24 hours. To force a refresh:

```rust
let versions = version_catalog::list_available_versions(true).await?;
```

## Testing

Run the tests with:

```bash
cd src-tauri
cargo test
```

Run the example to see it in action:

```bash
cd src-tauri
cargo run --example list_versions
```

## Supported Version Range

The system supports all kernel versions from 6.6.x through 6.17.x and beyond, dynamically fetching new versions as they are released.
