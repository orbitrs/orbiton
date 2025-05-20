# GitHub Copilot Instructions for orbiton

## About This Project

This project, `orbiton`, is part of the Orbit UI framework ecosystem.
- **Primary Language:** Rust
- **Core Focus:** The official command-line interface (CLI) for the Orbit UI framework, used for project scaffolding, development, building, and other management tasks.

Refer to the main `README.md` in the project root for detailed information on its features, commands, and architecture.

## Key Technologies & Concepts

- **Orbit Framework:** Understand the overall Orbit ecosystem, as `orbiton` is the primary developer tool for it.
- **CLI Design:** Familiarize yourself with best practices for CLI tools (command structure, arguments, flags, user feedback).
- **Rust for CLI Tools:** Leverage crates like `clap` for argument parsing, `miette` for error reporting, etc.
- **Build Processes:** `orbiton` handles build pipelines for WASM, native, and embedded targets. Knowledge of these processes is beneficial.
- **Development Workflow:** Understand features like hot reloading and integration with other tools like `orbit-analyzer`.
- **Renderer Configuration:** `orbiton` allows users to configure the default renderer (Skia, WGPU, Auto).

## When Assisting:

- **Consult READMEs:** Always check the `README.md` in the `orbiton` project root before providing solutions or suggesting new CLI features.
- **Command Structure:** When suggesting new commands or modifying existing ones, ensure they follow a clear and consistent structure.
- **User Experience:** Prioritize clear and helpful output for the user. Error messages should be actionable.
- **Integration:** Be mindful of how `orbiton` interacts with other parts of the Orbit ecosystem, such as `orbit-analyzer` for linting or `orbitkit` for component management (if applicable in the future).
- **Platform Targets:** Remember that `orbiton` needs to support building for Web (WASM), Native, and Embedded platforms.

By following these guidelines, you can provide more accurate and helpful assistance for the `orbiton` project.
