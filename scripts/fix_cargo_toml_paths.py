#!/usr/bin/env python3
"""Fix BRRTRouter dependency paths in generated Cargo.toml files.

This script updates the brrtrouter and brrtrouter_macros path dependencies
to point to the correct BRRTRouter repository location.
"""

import re
import sys
import os
from pathlib import Path


def fix_cargo_toml(cargo_toml_path: Path) -> None:
    """Fix BRRTRouter paths in a Cargo.toml file."""
    if not cargo_toml_path.exists():
        print(f"Warning: {cargo_toml_path} does not exist, skipping")
        return
    
    with open(cargo_toml_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Calculate relative path from the Cargo.toml location to BRRTRouter
    # The Cargo.toml is in microservices/accounting/{service}/Cargo.toml
    # BRRTRouter is at ../BRRTRouter relative to RERP root
    # From service crate (microservices/accounting/{service}/), BRRTRouter is at ../../../BRRTRouter
    cargo_toml_dir = cargo_toml_path.parent
    # Navigate up: microservices/accounting/{service} -> microservices/accounting -> microservices -> rerp
    project_root = cargo_toml_dir.parent.parent.parent
    brrtrouter_path = project_root.parent / "BRRTRouter"
    
    # Calculate relative path from Cargo.toml to BRRTRouter
    try:
        rel_path = Path(os.path.relpath(brrtrouter_path, cargo_toml_dir)).as_posix()
        rel_path_macros = Path(os.path.relpath(brrtrouter_path / "brrtrouter_macros", cargo_toml_dir)).as_posix()
    except ValueError:
        # If paths are on different drives (Windows), use absolute path
        rel_path = str(brrtrouter_path)
        rel_path_macros = str(brrtrouter_path / "brrtrouter_macros")
    
    # Replace brrtrouter path
    content = re.sub(
        r'brrtrouter = \{ path = "[^"]+" \}',
        f'brrtrouter = {{ path = "{rel_path}" }}',
        content
    )
    
    # Replace brrtrouter_macros path
    content = re.sub(
        r'brrtrouter_macros = \{ path = "[^"]+" \}',
        f'brrtrouter_macros = {{ path = "{rel_path_macros}" }}',
        content
    )
    
    if content != original_content:
        with open(cargo_toml_path, 'w') as f:
            f.write(content)
        print(f"✅ Fixed paths in {cargo_toml_path}")
    else:
        print(f"ℹ️  No changes needed in {cargo_toml_path}")


def main():
    """Main entry point."""
    if len(sys.argv) < 2:
        print("Usage: fix_cargo_toml_paths.py <path-to-Cargo.toml>")
        sys.exit(1)
    
    cargo_toml_path = Path(sys.argv[1])
    fix_cargo_toml(cargo_toml_path)


if __name__ == '__main__':
    main()
