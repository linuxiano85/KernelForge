// src-tauri/src/lib.rs

pub mod core;

// Re-export commonly used types
pub use core::version::KernelVersion;
pub use core::build_plan::{BuildPlan, BuildPlanBuilder, ToolchainConfig, Compiler, LtoMode};
pub use core::patches::{Patch, PatchResolver};
pub use core::config::{ConfigOption, ConfigValue, ConfigGenerator};
