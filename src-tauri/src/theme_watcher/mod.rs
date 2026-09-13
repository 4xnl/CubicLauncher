use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, mpsc};
use std::time::{Duration, Instant};

use crate::core::{AppEvent, PathManager, emit, validate_identifier};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use parking_lot::Mutex;
use tracing::warn;

type Notification = (u64, notify::Result<Event>);
static STATE: LazyLock<Mutex<WatchState>> = LazyLock::new(|| Mutex::new(WatchState::default()));

#[derive(Default)]
struct WatchState {
    target: Option<PathBuf>,
    event_root: Option<PathBuf>,
    paused: HashMap<PathBuf, usize>,
    generation: u64,
    watcher: Option<notify::RecommendedWatcher>,
    tx: Option<mpsc::Sender<Notification>>,
    last_event: Option<Instant>,
}

impl WatchState {
    fn select(&mut self, target: Option<PathBuf>) {
        if self.target != target {
            self.target = target;
            self.rearm();
        }
    }

    fn rearm(&mut self) {
        // The lock acknowledges logical invalidation, not OS thread teardown.
        // Each old callback keeps its original generation, even if delayed.
        self.generation += 1;
        self.last_event = None;
        self.watcher = None;
        self.event_root = None;
        let Some(path) = &self.target else { return };
        if self.paused.contains_key(path) {
            return;
        }
        self.event_root = std::fs::canonicalize(path).ok();
        let Some(tx) = self.tx.clone() else { return };
        let generation = self.generation;
        let result = notify::recommended_watcher(move |event| {
            let _ = tx.send((generation, event));
        })
        .and_then(|mut watcher| {
            watcher.watch(path, RecursiveMode::Recursive)?;
            Ok(watcher)
        });
        match result {
            Ok(watcher) => self.watcher = Some(watcher),
            Err(e) => warn!("ThemeWatcher: no se pudo observar {:?}: {}", path, e),
        }
    }

    fn pause(&mut self, path: &Path) {
        *self.paused.entry(path.to_owned()).or_default() += 1;
        if self.target.as_deref() == Some(path) {
            self.rearm();
        }
    }

    fn resume(&mut self, path: &Path) -> bool {
        if let Some(count) = self.paused.get_mut(path) {
            *count -= 1;
            if *count == 0 {
                self.paused.remove(path);
            }
        }
        let active = self.target.as_deref() == Some(path);
        if active {
            self.rearm();
        }
        active
    }

    fn accepts(&self, generation: u64, event: &Event) -> bool {
        generation == self.generation
            && self.target.as_ref().is_some_and(|path| {
                !self.paused.contains_key(path)
                    && event.paths.iter().any(|p| {
                        p.starts_with(path)
                            || self
                                .event_root
                                .as_ref()
                                .is_some_and(|root| p.starts_with(root))
                    })
            })
            && matches!(
                event.kind,
                EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
            )
    }
}

pub struct ThemeWatcher;

pub(crate) struct ImportWatchGuard {
    path: PathBuf,
    successful: bool,
}

impl ImportWatchGuard {
    pub fn finish(mut self) {
        self.successful = true;
    }
}

impl Drop for ImportWatchGuard {
    fn drop(&mut self) {
        let mut state = STATE.lock();
        if state.resume(&self.path) && self.successful {
            emit_change(&self.path);
        }
    }
}

fn emit_change(path: &Path) {
    if let Some(id) = path.file_name().and_then(|name| name.to_str()) {
        emit(AppEvent::ThemeChanged {
            id: format!("user:{id}").into(),
        });
    }
}

impl ThemeWatcher {
    pub fn watch(id: Option<String>) {
        let target = id.and_then(|id| {
            if let Err(e) = validate_identifier(&id) {
                warn!("ThemeWatcher: id invalido: {}", e);
                return None;
            }
            Some(PathManager::get().get_themes_dir().join(id))
        });
        STATE.lock().select(target);
    }

    pub(crate) fn pause_import(path: &Path) -> ImportWatchGuard {
        STATE.lock().pause(path);
        ImportWatchGuard {
            path: path.to_owned(),
            successful: false,
        }
    }

    pub async fn start() {
        let (tx, rx) = mpsc::channel::<Notification>();
        {
            let mut state = STATE.lock();
            if state.tx.is_some() {
                return;
            }
            state.tx = Some(tx);
            state.rearm();
        }
        tokio::task::spawn_blocking(move || {
            loop {
                let notification = rx.recv_timeout(Duration::from_millis(100));
                let mut state = STATE.lock();
                match notification {
                    Ok((generation, Ok(event))) if state.accepts(generation, &event) => {
                        state.last_event = Some(Instant::now());
                    }
                    Ok((generation, Err(e))) if generation == state.generation => {
                        warn!("ThemeWatcher: error de notify: {}", e);
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => return,
                    _ => {}
                }
                if state
                    .last_event
                    .is_some_and(|last| last.elapsed() >= Duration::from_millis(200))
                {
                    state.last_event = None;
                    if let Some(path) = &state.target {
                        emit_change(path);
                    }
                }
            }
        });
    }
}

#[cfg(test)]
#[path = "../tests/theme_watcher.rs"]
mod tests;
