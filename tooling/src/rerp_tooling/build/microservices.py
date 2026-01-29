"""Build microservices/accounting workspace (delegate to brrtrouter_tooling; RERP package list + gen)."""

from __future__ import annotations

import sys
from pathlib import Path
from typing import Dict

from brrtrouter_tooling.build import (
    build_package_with_options,
    build_workspace_with_options,
)
from rerp_tooling.discovery import suite_sub_service_names

# Service (directory) name -> Cargo [package] name (impl crate). RERP-specific.
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
    """Generate gen crates if they don't exist using BRRTRouter (RERP paths)."""
    probe = project_root / "microservices" / "accounting" / "general-ledger" / "gen" / "Cargo.toml"
    if probe.exists():
        return
    print(
        "📦 microservices/accounting crates missing; running brrtrouter-gen for all accounting services...",
        file=sys.stderr,
    )
    from brrtrouter_tooling.gen import call_brrtrouter_generate
    from brrtrouter_tooling.ci import run_fix_cargo_paths

    for name in suite_sub_service_names(project_root, "accounting"):
        spec = project_root / "openapi" / "accounting" / name / "openapi.yaml"
        if not spec.exists():
            continue
        out = project_root / "microservices" / "accounting" / name / "gen"
        out.mkdir(parents=True, exist_ok=True)
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
            err = result.stderr or ""
            print(f"⚠️  Failed to generate {name}: {err}", file=sys.stderr)
            continue
        ct = out / "Cargo.toml"
        if ct.exists():
            run_fix_cargo_paths(ct, project_root)
    print("✅ accounting codegen complete")


def build_microservices_workspace(project_root: Path, arch: str, release: bool) -> int:
    """Build microservices/ workspace. arch: amd64|arm64|arm7. Returns 0/1."""
    return build_workspace_with_options(
        project_root,
        workspace_dir="microservices",
        arch=arch,
        release=release,
        gen_if_missing_callback=run_accounting_gen_if_missing,
    )


def build_microservice(project_root: Path, name: str, release: bool) -> int:
    """Build one accounting microservice. name e.g. general-ledger. Returns 0/1."""
    pkg = PACKAGE_NAMES.get(name)
    if not pkg:
        print(
            f"❌ unknown service: {name}. Valid: {', '.join(PACKAGE_NAMES)}",
            file=sys.stderr,
        )
        return 1
    return build_package_with_options(
        project_root,
        workspace_dir="microservices",
        package_name=pkg,
        arch="amd64",
        release=release,
        gen_if_missing_callback=run_accounting_gen_if_missing,
    )
