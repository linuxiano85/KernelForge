// Example usage of KernelForge API

use kernelforge::{KernelVersion, BuildPlan, BuildPlanBuilder, PatchResolver};

fn main() {
    println!("=== KernelForge: Linux Kernel Build Planner ===\n");

    // 1. Use default kernel version (6.6 LTS)
    let default_version = KernelVersion::default_target();
    println!("Default kernel version: {}", default_version.full_name());
    println!("  - Version string: {}", default_version);
    println!("  - Is LTS: {}", default_version.is_lts());
    println!();

    // 2. Create a build plan for default version
    let default_plan = BuildPlan::new(default_version);
    println!("Default Build Plan:");
    println!("{}", default_plan.summary());
    println!();

    // 3. Create a build plan for 6.17
    println!("=== Trying Latest Kernel (6.17) ===");
    let latest_plan = BuildPlan::new(KernelVersion::V6_17);
    println!("{}", latest_plan.summary());
    println!();

    // 4. Custom build plan with ThinLTO
    println!("=== Custom Build Plan with ThinLTO ===");
    let custom_plan = BuildPlanBuilder::new(KernelVersion::V6_6_Lts)
        .enable_thin_lto()
        .build();
    println!("{}", custom_plan.summary());
    println!();

    // 5. Compare patches between versions
    println!("=== Patch Comparison ===");
    let v66_patches = PatchResolver::get_patches_for_version(KernelVersion::V6_6_Lts);
    let v617_patches = PatchResolver::get_patches_for_version(KernelVersion::V6_17);
    
    println!("6.6 LTS patches: {}", v66_patches.len());
    for patch in &v66_patches {
        println!("  - {} ({})", patch.name, if patch.is_upstream { "upstream" } else { "external" });
    }
    
    println!("\n6.17 patches: {}", v617_patches.len());
    for patch in &v617_patches {
        println!("  - {} ({})", patch.name, if patch.is_upstream { "upstream" } else { "external" });
    }
    println!();

    // 6. Show external patches that need to be applied
    println!("=== External Patches (need manual application) ===");
    let external_patches = custom_plan.external_patches();
    println!("Found {} external patches:", external_patches.len());
    for patch in external_patches {
        println!("  - {}", patch.name);
        println!("    Description: {}", patch.description);
        println!("    Source: {}", patch.source);
    }
    println!();

    // 7. Parse version from string
    println!("=== Version Parsing ===");
    let test_strings = vec!["6.6", "6.6-lts", "6.17", "v6.17", "invalid"];
    for s in test_strings {
        match KernelVersion::from_string(s) {
            Some(v) => println!("  '{}' -> {}", s, v),
            None => println!("  '{}' -> Invalid", s),
        }
    }
    println!();

    // 8. Validate a build plan
    println!("=== Build Plan Validation ===");
    match custom_plan.validate() {
        Ok(_) => println!("✓ Build plan is valid!"),
        Err(e) => println!("✗ Build plan validation failed: {}", e),
    }
}
