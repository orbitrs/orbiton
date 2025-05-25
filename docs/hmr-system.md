# Hot Module Reload (HMR) System

This document explains the implementation and usage of the Hot Module Reload (HMR) system in the Orbit Framework's development server.

## Overview

The HMR system allows for rapid development feedback by:

1. Detecting file changes in the project
2. Intelligently rebuilding only when necessary
3. Communicating changes to the client browser
4. Allowing selective updates without a full page refresh

## Architecture

The HMR system consists of several components:

### Server Components

1. **HmrContext** (`src/hmr.rs`): Manages the state of changed modules and tracks which ones need updates
2. **DevServer** (`src/dev_server.rs`): Hosts the HTTP and WebSocket servers
3. **File Watcher** (`src/commands/dev.rs`): Monitors project files for changes
4. **HMR Injector** (`src/hmr_inject.rs`): Injects the HMR client code into HTML responses

### Client Components

1. **HMR Client** (`src/hmr_client.js`): JavaScript code that runs in the browser to handle HMR updates

## Implementation Details

### File Change Detection

The file watcher monitors project files and notifies the HMR context when changes occur. The HMR context tracks changed modules and determines when a rebuild is required.

```rust
// File change detection (simplified)
if let Some(module) = hmr_context.record_file_change(path) {
    changed_modules.push(module);
}
```

### Debouncing

To prevent excessive rebuilds, the HMR context implements debouncing:

```rust
// Check if enough time has passed since last rebuild
pub fn should_rebuild(&self, debounce_time: Duration) -> bool {
    // Check if enough time has passed since last rebuild
    let last_rebuild = self.last_rebuild.lock().unwrap();
    if let Some(instant) = *last_rebuild {
        if instant.elapsed() < debounce_time {
            return false;
        }
    }
    
    // Check if we have pending updates
    self.needs_update()
}
```

### Module Tracking

The HMR context tracks module changes to enable targeted updates:

```rust
pub fn get_pending_updates(&self) -> Vec<String> {
    let modules = self.modules.lock().unwrap();
    modules
        .values()
        .filter(|update| !update.is_updated)
        .map(|update| update.module.clone())
        .collect()
}
```

### Client Communication

The server communicates with clients via WebSockets, sending messages about:
- File changes
- Rebuild status (starting, completed, failed)
- HMR updates with affected modules

## How to Use

### Starting the Development Server

```bash
# Basic usage
orbiton dev

# With beta toolchain
orbiton dev --beta

# With custom port
orbiton dev --port 9000
```

### Client-Side Integration

To enable surgical updates in your application, register an HMR handler:

```javascript
window.__ORBIT_REGISTER_HMR_HANDLER(function(modules) {
  console.log("Modules updated:", modules);
  
  // Your update logic here
  // For example, reload specific components or update state
});
```

Without a handler, the system will fall back to a full page reload.

## Beta Toolchain Support

The system supports using the Rust beta toolchain for builds:

```rust
fn rebuild_project(project_dir: &Path, use_beta: bool) -> bool {
    // Determine which toolchain to use
    let mut command = if use_beta {
        let mut cmd = std::process::Command::new("cargo");
        cmd.arg("+beta");
        cmd
    } else {
        std::process::Command::new("cargo")
    };
    
    // Rest of build command setup...
}
```

## Further Development

Future improvements could include:

1. Per-component HMR updates for more granular control
2. Dependency graph analysis to update dependent modules
3. State preservation during updates
4. Integration with the component system for automatic reconciliation
