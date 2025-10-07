// src-tauri/examples/generate_config.rs
//! Example demonstrating how to generate a complete kernel configuration
//! using the KernelForge config and toolchain modules.

use kernelforge::core::config::KernelConfig;
use kernelforge::core::toolchain::ToolchainDetector;

fn main() -> anyhow::Result<()> {
    println!("KernelForge Configuration Generator\n");
    println!("====================================\n");

    // Step 1: Detect available toolchain
    println!("1. Detecting toolchain...");
    let detector = ToolchainDetector::new();
    
    match detector.detect() {
        Ok(toolchain) => {
            println!("   ✓ Found {} version {}", toolchain.name(), toolchain.version());
            
            // Step 2: Create build plan
            println!("\n2. Creating build plan...");
            let build_plan = detector.create_build_plan()?;
            println!("   {}", build_plan.description());
            
            // Step 3: Generate kernel config
            println!("\n3. Generating kernel configuration...");
            let mut config = KernelConfig::desktop_gaming();
            
            // Apply toolchain-specific options
            for (key, value) in &build_plan.kconfig_options {
                config.set(key, value);
            }
            
            println!("   ✓ Configuration generated with x86_64-only baseline");
            println!("   ✓ Desktop/gaming optimizations applied");
            println!("   ✓ Bloat removal categories applied");
            
            // Step 4: Show sample of generated config
            println!("\n4. Sample configuration output:");
            println!("   -----------------------------------");
            let config_output = config.emit();
            for line in config_output.lines().take(20) {
                println!("   {}", line);
            }
            println!("   ... ({} total lines)", config_output.lines().count());
            
            // Step 5: Show build command
            println!("\n5. Suggested build command:");
            let make_cmd = build_plan.make_command();
            println!("   {}", make_cmd.join(" "));
            
            println!("\n✓ Configuration ready!");
            println!("\nTo use this configuration:");
            println!("  1. Save the output to .config in your kernel source tree");
            println!("  2. Run 'make olddefconfig' to fill in missing options");
            println!("  3. Build with: {}", make_cmd.join(" "));
        }
        Err(e) => {
            println!("   ✗ No toolchain found: {}", e);
            println!("\nPlease install either:");
            println!("  - clang + ld.lld (recommended)");
            println!("  - gcc");
            return Err(e);
        }
    }

    Ok(())
}
