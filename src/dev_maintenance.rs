// Development server maintenance and monitoring utilities

use crate::hmr::HmrContext;
use std::path::PathBuf;
use std::time::Duration;
use console::style;
use log::info;

/// Maintenance utilities for the development server
pub struct DevMaintenance {
    hmr_context: HmrContext,
}

impl DevMaintenance {
    pub fn new(project_root: PathBuf) -> Self {
        Self {
            hmr_context: HmrContext::new(project_root),
        }
    }

    /// Perform maintenance cleanup on HMR context
    pub fn cleanup_stale_updates(&self, max_age: Duration) -> usize {
        let stale_modules = self.hmr_context.get_stale_updates(max_age);
        let count = stale_modules.len();
        
        if count > 0 {
            println!(
                "{} {} stale HMR modules older than {:?}",
                style("Cleaning up").bold().yellow(),
                count,
                max_age
            );
            
            for module in &stale_modules {
                info!("Cleaning stale module: {}", module);
            }
            
            self.hmr_context.clear_stale_updates(max_age);
        }
        
        count
    }

    /// Get development server status
    pub fn get_status(&self) -> DevStatus {
        let pending_updates = self.hmr_context.get_pending_updates();
        let oldest_update_age = self.hmr_context.get_oldest_update_age();
        let needs_update = self.hmr_context.needs_update();

        DevStatus {
            pending_modules: pending_updates,
            oldest_update_age,
            needs_update,
        }
    }

    /// Clear all pending updates (useful for reset)
    pub fn reset_hmr_state(&self) {
        println!("{} HMR state", style("Resetting").bold().blue());
        self.hmr_context.clear();
        info!("HMR state has been reset");
    }

    /// Get detailed HMR statistics
    pub fn get_hmr_stats(&self) -> HmrStats {
        let pending = self.hmr_context.get_pending_updates();
        let stale_5min = self.hmr_context.get_stale_updates(Duration::from_secs(300));
        let stale_1min = self.hmr_context.get_stale_updates(Duration::from_secs(60));
        
        HmrStats {
            total_pending: pending.len(),
            stale_1min: stale_1min.len(),
            stale_5min: stale_5min.len(),
            oldest_age: self.hmr_context.get_oldest_update_age(),
            needs_rebuild: self.hmr_context.needs_update(),
        }
    }
}

/// Development server status
#[derive(Debug)]
pub struct DevStatus {
    pub pending_modules: Vec<String>,
    pub oldest_update_age: Option<Duration>,
    pub needs_update: bool,
}

/// HMR statistics
#[derive(Debug)]
pub struct HmrStats {
    pub total_pending: usize,
    pub stale_1min: usize,
    pub stale_5min: usize,
    pub oldest_age: Option<Duration>,
    pub needs_rebuild: bool,
}

impl DevStatus {
    pub fn print_status(&self) {
        println!("\n{}", style("Development Server Status").bold().underlined());
        
        if self.needs_update {
            println!("  Status: {} (needs rebuild)", style("Pending changes").yellow());
        } else {
            println!("  Status: {}", style("Up to date").green());
        }
        
        println!("  Pending modules: {}", self.pending_modules.len());
        
        if !self.pending_modules.is_empty() {
            for module in &self.pending_modules {
                println!("    - {}", style(module).cyan());
            }
        }
        
        if let Some(age) = self.oldest_update_age {
            println!("  Oldest change: {:?} ago", age);
        }
    }
}

impl HmrStats {
    pub fn print_stats(&self) {
        println!("\n{}", style("HMR Statistics").bold().underlined());
        println!("  Total pending modules: {}", style(self.total_pending).cyan());
        println!("  Stale (1 min): {}", style(self.stale_1min).yellow());
        println!("  Stale (5 min): {}", style(self.stale_5min).red());
        
        if let Some(age) = self.oldest_age {
            println!("  Oldest update: {:?} ago", age);
        }
        
        println!("  Needs rebuild: {}", 
                if self.needs_rebuild { 
                    style("Yes").red() 
                } else { 
                    style("No").green() 
                });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_dev_maintenance() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path().to_path_buf();
        
        // Create project structure
        let src_dir = project_root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        
        let test_file = src_dir.join("test.rs");
        fs::write(&test_file, "// test content").unwrap();
        
        let maintenance = DevMaintenance::new(project_root);
        
        // Simulate some file changes
        maintenance.hmr_context.record_file_change(&test_file);
        
        // Test status
        let status = maintenance.get_status();
        assert!(status.needs_update);
        assert_eq!(status.pending_modules.len(), 1);
        
        // Test stats
        let stats = maintenance.get_hmr_stats();
        assert_eq!(stats.total_pending, 1);
        assert!(stats.needs_rebuild);
        
        // Test reset
        maintenance.reset_hmr_state();
        let status_after_reset = maintenance.get_status();
        assert!(!status_after_reset.needs_update);
        assert_eq!(status_after_reset.pending_modules.len(), 0);
    }

    #[test]
    fn test_stale_cleanup() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path().to_path_buf();
        
        let src_dir = project_root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        
        let test_file = src_dir.join("test.rs");
        fs::write(&test_file, "// test content").unwrap();
        
        let maintenance = DevMaintenance::new(project_root);
        
        // Record file change
        maintenance.hmr_context.record_file_change(&test_file);
        
        // Cleanup stale updates (should be 0 since update is fresh)
        let cleaned = maintenance.cleanup_stale_updates(Duration::from_secs(1));
        assert_eq!(cleaned, 0);
        
        // Cleanup very fresh updates
        let cleaned = maintenance.cleanup_stale_updates(Duration::from_millis(1));
        assert_eq!(cleaned, 1);
    }
}
