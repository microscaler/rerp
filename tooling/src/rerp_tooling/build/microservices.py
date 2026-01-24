"""Build microservices/accounting workspace (brrtrouter-generated). Replaces build-microservice.sh."""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path
from typing import Dict

from rerp_tooling.discovery import suite_sub_service_names

from .host_aware import (
    ARCH_TARGETS,
    _get_cargo_env,
    should_use_cross,
    should_use_zigbuild,
)

# Service (directory) name -> Cargo [package] name (brrtrouter-gen output).
# List of services for codegen comes from suite_sub_service_names; this mapping is
# maintained by bootstrap when adding a service. Teardown uses tilt_service_names (discovery).
PACKAGE_NAMES: Dict[str, str] = {
    "general-ledger": "general_ledger",
    "invoice": "invoice_management",
    "accounts-receivable": "accounts_receivable",
    "accounts-payable": "accounts_payable",
    "bank-sync": "bank_synchronization",
    "asset": "asset_management",
    "budget": "budgeting",
    "edi": "edi___compliance",
    "financial-reports": "financial_reports",
    "bff": "rerp_accounting_backend_for_frontend_api",
}


def run_accounting_gen_if_missing(project_root: Path) -> None:
    probe = project_root / "microservices" / "accounting" / "general-ledger" / "Cargo.toml"
    if probe.exists():
        return
    print("📦 microservices/accounting crates missing; running brrtrouter-gen for all accounting services...")
    brrt = project_root.parent / "BRRTRouter" / "target" / "debug" / "brrtrouter-gen"
    brrt_manifest = project_root.parent / "BRRTRouter" / "Cargo.toml"
    if not brrt_manifest.exists():
        raise FileNotFoundError(f"BRRTRouter not found at {brrt_manifest.parent}")

    for name in suite_sub_service_names(project_root, "accounting"):
        spec = project_root / "openapi" / "accounting" / name / "openapi.yaml"
        if not spec.exists():
            continue
        out = project_root / "microservices" / "accounting" / name
        out.mkdir(parents=True, exist_ok=True)
        cmd = [str(brrt), "generate", "--spec", str(spec), "--output", str(out), "--force"] if brrt.exists() else [
            "cargo", "run", "--manifest-path", str(brrt_manifest), "--bin", "brrtrouter-gen", "--",
            "generate", "--spec", str(spec), "--output", str(out), "--force",
        ]
        subprocess.run(cmd, check=True, cwd=str(project_root), capture_output=True, text=True)
        ct = out / "Cargo.toml"
        if ct.exists():
            from rerp_tooling.ci.fix_cargo_paths import run as run_fix
            run_fix(ct, project_root)
    print("✅ accounting codegen complete")


def build_microservices_workspace(project_root: Path, arch: str, release: bool) -> int:
    """Build microservices/ workspace. arch: amd64|arm64|arm7. Returns 0/1."""
    ms = project_root / "microservices"
    manifest = ms / "Cargo.toml"
    if not manifest.exists():
        print(f"❌ {manifest} not found", file=sys.stderr)
        return 1
    run_accounting_gen_if_missing(project_root)

    rust_target = ARCH_TARGETS.get(arch, ARCH_TARGETS["amd64"])
    use_cross = should_use_cross()
    use_zigbuild = should_use_zigbuild()
    rel = ["--release"] if release else []
    # Disable jemalloc on armv7: tikv-jemalloc-sys needs __ffsdi2 (compiler-rt), which
    # is not available when cross-linking for armv7-unknown-linux-musleabihf with musl.
    no_jemalloc = rust_target == "armv7-unknown-linux-musleabihf"
    base = ["--no-default-features"] if no_jemalloc else []

    if use_cross:
        cmd = ["cross", "build", "--manifest-path", str(manifest), "--target", rust_target, "--workspace"] + base + rel
        subprocess.run(cmd, check=True, cwd=str(project_root))
        return 0

    if use_zigbuild:
        cmd = ["cargo", "zigbuild", "--manifest-path", str(manifest), "--target", rust_target, "--workspace"] + base + rel
        subprocess.run(cmd, check=True, cwd=str(project_root))
    else:
        cmd = ["cargo", "build", "--manifest-path", str(manifest), "--target", rust_target, "--workspace"] + base + rel
        env = {**os.environ, **_get_cargo_env(rust_target)}
        subprocess.run(cmd, check=True, cwd=str(project_root), env=env)
    return 0


def build_microservice(project_root: Path, name: str, release: bool) -> int:
    """Build one accounting microservice. name e.g. general-ledger. Returns 0/1."""
    pkg = PACKAGE_NAMES.get(name)
    if not pkg:
        print(f"❌ unknown service: {name}. Valid: {', '.join(PACKAGE_NAMES)}", file=sys.stderr)
        return 1
    run_accounting_gen_if_missing(project_root)

    ms = project_root / "microservices"
    manifest = ms / "Cargo.toml"
    if not manifest.exists():
        print(f"❌ {manifest} not found", file=sys.stderr)
        return 1

    target = "x86_64-unknown-linux-musl"
    use_zigbuild = should_use_zigbuild()
    rel = ["--release"] if release else []

    if use_zigbuild:
        cmd = ["cargo", "zigbuild", "--manifest-path", str(manifest), "--target", target, "-p", pkg] + rel
        subprocess.run(cmd, check=True, cwd=str(project_root))
    else:
        cmd = ["cargo", "build", "--manifest-path", str(manifest), "--target", target, "-p", pkg] + rel
        env = {**os.environ, **_get_cargo_env(target)}
        subprocess.run(cmd, check=True, cwd=str(project_root), env=env)
    return 0
