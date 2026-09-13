use serde::Serialize;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU8, Ordering};

const STATUS_OFF: u8 = 0;
const STATUS_STARTING: u8 = 1;
const STATUS_STARTED: u8 = 2;
const STATUS_ERROR: u8 = 3;

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum InstanceStatus {
    Off,
    Starting,
    Started,
    Error(String),
}

/// Status sin lock para lecturas frecuentes (polling).
/// Escribe el mensaje de error ANTES de cambiar el AtomicU8
/// para garantizar consistencia con el ordering Release/Acquire.
pub(crate) struct AtomicStatus {
    state: AtomicU8,
    error: Mutex<String>,
}

impl AtomicStatus {
    pub fn new() -> Self {
        Self {
            state: AtomicU8::new(STATUS_OFF),
            error: Mutex::new(String::new()),
        }
    }

    pub fn get(&self) -> InstanceStatus {
        match self.state.load(Ordering::Acquire) {
            STATUS_STARTING => InstanceStatus::Starting,
            STATUS_STARTED => InstanceStatus::Started,
            STATUS_ERROR => {
                let msg = self.error.lock().unwrap_or_else(|e| e.into_inner()).clone();
                InstanceStatus::Error(msg)
            }
            _ => InstanceStatus::Off,
        }
    }

    pub fn set(&self, status: InstanceStatus) {
        match &status {
            InstanceStatus::Off => self.state.store(STATUS_OFF, Ordering::Release),
            InstanceStatus::Starting => self.state.store(STATUS_STARTING, Ordering::Release),
            InstanceStatus::Started => self.state.store(STATUS_STARTED, Ordering::Release),
            InstanceStatus::Error(e) => {
                *self.error.lock().unwrap_or_else(|e| e.into_inner()) = e.clone();
                self.state.store(STATUS_ERROR, Ordering::Release);
            }
        }
    }
}

impl InstanceStatus {
    pub fn is_busy(&self) -> bool {
        matches!(self, InstanceStatus::Starting | InstanceStatus::Started)
    }
}

#[cfg(test)]
#[path = "../../tests/services/instance_manager/status.rs"]
mod tests;
