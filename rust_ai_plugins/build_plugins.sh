#!/bin/bash
# Script to build all plugins for the host platform.
# For Android, use the multi-arch build scripts instead.

PLUGINS=("voice_assistant" "summarizer" "analytics")

# Since this is a Cargo workspace, we can build all from the root
# but copying artifacts requires finding them in the root target directory.
for p in "${PLUGINS[@]}"; do
    echo "Building plugin: $p"
    # Build from root for consistent target paths
    cargo build -p "$p" --release

    # Copy the built library to the rust_ai_plugins/ directory
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        cp "../target/release/lib$p.so" . 2>/dev/null || cp "../target/release/deps/lib$p.so" .
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        cp "../target/release/lib$p.dylib" . 2>/dev/null || cp "../target/release/deps/lib$p.dylib" .
    elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
        cp "../target/release/$p.dll" . 2>/dev/null || cp "../target/release/deps/$p.dll" .
    fi
done

echo "All plugins built and libraries copied to rust_ai_plugins/"
