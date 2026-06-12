use serde::{Deserialize, Serialize};

use crate::Loader;

/// A structured Minecraft version that includes loader information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameVersion {
    pub mc_version: String,
    pub loader: Loader,
}

impl GameVersion {
    /// Parse a version ID string into structured version info.
    ///
    /// Handles formats:
    ///   "1.20.1"                          → Vanilla
    ///   "fabric-loader-0.15.11-1.20.1"   → Fabric(0.15.11) + 1.20.1
    ///   "1.20.1-forge-47.2.0"            → Forge(47.2.0) + 1.20.1
    ///   "1.21-neoforge-21.0.0"           → NeoForge(21.0.0) + 1.21
    ///   "1.20.1-quilt-0.25.0"            → Quilt(0.25.0) + 1.20.1
    pub fn from_version_id(id: &str) -> Self {
        let loader = Loader::from_version_id(id);

        let mc_version = match &loader {
            Loader::Vanilla => id.to_string(),
            Loader::Fabric(full_id) | Loader::Forge(full_id)
            | Loader::NeoForge(full_id) | Loader::Quilt(full_id) => {
                Self::extract_mc_version(full_id)
            }
        };

        Self { mc_version, loader }
    }

    /// Serialize back to a version ID string suitable for instance storage.
    pub fn to_version_id(&self) -> String {
        match &self.loader {
            Loader::Vanilla => self.mc_version.clone(),
            Loader::Fabric(v) => format!("fabric-loader-{}-{}", v, self.mc_version),
            Loader::Forge(v) => format!("{}-forge-{}", self.mc_version, v),
            Loader::NeoForge(v) => format!("{}-neoforge-{}", self.mc_version, v),
            Loader::Quilt(v) => format!("{}-quilt-{}", self.mc_version, v),
        }
    }

    pub fn display_name(&self) -> String {
        match &self.loader {
            Loader::Vanilla => self.mc_version.clone(),
            Loader::Fabric(v) => format!("{} (Fabric {})", self.mc_version, v),
            Loader::Forge(v) => format!("{} (Forge {})", self.mc_version, v),
            Loader::NeoForge(v) => format!("{} (NeoForge {})", self.mc_version, v),
            Loader::Quilt(v) => format!("{} (Quilt {})", self.mc_version, v),
        }
    }

    fn extract_mc_version(full_id: &str) -> String {
        for prefix in &["fabric-loader-", "forge-", "neoforge-", "quilt-"] {
            if let Some(rest) = full_id.strip_prefix(prefix) {
                return rest.to_string();
            }
        }
        for suffix in &["-forge", "-neoforge", "-quilt"] {
            if let Some(rest) = full_id.strip_suffix(suffix) {
                return rest.to_string();
            }
        }
        full_id
            .split('-')
            .last()
            .unwrap_or(full_id)
            .to_string()
    }
}

impl std::fmt::Display for GameVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
