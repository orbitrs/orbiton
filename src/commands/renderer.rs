// Command for configuring the renderer

use anyhow::{Context, Result};
use clap::Args;
use console::style;
use std::path::PathBuf;

#[derive(Args)]
pub struct RendererArgs {
    /// Renderer configuration (skia, wgpu, auto)
    #[arg(short, long)]
    config: String,

    /// Project directory
    #[arg(short, long)]
    dir: Option<PathBuf>,
}

pub fn execute(args: RendererArgs) -> Result<()> {
    // Determine the project directory
    let project_dir = match args.dir {
        Some(dir) => dir,
        None => std::env::current_dir()?,
    };

    println!(
        "{} renderer to {}",
        style("Configuring").bold().green(),
        style(&args.config).bold()
    );

    // Validate the renderer configuration
    let renderer_type = match args.config.to_lowercase().as_str() {
        "skia" => "skia",
        "wgpu" => "wgpu",
        "auto" => "auto",
        _ => {
            return Err(anyhow::anyhow!(
                "Invalid renderer configuration: {}. Valid options are: skia, wgpu, auto",
                args.config
            ));
        }
    };

    // Update the project configuration file
    let config_file = project_dir.join("orbit.config.json");

    // If the config file exists, read it; otherwise, create a new one
    let mut config = if config_file.exists() {
        let config_str = std::fs::read_to_string(&config_file)
            .with_context(|| format!("Failed to read config file: {config_file:?}"))?;

        serde_json::from_str(&config_str)
            .with_context(|| format!("Failed to parse config file: {config_file:?}"))?
    } else {
        serde_json::json!({})
    };

    // Update the renderer configuration
    if let Some(config_obj) = config.as_object_mut() {
        config_obj.insert(
            "renderer".to_string(),
            serde_json::Value::String(renderer_type.to_string()),
        );
    }

    // Write the updated configuration
    let config_str =
        serde_json::to_string_pretty(&config).with_context(|| "Failed to serialize config")?;

    std::fs::write(&config_file, config_str)
        .with_context(|| format!("Failed to write config file: {config_file:?}"))?;

    println!(
        "Renderer configured to {} in {config_file:?}",
        style(renderer_type).bold()
    );

    Ok(())
}
