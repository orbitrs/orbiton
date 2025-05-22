# Dependency Management in Orbiton

This document explains how dependencies are managed in the Orbiton project, particularly focusing on the relationship between local development and CI environments.

## Overview

Orbiton has two primary dependencies:
- `orbitrs`: The core Orbit UI framework
- `orbit-analyzer`: Static analysis tool for Orbit UI files

These dependencies are referenced in different ways depending on the environment:

## Development Environment

In a local development environment:
1. Dependencies are declared as Git repository references in `Cargo.toml`
2. The `.cargo/config.toml` file contains patch sections that override these Git references with local paths
3. The `local-dependencies` feature is activated by default

```toml
# In Cargo.toml
[dependencies]
orbitrs = { git = "https://github.com/orbitrs/orbitrs.git", branch = "main" }
orbit-analyzer = { git = "https://github.com/orbitrs/orbit-analyzer.git", branch = "main" }

[features]
default = ["local-dependencies"]
local-dependencies = []
ci = []

# In .cargo/config.toml
[patch."https://github.com/orbitrs/orbitrs.git"]
orbitrs = { path = "../orbitrs" }

[patch."https://github.com/orbitrs/orbit-analyzer.git"]
orbit-analyzer = { path = "../orbit-analyzer" }
```

## CI Environment

In CI environments:
1. The workspace is set up with all necessary repositories
2. The default feature is changed from `local-dependencies` to `ci`
3. Patch declarations are added/updated to point to the correct paths
4. Version ambiguities (e.g., for the `tempfile` crate) are resolved by specifying exact versions

## Fixing CI Issues

When encountering CI issues related to dependencies:

1. **Patch Resolution Issues**: Ensure exact versions are specified for ambiguous dependencies
   ```toml
   tempfile = "3.8.0"  # Instead of just tempfile = "*"
   ```

2. **Path Issues**: Make sure the CI scripts correctly set up the workspace structure
   ```bash
   # Directory structure should be:
   orbitrs-workspace/
   ├── orbiton/
   ├── orbitrs/
   └── orbit-analyzer/  (or /tmp/orbit-analyzer in some CI configs)
   ```

3. **Feature Issues**: Don't include dependencies in features directly
   ```toml
   # DON'T do this:
   local-dependencies = ["orbitrs", "orbit-analyzer"]
   
   # DO this instead:
   local-dependencies = []
   ```

## Useful Scripts

Several scripts help maintain the correct dependency configuration:

- `orbit-workflows/scripts/setup-ci-workspace.sh`: Sets up the CI workspace structure
- `orbit-workflows/scripts/fix-orbit-analyzer-dependency.sh`: Fixes the orbit-analyzer dependency
- `orbit-workflows/scripts/ci-update-features.sh`: Updates feature flags for CI

## Testing Locally

To test CI dependency resolution locally:

```bash
# Clone the repository in the right structure
mkdir -p orbitrs-workspace
cd orbitrs-workspace
git clone https://github.com/orbitrs/orbiton.git
git clone https://github.com/orbitrs/orbitrs.git
git clone https://github.com/orbitrs/orbit-analyzer.git

# Run the CI setup script
cd orbiton
./scripts/setup-ci-workspace.sh

# Test with CI features
cd ..
cd orbiton
cargo check --features ci --no-default-features
```
