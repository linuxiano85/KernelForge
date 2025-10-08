// src-tauri/src/core/version.rs

use serde::{Deserialize, Serialize};

/// Represents supported Linux kernel versions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum KernelVersion {
    /// Linux kernel 6.6 LTS - stable, long-term support
    V6_6_Lts,
    /// Linux kernel 6.17 - latest features
    V6_17,
}

impl KernelVersion {
    /// Returns the default target kernel version (6.6 LTS for stability)
    pub fn default_target() -> Self {
        KernelVersion::V6_6_Lts
    }

    /// Parse a kernel version from a string
    /// 
    /// # Examples
    /// ```
    /// use kernelforge::core::version::KernelVersion;
    /// 
    /// assert_eq!(KernelVersion::from_string("6.6"), Some(KernelVersion::V6_6_Lts));
    /// assert_eq!(KernelVersion::from_string("6.6-lts"), Some(KernelVersion::V6_6_Lts));
    /// assert_eq!(KernelVersion::from_string("6.17"), Some(KernelVersion::V6_17));
    /// assert_eq!(KernelVersion::from_string("invalid"), None);
    /// ```
    pub fn from_string(s: &str) -> Option<Self> {
        let normalized = s.to_lowercase().trim().to_string();
        match normalized.as_str() {
            "6.6" | "6.6-lts" | "6.6_lts" | "v6.6" | "v6_6_lts" => Some(KernelVersion::V6_6_Lts),
            "6.17" | "v6.17" | "v6_17" => Some(KernelVersion::V6_17),
            _ => None,
        }
    }

    /// Get the version string representation
    pub fn to_string(&self) -> &'static str {
        match self {
            KernelVersion::V6_6_Lts => "6.6-LTS",
            KernelVersion::V6_17 => "6.17",
        }
    }

    /// Get the full version name
    pub fn full_name(&self) -> &'static str {
        match self {
            KernelVersion::V6_6_Lts => "Linux Kernel 6.6 LTS",
            KernelVersion::V6_17 => "Linux Kernel 6.17",
        }
    }

    /// Check if this version is an LTS release
    pub fn is_lts(&self) -> bool {
        matches!(self, KernelVersion::V6_6_Lts)
    }

    /// Get major version number
    pub fn major(&self) -> u32 {
        6
    }

    /// Get minor version number
    pub fn minor(&self) -> u32 {
        match self {
            KernelVersion::V6_6_Lts => 6,
            KernelVersion::V6_17 => 17,
        }
    }
}

impl std::fmt::Display for KernelVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Default for KernelVersion {
    fn default() -> Self {
        Self::default_target()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_target() {
        assert_eq!(KernelVersion::default_target(), KernelVersion::V6_6_Lts);
    }

    #[test]
    fn test_from_string_v6_6() {
        assert_eq!(KernelVersion::from_string("6.6"), Some(KernelVersion::V6_6_Lts));
        assert_eq!(KernelVersion::from_string("6.6-lts"), Some(KernelVersion::V6_6_Lts));
        assert_eq!(KernelVersion::from_string("6.6_lts"), Some(KernelVersion::V6_6_Lts));
        assert_eq!(KernelVersion::from_string("v6.6"), Some(KernelVersion::V6_6_Lts));
        assert_eq!(KernelVersion::from_string("v6_6_lts"), Some(KernelVersion::V6_6_Lts));
        assert_eq!(KernelVersion::from_string("V6.6"), Some(KernelVersion::V6_6_Lts));
    }

    #[test]
    fn test_from_string_v6_17() {
        assert_eq!(KernelVersion::from_string("6.17"), Some(KernelVersion::V6_17));
        assert_eq!(KernelVersion::from_string("v6.17"), Some(KernelVersion::V6_17));
        assert_eq!(KernelVersion::from_string("v6_17"), Some(KernelVersion::V6_17));
        assert_eq!(KernelVersion::from_string("V6.17"), Some(KernelVersion::V6_17));
    }

    #[test]
    fn test_from_string_invalid() {
        assert_eq!(KernelVersion::from_string(""), None);
        assert_eq!(KernelVersion::from_string("invalid"), None);
        assert_eq!(KernelVersion::from_string("5.10"), None);
        assert_eq!(KernelVersion::from_string("7.0"), None);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(KernelVersion::V6_6_Lts.to_string(), "6.6-LTS");
        assert_eq!(KernelVersion::V6_17.to_string(), "6.17");
    }

    #[test]
    fn test_is_lts() {
        assert!(KernelVersion::V6_6_Lts.is_lts());
        assert!(!KernelVersion::V6_17.is_lts());
    }

    #[test]
    fn test_version_numbers() {
        assert_eq!(KernelVersion::V6_6_Lts.major(), 6);
        assert_eq!(KernelVersion::V6_6_Lts.minor(), 6);
        assert_eq!(KernelVersion::V6_17.major(), 6);
        assert_eq!(KernelVersion::V6_17.minor(), 17);
    }
}
