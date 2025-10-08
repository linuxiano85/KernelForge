// src-tauri/src/core/version_catalog.rs

use anyhow::{Context, Result};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

/// Represents a kernel version with metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KernelVersion {
    /// Semantic version (e.g., "6.6.0", "6.17.3")
    pub version: String,
    /// Parsed semver for comparison
    #[serde(skip)]
    pub semver: Option<Version>,
    /// Channel: stable, mainline, longterm
    pub channel: String,
    /// Release date
    pub released: Option<String>,
    /// End of life status
    pub eol: bool,
}

/// Cache for kernel versions
#[derive(Debug, Serialize, Deserialize)]
struct VersionCache {
    versions: Vec<KernelVersion>,
    cached_at: SystemTime,
}

/// Kernel.org releases.json response structure
#[derive(Debug, Deserialize)]
struct KernelOrgRelease {
    version: String,
    moniker: String,
    released: Option<KernelOrgDate>,
    #[serde(default)]
    iseol: bool,
}

#[derive(Debug, Deserialize)]
struct KernelOrgDate {
    isodate: String,
}

#[derive(Debug, Deserialize)]
struct KernelOrgReleasesResponse {
    releases: Vec<KernelOrgRelease>,
}

const CACHE_TTL: Duration = Duration::from_secs(24 * 60 * 60); // 24 hours

/// Gets the cache directory for KernelForge
fn get_cache_dir() -> Result<PathBuf> {
    let cache_dir = if let Ok(xdg_cache) = std::env::var("XDG_CACHE_HOME") {
        PathBuf::from(xdg_cache)
    } else {
        let dirs = directories::BaseDirs::new()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
        dirs.cache_dir().to_path_buf()
    };
    
    let kernelforge_cache = cache_dir.join("kernelforge");
    fs::create_dir_all(&kernelforge_cache)
        .context("Failed to create cache directory")?;
    
    Ok(kernelforge_cache)
}

/// Gets the path to the versions cache file
fn get_cache_path() -> Result<PathBuf> {
    Ok(get_cache_dir()?.join("versions.json"))
}

/// Checks if the cache is still valid
fn is_cache_valid(cache: &VersionCache) -> bool {
    if let Ok(elapsed) = cache.cached_at.elapsed() {
        elapsed < CACHE_TTL
    } else {
        false
    }
}

/// Reads the version cache from disk
fn read_cache() -> Result<Option<VersionCache>> {
    let cache_path = get_cache_path()?;
    
    if !cache_path.exists() {
        return Ok(None);
    }
    
    let content = fs::read_to_string(&cache_path)
        .context("Failed to read cache file")?;
    
    let cache: VersionCache = serde_json::from_str(&content)
        .context("Failed to parse cache file")?;
    
    if is_cache_valid(&cache) {
        Ok(Some(cache))
    } else {
        Ok(None)
    }
}

/// Writes the version cache to disk
fn write_cache(versions: Vec<KernelVersion>) -> Result<()> {
    let cache = VersionCache {
        versions,
        cached_at: SystemTime::now(),
    };
    
    let cache_path = get_cache_path()?;
    let content = serde_json::to_string_pretty(&cache)
        .context("Failed to serialize cache")?;
    
    fs::write(&cache_path, content)
        .context("Failed to write cache file")?;
    
    Ok(())
}

/// Fetches kernel versions from kernel.org releases.json
async fn fetch_from_kernel_org() -> Result<Vec<KernelVersion>> {
    let url = "https://www.kernel.org/releases.json";
    
    let response = reqwest::get(url)
        .await
        .context("Failed to fetch from kernel.org")?;
    
    let releases_response: KernelOrgReleasesResponse = response
        .json()
        .await
        .context("Failed to parse kernel.org response")?;
    
    let mut versions = Vec::new();
    
    for release in releases_response.releases {
        let channel = match release.moniker.as_str() {
            "mainline" => "mainline",
            "stable" => "stable",
            "longterm" => "longterm",
            _ => "other",
        };
        
        let released = release.released.map(|d| d.isodate);
        
        // Parse semver
        let semver = Version::parse(&release.version).ok();
        
        let kernel_version = KernelVersion {
            version: release.version,
            semver,
            channel: channel.to_string(),
            released,
            eol: release.iseol,
        };
        
        versions.push(kernel_version);
    }
    
    Ok(versions)
}

/// Fetches kernel versions with offline fallback
async fn fetch_versions_internal() -> Result<Vec<KernelVersion>> {
    match fetch_from_kernel_org().await {
        Ok(versions) => Ok(versions),
        Err(e) => {
            eprintln!("Failed to fetch from kernel.org: {}", e);
            // Fallback: return a minimal set of known versions
            Ok(get_fallback_versions())
        }
    }
}

