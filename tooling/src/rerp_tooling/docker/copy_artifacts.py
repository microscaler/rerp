"""Copy microservice binaries to build_artifacts (delegate to brrtrouter_tooling; RERP-specific BINARY_NAMES)."""

from __future__ import annotations

from pathlib import Path

from rerp_tooling.build.microservices import PACKAGE_NAMES

# Service (directory) name -> binary name in build_artifacts (Dockerfile COPY expects this).
BINARY_NAMES: dict[str, str] = {
    "general-ledger": "general_ledger",
    "invoice": "invoice",
    "accounts-receivable": "accounts_receivable",
    "accounts-payable": "accounts_payable",
    "bank-sync": "bank_sync",
    "asset": "asset",
    "budget": "budget",
    "edi": "edi",
    "financial-reports": "financial_reports",
    "bff": "bff",
}


def run(arch: str, project_root: Path) -> int:
    """Copy from microservices/target to build_artifacts (via brrtrouter_tooling). Returns 0 or 1."""
    from brrtrouter_tooling.docker.copy_artifacts import run as run_brt

    return run_brt(arch, project_root, package_names=PACKAGE_NAMES, binary_names=BINARY_NAMES)


def validate_build_artifacts(project_root: Path) -> int:
    """Validate build_artifacts (via brrtrouter_tooling). Returns 0 or 1."""
    from brrtrouter_tooling.docker.copy_artifacts import validate_build_artifacts as validate_brt

    return validate_brt(project_root, BINARY_NAMES)
