use super::*;

#[test]
fn test_loader_to_mmc_uid() {
    assert_eq!(loader_to_mmc_uid(&Loader::Vanilla), None);
    assert_eq!(
        loader_to_mmc_uid(&Loader::Fabric("0.15.0".into())),
        Some("net.fabricmc.fabric-loader")
    );
    assert_eq!(
        loader_to_mmc_uid(&Loader::NeoForge("21.0.0".into())),
        Some("net.neoforged")
    );
}

#[test]
fn test_build_mmc_pack_json() {
    let input = ExportInput {
        uuid: "uuid".into(),
        name: "Test".into(),
        version_id: "1.21-fabric-0.15.0".into(),
        mc_version: "1.21".into(),
        loader_name: "Fabric".into(),
        loader_version: Some("0.15.0".into()),
        loader_mmc_uid: loader_to_mmc_uid(&Loader::Fabric("0.15.0".into())),
        instance_dir: PathBuf::new(),
        min_memory: 512,
        max_memory: 2048,
        overrides: None,
        icon_src: None,
    };
    let json = build_mmc_pack(&input);
    assert!(json.contains("net.minecraft"));
    assert!(json.contains("net.fabricmc.fabric-loader"));
    assert!(json.contains("0.15.0"));
}

#[test]
fn test_build_instance_cfg() {
    let input = ExportInput {
        uuid: "uuid".into(),
        name: "MiInstancia".into(),
        version_id: "1.20.1".into(),
        mc_version: "1.20.1".into(),
        loader_name: "Vanilla".into(),
        loader_version: None,
        loader_mmc_uid: None,
        instance_dir: PathBuf::new(),
        min_memory: 1024,
        max_memory: 4096,
        overrides: None,
        icon_src: None,
    };
    let cfg = build_instance_cfg(&input);
    assert!(cfg.contains("name=MiInstancia"));
    assert!(cfg.contains("MinMemAlloc=1024"));
    assert!(cfg.contains("MaxMemAlloc=4096"));
    assert!(!cfg.contains("iconKey"));
}
