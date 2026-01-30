"""Fix BRRTRouter path deps in Cargo.toml; RERP gen crate name/version via brrtrouter_tooling."""

from __future__ import annotations

from pathlib import Path

from brrtrouter_tooling.ci import fix_cargo_toml as brrt_fix_cargo_toml
from brrtrouter_tooling.ci.fix_cargo_paths import run as brrt_run

# RERP gen crate naming: rerp_{suite}_{service}_gen, version 0.1.3
RERP_GEN_CRATE_CONFIG: tuple[str, str] = ("rerp_{suite}_{service}_gen", "0.1.3")


def fix_cargo_toml(cargo_toml_path: Path, project_root: Path | None = None) -> bool:
    """Fix paths and RERP gen crate name/version via brrtrouter_tooling."""
    return brrt_fix_cargo_toml(
        cargo_toml_path,
        project_root=project_root,
        gen_crate_config=RERP_GEN_CRATE_CONFIG,
    )


def run(cargo_toml_path: Path, project_root: Path | None = None) -> int:
    """Run fix for one Cargo.toml. Returns 0."""
    brrt_run(
        cargo_toml_path,
        project_root=project_root,
        gen_crate_config=RERP_GEN_CRATE_CONFIG,
    )
    return 0
