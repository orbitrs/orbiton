use anyhow::{Context, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTemplate {
    pub name: String,
    pub description: String,
    pub files: Vec<TemplateFile>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub format: Option<ComponentFormat>, // Default to Legacy if None
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub enum TemplateType {
    Basic,
    ComponentLibrary,
    Advanced,
}

impl fmt::Display for TemplateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::Advanced => write!(f, "advanced"),
            Self::ComponentLibrary => write!(f, "component-library"),
        }
    }
}

impl TemplateType {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "basic" => Ok(Self::Basic),
            "advanced" => Ok(Self::Advanced),
            "component-library" => Ok(Self::ComponentLibrary),
            _ => Err(anyhow::anyhow!("Invalid template type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComponentFormat {
    #[serde(rename = "legacy")]
    Legacy, // Old <script> format
    #[serde(rename = "modern")]
    Modern, // New <code> format with section tags
    #[serde(rename = "markdown")]
    Markdown, // Full Markdown format with code blocks
}

impl Default for ComponentFormat {
    fn default() -> Self {
        Self::Legacy
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSection {
    pub name: String, // e.g., "template", "style", "code", "tests", "markdown"
    pub lang: String, // e.g., "html", "css", "rust", "markdown"
    pub content: String,
}

pub struct TemplateManager {
    templates_dir: std::path::PathBuf,
}

impl TemplateManager {
    pub fn new() -> Result<Self> {
        // List of possible template directories
        let mut possible_dirs = Vec::new();

        // Try relative to executable
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                possible_dirs.push(dir.join("templates"));
            }
        }

        // Try relative to crate root (for development)
        if let Ok(dir) = std::env::current_dir() {
            possible_dirs.push(dir.join("templates"));
        }

        // Try relative to workspace root
        if let Ok(dir) = std::env::current_dir() {
            possible_dirs.extend(dir.ancestors().take(3).map(|p| p.join("templates")));
        }

        // Try relative to cargo manifest directory (for development)
        if let Ok(dir) = std::env::var("CARGO_MANIFEST_DIR") {
            possible_dirs.push(PathBuf::from(dir).join("templates"));
        }

        for templates_dir in possible_dirs.iter() {
            debug!("Checking for templates in {:?}", templates_dir);
            if templates_dir.exists() {
                return Ok(Self {
                    templates_dir: templates_dir.clone(),
                });
            }
        }

        Err(anyhow::anyhow!(
            "Templates directory not found. Make sure the template files are properly installed. Looked in:\n{}",
            possible_dirs
                .iter()
                .map(|p| format!("- {:?}", p))
                .collect::<Vec<_>>()
                .join("\n")
        ))
    }

    pub fn list_templates(&self) -> Vec<TemplateType> {
        vec![
            TemplateType::Basic,
            TemplateType::ComponentLibrary,
            TemplateType::Advanced,
        ]
    }

    pub fn generate_project(
        &self,
        name: &str,
        template_type: TemplateType,
        output_dir: &Path,
    ) -> Result<()> {
        let template_dir = self.templates_dir.join(template_type.to_string());
        if !template_dir.exists() {
            return Err(anyhow::anyhow!(
                "Template directory not found: {:?}",
                template_dir
            ));
        }

        let template_json = std::fs::read_to_string(template_dir.join("template.json"))
            .with_context(|| format!("Failed to read template.json from {:?}", template_dir))?;

        let mut template: ProjectTemplate =
            serde_json::from_str(&template_json).context("Failed to parse template.json")?;

        template.name = name.to_string();

        for mut file in template.files {
            // Convert file extension for Markdown components if needed
            if template.format == Some(ComponentFormat::Markdown) && file.path.ends_with(".orbit") {
                file.path = file.path.replace(".orbit", ".orbit.md");
            }

            let target_path = output_dir.join(&file.path);
            if let Some(parent) = target_path.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory {:?}", parent))?;
            }

            std::fs::write(&target_path, file.content)
                .with_context(|| format!("Failed to write file: {:?}", target_path))?;
        }

        Ok(())
    }

    #[allow(dead_code)] // Used in other modules
    pub fn parse_component_sections(
        content: &str,
        format: ComponentFormat,
    ) -> Result<Vec<ComponentSection>> {
        match format {
            ComponentFormat::Legacy => Self::parse_legacy_format(content),
            ComponentFormat::Modern => Self::parse_modern_format(content),
            ComponentFormat::Markdown => Self::parse_markdown_format(content),
        }
    }

    #[allow(dead_code)]
    fn parse_markdown_format(content: &str) -> Result<Vec<ComponentSection>> {
        let mut sections = Vec::new();
        let mut current_section: Option<ComponentSection> = None;
        let mut in_code_block = false;
        let mut code_fence_count = 0;
        let mut markdown_content = String::new();

        for line in content.lines() {
            if line.starts_with("```") {
                code_fence_count += 1;
                if code_fence_count % 2 == 1 {
                    // Start of code block
                    in_code_block = true;
                    let lang = line.trim_start_matches('`').trim();
                    if !lang.is_empty() {
                        // Save any accumulated markdown content
                        if !markdown_content.trim().is_empty() {
                            sections.push(ComponentSection {
                                name: "markdown".to_string(),
                                lang: "markdown".to_string(),
                                content: markdown_content.trim().to_string(),
                            });
                            markdown_content.clear();
                        }

                        if let Some(section) = current_section {
                            sections.push(section);
                        }
                        current_section = Some(ComponentSection {
                            name: Self::determine_section_name(lang),
                            lang: lang.to_string(),
                            content: String::new(),
                        });
                    }
                } else {
                    // End of code block
                    in_code_block = false;
                    if let Some(section) = current_section.take() {
                        sections.push(section);
                    }
                }
            } else if in_code_block {
                if let Some(ref mut section) = current_section {
                    section.content.push_str(line);
                    section.content.push('\n');
                }
            } else {
                // This is Markdown content outside code blocks
                markdown_content.push_str(line);
                markdown_content.push('\n');
            }
        }

        Ok(sections)
    }

    #[allow(dead_code)]
    fn parse_modern_format(content: &str) -> Result<Vec<ComponentSection>> {
        let mut sections = Vec::new();
        let mut current_section: Option<ComponentSection> = None;

        for line in content.lines() {
            if line.starts_with('<') && line.ends_with('>') {
                // This line indicates a new section
                if let Some(section) = current_section.take() {
                    sections.push(section);
                }

                let lang = line.trim_matches('<').trim_matches('>').trim();
                current_section = Some(ComponentSection {
                    name: Self::determine_section_name(lang),
                    lang: lang.to_string(),
                    content: String::new(),
                });
            } else if let Some(ref mut section) = current_section {
                section.content.push_str(line);
                section.content.push('\n');
            }
        }

        if let Some(section) = current_section {
            sections.push(section);
        }

        Ok(sections)
    }

    #[allow(dead_code)]
    fn parse_legacy_format(content: &str) -> Result<Vec<ComponentSection>> {
        let mut sections = Vec::new();
        let mut current_section: Option<ComponentSection> = None;

        // Legacy format has <template>, <style>, and <script> tags
        for line in content.lines() {
            if line.contains("<template>") {
                if let Some(section) = current_section.take() {
                    sections.push(section);
                }
                current_section = Some(ComponentSection {
                    name: "template".to_string(),
                    lang: "html".to_string(),
                    content: String::new(),
                });
            } else if line.contains("</template>") {
                if let Some(section) = current_section.take() {
                    sections.push(section);
                }
            } else if line.contains("<style>") {
                if let Some(section) = current_section.take() {
                    sections.push(section);
                }
                current_section = Some(ComponentSection {
                    name: "style".to_string(),
                    lang: "css".to_string(),
                    content: String::new(),
                });
            } else if line.contains("</style>") {
                if let Some(section) = current_section.take() {
                    sections.push(section);
                }
            } else if line.contains("<script>") {
                if let Some(section) = current_section.take() {
                    sections.push(section);
                }
                current_section = Some(ComponentSection {
                    name: "code".to_string(),
                    lang: "rust".to_string(),
                    content: String::new(),
                });
            } else if line.contains("</script>") {
                if let Some(section) = current_section.take() {
                    sections.push(section);
                }
            } else if let Some(ref mut section) = current_section {
                section.content.push_str(line);
                section.content.push('\n');
            }
        }

        if let Some(section) = current_section {
            sections.push(section);
        }

        Ok(sections)
    }

    #[allow(dead_code)]
    fn determine_section_name(lang: &str) -> String {
        match lang {
            "html" | "template" => "template",
            "css" | "style" => "style",
            "rust" => "code",
            "markdown" | "md" => "markdown",
            _ => lang,
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir; // Now properly imported from added dependency

    #[test]
    fn test_template_type_from_str() {
        assert!(matches!(
            TemplateType::from_str("basic"),
            Ok(TemplateType::Basic)
        ));
        assert!(matches!(
            TemplateType::from_str("advanced"),
            Ok(TemplateType::Advanced)
        ));
        assert!(matches!(
            TemplateType::from_str("component-library"),
            Ok(TemplateType::ComponentLibrary)
        ));
        assert!(TemplateType::from_str("invalid").is_err());
    }

    #[test]
    #[ignore] // Ignore this test as it requires template files to be installed
    fn test_create_from_template() -> Result<()> {
        let temp_dir = tempdir()?;
        let template_manager = TemplateManager::new()?;

        // Create a basic project
        template_manager.generate_project("test-project", TemplateType::Basic, temp_dir.path())?;

        // Verify created files
        assert!(temp_dir.path().join("Cargo.toml").exists());
        assert!(temp_dir.path().join("src/main.rs").exists());

        Ok(())
    }
}
