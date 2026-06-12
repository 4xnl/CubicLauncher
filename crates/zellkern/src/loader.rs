use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Loader {
    Vanilla,
    Fabric(String),
    Forge(String),
    NeoForge(String),
    Quilt(String),
}

impl Loader {
    /// Infer loader from a version ID string.
    /// Checks substrings to handle formats like:
    ///   "1.20.1"
    ///   "fabric-loader-0.15.11-1.20.1"
    ///   "1.20.1-forge-47.2.0"
    ///   "1.21-neoforge-21.0.0"
    ///   "1.20.1-quilt-0.25.0"
    pub fn from_version_id(id: &str) -> Self {
        let lower = id.to_lowercase();
        if lower.contains("neoforge") {
            Self::NeoForge(id.to_string())
        } else if lower.contains("forge") {
            Self::Forge(id.to_string())
        } else if lower.contains("quilt") {
            Self::Quilt(id.to_string())
        } else if lower.contains("fabric") {
            Self::Fabric(id.to_string())
        } else {
            Self::Vanilla
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Vanilla => "Vanilla",
            Self::Fabric(_) => "Fabric",
            Self::Forge(_) => "Forge",
            Self::NeoForge(_) => "NeoForge",
            Self::Quilt(_) => "Quilt",
        }
    }

    pub fn version(&self) -> Option<&str> {
        match self {
            Self::Vanilla => None,
            Self::Fabric(v) | Self::Forge(v) | Self::NeoForge(v) | Self::Quilt(v) => {
                Some(v.as_str())
            }
        }
    }

    pub fn is_vanilla(&self) -> bool {
        matches!(self, Self::Vanilla)
    }
}

impl std::fmt::Display for Loader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vanilla => write!(f, "Vanilla"),
            Self::Fabric(v) => write!(f, "Fabric ({v})"),
            Self::Forge(v) => write!(f, "Forge ({v})"),
            Self::NeoForge(v) => write!(f, "NeoForge ({v})"),
            Self::Quilt(v) => write!(f, "Quilt ({v})"),
        }
    }
}
