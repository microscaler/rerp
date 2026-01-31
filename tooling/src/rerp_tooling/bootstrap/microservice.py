"""Bootstrap a new microservice crate from an OpenAPI specification.

Creates crate (via BRRTRouter), Dockerfile, config, workspace Cargo.toml, Tiltfile.
Idempotent: safe to run multiple times.
"""

from __future__ import annotations

import json
import os
import re
from pathlib import Path
from typing import Any, Optional

import yaml


def _get_registry_path(project_root: Path) -> Optional[Path]:
    p = (
        Path(os.environ.get("RERP_PORT_REGISTRY", "")).resolve()
        if os.environ.get("RERP_PORT_REGISTRY")
        else None
    )
    if p and p.exists():
        return p
    p = project_root / "port-registry.json"
    return p if p.exists() else None


def _get_port_from_registry(project_root: Path, service_name: str) -> Optional[int]:
    path = _get_registry_path(project_root)
    if not path:
        return None
    with path.open() as f:
        data = json.load(f)
    asn = data.get("assignments", {})
    return asn.get(service_name)


def to_snake_case(name: str) -> str:
    name = re.sub(r"[- ]+", "_", name)
    name = re.sub(r"(?<!^)(?<!_)([A-Z])", r"_\1", name)
    return name.lower()


def to_pascal_case(name: str) -> str:
    return "".join(word.capitalize() for word in name.split("-"))


def derive_binary_name(openapi_spec: dict[str, Any], service_name: str) -> str:
    title = (openapi_spec.get("info") or {}).get("title", "")
    if title:
        binary_name = to_snake_case(title)
        if not binary_name.endswith("_api"):
            binary_name = (
                binary_name + "_api"
                if binary_name.endswith("_service")
                else binary_name + "_service_api"
            )
        return binary_name
    return f"{service_name.replace('-', '_')}_service_api"


def load_openapi_spec(spec_path: Path) -> dict[str, Any]:
    with spec_path.open() as f:
        return yaml.safe_load(f)


def create_dockerfile(service_name: str, binary_name: str, port: int, output_path: Path) -> None:
    content = f"""# Minimal runtime-only Dockerfile for {to_pascal_case(service_name)} Service (Tilt development)
# Binary is cross-compiled on host (Apple Silicon -> x86_64 Linux) and copied in

ARG TARGETPLATFORM=linux/amd64
FROM --platform=${{TARGETPLATFORM}} alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache \\
    ca-certificates \\
    libgcc

# Create app directory with proper permissions for live updates
WORKDIR /app

# Copy pre-built binary from staging directory (x86_64 Linux musl target)
COPY ./build_artifacts/{binary_name} /app/{binary_name}
RUN chmod +x /app/{binary_name}

# Create directories for configuration and assets with write permissions
RUN mkdir -p /app/config /app/doc /app/static_site && \\
    chmod -R 777 /app

# Copy configuration and assets to writable locations
COPY ./microservices/accounting/{service_name}/impl/config /app/config
COPY ./microservices/accounting/{service_name}/gen/doc /app/doc
COPY ./microservices/accounting/{service_name}/gen/static_site /app/static_site

# Expose HTTP port
EXPOSE {port}

# Set runtime environment
ENV RUST_BACKTRACE=1
ENV RUST_LOG=debug

# Run the service
ENTRYPOINT ["/app/{binary_name}", \\
    "--spec", "/app/doc/openapi.yaml", \\
    "--doc-dir", "/app/doc", \\
    "--static-dir", "/app/static_site", \\
    "--config", "/app/config/config.yaml"]
"""
    output_path.write_text(content)
    print(f"✅ Created Dockerfile: {output_path}")


def create_config_yaml(output_path: Path) -> None:
    config_content = """# BRRTRouter application configuration (YAML)
# Adjust values per environment and reload/restart the app.

security:
  api_keys:
    ApiKeyHeader:
      key: "test123"
http:
  keep_alive: true
  timeout_secs: 5
  max_requests: 5000
"""
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(config_content)
    print(f"✅ Created config.yaml: {output_path}")


