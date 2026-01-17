#!/bin/bash
set -e

echo "=== Rust + Electron + Sycamore Build Script ==="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check for required tools
check_tool() {
    if ! command -v $1 &> /dev/null; then
        echo -e "${YELLOW}Warning: $1 not found. $2${NC}"
        return 1
    fi
    return 0
}

echo "Checking required tools..."
check_tool "rustup" "Install from https://rustup.rs"
check_tool "trunk" "Install with: cargo install --locked trunk"
check_tool "npm" "Install Node.js from https://nodejs.org"

# Ensure WASM target is installed
echo ""
echo "Ensuring wasm32-unknown-unknown target is installed..."
rustup target add wasm32-unknown-unknown

# Install root npm dependencies
echo ""
echo -e "${GREEN}[1/4] Installing root npm dependencies...${NC}"
npm install

# Build WASM frontend
echo ""
echo -e "${GREEN}[2/4] Building Sycamore WASM frontend...${NC}"
cd crates/frontend
trunk build --release
cd ../..

# Install and build Neon backend
echo ""
echo -e "${GREEN}[3/4] Installing backend npm dependencies...${NC}"
cd crates/backend
npm install
cd ../..

echo ""
echo -e "${GREEN}[4/4] Building Neon backend...${NC}"
npm run build:backend

echo ""
echo -e "${GREEN}=== Build Complete! ===${NC}"
echo ""
echo "To run the app:"
echo "  npm start"
echo ""
echo "For development with DevTools:"
echo "  npm run dev"
