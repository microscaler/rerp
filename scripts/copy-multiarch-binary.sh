#!/usr/bin/env bash
set -euo pipefail

# Copy built microservice binaries for all architectures to build_artifacts
#
# Usage:
#   scripts/copy-multiarch-binary.sh <system> <module> [architecture]
#
# Example:
#   scripts/copy-multiarch-binary.sh auth idam
#   scripts/copy-multiarch-binary.sh auth idam amd64

if [[ $# -lt 2 ]]; then
  echo "usage: $0 <system> <module> [architecture]" >&2
  echo "  example: $0 auth idam" >&2
  echo "  example: $0 auth idam amd64" >&2
  echo "  architecture: amd64, arm64, arm7, or all (default: all)" >&2
  exit 1
fi

system="$1"
module="$2"
requested_arch="${3:-all}"

# Convert module name to binary name format
binary_name="rerp_${system}_${module//-/_}_impl"

# Architecture definitions
declare -A ARCH_TARGETS=(
  ["amd64"]="x86_64-unknown-linux-musl"
  ["arm64"]="aarch64-unknown-linux-musl"
  ["arm7"]="armv7-unknown-linux-musleabihf"
)

# Determine which architectures to copy
copy_archs=()
if [[ "$requested_arch" == "all" ]]; then
  copy_archs=("amd64" "arm64" "arm7")
else
  copy_archs=("$requested_arch")
fi

# Create build_artifacts directory structure
mkdir -p "./build_artifacts/${system}_${module}"

# Copy binaries for each architecture
for arch in "${copy_archs[@]}"; do
  rust_target="${ARCH_TARGETS[$arch]}"
  source_binary="./components/target/${rust_target}/release/${binary_name}"
  dest_dir="./build_artifacts/${system}_${module}/${arch}"
  
  # Verify source binary exists
  if [[ ! -f "$source_binary" ]]; then
    echo "❌ Error: Binary not found: $source_binary" >&2
    echo "   Build the service first with: python3 scripts/host-aware-build.py ${system}_${module} ${arch}" >&2
    continue
  fi
  
  # Copy binary
  mkdir -p "$dest_dir"
  echo "📦 Copying ${arch} binary: $source_binary -> $dest_dir/${binary_name}"
  cp "$source_binary" "$dest_dir/${binary_name}"
  chmod +x "$dest_dir/${binary_name}"
  
  # Generate SHA256 hash for change detection
  hash_file="$dest_dir/${binary_name}.sha256"
  sha256sum "$dest_dir/${binary_name}" | cut -d' ' -f1 > "$hash_file"
  
  echo "✅ ${arch} binary copied and hash generated: $hash_file"
done

echo "🎉 All binaries copied!"
