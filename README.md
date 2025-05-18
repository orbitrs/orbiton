# 📦 `orbiton` — CLI Tooling for Orbit Framework

> **Command-line interface to build, develop, and manage Orbit applications.**

---

### 🚀 Overview

`orbiton` is the official CLI for the [Orbit UI framework](https://github.com/orbitrs/orbit), designed to simplify the developer workflow across all supported targets: **Web**, **Native**, and **Embedded**.

Whether you're scaffolding a new component, starting a development server, or compiling to WebAssembly, `orbiton` has you covered.

---

### 🔧 Features

* 📁 Project scaffolding: `orbiton new`
* ⚡ Dev server with hot reload: `orbiton dev`
* 🛠️ Build pipeline for WASM, native, and embedded: `orbiton build`
* 🧪 Component test utilities (planned)
* 🔍 Analyzer integration: `orbiton lint` (via [orbit-analyzer](https://github.com/orbitrs/orbit-analyzer))

---

### 📦 Installation

```bash
cargo install orbiton
```

---

### 💻 Usage

```bash
orbiton new my-app
cd my-app
orbiton dev
```

#### Other Commands

```bash
orbiton build        # Build app for target (auto-detects platform)
orbiton lint         # Analyze your .orbit files for errors
orbiton generate     # Generate components, services, or stores
```

---

### 🔮 Roadmap

* [ ] Target switching (Web, Native, Embedded)
* [ ] Preview mode for single components
* [ ] Integrated formatter
* [ ] OrbitKit component sync
* [ ] Orbit playground launcher

---

## 🌍 Repository

👉 [orbiton on GitHub](https://github.com/orbitrs/orbiton)

