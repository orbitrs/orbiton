#!/bin/bash
# CI setup script for orbiton repository
# Handles setup of workspace and dependencies for CI environment

set -euo pipefail

echo "Setting up CI environment for orbiton..."

# Create workspace directory structure
mkdir -p ../orbitrs-workspace
mkdir -p ../orbitrs-workspace/orbiton
mkdir -p ../orbitrs-workspace/orbit
mkdir -p ../orbitrs-workspace/orlint

# Copy orbiton to workspace
echo "Copying orbiton to workspace..."
cp -R . ../orbitrs-workspace/orbiton/

# Clone dependencies
echo "Cloning dependencies..."
git clone --depth 1 https://github.com/orbitrs/orbit.git ../orbitrs-workspace/orbit
git clone --depth 1 https://github.com/orbitrs/orlint.git ../orbitrs-workspace/orlint

# Update Cargo.toml for CI environment
echo "Configuring dependencies for CI..."
cd ../orbitrs-workspace/orbiton

# Create proper cargo config
mkdir -p .cargo
echo "[patch.\"https://github.com/orbitrs/orbit.git\"]" > .cargo/config.toml
echo "orbit = { path = \"../orbit\" }" >> .cargo/config.toml
echo "[patch.\"https://github.com/orbitrs/orlint.git\"]" >> .cargo/config.toml
echo "orlint = { path = \"../orlint\" }" >> .cargo/config.toml

# Add temporary workspace configuration for formatting
cp Cargo.toml Cargo.toml.original
echo -e "[workspace]\nmembers = [\".\"]\n" > Cargo.toml.temp
cat Cargo.toml.original >> Cargo.toml.temp
mv Cargo.toml.temp Cargo.toml

# Enable CI feature flag
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' 's/default = \["local-dependencies"\]/default = ["ci"]/g' Cargo.toml
else
    sed -i 's/default = \["local-dependencies"\]/default = ["ci"]/g' Cargo.toml
fi

echo "CI setup complete"
