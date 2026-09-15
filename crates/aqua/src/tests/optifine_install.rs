use super::*;

#[test]
fn profile_preserves_legacy_arguments_and_modern_inheritance() {
    let version = OptiFineVersion::from_filename("OptiFine_1.12.2_HD_U_G5.jar").unwrap();
    let legacy = make_profile(
        &version,
        &json!({"minecraftArguments": "--username ${auth_player_name}"}),
        vec![],
    );
    assert_eq!(
        legacy["minecraftArguments"],
        "--username ${auth_player_name} --tweakClass optifine.OptiFineTweaker"
    );
    assert!(legacy.get("arguments").is_none());
    let modern = make_profile(
        &version,
        &json!({"arguments": {"game": ["--username", "${auth_player_name}"]}}),
        vec![],
    );
    assert!(modern.get("minecraftArguments").is_none());
    assert_eq!(modern["inheritsFrom"], "1.12.2");
    assert_eq!(modern["arguments"]["game"][1], "optifine.OptiFineTweaker");
}

#[tokio::test]
async fn invalid_installer_does_not_publish_a_version() {
    let shared =
        std::env::temp_dir().join(format!("cubic-optifine-invalid-{}", uuid::Uuid::new_v4()));
    let staging = shared.join("temp");
    tokio::fs::create_dir_all(&staging).await.unwrap();
    tokio::fs::write(
        staging.join("installer.jar"),
        b"<html>Download unavailable</html>",
    )
    .await
    .unwrap();
    let version = OptiFineVersion::from_filename("OptiFine_1.20.1_HD_U_I6.jar").unwrap();
    let batch = OptiFineBatch {
        version,
        shared_dir: shared.clone(),
        staging_dir: staging.clone(),
        java_path: "java".into(),
        items: vec![],
    };
    let result = batch.finalize(None).await;
    let published = shared.join("versions").exists();
    let cleaned = !staging.exists();
    let _ = tokio::fs::remove_dir_all(&shared).await;
    assert!(result.is_err());
    assert!(!published);
    assert!(cleaned);
}

/// Exercises the real official catalog, download token, patcher and profile resolver.
/// Set OPTIFINE_TEST_JAVA to a Java executable if it is not available on PATH.
#[tokio::test]
#[ignore = "downloads Minecraft clients and OptiFine installers; requires Java and network"]
async fn optifine_official_install_smoke() {
    let shared = std::env::temp_dir().join(format!("cubic-optifine-test-{}", uuid::Uuid::new_v4()));
    let result = smoke_install(&shared).await;
    let _ = tokio::fs::remove_dir_all(&shared).await;
    result.unwrap();
}

async fn smoke_install(shared: &Path) -> Result<(), AquaError> {
    let catalog = crate::optifine::fetch_optifine_versions().await?;
    let java = PathBuf::from(std::env::var("OPTIFINE_TEST_JAVA").unwrap_or_else(|_| "java".into()));
    for filename in ["OptiFine_1.12.2_HD_U_G5.jar", "OptiFine_1.20.1_HD_U_I6.jar"] {
        let version = catalog
            .iter()
            .find(|v| v.filename == filename)
            .unwrap()
            .clone();
        let (base, raw) = crate::resolve_version_data(&version.game_version).await?;
        let base_dir = shared.join("versions").join(&version.game_version);
        tokio::fs::create_dir_all(&base_dir).await?;
        tokio::fs::write(
            base_dir.join(format!("{}.json", version.game_version)),
            &raw,
        )
        .await?;
        download_file(
            &base.client_jar.url,
            &base_dir.join(format!("{}.jar", version.game_version)),
            &base.client_jar.sha1,
            Some(base.client_jar.size),
            None,
        )
        .await?;
        let id = version.version_id.clone();
        let batch = OptiFineBatch::new(shared, version, java.clone()).await?;
        let handle = crate::DownloadManager::new(shared.into())
            .prepare_batch(Box::new(batch))
            .await?;
        handle.download_all(None).await?;

        let profile = zellkern::VersionManifest::from_file(
            shared.join("versions").join(&id).join(format!("{id}.json")),
        )?;
        let base_manifest = zellkern::VersionManifest::from_bytes(&raw)?;
        let resolved = profile.resolve(&base_manifest);
        assert_eq!(
            resolved.main_class.as_deref(),
            Some("net.minecraft.launchwrapper.Launch")
        );
        assert_eq!(resolved.java_major_version(), base.java_version);
        for lib in profile.libraries.as_ref().unwrap() {
            let path = shared.join("libraries").join(lib.get_path());
            validate_jar(&path)?;
            assert!(
                crate::utilities::verify_file_hash(
                    &path,
                    lib.downloads
                        .as_ref()
                        .unwrap()
                        .artifact
                        .as_ref()
                        .unwrap()
                        .sha1
                        .as_ref()
                        .unwrap()
                )
                .await?
            );
        }
        assert!(
            resolved
                .get_classpath(&shared.join("libraries"))
                .contains("OptiFine-")
        );
        println!("Installed and resolved {id}");
    }
    Ok(())
}
