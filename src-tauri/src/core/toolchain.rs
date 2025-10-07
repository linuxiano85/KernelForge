// src-tauri/src/core/toolchain.rs

use std::process::Command;
use anyhow::{Result, Context};

/// Represents the type of toolchain detected
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Toolchain {
    /// LLVM/Clang toolchain with version info
    Clang { version: String },
    /// GCC toolchain with version info
    Gcc { version: String },
}

impl Toolchain {
    /// Returns the name of the toolchain
    pub fn name(&self) -> &str {
        match self {
            Toolchain::Clang { .. } => "clang",
            Toolchain::Gcc { .. } => "gcc",
        }
    }

    /// Returns the version string
    pub fn version(&self) -> &str {
        match self {
            Toolchain::Clang { version } => version,
            Toolchain::Gcc { version } => version,
        }
    }
}

/// LTO (Link Time Optimization) configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LtoConfig {
    /// Clang ThinLTO (recommended for kernel builds)
    ThinLto,
    /// Full LTO (slower build, maximum optimization)
    FullLto,
    /// No LTO
    None,
}

/// Build plan containing toolchain selection and configuration
#[derive(Debug, Clone)]
pub struct BuildPlan {
    /// Selected toolchain
    pub toolchain: Toolchain,
    /// LTO configuration
    pub lto: LtoConfig,
    /// Additional make flags
    pub make_flags: Vec<String>,
    /// Kconfig options to set
    pub kconfig_options: Vec<(String, String)>,
}

impl BuildPlan {
    /// Creates a new BuildPlan with the given toolchain
    pub fn new(toolchain: Toolchain) -> Self {
        let (lto, make_flags, kconfig_options) = match &toolchain {
            Toolchain::Clang { .. } => {
                // Use LLVM/Clang with ThinLTO
                let lto = LtoConfig::ThinLto;
                let make_flags = vec![
                    "LLVM=1".to_string(),
                ];
                let kconfig_options = vec![
                    ("CONFIG_LTO_CLANG_THIN".to_string(), "y".to_string()),
                    ("CONFIG_CC_OPTIMIZE_FOR_PERFORMANCE".to_string(), "y".to_string()),
                ];
                (lto, make_flags, kconfig_options)
            }
            Toolchain::Gcc { .. } => {
                // Use GCC with performance optimization
                // Avoid GCC LTO as it's less stable for kernel builds
                let lto = LtoConfig::None;
                let make_flags = vec![];
                let kconfig_options = vec![
                    ("CONFIG_CC_OPTIMIZE_FOR_PERFORMANCE".to_string(), "y".to_string()),
                ];
                (lto, make_flags, kconfig_options)
            }
        };

        BuildPlan {
            toolchain,
            lto,
            make_flags,
            kconfig_options,
        }
    }

    /// Generates the make command line with all flags
    pub fn make_command(&self) -> Vec<String> {
        let mut cmd = vec!["make".to_string()];
        cmd.extend(self.make_flags.clone());
        cmd
    }

    /// Returns a description of the build plan
    pub fn description(&self) -> String {
        format!(
            "Toolchain: {} {}, LTO: {:?}",
            self.toolchain.name(),
            self.toolchain.version(),
            self.lto
        )
    }
}

/// Toolchain detector for finding available compilers
pub struct ToolchainDetector {
    /// Optional custom PATH to search
    custom_path: Option<String>,
}

impl ToolchainDetector {
    /// Creates a new ToolchainDetector with system PATH
    pub fn new() -> Self {
        ToolchainDetector {
            custom_path: None,
        }
    }

    /// Creates a new ToolchainDetector with custom PATH (for testing)
    pub fn with_path(path: String) -> Self {
        ToolchainDetector {
            custom_path: Some(path),
        }
    }

    /// Detects available toolchain, preferring Clang over GCC
    /// 
    /// Priority:
    /// 1. Clang + ld.lld (LLVM toolchain)
    /// 2. GCC (fallback)
    /// 
    /// Returns an error if no toolchain is found
    pub fn detect(&self) -> Result<Toolchain> {
        // Try Clang first (preferred for kernel LTO)
        if let Ok(clang) = self.detect_clang() {
            // Verify ld.lld is also available for full LLVM toolchain
            if self.has_lld() {
                return Ok(clang);
            }
        }

        // Fall back to GCC
        self.detect_gcc()
            .context("No suitable toolchain found. Please install clang+lld or gcc")
    }

    /// Detects Clang toolchain
    fn detect_clang(&self) -> Result<Toolchain> {
        let version = self.get_command_version("clang", &["--version"])?;
        Ok(Toolchain::Clang { version })
    }

    /// Detects GCC toolchain
    fn detect_gcc(&self) -> Result<Toolchain> {
        let version = self.get_command_version("gcc", &["--version"])?;
        Ok(Toolchain::Gcc { version })
    }

    /// Checks if ld.lld is available
    fn has_lld(&self) -> bool {
        self.check_command_exists("ld.lld")
    }

