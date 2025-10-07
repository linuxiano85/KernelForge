// src-tauri/examples/full_workflow.rs
//! Complete KernelForge workflow example showing integration of all modules

use kernelforge::core::bloat_removal::BloatRemovalEngine;
use kernelforge::core::config::KernelConfig;
use kernelforge::core::toolchain::ToolchainDetector;

fn main() -> anyhow::Result<()> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║         KernelForge Complete Workflow Example             ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Step 1: Explore bloat removal categories
    println!("📦 Step 1: Available Bloat Removal Categories\n");
    let bloat_engine = BloatRemovalEngine::new();
    let categories = bloat_engine.get_category_names();
    
    for (i, category) in categories.iter().enumerate() {
        println!("   {}. {}", i + 1, category);
    }
    println!("\n   Estimated total savings: {} MB\n", bloat_engine.estimate_size_savings());

    // Step 2: Detect toolchain
    println!("🔧 Step 2: Toolchain Detection\n");
    let detector = ToolchainDetector::new();
    
    let toolchain = detector.detect()?;
    println!("   ✓ Detected: {} version {}", toolchain.name(), toolchain.version());
    
    let build_plan = detector.create_build_plan()?;
    println!("   ✓ LTO Configuration: {:?}", build_plan.lto);
    println!("   ✓ Make flags: {:?}\n", build_plan.make_flags);

    // Step 3: Generate kernel configuration
    println!("⚙️  Step 3: Kernel Configuration Generation\n");
    let mut config = KernelConfig::x86_64_baseline();
    println!("   ✓ x86_64 baseline configured");
    
    config.apply_desktop_optimizations();
    println!("   ✓ Desktop optimizations applied");
    
    // Apply specific bloat removal categories
    let selected_bloat_categories = [
        "Architecture Cleanup",
        "Industrial Hardware Removal", 
        "Legacy Hardware Removal",
        "Networking Protocols Cleanup",
    ];
    config.apply_bloat_removal(&selected_bloat_categories);
    println!("   ✓ Bloat removal applied ({} categories)", selected_bloat_categories.len());
    
    // Apply toolchain-specific optimizations
    for (key, value) in &build_plan.kconfig_options {
        config.set(key, value);
    }
    println!("   ✓ Toolchain optimizations applied\n");

    // Step 4: Show configuration summary
    println!("📋 Step 4: Configuration Summary\n");
    let config_output = config.emit();
    let lines: Vec<&str> = config_output.lines().collect();
    
    let enabled_count = lines.iter().filter(|l| !l.starts_with('#')).count();
    let disabled_count = lines.iter().filter(|l| l.starts_with('#')).count();
    
    println!("   Total config options: {}", lines.len());
    println!("   Enabled options:      {}", enabled_count);
    println!("   Disabled options:     {}", disabled_count);
    println!();

    // Step 5: Show sample configuration
    println!("📄 Step 5: Sample Configuration (first 30 lines)\n");
    println!("   ┌────────────────────────────────────────────────────────┐");
    for line in lines.iter().take(30) {
        println!("   │ {:54} │", line);
    }
    println!("   │ ... ({} more lines) ...", lines.len() - 30);
    println!("   └────────────────────────────────────────────────────────┘\n");

    // Step 6: Show build instructions
    println!("🏗️  Step 6: Build Instructions\n");
    println!("   To build your optimized kernel:\n");
    println!("   1. Save the configuration:");
    println!("      $ cargo run --example full_workflow > /path/to/kernel/.config\n");
    println!("   2. Navigate to kernel source:");
    println!("      $ cd /path/to/kernel\n");
    println!("   3. Apply defaults:");
    println!("      $ make olddefconfig\n");
    println!("   4. Build the kernel:");
    let build_cmd = build_plan.make_command().join(" ");
    println!("      $ {} -j$(nproc)\n", build_cmd);
    
    if toolchain.name() == "clang" {
        println!("   💡 Tip: You're using Clang with ThinLTO for optimal performance!");
    }

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                    ✓ Configuration Ready                   ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    // Output the actual config for piping to file
    if std::env::args().any(|arg| arg == "--output-config") {
        println!("\n{}", config_output);
    }

    Ok(())
}
