#!/bin/bash

# Verify and format Rust code
# Usage: ./verify-formatting.sh

set -e

echo "Checking code formatting..."

# Check if rustfmt is installed
if ! command -v rustfmt &> /dev/null; then
    echo "Installing rustfmt..."
    rustup component add rustfmt
fi

# Verify cargo workspace setup
if [ ! -f ".cargo/config.toml" ]; then
    echo "Setting up cargo config..."
    mkdir -p .cargo
    echo '[patch."https://github.com/orbitrs/orbitrs.git"]' > .cargo/config.toml
    echo 'orbitrs = { path = "../orbitrs" }' >> .cargo/config.toml
    echo '[patch."https://github.com/orbitrs/orbit-analyzer.git"]' >> .cargo/config.toml
    echo 'orbit-analyzer = { path = "../orbit-analyzer" }' >> .cargo/config.toml
fi

# Temporarily add workspace config if needed
if ! grep -q "\[workspace\]" Cargo.toml; then
    echo "Adding temporary workspace configuration..."
    cp Cargo.toml Cargo.toml.bak
    echo -e "[workspace]\nmembers = [\".\"]\n$(cat Cargo.toml)" > Cargo.toml
fi

# Run formatting check
echo "Running cargo fmt..."
cargo fmt --all -- --check

# Restore original Cargo.toml if it was modified
if [ -f "Cargo.toml.bak" ]; then
    mv Cargo.toml.bak Cargo.toml
fi

echo "Formatting check completed successfully!"