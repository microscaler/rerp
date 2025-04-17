#!/usr/bin/env bash
set -euo pipefail

# Build and push Docker image for a microservice
#
# Usage: build-microservice-docker.sh <system> <module> <image_name> [port]
#
# Example:
#   build-microservice-docker.sh \
#     auth \
#     idam \
#     localhost:5001/rerp-auth-idam \
#     8000

if [[ $# -lt 3 ]]; then
  echo "usage: $0 <system> <module> <image_name> [port]" >&2
  echo "  example: $0 auth idam localhost:5001/rerp-auth-idam 8000" >&2
  exit 1
fi

system="$1"
module="$2"
image_name="$3"
port="${4:-8000}"

# Convert module name to binary name format
binary_name="rerp_${system}_${module//-/_}_impl"
hash_path="./build_artifacts/${binary_name}.sha256"
artifact_path="./build_artifacts/${binary_name}"

# Verify hash file exists (indicates copy completed)
if [[ ! -f "$hash_path" ]]; then
  echo "❌ Error: Hash file not found: $hash_path" >&2
  echo "   This indicates copy-microservice-binary.sh has not completed yet" >&2
  exit 1
fi

# Verify artifact exists
if [[ ! -f "$artifact_path" ]]; then
  echo "❌ Error: Artifact not found: $artifact_path" >&2
  exit 1
fi

# Generate Dockerfile from template if it doesn't exist
dockerfile="./docker/microservices/Dockerfile.${system}_${module}"
if [[ ! -f "$dockerfile" ]]; then
  echo "📝 Generating Dockerfile from template..."
  # Use sed to replace template variables
  sed -e "s/{{service_name}}/${system}-${module}/g" \
      -e "s/{{binary_name}}/${binary_name}/g" \
      -e "s/{{system}}/${system}/g" \
      -e "s/{{module}}/${module}/g" \
      -e "s/{{port}}/${port}/g" \
      ./docker/microservices/Dockerfile.template > "$dockerfile"
  echo "✅ Generated: $dockerfile"
fi

# Build base image first if it doesn't exist
if ! docker images | grep -q "rerp/base"; then
  echo "🔨 Building base image: rerp/base:latest"
  docker build -t rerp/base:latest -f ./docker/base/Dockerfile .
fi

# Build Docker image
echo "🔨 Building Docker image: $image_name:tilt"
if ! docker build -t "${image_name}:tilt" --rm --force-rm -f "$dockerfile" .; then
  echo "❌ Error: Docker build failed" >&2
  exit 1
fi

# Push Docker image
echo "📤 Pushing Docker image: $image_name:tilt"
if ! docker push "${image_name}:tilt"; then
  echo "❌ Error: Docker push failed" >&2
  exit 1
fi

echo "✅ Docker image built and pushed: $image_name:tilt"
