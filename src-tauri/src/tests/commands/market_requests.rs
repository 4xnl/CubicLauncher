use super::*;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[test]
fn market_request_accepts_frontend_command_and_argument_names() {
    let request: MarketRequest = serde_json::from_value(serde_json::json!({
        "command": "search_curseforge",
        "args": { "query": "sodium", "loader": "fabric", "gameVersion": "1.21.1", "category": null, "index": "relevance", "limit": 20, "offset": 40 }
    })).unwrap();
    assert!(matches!(
        request,
        MarketRequest::SearchCurseforge { offset: 40, .. }
    ));
    assert!(
        serde_json::from_value::<MarketRequest>(serde_json::json!({
            "command": "download_mods", "args": {}
        }))
        .is_err()
    );
}

#[tokio::test]
async fn market_request_cancellation_drops_work_and_unregisters() {
    struct Dropped(Arc<AtomicBool>);
    impl Drop for Dropped {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }
    let key = ("test".into(), uuid::Uuid::new_v4().to_string());
    let dropped = Arc::new(AtomicBool::new(false));
    let flag = Dropped(dropped.clone());
    let task = tokio::spawn(run(key.clone(), async move {
        let _flag = flag;
        std::future::pending::<Result<(), String>>().await
    }));
    while !REQUESTS.lock().active.contains_key(&key) {
        tokio::task::yield_now().await;
    }
    cancel(key.clone());
    assert_eq!(task.await.unwrap(), Err(CANCELLED.into()));
    assert!(dropped.load(Ordering::SeqCst));
    assert!(!REQUESTS.lock().active.contains_key(&key));
}

#[tokio::test]
async fn market_request_cancel_before_registration_and_window_isolation() {
    let id = uuid::Uuid::new_v4().to_string();
    cancel(("first".into(), id.clone()));
    assert_eq!(
        run(("second".into(), id.clone()), async { Ok(42) }).await,
        Ok(42)
    );
    assert_eq!(
        run(("first".into(), id), async { Ok(42) }).await,
        Err(CANCELLED.into())
    );
}
