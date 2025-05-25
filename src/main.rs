// Main entry point for the orbiton CLI tool

use clap::{Parser, Subcommand};
use console::style;
use log::info;

mod commands;
mod config;
mod dev_server;
mod hmr;
mod hmr_inject;
#[cfg(test)]
mod integration_tests;
mod maintenance;
mod templates;
mod test_hmr_module;
mod utils;

/// Version of the orbiton CLI
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "orbiton")]
#[command(author = "Orbit Framework Team")]
#[command(version = VERSION)]
#[command(about = "CLI tooling for the Orbit UI framework", long_about = None)]
struct Cli {
    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Orbit project
    New(commands::new::NewArgs),

    /// Start the development server
    Dev(commands::dev::DevArgs),

    /// Build the project
    Build(commands::build::BuildArgs),

    /// Configure the renderer
    Renderer(commands::renderer::RendererArgs),
    /// Run tests for the project
    Test(commands::test::TestCommand),

    /// Manage configuration
    Config(commands::config::ConfigArgs),

    /// Perform maintenance operations
    Maintenance(commands::maintenance::MaintenanceArgs),
}

fn main() -> anyhow::Result<()> {
    // Parse the command line arguments
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

    // Print welcome message
    println!("{} v{}", style("orbiton").bold().green(), VERSION);

    // Execute the appropriate command
    match cli.command {
        Commands::New(args) => {
            commands::new::execute(args)?;
        }
        Commands::Dev(args) => {
            commands::dev::execute(args)?;
        }
        Commands::Build(args) => {
            commands::build::execute(args)?;
        }
        Commands::Renderer(args) => {
            commands::renderer::execute(args)?;
        }
        Commands::Test(args) => {
            args.execute()?;
        }
        Commands::Config(args) => {
            commands::config::execute(args)?;
        }
        Commands::Maintenance(args) => {
            commands::maintenance::execute(args)?;
        }
    }
    info!("Command completed successfully");
    Ok(())
}

/// Print version information and available commands
#[allow(dead_code)] // Utility function for help/documentation
pub fn show_help_info() {
    println!("Available commands:");
    println!("  new         - Create a new Orbit project");
    println!("  dev         - Start development server");
    println!("  build       - Build project");
    println!("  test        - Run tests");
    println!("  config      - Manage configuration");
    println!("  maintenance - Perform maintenance operations");
}
