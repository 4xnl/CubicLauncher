use super::*;
use crate::progress::DownloadProgress;
use std::time::Duration;
use tokio::sync::watch;

#[tokio::test]
async fn required_item_failure_does_not_hang() {
    let temp_dir = std::env::temp_dir().join(format!("aqua-test-failure-{}", uuid::Uuid::new_v4()));
    tokio::fs::create_dir_all(&temp_dir).await.unwrap();

    let manager = DownloadManager::new(temp_dir.clone());

    // Empty URL + required item forces `download_file_with_headers` to
    // fail immediately, which previously left the progress forwarder
    // looping forever and blocked callers waiting on the watch channel.
    let item = DownloadItemSpec::new("", temp_dir.join("missing.jar"), "missing-library")
        .with_hash("deadbeef");
    let batch = GenericBatch::new("test-batch", vec![item]);

    let handle = manager.prepare_batch(Box::new(batch)).await.unwrap();

    let (tx, _rx) = watch::channel(DownloadProgress::empty(handle.progress().1));

    let result = tokio::time::timeout(Duration::from_secs(5), handle.download_all(Some(tx))).await;

    let _ = tokio::fs::remove_dir_all(&temp_dir).await;

    assert!(
        result.is_ok(),
        "download_all timed out (progress forwarder deadlock?)"
    );
    assert!(
        result.unwrap().is_err(),
        "download_all should have returned an error"
    );
}