def update_workspace_cargo_toml(service_name: str, cargo_toml_path: Path) -> None:
    if not cargo_toml_path.exists():
        return
    content = cargo_toml_path.read_text()
    gen_member = f'"accounting/{service_name}/gen"'
    impl_member = f'"accounting/{service_name}/impl"'
    if gen_member in content and impl_member in content:
        return
    m = re.search(r"(members\s*=\s*\[)(.*?)(\])", content, re.DOTALL)
    if not m:
        return
    existing = [x.strip().strip('"') for x in m.group(2).split(",") if x.strip()]
    if gen_member.strip('"') in existing and impl_member.strip('"') in existing:
        return
    if gen_member.strip('"') not in existing:
        existing.append(gen_member.strip('"'))
    if impl_member.strip('"') not in existing:
        existing.append(impl_member.strip('"'))
    existing.sort()
    new_members = '    "' + '",\n    "'.join(existing) + '",\n'
    new_content = content[: m.start()] + m.group(1) + "\n" + new_members + "]" + content[m.end() :]
    cargo_toml_path.write_text(new_content)
    print(f"✅ Added {service_name}/gen and {service_name}/impl to workspace Cargo.toml")


def update_tiltfile(
    service_name: str, spec_file: str, binary_name: str, port: int, tiltfile_path: Path
) -> None:
    if not tiltfile_path.exists():
        return
    content = tiltfile_path.read_text()
    orig = content

    # 1. BINARY_NAMES
    m = re.search(r"(BINARY_NAMES\s*=\s*\{)(.*?)(\})", content, re.DOTALL)
    if m and f"'{service_name}':" not in m.group(2) and f'"{service_name}":' not in m.group(2):
        entries = [
            line.strip()
            for line in m.group(2).split("\n")
            if line.strip() and not line.strip().startswith("#")
        ]
        entries.append(f"'{service_name}': '{binary_name}',")
        entries.sort()
        content = (
            content[: m.start()]
            + m.group(1)
            + "\n"
            + "\n".join("    " + e for e in entries)
            + "\n}"
            + content[m.end() :]
        )

    # 2. create_microservice_lint
    lint_call = f"create_microservice_lint('{service_name}', '{spec_file}')"
    if lint_call not in content:
        for m in list(re.finditer(r"(create_microservice_lint\([^\n]+\n)", content))[::-1]:
            content = content[: m.end()] + lint_call + "\n" + content[m.end() :]
            break

    # 3. create_microservice_gen
    gen_call = f"create_microservice_gen('{service_name}', '{spec_file}', '{service_name}')"
    if gen_call not in content:
        for m in list(re.finditer(r"(create_microservice_gen\([^\n]+\n)", content))[::-1]:
            content = content[: m.end()] + gen_call + "\n" + content[m.end() :]
            break

    # 4. resource_deps (build-workspace) and deps (Cargo.toml)
    m = re.search(
        r"(resource_deps=\[)(.*?)(\]\s*labels=\['microservices-build'\])",
        content,
        re.DOTALL,
    )
    if m and f"'{service_name}-service-gen'" not in m.group(2):
        deps = [d.strip().strip("'\"") for d in m.group(2).split(",") if d.strip()]
        deps.append(f"{service_name}-service-gen")
        deps.sort()
        content = (
            content[: m.start()]
            + m.group(1)
            + "'"
            + "', '".join(deps)
            + "',\n    "
            + m.group(3)
            + content[m.end() :]
        )

    m = re.search(r"(deps=\[)(.*?)(\]\s*resource_deps=)", content, re.DOTALL)
    gen_cargo = f"'./microservices/accounting/{service_name}/gen/Cargo.toml'"
    impl_cargo = f"'./microservices/accounting/{service_name}/impl/Cargo.toml'"
    if m:
        deps = [d.strip().strip("'\"") for d in m.group(2).split(",") if d.strip()]
        if gen_cargo.strip("'\"") not in deps:
            deps.append(gen_cargo.strip("'\""))
        if impl_cargo.strip("'\"") not in deps:
            deps.append(impl_cargo.strip("'\""))
        deps.sort()
        content = (
            content[: m.start()]
            + m.group(1)
            + "'"
            + "',\n        '".join(deps)
            + "',\n    "
            + m.group(3)
            + content[m.end() :]
        )

    # 5. get_service_port ports dict
    m = re.search(r"(ports\s*=\s*\{)(.*?)(\s*\})", content, re.DOTALL)
    if m and f"'{service_name}':" not in m.group(2):
        lines = [
            line.strip()
            for line in m.group(2).split("\n")
            if line.strip() and not line.strip().startswith("#")
        ]
        lines.append(f"'{service_name}': '{port}',")
        lines.sort()
        content = (
            content[: m.start()]
            + m.group(1)
            + "\n"
            + "\n".join("        " + e for e in lines)
            + m.group(3)
            + content[m.end() :]
        )

    # 6. create_microservice_deployment
    deployment_call = f"create_microservice_deployment('{service_name}')"
    if deployment_call not in content:
        for m in list(re.finditer(r"(create_microservice_deployment\([^\n]+\n)", content))[::-1]:
            content = content[: m.end()] + deployment_call + "\n" + content[m.end() :]
            break

    if content != orig:
        tiltfile_path.write_text(content)
        print("✅ Updated Tiltfile")


