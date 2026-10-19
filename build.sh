#!/bin/bash

set -e  # Stop the script if any command fails

echo "=== Adding targets for all platforms ==="

# Intel Target
echo "Adding target: x86_64-apple-darwin (Intel Mac)"
rustup target add x86_64-apple-darwin || { echo "Failed to add Intel Mac target"; exit 1; }

# ARM Target
echo "Adding target: aarch64-apple-darwin (ARM Mac)"
rustup target add aarch64-apple-darwin || { echo "Failed to add ARM Mac target"; exit 1; }

# Windows Target
echo "Adding target: x86_64-pc-windows-gnu (Windows)"
rustup target add x86_64-pc-windows-gnu || { echo "Failed to add Windows target"; exit 1; }

# Linux Target
echo "Adding target: x86_64-unknown-linux-musl (Linux)"
rustup target add x86_64-unknown-linux-musl || { echo "Failed to add Linux target"; exit 1; }

echo "=== Starting builds for all platforms ==="

# Intel Build
echo "Building for Intel Mac (x86_64-apple-darwin)..."
cargo build --release --target x86_64-apple-darwin || { echo "Build for Intel Mac failed"; exit 1; }
echo "Build for Intel Mac completed."

# ARM Build
echo "Building for ARM Mac (aarch64-apple-darwin)..."
cargo build --release --target aarch64-apple-darwin || { echo "Build for ARM Mac failed"; exit 1; }
echo "Build for ARM Mac completed."

# Windows Build
echo "Building for Windows (x86_64-pc-windows-gnu)..."
cargo build --release --target x86_64-pc-windows-gnu || { echo "Build for Windows failed"; exit 1; }
echo "Build for Windows completed."

# Linux Build
echo "Building for Linux (x86_64-unknown-linux-musl)..."
cargo build --release --target x86_64-unknown-linux-musl || { echo "Build for Linux failed"; exit 1; }
echo "Build for Linux completed."

echo "=== All builds completed successfully! ==="