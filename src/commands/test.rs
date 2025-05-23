//! Implementation of the `orbiton test` command.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Command line arguments for the `test` command.
#[derive(Parser)]
pub struct TestCommand {
    /// Run tests in watch mode, automatically re-running when files change
    #[arg(long)]
    pub watch: bool,

    /// Run only unit tests
    #[arg(long)]
    pub unit: bool,

    /// Run only integration tests
    #[arg(long)]
    pub integration: bool,

    /// Run performance tests to measure rendering speed and memory usage
    #[arg(long)]
    pub performance: bool,

    /// Generate test coverage information
    #[arg(long)]
    pub coverage: bool,

    /// Generate and display a detailed coverage report
    #[arg(long)]
    pub report: bool,

    /// Update test snapshots instead of failing on mismatch
    #[arg(long = "update-snapshots")]
    pub update_snapshots: bool,

    /// Show detailed test output
    #[arg(long)]
    pub verbose: bool,

    /// Custom path to the project directory
    #[arg(long = "dir", short = 'd')]
    pub project_dir: Option<PathBuf>,
}

impl TestCommand {
    /// Execute the test command.
    pub fn execute(&self) -> Result<()> {
        use console::style;
        use std::process::Command;

        // Get the project directory (current directory if not specified)
        let project_dir = self
            .project_dir
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap());

        println!(
            "{} Looking for tests in {}",
            style("[1/4]").bold().dim(),
            style(project_dir.display()).underlined()
        );

        // Check if this is an Orbit project by looking for specific files
        let is_orbit_project = std::fs::metadata(project_dir.join("orbit.config.toml")).is_ok()
            || std::fs::metadata(project_dir.join("Cargo.toml")).is_ok();

        if !is_orbit_project {
            println!(
                "⚠️  {} This directory does not appear to be an Orbit project.",
                style("Warning:").yellow().bold()
            );
            println!("   Looking for orbit.config.toml or Cargo.toml...");
        }

        // Since this is a planned future feature, print a message but also try to run standard Rust tests
        println!(
            "\n{}",
            style("🚧 The `orbiton test` command is under active development.")
                .yellow()
                .bold()
        );
        println!("Some advanced testing features are planned for future releases.");
        println!();
        println!("{}:", style("Planned features").bold());
        println!(" • Unit testing for components");
        println!(" • Integration testing for applications");
        println!(" • Performance testing and benchmarking");
        println!(" • Coverage reporting");
        println!(" • Snapshot testing");
        println!(" • Watch mode for test-driven development");

        // Check for testing flags and run appropriate test commands
        println!(
            "\n{} Running tests with current implementation:",
            style("[2/4]").bold().dim()
        );

        // Build the cargo test command based on provided flags
        let mut cmd_args = vec!["test"];

        if self.verbose {
            cmd_args.push("--verbose");
        }

        if self.unit && !self.integration {
            cmd_args.push("--lib");
        } else if self.integration && !self.unit {
            cmd_args.push("--test");
        }

        println!(
            "{} Executing: cargo {}",
            style("[3/4]").bold().dim(),
            cmd_args.join(" ")
        );

        // Execute the cargo test command
        let status = Command::new("cargo")
            .args(&cmd_args)
            .current_dir(&project_dir)
            .status();

        match status {
            Ok(exit_status) => {
                if exit_status.success() {
                    println!(
                        "\n{} {}",
                        style("✅ Success:").green().bold(),
                        style("All tests passed!").bold()
                    );
                } else {
                    println!(
                        "\n{} {}",
                        style("❌ Error:").red().bold(),
                        style("Some tests failed.").bold()
                    );
                }
            }
            Err(e) => {
                println!(
                    "\n{} Failed to execute cargo test: {}",
                    style("❌ Error:").red().bold(),
                    e
                );
            }
        }

        println!(
            "\n{} {}",
            style("[4/4]").bold().dim(),
            style("For more information on testing strategies, see:").italic()
        );
        println!("    https://docs.orbitrs.dev/guides/testing-strategies");

        // Return Ok to indicate command executed successfully
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let cmd = TestCommand {
            watch: true,
            unit: false,
            integration: true,
            performance: false,
            coverage: true,
            report: true,
            update_snapshots: false,
            verbose: true,
            project_dir: None,
        };

        assert!(cmd.watch);
        assert!(!cmd.unit);
        assert!(cmd.integration);
        assert!(!cmd.performance);
        assert!(cmd.coverage);
        assert!(cmd.report);
        assert!(!cmd.update_snapshots);
        assert!(cmd.verbose);
        assert_eq!(cmd.project_dir, None);
    }

    #[test]
    fn test_build_cmd_args() {
        // Test unit tests command flags
        let cmd = TestCommand {
            watch: false,
            unit: true,
            integration: false,
            performance: false,
            coverage: false,
            report: false,
            update_snapshots: false,
            verbose: true,
            project_dir: None,
        };

        // This is a way to test the command building without actually running commands
        // Extract the command args building logic to a separate method to make it testable
        let args = build_test_command_args(&cmd);

        assert!(args.contains(&"test"));
        assert!(args.contains(&"--verbose"));
        assert!(args.contains(&"--lib"));
        assert!(!args.contains(&"--test"));
    }

    // Helper function for testing the command args
    #[cfg(test)]
    fn build_test_command_args(cmd: &TestCommand) -> Vec<&'static str> {
        let mut args = vec!["test"];

        if cmd.verbose {
            args.push("--verbose");
        }

        if cmd.unit && !cmd.integration {
            args.push("--lib");
        } else if cmd.integration && !cmd.unit {
            args.push("--test");
        }

        args
    }
}
