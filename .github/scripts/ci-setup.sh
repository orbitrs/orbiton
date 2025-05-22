#!/bin/bash
# CI setup script for orbiton repository
# Handles setup of workspace and dependencies for CI environment

set -euo pipefail

echo "Setting up CI environment for orbiton..."

# Create workspace directory structure
mkdir -p ../orbitrs-workspace
mkdir -p ../orbitrs-workspace/orbiton
mkdir -p ../orbitrs-workspace/orbitrs
mkdir -p ../orbitrs-workspace/orbit-analyzer

# Copy orbiton to workspace
echo "Copying orbiton to workspace..."
cp -R . ../orbitrs-workspace/orbiton/

# Clone dependencies
echo "Cloning dependencies..."
git clone --depth 1 https://github.com/orbitrs/orbitrs.git ../orbitrs-workspace/orbitrs
git clone --depth 1 https://github.com/orbitrs/orbit-analyzer.git ../orbitrs-workspace/orbit-analyzer

# Update Cargo.toml for CI environment
echo "Configuring dependencies for CI..."
cd ../orbitrs-workspace/orbiton

# Enable CI feature flag
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' 's/default = \["local-dependencies"\]/default = ["ci"]/g' Cargo.toml
else
    sed -i 's/default = \["local-dependencies"\]/default = ["ci"]/g' Cargo.toml
fi

echo "CI setup complete"