/// Provides fallback versions when offline or kernel.org is unavailable
fn get_fallback_versions() -> Vec<KernelVersion> {
    vec![
        KernelVersion {
            version: "6.6.0".to_string(),
            semver: Version::parse("6.6.0").ok(),
            channel: "longterm".to_string(),
            released: Some("2023-10-29".to_string()),
            eol: false,
        },
        KernelVersion {
            version: "6.7.0".to_string(),
            semver: Version::parse("6.7.0").ok(),
            channel: "stable".to_string(),
            released: Some("2024-01-07".to_string()),
            eol: false,
        },
        KernelVersion {
            version: "6.8.0".to_string(),
            semver: Version::parse("6.8.0").ok(),
            channel: "stable".to_string(),
            released: Some("2024-03-10".to_string()),
            eol: false,
        },
        KernelVersion {
            version: "6.9.0".to_string(),
            semver: Version::parse("6.9.0").ok(),
            channel: "stable".to_string(),
            released: Some("2024-05-12".to_string()),
            eol: false,
        },
        KernelVersion {
            version: "6.10.0".to_string(),
            semver: Version::parse("6.10.0").ok(),
            channel: "stable".to_string(),
            released: Some("2024-07-14".to_string()),
            eol: false,
        },
        KernelVersion {
            version: "6.11.0".to_string(),
            semver: Version::parse("6.11.0").ok(),
            channel: "stable".to_string(),
            released: Some("2024-09-15".to_string()),
            eol: false,
        },
        KernelVersion {
            version: "6.12.0".to_string(),
            semver: Version::parse("6.12.0").ok(),
            channel: "stable".to_string(),
            released: Some("2024-11-17".to_string()),
            eol: false,
        },
        KernelVersion {
            version: "6.13.0".to_string(),
            semver: Version::parse("6.13.0").ok(),
            channel: "mainline".to_string(),
            released: Some("2025-01-19".to_string()),
            eol: false,
        },
        KernelVersion {
            version: "6.14.0".to_string(),
            semver: Version::parse("6.14.0").ok(),
            channel: "mainline".to_string(),
            released: Some("2025-03-23".to_string()),
            eol: false,
        },
        KernelVersion {
            version: "6.15.0".to_string(),
            semver: Version::parse("6.15.0").ok(),
            channel: "mainline".to_string(),
            released: Some("2025-05-25".to_string()),
            eol: false,
        },
        KernelVersion {
            version: "6.16.0".to_string(),
            semver: Version::parse("6.16.0").ok(),
            channel: "mainline".to_string(),
            released: Some("2025-07-27".to_string()),
            eol: false,
        },
        KernelVersion {
            version: "6.17.0".to_string(),
            semver: Version::parse("6.17.0").ok(),
            channel: "mainline".to_string(),
            released: Some("2025-09-28".to_string()),
            eol: false,
        },
    ]
}

/// Lists available kernel versions
/// 
/// # Arguments
/// * `force_refresh` - If true, bypasses the cache and fetches fresh data
/// 
/// # Returns
/// A vector of `KernelVersion` objects with metadata
pub async fn list_available_versions(force_refresh: bool) -> Result<Vec<KernelVersion>> {
    // Try to use cache if not forcing refresh
    if !force_refresh {
        if let Ok(Some(cache)) = read_cache() {
            return Ok(cache.versions);
        }
    }
    
    // Fetch fresh versions
    let versions = fetch_versions_internal().await?;
    
    // Write to cache
    if let Err(e) = write_cache(versions.clone()) {
        eprintln!("Warning: Failed to write cache: {}", e);
    }
    
    Ok(versions)
}

/// Synchronous version of list_available_versions using blocking runtime
pub fn list_available_versions_blocking(force_refresh: bool) -> Result<Vec<KernelVersion>> {
    tokio::runtime::Runtime::new()?.block_on(list_available_versions(force_refresh))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_versions() {
        let versions = get_fallback_versions();
        assert!(!versions.is_empty());
        
        // Check that versions include 6.6 and 6.17
        let has_6_6 = versions.iter().any(|v| v.version.starts_with("6.6"));
        let has_6_17 = versions.iter().any(|v| v.version.starts_with("6.17"));
        
        assert!(has_6_6, "Should include 6.6.x version");
        assert!(has_6_17, "Should include 6.17.x version");
    }

    #[test]
    fn test_semver_parsing() {
        let version = KernelVersion {
            version: "6.6.0".to_string(),
            semver: Version::parse("6.6.0").ok(),
            channel: "stable".to_string(),
            released: None,
            eol: false,
        };
        
        assert!(version.semver.is_some());
        assert_eq!(version.semver.unwrap().to_string(), "6.6.0");
    }

    #[tokio::test]
    async fn test_list_available_versions_with_fallback() {
        // This test will use fallback if kernel.org is not accessible
        let result = list_available_versions(true).await;
        assert!(result.is_ok());
        
        let versions = result.unwrap();
        assert!(!versions.is_empty());
    }

    #[test]
    fn test_cache_directory_creation() {
        let cache_dir = get_cache_dir();
        assert!(cache_dir.is_ok());
    }
}
