// src-tauri/tests/version_catalog_integration.rs

use kernelforge::version_catalog;
use tempfile::TempDir;

#[tokio::test]
async fn test_version_catalog_with_custom_cache() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Set the XDG_CACHE_HOME to our temp directory
    std::env::set_var("XDG_CACHE_HOME", temp_path.to_str().unwrap());
    
    // First call should create cache
    let versions1 = version_catalog::list_available_versions(false).await;
    assert!(versions1.is_ok());
    let versions1 = versions1.unwrap();
    assert!(!versions1.is_empty());
    
    // Check that cache file was created
    let cache_path = temp_path.join("kernelforge").join("versions.json");
    assert!(cache_path.exists());
    
    // Second call should use cache
    let versions2 = version_catalog::list_available_versions(false).await;
    assert!(versions2.is_ok());
    let versions2 = versions2.unwrap();
    
    // Should return the same versions
    assert_eq!(versions1.len(), versions2.len());
    
    // Force refresh should bypass cache
    let versions3 = version_catalog::list_available_versions(true).await;
    assert!(versions3.is_ok());
    let versions3 = versions3.unwrap();
    assert!(!versions3.is_empty());
    
    // Clean up
    std::env::remove_var("XDG_CACHE_HOME");
}

#[test]
fn test_blocking_api() {
    let result = version_catalog::list_available_versions_blocking(false);
    assert!(result.is_ok());
    let versions = result.unwrap();
    assert!(!versions.is_empty());
    
    // Check that versions include the expected range
    let has_6_6 = versions.iter().any(|v| v.version.starts_with("6.6"));
    let has_recent = versions.iter().any(|v| {
        v.version.starts_with("6.10") || 
        v.version.starts_with("6.11") ||
        v.version.starts_with("6.12")
    });
    
    assert!(has_6_6, "Should include 6.6.x version");
    assert!(has_recent, "Should include recent versions");
}

#[tokio::test]
async fn test_version_metadata() {
    let versions = version_catalog::list_available_versions(false).await.unwrap();
    
    // Check that all versions have required metadata
    for version in &versions {
        assert!(!version.version.is_empty(), "Version should not be empty");
        assert!(!version.channel.is_empty(), "Channel should not be empty");
        
        // Check channel is one of the expected values
        assert!(
            version.channel == "stable" || 
            version.channel == "mainline" || 
            version.channel == "longterm" ||
            version.channel == "other",
            "Channel should be valid: {}", version.channel
        );
    }
}

#[test]
fn test_semver_comparison() {
    use semver::Version;
    
    // Test that semver parsing works for kernel versions
    let v1 = Version::parse("6.6.0").unwrap();
    let v2 = Version::parse("6.17.0").unwrap();
    
    assert!(v2 > v1);
    assert!(v1 < v2);
    assert_eq!(v1.major, 6);
    assert_eq!(v1.minor, 6);
    assert_eq!(v2.minor, 17);
}
