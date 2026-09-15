//! Bounded serialized API payloads. One weak sweeper per live cache also expires
//! entries while the market is idle; no task is created per entry or request.
use parking_lot::Mutex;
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

const MAX_ENTRIES: usize = 200;
const MAX_ENTRY_BYTES: usize = 1024 * 1024;
const TTL: Duration = Duration::from_secs(300);

struct Entry {
    data: Arc<[u8]>,
    inserted: Instant,
    used: Instant,
}
#[derive(Default)]
struct Entries {
    map: HashMap<String, Entry>,
    bytes: usize,
}

pub(crate) struct JsonCache {
    entries: Mutex<Entries>,
    max_bytes: usize,
    sweeping: AtomicBool,
}

impl JsonCache {
    pub(crate) fn new(max_bytes: usize) -> Arc<Self> {
        Arc::new(Self {
            entries: Mutex::new(Entries::default()),
            max_bytes,
            sweeping: AtomicBool::new(false),
        })
    }

    fn expire(&self) {
        let mut entries = self.entries.lock();
        entries
            .map
            .retain(|_, entry| entry.inserted.elapsed() < TTL);
        entries.bytes = entries
            .map
            .iter()
            .map(|(key, entry)| key.len() + entry.data.len())
            .sum();
    }

    pub(crate) fn get(&self, key: &str) -> Option<Value> {
        self.expire();
        let data = {
            let mut entries = self.entries.lock();
            let entry = entries.map.get_mut(key)?;
            entry.used = Instant::now();
            Arc::clone(&entry.data)
        };
        serde_json::from_slice(&data).ok()
    }

    pub(crate) fn set(self: &Arc<Self>, key: String, value: impl std::borrow::Borrow<Value>) {
        let Ok(data) = serde_json::to_vec(value.borrow()) else {
            return;
        };
        let size = key.len() + data.len();
        if size > MAX_ENTRY_BYTES || size > self.max_bytes {
            return;
        }
        self.expire();
        {
            let mut entries = self.entries.lock();
            if let Some(old) = entries.map.remove(&key) {
                entries.bytes -= key.len() + old.data.len();
            }
            while entries.map.len() >= MAX_ENTRIES || entries.bytes + size > self.max_bytes {
                let Some(oldest) = entries
                    .map
                    .iter()
                    .min_by_key(|(_, entry)| entry.used)
                    .map(|(key, _)| key.clone())
                else {
                    break;
                };
                if let Some(old) = entries.map.remove(&oldest) {
                    entries.bytes -= oldest.len() + old.data.len();
                }
            }
            let now = Instant::now();
            entries.map.insert(
                key,
                Entry {
                    data: data.into(),
                    inserted: now,
                    used: now,
                },
            );
            entries.bytes += size;
        }
        if let Ok(runtime) = tokio::runtime::Handle::try_current()
            && !self.sweeping.swap(true, Ordering::Relaxed)
        {
            let weak = Arc::downgrade(self);
            runtime.spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(60)).await;
                    let Some(cache) = weak.upgrade() else { break };
                    cache.expire();
                }
            });
        }
    }
}

#[cfg(test)]
#[path = "../tests/core/market_cache.rs"]
mod tests;
