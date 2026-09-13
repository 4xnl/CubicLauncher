use super::*;

#[test]
fn test_app_error_not_found() {
    let s: String = AppError::Instance(InstanceError::NotFound).into();
    assert_eq!(s, r#"{"code":"INST_NOT_FOUND"}"#);
}

#[test]
fn test_app_error_with_params() {
    let s: String = AppError::Instance(InstanceError::JreNotFound("17".into())).into();
    assert_eq!(
        s,
        r#"{"code":"INST_JRE_MISSING","params":{"version":"17"}}"#
    );
}

#[test]
fn test_fs_error() {
    let s: String = AppError::Fs(FsError::NotFound("/x".into())).into();
    assert_eq!(s, r#"{"code":"FS_NOT_FOUND","params":{"path":"/x"}}"#);
}

#[test]
fn test_auth_error() {
    let s: String = AppError::Auth(AuthError::SaveTokensFailed("oops".into())).into();
    assert_eq!(
        s,
        r#"{"code":"AUTH_TOKENS_SAVE","params":{"error":"oops"}}"#
    );
}

#[test]
fn test_auth_session_expired_error() {
    let s: String = AppError::Auth(AuthError::SessionExpired("token revoked".into())).into();
    assert_eq!(
        s,
        r#"{"code":"AUTH_SESSION_EXPIRED","params":{"error":"token revoked"}}"#
    );
}

#[test]
fn test_download_error() {
    let s: String = AppError::Download(DownloadError::NoFabricLoader).into();
    assert_eq!(s, r#"{"code":"DL_NO_FABRIC"}"#);
}

#[test]
fn test_core_error() {
    let s: String = CoreError::LockPoisoned("oh no".into()).into();
    assert_eq!(s, r#"{"code":"CORE_LOCK","params":{"error":"oh no"}}"#);
}

#[test]
fn test_instance_error_into_string() {
    let s: String = InstanceError::NotFound.into();
    assert_eq!(s, r#"{"code":"INST_NOT_FOUND"}"#);
}

#[test]
fn test_fs_nested_in_instance() {
    let s: String = AppError::Instance(InstanceError::Fs(FsError::NotFound("/tmp".into()))).into();
    assert_eq!(s, r#"{"code":"FS_NOT_FOUND","params":{"path":"/tmp"}}"#);
}

#[test]
fn test_json_escaping() {
    let s: String = InstanceError::InstNameParse(r#"bad"name"#.into()).into();
    assert!(s.contains(r#"bad\"name"#));
}
