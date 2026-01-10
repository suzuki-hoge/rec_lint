use std::collections::HashSet;
use std::ffi::OsString;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct RawRootConfig {
    #[serde(default)]
    pub include_extensions: Vec<String>,
    #[serde(default)]
    pub exclude_dirs: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct RootConfig {
    pub include_extensions: HashSet<OsString>,
    pub exclude_dirs: HashSet<String>,
}

impl RawRootConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let content =
            fs::read_to_string(path).with_context(|| format!("Failed to read config file: {}", path.display()))?;
        // Empty file returns default config
        if content.trim().is_empty() || content.trim().starts_with('#') && !content.contains(':') {
            return Ok(Self::default());
        }
        serde_yaml::from_str(&content).with_context(|| format!("Failed to parse YAML: {}", path.display()))
    }
}

impl From<RawRootConfig> for RootConfig {
    fn from(raw: RawRootConfig) -> Self {
        RootConfig {
            include_extensions: raw.include_extensions.into_iter().map(OsString::from).collect(),
            exclude_dirs: raw.exclude_dirs.into_iter().collect(),
        }
    }
}

impl RootConfig {
    /// Check if a file extension should be included
    pub fn should_include_extension(&self, ext: Option<&std::ffi::OsStr>) -> bool {
        if self.include_extensions.is_empty() {
            return true; // No filter means all extensions are allowed
        }
        match ext {
            Some(ext) => {
                let ext_with_dot = format!(".{}", ext.to_string_lossy());
                self.include_extensions.contains(OsString::from(&ext_with_dot).as_os_str())
            }
            None => false, // Files without extension are excluded when filter is set
        }
    }

    /// Check if a directory should be excluded
    pub fn should_exclude_dir(&self, dir_name: &std::ffi::OsStr) -> bool {
        let name = dir_name.to_string_lossy();
        self.exclude_dirs.contains(name.as_ref())
    }
}
