# Puppet Plugin Template (Rust)

This is a template project for creating Puppet plugins using Rust. Follow the instructions below to get started with your plugin development.

## Important Note

When creating your plugin, **do not modify the plugin name in `Cargo.toml`**. Instead, change the plugin name in `manifest.json`. This ensures proper plugin identification and loading in Puppet.

## Prerequisites

Before building the plugin, ensure you have the WebAssembly target installed for Rust. If you haven't added it yet, run:

```bash
rustup target add wasm32-unknown-unknown
# if you are going to use wasi api's add wasi target
rustup target add wasm32-wasip1
```

## Building

To build your plugin, use the following command:

```bash
cargo build --release --target wasm32-unknown-unknown && cp ./target/wasm32-unknown-unknown/release/plugin.wasm .
# if you used wasi api, for example file operations, build wiht wasi target, and set "wasi" to "true" at manifest.json
cargo build --release --target wasm32-wasip1 && cp ./target/wasm32-wasip1/release/plugin.wasm .
```

This will compile your Rust code into WebAssembly, which can be loaded by Puppet.

## Output

Your wasm binary should be located at the root of the plugin folder and file name must be `plugin.wasm`.
