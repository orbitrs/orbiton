// Command for starting the development server

use anyhow::Result;
use clap::Args;
use console::style;
use log::{debug, error, info};
use notify::{Event, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use crate::config::OrbitonConfig;
use crate::dev_server::DevServer;

#[derive(Args)]
pub struct DevArgs {
    /// Port to use for the development server
    #[arg(short, long, default_value = "8000")]
    port: u16,

    /// Project directory
    #[arg(short, long)]
    dir: Option<PathBuf>,

    /// Open in browser
    #[arg(short, long)]
    open: bool,

    /// Use beta toolchain for building and testing
    #[arg(long)]
    beta: bool,
}

pub fn execute(args: DevArgs) -> Result<()> {
    // Determine the project directory
    let project_dir = match args.dir {
        Some(dir) => dir,
        None => std::env::current_dir()?,
    };

    // Load configuration from .orbiton.toml or use defaults
    let mut config = OrbitonConfig::load_from_project(&project_dir)?;

    // Override config with command line arguments
    if args.port != 8000 {
        config.dev_server.port = args.port;
    }
    if args.beta {
        config.build.use_beta_toolchain = true;
    }

    // Validate the configuration
    config.validate()?;

    if config.build.use_beta_toolchain {
        println!(
            "{} development server with {} toolchain for project at {project_dir:?}",
            style("Starting").bold().green(),
            style("beta").bold().yellow()
        );
    } else {
        println!(
            "{} development server for project at {project_dir:?}",
            style("Starting").bold().green()
        );
    }

    // Create a development server using the configuration
    let mut server = DevServer::new_with_options(
        config.dev_server.port,
        &project_dir,
        config.build.use_beta_toolchain,
    )?;

    if config.build.use_beta_toolchain {
        // Verify beta toolchain is installed
        match std::process::Command::new("rustup")
            .args(["toolchain", "list"])
            .output()
        {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                if !output_str.contains("beta") {
                    println!(
                        "{} Beta toolchain not installed. Installing...",
                        style("Warning:").bold().yellow()
                    );

                    // Try to install beta toolchain
                    let install_result = std::process::Command::new("rustup")
                        .args(["toolchain", "install", "beta"])
                        .status();

                    if let Err(e) = install_result {
                        return Err(anyhow::anyhow!("Failed to install beta toolchain: {}", e));
                    }
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to check for beta toolchain: {}", e));
            }
        }

        println!(
            "{} Using Rust beta toolchain for builds",
            style("Info:").bold().blue()
        );
    }

    // Start the server in a separate thread
    let _server_handle = server.start()?;

    println!(
        "Development server running at {}",
        style(format!("http://localhost:{}", config.dev_server.port))
            .bold()
            .blue()
            .underlined()
    );

    // Open the browser if requested (use config or CLI args)
    let should_open = args.open || config.dev_server.auto_open;
    if should_open {
        if let Err(e) = open::that(format!("http://localhost:{}", config.dev_server.port)) {
            error!("Failed to open browser: {e}");
        }
    }

    // Set up file watching
    setup_file_watching(project_dir.as_path(), &server)?;

    // Wait for Ctrl+C
    println!("Press {} to stop the server", style("Ctrl+C").bold());
    ctrlc::set_handler(move || {
        println!("\n{} development server", style("Stopping").bold().red());
        std::process::exit(0);
    })?;

    // Keep the main thread running
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

/// Rebuild the project using cargo
///
/// Returns true if the build was successful, false otherwise
fn rebuild_project(project_dir: &Path, use_beta: bool) -> bool {
    // Determine which toolchain to use
    let mut command = if use_beta {
        let mut cmd = std::process::Command::new("cargo");
        cmd.arg("+beta");
        cmd
    } else {
        std::process::Command::new("cargo")
    };

    // Set up the build command with appropriate arguments
    command
        .arg("build")
        .arg("--color=always")
        .current_dir(project_dir);

    // Execute the build command
    debug!("Running build command: {command:?}");

    match command.status() {
        Ok(status) => {
            if status.success() {
                info!("Project rebuilt successfully");
                true
            } else {
                error!("Project rebuild failed with status: {status}");
                false
            }
        }
        Err(e) => {
            error!("Failed to execute build command: {e}");
            false
        }
    }
}

fn setup_file_watching(project_dir: &Path, server: &DevServer) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let server = server.clone();
    let project_dir = project_dir.to_path_buf();
    let watcher_dir = project_dir.clone();
    let log_dir = project_dir.clone();
    let hmr_context = Arc::clone(server.hmr_context());

    // Create a watcher
    let mut watcher =
        notify::recommended_watcher(move |res: std::result::Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    // Handle file change event
                    if let Err(e) = tx.send(event) {
                        error!("Failed to send file change event: {e}");
                    }
                }
                Err(e) => error!("Watch error: {e}"),
            }
        })?;

    // Watch the project directory
    watcher.watch(&watcher_dir, RecursiveMode::Recursive)?;

    // Keep track of the watcher to prevent it from being dropped
    std::thread::spawn(move || {
        let _watcher = watcher; // Keep watcher alive
        let pdir = project_dir.clone(); // Create a new binding for the project directory

        // Debounce mechanism to avoid multiple rebuilds in quick succession
        let mut last_rebuild = std::time::Instant::now();
        const DEBOUNCE_TIME: Duration = Duration::from_millis(500);

        for event in rx {
            debug!("File change event: {event:?}");

            // Check if enough time has passed since last rebuild for additional debouncing
            let now = std::time::Instant::now();
            if now.duration_since(last_rebuild) < DEBOUNCE_TIME {
                debug!("Skipping event due to debounce (last rebuild too recent)");
                continue;
            }

            let paths = event
                .paths
                .iter()
                .map(|p| {
                    p.strip_prefix(&pdir)
                        .unwrap_or(p)
                        .to_string_lossy()
                        .into_owned()
                })
                .collect::<Vec<_>>();

            // Send the file change event to all connected clients
            let message = serde_json::json!({
                "type": "fileChange",
                "paths": paths,
                "kind": format!("{:?}", event.kind)
            })
            .to_string();

            if let Err(e) = server.broadcast_update(message) {
                error!("Failed to broadcast file change: {e}");
            } // Track changed modules in HMR context for intelligent updates
            let mut changed_modules = Vec::new();
            for path in &event.paths {
                if let Some(module) = hmr_context.record_file_change(path) {
                    changed_modules.push(module.clone());

                    // Log which file triggered the update
                    println!(
                        "{} {}",
                        style("File changed:").bold().blue(),
                        style(&module).dim()
                    );
                }
            }
            // Determine if we should rebuild using HMR context debouncing
            let should_rebuild = hmr_context.should_rebuild(DEBOUNCE_TIME);

            if should_rebuild {
                last_rebuild = now;

                println!(
                    "{} project due to file changes",
                    style("Rebuilding").bold().yellow()
                );

                // Send rebuild start notification using dev server method
                if let Err(e) = server.send_rebuild_status("started") {
                    error!("Failed to send rebuild start status: {e}");
                }

                // Perform the actual rebuild
                let rebuild_status = rebuild_project(&pdir, server.is_using_beta());

                // Report the rebuild status
                let status = match rebuild_status {
                    true => "completed",
                    false => "failed",
                };

                println!(
                    "{} {}",
                    style("Rebuild").bold(),
                    if rebuild_status {
                        style("completed successfully").green()
                    } else {
                        style("failed").red()
                    }
                );

                // Send the rebuild status using dev server method
                if let Err(e) = server.send_rebuild_status(status) {
                    error!("Failed to send rebuild status: {e}");
                }

                // If rebuild succeeded, record the rebuild and send HMR updates
                if rebuild_status {
                    // Record successful rebuild
                    hmr_context.record_rebuild();

                    // Get affected modules from HMR context
                    let affected_modules = hmr_context.get_pending_updates();

                    if !affected_modules.is_empty() {
                        // Log the modules being updated
                        println!(
                            "{} HMR update for modules: {}",
                            style("Sending").bold().blue(),
                            style(affected_modules.join(", ")).italic()
                        );

                        // Send HMR update using dev server method
                        if let Err(e) = server.send_hmr_update(affected_modules) {
                            error!("Failed to send HMR update: {e}");
                        }
                    }
                } else {
                    // On rebuild failure, send reload command to refresh the page
                    if let Err(e) = server.send_reload_command() {
                        error!("Failed to send reload command: {e}");
                    }
                }
            }
        }
    });

    info!("File watching set up for {log_dir:?}");
    Ok(())
}
