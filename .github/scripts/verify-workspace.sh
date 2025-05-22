#!/bin/bash

# Verify the local development workspace setup
# Usage: ./verify-workspace.sh

set -e

# Create workspace directory if it doesn't exist
mkdir -p ../orbitrs-workspace

# Check if required dependencies are present
if [ ! -d "../orbit" ]; then
    echo "Error: orbit dependency not found"
    echo "Please clone https://github.com/orbitrs/orbit into the parent directory"
    exit 1
fi

if [ ! -d "../orlint" ]; then
    echo "Error: orlint dependency not found"
    echo "Please clone https://github.com/orbitrs/orlint into the parent directory"
    exit 1
fi

# Verify Cargo.toml setup
mkdir -p .cargo
if [ ! -f ".cargo/config.toml" ] || ! grep -q '\[patch."https://github.com/orbitrs/orbit.git"\]' .cargo/config.toml; then
    echo "Setting up local patch for orbit..."
    echo '[patch."https://github.com/orbitrs/orbit.git"]' > .cargo/config.toml
    echo 'orbit = { path = "../orbit" }' >> .cargo/config.toml
fi

if ! grep -q '\[patch."https://github.com/orbitrs/orlint.git"\]' .cargo/config.toml; then
    echo "Setting up local patch for orlint..."
    echo '[patch."https://github.com/orbitrs/orlint.git"]' >> .cargo/config.toml
    echo 'orlint = { path = "../orlint" }' >> .cargo/config.toml
fi

echo "Workspace verification completed!"