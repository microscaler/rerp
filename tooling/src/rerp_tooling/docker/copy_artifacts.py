"""Copy microservice binaries from microservices/target/{triple}/release to build_artifacts/{arch}. Replaces --copy-only of build-and-push-microservice-containers.sh."""

from __future__ import annotations

import shutil
import sys
from pathlib import Path

from rerp_tooling.build.microservices import PACKAGE_NAMES

# arch -> artifact dir for build_artifacts (arm7 -> arm for TARGETARCH=arm in Dockerfiles)
ARCH_TO_ARTIFACT_DIR: dict[str, str] = {
    "amd64": "amd64",
    "arm64": "arm64",
    "arm7": "arm",
}

# Service (directory) name -> binary name in build_artifacts (Dockerfile COPY expects this).
# Service list is PACKAGE_NAMES.keys(); both mappings are maintained by bootstrap.
# Teardown uses tilt_service_names (discovery) so no list to update there.
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

# Reuse triple from host_aware (import here to avoid circular import)
from rerp_tooling.build.host_aware import ARCH_TARGETS  # noqa: E402


def run(arch: str, project_root: Path) -> int:
    """Copy from microservices/target/{triple}/release/{pkg} to build_artifacts/{artifact_dir}/{bin}. Returns 0 or 1."""
    if arch not in ARCH_TARGETS:
        print(f"❌ Unknown arch: {arch}. Use amd64, arm64, or arm7.", file=sys.stderr)
        return 1
    triple = ARCH_TARGETS[arch]
    artifact_dir = ARCH_TO_ARTIFACT_DIR.get(arch, "amd64")
    release_dir = project_root / "microservices" / "target" / triple / "release"
    out_dir = project_root / "build_artifacts" / artifact_dir
    out_dir.mkdir(parents=True, exist_ok=True)
    for name, pkg in PACKAGE_NAMES.items():
        bin_name = BINARY_NAMES.get(name, pkg)
        src = release_dir / pkg
        dst = out_dir / bin_name
        if not src.exists():
            print(
                f"❌ Binary not found: {src} (run: tooling/.venv/bin/rerp build microservices {arch} --release)",
                file=sys.stderr,
            )
            return 1
        shutil.copy2(src, dst)
        dst.chmod(0o755)
        print(f"📦 Copying {name}: {src.name} -> {dst.relative_to(project_root)}")
    print(f"✅ Copied to build_artifacts/{artifact_dir}/")
    return 0