def _update_gen_cargo_toml(cargo_path: Path, service_name: str) -> None:
    """Update gen/Cargo.toml to be a library crate (version, [lib]). [package].name is NOT touched — it is set by brrtrouter-gen via --package-name."""
    content = cargo_path.read_text()
    service_snake = service_name.replace("-", "_")
    gen_crate_name = f"rerp_accounting_{service_snake}_gen"

    # Do not overwrite [package].name — it is written by brrtrouter-gen when we pass --package-name.
    # Update version to match workspace
    content = re.sub(r'version = "[^"]+"', 'version = "0.1.3"', content, count=1)

    # Add [lib] section if not present
    if "[lib]" not in content:
        # Insert after [package] section
        content = re.sub(
            r"(\[package\][^\[]+)",
            r'\1\n[lib]\nname = "' + gen_crate_name + '"\npath = "src/lib.rs"\n',
            content,
            count=1,
        )

    cargo_path.write_text(content)


def _create_impl_cargo_toml(cargo_path: Path, service_name: str) -> None:
    """Create impl/Cargo.toml as a binary crate."""
    service_snake = service_name.replace("-", "_")
    gen_crate_name = f"rerp_accounting_{service_snake}_gen"
    impl_crate_name = f"rerp_accounting_{service_snake}"

    cargo_content = f"""[package]
name = "{impl_crate_name}"
version = "0.1.3"
edition = "2021"

[[bin]]
name = "{impl_crate_name}"
path = "src/main.rs"

[features]
default = ["jemalloc"]
jemalloc = ["tikv-jemallocator"]

[dependencies]
# Generated crate
{gen_crate_name} = {{ path = "../gen" }}

# BRRTRouter (re-exported from gen crate, but may need direct access)
brrtrouter = {{ workspace = true }}
brrtrouter_macros = {{ workspace = true }}

# Decimal types from OpenAPI format: decimal / money (used in generated stubs)
rust_decimal = {{ workspace = true }}

# Standard dependencies
serde = {{ workspace = true }}
serde_json = {{ workspace = true }}
serde_yaml = {{ workspace = true }}
config = {{ workspace = true }}
http = {{ workspace = true }}
may = {{ workspace = true }}
may_minihttp = {{ workspace = true }}
anyhow = {{ workspace = true }}
clap = {{ workspace = true }}
tikv-jemallocator = {{ workspace = true, optional = true }}
"""

    cargo_path.parent.mkdir(parents=True, exist_ok=True)
    cargo_path.write_text(cargo_content)


