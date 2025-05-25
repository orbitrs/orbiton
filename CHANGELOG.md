# Changelog

All notable changes to the Orbiton CLI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial implementation of the CLI for Orbit UI framework
- Commands for project scaffolding (`orbiton new`)
- Development server with auto-refresh (`orbiton dev`)
- Build command with output optimization (`orbiton build`)
- Integration with orlint for linting
- **Configuration system with .orbiton.toml support**
- **HMR (Hot Module Reload) system with timestamp-based cleanup**
- **Maintenance command for project cleanup operations**
- **Enhanced development server with configuration integration**
- **Comprehensive integration tests for all core functionality**

### Fixed
- Improved error messages and handling
- Enhanced project templates with better documentation
- Configuration file handling with validation
- CI/CD pipeline compatibility with multi-target builds
- WASM build process with proper feature flag handling
- **Resolved all compiler warnings through proper code usage patterns**
- **Fixed dead code warnings with appropriate allow annotations**

## [0.1.0] - 2025-05-21
- Initial public release

[Unreleased]: https://github.com/orbitrs/orbiton/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/orbitrs/orbiton/releases/tag/v0.1.0
