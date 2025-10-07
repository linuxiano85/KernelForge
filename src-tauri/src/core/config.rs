// src-tauri/src/core/config.rs

use std::collections::HashMap;

/// KernelConfig builder for generating Linux kernel .config files
/// Manages kernel configuration options with set/unset operations
pub struct KernelConfig {
    /// Map of config option names to their values (or None for unset)
    config_map: HashMap<String, Option<String>>,
}

impl KernelConfig {
    /// Creates a new empty KernelConfig
    pub fn new() -> Self {
        KernelConfig {
            config_map: HashMap::new(),
        }
    }

    /// Sets a configuration option to a specific value
    /// 
    /// # Arguments
    /// * `name` - Config option name (e.g., "CONFIG_X86_64")
    /// * `value` - Value to set (e.g., "y", "m", or a string/number)
    /// 
    /// # Examples
    /// ```
    /// use kernelforge::core::config::KernelConfig;
    /// let mut config = KernelConfig::new();
    /// config.set("CONFIG_X86_64", "y");
    /// config.set("CONFIG_HZ", "1000");
    /// ```
    pub fn set(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.config_map.insert(name.into(), Some(value.into()));
    }

    /// Explicitly unsets (disables) a configuration option
    /// This generates "# CONFIG_XXX is not set" in the output
    /// 
    /// # Arguments
    /// * `name` - Config option name to unset
    /// 
    /// # Examples
    /// ```
    /// use kernelforge::core::config::KernelConfig;
    /// let mut config = KernelConfig::new();
    /// config.unset("CONFIG_ARM");
    /// ```
    pub fn unset(&mut self, name: impl Into<String>) {
        self.config_map.insert(name.into(), None);
    }

    /// Generates the .config file content as a String
    /// 
    /// Format:
    /// - Set options: CONFIG_NAME=value
    /// - Unset options: # CONFIG_NAME is not set
    /// 
    /// # Returns
    /// A String containing the complete .config content
    pub fn emit(&self) -> String {
        let mut lines = Vec::new();
        
        // Sort keys for deterministic output
        let mut keys: Vec<&String> = self.config_map.keys().collect();
        keys.sort();
        
        for key in keys {
            if let Some(value_opt) = self.config_map.get(key) {
                match value_opt {
                    Some(value) => {
                        lines.push(format!("{}={}", key, value));
                    }
                    None => {
                        lines.push(format!("# {} is not set", key));
                    }
                }
            }
        }
        
        lines.join("\n") + "\n"
    }

    /// Creates a KernelConfig with x86_64-only baseline configuration
    /// Enables x86_64 and disables all other architectures
    pub fn x86_64_baseline() -> Self {
        let mut config = Self::new();
        
        // Enable x86_64
        config.set("CONFIG_X86_64", "y");
        config.set("CONFIG_64BIT", "y");
        
        // Disable all other architectures
        config.unset("CONFIG_ARM");
        config.unset("CONFIG_ARM64");
        config.unset("CONFIG_MIPS");
        config.unset("CONFIG_POWERPC");
        config.unset("CONFIG_PPC");
        config.unset("CONFIG_PPC64");
        config.unset("CONFIG_RISCV");
        config.unset("CONFIG_S390");
        config.unset("CONFIG_IA64");
        config.unset("CONFIG_ALPHA");
        config.unset("CONFIG_M68K");
        config.unset("CONFIG_MICROBLAZE");
        config.unset("CONFIG_NDS32");
        config.unset("CONFIG_ARC");
        config.unset("CONFIG_SH");
        config.unset("CONFIG_SPARC");
        config.unset("CONFIG_SPARC64");
        config.unset("CONFIG_HEXAGON");
        
        // Disable legacy buses and hardware
        config.unset("CONFIG_ISA");
        config.unset("CONFIG_EISA");
        config.unset("CONFIG_MCA");
        config.unset("CONFIG_PARALLEL_PORT");
        config.unset("CONFIG_FLOPPY");
        config.unset("CONFIG_IDE");
        
        config
    }

