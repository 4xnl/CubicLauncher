use super::*;

fn temp_dir() -> PathBuf {
    std::env::temp_dir().join(format!("cubic-i18n-test-{}", uuid::Uuid::new_v4()))
}

#[tokio::test]
async fn migrates_short_locale_filename() {
    let dir = temp_dir();
    fs::create_dir_all(&dir).await.unwrap();
    fs::write(dir.join("ja.json"), r#"{"id":"ja-JP","version":"1.0.0"}"#)
        .await
        .unwrap();

    let locales = read_stored_locales(&dir).await.unwrap();

    assert_eq!(locales.len(), 1);
    assert_eq!(locales[0].code, "ja");
    assert_eq!(locales[0].id, "ja-JP");
    assert!(dir.join("ja-JP.json").exists());
    assert!(!dir.join("ja.json").exists());
    fs::remove_dir_all(dir).await.unwrap();
}

#[tokio::test]
async fn saves_locale_with_full_id() {
    let dir = temp_dir();
    let data = r#"{"id":"fr-FR","version":"1.0.0"}"#.to_string();

    save_locale_to(&dir, data.clone()).await.unwrap();

    assert_eq!(
        fs::read_to_string(dir.join("fr-FR.json")).await.unwrap(),
        data
    );
    assert!(!dir.join("fr.json").exists());
    fs::remove_dir_all(dir).await.unwrap();
}

#[tokio::test]
async fn seeds_bundled_locales() {
    let dir = temp_dir();

    ensure_bundled_locales(&dir).await.unwrap();

    assert_eq!(
        fs::read_to_string(dir.join("es-ES.json")).await.unwrap(),
        BUNDLED_LOCALES[0].1
    );
    assert_eq!(
        fs::read_to_string(dir.join("en-US.json")).await.unwrap(),
        BUNDLED_LOCALES[1].1
    );
    fs::remove_dir_all(dir).await.unwrap();
}
