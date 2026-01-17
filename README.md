# Rust + Electron + Sycamore + Neon Template


![App Screenshot](./defaultui.png)

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

### Create your New Project using cargo-generate

```bash
# Install cargo-generate (once)
cargo install cargo-generate
or
cargo binstall cargo-generate (faster)

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
- [just](https://github.com/casey/just) (command runner)

Then, run:

```bash
just setup    # Install deps, build everything, and run
```

### Licensing

This **template repo** is licensed under the [MIT License](LICENSE). The repo
that you generate from this template will **not** contain this license file.
During the cargo-generate process, you will be prompted to choose your own
licence, which will be added to the workspace `Cargo.toml`. However, you still
need to add the appropriate LICENCE file to your code. This is not done
automatically by cargo-generate.

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

## Integrating External JavaScript Libraries

This template includes a Chart.js integration example demonstrating how to incorporate external JavaScript libraries into your Sycamore/WASM application. The pattern described here applies to any JavaScript library you want to use.

### Prefer Local Assets Over CDNs

For Electron desktop applications, **always prefer bundling JavaScript libraries locally** rather than loading them from CDNs. Reasons include:

- **Offline support**: Desktop apps should work without an internet connection
- **Predictable behavior**: No risk of CDN outages or version changes breaking your app
- **Security**: Reduced attack surface from third-party hosted scripts
- **Performance**: No network latency on startup

Only use CDN-loaded assets if you have a specific requirement for it (e.g., loading user-specified external resources).

### How to Add a JavaScript Library

#### 1. Install via npm

```bash
npm install <library-name> --save
```

This downloads the library to `node_modules/` and records the dependency in `package.json`.

#### 2. Copy to Frontend Assets

Copy the library's distributable file (usually a UMD or IIFE build) to the frontend assets:

```bash
mkdir -p crates/frontend/assets
cp node_modules/<library-name>/dist/<library>.min.js crates/frontend/assets/
```

Look for `.umd.js`, `.umd.min.js`, or `.min.js` files in the library's `dist/` folder. These are browser-ready builds that don't require a module bundler.

#### 3. Configure Trunk to Bundle the Asset

In `crates/frontend/index.html`, add a trunk directive to copy the file and a script tag to load it:

```html
<link data-trunk rel="copy-file" href="assets/<library>.min.js" />
<script src="<library>.min.js"></script>
```

When trunk builds the frontend, it copies the asset to the output directory (`electron/renderer/`).

#### 4. Create a JavaScript Helper Function

Add a helper function in `index.html` that your Rust code can call:

```html
<script>
  window.myLibraryHelper = function(arg1, arg2) {
    // Use the library here
    return SomeLibrary.doSomething(arg1, arg2);
  };
</script>
```

This pattern keeps library-specific JavaScript isolated and provides a clean interface for Rust.

#### 5. Call from Rust via wasm_bindgen

In your Rust code, declare the external function and call it:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = myLibraryHelper)]
    fn my_library_helper(arg1: &str, arg2: i32);
}

// Then call it from your component
my_library_helper("hello", 42);
```

For functions that need to run after the DOM is ready, use Sycamore's `on_mount`:

```rust
use sycamore::web::on_mount;

#[component]
fn MyComponent() -> View {
    on_mount(|| {
        my_library_helper("hello", 42);
    });

    view! { /* ... */ }
}
```

### How Assets End Up in the Final Bundle

The build pipeline works as follows:

1. `npm run build:wasm` runs trunk in `crates/frontend/`
2. Trunk compiles Rust to WASM and processes `index.html`
3. Assets marked with `data-trunk rel="copy-file"` are copied to `electron/renderer/`
4. Electron loads `electron/renderer/index.html` which references the local assets
5. When packaging the app, everything in `electron/` is bundled into the final executable

### Example: Chart.js Integration

This template includes a working Chart.js example. The relevant files are:

- `crates/frontend/assets/chart.umd.min.js` - Local Chart.js library
- `crates/frontend/index.html` - Trunk copy directive, script tag, and `initDemoChart` helper
- `crates/frontend/src/lib.rs` - `ChartDemo` component with `wasm_bindgen` extern declaration
- `crates/frontend/styles.css` - Chart container styling

To remove this example from your project, delete the `ChartDemo` component and its usage in `App`, remove the Chart.js script/helper from `index.html`, delete the assets file, and remove the chart styles from CSS.