    /// Checks if a command exists in PATH
    fn check_command_exists(&self, cmd: &str) -> bool {
        let mut command = if let Some(ref path) = self.custom_path {
            let mut c = Command::new("sh");
            c.arg("-c");
            c.arg(format!("PATH='{}' command -v {}", path, cmd));
            c
        } else {
            Command::new(cmd)
        };

        command
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Gets version string from a command
    fn get_command_version(&self, cmd: &str, args: &[&str]) -> Result<String> {
        let output = if let Some(ref path) = self.custom_path {
            let mut command = Command::new("sh");
            command.arg("-c");
            command.arg(format!("PATH='{}' {} {}", path, cmd, args.join(" ")));
            command.output()?
        } else {
            let mut command = Command::new(cmd);
            for arg in args {
                command.arg(arg);
            }
            command.output()?
        };

        if !output.status.success() {
            anyhow::bail!("{} not found or not working", cmd);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Extract version from first line
        let first_line = stdout.lines().next().unwrap_or("");
        
        // Parse version number (look for X.Y.Z pattern)
        let version = first_line
            .split_whitespace()
            .find(|s| s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
            .unwrap_or("unknown")
            .to_string();

        Ok(version)
    }

    /// Creates a BuildPlan with the best available toolchain
    pub fn create_build_plan(&self) -> Result<BuildPlan> {
        let toolchain = self.detect()?;
        Ok(BuildPlan::new(toolchain))
    }
}

impl Default for ToolchainDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolchain_name() {
        let clang = Toolchain::Clang { version: "14.0.0".to_string() };
        assert_eq!(clang.name(), "clang");

        let gcc = Toolchain::Gcc { version: "11.3.0".to_string() };
        assert_eq!(gcc.name(), "gcc");
    }

    #[test]
    fn test_toolchain_version() {
        let clang = Toolchain::Clang { version: "14.0.0".to_string() };
        assert_eq!(clang.version(), "14.0.0");

        let gcc = Toolchain::Gcc { version: "11.3.0".to_string() };
        assert_eq!(gcc.version(), "11.3.0");
    }

    #[test]
    fn test_build_plan_clang() {
        let toolchain = Toolchain::Clang { version: "14.0.0".to_string() };
        let plan = BuildPlan::new(toolchain);

        assert_eq!(plan.lto, LtoConfig::ThinLto);
        assert!(plan.make_flags.contains(&"LLVM=1".to_string()));
        assert!(plan.kconfig_options.iter().any(|(k, v)| k == "CONFIG_LTO_CLANG_THIN" && v == "y"));
    }

    #[test]
    fn test_build_plan_gcc() {
        let toolchain = Toolchain::Gcc { version: "11.3.0".to_string() };
        let plan = BuildPlan::new(toolchain);

        assert_eq!(plan.lto, LtoConfig::None);
        assert!(!plan.make_flags.contains(&"LLVM=1".to_string()));
        assert!(plan.kconfig_options.iter().any(|(k, v)| k == "CONFIG_CC_OPTIMIZE_FOR_PERFORMANCE" && v == "y"));
    }

    #[test]
    fn test_make_command() {
        let toolchain = Toolchain::Clang { version: "14.0.0".to_string() };
        let plan = BuildPlan::new(toolchain);

        let cmd = plan.make_command();
        assert_eq!(cmd[0], "make");
        assert!(cmd.contains(&"LLVM=1".to_string()));
    }

    #[test]
    fn test_build_plan_description() {
        let toolchain = Toolchain::Clang { version: "14.0.0".to_string() };
        let plan = BuildPlan::new(toolchain);

        let desc = plan.description();
        assert!(desc.contains("clang"));
        assert!(desc.contains("14.0.0"));
        assert!(desc.contains("ThinLto"));
    }

    #[test]
    fn test_detector_default() {
        let detector = ToolchainDetector::default();
        assert!(detector.custom_path.is_none());
    }

    #[test]
    fn test_detector_with_path() {
        let detector = ToolchainDetector::with_path("/custom/path".to_string());
        assert_eq!(detector.custom_path, Some("/custom/path".to_string()));
    }

    #[test]
    fn test_detector_finds_toolchain() {
        // This test will work if system has gcc or clang
        let detector = ToolchainDetector::new();
        
        // At least one toolchain should be available on the system
        let result = detector.detect();
        
        // We can't assert success here as it depends on the system,
        // but we can check the code compiles and runs
        match result {
            Ok(toolchain) => {
                assert!(!toolchain.version().is_empty());
            }
            Err(_) => {
                // No toolchain available on test system, that's okay
            }
        }
    }

    #[test]
    fn test_create_build_plan() {
        let detector = ToolchainDetector::new();
        
        // Try to create a build plan
        match detector.create_build_plan() {
            Ok(plan) => {
                // If successful, verify it's valid
                assert!(!plan.toolchain.version().is_empty());
                assert!(!plan.make_command().is_empty());
            }
            Err(_) => {
                // No toolchain available on test system, that's okay
            }
        }
    }
}
