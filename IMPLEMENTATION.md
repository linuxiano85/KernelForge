# KernelForge 6.17 Support - Implementation Documentation

## Overview

This implementation adds first-class support for Linux kernel 6.17 alongside the existing 6.6 LTS version, providing version-aware configuration, patch selection, and build planning.

## Architecture

### Core Modules

1. **version.rs** - Kernel version management
2. **patches.rs** - Per-version patch resolution
3. **config.rs** - Kernel configuration generation
4. **build_plan.rs** - Complete build plan orchestration

## API Reference

### KernelVersion Enum

```rust
pub enum KernelVersion {
    V6_6_Lts,  // Linux 6.6 LTS - stable, long-term support
    V6_17,     // Linux 6.17 - latest features
}
```

**Key Methods:**
- `default_target() -> KernelVersion` - Returns V6_6_Lts (stable default)
- `from_string(s: &str) -> Option<KernelVersion>` - Parse version from string
- `to_string(&self) -> &'static str` - Get version string representation
- `is_lts(&self) -> bool` - Check if version is LTS
- `major() -> u32` / `minor() -> u32` - Get version numbers

### PatchResolver

Manages version-specific patches for gaming/performance optimizations.

**Available Patches:**

**6.6 LTS:**
- BORE (external) - Burst-Oriented Response Enhancer scheduler
- BBRv3 (external) - TCP congestion control v3
- FUTEX2 (upstream) - Already in kernel
- PREEMPT_RT (external) - Real-time preemption

**6.17:**
- BORE (external) - Compatible with 6.17
- BBRv3 (upstream) - Now in mainline
- FUTEX2 (upstream) - Already in kernel
- Note: PREEMPT_RT not yet available for 6.17

**Key Methods:**
- `get_patches_for_version(version) -> Vec<Patch>`
- `is_patch_available(version, name) -> bool`
- `get_external_patches(version) -> Vec<Patch>`

### ConfigGenerator

Generates safe kernel configurations with gaming optimizations.

**Baseline Configuration:**
- Architecture: CONFIG_64BIT, CONFIG_X86_64
- Modules: CONFIG_MODULES, CONFIG_MODULE_UNLOAD
- Preemption: CONFIG_PREEMPT, CONFIG_PREEMPT_COUNT
- Timer: CONFIG_HZ_1000 (1000Hz for low latency)
- Filesystems: ext4, btrfs
- Networking: NET, INET, TCP_CONG_BBR

**Key Methods:**
- `generate_baseline(version) -> Vec<ConfigOption>`
- `to_config_file(options) -> String`
- `validate_config(version, options) -> Result<(), String>`

### BuildPlan

Complete build plan including version, patches, config, and toolchain.

**Toolchain Configuration:**
- Default: Clang/LLVM with lld linker
- Fallback: GCC with traditional ld
- LTO: None by default (ThinLTO opt-in via `with_thin_lto()`)

**Key Methods:**
- `new(version) -> BuildPlan` - Create plan with defaults
- `with_thin_lto(self) -> Self` - Enable ThinLTO
- `with_gcc(self) -> Self` - Use GCC instead of Clang
- `external_patches(&self) -> Vec<&Patch>` - Get patches to apply
- `validate(&self) -> Result<(), String>` - Validate plan

## Usage Examples

### Basic Usage

```rust
use kernelforge::{KernelVersion, BuildPlan};

// Use default (6.6 LTS)
let plan = BuildPlan::new(KernelVersion::default_target());

// Or select 6.17 explicitly
let latest = BuildPlan::new(KernelVersion::V6_17);
```

### Custom Build Plan

```rust
use kernelforge::{BuildPlanBuilder, KernelVersion};

let plan = BuildPlanBuilder::new(KernelVersion::V6_6_Lts)
    .enable_thin_lto()
    .build();
```

### Version Parsing

```rust
use kernelforge::KernelVersion;

let v = KernelVersion::from_string("6.17").unwrap();
assert_eq!(v, KernelVersion::V6_17);
```

### Patch Inspection

```rust
use kernelforge::{BuildPlan, KernelVersion};

let plan = BuildPlan::new(KernelVersion::V6_17);
let external = plan.external_patches();

for patch in external {
    println!("Apply: {} from {}", patch.name, patch.source);
}
```

## Version Trade-offs

### 6.6 LTS (Recommended)
✅ Pros:
- Long-term support and security updates
- All gaming patches tested and confirmed
- Maximum stability
- PREEMPT_RT available

⚠️ Cons:
- Newer features arrive more slowly
- Some patches remain external

### 6.17 (Latest)
✅ Pros:
- Latest kernel features
- BBRv3 now upstream
- Recent performance improvements

⚠️ Cons:
- Short-term support
- PREEMPT_RT not yet available
- Some patches may lag behind

## Design Decisions

1. **Default to 6.6 LTS**: Prioritizes stability for most users
2. **Explicit 6.17 selection**: Users must opt-in to latest version
3. **Upstream preferred**: Use mainline features when available
4. **Compatible patches only**: Only apply external patches confirmed for version
5. **ThinLTO opt-in**: Advanced optimization requires explicit enablement
6. **Clang preferred**: Modern toolchain by default, GCC available

## Testing

All functionality is thoroughly tested:
- 28 unit tests covering all modules
- 1 documentation test
- Example program demonstrating API usage

Run tests:
```bash
cd src-tauri
cargo test
```

Run example:
```bash
cd src-tauri
cargo run --example kernel_version_demo
```

## Future Extensions

Potential additions:
- Support for additional kernel versions (6.18, 6.19, etc.)
- More patch options (Custom schedulers, I/O schedulers)
- Architecture support beyond x86_64
- Configuration profiles (gaming, server, minimal)
- Patch compatibility matrix
- Automated patch application

## Notes

- This implementation focuses on core infrastructure
- Actual kernel building is not included (config generation only)
- Patch URLs are examples; real implementations need valid sources
- Some patches may require manual verification for compatibility
