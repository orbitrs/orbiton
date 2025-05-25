// Maintenance command for cleanup operations

use clap::{Parser, Subcommand};
use log::info;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

use crate::maintenance::MaintenanceManager;

#[derive(Parser)]
pub struct MaintenanceArgs {
    /// Project directory (defaults to current directory)
    #[arg(short = 'd', long)]
    project_dir: Option<PathBuf>,

    #[command(subcommand)]
    action: MaintenanceAction,
}

#[derive(Subcommand)]
enum MaintenanceAction {
    /// Clean up stale HMR updates
    Cleanup {
        /// Maximum age of updates to keep (in seconds)
        #[arg(short, long, default_value = "300")]
        max_age: u64,
    },
    /// Clear all pending HMR updates
    Clear,
    /// Show maintenance status
    Status,
}

pub fn execute(args: MaintenanceArgs) -> anyhow::Result<()> {
    let project_dir = args
        .project_dir
        .unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));

    info!(
        "Running maintenance operations in: {}",
        project_dir.display()
    );

    let manager = MaintenanceManager::new(&project_dir)?;

    match args.action {
        MaintenanceAction::Cleanup { max_age } => {
            let duration = Duration::from_secs(max_age);
            manager.cleanup_stale_updates(duration);
        }
        MaintenanceAction::Clear => {
            manager.clear_all_updates();
        }
        MaintenanceAction::Status => {
            manager.show_status();
        }
    }

    Ok(())
}
