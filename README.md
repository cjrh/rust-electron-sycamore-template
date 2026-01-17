# Rust + Electron + Sycamore Template

A full Rust stack template for building desktop applications with Electron.

- **Frontend**: [Sycamore](https://sycamore-rs.netlify.app/) reactive UI framework compiled to WASM
- **Backend**: [Neon](https://neon-rs.dev/) for native Node.js bindings (file I/O, system APIs, etc.)
- **Shell**: Electron with IPC bridge between WASM frontend and Neon backend

## Overview

This is a template for [cargo-generate](https://github.com/cargo-generate/cargo-generate). It scaffolds a complete Rust + Electron application with:

```
┌─────────────────────────────────────────────┐
│            Electron App                     │
│  ┌───────────────────────────────────────┐  │
│  │  Main Process (Node.js + Neon)        │  │
│  │  - Native Rust backend via Neon       │  │
│  └───────────────────────────────────────┘  │
│                    │ IPC                    │
│  ┌───────────────────────────────────────┐  │
│  │  Renderer Process (Chromium + WASM)   │  │
│  │  - Sycamore reactive UI               │  │
│  └───────────────────────────────────────┘  │
└─────────────────────────────────────────────┘
```

### Create a New Project

```bash
# Install cargo-generate (once)
cargo install cargo-generate

# Generate from this template
cargo generate --git https://github.com/youruser/rust-electron-sycamore-template
```

You'll be prompted for:
- **Project name** (kebab-case, e.g., `my-app`)
- **Description**
- **Author**

## Justfile Commands

This project uses [just](https://github.com/casey/just) as a command runner.

### Getting Started

You will need to have the following installed:
- [Node.js](https://nodejs.org/) (v16+)
- [Rust](https://www.rust-lang.org/) (stable)
- [trunk](https://trunkrs.dev/) (for building WASM)

Then, run:

```bash
just setup    # Install deps, build everything, and run
```

### Development

```bash
just dev      # Run with DevTools open
just watch    # Watch frontend for changes (run in separate terminal)
```

### Building

```bash
just build          # Build everything (release)
just build-wasm     # Build only WASM frontend
just build-backend  # Build only Neon backend
```

### Maintenance

```bash
just check    # Cargo check workspace
just fmt      # Format Rust code
just lint     # Run clippy
just clean    # Remove all build artifacts
```

Run `just` with no arguments to see all available commands.
