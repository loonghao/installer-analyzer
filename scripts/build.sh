#!/bin/bash

# Build script for installer-analyzer
set -e

echo "Building installer-analyzer..."

# Clean previous builds
cargo clean

# Run tests
echo "Running tests..."
cargo test

# Build for different targets
echo "Building for multiple targets..."

# Linux x86_64
echo "Building for Linux x86_64..."
cargo build --release --target x86_64-unknown-linux-gnu

# Linux x86_64 musl
echo "Building for Linux x86_64 musl..."
cargo build --release --target x86_64-unknown-linux-musl

# Windows x86_64
echo "Building for Windows x86_64..."
cargo build --release --target x86_64-pc-windows-msvc

# macOS x86_64
echo "Building for macOS x86_64..."
cargo build --release --target x86_64-apple-darwin

# macOS ARM64
echo "Building for macOS ARM64..."
cargo build --release --target aarch64-apple-darwin

echo "Build completed successfully!"
echo "Binaries are located in target/<target>/release/"
