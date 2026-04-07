# Rust AI Runtime Core

The heart of the RustAI Ecosystem. This module contains the high-performance AI engine and internal validation examples.

## Project Structure
- `rust_runtime/`: The core Rust library providing the C-FFI interface.
- `examples/`: Minimal integration examples for quick validation.
- `scripts/`: Build utilities for cross-platform and multi-arch distribution.
- `models/`: Placeholder for sample AI models.

## Build Instructions
### Build for Android (Multi-Arch)
Ensure `cargo-ndk` is installed:
```bash
cargo install cargo-ndk
```
Run the build script from the root of this module:
```bash
cd rust_ai_runtime
./scripts/build_android.sh
```
The artifacts will be placed in the `rust_ai_examples/android_project` library module.

### Build for Host Desktop
```bash
cargo build -p rust_runtime --release
```

## Internal Examples
- `android_example/`: A minimal Android Studio project for core engine validation.
- `cli_example/`: A simple CLI tool for testing inference locally.
