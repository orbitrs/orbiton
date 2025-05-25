# Orbit Framework HMR Usage Guide

This guide shows you how to use the Hot Module Reload (HMR) system in the Orbit Framework development server.

## Quick Start

1. **Start the development server:**
   ```bash
   orbiton dev
   ```
   
   The server will start on port 8000 by default and automatically inject HMR client code into your HTML files.

2. **Open your application in a browser:**
   ```
   http://localhost:8000
   ```

3. **Make changes to your Rust files** and watch them rebuild automatically!

## Command Line Options

```bash
# Basic usage
orbiton dev

# Custom port
orbiton dev --port 9000

# Use beta Rust toolchain
orbiton dev --beta

# Specify project directory
orbiton dev --dir /path/to/project

# Open browser automatically
orbiton dev --open
```

## What Gets Watched

The HMR system automatically watches for changes in:
- `.rs` files (Rust source code)
- `.orbit` files (Orbit component files)
- Files in the `src/` directory

## HMR Client Integration

### Automatic Integration

The HMR client code is automatically injected into all HTML files served by the development server. No manual setup required!

### Custom HMR Handlers

For advanced use cases, you can register a custom HMR handler:

```javascript
// Register a custom handler that will be called when modules are updated
window.__ORBIT_REGISTER_HMR_HANDLER(function(modules) {
    console.log("Updated modules:", modules);
    
    // Your custom update logic here
    // Return a Promise if you need async operations
    return Promise.resolve();
});
```

### HMR Events

The client receives several types of events:

1. **File Changes**: Notified when files are modified
2. **Rebuild Status**: Updates on build progress (started, completed, failed)
3. **HMR Updates**: List of modules that were updated

## Visual Indicators

The HMR system provides visual feedback:

- **Build Status**: Shows rebuild progress in bottom-right corner
- **Success/Error States**: Green for success, red for errors
- **HMR Activity**: Displays when HMR updates are being applied

## WebSocket Communication

The HMR system uses WebSockets for real-time communication:
- **HTTP Server**: Runs on the specified port (default: 8000)
- **WebSocket Server**: Runs on port + 1 (default: 8001)

## Beta Toolchain Support

Use the latest Rust beta features:

```bash
orbiton dev --beta
```

This will:
1. Check if the beta toolchain is installed
2. Install it automatically if missing
3. Use `cargo +beta` for all builds

## Troubleshooting

### Connection Issues

If the HMR client can't connect:
1. Check that the WebSocket port (HTTP port + 1) isn't blocked
2. Ensure your firewall allows the connection
3. Look for error messages in the browser console

### Build Failures

When builds fail:
1. Check the terminal output for compile errors
2. Fix the errors and save the file again
3. The system will automatically retry the build

### Performance

For large projects:
1. The debounce time prevents excessive rebuilds (500ms default)
2. Only changed modules are tracked for updates
3. Use `--beta` for potential performance improvements

## Advanced Configuration

### Custom HTML Files

The HMR client is automatically injected into HTML files. If you need to serve HTML from a different location, place your files in the project root directory.

### API Endpoints

Special endpoints provided by the dev server:
- `/__orbit_hmr_client.js`: The HMR client script
- All other files are served from the project directory

## Integration with IDEs

The HMR system works with any text editor or IDE. For the best experience:
1. Configure your editor to save files automatically
2. Use the built-in terminal to see build output
3. Keep the browser and editor side-by-side for instant feedback

## Example Workflow

1. Start the dev server: `orbiton dev --open`
2. Browser opens to your application
3. Edit a Rust file and save
4. Watch the rebuild happen automatically
5. See the HMR update applied (or page reload if no handler)
6. Continue developing with instant feedback!

This HMR system is designed to speed up your development workflow by providing near-instant feedback when you make changes to your Orbit Framework applications.
