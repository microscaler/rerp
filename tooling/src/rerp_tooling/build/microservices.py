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

# Service (directory) name -> Cargo [package] name (impl crate binary).
# List of services for codegen comes from suite_sub_service_names; this mapping is
# maintained by bootstrap when adding a service. Teardown uses tilt_service_names (discovery).
PACKAGE_NAMES: Dict[str, str] = {
    "general-ledger": "rerp_accounting_general_ledger",
    "invoice": "rerp_accounting_invoice",
    "accounts-receivable": "rerp_accounting_accounts_receivable",
    "accounts-payable": "rerp_accounting_accounts_payable",
    "bank-sync": "rerp_accounting_bank_sync",
    "asset": "rerp_accounting_asset",
    "budget": "rerp_accounting_budget",
    "edi": "rerp_accounting_edi",
    "financial-reports": "rerp_accounting_financial_reports",
    "bff": "rerp_accounting_bff",
}


def run_accounting_gen_if_missing(project_root: Path) -> None:
    """Generate gen crates if they don't exist using BRRTRouter."""
    # Check for gen/Cargo.toml in the new structure
    probe = project_root / "microservices" / "accounting" / "general-ledger" / "gen" / "Cargo.toml"
    if probe.exists():
        return
    print("📦 microservices/accounting crates missing; running brrtrouter-gen for all accounting services...")
    
    from rerp_tooling.gen.brrtrouter import call_brrtrouter_generate
    from rerp_tooling.ci.fix_cargo_paths import run as run_fix_cargo_paths

    for name in suite_sub_service_names(project_root, "accounting"):
        spec = project_root / "openapi" / "accounting" / name / "openapi.yaml"
        if not spec.exists():
            continue
        # Output to gen/ subdirectory in the new structure
        out = project_root / "microservices" / "accounting" / name / "gen"
        out.mkdir(parents=True, exist_ok=True)
        
        # Check for dependencies config
        deps_config_path = spec.parent / "brrtrouter-dependencies.toml"
        deps_config = deps_config_path if deps_config_path.exists() else None
        
        result = call_brrtrouter_generate(
            spec_path=spec,
            output_dir=out,
            project_root=project_root,
            deps_config_path=deps_config,
            capture_output=True,
        )
        
        if result.returncode != 0:
            print(f"⚠️  Failed to generate {name}: {result.stderr}")
            continue
        
        # Fix Cargo.toml paths
        ct = out / "Cargo.toml"
        if ct.exists():
            run_fix_cargo_paths(ct, project_root)
    
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
