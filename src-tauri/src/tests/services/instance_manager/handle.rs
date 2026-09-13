use super::*;

#[tokio::test]
async fn world_operation_lock_survives_cancelled_async_caller() {
    let handle = InstanceHandle::new(InstanceData::new("test".into(), "1.21".into(), None));
    let cloned = handle.clone();
    let guard = handle.try_lock_files().unwrap();
    let (started_tx, started_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let (done_tx, done_rx) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(async move {
        tokio::task::spawn_blocking(move || {
            started_tx.send(()).unwrap();
            release_rx.recv().unwrap();
            drop(guard);
            done_tx.send(()).unwrap();
        })
        .await
        .unwrap();
    });
    started_rx.await.unwrap();
    assert!(cloned.try_lock_files().is_err());
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    assert!(handle.try_lock_files().is_err());
    release_tx.send(()).unwrap();
    done_rx.await.unwrap();
    assert!(handle.try_lock_files().is_ok());
}