    /// Applies desktop/gaming optimizations to the config
    /// Adds settings suitable for desktop/gaming workloads
    pub fn apply_desktop_optimizations(&mut self) {
        // Performance optimization
        self.set("CONFIG_CC_OPTIMIZE_FOR_PERFORMANCE", "y");
        
        // High resolution timers and low latency
        self.set("CONFIG_HIGH_RES_TIMERS", "y");
        self.set("CONFIG_NO_HZ_FULL", "y");
        self.set("CONFIG_HZ_1000", "y");
        self.set("CONFIG_HZ", "1000");
        
        // Preemption for desktop responsiveness
        self.set("CONFIG_PREEMPT", "y");
        self.set("CONFIG_PREEMPT_COUNT", "y");
        
        // Modern CPU features
        self.set("CONFIG_X86_X2APIC", "y");
        self.set("CONFIG_X86_TSC", "y");
        
        // Gaming-relevant features
        self.set("CONFIG_FUTEX", "y");
        self.set("CONFIG_FUTEX2", "y");
        
        // Disable unnecessary features for desktop
        self.unset("CONFIG_EMBEDDED");
        
        // Keep essential filesystems
        self.set("CONFIG_EXT4_FS", "y");
        self.set("CONFIG_BTRFS_FS", "y");
        self.set("CONFIG_XFS_FS", "y");
        self.set("CONFIG_F2FS_FS", "y");
        self.set("CONFIG_VFAT_FS", "y");
        self.set("CONFIG_NTFS_FS", "y");
        self.set("CONFIG_NTFS3_FS", "y");
        
        // Disable obscure filesystems
        self.unset("CONFIG_REISERFS_FS");
        self.unset("CONFIG_JFS_FS");
        self.unset("CONFIG_HFS_FS");
        self.unset("CONFIG_HFSPLUS_FS");
    }

    /// Applies bloat removal categories to the config
    /// Uses the categories defined in bloat_removal module
    pub fn apply_bloat_removal(&mut self, categories: &[&str]) {
        for category in categories {
            match *category {
                "Architecture Cleanup" => {
                    // Already handled in x86_64_baseline
                }
                "Industrial Hardware Removal" => {
                    self.unset("CONFIG_INFINIBAND");
                    self.unset("CONFIG_FIBRE_CHANNEL");
                    self.unset("CONFIG_SCSI_TAPE");
                    self.unset("CONFIG_LEGACY_HARDWARE");
                }
                "Enterprise Features Removal" => {
                    self.unset("CONFIG_CLUSTERING");
                    self.unset("CONFIG_MAINFRAME_SUPPORT");
                    // Keep virtualization for desktop gaming (may use VMs/containers)
                    // self.unset("CONFIG_VIRTUALIZATION");
                }
                "Embedded Systems Removal" => {
                    self.unset("CONFIG_SPI");
                    self.unset("CONFIG_I2C_SENSORS");
                    self.unset("CONFIG_INDUSTRIAL_BUSES");
                }
                "Legacy Hardware Removal" => {
                    // Already handled in x86_64_baseline
                }
                "Networking Protocols Cleanup" => {
                    self.unset("CONFIG_DECNET");
                    self.unset("CONFIG_APPLETALK");
                    self.unset("CONFIG_X25");
                    self.unset("CONFIG_AMATEUR_RADIO");
                }
                _ => {
                    // Unknown category, skip
                }
            }
        }
    }

    /// Creates a complete desktop/gaming config with all optimizations
    /// This is the main entry point for generating a KernelForge config
    pub fn desktop_gaming() -> Self {
        let mut config = Self::x86_64_baseline();
        config.apply_desktop_optimizations();
        config.apply_bloat_removal(&[
            "Architecture Cleanup",
            "Industrial Hardware Removal",
            "Enterprise Features Removal",
            "Embedded Systems Removal",
            "Legacy Hardware Removal",
            "Networking Protocols Cleanup",
        ]);
        config
    }
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_emit() {
        let mut config = KernelConfig::new();
        config.set("CONFIG_X86_64", "y");
        config.set("CONFIG_HZ", "1000");
        
        let output = config.emit();
        assert!(output.contains("CONFIG_X86_64=y"));
        assert!(output.contains("CONFIG_HZ=1000"));
    }

