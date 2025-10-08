// Integration tests for KernelForge

use kernelforge::{
    BuildPlan, BuildPlanBuilder, ConfigGenerator, ConfigValue, KernelVersion, PatchResolver,
};

#[test]
fn test_complete_workflow_6_6_lts() {
    // Create a build plan for 6.6 LTS
    let plan = BuildPlan::new(KernelVersion::V6_6_Lts);

    // Verify version
    assert_eq!(plan.version, KernelVersion::V6_6_Lts);
    assert!(plan.version.is_lts());

    // Verify patches are present
    assert!(!plan.patches.is_empty());
    assert!(plan.patches.iter().any(|p| p.name == "BORE"));
    assert!(plan.patches.iter().any(|p| p.name == "FUTEX2"));

    // Verify external patches
    let external = plan.external_patches();
    assert!(!external.is_empty());

    // Verify configuration
    assert!(!plan.config_options.is_empty());
    assert!(plan
        .config_options
        .iter()
        .any(|o| o.key == "CONFIG_64BIT" && o.value == ConfigValue::Yes));
    assert!(plan
        .config_options
        .iter()
        .any(|o| o.key == "CONFIG_PREEMPT" && o.value == ConfigValue::Yes));

    // Verify toolchain defaults
    assert_eq!(
        plan.toolchain.compiler,
        kernelforge::Compiler::Clang
    );

    // Validate the plan
    assert!(plan.validate().is_ok());
}

#[test]
fn test_complete_workflow_6_17() {
    // Create a build plan for 6.17
    let plan = BuildPlan::new(KernelVersion::V6_17);

    // Verify version
    assert_eq!(plan.version, KernelVersion::V6_17);
    assert!(!plan.version.is_lts());

    // Verify patches are present
    assert!(!plan.patches.is_empty());
    assert!(plan.patches.iter().any(|p| p.name == "BORE"));

    // BBRv3 should be upstream in 6.17
    let bbrv3 = plan.patches.iter().find(|p| p.name == "BBRv3");
    assert!(bbrv3.is_some());
    assert!(bbrv3.unwrap().is_upstream);

    // PREEMPT_RT should not be available for 6.17
    assert!(!plan.patches.iter().any(|p| p.name == "PREEMPT_RT"));

    // Validate the plan
    assert!(plan.validate().is_ok());
}

#[test]
fn test_version_parsing_and_selection() {
    // Test default
    let default = KernelVersion::default_target();
    assert_eq!(default, KernelVersion::V6_6_Lts);

    // Test parsing various formats
    assert_eq!(
        KernelVersion::from_string("6.6"),
        Some(KernelVersion::V6_6_Lts)
    );
    assert_eq!(
        KernelVersion::from_string("6.6-lts"),
        Some(KernelVersion::V6_6_Lts)
    );
    assert_eq!(
        KernelVersion::from_string("6.17"),
        Some(KernelVersion::V6_17)
    );
    assert_eq!(
        KernelVersion::from_string("v6.17"),
        Some(KernelVersion::V6_17)
    );

    // Test invalid inputs
    assert_eq!(KernelVersion::from_string(""), None);
    assert_eq!(KernelVersion::from_string("invalid"), None);
    assert_eq!(KernelVersion::from_string("5.10"), None);
}

#[test]
fn test_patch_availability_per_version() {
    // BORE should be available in both versions
    assert!(PatchResolver::is_patch_available(
        KernelVersion::V6_6_Lts,
        "BORE"
    ));
    assert!(PatchResolver::is_patch_available(
        KernelVersion::V6_17,
        "BORE"
    ));

    // PREEMPT_RT only in 6.6
    assert!(PatchResolver::is_patch_available(
        KernelVersion::V6_6_Lts,
        "PREEMPT_RT"
    ));
    assert!(!PatchResolver::is_patch_available(
        KernelVersion::V6_17,
        "PREEMPT_RT"
    ));

    // FUTEX2 in both (upstream)
    assert!(PatchResolver::is_patch_available(
        KernelVersion::V6_6_Lts,
        "FUTEX2"
    ));
    assert!(PatchResolver::is_patch_available(
        KernelVersion::V6_17,
        "FUTEX2"
    ));
}

