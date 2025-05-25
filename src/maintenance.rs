// Maintenance utilities for the development server
// This module provides cleanup and maintenance functionality for HMR and project state

use console::style;
use log::{info, warn};
use std::path::Path;
use std::time::Duration;

use crate::config::OrbitonConfig;
use crate::dev_server::DevServer;
use crate::hmr::HmrContext;

/// Maintenance operations for the development environment
pub struct MaintenanceManager {
    hmr_context: HmrContext,
    config: OrbitonConfig,
}

impl MaintenanceManager {
    /// Create a new maintenance manager
    pub fn new(project_dir: &Path) -> anyhow::Result<Self> {
        let config = OrbitonConfig::load_from_project(project_dir)?;
        let hmr_context = HmrContext::new(project_dir.to_path_buf());

        Ok(Self {
            hmr_context,
            config,
        })
    }

    /// Perform cleanup of stale HMR updates
    pub fn cleanup_stale_updates(&self, max_age: Duration) {
        info!("Cleaning up stale HMR updates older than {:?}", max_age);

        let stale_modules = self.hmr_context.get_stale_updates(max_age);
        if !stale_modules.is_empty() {
            println!(
                "{} Removing {} stale modules: {}",
                style("Cleanup:").bold().yellow(),
                stale_modules.len(),
                stale_modules.join(", ")
            );

            self.hmr_context.clear_stale_updates(max_age);
        } else {
            println!("{} No stale modules found", style("Info:").bold().blue());
        }
    }

    /// Clear all pending HMR updates
    pub fn clear_all_updates(&self) {
        info!("Clearing all pending HMR updates");
        self.hmr_context.clear();
        println!(
            "{} All HMR updates cleared",
            style("Cleanup:").bold().green()
        );
    }
    /// Get information about the oldest pending update
    #[allow(dead_code)] // Used in tests and maintenance operations
    pub fn get_update_info(&self) -> Option<Duration> {
        let oldest_age = self.hmr_context.get_oldest_update_age();
        if let Some(age) = oldest_age {
            println!(
                "{} Oldest pending update is {:?} old",
                style("Info:").bold().blue(),
                age
            );
        } else {
            println!("{} No pending updates", style("Info:").bold().blue());
        }
        oldest_age
    }
    /// Merge configuration with override settings
    #[allow(dead_code)] // Used in tests and maintenance operations
    pub fn apply_config_overrides(&mut self, overrides: OrbitonConfig) {
        info!("Applying configuration overrides");
        self.config.merge_with(&overrides);

        println!(
            "{} Configuration updated with overrides",
            style("Config:").bold().green()
        );
    }
    /// Create a simple dev server for testing (uses DevServer::new)
    #[allow(dead_code)] // Used in tests and maintenance operations
    pub fn create_simple_dev_server(
        &self,
        port: u16,
        project_dir: &Path,
    ) -> anyhow::Result<DevServer> {
        info!("Creating simple development server on port {}", port);
        DevServer::new(port, project_dir)
    }
    /// Perform automated maintenance based on configuration
    #[allow(dead_code)] // Used in tests and maintenance operations
    pub fn perform_automated_maintenance(&self) -> anyhow::Result<()> {
        info!("Performing automated maintenance");

        // Get cleanup threshold from config (default to 5 minutes)
        let cleanup_threshold = Duration::from_millis(self.config.hmr.debounce_ms * 300); // 300x debounce time

        // Cleanup stale updates
        self.cleanup_stale_updates(cleanup_threshold);

        // Show update info
        self.get_update_info();

        // If we have too many pending updates, warn about it
        let pending_count = self.hmr_context.get_pending_updates().len();
        if pending_count > 10 {
            warn!(
                "High number of pending updates ({}), consider restarting the dev server",
                pending_count
            );

            println!(
                "{} {} pending updates - consider restarting for optimal performance",
                style("Warning:").bold().yellow(),
                pending_count
            );
        }

        Ok(())
    }

