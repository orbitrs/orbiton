#!/usr/bin/env zsh
# This script verifies that the workspace is correctly configured
# for both local development and CI purposes

set -e

SCRIPT_DIR=$(dirname "$0")
REPO_ROOT=$(cd "$SCRIPT_DIR/.." && pwd)

echo "🔍 Verifying Orbiton workspace configuration..."

# Define expected paths
ORBITRS_PATH="../orbitrs"
ANALYZER_PATH="../orbit-analyzer"

# Check for Cargo.toml
if [[ ! -f "$REPO_ROOT/Cargo.toml" ]]; then
    echo "❌ Error: Cargo.toml not found in repository root"
    exit 1
fi

# Check for features in Cargo.toml
if ! grep -q "\[features\]" "$REPO_ROOT/Cargo.toml"; then
    echo "⚠️ Warning: No [features] section found in Cargo.toml"
fi

# Check for default features
if ! grep -q "default = \[" "$REPO_ROOT/Cargo.toml"; then
    echo "⚠️ Warning: No default features defined in Cargo.toml"
fi

# Check for orbitrs dependency
if ! grep -q "orbitrs = " "$REPO_ROOT/Cargo.toml"; then
    echo "❌ Error: orbitrs dependency not found in Cargo.toml"
    exit 1
fi

# Check for orbit-analyzer dependency
if ! grep -q "orbit-analyzer = " "$REPO_ROOT/Cargo.toml"; then
    echo "❌ Error: orbit-analyzer dependency not found in Cargo.toml"
    exit 1
fi

# Check for patch sections
if ! grep -q "\[patch" "$REPO_ROOT/Cargo.toml"; then
    echo "⚠️ Warning: No patch sections found in Cargo.toml"
fi

# Check .cargo/config.toml
if [[ ! -f "$REPO_ROOT/.cargo/config.toml" ]]; then
    echo "⚠️ Warning: .cargo/config.toml not found. Local development may not work correctly."
else
    echo "✅ .cargo/config.toml found. Checking contents..."
    if ! grep -q "\[patch" "$REPO_ROOT/.cargo/config.toml"; then
        echo "⚠️ Warning: No patch sections found in .cargo/config.toml"
    fi
fi

# Check for actual dependencies on disk
if [[ ! -d "$REPO_ROOT/$ORBITRS_PATH" ]]; then
    echo "⚠️ Warning: orbitrs repository not found at $ORBITRS_PATH"
else
    echo "✅ orbitrs repository found at $ORBITRS_PATH"
fi

if [[ ! -d "$REPO_ROOT/$ANALYZER_PATH" ]]; then
    echo "⚠️ Warning: orbit-analyzer repository not found at $ANALYZER_PATH"
else
    echo "✅ orbit-analyzer repository found at $ANALYZER_PATH"
fi

# Check for tempfile with explicit version
if grep -q "tempfile = " "$REPO_ROOT/Cargo.toml"; then
    if ! grep -q "tempfile = \"[0-9]" "$REPO_ROOT/Cargo.toml"; then
        echo "⚠️ Warning: tempfile dependency without specific version may cause issues in CI"
    else
        echo "✅ tempfile dependency has specific version"
    fi
fi

echo ""
echo "🏁 Workspace verification complete!"
echo ""
echo "If you see any warnings or errors above, please refer to docs/dependency-management.md"
echo "for guidance on setting up the workspace correctly."
