use super::*;

#[test]
fn test_validate_uuid_valid() {
    assert!(validate_uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
}

#[test]
fn test_validate_uuid_invalid() {
    assert!(validate_uuid("not-a-uuid").is_err());
}

#[test]
fn test_validate_uuid_empty() {
    assert!(validate_uuid("").is_err());
}

#[test]
fn test_sanitize_absolute_path() {
    let base = Path::new("/tmp/instances/test");
    let result = sanitize_sub_path(base, Path::new("/etc/passwd"));
    assert!(result.is_err());
}

#[test]
fn test_sanitize_parent_dir() {
    let base = Path::new("/tmp/instances/test");
    let result = sanitize_sub_path(base, Path::new("../malicious"));
    assert!(result.is_err());
}

#[test]
fn test_sanitize_dotdot_nested() {
    let base = Path::new("/tmp/instances/test");
    let result = sanitize_sub_path(base, Path::new("mods/../../secrets"));
    assert!(result.is_err());
}

#[test]
fn test_sanitize_valid_sub_path() {
    let base = PathBuf::from("/tmp/instances/test");
    let result = sanitize_sub_path(&base, Path::new("mods"));
    assert_eq!(result.unwrap(), base.join("mods"));
}

#[test]
fn test_sanitize_nested_valid() {
    let base = PathBuf::from("/tmp/instances/test");
    let result = sanitize_sub_path(&base, Path::new("screenshots/2025-01-01"));
    assert_eq!(result.unwrap(), base.join("screenshots/2025-01-01"));
}
