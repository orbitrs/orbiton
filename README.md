# 📦 `orbiton` — CLI Tooling for Orbit Framework

![CI Status](https://github.com/orbitrs/orbiton/actions/workflows/ci.yml/badge.svg)
![Release Status](https://github.com/orbitrs/orbiton/actions/workflows/release.yml/badge.svg)
[![codecov](https://codecov.io/gh/orbitrs/orbiton/branch/main/graph/badge.svg?token=CODECOV_TOKEN)](https://codecov.io/gh/orbitrs/orbiton)
[![crates.io](https://img.shields.io/crates/v/orbiton.svg)](https://crates.io/crates/orbiton)

> **Command-line interface to build, develop, and manage Orbit applications.**

---

### 🚀 Overview

`orbiton` is the official CLI for the [Orbit UI framework](https://github.com/orbitrs/orbit), designed to simplify the developer workflow across all supported targets: **Web**, **Native**, and **Embedded**.

Whether you're scaffolding a new component, starting a development server, or compiling to WebAssembly, `orbiton` has you covered.

---

### 🔧 Features

* 📁 **Project scaffolding**: `orbiton new` - Bootstrap new Orbit projects
* ⚡ **Dev server with HMR**: `orbiton dev` - Hot Module Reload for rapid development  
* 🛠️ **Multi-target builds**: `orbiton build` - WASM, native, and embedded compilation
* 🚀 **Deployment tools**: `orbiton deploy` - Streamlined deployment workflows
* 📊 **Performance profiling**: `orbiton profile` - Analyze and optimize performance
* 🎨 **Renderer configuration**: `orbiton renderer` - Skia/WGPU/Auto backend selection
* 🧪 **Testing utilities**: Component and integration testing (planned)
* 🔍 **Static analysis**: `orbiton lint` - Code quality via [orlint](https://github.com/orbitrs/orlint)
* 🔄 **Beta toolchain support**: Optional Rust beta feature testing

---

### 📦 Installation

```bash
cargo install orbiton
```

---

### 🛠️ Development Setup

For development, you'll need to clone the orbiton repository alongside its dependencies:

```bash
# Create a workspace directory
mkdir -p orbit-workspace
cd orbit-workspace

# Clone all required repositories
git clone https://github.com/orbitrs/orbiton.git
git clone https://github.com/orbitrs/orbitrs.git
git clone https://github.com/orbitrs/orlint.git

# Or use our setup script
cd orbiton
./scripts/setup-workspace.sh
```

#### Dependency Management

`orbiton` uses a feature-based system to manage its dependencies:

- Dependencies are specified as git URLs in the Cargo.toml file
- In local development, the `local-dependencies` feature (default) activates
- In CI environments, the `ci` feature is used
- The patch system in Cargo.toml and .cargo/config.toml overrides git dependencies with local paths
- This two-tier approach ensures consistent behavior between local development and CI

```bash
# 💻 Usage

```bash
orbiton new my-app
cd my-app
orbiton dev
```

#### Other Commands

```bash
orbiton build                       # Build app for target (auto-detects platform)
orbiton lint                        # Analyze your .orbit files for errors
orbiton generate                    # Generate components, services, or stores
orbiton renderer --config skia      # Configure default renderer to Skia
orbiton renderer --config wgpu      # Configure default renderer to WGPU
orbiton renderer --config auto      # Configure automatic renderer selection
```

---

### ⚡ Hot Module Reload (HMR)

The development server includes a full Hot Module Reload implementation that allows for rapid development without full page reloads. Key features include:

* **Intelligent Rebuilds**: Only rebuilds when necessary, with proper debouncing
* **Live Updates**: Real-time feedback on build success/failure
* **Beta Toolchain Support**: Opt-in testing with the latest Rust beta features

#### Using HMR

To start the development server with HMR:

```bash
# Basic usage
orbiton dev

# With beta toolchain
orbiton dev --beta

# Custom port
orbiton dev --port 9000
```

#### HMR Client Integration

For custom applications, you can register an HMR handler to enable surgical updates without a full page reload:

```javascript
// Register an HMR handler function that will be called when modules are updated
window.__ORBIT_REGISTER_HMR_HANDLER(function(modules) {
  console.log("Updated modules:", modules);
  
  // Your custom update logic here
  // For example, you might want to re-render specific components
});
```

---

### 🔮 Roadmap

* [ ] Target switching (Web, Native, Embedded)
* [ ] Preview mode for single components with renderer selection
* [ ] Integrated formatter
* [ ] Orbit component library sync
* [ ] Orbit playground launcher
* [ ] Performance analyzer for renderer optimization
* [ ] Visual renderer debugging tools

---

## 🌍 Repository

👉 [orbiton on GitHub](https://github.com/orbitrs/orbiton)

