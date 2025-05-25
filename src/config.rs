// Configuration system for Orbiton CLI
// Supports .orbiton.toml configuration files for customizing build and dev behavior

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Main configuration structure for Orbiton
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitonConfig {
    /// Project configuration
    #[serde(default)]
    pub project: ProjectConfig,

    /// Development server configuration
    #[serde(default)]
    pub dev_server: DevServerConfig,

    /// Hot Module Reload configuration
    #[serde(default)]
    pub hmr: HmrConfig,

    /// Build configuration
    #[serde(default)]
    pub build: BuildConfig,

    /// Linting configuration
    #[serde(default)]
    pub lint: LintConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project name
    pub name: Option<String>,

    /// Project version
    pub version: Option<String>,

    /// Source directory (default: "src")
    #[serde(default = "default_src_dir")]
    pub src_dir: String,

    /// Output directory for builds (default: "dist")
    #[serde(default = "default_dist_dir")]
    pub dist_dir: String,

    /// Entry point file (default: "main.rs")
    #[serde(default = "default_entry_point")]
    pub entry_point: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevServerConfig {
    /// Port for the development server (default: 3000)
    #[serde(default = "default_dev_port")]
    pub port: u16,

    /// Host for the development server (default: "127.0.0.1")
    #[serde(default = "default_dev_host")]
    pub host: String,

    /// Whether to open browser automatically (default: true)
    #[serde(default = "default_auto_open")]
    pub auto_open: bool,

    /// Additional static file directories to serve
    #[serde(default)]
    pub static_dirs: Vec<String>,

    /// Custom headers to add to responses
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HmrConfig {
    /// Whether HMR is enabled (default: true)
    #[serde(default = "default_hmr_enabled")]
    pub enabled: bool,

    /// Debounce time in milliseconds for file changes (default: 100)
    #[serde(default = "default_hmr_debounce")]
    pub debounce_ms: u64,

    /// Files and patterns to ignore during HMR
    #[serde(default)]
    pub ignore_patterns: Vec<String>,

    /// Whether to preserve component state during HMR (default: true)
    #[serde(default = "default_preserve_state")]
    pub preserve_state: bool,

    /// Maximum number of HMR retries before full reload (default: 3)
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,

    /// Whether to show HMR notifications in browser (default: true)
    #[serde(default = "default_show_notifications")]
    pub show_notifications: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Whether to use beta Rust toolchain (default: false)
    #[serde(default)]
    pub use_beta_toolchain: bool,

    /// Release mode for builds (default: false)
    #[serde(default)]
    pub release: bool,

    /// Target triple for builds
    pub target: Option<String>,

    /// Additional build features to enable
    #[serde(default)]
    pub features: Vec<String>,

    /// Build optimization level (0-3, s, z)
    pub opt_level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintConfig {
    /// Whether linting is enabled (default: true)
    #[serde(default = "default_lint_enabled")]
    pub enabled: bool,

    /// Lint rules to enable/disable
    #[serde(default)]
    pub rules: HashMap<String, bool>,

    /// Custom lint configuration
    #[serde(default)]
    pub custom_rules: Vec<String>,
}

// Default value functions
fn default_src_dir() -> String {
    "src".to_string()
}
fn default_dist_dir() -> String {
    "dist".to_string()
}
fn default_entry_point() -> String {
    "main.rs".to_string()
}
fn default_dev_port() -> u16 {
    3000
}
fn default_dev_host() -> String {
    "127.0.0.1".to_string()
}
fn default_auto_open() -> bool {
    true
}
fn default_hmr_enabled() -> bool {
    true
}
fn default_hmr_debounce() -> u64 {
    100
}
fn default_preserve_state() -> bool {
    true
}
fn default_max_retries() -> u32 {
    3
}
fn default_show_notifications() -> bool {
    true
}
fn default_lint_enabled() -> bool {
    true
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: None,
            version: None,
            src_dir: default_src_dir(),
            dist_dir: default_dist_dir(),
            entry_point: default_entry_point(),
        }
    }
}

impl Default for DevServerConfig {
    fn default() -> Self {
        Self {
            port: default_dev_port(),
            host: default_dev_host(),
            auto_open: default_auto_open(),
            static_dirs: vec![],
            headers: HashMap::new(),
        }
    }
}

impl Default for HmrConfig {
    fn default() -> Self {
        Self {
            enabled: default_hmr_enabled(),
            debounce_ms: default_hmr_debounce(),
            ignore_patterns: vec![
                "target/**".to_string(),
                "**/.git/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/*.log".to_string(),
            ],
            preserve_state: default_preserve_state(),
            max_retries: default_max_retries(),
            show_notifications: default_show_notifications(),
        }
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            use_beta_toolchain: false,
            release: false,
            target: None,
            features: vec![],
            opt_level: None,
        }
    }
}

impl Default for LintConfig {
    fn default() -> Self {
        Self {
            enabled: default_lint_enabled(),
            rules: HashMap::new(),
            custom_rules: vec![],
        }
    }
}

impl Default for OrbitonConfig {
    fn default() -> Self {
        Self {
            project: ProjectConfig::default(),
            dev_server: DevServerConfig::default(),
            hmr: HmrConfig::default(),
            build: BuildConfig::default(),
            lint: LintConfig::default(),
        }
    }
}

