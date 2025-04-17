#!/usr/bin/env bash
set -euo pipefail

# Build and push multi-architecture Docker images for a microservice
#
# Usage: build-multiarch-docker.sh <system> <module> <image_name> [tag] [push]
#
# Example:
#   build-multiarch-docker.sh auth idam rerp/auth-idam latest push
#
# This script:
# 1. Builds binaries for all architectures (amd64, arm64, arm7)
# 2. Creates Docker images for each architecture
# 3. Creates a multi-arch manifest
# 4. Optionally pushes to Docker Hub

if [[ $# -lt 3 ]]; then
  echo "usage: $0 <system> <module> <image_name> [tag] [push]" >&2
  echo "  example: $0 auth idam rerp/auth-idam latest push" >&2
  echo "  tag: Docker image tag (default: latest)" >&2
  echo "  push: 'push' to push to registry, omit to build only" >&2
  exit 1
fi

system="$1"
module="$2"
image_name="$3"
tag="${4:-latest}"
should_push="${5:-}"

# Convert module name to binary name
binary_name="rerp_${system}_${module//-/_}_impl"

# Architecture definitions
declare -A ARCH_TARGETS=(
  ["amd64"]="x86_64-unknown-linux-musl"
  ["arm64"]="aarch64-unknown-linux-musl"
  ["arm7"]="armv7-unknown-linux-musleabihf"
)

declare -A ARCH_PLATFORMS=(
  ["amd64"]="linux/amd64"
  ["arm64"]="linux/arm64"
  ["arm7"]="linux/arm/v7"
)

# Build binaries for all architectures
echo "🔨 Building binaries for all architectures..."
python3 ./scripts/host-aware-build.py "${system}_${module}" all

# Create build_artifacts directory structure
mkdir -p "./build_artifacts/${system}_${module}"

# Copy binaries to architecture-specific directories
for arch in amd64 arm64 arm7; do
  rust_target="${ARCH_TARGETS[$arch]}"
  source_binary="./components/target/${rust_target}/release/${binary_name}"
  dest_dir="./build_artifacts/${system}_${module}/${arch}"
  
  if [[ ! -f "$source_binary" ]]; then
    echo "❌ Error: Binary not found: $source_binary" >&2
    exit 1
  fi
  
  mkdir -p "$dest_dir"
  cp "$source_binary" "$dest_dir/${binary_name}"
  chmod +x "$dest_dir/${binary_name}"
  
  # Generate SHA256 hash
  sha256sum "$dest_dir/${binary_name}" | cut -d' ' -f1 > "$dest_dir/${binary_name}.sha256"
  echo "✅ Copied ${arch} binary"
done

# Generate Dockerfile if it doesn't exist
dockerfile="./docker/microservices/Dockerfile.${system}_${module}"
if [[ ! -f "$dockerfile" ]]; then
  echo "📝 Generating Dockerfile..."
  python3 ./scripts/generate-dockerfile.py "$system" "$module" 8000
fi

# Build base image for each architecture if needed
echo "🔨 Building base images for all architectures..."
for arch in amd64 arm64 arm7; do
  platform="${ARCH_PLATFORMS[$arch]}"
  base_image="rerp/base:${arch}"
  
  if ! docker images | grep -q "rerp/base.*${arch}"; then
    echo "  Building base image for ${arch}..."
    docker buildx build \
      --platform "$platform" \
      --tag "$base_image" \
      --load \
      -f ./docker/base/Dockerfile .
  fi
done

# Build Docker images for each architecture
echo "🔨 Building Docker images for all architectures..."
image_tags=()
for arch in amd64 arm64 arm7; do
  platform="${ARCH_PLATFORMS[$arch]}"
  arch_tag="${image_name}:${tag}-${arch}"
  image_tags+=("$arch_tag")
  
  echo "  Building ${arch} image: $arch_tag"
  
  # Create architecture-specific Dockerfile
  arch_dockerfile="${dockerfile}.${arch}"
  sed -e "s|FROM rerp/base:latest|FROM rerp/base:${arch}|g" \
      -e "s|COPY ./build_artifacts/${binary_name}|COPY ./build_artifacts/${system}_${module}/${arch}/${binary_name}|g" \
      "$dockerfile" > "$arch_dockerfile"
  
  docker buildx build \
    --platform "$platform" \
    --tag "$arch_tag" \
    --file "$arch_dockerfile" \
    --load \
    .
  
  echo "  ✅ Built: $arch_tag"
done

# Create multi-architecture manifest
echo "🔗 Creating multi-architecture manifest..."
manifest_tag="${image_name}:${tag}"
docker manifest create "$manifest_tag" "${image_tags[@]}"

# Annotate each architecture
for arch in amd64 arm64 arm7; do
  platform="${ARCH_PLATFORMS[$arch]}"
  arch_tag="${image_name}:${tag}-${arch}"
  docker manifest annotate \
    --arch "$(echo $platform | cut -d'/' -f2)" \
    --os "$(echo $platform | cut -d'/' -f1)" \
    "$manifest_tag" \
    "$arch_tag"
done

echo "✅ Multi-architecture manifest created: $manifest_tag"

# Push to Docker Hub if requested
if [[ "$should_push" == "push" ]]; then
  echo "📤 Pushing images to Docker Hub..."
  
  # Push individual architecture images
  for arch_tag in "${image_tags[@]}"; do
    echo "  Pushing: $arch_tag"
    docker push "$arch_tag"
  done
  
  # Push manifest
  echo "  Pushing manifest: $manifest_tag"
  docker manifest push "$manifest_tag"
  
  echo "✅ All images pushed to Docker Hub"
else
  echo "ℹ️  Images built locally. Use 'push' argument to push to Docker Hub."
fi

echo "🎉 Multi-architecture build complete!"
