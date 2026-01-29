"""`rerp openapi` subcommands: validate, generate, fix-operation-id-casing. Delegates to brrtrouter_tooling.openapi."""

from pathlib import Path
from typing import Optional

from rerp_tooling.openapi import fix_operation_id_run, validate_specs


def run_openapi(args, project_root: Path) -> int:
    if args.openapi_cmd == "validate":
        return _run_validate(project_root, getattr(args, "openapi_dir", None))
    if args.openapi_cmd == "fix-operation-id-casing":
        return _run_fix_operation_id_casing(args, project_root)
    return 0


def _run_validate(project_root: Path, openapi_dir_override: Optional[Path]) -> int:
    openapi_dir = (
        openapi_dir_override if openapi_dir_override is not None else (project_root / "openapi")
    )
    errors = validate_specs(openapi_dir)
    # Optional: list valid files (CI does). We only have errors from validate_specs; valid files are not returned.
    # To mimic CI we could rglob and for each: if (p, e) in errors then print ❌ else print ✅. Simpler: just report errors.
    for path, exc in errors:
        print(f"❌ {path}: {exc}")
    if errors:
        print(f"\n❌ Found {len(errors)} invalid OpenAPI specs")
        return 1
    if openapi_dir.exists():
        count = len(list(openapi_dir.rglob("openapi.yaml")))
        if count > 0:
            print(f"\n✅ All {count} OpenAPI specs are valid")
        else:
            print("\n✅ No openapi.yaml found; nothing to validate.")
    else:
        print("\n✅ openapi/ directory not found; nothing to validate.")
    return 0


def _run_fix_operation_id_casing(args, project_root: Path) -> int:
    openapi_dir = (getattr(args, "openapi_dir", None) or (project_root / "openapi")).resolve()
    dry_run = getattr(args, "dry_run", False)
    verbose = getattr(args, "verbose", False)
    total, touched = fix_operation_id_run(
        openapi_dir, dry_run=dry_run, verbose=verbose, rel_to=project_root
    )
    if touched:
        prefix = "[DRY-RUN] " if dry_run else ""
        print(f"{prefix}Updated {touched} file(s), {total} operationId(s) converted to snake_case.")
    else:
        print("No operationId casing changes needed.")
    return 0
