#!/usr/bin/env bash
set -euo pipefail

# Build all RERP accounting microservice containers (multi-arch: amd64, arm64, arm/v7) and push to GHCR (and optionally Docker Hub).
#
# Prerequisites for build-and-push (after --copy-only for each arch):
#   - build_artifacts/amd64/, build_artifacts/arm64/, build_artifacts/arm/ (arm=armv7) populated via --copy-only
#   - Docker buildx available
#   - Logged in: docker login ghcr.io (use GITHUB_TOKEN in CI)
#   - Optional: docker login for Docker Hub if DOCKERHUB_ORG is set
#
# Usage:
#   ./scripts/build-and-push-microservice-containers.sh [--copy-only=ARCH] <TAG> [extra_tag]
#
#   --copy-only=amd64|arm64|arm7  Copy binaries from microservices/target to build_artifacts/{arch} and exit. arm7 -> build_artifacts/arm/
#
# Examples:
#   ./scripts/build-and-push-microservice-containers.sh --copy-only=amd64
#   ./scripts/build-and-push-microservice-containers.sh sha-abc1234
#   ./scripts/build-and-push-microservice-containers.sh sha-abc1234 latest
#
# Env (optional):
#   GHCR_OWNER     - for ghcr.io/GHCR_OWNER/rerp-<service> (default: $GITHUB_REPOSITORY_OWNER)
#   DOCKERHUB_ORG  - if set, also push to docker.io/DOCKERHUB_ORG/rerp-<service>

# Service name -> (Cargo package name, artifact name for Dockerfile COPY)
# Must match Tiltfile PACKAGE_NAMES and BINARY_NAMES
declare -A PKG=(
  [general-ledger]=general_ledger
  [invoice]=invoice_management
  [accounts-receivable]=accounts_receivable
  [accounts-payable]=accounts_payable
  [bank-sync]=bank_synchronization
  [asset]=asset_management
  [budget]=budgeting
  [edi]=edi___compliance
  [financial-reports]=financial_reports
  [bff]=rerp_accounting_backend_for_frontend_api
)
declare -A BIN=(
  [general-ledger]=general_ledger
  [invoice]=invoice
  [accounts-receivable]=accounts_receivable
  [accounts-payable]=accounts_payable
  [bank-sync]=bank_sync
  [asset]=asset
  [budget]=budget
  [edi]=edi
  [financial-reports]=financial_reports
  [bff]=bff
)

SERVICES=(general-ledger invoice accounts-receivable accounts-payable bank-sync asset budget edi financial-reports bff)

arch_to_triple() { case "$1" in amd64) echo "x86_64-unknown-linux-musl" ;; arm64) echo "aarch64-unknown-linux-musl" ;; arm7) echo "armv7-unknown-linux-musleabihf" ;; *) echo "x86_64-unknown-linux-musl" ;; esac; }
arch_to_artifact_dir() { case "$1" in amd64) echo "amd64" ;; arm64) echo "arm64" ;; arm7) echo "arm" ;; *) echo "amd64" ;; esac; }

# --- --copy-only=ARCH: copy from microservices/target to build_artifacts/{arch} ---
if [[ "${1:-}" == --copy-only=* ]]; then
  COPY_ARCH="${1#--copy-only=}"
  TRIPLE=$(arch_to_triple "$COPY_ARCH")
  ARTIFACT_DIR=$(arch_to_artifact_dir "$COPY_ARCH")
  RELEASE_DIR="microservices/target/${TRIPLE}/release"
  mkdir -p "build_artifacts/${ARTIFACT_DIR}"
  for name in "${SERVICES[@]}"; do
    pkg="${PKG[$name]}"
    bin="${BIN[$name]}"
    src="${RELEASE_DIR}/${pkg}"
    dst="build_artifacts/${ARTIFACT_DIR}/${bin}"
    if [[ ! -f "$src" ]]; then
      echo "❌ Binary not found: $src (run: ./scripts/build-microservice.sh workspace --arch $COPY_ARCH --release)" >&2
      exit 1
    fi
    echo "📦 Copying $name: $src -> $dst"
    cp "$src" "$dst"
    chmod +x "$dst"
  done
  echo "✅ Copied to build_artifacts/${ARTIFACT_DIR}/"
  exit 0
fi

# --- Build and push (multi-arch) ---
if [[ $# -lt 1 ]]; then
  echo "usage: $0 [--copy-only=amd64|arm64|arm7] <TAG> [extra_tag]" >&2
  exit 1
fi
TAG="$1"
EXTRA_TAG="${2:-}"

GHCR_OWNER="${GHCR_OWNER:-${GITHUB_REPOSITORY_OWNER:-}}"
if [[ -z "$GHCR_OWNER" ]]; then
  echo "❌ GHCR_OWNER or GITHUB_REPOSITORY_OWNER must be set" >&2
  exit 1
fi

for d in build_artifacts/amd64 build_artifacts/arm64 build_artifacts/arm; do
  if [[ ! -d "$d" ]]; then
    echo "❌ $d not found. Run --copy-only=amd64, --copy-only=arm64, --copy-only=arm7 after building each arch." >&2
    exit 1
  fi
done

PLATFORMS="linux/amd64,linux/arm64,linux/arm/v7"

for name in "${SERVICES[@]}"; do
  bin="${BIN[$name]}"
  df="docker/microservices/Dockerfile.${name}"

  for d in build_artifacts/amd64 build_artifacts/arm64 build_artifacts/arm; do
    if [[ ! -f "${d}/${bin}" ]]; then
      echo "❌ ${d}/${bin} not found" >&2
      exit 1
    fi
  done
  if [[ ! -f "$df" ]]; then
    echo "❌ Dockerfile not found: $df" >&2
    exit 1
  fi

  ghcr_img="ghcr.io/${GHCR_OWNER}/rerp-${name}"
  tags=("${ghcr_img}:${TAG}")
  [[ -n "$EXTRA_TAG" ]] && tags+=("${ghcr_img}:${EXTRA_TAG}")

  if [[ -n "${DOCKERHUB_ORG:-}" ]]; then
    dh_img="docker.io/${DOCKERHUB_ORG}/rerp-${name}"
    tags+=("${dh_img}:${TAG}")
    [[ -n "$EXTRA_TAG" ]] && tags+=("${dh_img}:${EXTRA_TAG}")
  fi

  tag_args=()
  for t in "${tags[@]}"; do tag_args+=(-t "$t"); done

  echo "🔨 Building and pushing $name (${PLATFORMS}) (${tags[*]})..."
  docker buildx build \
    --platform "$PLATFORMS" \
    -f "$df" \
    --push \
    "${tag_args[@]}" \
    .

  echo "✅ Pushed $name (all archs)"
done

echo "🎉 All microservice images pushed (each service: amd64, arm64, arm/v7)."
