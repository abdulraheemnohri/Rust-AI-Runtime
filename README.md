# Rust AI Runtime

Rust-based, high-performance, fully offline AI runtime designed for Android initially, with cross-platform potential (Linux, Windows, macOS). It allows loading multiple AI models, running inference locally, and extending functionality via a plugin system.

## 1. Project Overview
- **Offline AI model inference**: No internet connection required.
- **Multi-model support**: Load and switch between multiple models dynamically.
- **Modular and plugin-ready**: Extend functionality via Rust plugins.
- **Lightweight and fast**: Memory-safe and multi-threaded Rust engine.
- **Easily integrable**: Ready for Android apps with JNI support.

## 2. Key Features
- **Runtime Core**: Memory-safe, multi-threaded Rust engine with `init_runtime()`.
- **Model Manager**: Load and cache models (`.ggml`, `.onnx`, etc.).
- **Inference Engine**: Local inference support (GGML/LLaMA, ONNX).
- **Plugin System**: Extend via Rust `.so` plugins.
- **Android Integration**: Multi-arch build support (`arm64-v8a`, `armeabi-v7a`, `x86_64`).

## 3. Architecture
```
[Android App / Desktop App]
        |
        v
[Rust AI Runtime Core]  <-- exposes extern "C" functions
        |
        +--> Model Manager      (load/unload/cache models)
        +--> Inference Engine   (GGML/ONNX, multi-threaded)
        +--> Plugin System      (custom extensions)
        +--> Storage Layer      (models, logs, user data)
```

## 4. Build Instructions
### Build for Android
1. Install `cargo-ndk`:
   ```bash
   cargo install cargo-ndk
   ```
2. Run the build script:
   ```bash
   ./scripts/build_android.sh
   ```
This will build the `.so` files and place them in the Android example's `jniLibs` directory.

### Build for Desktop (Linux/macOS/Windows)
```bash
cd rust_runtime
cargo build --release
```

## 5. Examples
See `examples/android_example` for a minimal Kotlin integration example.

## 6. Support & Monetization
This is an open-source project. If you find it useful, please consider supporting development through:
- **GitHub Sponsors**: [Link to your sponsors page]
- **Donations**: [PayPal/Ko-fi link]
- **Enterprise Licensing**: Contact for custom offline AI solutions.

## 7. License
This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
