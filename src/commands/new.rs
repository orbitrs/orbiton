// Command for creating a new Orbit project

use anyhow::{Context, Result};
use clap::Args;
use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use log::debug;
use std::fs;
use std::path::PathBuf;

use crate::templates::project_templates::{TemplateManager, TemplateType};

#[derive(Args)]
pub struct NewArgs {
    /// Name of the project
    #[arg(required = true)]
    name: String,

    /// Template to use (basic, component-library, full-app)
    #[arg(short, long)]
    template: Option<String>,

    /// Output directory
    #[arg(short, long)]
    output_dir: Option<PathBuf>,
}

pub fn execute(args: NewArgs) -> Result<()> {
    println!(
        "{} a new Orbit project: {}",
        style("Creating").bold().green(),
        style(&args.name).bold()
    );

    let template_manager =
        TemplateManager::new().context("Failed to initialize template manager")?;

    // Determine the template to use
    let template_type = if let Some(template) = args.template {
        TemplateType::from_str(&template)
            .with_context(|| format!("Invalid template type: {}", template))?
    } else {
        // Prompt the user to select a template
        let templates = template_manager.list_templates();
        let template_names: Vec<String> = templates.iter().map(|t| t.to_string()).collect();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a project template")
            .default(0)
            .items(&template_names)
            .interact()
            .context("Failed to get user selection")?;

        templates
            .get(selection)
            .ok_or_else(|| anyhow::anyhow!("Invalid template selection"))?
            .clone()
    };

    // Determine the output directory
    let output_dir = match args.output_dir {
        Some(dir) => dir,
        None => {
            // Use the current directory + project name
            let mut dir = std::env::current_dir()?;
            dir.push(&args.name);
            dir
        }
    };

    // Create the output directory if it doesn't exist
    if !output_dir.exists() {
        debug!("Creating output directory: {:?}", output_dir);
        fs::create_dir_all(&output_dir)
            .with_context(|| format!("Failed to create directory: {:?}", output_dir))?;
    }

    // Generate the project from the template
    template_manager
        .generate_project(&args.name, template_type, &output_dir)
        .with_context(|| format!("Failed to generate project in {:?}", output_dir))?;

    println!(
        "\n{} project created at {:?}",
        style("Successfully").bold().green(),
        output_dir
    );

    // Print next steps
    println!("\n{}", style("Next steps:").bold());
    println!("  cd {}", args.name);
    println!("  orbiton dev");

    Ok(())
}
