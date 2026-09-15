use super::*;

#[test]
fn parses_catalog_and_deduplicates_mirrors() {
    let html = r#"<a href="adloadx?f=OptiFine_1.20.1_HD_U_I6.jar&x=abc">Download</a>
        <a href='adloadx?f=OptiFine_1.20.1_HD_U_I6.jar'>Mirror</a>
        <a href='adloadx?f=preview_OptiFine_1.21_HD_U_J1_pre9.jar'>Preview</a>"#;
    let versions = parse_catalog(html).unwrap();
    assert_eq!(versions.len(), 2);
    assert_eq!(versions[0].version_id, "1.20.1-OptiFine_HD_U_I6");
    assert!(versions[0].stable);
    assert_eq!(versions[1].optifine_version, "HD_U_J1_pre9");
    assert!(!versions[1].stable);
    assert!(parse_catalog("Service unavailable").is_err());
}

#[test]
fn rejects_invalid_filenames_and_foreign_download_links() {
    for filename in [
        "../OptiFine_1.20.1_HD_U_I6.jar",
        "OptiFine_1.20.1_HD_U_../I6.jar",
        "OptiFine__HD_U_I6.jar",
    ] {
        assert!(OptiFineVersion::from_filename(filename).is_none());
    }
    let filename = "OptiFine_1.20.1_HD_U_I6.jar";
    let html = format!(
        r#"<a href='https://example.com/downloadx?f={filename}&x=bad'>x</a>
        <a href="downloadx?f={filename}&amp;x=token">Download</a>"#
    );
    assert_eq!(
        parse_download_url(&html, filename).unwrap(),
        format!("https://optifine.net/downloadx?f={filename}&x=token")
    );
    assert!(parse_download_url("<a href='downloadx?f=other.jar&x=token'>", filename).is_err());
}
