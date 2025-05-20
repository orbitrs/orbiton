// Command for starting the development server

use anyhow::Result;
use clap::Args;
use console::style;
use log::{debug, error, info};
use notify::{Event, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::time::Duration;

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
}

pub fn execute(args: DevArgs) -> Result<()> {
    // Determine the project directory
    let project_dir = match args.dir {
        Some(dir) => dir,
        None => std::env::current_dir()?,
    };

    println!(
        "{} development server for project at {:?}",
        style("Starting").bold().green(),
        project_dir
    );

    // Create a development server
    let mut server = DevServer::new(args.port, &project_dir)?;

    // Start the server in a separate thread
    let _server_handle = server.start()?;

    println!(
        "Development server running at {}",
        style(format!("http://localhost:{}", args.port))
            .bold()
            .blue()
            .underlined()
    );

    // Open the browser if requested
    if args.open {
        if let Err(e) = open::that(format!("http://localhost:{}", args.port)) {
            error!("Failed to open browser: {}", e);
        }
    }

    // Set up file watching
    setup_file_watching(&project_dir)?;

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

fn setup_file_watching(project_dir: &Path) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Create a watcher
    let mut watcher =
        notify::recommended_watcher(move |res: std::result::Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    // Handle file change event
                    if let Err(e) = tx.send(event) {
                        error!("Failed to send file change event: {}", e);
                    }
                }
                Err(e) => error!("Watch error: {}", e),
            }
        })?;

    // Watch the project directory
    watcher.watch(project_dir, RecursiveMode::Recursive)?;

    // Spawn a thread to handle file change events
    std::thread::spawn(move || {
        for event in rx {
            debug!("File change event: {:?}", event);

            // Determine if we should rebuild
            let should_rebuild = event.paths.iter().any(|path| {
                let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                extension == "orbit" || extension == "rs"
            });

            if should_rebuild {
                println!(
                    "{} project due to file changes",
                    style("Rebuilding").bold().yellow()
                );
                // In a real implementation, we would trigger a rebuild here
            }
        }
    });

    info!("File watching set up for {:?}", project_dir);
    Ok(())
}
