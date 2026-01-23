#!/usr/bin/env bash
set -euo pipefail

# Build and push Docker image for a microservice
#
# Usage: build-microservice-docker-simple.sh <image_name> <dockerfile> <hash_path> <artifact_path>
#
# Example:
#   build-microservice-docker-simple.sh \
#     localhost:5001/rerp-general-ledger \
#     docker/microservices/Dockerfile.general-ledger \
#     build_artifacts/general_ledger.sha256 \
#     build_artifacts/general_ledger

if [[ $# -lt 4 ]]; then
  echo "usage: $0 <image_name> <dockerfile> <hash_path> <artifact_path>" >&2
  echo "  example: $0 localhost:5001/rerp-general-ledger docker/microservices/Dockerfile.general-ledger build_artifacts/general_ledger.sha256 build_artifacts/general_ledger" >&2
  exit 1
fi

image_name="$1"
dockerfile="$2"
hash_path="$3"
artifact_path="$4"

# Verify hash file exists (indicates copy completed)
if [[ ! -f "$hash_path" ]]; then
  echo "❌ Error: Hash file not found: $hash_path" >&2
  echo "   This indicates copy script has not completed yet" >&2
  exit 1
fi

# Verify artifact exists
if [[ ! -f "$artifact_path" ]]; then
  echo "❌ Error: Artifact not found: $artifact_path" >&2
  exit 1
fi

# Verify dockerfile exists
if [[ ! -f "$dockerfile" ]]; then
  echo "❌ Error: Dockerfile not found: $dockerfile" >&2
  exit 1
fi

# Build Docker image
echo "🔨 Building Docker image: $image_name:tilt"
if ! docker build -t "${image_name}:tilt" --rm --force-rm -f "$dockerfile" .; then
  echo "❌ Error: Docker build failed" >&2
  exit 1
fi

# Push Docker image (to local registry if available, or just tag for Kind)
echo "📤 Tagging Docker image: $image_name:tilt"
# For Kind, we can use localhost:5001 or just load directly
# Try to push to local registry, but don't fail if it doesn't exist
if docker push "${image_name}:tilt" 2>/dev/null; then
  echo "✅ Docker image pushed to registry: $image_name:tilt"
else
  echo "⚠️  Warning: Could not push to registry (registry may not be running)"
  echo "   Image tagged as: $image_name:tilt"
  echo "   Kind will load the image directly"
fi

echo "✅ Docker image built: $image_name:tilt"
