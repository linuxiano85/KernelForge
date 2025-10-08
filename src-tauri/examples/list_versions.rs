// src-tauri/examples/list_versions.rs

use kernelforge::version_catalog;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Fetching kernel versions from kernel.org...\n");
    
    // List available versions (will use cache if valid)
    let versions = version_catalog::list_available_versions(false).await?;
    
    println!("Found {} kernel versions:\n", versions.len());
    
    // Group by channel
    let mut stable = vec![];
    let mut mainline = vec![];
    let mut longterm = vec![];
    let mut other = vec![];
    
    for version in &versions {
        match version.channel.as_str() {
            "stable" => stable.push(version),
            "mainline" => mainline.push(version),
            "longterm" => longterm.push(version),
            _ => other.push(version),
        }
    }
    
    // Print mainline versions
    if !mainline.is_empty() {
        println!("📦 Mainline versions:");
        for v in &mainline {
            let eol_marker = if v.eol { " [EOL]" } else { "" };
            println!("  - {}{}", v.version, eol_marker);
        }
        println!();
    }
    
    // Print stable versions
    if !stable.is_empty() {
        println!("✅ Stable versions:");
        for v in &stable {
            let eol_marker = if v.eol { " [EOL]" } else { "" };
            println!("  - {}{}", v.version, eol_marker);
        }
        println!();
    }
    
    // Print longterm versions
    if !longterm.is_empty() {
        println!("🛡️  Longterm versions:");
        for v in &longterm {
            let eol_marker = if v.eol { " [EOL]" } else { "" };
            println!("  - {}{}", v.version, eol_marker);
        }
        println!();
    }
    
    // Print other versions
    if !other.is_empty() {
        println!("📋 Other versions:");
        for v in &other {
            let eol_marker = if v.eol { " [EOL]" } else { "" };
            println!("  - {} [{}]{}", v.version, v.channel, eol_marker);
        }
        println!();
    }
    
    println!("Force refresh example (bypassing cache):");
    let versions_fresh = version_catalog::list_available_versions(true).await?;
    println!("Fetched {} fresh versions", versions_fresh.len());
    
    Ok(())
}
