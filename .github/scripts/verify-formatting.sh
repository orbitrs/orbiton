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
    echo '[patch."https://github.com/orbitrs/orbit.git"]' > .cargo/config.toml
    echo 'orbit = { path = "../orbit" }' >> .cargo/config.toml
    echo '[patch."https://github.com/orbitrs/orlint.git"]' >> .cargo/config.toml
    echo 'orlint = { path = "../orlint" }' >> .cargo/config.toml
fi

# Temporarily add workspace config and fix workspace dependencies
cp Cargo.toml Cargo.toml.bak
echo "Preparing Cargo.toml for formatting check..."

# Create a sed command that works on both macOS and Linux
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS needs an empty string for the extension to modify files in-place
    SED_INPLACE="sed -i ''"
else
    # Linux version
    SED_INPLACE="sed -i"
fi

# Create temporary file with workspace configuration and explicit dependencies
echo "[workspace]" > Cargo.toml.tmp
echo "members = [\".\"]\n" >> Cargo.toml.tmp

# Replace workspace dependencies with explicit versions
cat Cargo.toml | $SED_INPLACE -E 's/orbit\.workspace = true/orbit = { path = "..\/orbit" }/g' Cargo.toml.tmp
cat Cargo.toml | $SED_INPLACE -E 's/orlint\.workspace = true/orlint = { path = "..\/orlint" }/g' Cargo.toml.tmp

# Append modified content
cat Cargo.toml >> Cargo.toml.tmp
mv Cargo.toml.tmp Cargo.toml

# Run formatting check
echo "Running cargo fmt..."
cargo fmt --all -- --check
FORMAT_EXIT_CODE=$?

# Restore original Cargo.toml
echo "Restoring original Cargo.toml..."
mv Cargo.toml.bak Cargo.toml

# Return the formatter exit code
exit $FORMAT_EXIT_CODE

echo "Formatting check completed successfully!"