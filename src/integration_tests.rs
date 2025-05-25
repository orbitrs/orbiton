// Integration tests for the HMR system and development server

#[cfg(test)]
mod tests {
    use crate::config::OrbitonConfig;
    use crate::dev_server::DevServer;
    use crate::hmr::HmrContext;
    use std::time::Duration;
    use tempfile::tempdir;

    #[test]
    fn test_hmr_context_integration() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path().to_path_buf();

        // Create a src directory and test files
        let src_dir = project_root.join("src");
        std::fs::create_dir_all(&src_dir).unwrap();

        let test_file = src_dir.join("test_module.rs");
        std::fs::write(&test_file, "// Test module content").unwrap();

        let hmr_context = HmrContext::new(project_root);

        // Test recording file changes
        let module = hmr_context.record_file_change(&test_file);
        assert!(module.is_some());
        assert_eq!(module.unwrap(), "test_module");

        // Test checking for updates
        assert!(hmr_context.needs_update());

        // Test getting pending updates
        let pending = hmr_context.get_pending_updates();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0], "test_module");

        // Test marking modules as updated
        hmr_context.mark_modules_updated();
        assert!(!hmr_context.needs_update());

        // Test clearing updates
        let _ = hmr_context.record_file_change(&test_file);
        assert!(hmr_context.needs_update());
        hmr_context.clear();
        assert!(!hmr_context.needs_update());
    }

    #[test]
    fn test_hmr_timestamp_functionality() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path().to_path_buf();

        let src_dir = project_root.join("src");
        std::fs::create_dir_all(&src_dir).unwrap();

        let test_file = src_dir.join("test_module.rs");
        std::fs::write(&test_file, "// Test module content").unwrap();

        let hmr_context = HmrContext::new(project_root);

        // Record a file change
        hmr_context.record_file_change(&test_file);

        // Test timestamp-related methods
        let oldest_age = hmr_context.get_oldest_update_age();
        assert!(oldest_age.is_some());
        assert!(oldest_age.unwrap() < Duration::from_millis(100));

        // Test getting stale updates (should be empty since update is fresh)
        let stale = hmr_context.get_stale_updates(Duration::from_secs(1));
        assert!(stale.is_empty());
        // Test clearing stale updates (should not remove fresh update)
        hmr_context.clear_stale_updates(Duration::from_secs(1));
        assert!(hmr_context.needs_update());

        // Wait a moment to make sure update ages
        std::thread::sleep(Duration::from_millis(10));

        // Test clearing very fresh updates (should clear after delay)
        hmr_context.clear_stale_updates(Duration::from_millis(1));
        assert!(!hmr_context.needs_update());
    }

    #[test]
    fn test_config_integration() {
        let temp_dir = tempdir().unwrap();
        let project_dir = temp_dir.path();

        // Test creating default config
        let config_path = OrbitonConfig::create_default_config(project_dir).unwrap();
        assert!(config_path.exists());

        // Test loading from project
        let loaded_config = OrbitonConfig::load_from_project(project_dir).unwrap();
        assert_eq!(loaded_config.dev_server.port, 3000);

        // Test finding config file
        let found_path = OrbitonConfig::find_config_file(project_dir);
        assert!(found_path.is_some());
        assert_eq!(found_path.unwrap(), config_path);

        // Test config merging
        let mut base_config = OrbitonConfig::default();
        let mut override_config = OrbitonConfig::default();
        override_config.dev_server.port = 8080;
        override_config.hmr.enabled = false;

        base_config.merge_with(&override_config);
        assert_eq!(base_config.dev_server.port, 8080);
        assert!(!base_config.hmr.enabled);

        // Test validation
        assert!(base_config.validate().is_ok());

        base_config.dev_server.port = 0;
        assert!(base_config.validate().is_err());
    }

    #[test]
    fn test_dev_server_creation() {
        let temp_dir = tempdir().unwrap();
        let project_dir = temp_dir.path();

        // Test creating new dev server
        let server = DevServer::new(3000, project_dir);
        assert!(server.is_ok());

        let server = server.unwrap();
        assert_eq!(server.port(), 3000);
        assert!(!server.is_using_beta());
    }

    #[test]
    fn test_hmr_system_end_to_end() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path().to_path_buf();

        // Create project structure
        let src_dir = project_root.join("src");
        std::fs::create_dir_all(&src_dir).unwrap();

        let main_file = src_dir.join("main.rs");
        let lib_file = src_dir.join("lib.rs");

        std::fs::write(&main_file, "fn main() { println!(\"Hello\"); }").unwrap();
        std::fs::write(&lib_file, "pub fn test() {}").unwrap();

        let hmr_context = HmrContext::new(project_root.clone());

        // Simulate file changes
        hmr_context.record_file_change(&main_file);
        hmr_context.record_file_change(&lib_file);

        assert!(hmr_context.needs_update());
        let pending = hmr_context.get_pending_updates();
        assert_eq!(pending.len(), 2);
        assert!(pending.contains(&"main".to_string()));
        assert!(pending.contains(&"lib".to_string()));

        // Simulate rebuild
        hmr_context.record_rebuild();
        assert!(!hmr_context.needs_update());

        // Test debounce functionality
        let should_rebuild = hmr_context.should_rebuild(Duration::from_millis(100));
        assert!(!should_rebuild); // Should not rebuild immediately

        std::thread::sleep(Duration::from_millis(150));

        // Add another change
        hmr_context.record_file_change(&main_file);
        let should_rebuild = hmr_context.should_rebuild(Duration::from_millis(100));
        assert!(should_rebuild); // Should rebuild after debounce time
    }
    #[test]
    fn test_test_component_functionality() {
        use crate::test_hmr_module::TestComponent;

        let mut component = TestComponent::new();
        assert_eq!(
            component.render(),
            "TestComponent { value: 42, message: 'Hello from Orbit HMR!' }"
        );

        component.update(100);
        assert_eq!(
            component.render(),
            "TestComponent { value: 100, message: 'Hello from Orbit HMR!' }"
        );

        component.update(-10);
        assert_eq!(
            component.render(),
            "TestComponent { value: -10, message: 'Hello from Orbit HMR!' }"
        );
    }

    #[tokio::test]
    async fn test_dev_server_hmr_methods() {
        let temp_dir = tempdir().unwrap();
        let project_dir = temp_dir.path();

        let server = DevServer::new(0, project_dir).unwrap(); // Use port 0 for testing

        // Test HMR methods (these should not fail even without active connections)
        let result = server.send_hmr_update(vec!["test_module".to_string()]);
        // This might fail due to no connections, but should not panic
        let _ = result;

        let result = server.send_reload_command();
        let _ = result;

        let result = server.send_rebuild_status("completed");
        let _ = result;
    }

    #[test]
    fn test_maintenance_manager_functionality() {
        use crate::maintenance::{
            demo_config_merging, perform_project_maintenance, MaintenanceManager,
        };

        let temp_dir = tempdir().unwrap();
        let project_dir = temp_dir.path();

        // Create a config file for testing
        let _config_path = OrbitonConfig::create_default_config(project_dir).unwrap();

        let mut manager = MaintenanceManager::new(project_dir).unwrap();

        // Test config access
        let config = manager.config();
        assert_eq!(config.dev_server.port, 3000);

        // Test HMR context access
        let hmr_context = manager.hmr_context();
        assert!(!hmr_context.needs_update());

        // Test update info
        let update_info = manager.get_update_info();
        assert!(update_info.is_none()); // No updates yet

        // Test config overrides
        let mut override_config = OrbitonConfig::default();
        override_config.dev_server.port = 8080;
        override_config.hmr.enabled = false;

        manager.apply_config_overrides(override_config);
        assert_eq!(manager.config().dev_server.port, 8080);
        assert!(!manager.config().hmr.enabled);

        // Test simple dev server creation
        let server = manager.create_simple_dev_server(9000, project_dir);
        assert!(server.is_ok());
        let server = server.unwrap();
        assert_eq!(server.port(), 9000);

        // Test automated maintenance
        let result = manager.perform_automated_maintenance();
        assert!(result.is_ok());

        // Test standalone functions
        let result = perform_project_maintenance(project_dir);
        assert!(result.is_ok());

        let result = demo_config_merging(project_dir);
        assert!(result.is_ok());
    }
}
