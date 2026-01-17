# {{project-name}}
# Run `just` to see available commands

default:
    @just --list

# Install all dependencies (npm + cargo)
install:
    rustup target add wasm32-unknown-unknown
    npm install
    cd crates/backend && npm install

# Build everything (WASM frontend + Neon backend)
build: build-wasm build-backend

# Build WASM frontend (release)
build-wasm:
    cd crates/frontend && trunk build --release

# Build WASM frontend (dev)
build-wasm-dev:
    cd crates/frontend && trunk build

# Build Neon backend (release)
build-backend:
    cd crates/backend && npm run build -- --release

# Build Neon backend (dev)
build-backend-dev:
    cd crates/backend && npm run build

# Run the Electron app
run:
    npm start

# Run in development mode (with DevTools)
dev:
    NODE_ENV=development npm start

# Watch WASM frontend for changes (run in separate terminal)
watch:
    cd crates/frontend && trunk watch

# Clean all build artifacts
clean:
    rm -rf target
    rm -rf node_modules
    rm -rf crates/backend/node_modules
    rm -rf crates/backend/target
    rm -rf crates/backend/index.node
    rm -rf electron/renderer

# Clean and rebuild everything
rebuild: clean install build

# Check Rust code (both crates)
check:
    cargo check --workspace

# Run tests
test:
    cargo test --workspace

# Format Rust code
fmt:
    cargo fmt --all

# Check formatting (CI-style, no auto-fix)
check-fmt:
    cargo fmt --all -- --check

# Run clippy lints (strict: fail on warnings)
lint:
    cargo clippy --workspace -- -D warnings

# Run all checks (check-fmt, lint, test)
ci: check-fmt lint test

# Full dev setup: install, build, and run
setup: install build run

# === Packaging / Distribution ===

# Build distributable packages for current platform
dist: build
    npm run dist

# Build Linux packages (AppImage, deb, rpm)
dist-linux: build
    npm run dist:linux

# Build Windows installer (requires Wine on Linux)
dist-win: build
    npm run dist:win

# Build macOS packages (requires macOS)
dist-mac: build
    npm run dist:mac

# Build all platforms (best done in CI)
dist-all: build
    npm run dist:linux
    npm run dist:win
    npm run dist:mac
