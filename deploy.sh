#!/bin/bash
# Build, push, and deploy mkube-dashboard to mkube on rose1
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

REGISTRY="registry.gt.lo:5000"
IMAGE="$REGISTRY/mkube-dashboard:edge"

echo "=== Deploying mkube-dashboard ==="

# Build
"$SCRIPT_DIR/build.sh"

# Push to local registry (mkube will detect and redeploy)
echo "Pushing to $REGISTRY..."
podman push --tls-verify=false "$IMAGE"

echo ""
echo "=== Done ==="
echo "Deployed mkube-dashboard to $REGISTRY"
echo "Auto-updated by mkube"
