use super::*;

#[tokio::test]
async fn test_preview_session_lifecycle() {
    let dir = std::env::temp_dir().join(format!("cubic_preview_test_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("instance.cfg"), "name=Test\n").unwrap();

    let token = register_preview(dir.clone());
    assert!(dir.exists());

    let session = take_preview(&token).expect("token should exist");
    assert_eq!(session.preview_dir(), dir);

    session.cleanup().await;
    assert!(!dir.exists());
}

#[tokio::test]
async fn test_cancel_preview() {
    let dir = std::env::temp_dir().join(format!("cubic_preview_test_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();

    let token = register_preview(dir.clone());
    assert!(cancel_preview(&token).await);
    assert!(!dir.exists());
    assert!(!cancel_preview(&token).await);
}
