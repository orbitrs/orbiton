// Templates for creating new Orbit projects

use std::collections::HashMap;

/// Get a template by name
pub fn get_template(name: &str) -> Result<HashMap<String, String>, String> {
    match name {
        "basic" => Ok(basic_template()),
        "component-library" => Ok(component_library_template()),
        "full-app" => Ok(full_app_template()),
        _ => Err(format!("Unknown template: {}", name)),
    }
}

/// Basic template for a simple Orbit project
fn basic_template() -> HashMap<String, String> {
    let mut template = HashMap::new();

    // Cargo.toml
    template.insert(
        "Cargo.toml".to_string(),
        r#"[package]
name = "{{ project.project_name }}"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]

[dependencies]
orbit = "{{ project.orbit_version }}"

[build-dependencies]
orbiton = "{{ project.orbiton_version }}"
"#
        .to_string(),
    );

    // Main lib.rs
    template.insert(
        "src/lib.rs".to_string(),
        r#"// Main library for {{ project.project_name }}

pub mod components;

/// Initialize the application
pub fn init() -> Result<(), orbitrs::Error> {
    // Initialize Orbit
    orbitrs::init()?;
    
    // Additional initialization here
    
    Ok(())
}
"#
        .to_string(),
    );

    // Entry point
    template.insert(
        "src/main.rs".to_string(),
        r#"// Entry point for {{ project.project_name }}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the application
    {{ project.project_name }}::init()?;
    
    // Start the application
    // ...
    
    Ok(())
}
"#
        .to_string(),
    );

    // Components module
    template.insert(
        "src/components/mod.rs".to_string(),
        r#"// Components for {{ project.project_name }}

pub mod counter;
"#
        .to_string(),
    );

    // Sample component
    template.insert(
        "src/components/counter.orbitrs".to_string(),
        r#"<template>
  <div class="counter">
    <h2>{{ count }}</h2>
    <button @click="increment">Increment</button>
    <button @click="decrement">Decrement</button>
  </div>
</template>

<style>
.counter {
  margin: 2rem;
  padding: 1rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  text-align: center;
}

button {
  margin: 0.5rem;
  padding: 0.5rem 1rem;
  background-color: #0070f3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  background-color: #0050c3;
}
</style>

<script>
use orbitrs::prelude::*;

#[derive(Default)]
pub struct Counter {
    count: i32,
}

impl Component for Counter {
    type Props = ();
    
    fn new(_props: Self::Props) -> Self {
        Self::default()
    }
    
    fn render(&self) -> String {
        // The template is automatically compiled to this function
        // This is just a placeholder
        "Counter component".to_string()
    }
}

impl Counter {
    pub fn increment(&mut self) {
        self.count += 1;
    }
    
    pub fn decrement(&mut self) {
        self.count -= 1;
    }
}
</script>
"#
        .to_string(),
    );

    // README.md
    template.insert(
        "README.md".to_string(),
        r#"# {{ project.project_name }}

This is an Orbit UI project.

## Development

```bash
orbiton dev
```

## Building

```bash
orbiton build
```
"#
        .to_string(),
    );

    // orbit.config.json
    template.insert(
        "orbit.config.json".to_string(),
        r#"{
  "renderer": "auto",
  "target": "web"
}
"#
        .to_string(),
    );

    template
}