def _create_impl_main(gen_main_path: Path, impl_main_path: Path, service_name: str) -> None:
    """Create impl/src/main.rs from gen/src/main.rs with necessary modifications.

    Modifications:
    1. Change imports to use gen crate
    2. Import impl controllers instead of gen controllers
    3. Update paths for config and doc directories
    """
    import re

    if not gen_main_path.exists():
        print(f"❌ Gen main.rs not found: {gen_main_path}")
        return

    content = gen_main_path.read_text()
    service_snake = service_name.replace("-", "_")
    gen_crate_name = f"rerp_accounting_{service_snake}_gen"

    # 1. Remove the warning comments at the top (they're for gen, not impl)
    content = re.sub(
        r"^// ⚠️ WARNING: This file is auto-generated by BRRTRouter.*?\n",
        "",
        content,
        flags=re.MULTILINE | re.DOTALL,
    )

    # 2. Add comment at top explaining this uses gen crate
    header = f"""// This file uses generated code from the {gen_crate_name} crate
// Business logic controllers are in impl/src/controllers/
// This file is based on gen/src/main.rs but uses impl controllers

"""
    content = header + content

    # 3. Remove local mod declarations (handlers, registry, controllers are in gen crate)
    # Remove mod handlers, mod registry, and mod controllers (with optional whitespace)
    content = re.sub(r"^mod (handlers|registry|controllers);\s*\n", "", content, flags=re.MULTILINE)

    # 4. Add imports for gen crate and impl controllers
    imports_section = f"""// Use generated code from gen crate
use {gen_crate_name}::*;
use {gen_crate_name}::handlers::*;
use {gen_crate_name}::registry::*;

// Import implementation controllers (business logic)
mod controllers;
use controllers::*;

"""

    # Find where to insert: after the header comments, before the first use statement
    lines = content.split("\n")
    insert_idx = 0
    for i, line in enumerate(lines):
        if line.startswith("use ") and not line.startswith("use crate::"):
            insert_idx = i
            break

    # Insert the imports section
    lines.insert(insert_idx, imports_section.rstrip())
    content = "\n".join(lines)

    # 5. Update registry registration - add note about impl controllers
    content = re.sub(
        r"(unsafe \{\s*registry::register_from_spec)",
        r"// NOTE: This registers controllers from gen crate. We need to register impl controllers instead.\n    // TODO: Update to register impl controllers\n    \1",
        content,
        flags=re.MULTILINE,
    )

    # 6. Update config path (it's now in impl/config/)
    content = re.sub(
        r'default_value = "\./config/config\.yaml"',
        r'default_value = "./config/config.yaml"',
        content,
    )

    # 7. Update doc path (it's now in gen/doc/)
    content = re.sub(r'default_value = "\./doc"', r'default_value = "../gen/doc"', content)

    # 8. Update spec path (it's now in gen/doc/)
    content = re.sub(
        r'default_value = "\./doc/openapi\.yaml"',
        r'default_value = "../gen/doc/openapi.yaml"',
        content,
    )

    # Write the file
    impl_main_path.parent.mkdir(parents=True, exist_ok=True)
    impl_main_path.write_text(content)
    print(f"✅ Created {impl_main_path}")


def generate_code_with_brrtrouter(
    spec_path: Path,
    output_dir: Path,
    project_root: Path,
    service_name: Optional[str] = None,
) -> None:
    """Generate gen crate via BRRTRouter; pass package_name so [package].name is correct."""
    from brrtrouter_tooling.gen.brrtrouter import call_brrtrouter_generate

    from rerp_tooling.build.constants import get_package_names

    package_names = get_package_names(project_root)
    package_name = None
    if service_name and service_name in package_names:
        package_name = f"{package_names[service_name]}_gen"

    result = call_brrtrouter_generate(
        spec_path=spec_path,
        output_dir=output_dir,
        project_root=project_root,
        brrtrouter_path=project_root.parent / "BRRTRouter",
        package_name=package_name,
        capture_output=True,
    )
    if result.returncode != 0:
        msg = f"brrtrouter-gen generate failed: {result.stderr or result.stdout or 'unknown'}"
        raise RuntimeError(msg)
    print("✅ Code generation complete")


