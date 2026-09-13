use super::*;

#[test]
fn test_download_state_pending() {
    let s = DownloadState::new(Arc::from("1.21"));
    assert_eq!(s.status, DownloadStatus::Pending);
    assert!(s.is_active());
}

#[test]
fn test_download_state_not_active_done() {
    let mut s = DownloadState::new(Arc::from("1.21"));
    s.status = DownloadStatus::Done;
    assert!(!s.is_active());
}

#[test]
fn test_download_state_not_active_error() {
    let mut s = DownloadState::new(Arc::from("1.21"));
    s.status = DownloadStatus::Error("err".into());
    assert!(!s.is_active());
}
