use std::collections::HashSet;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{AquaError, utilities::HTTP_CLIENT};

const BASE_URL: &str = "https://optifine.net/";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptiFineVersion {
    pub version_id: String,
    pub game_version: String,
    pub optifine_version: String,
    pub filename: String,
    pub stable: bool,
}

impl OptiFineVersion {
    pub fn from_filename(filename: &str) -> Option<Self> {
        // Filenames from the official catalog also become local path components.
        if !filename
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._".contains(&b))
        {
            return None;
        }
        let stable = !filename.starts_with("preview_");
        let name = filename.strip_prefix("preview_").unwrap_or(filename);
        let (game_version, release) = name
            .strip_prefix("OptiFine_")?
            .strip_suffix(".jar")?
            .split_once('_')?;
        if !game_version
            .split('.')
            .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
            || !game_version.contains('.')
            || !release.starts_with("HD_")
            || release.contains("..")
        {
            return None;
        }
        Some(Self {
            version_id: format!("{game_version}-OptiFine_{release}"),
            game_version: game_version.into(),
            optifine_version: release.into(),
            filename: filename.into(),
            stable,
        })
    }
}

pub(crate) async fn fetch_page(url: &str) -> Result<String, AquaError> {
    Ok(HTTP_CLIENT
        .get(url)
        .timeout(Duration::from_secs(30))
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?)
}

fn parse_catalog(html: &str) -> Result<Vec<OptiFineVersion>, AquaError> {
    let mut seen = HashSet::new();
    let versions: Vec<_> = html
        .split("?f=")
        .skip(1)
        .filter_map(|part| {
            let filename = part
                .split(['&', '\"', '\'', '<', '>', ' ', '\n', '\r'])
                .next()?;
            let version = OptiFineVersion::from_filename(filename)?;
            seen.insert(version.version_id.clone()).then_some(version)
        })
        .collect();
    if versions.is_empty() {
        return Err(AquaError::Other(
            "No OptiFine versions found in the official catalog".into(),
        ));
    }
    Ok(versions)
}

pub async fn fetch_optifine_versions() -> Result<Vec<OptiFineVersion>, AquaError> {
    parse_catalog(&fetch_page(&format!("{BASE_URL}downloads")).await?)
}

fn parse_download_url(html: &str, filename: &str) -> Result<String, AquaError> {
    for part in html.split(['\"', '\'']) {
        let part = part.replace("&amp;", "&");
        let Ok(url) = reqwest::Url::parse(BASE_URL).unwrap().join(&part) else {
            continue;
        };
        if url.host_str() == Some("optifine.net")
            && url.path() == "/downloadx"
            && url.query_pairs().any(|(k, v)| k == "f" && v == filename)
            && url.query_pairs().any(|(k, v)| k == "x" && !v.is_empty())
        {
            let mut url = url;
            let _ = url.set_scheme("https");
            return Ok(url.into());
        }
    }
    Err(AquaError::Other(
        "OptiFine download link was not found; refresh the catalog and try again".into(),
    ))
}

pub(crate) async fn resolve_download_url(version: &OptiFineVersion) -> Result<String, AquaError> {
    let html = fetch_page(&format!("{BASE_URL}adloadx?f={}", version.filename)).await?;
    parse_download_url(&html, &version.filename)
}

#[cfg(test)]
#[path = "tests/optifine.rs"]
mod tests;