def _ensure_impl_scaffold(project_root: Path, suite: str, service_name: str) -> None:
    """Create impl directory structure, Cargo.toml, main.rs, and config if impl is missing.

    Used when impl was deleted and user runs 'rerp gen stubs'; allows stubs to be
    generated without full bootstrap. Requires gen/ to exist (for main.rs and types).
    """
    crate_dir = project_root / "microservices" / suite / service_name
    gen_dir = crate_dir / "gen"
    impl_dir = crate_dir / "impl"
    if impl_dir.exists():
        return
    if not gen_dir.exists():
        msg = f"gen/ not found at {gen_dir}; run 'rerp gen accounting {service_name}' first"
        raise FileNotFoundError(msg)
    impl_dir.mkdir(parents=True, exist_ok=True)
    (impl_dir / "config").mkdir(parents=True, exist_ok=True)
    (impl_dir / "src" / "controllers").mkdir(parents=True, exist_ok=True)
    config_path = impl_dir / "config" / "config.yaml"
    if not config_path.exists():
        create_config_yaml(config_path)
    impl_cargo = impl_dir / "Cargo.toml"
    _create_impl_cargo_toml(impl_cargo, service_name)
    gen_main = gen_dir / "src" / "main.rs"
    impl_main = impl_dir / "src" / "main.rs"
    if gen_main.exists():
        _create_impl_main(gen_main, impl_main, service_name)
    cargo_toml_path = project_root / "microservices" / "Cargo.toml"
    update_workspace_cargo_toml(service_name, cargo_toml_path)
    print(f"✅ Created impl scaffold for {service_name} (impl/, Cargo.toml, main.rs, config)")


def generate_impl_stubs_with_brrtrouter(
    spec_path: Path,
    impl_dir: Path,
    project_root: Path,
    service_name: str,
    *,
    force: bool = False,
    sync: bool = False,
) -> None:
    """Generate impl controller stubs via brrtrouter-gen generate-stubs.

    Stubs use correct types from the OpenAPI spec (e.g. Decimal literals from
    dummy_value/rust_literal_for_example). Do not edit impl controllers by hand
    for initial creation; regenerate stubs with --force if needed.
    Use --sync to patch only signature/Response for stubs that have the user-owned sentinel.
    """
    from brrtrouter_tooling.gen.brrtrouter import call_brrtrouter_generate_stubs

    from rerp_tooling.build.constants import get_package_names

    package_names = get_package_names(project_root)
    if service_name not in package_names:
        msg = f"Unknown service_name for stub generation: {service_name!r}"
        raise ValueError(msg)
    component_name = f"{package_names[service_name]}_gen"

    result = call_brrtrouter_generate_stubs(
        spec_path=spec_path,
        impl_dir=impl_dir,
        component_name=component_name,
        project_root=project_root,
        brrtrouter_path=project_root.parent / "BRRTRouter",
        force=force,
        sync=sync,
        capture_output=True,
    )
    if result.returncode != 0:
        msg = f"brrtrouter-gen generate-stubs failed: {result.stderr or result.stdout or 'unknown'}"
        raise RuntimeError(msg)
    print("✅ Implementation stubs generated (impl/src/controllers)")


def regenerate_impl_stubs(
    project_root: Path,
    suite: str,
    service: Optional[str] = None,
    *,
    force: bool = False,
    sync: bool = False,
) -> int:
    """Regenerate impl controller stubs for one or all services in a suite.

    Uses brrtrouter-gen generate-stubs. With --force, overwrites existing stub
    files (handlers with sentinel are preserved). With --sync, only patches
    signature/Response for stubs that have the user-owned sentinel.
    Returns 0 on success, 1 on error.
    """
    from rerp_tooling.discovery import suite_sub_service_names

    services = [service] if service else suite_sub_service_names(project_root, suite)
    if not services:
        print(f"⚠️  No services found for suite: {suite}")
        return 1
    for svc in services:
        spec_path = project_root / "openapi" / suite / svc / "openapi.yaml"
        impl_dir = project_root / "microservices" / suite / svc / "impl"
        if not spec_path.exists():
            print(f"⚠️  Skipping {svc}: spec not found at {spec_path}")
            continue
        if not impl_dir.exists():
            try:
                _ensure_impl_scaffold(project_root, suite, svc)
            except FileNotFoundError as e:
                print(f"⚠️  Skipping {svc}: {e}")
                continue
        try:
            generate_impl_stubs_with_brrtrouter(
                spec_path, impl_dir, project_root, svc, force=force, sync=sync
            )
        except (ValueError, RuntimeError) as e:
            print(f"❌ {svc}: {e}")
            return 1
    return 0


