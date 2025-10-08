// src-tauri/src/core/build_plan.rs

use crate::core::version::KernelVersion;
use crate::core::patches::{Patch, PatchResolver};
use crate::core::config::{ConfigOption, ConfigGenerator};
use serde::{Deserialize, Serialize};

/// Represents a complete build plan for a kernel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildPlan {
    /// Target kernel version
    pub version: KernelVersion,
    /// List of patches to apply
    pub patches: Vec<Patch>,
    /// Kernel configuration options
    pub config_options: Vec<ConfigOption>,
    /// Toolchain preferences
    pub toolchain: ToolchainConfig,
}

/// Toolchain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainConfig {
    /// Preferred compiler (Clang or GCC)
    pub compiler: Compiler,
    /// Whether to use lld linker
    pub use_lld: bool,
    /// LTO configuration (ThinLTO opt-in)
    pub lto: LtoMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Compiler {
    Clang,
    Gcc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LtoMode {
    None,
    Thin,
    Full,
}

impl Default for ToolchainConfig {
    fn default() -> Self {
        ToolchainConfig {
            compiler: Compiler::Clang,
            use_lld: true,
            lto: LtoMode::None, // ThinLTO is opt-in
        }
    }
}

impl BuildPlan {
    /// Create a new build plan for a specific kernel version
    pub fn new(version: KernelVersion) -> Self {
        let patches = PatchResolver::get_patches_for_version(version);
        let config_options = ConfigGenerator::generate_baseline(version);
        
        BuildPlan {
            version,
            patches,
            config_options,
            toolchain: ToolchainConfig::default(),
        }
    }

    /// Create a build plan with custom patches
    pub fn with_patches(version: KernelVersion, patches: Vec<Patch>) -> Self {
        let config_options = ConfigGenerator::generate_baseline(version);
        
        BuildPlan {
            version,
            patches,
            config_options,
            toolchain: ToolchainConfig::default(),
        }
    }

    /// Set the toolchain configuration
    pub fn with_toolchain(mut self, toolchain: ToolchainConfig) -> Self {
        self.toolchain = toolchain;
        self
    }

    /// Enable ThinLTO
    pub fn with_thin_lto(mut self) -> Self {
        self.toolchain.lto = LtoMode::Thin;
        self
    }

    /// Use GCC instead of Clang
    pub fn with_gcc(mut self) -> Self {
        self.toolchain.compiler = Compiler::Gcc;
        self.toolchain.use_lld = false; // lld is typically used with Clang
        self
    }

    /// Get only external patches that need to be applied
    pub fn external_patches(&self) -> Vec<&Patch> {
        self.patches.iter()
            .filter(|p| !p.is_upstream)
            .collect()
    }

    /// Validate the build plan
    pub fn validate(&self) -> Result<(), String> {
        // Validate configuration
        ConfigGenerator::validate_config(self.version, &self.config_options)?;
        
        // Ensure we have at least some configuration
        if self.config_options.is_empty() {
            return Err("Build plan has no configuration options".to_string());
        }
        
        Ok(())
    }

    /// Get a summary of the build plan
    pub fn summary(&self) -> String {
        let patch_count = self.patches.len();
        let external_count = self.external_patches().len();
        let config_count = self.config_options.len();
        
        format!(
            "Build Plan for {}\n\
             - Patches: {} total ({} external)\n\
             - Config options: {}\n\
             - Compiler: {:?}\n\
             - LTO: {:?}",
            self.version.full_name(),
            patch_count,
            external_count,
            config_count,
            self.toolchain.compiler,
            self.toolchain.lto
        )
    }
}

/// Builder for creating custom build plans
pub struct BuildPlanBuilder {
    version: KernelVersion,
    patches: Option<Vec<Patch>>,
    toolchain: ToolchainConfig,
}

impl BuildPlanBuilder {
    /// Create a new builder for a kernel version
    pub fn new(version: KernelVersion) -> Self {
        BuildPlanBuilder {
            version,
            patches: None,
            toolchain: ToolchainConfig::default(),
        }
    }

    /// Set custom patches
    pub fn patches(mut self, patches: Vec<Patch>) -> Self {
        self.patches = Some(patches);
        self
    }

    /// Set toolchain configuration
    pub fn toolchain(mut self, toolchain: ToolchainConfig) -> Self {
        self.toolchain = toolchain;
        self
    }

    /// Enable ThinLTO
    pub fn enable_thin_lto(mut self) -> Self {
        self.toolchain.lto = LtoMode::Thin;
        self
    }

    /// Use GCC compiler
    pub fn use_gcc(mut self) -> Self {
        self.toolchain.compiler = Compiler::Gcc;
        self.toolchain.use_lld = false;
        self
    }

    /// Build the final BuildPlan
    pub fn build(self) -> BuildPlan {
        let patches = self.patches.unwrap_or_else(|| {
            PatchResolver::get_patches_for_version(self.version)
        });
        
        let config_options = ConfigGenerator::generate_baseline(self.version);
        
        BuildPlan {
            version: self.version,
            patches,
            config_options,
            toolchain: self.toolchain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_build_plan() {
        let plan = BuildPlan::new(KernelVersion::V6_6_Lts);
        assert_eq!(plan.version, KernelVersion::V6_6_Lts);
        assert!(!plan.patches.is_empty());
        assert!(!plan.config_options.is_empty());
        assert_eq!(plan.toolchain.compiler, Compiler::Clang);
        assert_eq!(plan.toolchain.lto, LtoMode::None);
    }

    #[test]
    fn test_build_plan_with_thin_lto() {
        let plan = BuildPlan::new(KernelVersion::V6_17).with_thin_lto();
        assert_eq!(plan.toolchain.lto, LtoMode::Thin);
    }

    #[test]
    fn test_build_plan_with_gcc() {
        let plan = BuildPlan::new(KernelVersion::V6_6_Lts).with_gcc();
        assert_eq!(plan.toolchain.compiler, Compiler::Gcc);
        assert!(!plan.toolchain.use_lld);
    }

    #[test]
    fn test_external_patches() {
        let plan = BuildPlan::new(KernelVersion::V6_6_Lts);
        let external = plan.external_patches();
        
        // Should have external patches like BORE
        assert!(!external.is_empty());
        assert!(external.iter().any(|p| p.name == "BORE"));
    }

    #[test]
    fn test_validate_build_plan() {
        let plan = BuildPlan::new(KernelVersion::V6_6_Lts);
        assert!(plan.validate().is_ok());
    }

    #[test]
    fn test_build_plan_builder() {
        let plan = BuildPlanBuilder::new(KernelVersion::V6_17)
            .enable_thin_lto()
            .build();
        
        assert_eq!(plan.version, KernelVersion::V6_17);
        assert_eq!(plan.toolchain.lto, LtoMode::Thin);
    }

    #[test]
    fn test_build_plan_builder_with_gcc() {
        let plan = BuildPlanBuilder::new(KernelVersion::V6_6_Lts)
            .use_gcc()
            .build();
        
        assert_eq!(plan.toolchain.compiler, Compiler::Gcc);
        assert!(!plan.toolchain.use_lld);
    }

    #[test]
    fn test_default_toolchain() {
        let toolchain = ToolchainConfig::default();
        assert_eq!(toolchain.compiler, Compiler::Clang);
        assert!(toolchain.use_lld);
        assert_eq!(toolchain.lto, LtoMode::None);
    }
}
