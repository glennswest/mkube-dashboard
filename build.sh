#!/bin/bash
# Build mkube-dashboard: cross-compile locally, then podman build
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

REGISTRY="registry.gt.lo:5000"
IMAGE="$REGISTRY/mkube-dashboard:edge"

echo "=== Building mkube-dashboard ==="

# Cross-compile for ARM64 Linux
echo "Cross-compiling for aarch64-unknown-linux-musl..."
cargo build --release --target aarch64-unknown-linux-musl

# Build container image
echo "Building container image..."
podman build --platform linux/arm64 -t "$IMAGE" -f Containerfile .

echo "=== Build complete ==="
echo "Image: $IMAGE"
echo "Run ./deploy.sh to push and deploy to rose1"
