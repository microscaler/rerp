#!/usr/bin/env bash
set -euo pipefail

# Copy built microservice binary to build_artifacts directory
# This prepares binaries for Docker image building
#
# Usage:
#   scripts/copy-microservice-binary-simple.sh <source_path> <dest_path> <binary_name>
#
# Example:
#   scripts/copy-microservice-binary-simple.sh \
#     microservices/target/x86_64-unknown-linux-musl/debug/general_ledger \
#     build_artifacts/general_ledger \
#     general_ledger

if [[ $# -lt 3 ]]; then
  echo "usage: $0 <source_path> <dest_path> <binary_name>" >&2
  echo "  example: $0 microservices/target/.../general_ledger build_artifacts/general_ledger general_ledger" >&2
  exit 1
fi

source_path="$1"
dest_path="$2"
binary_name="$3"

# Create build_artifacts directory if it doesn't exist
mkdir -p "$(dirname "$dest_path")"

# Verify source binary exists
if [[ ! -f "$source_path" ]]; then
  echo "❌ Error: Binary not found: $source_path" >&2
  echo "   Build the service first" >&2
  exit 1
fi

# Copy binary
echo "📦 Copying binary: $source_path -> $dest_path"
cp "$source_path" "$dest_path"
chmod +x "$dest_path"

# Generate SHA256 hash for change detection
hash_file="${dest_path}.sha256"
sha256sum "$dest_path" | cut -d' ' -f1 > "$hash_file"

echo "✅ Binary copied and hash generated: $hash_file"
