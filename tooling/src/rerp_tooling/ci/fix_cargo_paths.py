"""Fix BRRTRouter path dependencies in generated Cargo.toml files.

Updates brrtrouter and brrtrouter_macros path deps to point at the BRRTRouter
repository (sibling of project root: ../BRRTRouter). For use in local dev after
brrtrouter-gen; CI uses rerp ci patch-brrtrouter to switch to git deps instead.
"""

from __future__ import annotations

import os
import re
from pathlib import Path
from typing import Optional


def fix_cargo_toml(cargo_toml_path: Path, project_root: Optional[Path] = None) -> bool:
    """
    Fix BRRTRouter paths in a Cargo.toml file.
    For gen/ Cargo.toml files, also updates package name and version to match workspace.
    Assumes Cargo.toml at microservices/accounting/{service}/gen/Cargo.toml or similar;
    project_root defaults to 3 levels up from the file's parent.
    BRRTRouter is project_root.parent / "BRRTRouter".
    Returns True if content was changed.
    """
    if not cargo_toml_path.exists():
        print(f"Warning: {cargo_toml_path} does not exist, skipping")
        return False

    content = cargo_toml_path.read_text()
    original = content

    cargo_toml_dir = cargo_toml_path.parent.resolve()
    if project_root is not None:
        root = Path(project_root).resolve()
    else:
        # microservices/accounting/{service}/gen/ -> gen -> service -> accounting -> microservices -> repo root
        # or microservices/accounting/{service}/ -> service -> accounting -> microservices -> repo root
        if cargo_toml_dir.name == "gen":
            root = cargo_toml_dir.parent.parent.parent.parent
        else:
            root = cargo_toml_dir.parent.parent.parent

    brrtrouter_path = root.parent / "BRRTRouter"
    try:
        rel = Path(os.path.relpath(brrtrouter_path, cargo_toml_dir)).as_posix()
        rel_macros = Path(
            os.path.relpath(brrtrouter_path / "brrtrouter_macros", cargo_toml_dir)
        ).as_posix()
    except ValueError:
        rel = str(brrtrouter_path)
        rel_macros = str(brrtrouter_path / "brrtrouter_macros")

    content = re.sub(
        r'brrtrouter = \{ path = "[^"]+" \}',
        f'brrtrouter = {{ path = "{rel}" }}',
        content,
    )
    content = re.sub(
        r'brrtrouter_macros = \{ path = "[^"]+" \}',
        f'brrtrouter_macros = {{ path = "{rel_macros}" }}',
        content,
    )

    # If this is a gen/ Cargo.toml, also update package name and version
    if cargo_toml_dir.name == "gen" and "accounting" in cargo_toml_dir.parts:
        # Extract service name from path: microservices/accounting/{service}/gen/
        service_name = cargo_toml_dir.parent.name
        service_snake = service_name.replace("-", "_")
        gen_crate_name = f"rerp_accounting_{service_snake}_gen"

        # Update package name if it doesn't match
        if f'name = "{gen_crate_name}"' not in content:
            content = re.sub(r'name = "[^"]+"', f'name = "{gen_crate_name}"', content, count=1)

        # Update version to match workspace (0.1.3)
        content = re.sub(r'version = "[^"]+"', 'version = "0.1.3"', content, count=1)

        # Add [lib] section if not present
        if "[lib]" not in content:
            content = re.sub(
                r"(\[package\][^\[]+)",
                r'\1\n[lib]\nname = "' + gen_crate_name + '"\npath = "src/lib.rs"\n',
                content,
                count=1,
            )

    if content != original:
        cargo_toml_path.write_text(content)
        print(f"✅ Fixed paths in {cargo_toml_path}")
        return True
    print(f"Info:  No changes needed in {cargo_toml_path}")
    return False


def run(cargo_toml_path: Path, project_root: Optional[Path] = None) -> int:
    """Run fix for one Cargo.toml. Returns 0."""
    fix_cargo_toml(cargo_toml_path, project_root)
    return 0