def run_bootstrap_microservice(
    service_name: str,
    port: Optional[int],
    project_root: Path,
    *,
    force_stubs: bool = False,
) -> int:
    """Bootstrap microservice. Returns 0 on success, 1 on error."""
    if port is None:
        port = _get_port_from_registry(project_root, service_name)
    if port is None:
        print(
            f"⚠️  No port for {service_name}. Run: rerp ports assign {service_name} --update-configs"
        )
        return 1

    spec_file = f"{service_name}/openapi.yaml"
    spec_path = project_root / "openapi" / "accounting" / service_name / "openapi.yaml"
    crate_dir = project_root / "microservices" / "accounting" / service_name
    gen_dir = crate_dir / "gen"
    impl_dir = crate_dir / "impl"
    dockerfile_path = project_root / "docker" / "microservices" / f"Dockerfile.{service_name}"
    config_path = impl_dir / "config" / "config.yaml"
    cargo_toml_path = project_root / "microservices" / "Cargo.toml"
    tiltfile_path = project_root / "Tiltfile"

    if not spec_path.exists():
        print(f"❌ OpenAPI spec not found: {spec_path}")
        return 1

    openapi_spec = load_openapi_spec(spec_path)
    binary_name = derive_binary_name(openapi_spec, service_name)
    print(f"🚀 Bootstrapping {service_name} (port {port}, binary {binary_name})")

    # Generate code to gen/ directory (pass service_name so --package-name is set)
    generate_code_with_brrtrouter(spec_path, gen_dir, project_root, service_name=service_name)

    gen_cargo = gen_dir / "Cargo.toml"
    if gen_cargo.exists():
        from rerp_tooling.ci.fix_cargo_paths import run as run_fix_cargo_paths

        run_fix_cargo_paths(gen_cargo, project_root)
        # Update gen/Cargo.toml to be a library crate
        _update_gen_cargo_toml(gen_cargo, service_name)

    # Create impl directory structure
    impl_dir.mkdir(parents=True, exist_ok=True)
    (impl_dir / "config").mkdir(parents=True, exist_ok=True)
    (impl_dir / "src" / "controllers").mkdir(parents=True, exist_ok=True)

    if not config_path.exists():
        create_config_yaml(config_path)

    # Create impl/Cargo.toml if it doesn't exist
    impl_cargo = impl_dir / "Cargo.toml"
    if not impl_cargo.exists():
        _create_impl_cargo_toml(impl_cargo, service_name)

    # Create impl/src/main.rs if gen/src/main.rs exists
    impl_main = impl_dir / "src" / "main.rs"
    gen_main = gen_dir / "src" / "main.rs"
    if not impl_main.exists() and gen_main.exists():
        _create_impl_main(gen_main, impl_main, service_name)

    # Generate impl controller stubs from OpenAPI (Decimal etc. come from brrtrouter-gen)
    generate_impl_stubs_with_brrtrouter(
        spec_path, impl_dir, project_root, service_name, force=force_stubs
    )

    create_dockerfile(service_name, binary_name, port, dockerfile_path)
    update_workspace_cargo_toml(service_name, cargo_toml_path)
    update_tiltfile(service_name, spec_file, binary_name, port, tiltfile_path)

    print(f"✅ Bootstrap complete for {service_name}. Next: tilt up")
    return 0
