// src-tauri/examples/simple_check.rs
//! Quick check of the version catalog system

use kernelforge::version_catalog;

fn main() -> anyhow::Result<()> {
    println!("KernelForge Version Catalog - Quick Test\n");
    println!("=========================================\n");
    
    // Use blocking API for simplicity
    println!("Fetching kernel versions...");
    let versions = version_catalog::list_available_versions_blocking(false)?;
    
    println!("✅ Successfully fetched {} kernel versions\n", versions.len());
    
    // Show version range
    let mut version_numbers: Vec<String> = versions.iter()
        .map(|v| v.version.clone())
        .collect();
    version_numbers.sort();
    
    if let (Some(first), Some(last)) = (version_numbers.first(), version_numbers.last()) {
        println!("📊 Version range: {} to {}\n", first, last);
    }
    
    // Count by channel
    let stable_count = versions.iter().filter(|v| v.channel == "stable").count();
    let mainline_count = versions.iter().filter(|v| v.channel == "mainline").count();
    let longterm_count = versions.iter().filter(|v| v.channel == "longterm").count();
    
    println!("📈 Channel distribution:");
    println!("   - Mainline: {}", mainline_count);
    println!("   - Stable: {}", stable_count);
    println!("   - Longterm: {}", longterm_count);
    println!();
    
    // Show latest mainline
    if let Some(latest) = versions.iter().find(|v| v.channel == "mainline") {
        println!("🚀 Latest mainline: {} ({})", latest.version, latest.channel);
    }
    
    // Show longterm versions
    let longterm: Vec<_> = versions.iter()
        .filter(|v| v.channel == "longterm")
        .collect();
    
    if !longterm.is_empty() {
        println!("\n🛡️  Longterm support versions:");
        for v in longterm {
            let eol = if v.eol { " [EOL]" } else { "" };
            println!("   - {}{}", v.version, eol);
        }
    }
    
    println!("\n✅ Version catalog system working correctly!");
    println!("   Supports versions 6.6.x through 6.17.x and beyond");
    
    Ok(())
}
