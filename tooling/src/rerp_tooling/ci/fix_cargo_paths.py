"""Fix BRRTRouter path deps in Cargo.toml; RERP-specific: gen crate name and version."""

from __future__ import annotations

import re
from pathlib import Path

from brrtrouter_tooling.ci import fix_cargo_toml as brrt_fix_cargo_toml


def fix_cargo_toml(cargo_toml_path: Path, project_root: Path | None = None) -> bool:
    """Fix paths via brrtrouter_tooling, then apply RERP gen crate name/version."""
    changed = brrt_fix_cargo_toml(cargo_toml_path, project_root=project_root)

    if not cargo_toml_path.exists():
        return changed

    content = cargo_toml_path.read_text()
    original = content
    cargo_toml_dir = cargo_toml_path.parent.resolve()

    if cargo_toml_dir.name == "gen" and "accounting" in cargo_toml_dir.parts:
        service_name = cargo_toml_dir.parent.name
        service_snake = service_name.replace("-", "_")
        gen_crate_name = f"rerp_accounting_{service_snake}_gen"

        if f'name = "{gen_crate_name}"' not in content:
            content = re.sub(r'name = "[^"]+"', f'name = "{gen_crate_name}"', content, count=1)
        content = re.sub(r'version = "[^"]+"', 'version = "0.1.3"', content, count=1)
        if "[lib]" not in content:
            content = re.sub(
                r"(\[package\][^\[]+)",
                r'\1\n[lib]\nname = "' + gen_crate_name + '"\npath = "src/lib.rs"\n',
                content,
                count=1,
            )

    if content != original:
        cargo_toml_path.write_text(content)
        changed = True

    return changed


def run(cargo_toml_path: Path, project_root: Path | None = None) -> int:
    """Run fix for one Cargo.toml. Returns 0."""
    fix_cargo_toml(cargo_toml_path, project_root)
    return 0
