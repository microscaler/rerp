"""`rerp bff` subcommands: generate-system."""

from pathlib import Path

from rerp_tooling.bff import (
    discover_sub_services,
    generate_system_bff_spec,
    list_systems_with_sub_services,
)


def run_bff(args, project_root: Path) -> None:
    if getattr(args, "bff_cmd", None) == "generate-system":
        _run_generate_system(project_root, args)
        return
    msg = "bff: missing or unknown subcommand"
    raise ValueError(msg)


def _run_generate_system(project_root: Path, args) -> None:
    openapi_dir = getattr(args, "openapi_dir", None) or (project_root / "openapi")
    system = getattr(args, "system", None)
    output = getattr(args, "output", None)

    if system:
        out_path = Path(output) if output else None
        subs = discover_sub_services(openapi_dir, system)
        if not subs:
            print(f"⚠️  No sub-services found for {system}")
            return
        print(
            f"🔄 Generating {system} system BFF OpenAPI specification ({len(subs)} sub-services)..."
        )
        generate_system_bff_spec(openapi_dir, system, output_path=out_path)
        out = Path(output) if output else (openapi_dir / system / "openapi.yaml")
        print(f"✅ Generated {system} BFF spec: {out}")
    else:
        systems = list_systems_with_sub_services(openapi_dir)
        print(
            f"🔄 Generating system BFF specs for all systems ({len(systems)} with sub-services)..."
        )
        for s in systems:
            generate_system_bff_spec(openapi_dir, s, output_path=None)
            print(f"✅ {s} → openapi/{s}/openapi.yaml")
