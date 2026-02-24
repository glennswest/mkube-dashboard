#!/bin/bash
set -e

REGISTRY="192.168.200.3:5000"
IMAGE="mkube-dashboard"
TAG="edge"

echo "=== Deploying mkube-dashboard ==="

# Push to GHCR
echo "Pushing to GHCR..."
podman push ghcr.io/glennswest/${IMAGE}:${TAG}

# Copy to local registry
echo "Copying to local registry at ${REGISTRY}..."
crane copy ghcr.io/glennswest/${IMAGE}:${TAG} ${REGISTRY}/${IMAGE}:${TAG} --insecure

echo "=== Deploy complete ==="
echo "Image available at: ${REGISTRY}/${IMAGE}:${TAG}"
echo ""
echo "To deploy the pod:"
echo "  curl -X POST http://192.168.200.2:8082/api/v1/namespaces/infra/pods \\"
echo "    -H 'Content-Type: application/yaml' \\"
echo "    -d @pod.yaml"
