#!/bin/bash
# Script to set up the orbiton workspace for development

set -e

# Colors for console output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Determine the workspace root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
ORBITON_DIR="$SCRIPT_DIR"
WORKSPACE_DIR="$(dirname "$ORBITON_DIR")"

echo -e "${GREEN}Setting up development workspace for orbiton...${NC}"

# Check if we're in the expected directory structure
if [[ "$(basename "$ORBITON_DIR")" != "orbiton" ]]; then
    echo -e "${RED}Error: This script should be run from the orbiton directory.${NC}"
    exit 1
fi

# Check if orbitrs repository exists
if [[ ! -d "$WORKSPACE_DIR/orbitrs" ]]; then
    echo -e "${YELLOW}orbitrs repository not found. Cloning...${NC}"
    cd "$WORKSPACE_DIR"
    git clone https://github.com/orbitrs/orbitrs.git
else
    echo -e "${GREEN}orbitrs repository found.${NC}"
    cd "$WORKSPACE_DIR/orbitrs"
    git pull
fi

# Check if orbit-analyzer repository exists
if [[ ! -d "$WORKSPACE_DIR/orbit-analyzer" ]]; then
    echo -e "${YELLOW}orbit-analyzer repository not found. Cloning...${NC}"
    cd "$WORKSPACE_DIR"
    git clone https://github.com/orbitrs/orbit-analyzer.git
else
    echo -e "${GREEN}orbit-analyzer repository found.${NC}"
    cd "$WORKSPACE_DIR/orbit-analyzer"
    git pull
fi

echo -e "${GREEN}Workspace setup complete. Directory structure:${NC}"
ls -la "$WORKSPACE_DIR"

echo -e "\n${GREEN}You can now build orbiton with:${NC}"
echo -e "cd $ORBITON_DIR && cargo build"

# Create .cargo directory if it doesn't exist
if [[ ! -d "$ORBITON_DIR/.cargo" ]]; then
    echo -e "${YELLOW}Creating .cargo directory...${NC}"
    mkdir -p "$ORBITON_DIR/.cargo"
fi

# Create config.toml if it doesn't exist
if [[ ! -f "$ORBITON_DIR/.cargo/config.toml" ]]; then
    echo -e "${YELLOW}Creating .cargo/config.toml...${NC}"
    cat > "$ORBITON_DIR/.cargo/config.toml" << EOL
[patch.crates-io]
# Use local versions of dependencies when available
orbitrs = { path = "../orbitrs" }
orbit-analyzer = { path = "../orbit-analyzer" }
EOL
    echo -e "${GREEN}.cargo/config.toml created.${NC}"
else
    echo -e "${GREEN}.cargo/config.toml already exists.${NC}"
fi