    #[test]
    fn test_unset_and_emit() {
        let mut config = KernelConfig::new();
        config.unset("CONFIG_ARM");
        config.unset("CONFIG_MIPS");
        
        let output = config.emit();
        assert!(output.contains("# CONFIG_ARM is not set"));
        assert!(output.contains("# CONFIG_MIPS is not set"));
    }

    #[test]
    fn test_mixed_set_unset() {
        let mut config = KernelConfig::new();
        config.set("CONFIG_X86_64", "y");
        config.unset("CONFIG_ARM");
        config.set("CONFIG_HZ", "1000");
        
        let output = config.emit();
        assert!(output.contains("CONFIG_X86_64=y"));
        assert!(output.contains("# CONFIG_ARM is not set"));
        assert!(output.contains("CONFIG_HZ=1000"));
    }

    #[test]
    fn test_x86_64_baseline() {
        let config = KernelConfig::x86_64_baseline();
        let output = config.emit();
        
        // Should enable x86_64
        assert!(output.contains("CONFIG_X86_64=y"));
        assert!(output.contains("CONFIG_64BIT=y"));
        
        // Should disable other architectures
        assert!(output.contains("# CONFIG_ARM is not set"));
        assert!(output.contains("# CONFIG_ARM64 is not set"));
        assert!(output.contains("# CONFIG_MIPS is not set"));
        assert!(output.contains("# CONFIG_POWERPC is not set"));
        assert!(output.contains("# CONFIG_RISCV is not set"));
        assert!(output.contains("# CONFIG_S390 is not set"));
        assert!(output.contains("# CONFIG_SPARC is not set"));
        
        // Should disable legacy hardware
        assert!(output.contains("# CONFIG_ISA is not set"));
        assert!(output.contains("# CONFIG_FLOPPY is not set"));
    }

    #[test]
    fn test_desktop_optimizations() {
        let mut config = KernelConfig::new();
        config.apply_desktop_optimizations();
        
        let output = config.emit();
        
        // Performance settings
        assert!(output.contains("CONFIG_HZ_1000=y"));
        assert!(output.contains("CONFIG_HZ=1000"));
        assert!(output.contains("CONFIG_PREEMPT=y"));
        
        // Gaming features
        assert!(output.contains("CONFIG_FUTEX=y"));
        assert!(output.contains("CONFIG_FUTEX2=y"));
        
        // Filesystems
        assert!(output.contains("CONFIG_EXT4_FS=y"));
        assert!(output.contains("CONFIG_BTRFS_FS=y"));
        assert!(output.contains("# CONFIG_REISERFS_FS is not set"));
    }

    #[test]
    fn test_bloat_removal() {
        let mut config = KernelConfig::new();
        config.apply_bloat_removal(&["Industrial Hardware Removal"]);
        
        let output = config.emit();
        assert!(output.contains("# CONFIG_INFINIBAND is not set"));
        assert!(output.contains("# CONFIG_FIBRE_CHANNEL is not set"));
    }

    #[test]
    fn test_desktop_gaming_complete() {
        let config = KernelConfig::desktop_gaming();
        let output = config.emit();
        
        // Should have x86_64
        assert!(output.contains("CONFIG_X86_64=y"));
        
        // Should have performance settings
        assert!(output.contains("CONFIG_HZ=1000"));
        
        // Should disable other architectures
        assert!(output.contains("# CONFIG_ARM is not set"));
        
        // Should disable bloat
        assert!(output.contains("# CONFIG_INFINIBAND is not set"));
    }

    #[test]
    fn test_deterministic_output() {
        let mut config1 = KernelConfig::new();
        config1.set("CONFIG_A", "y");
        config1.set("CONFIG_B", "y");
        
        let mut config2 = KernelConfig::new();
        config2.set("CONFIG_B", "y");
        config2.set("CONFIG_A", "y");
        
        // Output should be identical regardless of insertion order
        assert_eq!(config1.emit(), config2.emit());
    }
}
