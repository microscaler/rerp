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

# Push to local registry, or load into Kind if registry is not running (e.g. dev-up without kind-registry)
echo "📤 Pushing or loading image: $image_name:tilt"
if docker push "${image_name}:tilt" 2>/dev/null; then
  echo "✅ Docker image pushed to registry: $image_name:tilt"
else
  echo "⚠️  Registry not available at localhost:5001; loading into Kind cluster..."
  if kind load docker-image "${image_name}:tilt" --name rerp 2>/dev/null; then
    echo "✅ Image loaded into Kind: $image_name:tilt"
  else
    echo "⚠️  Could not push or kind load; image tagged as: $image_name:tilt"
  fi
fi

echo "✅ Docker image ready: $image_name:tilt"
