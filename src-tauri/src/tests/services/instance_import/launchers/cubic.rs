use super::*;
use crate::services::instance_import::sanitize_instance_name;

#[test]
fn test_detect_cubic_format() {
    let temp =
        std::env::temp_dir().join(format!("cubic_import_detect_test_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp).unwrap();
    std::fs::write(temp.join("cubic-manifest.json"), r#"{"format_version":1,"exported_by":"CubicLauncher","uuid":"u","name":"Test","version_id":"1.21","mc_version":"1.21","loader":"Vanilla","loader_version":null,"min_memory":512,"max_memory":2048,"overrides":null}"#).unwrap();

    let provider = CubicProvider;
    assert!(provider.detect(&temp));

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn test_preview_cubic_instance() {
    let temp = std::env::temp_dir().join(format!(
        "cubic_import_preview_test_{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&temp).unwrap();
    std::fs::write(temp.join("cubic-manifest.json"), r#"{"format_version":1,"exported_by":"CubicLauncher","uuid":"u","name":"Mi Instancia","version_id":"fabric-loader-0.15.0-1.21","mc_version":"1.21","loader":"Fabric","loader_version":"0.15.0","min_memory":1024,"max_memory":4096,"overrides":null}"#).unwrap();

    let provider = CubicProvider;
    let plan = provider.preview(&temp).unwrap();

    assert_eq!(plan.format_id, "cubic");
    assert_eq!(plan.minecraft_version.as_deref(), Some("1.21"));
    assert_eq!(plan.loader.as_deref(), Some("Fabric"));
    assert_eq!(plan.loader_version.as_deref(), Some("0.15.0"));
    assert_eq!(plan.sanitized_name, sanitize_instance_name("Mi Instancia"));

    let _ = std::fs::remove_dir_all(&temp);
}
