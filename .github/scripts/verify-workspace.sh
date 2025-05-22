#!/bin/bash

# Verify the local development workspace setup
# Usage: ./verify-workspace.sh

set -e

# Create workspace directory if it doesn't exist
mkdir -p ../orbitrs-workspace

# Check if required dependencies are present
if [ ! -d "../orbitrs" ]; then
    echo "Error: orbitrs dependency not found"
    echo "Please clone https://github.com/orbitrs/orbitrs into the parent directory"
    exit 1
fi

if [ ! -d "../orbit-analyzer" ]; then
    echo "Error: orbit-analyzer dependency not found"
    echo "Please clone https://github.com/orbitrs/orbit-analyzer into the parent directory"
    exit 1
fi

# Verify Cargo.toml setup
if ! grep -q '\[patch."https://github.com/orbitrs/orbitrs.git"\]' .cargo/config.toml 2>/dev/null; then
    echo "Warning: Local patch for orbitrs not found in .cargo/config.toml"
fi

if ! grep -q '\[patch."https://github.com/orbitrs/orbit-analyzer.git"\]' .cargo/config.toml 2>/dev/null; then
    echo "Warning: Local patch for orbit-analyzer not found in .cargo/config.toml"
fi

echo "Workspace verification completed!"