use crate::core::path_manager::PathManager;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::warn;

const BUNDLED_LOCALES: [(&str, &str); 2] = [
    ("es-ES", include_str!("../../../src/lib/i18n/es-ES.json")),
    ("en-US", include_str!("../../../src/lib/i18n/en-US.json")),
];

#[derive(Debug, Deserialize)]
struct LocaleMetadata {
    id: String,
}

#[derive(Debug, Serialize)]
pub struct StoredLocale {
    code: String,
    id: String,
    data: String,
}

fn locales_dir() -> PathBuf {
    PathManager::get().get_settings_dir().join("i18n")
}

fn parse_locale_metadata(data: &str) -> Result<(String, String), String> {
    let metadata: LocaleMetadata =
        serde_json::from_str(data).map_err(|e| format!("Invalid locale JSON: {e}"))?;

    if metadata.id.is_empty()
        || !metadata
            .id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
    {
        return Err(format!("Invalid locale id: {}", metadata.id));
    }

    let code = metadata
        .id
        .split('-')
        .next()
        .filter(|code| !code.is_empty())
        .ok_or_else(|| format!("Invalid locale id: {}", metadata.id))?
        .to_ascii_lowercase();

    Ok((code, metadata.id))
}

async fn ensure_bundled_locales(dir: &Path) -> Result<(), String> {
    fs::create_dir_all(dir).await.map_err(|e| e.to_string())?;

    for (id, data) in BUNDLED_LOCALES {
        let path = dir.join(format!("{id}.json"));
        let is_current = fs::read_to_string(&path)
            .await
            .is_ok_and(|stored| stored == data);

        if !is_current {
            fs::write(path, data).await.map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

async fn read_stored_locales(dir: &Path) -> Result<Vec<StoredLocale>, String> {
    let mut entries = fs::read_dir(dir).await.map_err(|e| e.to_string())?;
    let mut paths = Vec::new();

    while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
        let path = entry.path();
        let is_json = path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("json"));

        if entry
            .file_type()
            .await
            .map_err(|e| e.to_string())?
            .is_file()
            && is_json
        {
            paths.push(path);
        }
    }

    let mut locales = BTreeMap::new();
    for original_path in paths {
        let data = match fs::read_to_string(&original_path).await {
            Ok(data) => data,
            Err(error) => {
                warn!(path = %original_path.display(), %error, "Could not read locale file");
                continue;
            }
        };
        let (code, id) = match parse_locale_metadata(&data) {
            Ok(metadata) => metadata,
            Err(error) => {
                warn!(path = %original_path.display(), %error, "Ignoring invalid locale file");
                continue;
            }
        };

        let canonical_path = dir.join(format!("{id}.json"));
        if original_path != canonical_path {
            if canonical_path.exists() {
                if let Err(error) = fs::remove_file(&original_path).await {
                    warn!(path = %original_path.display(), %error, "Could not remove legacy locale file");
                }
                continue;
            }

            if let Err(error) = fs::rename(&original_path, &canonical_path).await {
                warn!(
                    from = %original_path.display(),
                    to = %canonical_path.display(),
                    %error,
                    "Could not migrate locale file"
                );
            }
        }

        locales.insert(id.clone(), StoredLocale { code, id, data });
    }

    Ok(locales.into_values().collect())
}

async fn save_locale_to(dir: &Path, data: String) -> Result<(), String> {
    let (code, id) = parse_locale_metadata(&data)?;
    fs::create_dir_all(dir).await.map_err(|e| e.to_string())?;

    let path = dir.join(format!("{id}.json"));
    fs::write(&path, data).await.map_err(|e| e.to_string())?;

    let legacy_path = dir.join(format!("{code}.json"));
    if legacy_path != path
        && legacy_path.exists()
        && let Err(error) = fs::remove_file(&legacy_path).await
    {
        warn!(path = %legacy_path.display(), %error, "Could not remove legacy locale file");
    }

    Ok(())
}

#[tauri::command]
pub async fn save_locale(data: String) -> Result<(), String> {
    save_locale_to(&locales_dir(), data).await
}

#[tauri::command]
pub async fn load_locales() -> Result<Vec<StoredLocale>, String> {
    let dir = locales_dir();
    ensure_bundled_locales(&dir).await?;
    read_stored_locales(&dir).await
}

#[cfg(test)]
#[path = "../tests/commands/i18n.rs"]
mod tests;
