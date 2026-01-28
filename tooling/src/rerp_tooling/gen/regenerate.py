"""Regenerate services from OpenAPI specs using BRRTRouter."""

from pathlib import Path
from typing import Optional

from rerp_tooling.ci.fix_cargo_paths import run as run_fix_cargo_paths
from rerp_tooling.discovery.suites import bff_service_to_suite
from rerp_tooling.gen.brrtrouter import call_brrtrouter_generate


def regenerate_service(
    project_root: Path,
    suite: str,
    service_name: str,
    brrtrouter_path: Optional[Path] = None,
) -> int:
    """Regenerate a single service from its OpenAPI spec.

    Returns 0 on success, 1 on error.
    """
    # Check if this is a BFF service (BFF services have their spec at openapi/{suite}/openapi_bff.yaml)
    # Regular services have their spec at openapi/{suite}/{service_name}/openapi.yaml
    is_bff = bff_service_to_suite(project_root, service_name) == suite

    if is_bff:
        spec_path = project_root / "openapi" / suite / "openapi_bff.yaml"
        # BFF dependencies config is at suite level
        deps_config_path = project_root / "openapi" / suite / "brrtrouter-dependencies.toml"
    else:
        spec_path = project_root / "openapi" / suite / service_name / "openapi.yaml"
        # Regular service dependencies config is alongside the spec
        deps_config_path = spec_path.parent / "brrtrouter-dependencies.toml"

    output_dir = project_root / "microservices" / suite / service_name / "gen"

    if not spec_path.exists():
        print(f"❌ OpenAPI spec not found: {spec_path}")
        return 1

    try:
        # Use shared utility to call BRRTRouter
        result = call_brrtrouter_generate(
            spec_path=spec_path,
            output_dir=output_dir,
            project_root=project_root,
            brrtrouter_path=brrtrouter_path,
            deps_config_path=deps_config_path if deps_config_path.exists() else None,
            capture_output=False,  # Show output
        )

        if result.returncode != 0:
            print(f"❌ Failed to regenerate {service_name}")
            return 1

        print(f"✅ Regenerated {service_name}")

        # Fix Cargo.toml paths
        gen_cargo = output_dir / "Cargo.toml"
        if gen_cargo.exists():
            run_fix_cargo_paths(gen_cargo, project_root)

        return 0
    except FileNotFoundError as e:
        print(f"❌ {e}")
        return 1


def regenerate_suite_services(
    project_root: Path,
    suite: str,
    service_names: list[str],
    brrtrouter_path: Optional[Path] = None,
) -> int:
    """Regenerate all services in a suite.

    Returns 0 if all succeed, 1 if any fail.
    """
    failed = []
    for service_name in service_names:
        rc = regenerate_service(project_root, suite, service_name, brrtrouter_path)
        if rc != 0:
            failed.append(service_name)

    if failed:
        print(f"\n❌ Failed to regenerate {len(failed)} service(s): {', '.join(failed)}")
        return 1

    print(f"\n✅ Successfully regenerated {len(service_names)} service(s) in suite '{suite}'")
    return 0
