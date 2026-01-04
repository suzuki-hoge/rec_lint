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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    // =========================================================================
    // RawRootConfig パース
    // =========================================================================

    #[test]
    fn 空のYAMLをパースするとデフォルト設定が生成される() {
        let raw: RawRootConfig = serde_yaml::from_str("").unwrap();
        assert!(raw.include_extensions.is_empty());
        assert!(raw.exclude_dirs.is_empty());
    }

    #[test]
    fn コメントのみのYAMLをパースするとデフォルト設定が生成される() {
        let content = "# yaml-language-server: $schema=...";
        let raw: RawRootConfig = serde_yaml::from_str(content).unwrap();
        assert!(raw.include_extensions.is_empty());
        assert!(raw.exclude_dirs.is_empty());
    }

    #[test]
    fn include_extensionsをパースできる() {
        let content = r#"
include_extensions:
  - .java
  - .kt
"#;
        let raw: RawRootConfig = serde_yaml::from_str(content).unwrap();
        assert_eq!(raw.include_extensions, vec![".java", ".kt"]);
    }

    #[test]
    fn exclude_dirsをパースできる() {
        let content = r#"
exclude_dirs:
  - target
  - node_modules
"#;
        let raw: RawRootConfig = serde_yaml::from_str(content).unwrap();
        assert_eq!(raw.exclude_dirs, vec!["target", "node_modules"]);
    }

    #[test]
    fn 完全な設定をパースできる() {
        let content = r#"
include_extensions:
  - .java
  - .kt
  - .rs
exclude_dirs:
  - target
  - node_modules
  - .git
"#;
        let raw: RawRootConfig = serde_yaml::from_str(content).unwrap();
        assert_eq!(raw.include_extensions, vec![".java", ".kt", ".rs"]);
        assert_eq!(raw.exclude_dirs, vec!["target", "node_modules", ".git"]);
    }

    // =========================================================================
    // RootConfig 変換
    // =========================================================================

    #[test]
    fn RawRootConfigからRootConfigに変換できる() {
        let raw = RawRootConfig {
            include_extensions: vec![".java".to_string(), ".kt".to_string()],
            exclude_dirs: vec!["target".to_string()],
        };
        let config = RootConfig::from(raw);
        assert_eq!(config.include_extensions.len(), 2);
        assert!(config.include_extensions.contains(OsString::from(".java").as_os_str()));
        assert!(config.include_extensions.contains(OsString::from(".kt").as_os_str()));
        assert_eq!(config.exclude_dirs.len(), 1);
        assert!(config.exclude_dirs.contains("target"));
    }

    // =========================================================================
    // should_include_extension
    // =========================================================================

    #[test]
    fn 空のinclude_extensionsは全ての拡張子を許可する() {
        let config = RootConfig::default();
        assert!(config.should_include_extension(Some(std::ffi::OsStr::new("java"))));
        assert!(config.should_include_extension(Some(std::ffi::OsStr::new("kt"))));
        assert!(config.should_include_extension(None));
    }

    #[test]
    fn 指定した拡張子のみを許可する() {
        let config = RootConfig {
            include_extensions: [".java", ".kt"].iter().map(|s| OsString::from(*s)).collect(),
            exclude_dirs: HashSet::new(),
        };
        assert!(config.should_include_extension(Some(std::ffi::OsStr::new("java"))));
        assert!(config.should_include_extension(Some(std::ffi::OsStr::new("kt"))));
        assert!(!config.should_include_extension(Some(std::ffi::OsStr::new("rs"))));
    }

    #[test]
    fn フィルタ設定時に拡張子なしファイルは除外される() {
        let config = RootConfig {
            include_extensions: [".java"].iter().map(|s| OsString::from(*s)).collect(),
            exclude_dirs: HashSet::new(),
        };
        assert!(!config.should_include_extension(None));
    }

    // =========================================================================
    // should_exclude_dir
    // =========================================================================

    #[test]
    fn 指定したディレクトリを除外する() {
        let config = RootConfig {
            include_extensions: HashSet::new(),
            exclude_dirs: ["target", "node_modules"].iter().map(|s| s.to_string()).collect(),
        };
        assert!(config.should_exclude_dir(std::ffi::OsStr::new("target")));
        assert!(config.should_exclude_dir(std::ffi::OsStr::new("node_modules")));
        assert!(!config.should_exclude_dir(std::ffi::OsStr::new("src")));
    }

    #[test]
    fn 空のexclude_dirsは何も除外しない() {
        let config = RootConfig::default();
        assert!(!config.should_exclude_dir(std::ffi::OsStr::new("target")));
        assert!(!config.should_exclude_dir(std::ffi::OsStr::new("node_modules")));
    }
}