#[test]
fn test_config_generation_and_validation() {
    // Generate config for 6.6
    let config_6_6 = ConfigGenerator::generate_baseline(KernelVersion::V6_6_Lts);
    assert!(!config_6_6.is_empty());

    // Validate it
    assert!(ConfigGenerator::validate_config(KernelVersion::V6_6_Lts, &config_6_6).is_ok());

    // Generate config for 6.17
    let config_6_17 = ConfigGenerator::generate_baseline(KernelVersion::V6_17);
    assert!(!config_6_17.is_empty());

    // Validate it
    assert!(ConfigGenerator::validate_config(KernelVersion::V6_17, &config_6_17).is_ok());

    // Both should have essential options
    for config in [&config_6_6, &config_6_17] {
        assert!(config.iter().any(|o| o.key == "CONFIG_64BIT"));
        assert!(config.iter().any(|o| o.key == "CONFIG_X86_64"));
        assert!(config.iter().any(|o| o.key == "CONFIG_MODULES"));
        assert!(config.iter().any(|o| o.key == "CONFIG_PREEMPT"));
        assert!(config.iter().any(|o| o.key == "CONFIG_HZ_1000"));
    }
}

#[test]
fn test_build_plan_builder() {
    // Test builder pattern
    let plan = BuildPlanBuilder::new(KernelVersion::V6_6_Lts)
        .enable_thin_lto()
        .build();

    assert_eq!(plan.version, KernelVersion::V6_6_Lts);
    assert_eq!(plan.toolchain.lto, kernelforge::LtoMode::Thin);

    // Test with GCC
    let gcc_plan = BuildPlanBuilder::new(KernelVersion::V6_17)
        .use_gcc()
        .build();

    assert_eq!(gcc_plan.toolchain.compiler, kernelforge::Compiler::Gcc);
    assert!(!gcc_plan.toolchain.use_lld);
}

#[test]
fn test_toolchain_defaults() {
    let plan = BuildPlan::new(KernelVersion::V6_6_Lts);

    // Should prefer Clang by default
    assert_eq!(
        plan.toolchain.compiler,
        kernelforge::Compiler::Clang
    );

    // Should use lld with Clang
    assert!(plan.toolchain.use_lld);

    // LTO should be opt-in (disabled by default)
    assert_eq!(plan.toolchain.lto, kernelforge::LtoMode::None);
}

#[test]
fn test_external_vs_upstream_patches() {
    let plan_6_6 = BuildPlan::new(KernelVersion::V6_6_Lts);
    let plan_6_17 = BuildPlan::new(KernelVersion::V6_17);

    // Check 6.6: BBRv3 should be external
    let bbr_6_6 = plan_6_6.patches.iter().find(|p| p.name == "BBRv3");
    assert!(bbr_6_6.is_some());
    assert!(!bbr_6_6.unwrap().is_upstream);

    // Check 6.17: BBRv3 should be upstream
    let bbr_6_17 = plan_6_17.patches.iter().find(|p| p.name == "BBRv3");
    assert!(bbr_6_17.is_some());
    assert!(bbr_6_17.unwrap().is_upstream);

    // FUTEX2 should be upstream in both
    assert!(plan_6_6
        .patches
        .iter()
        .find(|p| p.name == "FUTEX2")
        .unwrap()
        .is_upstream);
    assert!(plan_6_17
        .patches
        .iter()
        .find(|p| p.name == "FUTEX2")
        .unwrap()
        .is_upstream);
}

#[test]
fn test_config_file_generation() {
    let config = ConfigGenerator::generate_baseline(KernelVersion::V6_6_Lts);
    let config_file = ConfigGenerator::to_config_file(&config);

    // Should contain header
    assert!(config_file.contains("Automatically generated by KernelForge"));

    // Should contain essential configs
    assert!(config_file.contains("CONFIG_64BIT=y"));
    assert!(config_file.contains("CONFIG_X86_64=y"));
    assert!(config_file.contains("CONFIG_PREEMPT=y"));
    assert!(config_file.contains("CONFIG_HZ_1000=y"));
    assert!(config_file.contains("CONFIG_HZ=1000"));
}