    /// Show maintenance status information
    pub fn show_status(&self) {
        info!("Displaying maintenance status");

        println!("{}", style("=== Maintenance Status ===").bold().cyan());

        // Show HMR status
        let pending_updates = self.hmr_context.get_pending_updates();
        println!(
            "{} {} pending HMR updates",
            style("HMR:").bold().blue(),
            pending_updates.len()
        );

        if !pending_updates.is_empty() {
            println!("  Modules: {}", pending_updates.join(", "));

            if let Some(oldest_age) = self.hmr_context.get_oldest_update_age() {
                println!("  Oldest update: {:?} ago", oldest_age);
            }
        }

        // Show configuration status
        println!(
            "{} Port: {}, HMR: {}",
            style("Config:").bold().blue(),
            self.config.dev_server.port,
            if self.config.hmr.enabled {
                "enabled"
            } else {
                "disabled"
            }
        );
        println!(
            "  Debounce: {}ms, Source dir: {}",
            self.config.hmr.debounce_ms, self.config.project.src_dir
        );

        // Show stale update information
        let stale_threshold = Duration::from_secs(300); // 5 minutes
        let stale_updates = self.hmr_context.get_stale_updates(stale_threshold);
        if !stale_updates.is_empty() {
            println!(
                "{} {} stale updates (older than 5 minutes)",
                style("Warning:").bold().yellow(),
                stale_updates.len()
            );
        }

        println!("{}", style("==========================").bold().cyan());
    }
    /// Get the configuration for external use
    #[allow(dead_code)] // Used in tests and maintenance operations
    pub fn config(&self) -> &OrbitonConfig {
        &self.config
    }

    /// Get the HMR context for external use
    #[allow(dead_code)] // Used in tests and maintenance operations
    pub fn hmr_context(&self) -> &HmrContext {
        &self.hmr_context
    }
}

/// Utility function to create maintenance manager and perform cleanup
#[allow(dead_code)] // Used in tests and maintenance operations
pub fn perform_project_maintenance(project_dir: &Path) -> anyhow::Result<()> {
    let manager = MaintenanceManager::new(project_dir)?;
    manager.perform_automated_maintenance()
}

/// Utility function to demonstrate config merging
#[allow(dead_code)] // Used in tests and maintenance operations
pub fn demo_config_merging(project_dir: &Path) -> anyhow::Result<()> {
    let mut manager = MaintenanceManager::new(project_dir)?;

    // Create override config with different settings
    let mut override_config = OrbitonConfig::default();
    override_config.dev_server.port = 9000;
    override_config.hmr.enabled = false;
    override_config.hmr.debounce_ms = 1000;

    println!(
        "{} Original port: {}",
        style("Before:").bold().blue(),
        manager.config().dev_server.port
    );

    manager.apply_config_overrides(override_config);

    println!(
        "{} New port: {}",
        style("After:").bold().green(),
        manager.config().dev_server.port
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_maintenance_manager_creation() {
        let temp_dir = tempdir().unwrap();
        let result = MaintenanceManager::new(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_hmr_cleanup() {
        let temp_dir = tempdir().unwrap();
        let project_dir = temp_dir.path();

        // Create a source file
        let src_dir = project_dir.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        let test_file = src_dir.join("test.rs");
        fs::write(&test_file, "// test").unwrap();

        let manager = MaintenanceManager::new(project_dir).unwrap();

        // Record a file change
        manager.hmr_context().record_file_change(&test_file);

        // Test cleanup
        manager.cleanup_stale_updates(Duration::from_secs(1));
        manager.clear_all_updates();
        manager.get_update_info();
    }

    #[test]
    fn test_config_merging() {
        let temp_dir = tempdir().unwrap();
        let project_dir = temp_dir.path();

        let mut manager = MaintenanceManager::new(project_dir).unwrap();
        let original_port = manager.config().dev_server.port;

        let mut override_config = OrbitonConfig::default();
        override_config.dev_server.port = 9999;

        manager.apply_config_overrides(override_config);

        assert_ne!(manager.config().dev_server.port, original_port);
        assert_eq!(manager.config().dev_server.port, 9999);
    }

    #[test]
    fn test_simple_dev_server_creation() {
        let temp_dir = tempdir().unwrap();
        let project_dir = temp_dir.path();

        let manager = MaintenanceManager::new(project_dir).unwrap();
        let result = manager.create_simple_dev_server(0, project_dir); // Use port 0 for testing
        assert!(result.is_ok());
    }
}
