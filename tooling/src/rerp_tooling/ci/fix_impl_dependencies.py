"""Fix impl crate Cargo.toml files to include dependencies used by gen crates."""

import re
import sys
from pathlib import Path


def update_impl_cargo_dependencies(impl_cargo_path: Path) -> bool:
    """Update impl Cargo.toml to include dependencies from gen crate.

    Returns True if file was modified, False otherwise.
    """
    if not impl_cargo_path.exists():
        return False

    # Find gen crate directory
    gen_dir = impl_cargo_path.parent.parent / "gen"
    if not gen_dir.exists():
        return False

    # Check gen crate for dependencies
    gen_cargo = gen_dir / "Cargo.toml"
    uses_decimal = False
    uses_money = False

    if gen_cargo.exists():
        gen_cargo_content = gen_cargo.read_text()
        uses_decimal = "rust_decimal" in gen_cargo_content
        uses_money = "rusty-money" in gen_cargo_content
    else:
        # Fallback: check generated source files
        gen_src_dir = gen_dir / "src"
        if gen_src_dir.exists():
            for rust_file in gen_src_dir.rglob("*.rs"):
                try:
                    file_content = rust_file.read_text()
                    if "rust_decimal::Decimal" in file_content or "Decimal" in file_content:
                        uses_decimal = True
                    if "rusty_money::Money" in file_content or "Money<" in file_content:
                        uses_money = True
                    if uses_decimal and uses_money:
                        break
                except (OSError, UnicodeDecodeError):
                    continue

    # Read impl Cargo.toml
    content = impl_cargo_path.read_text()
    modified = False

    # Add rust_decimal if gen uses it but impl doesn't
    if uses_decimal and "rust_decimal" not in content:
        # Add after tikv-jemallocator line
        if "tikv-jemallocator" in content:
            content = re.sub(
                r"(tikv-jemallocator = \{[^\}]+\}\n)",
                r"\1rust_decimal = { workspace = true }\n",
                content,
                count=1,
            )
        else:
            # Add at end of dependencies section
            content = re.sub(
                r"(\[dependencies\][^\[]+)(\n)",
                r"\1rust_decimal = { workspace = true }\2",
                content,
                count=1,
            )
        modified = True

    # Add rusty-money if gen uses it but impl doesn't
    if uses_money and "rusty-money" not in content:
        # Add after rust_decimal if it exists, otherwise after tikv-jemallocator
        if "rust_decimal" in content:
            content = re.sub(
                r"(rust_decimal = \{[^\}]+\}\n)",
                r"\1rusty-money = { workspace = true }\n",
                content,
                count=1,
            )
        elif "tikv-jemallocator" in content:
            content = re.sub(
                r"(tikv-jemallocator = \{[^\}]+\}\n)",
                r"\1rusty-money = { workspace = true }\n",
                content,
                count=1,
            )
        else:
            # Add at end of dependencies section
            content = re.sub(
                r"(\[dependencies\][^\[]+)(\n)",
                r"\1rusty-money = { workspace = true }\2",
                content,
                count=1,
            )
        modified = True

    if modified:
        impl_cargo_path.write_text(content)
        return True

    return False


def fix_all_impl_dependencies(project_root: Path) -> int:
    """Fix all impl crate Cargo.toml files in microservices/accounting."""
    accounting_dir = project_root / "microservices" / "accounting"
    if not accounting_dir.exists():
        print(f"❌ Accounting directory not found: {accounting_dir}")
        return 1

    fixed_count = 0
    for service_dir in sorted(accounting_dir.iterdir()):
        if not service_dir.is_dir():
            continue

        impl_cargo = service_dir / "impl" / "Cargo.toml"
        if impl_cargo.exists() and update_impl_cargo_dependencies(impl_cargo):
            rel_path = impl_cargo.relative_to(project_root)
            print(f"✅ Updated {rel_path}")
            fixed_count += 1

    if fixed_count > 0:
        print(f"\n✅ Fixed {fixed_count} impl Cargo.toml file(s)")
    else:
        print("✅ All impl Cargo.toml files are up to date")

    return 0


def main() -> int:
    """CLI entry point."""
    if len(sys.argv) > 1:
        # Fix specific file
        impl_cargo_path = Path(sys.argv[1])
        if not impl_cargo_path.exists():
            print(f"❌ File not found: {impl_cargo_path}")
            return 1
        if update_impl_cargo_dependencies(impl_cargo_path):
            print(f"✅ Updated {impl_cargo_path}")
        else:
            print(f"✅ {impl_cargo_path} is up to date")
        return 0
    # Fix all in project
    project_root = Path(__file__).parent.parent.parent.parent.parent
    return fix_all_impl_dependencies(project_root)


if __name__ == "__main__":
    sys.exit(main())