impl OrbitonConfig {
    /// Load configuration from a .orbiton.toml file
    ///
    /// Searches for the configuration file in the following order:
    /// 1. Current directory
    /// 2. Parent directories (walking up the tree)
    /// 3. Uses default configuration if no file found
    pub fn load_from_project(project_dir: &Path) -> Result<Self> {
        let config_path = Self::find_config_file(project_dir);

        match config_path {
            Some(path) => Self::load_from_file(&path),
            None => {
                println!("No .orbiton.toml found, using default configuration");
                Ok(Self::default())
            }
        }
    }

    /// Load configuration from a specific file
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: OrbitonConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        println!("Loaded configuration from: {}", path.display());
        Ok(config)
    }

    /// Find the nearest .orbiton.toml file by walking up the directory tree
    pub fn find_config_file(start_dir: &Path) -> Option<PathBuf> {
        let mut current_dir = start_dir;

        loop {
            let config_path = current_dir.join(".orbiton.toml");
            if config_path.exists() {
                return Some(config_path);
            }

            match current_dir.parent() {
                Some(parent) => current_dir = parent,
                None => return None,
            }
        }
    }

    /// Save configuration to a file
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).context("Failed to serialize configuration")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        println!("Configuration saved to: {}", path.display());
        Ok(())
    }

    /// Create a default configuration file in the specified directory
    pub fn create_default_config(project_dir: &Path) -> Result<PathBuf> {
        let config_path = project_dir.join(".orbiton.toml");
        let default_config = Self::default();

        default_config.save_to_file(&config_path)?;
        Ok(config_path)
    }
    /// Merge with another configuration (other takes precedence)
    #[allow(dead_code)] // Used in tests and maintenance operations
    pub fn merge_with(&mut self, other: &OrbitonConfig) {
        // Merge project config
        if other.project.name.is_some() {
            self.project.name = other.project.name.clone();
        }
        if other.project.version.is_some() {
            self.project.version = other.project.version.clone();
        }

        // Merge dev server config
        if other.dev_server.port != default_dev_port() {
            self.dev_server.port = other.dev_server.port;
        }
        if other.dev_server.host != default_dev_host() {
            self.dev_server.host = other.dev_server.host.clone();
        }

        // Merge HMR config
        if !other.hmr.enabled {
            self.hmr.enabled = other.hmr.enabled;
        }
        if other.hmr.debounce_ms != default_hmr_debounce() {
            self.hmr.debounce_ms = other.hmr.debounce_ms;
        }

        // Merge ignore patterns
        if !other.hmr.ignore_patterns.is_empty() {
            self.hmr
                .ignore_patterns
                .extend(other.hmr.ignore_patterns.clone());
        }

        // Merge build config
        if other.build.use_beta_toolchain {
            self.build.use_beta_toolchain = other.build.use_beta_toolchain;
        }
        if other.build.release {
            self.build.release = other.build.release;
        }

        // Merge lint config
        if !other.lint.enabled {
            self.lint.enabled = other.lint.enabled;
        }
        for (rule, enabled) in &other.lint.rules {
            self.lint.rules.insert(rule.clone(), *enabled);
        }
    }

    /// Validate configuration and return any errors
    pub fn validate(&self) -> Result<()> {
        // Validate port range
        if self.dev_server.port == 0 {
            return Err(anyhow::anyhow!("Dev server port cannot be 0"));
        }

        // Validate paths exist
        let src_path = Path::new(&self.project.src_dir);
        if !src_path.exists() && self.project.src_dir != "src" {
            return Err(anyhow::anyhow!(
                "Source directory does not exist: {}",
                self.project.src_dir
            ));
        }

        // Validate HMR settings
        if self.hmr.debounce_ms > 5000 {
            return Err(anyhow::anyhow!(
                "HMR debounce time too high (max 5000ms): {}",
                self.hmr.debounce_ms
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = OrbitonConfig::default();
        assert_eq!(config.dev_server.port, 3000);
        assert_eq!(config.dev_server.host, "127.0.0.1");
        assert!(config.hmr.enabled);
        assert_eq!(config.hmr.debounce_ms, 100);
    }

    #[test]
    fn test_config_serialization() {
        let config = OrbitonConfig::default();
        let toml_str = toml::to_string(&config).unwrap();
        let parsed: OrbitonConfig = toml::from_str(&toml_str).unwrap();

        assert_eq!(config.dev_server.port, parsed.dev_server.port);
        assert_eq!(config.hmr.enabled, parsed.hmr.enabled);
    }

    #[test]
    fn test_config_file_operations() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join(".orbiton.toml");

        let config = OrbitonConfig::default();
        config.save_to_file(&config_path).unwrap();

        assert!(config_path.exists());

        let loaded_config = OrbitonConfig::load_from_file(&config_path).unwrap();
        assert_eq!(config.dev_server.port, loaded_config.dev_server.port);
    }

    #[test]
    fn test_config_validation() {
        let mut config = OrbitonConfig::default();
        assert!(config.validate().is_ok());

        config.dev_server.port = 0;
        assert!(config.validate().is_err());

        config.dev_server.port = 3000;
        config.hmr.debounce_ms = 10000;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_merge() {
        let mut base_config = OrbitonConfig::default();
        let mut override_config = OrbitonConfig::default();

        override_config.dev_server.port = 8080;
        override_config.hmr.enabled = false;

        base_config.merge_with(&override_config);

        assert_eq!(base_config.dev_server.port, 8080);
        assert!(!base_config.hmr.enabled);
    }
}
