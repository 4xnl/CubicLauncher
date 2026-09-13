use super::*;

#[test]
fn test_sanitize_absolute_path() {
    let base = Path::new("/tmp/instances/test");
    assert!(sanitize_path(base, Path::new("/etc/passwd")).is_err());
}

#[test]
fn test_sanitize_parent_dir() {
    let base = Path::new("/tmp/instances/test");
    assert!(sanitize_path(base, Path::new("../malicious")).is_err());
}

#[test]
fn test_sanitize_dotdot_nested() {
    let base = Path::new("/tmp/instances/test");
    assert!(sanitize_path(base, Path::new("mods/../../secrets")).is_err());
}

#[test]
fn test_sanitize_valid_sub_path() {
    let base = PathBuf::from("/tmp/instances/test");
    let result = sanitize_path(&base, Path::new("mods")).unwrap();
    assert_eq!(result, base.join("mods"));
}

#[test]
fn test_sanitize_nested_valid() {
    let base = PathBuf::from("/tmp/instances/test");
    let result = sanitize_path(&base, Path::new("screenshots/2025-01-01")).unwrap();
    assert_eq!(result, base.join("screenshots/2025-01-01"));
}

#[test]
fn test_validate_identifier_rejects_dotdot() {
    assert!(validate_identifier("..").is_err());
    assert!(validate_identifier("foo/../bar").is_err());
}

#[test]
fn test_validate_identifier_accepts_safe() {
    assert!(validate_identifier("my_theme").is_ok());
    assert!(validate_identifier("author.theme-v2").is_ok());
}

#[test]
fn test_validate_filename_rejects_path() {
    assert!(validate_filename("foo/bar.jar").is_err());
    assert!(validate_filename("../bar.jar").is_err());
}

#[test]
fn test_validate_filename_accepts_basename() {
    assert!(validate_filename("mod.jar").is_ok());
}