/// Component library template
fn component_library_template() -> HashMap<String, String> {
    let mut template = basic_template();

    // Override Cargo.toml for a component library
    template.insert(
        "Cargo.toml".to_string(),
        r#"[package]
name = "{{ project.project_name }}"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A component library for Orbit UI framework"
license = "MIT OR Apache-2.0"

[dependencies]
orbit = "{{ project.orbit_version }}"

[build-dependencies]
orbiton = "{{ project.orbiton_version }}"

[lib]
name = "{{ project.project_name }}"
path = "src/lib.rs"
"#
        .to_string(),
    );

    // Add more components
    template.insert(
        "src/components/mod.rs".to_string(),
        r#"// Components for {{ project.project_name }}

pub mod button;
pub mod card;
pub mod counter;
pub mod input;
"#
        .to_string(),
    );

    // Add button component
    template.insert(
        "src/components/button.orbitrs".to_string(),
        r#"<template>
  <button 
    class="orbit-button {{ variant }}" 
    :disabled="disabled"
    @click="onClick">
    <slot></slot>
  </button>
</template>

<style>
.orbit-button {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
  transition: background-color 0.2s;
}

.primary {
  background-color: #0070f3;
  color: white;
}

.secondary {
  background-color: #f5f5f5;
  color: #333;
}

.danger {
  background-color: #ff0000;
  color: white;
}

.orbit-button:hover:not(:disabled) {
  opacity: 0.8;
}

.orbit-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>

<script>
use orbitrs::prelude::*;

#[derive(Default)]
pub struct Button {
    variant: String,
    disabled: bool,
    on_click: Option<Box<dyn Fn()>>,
}

#[derive(Default)]
pub struct ButtonProps {
    pub variant: Option<String>,
    pub disabled: Option<bool>,
    pub on_click: Option<Box<dyn Fn()>>,
}

impl Props for ButtonProps {}

impl Component for Button {
    type Props = ButtonProps;
    
    fn new(props: Self::Props) -> Self {
        Self {
            variant: props.variant.unwrap_or_else(|| "primary".to_string()),
            disabled: props.disabled.unwrap_or(false),
            on_click: props.on_click,
        }
    }
    
    fn render(&self) -> String {
        // The template is automatically compiled to this function
        // This is just a placeholder
        "Button component".to_string()
    }
}

impl Button {
    fn onClick(&self) {
        if let Some(on_click) = &self.on_click {
            on_click();
        }
    }
}
</script>
"#
        .to_string(),
    );

    template
}

/// Full application template
fn full_app_template() -> HashMap<String, String> {
    let mut template = basic_template();

    // Override main.rs for a full application
    template.insert(
        "src/main.rs".to_string(),
        r#"// Entry point for {{ project.project_name }}

use orbitrs::platform::{self, PlatformType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the application
    {{ project.project_name }}::init()?;
    
    // Create a platform adapter
    let mut platform = platform::create_adapter(PlatformType::Auto)?;
    
    // Initialize the platform
    platform.init()?;
    
    // Create a window
    let window = platform.create_window("{{ project.project_name }}", 800, 600)?;
    
    // Create the main component
    let app = {{ project.project_name }}::components::app::App::new(());
    
    // Set the window content
    platform.set_window_content(window, &app)?;
    
    // Run the platform event loop
    platform.run()?;
    
    Ok(())
}
"#
        .to_string(),
    );

    // Add app component
    template.insert(
        "src/components/app.orbitrs".to_string(),
        r#"<template>
  <div class="app">
    <header>
      <h1>{{ project.project_name }}</h1>
    </header>
    
    <main>
      <counter></counter>
    </main>
    
    <footer>
      <p>Built with Orbit UI Framework</p>
    </footer>
  </div>
</template>

<style>
.app {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  font-family: Arial, sans-serif;
}

header {
  background-color: #0070f3;
  color: white;
  padding: 1rem;
  text-align: center;
}

main {
  flex: 1;
  padding: 2rem;
  display: flex;
  flex-direction: column;
  align-items: center;
}

footer {
  background-color: #f5f5f5;
  padding: 1rem;
  text-align: center;
}
</style>

<script>
use orbitrs::prelude::*;
use crate::components::counter::Counter;

pub struct App;

impl Component for App {
    type Props = ();
    
    fn new(_props: Self::Props) -> Self {
        Self
    }
    
    fn render(&self) -> String {
        // The template is automatically compiled to this function
        // This is just a placeholder
        "App component".to_string()
    }
}
</script>
"#
        .to_string(),
    );

    // Update components module
    template.insert(
        "src/components/mod.rs".to_string(),
        r#"// Components for {{ project.project_name }}

pub mod app;
pub mod counter;
"#
        .to_string(),
    );

    template
}
