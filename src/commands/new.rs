// Command for creating a new Orbit project

use anyhow::{Context, Result};
use clap::Args;
use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use log::{debug, info};
use std::fs;
use std::path::{Path, PathBuf};

use crate::templates;

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

    // Determine the template to use
    let template = match args.template {
        Some(template) => template,
        None => {
            // Prompt the user to select a template
            let templates = vec!["basic", "component-library", "full-app"];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a project template")
                .default(0)
                .items(&templates)
                .interact()?;

            templates[selection].to_string()
        }
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
    generate_project(&args.name, &template, &output_dir)?;

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

fn generate_project(name: &str, template: &str, output_dir: &Path) -> Result<()> {
    // Get the template content
    let template_content = match templates::get_template(template) {
        Ok(content) => content,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to get template {}: {}",
                template,
                e
            ))
        }
    };

    // Prepare template variables
    let vars = serde_json::json!({
        "project_name": name,
        "orbit_version": orbitrs::VERSION,
        "orbiton_version": crate::VERSION,
    });

    // Process each template file
    for (file_path, content) in template_content {
        // Render the template with the variables
        let rendered_content = match render_template(&content, &vars) {
            Ok(rendered) => rendered,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to render template for file {}: {}",
                    file_path,
                    e
                ))
            }
        };

        // Determine the output file path
        let mut output_file = PathBuf::from(output_dir);
        output_file.push(&file_path);

        // Create parent directories if needed
        if let Some(parent) = output_file.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }

        // Write the file
        debug!("Writing file: {:?}", output_file);
        fs::write(&output_file, rendered_content)
            .with_context(|| format!("Failed to write file: {:?}", output_file))?;
    }

    info!("Project generated from template: {}", template);
    Ok(())
}

fn render_template(content: &str, vars: &serde_json::Value) -> Result<String> {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()?
        .parse(content)?;

    let globals = liquid::object!({
        "project": vars,
    });

    let rendered = template.render(&globals)?;
    Ok(rendered)
}
