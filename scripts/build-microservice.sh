#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   scripts/build-microservice.sh <service-name|workspace> [extra args...]
#
# Selects the correct build strategy based on host OS/arch:
# - macOS: cargo zigbuild --target x86_64-unknown-linux-musl
# - Linux x86_64: cargo build --target x86_64-unknown-linux-musl with musl-gcc linker
#
# Service names: general-ledger, invoice, accounts-receivable, accounts-payable, bank-sync, asset, budget, edi, financial-reports, bff
# Special: "workspace" builds all microservices at once
#
# If microservices/accounting crates are missing, runs brrtrouter-gen for all accounting
# services first so the workspace can load (Cargo requires all members to exist).

ACCOUNTING_SERVICES=(general-ledger invoice accounts-receivable accounts-payable bank-sync asset budget edi financial-reports)

run_accounting_gen_if_missing() {
  # Workspace requires all accounting members to exist; check one
  if [[ ! -f "./microservices/accounting/general-ledger/Cargo.toml" ]]; then
    echo "📦 microservices/accounting crates missing; running brrtrouter-gen for all accounting services..."
    BRRT=${BRRTRouter:-../BRRTRouter}/target/debug/brrtrouter-gen
    if [[ ! -x "$BRRT" ]]; then
      BRRT="cargo run --manifest-path ../BRRTRouter/Cargo.toml --bin brrtrouter-gen --"
    fi
    for name in "${ACCOUNTING_SERVICES[@]}"; do
      echo "  generating $name..."
      $BRRT generate \
        --spec "./openapi/accounting/${name}/openapi.yaml" \
        --output "./microservices/accounting/${name}" \
        --force
      if [[ -f "./microservices/accounting/${name}/Cargo.toml" ]]; then
        python3 ./scripts/fix_cargo_toml_paths.py "./microservices/accounting/${name}/Cargo.toml"
      fi
    done
    echo "✅ accounting codegen complete"
  fi
}

if [[ $# -lt 1 ]]; then
  echo "usage: $0 <service-name|workspace> [extra cargo args...]" >&2
  echo "  service-name: general-ledger, invoice, accounts-receivable, accounts-payable, bank-sync, asset, budget, edi, financial-reports, bff" >&2
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
  run_accounting_gen_if_missing

  # Parse --arch from remaining args (amd64, arm64, arm7). arch=arm7 uses cross to armv7.
  ARCH=amd64
  CARGO_ARGS=()
  while [[ $# -gt 0 ]]; do
    if [[ "$1" == "--arch" && -n "${2:-}" ]]; then
      ARCH="$2"
      shift 2
      continue
    fi
    CARGO_ARGS+=("$1")
    shift
  done

  cd ./microservices || exit 1
  case "$ARCH" in
    arm64)
      exec cross build --target aarch64-unknown-linux-musl --workspace "${CARGO_ARGS[@]}"
      ;;
    arm7)
      exec cross build --target armv7-unknown-linux-musleabihf --workspace "${CARGO_ARGS[@]}"
      ;;
    amd64|*)
      if [[ ${use_zigbuild} == true ]]; then
        exec cargo zigbuild --target x86_64-unknown-linux-musl --workspace "${CARGO_ARGS[@]}"
      else
        exec env CC_x86_64_unknown_linux_musl=musl-gcc \
          CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc \
          cargo build --target x86_64-unknown-linux-musl --workspace "${CARGO_ARGS[@]}"
      fi
      ;;
  esac
fi

# Build individual service
# Map service (directory) names to Cargo [package] names from brrtrouter-gen output.
# Using case for portability (bash 3.2 on macOS does not support declare -A).
case "$target" in
  general-ledger)   package_name=general_ledger ;;
  invoice)          package_name=invoice_management ;;
  accounts-receivable) package_name=accounts_receivable ;;
  accounts-payable) package_name=accounts_payable ;;
  bank-sync)        package_name=bank_synchronization ;;
  asset)            package_name=asset_management ;;
  budget)           package_name=budgeting ;;
  edi)              package_name=edi___compliance ;;
  financial-reports) package_name=financial_reports ;;
  bff)              package_name=rerp_accounting_backend_for_frontend_api ;;
  *)
    echo "unknown service: ${target}" >&2
    echo "valid services: general-ledger, invoice, accounts-receivable, accounts-payable, bank-sync, asset, budget, edi, financial-reports, bff, workspace" >&2
    exit 3
    ;;
esac

run_accounting_gen_if_missing

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

