// src-tauri/src/core/patches.rs

use crate::core::version::KernelVersion;
use serde::{Deserialize, Serialize};

/// Represents a patch that can be applied to a kernel
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Patch {
    /// Name of the patch
    pub name: String,
    /// Description of what the patch does
    pub description: String,
    /// URL or path to the patch file
    pub source: String,
    /// Whether this patch is upstream or external
    pub is_upstream: bool,
}

impl Patch {
    pub fn new(name: &str, description: &str, source: &str, is_upstream: bool) -> Self {
        Patch {
            name: name.to_string(),
            description: description.to_string(),
            source: source.to_string(),
            is_upstream,
        }
    }
}

/// Resolver for selecting patches based on kernel version
pub struct PatchResolver;

impl PatchResolver {
    /// Get all available patches for a specific kernel version
    pub fn get_patches_for_version(version: KernelVersion) -> Vec<Patch> {
        match version {
            KernelVersion::V6_6_Lts => Self::get_v6_6_patches(),
            KernelVersion::V6_17 => Self::get_v6_17_patches(),
        }
    }

    /// Get gaming/performance patches for 6.6 LTS
    fn get_v6_6_patches() -> Vec<Patch> {
        vec![
            Patch::new(
                "BORE",
                "Burst-Oriented Response Enhancer scheduler - reduces latency for gaming",
                "https://github.com/firelzrd/bore-scheduler/6.6",
                false,
            ),
            Patch::new(
                "BBRv3",
                "TCP BBR v3 congestion control - improves network performance",
                "https://github.com/google/bbr/v3-6.6",
                false,
            ),
            Patch::new(
                "FUTEX2",
                "Futex2 system call - already in 6.6, no external patch needed",
                "upstream",
                true,
            ),
            Patch::new(
                "PREEMPT_RT",
                "Real-time preemption patches for low latency",
                "https://kernel.org/pub/linux/kernel/projects/rt/6.6/",
                false,
            ),
        ]
    }

    /// Get gaming/performance patches for 6.17
    fn get_v6_17_patches() -> Vec<Patch> {
        vec![
            Patch::new(
                "BORE",
                "Burst-Oriented Response Enhancer scheduler - reduces latency for gaming (6.17 compatible)",
                "https://github.com/firelzrd/bore-scheduler/6.17",
                false,
            ),
            Patch::new(
                "BBRv3",
                "TCP BBR v3 congestion control - many improvements in 6.17 mainline",
                "upstream",
                true,
            ),
            Patch::new(
                "FUTEX2",
                "Futex2 system call - upstream in mainline since 5.16",
                "upstream",
                true,
            ),
            // Note: PREEMPT_RT patches for 6.17 may not be available yet, so we don't include them
            // unless they are confirmed compatible
        ]
    }

    /// Get patches filtered by specific features
    pub fn get_patches_by_feature(version: KernelVersion, feature: &str) -> Vec<Patch> {
        Self::get_patches_for_version(version)
            .into_iter()
            .filter(|p| p.name.to_lowercase().contains(&feature.to_lowercase()))
            .collect()
    }

    /// Check if a specific patch is available for a version
    pub fn is_patch_available(version: KernelVersion, patch_name: &str) -> bool {
        Self::get_patches_for_version(version)
            .iter()
            .any(|p| p.name.eq_ignore_ascii_case(patch_name))
    }

    /// Get only external (non-upstream) patches that need to be applied
    pub fn get_external_patches(version: KernelVersion) -> Vec<Patch> {
        Self::get_patches_for_version(version)
            .into_iter()
            .filter(|p| !p.is_upstream)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_patches_v6_6() {
        let patches = PatchResolver::get_patches_for_version(KernelVersion::V6_6_Lts);
        assert!(!patches.is_empty());
        assert!(patches.iter().any(|p| p.name == "BORE"));
        assert!(patches.iter().any(|p| p.name == "BBRv3"));
        assert!(patches.iter().any(|p| p.name == "FUTEX2"));
    }

    #[test]
    fn test_get_patches_v6_17() {
        let patches = PatchResolver::get_patches_for_version(KernelVersion::V6_17);
        assert!(!patches.is_empty());
        assert!(patches.iter().any(|p| p.name == "BORE"));
        assert!(patches.iter().any(|p| p.name == "FUTEX2"));
    }

    #[test]
    fn test_patch_availability() {
        // BORE should be available in both versions
        assert!(PatchResolver::is_patch_available(KernelVersion::V6_6_Lts, "BORE"));
        assert!(PatchResolver::is_patch_available(KernelVersion::V6_17, "BORE"));

        // PREEMPT_RT only in 6.6
        assert!(PatchResolver::is_patch_available(KernelVersion::V6_6_Lts, "PREEMPT_RT"));
        assert!(!PatchResolver::is_patch_available(KernelVersion::V6_17, "PREEMPT_RT"));
    }

    #[test]
    fn test_external_patches() {
        let v6_6_external = PatchResolver::get_external_patches(KernelVersion::V6_6_Lts);
        let v6_17_external = PatchResolver::get_external_patches(KernelVersion::V6_17);

        // BORE should be external in both versions
        assert!(v6_6_external.iter().any(|p| p.name == "BORE"));
        assert!(v6_17_external.iter().any(|p| p.name == "BORE"));

        // FUTEX2 should not be in external (it's upstream)
        assert!(!v6_6_external.iter().any(|p| p.name == "FUTEX2"));
        assert!(!v6_17_external.iter().any(|p| p.name == "FUTEX2"));
    }

    #[test]
    fn test_get_patches_by_feature() {
        let bore_patches = PatchResolver::get_patches_by_feature(KernelVersion::V6_6_Lts, "bore");
        assert_eq!(bore_patches.len(), 1);
        assert_eq!(bore_patches[0].name, "BORE");
    }

    #[test]
    fn test_upstream_vs_external() {
        let patches_6_6 = PatchResolver::get_patches_for_version(KernelVersion::V6_6_Lts);
        let futex2_patch = patches_6_6.iter().find(|p| p.name == "FUTEX2").unwrap();
        assert!(futex2_patch.is_upstream);

        let bore_patch = patches_6_6.iter().find(|p| p.name == "BORE").unwrap();
        assert!(!bore_patch.is_upstream);
    }
}
