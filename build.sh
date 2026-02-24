#!/bin/bash
set -e

echo "=== Building mkube-dashboard ==="

# Cross-compile for ARM64 Linux
echo "Cross-compiling for aarch64-unknown-linux-musl..."
cargo build --release --target aarch64-unknown-linux-musl

# Build container image
echo "Building container image..."
podman build --platform linux/arm64 -t ghcr.io/glennswest/mkube-dashboard:edge -f Containerfile .

echo "=== Build complete ==="
echo "Binary: target/aarch64-unknown-linux-musl/release/mkube-dashboard"
echo "Image: ghcr.io/glennswest/mkube-dashboard:edge"
