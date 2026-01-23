#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   scripts/build-microservice.sh <service-name|workspace> [extra args...]
#
# Selects the correct build strategy based on host OS/arch:
# - macOS: cargo zigbuild --target x86_64-unknown-linux-musl
# - Linux x86_64: cargo build --target x86_64-unknown-linux-musl with musl-gcc linker
#
# Service names: general-ledger, invoice, accounts-receivable, accounts-payable, bank-sync, asset, budget
# Special: "workspace" builds all microservices at once

if [[ $# -lt 1 ]]; then
  echo "usage: $0 <service-name|workspace> [extra cargo args...]" >&2
  echo "  service-name: general-ledger, invoice, accounts-receivable, accounts-payable, bank-sync, asset, budget" >&2
  echo "  workspace: build all microservices at once" >&2
  exit 2
fi

target=${1}
shift || true

os_name=$(uname -s || echo unknown)
arch=$(uname -m || echo unknown)

use_zigbuild=true
if [[ ${os_name} == Linux && ${arch} == x86_64 ]]; then
  use_zigbuild=false
fi

# Check if building entire workspace
if [[ "${target}" == "workspace" ]]; then
  if [[ ! -f "./microservices/Cargo.toml" ]]; then
    echo "error: Cargo.toml not found in microservices directory" >&2
    exit 4
  fi
  
  # Build all workspace members from microservices directory
  # Using debug builds for active development (faster compilation, better debugging)
  cd ./microservices || exit 1
  if [[ ${use_zigbuild} == true ]]; then
    exec cargo zigbuild --target x86_64-unknown-linux-musl --workspace "$@"
  else
    exec env CC_x86_64_unknown_linux_musl=musl-gcc \
      CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc \
      cargo build --target x86_64-unknown-linux-musl --workspace "$@"
  fi
fi

# Build individual service
# Map service names to crate directories and package names
declare -A PACKAGE_NAMES=(
  ["general-ledger"]="general_ledger"
  ["invoice"]="invoice"
  ["accounts-receivable"]="accounts_receivable"
  ["accounts-payable"]="accounts_payable"
  ["bank-sync"]="bank_sync"
  ["asset"]="asset"
  ["budget"]="budget"
)

package_name=${PACKAGE_NAMES[$target]:-}

if [[ -z "$package_name" ]]; then
  echo "unknown service: ${target}" >&2
  echo "valid services: general-ledger, invoice, accounts-receivable, accounts-payable, bank-sync, asset, budget, workspace" >&2
  exit 3
fi

# Build the service binary using workspace (from microservices directory)
# The workspace Cargo.toml allows crates to reference themselves by name
# Using debug builds for active development (faster compilation, better debugging)
cd ./microservices || exit 1
if [[ ${use_zigbuild} == true ]]; then
  exec cargo zigbuild --target x86_64-unknown-linux-musl -p "$package_name" "$@"
else
  exec env CC_x86_64_unknown_linux_musl=musl-gcc \
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc \
    cargo build --target x86_64-unknown-linux-musl -p "$package_name" "$@"
fi

