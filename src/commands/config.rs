// Configuration management command

use anyhow::Result;
use clap::{Args, Subcommand};
use console::style;
use std::path::PathBuf;

use crate::config::OrbitonConfig;

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    command: ConfigCommand,
}

#[derive(Subcommand)]
enum ConfigCommand {
    /// Show current configuration
    Show {
        /// Project directory
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
    /// Create a default configuration file
    Init {
        /// Project directory
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
    /// Validate configuration
    Validate {
        /// Project directory
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
}

pub fn execute(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommand::Show { dir } => show_config(dir),
        ConfigCommand::Init { dir } => init_config(dir),
        ConfigCommand::Validate { dir } => validate_config(dir),
    }
}

fn get_project_dir(dir: Option<PathBuf>) -> Result<PathBuf> {
    match dir {
        Some(d) => Ok(d),
        None => std::env::current_dir()
            .map_err(|e| anyhow::anyhow!("Failed to get current directory: {}", e)),
    }
}

fn show_config(dir: Option<PathBuf>) -> Result<()> {
    let project_dir = get_project_dir(dir)?;

    println!(
        "{} configuration for project at {project_dir:?}",
        style("Showing").bold().blue()
    );

    let config = OrbitonConfig::load_from_project(&project_dir)?;

    println!("\n{}", style("Project Configuration:").bold().underlined());
    println!(
        "  Source directory: {}",
        style(&config.project.src_dir).cyan()
    );
    println!(
        "  Output directory: {}",
        style(&config.project.dist_dir).cyan()
    );
    println!(
        "  Entry point: {}",
        style(&config.project.entry_point).cyan()
    );

    println!("\n{}", style("Development Server:").bold().underlined());
    println!("  Port: {}", style(config.dev_server.port).cyan());
    println!("  Host: {}", style(&config.dev_server.host).cyan());
    println!(
        "  Auto-open browser: {}",
        style(config.dev_server.auto_open).cyan()
    );

    println!("\n{}", style("Hot Module Reload:").bold().underlined());
    println!("  Enabled: {}", style(config.hmr.enabled).cyan());
    println!(
        "  Debounce time: {}ms",
        style(config.hmr.debounce_ms).cyan()
    );
    println!(
        "  Preserve state: {}",
        style(config.hmr.preserve_state).cyan()
    );
    println!("  Max retries: {}", style(config.hmr.max_retries).cyan());

    println!("\n{}", style("Build Configuration:").bold().underlined());
    println!(
        "  Use beta toolchain: {}",
        style(config.build.use_beta_toolchain).cyan()
    );
    println!("  Release mode: {}", style(config.build.release).cyan());
    if let Some(target) = &config.build.target {
        println!("  Target: {}", style(target).cyan());
    }

    println!("\n{}", style("Lint Configuration:").bold().underlined());
    println!("  Enabled: {}", style(config.lint.enabled).cyan());

    Ok(())
}

fn init_config(dir: Option<PathBuf>) -> Result<()> {
    let project_dir = get_project_dir(dir)?;

    println!(
        "{} default configuration in {project_dir:?}",
        style("Creating").bold().green()
    );

    let config_path = OrbitonConfig::create_default_config(&project_dir)?;

    println!(
        "{} Configuration file created at: {}",
        style("Success!").bold().green(),
        style(config_path.display()).cyan()
    );

    println!("\nYou can now customize the configuration by editing the .orbiton.toml file.");

    Ok(())
}

fn validate_config(dir: Option<PathBuf>) -> Result<()> {
    let project_dir = get_project_dir(dir)?;

    println!(
        "{} configuration for project at {project_dir:?}",
        style("Validating").bold().yellow()
    );

    let config = OrbitonConfig::load_from_project(&project_dir)?;

    match config.validate() {
        Ok(()) => {
            println!(
                "{} Configuration is valid!",
                style("Success!").bold().green()
            );
        }
        Err(e) => {
            println!(
                "{} Configuration validation failed:",
                style("Error:").bold().red()
            );
            println!("  {e}");
            return Err(e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_init() {
        let temp_dir = tempdir().unwrap();
        let result = init_config(Some(temp_dir.path().to_path_buf()));
        assert!(result.is_ok());

        let config_path = temp_dir.path().join(".orbiton.toml");
        assert!(config_path.exists());
    }

    #[test]
    fn test_config_validate() {
        let temp_dir = tempdir().unwrap();

        // First create a config
        let _ = init_config(Some(temp_dir.path().to_path_buf()));

        // Then validate it
        let result = validate_config(Some(temp_dir.path().to_path_buf()));
        assert!(result.is_ok());
    }
}
