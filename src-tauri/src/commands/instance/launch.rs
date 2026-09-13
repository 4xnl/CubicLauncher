use crate::core::errors::{AppError, InstanceError};
use crate::core::sanitize_path;
use crate::services::{InstanceDto, InstanceManager, InstanceStatus, Launcher, signal_kill};
use std::path::{Path, PathBuf};
use tracing::{error, info, warn};

pub fn validate_uuid(uuid: &str) -> Result<(), String> {
    uuid::Uuid::parse_str(uuid).map_err(|_| format!("UUID inválido: '{}'", uuid))?;
    Ok(())
}

pub fn sanitize_sub_path(instance_dir: &Path, sub_path: &Path) -> Result<PathBuf, String> {
    sanitize_path(instance_dir, sub_path)
}

#[tauri::command]
pub async fn launch(instance_id: String, server_address: Option<String>) -> Result<(), String> {
    validate_uuid(&instance_id)?;
    let server = server_address
        .as_deref()
        .map(crate::services::server_status::ServerAddress::parse)
        .transpose()?;
    info!("Lanzando instancia {}", instance_id);
    let manager = InstanceManager::get();
    let Some(handle) = manager.get_handle(&instance_id).await else {
        error!("Instancia {} no encontrada para lanzar", instance_id);
        return Err(AppError::Instance(InstanceError::NotFound).to_json());
    };

    Launcher::get()
        .launch(handle.clone(), server)
        .await
        .map_err(|e| {
            error!("Error lanzando instancia {}: {}", instance_id, e);
            e.to_json()
        })?;

    info!("Instancia {} lanzada exitosamente", instance_id);
    Ok(())
}

#[tauri::command]
pub async fn kill_instance(uuid: String) -> Result<(), String> {
    validate_uuid(&uuid)?;
    info!("Matando instancia {}", uuid);
    let manager = InstanceManager::get();
    let handle = manager
        .get_handle(&uuid)
        .await
        .ok_or_else(|| InstanceError::NotFound.to_string())?;

    if !signal_kill(&uuid) {
        warn!("Instancia {} no estaba corriendo, forzando Off", uuid);
        handle.set_status(InstanceStatus::Off);
    }
    Ok(())
}

#[tauri::command]
pub async fn get_instances() -> Vec<InstanceDto> {
    InstanceManager::get().get_all_dtos().await
}

#[cfg(test)]
#[path = "../../tests/commands/instance/launch.rs"]
mod tests;
