# RustAI_Ecosystem

A modular, high-performance, offline AI platform built in Rust for Android, desktop, and potentially web.

## Project Structure
- `rust_ai_runtime/`: Core Rust library, internal examples, and build scripts.
- `rust_ai_plugins/`: Modular AI task plugins (Voice Assistant, Summarizer, Analytics).
- `rust_ai_examples/`: Production-ready application examples.

## Key Features
- **Offline AI Inference**: Run AI models locally on mobile and desktop.
- **Dynamic Plugin System**: Extend the runtime with specialized modules at runtime.
- **Multi-arch Android Support**: Built for `aarch64`, `armv7`, and `x86_64`.
- **Security-First**: Integrated SLSA 3 provenance for all build artifacts.
- **Easy Distribution**: Automated publishing to GitHub Packages.

## Releases & Distribution
Automated releases are available on the [GitHub Releases](https://github.com/YourUsername/RustAI_Ecosystem/releases) page.
- **Desktop Libraries**: Pre-built `.so`, `.dylib`, and `.dll` files.
- **Android AARs**: Available via GitHub Packages (Maven).
- **CLI Tools**: Ready-to-run binaries for quick evaluation.

To trigger a new release, use the provided script:
```bash
./rust_ai_runtime/scripts/publish.sh v1.0.0
```

## Getting Started
1. **Core Engine**: [rust_ai_runtime/README.md](rust_ai_runtime/README.md)
2. **Pluggable Tasks**: [rust_ai_plugins/README.md](rust_ai_plugins/README.md)
3. **App Integration**: [rust_ai_examples/README.md](rust_ai_examples/README.md)

## License
Licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.
