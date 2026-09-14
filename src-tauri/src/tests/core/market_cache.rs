use super::*;

#[test]
fn market_cache_bounds_payload_bytes_and_replaces_entries() {
    let cache = JsonCache::new(100);
    for i in 0..1000 {
        cache.set(i.to_string(), Value::String("x".repeat(20)));
    }
    assert!(cache.entries.lock().bytes <= 100);
    assert!(cache.entries.lock().map.len() <= 4);
    cache.set("999".into(), Value::String("new".into()));
    assert_eq!(cache.get("999"), Some(Value::String("new".into())));
    cache.set("huge".into(), Value::String("x".repeat(101)));
    assert!(cache.get("huge").is_none());
}

#[test]
fn market_cache_expires_without_revisiting_the_same_url() {
    let cache = JsonCache::new(100);
    cache.set("old".into(), Value::Null);
    cache.entries.lock().map.get_mut("old").unwrap().inserted = Instant::now() - TTL;
    cache.expire();
    assert_eq!(cache.entries.lock().bytes, 0);
    assert!(cache.entries.lock().map.is_empty());
}

#[test]
fn market_cache_evicts_least_recently_used_payload() {
    let cache = JsonCache::new(12);
    cache.set("a".into(), Value::Null);
    cache.set("b".into(), Value::Null);
    assert_eq!(cache.get("a"), Some(Value::Null));
    cache.set("c".into(), Value::Null);
    assert!(cache.get("b").is_none());
    assert!(cache.get("a").is_some());
}

#[tokio::test]
async fn market_cache_sweeper_does_not_keep_a_dropped_cache_alive() {
    let cache = JsonCache::new(100);
    let weak = Arc::downgrade(&cache);
    cache.set("a".into(), Value::Null);
    assert!(cache.sweeping.load(Ordering::Relaxed));
    drop(cache);
    tokio::task::yield_now().await;
    assert!(weak.upgrade().is_none());
}
