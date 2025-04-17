#!/usr/bin/env bash
set -euo pipefail

# Copy built microservice binary to build_artifacts directory
# This prepares binaries for Docker image building
#
# Usage:
#   scripts/copy-microservice-binary.sh <system> <module>
#
# Example:
#   scripts/copy-microservice-binary.sh auth idam

if [[ $# -lt 2 ]]; then
  echo "usage: $0 <system> <module>" >&2
  echo "  example: $0 auth idam" >&2
  exit 1
fi

system="$1"
module="$2"

# Convert module name to binary name format
# Binary names follow pattern: rerp_<system>_<module>_impl
binary_name="rerp_${system}_${module//-/_}_impl"

# Source binary path (from cargo build output)
source_binary="./components/target/x86_64-unknown-linux-musl/release/${binary_name}"

# Create build_artifacts directory if it doesn't exist
mkdir -p ./build_artifacts

# Destination path
dest_binary="./build_artifacts/${binary_name}"

# Verify source binary exists
if [[ ! -f "$source_binary" ]]; then
  echo "❌ Error: Binary not found: $source_binary" >&2
  echo "   Build the service first with: python3 scripts/host-aware-build.py ${system}_${module}" >&2
  exit 1
fi

# Copy binary
echo "📦 Copying binary: $source_binary -> $dest_binary"
cp "$source_binary" "$dest_binary"
chmod +x "$dest_binary"

# Generate SHA256 hash for change detection
hash_file="./build_artifacts/${binary_name}.sha256"
sha256sum "$dest_binary" | cut -d' ' -f1 > "$hash_file"

echo "✅ Binary copied and hash generated: $hash_file"
