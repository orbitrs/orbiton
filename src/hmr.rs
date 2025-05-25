// Hot Module Replacement (HMR) support for the Orbit UI framework

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// HMR update data
#[derive(Debug, Clone)]
pub struct HmrUpdate {
    /// The module path
    pub module: String,
    /// When the update was detected
    pub timestamp: Instant,
    /// Whether the module has been updated
    pub is_updated: bool,
}

/// HMR context manager
#[derive(Debug, Clone)]
pub struct HmrContext {
    /// Modified modules
    modules: Arc<Mutex<HashMap<String, HmrUpdate>>>,
    /// Last full rebuild time
    last_rebuild: Arc<Mutex<Option<Instant>>>,
    /// Project root directory
    project_root: PathBuf,
}

impl Default for HmrContext {
    fn default() -> Self {
        Self::new(PathBuf::from("."))
    }
}

impl HmrContext {
    /// Create a new HMR context
    pub fn new(project_root: PathBuf) -> Self {
        Self {
            modules: Arc::new(Mutex::new(HashMap::new())),
            last_rebuild: Arc::new(Mutex::new(None)),
            project_root,
        }
    }

    /// Record a file change
    pub fn record_file_change(&self, path: &Path) -> Option<String> {
        let rel_path = path.strip_prefix(&self.project_root).ok()?;
        let path_str = rel_path.to_string_lossy().replace('\\', "/");

        // Extract module path for Rust and Orbit files
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy();

            let module = if ext_str == "rs" || ext_str == "orbit" {
                if path_str.starts_with("src/") {
                    Some(
                        path_str
                            .replace("src/", "")
                            .replace(".rs", "")
                            .replace(".orbit", ""),
                    )
                } else {
                    // Not in src directory, might be lib or other code
                    None
                }
            } else {
                // Not a Rust or Orbit file
                None
            };

            if let Some(module_path) = module {
                let mut modules = self.modules.lock().unwrap();
                modules.insert(
                    module_path.clone(),
                    HmrUpdate {
                        module: module_path.clone(),
                        timestamp: Instant::now(),
                        is_updated: false,
                    },
                );
                return Some(module_path);
            }
        }

        None
    }

    /// Mark all modules as updated
    pub fn mark_modules_updated(&self) {
        let mut modules = self.modules.lock().unwrap();
        for update in modules.values_mut() {
            update.is_updated = true;
        }
    }

    /// Check if any modules need updating
    pub fn needs_update(&self) -> bool {
        let modules = self.modules.lock().unwrap();
        modules.values().any(|update| !update.is_updated)
    }

    /// Get pending module updates
    pub fn get_pending_updates(&self) -> Vec<String> {
        let modules = self.modules.lock().unwrap();
        modules
            .values()
            .filter(|update| !update.is_updated)
            .map(|update| update.module.clone())
            .collect()
    }

    /// Record a full rebuild
    pub fn record_rebuild(&self) {
        let mut last_rebuild = self.last_rebuild.lock().unwrap();
        *last_rebuild = Some(Instant::now());

        // Mark all modules as updated when a full rebuild happens
        self.mark_modules_updated();
    }
    /// Check if a rebuild is needed
    pub fn should_rebuild(&self, debounce_time: Duration) -> bool {
        // Check if enough time has passed since last rebuild
        let last_rebuild = self.last_rebuild.lock().unwrap();
        if let Some(instant) = *last_rebuild {
            if instant.elapsed() < debounce_time {
                return false;
            }
        }
        drop(last_rebuild); // Explicitly drop the lock before checking needs_update

        // Check if we have pending updates
        self.needs_update()
    }

    /// Clear all pending updates
    pub fn clear(&self) {
        let mut modules = self.modules.lock().unwrap();
        modules.clear();
    }
}
