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


def create_dependencies_config_toml(output_path: Path) -> None:
    """Create brrtrouter-dependencies.toml file alongside OpenAPI spec."""
    config_content = """# BRRTRouter Dependencies Configuration
#
# This file specifies additional dependencies for code generation.
# BRRTRouter will auto-detect this file alongside openapi.yaml

[dependencies]
# Always include these dependencies
# Example: serde_with = { workspace = true, features = ["chrono"] }

[conditional]
# Include rust_decimal if rust_decimal::Decimal is detected in generated code
rust_decimal = { detect = "rust_decimal::Decimal", workspace = true }
"""
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(config_content)
    print(f"✅ Created brrtrouter-dependencies.toml: {output_path}")


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
    """Update gen/Cargo.toml to be a library crate."""
    content = cargo_path.read_text()
    service_snake = service_name.replace("-", "_")
    gen_crate_name = f"rerp_accounting_{service_snake}_gen"

    # Update package name
    content = re.sub(r'name = "[^"]+"', f'name = "{gen_crate_name}"', content, count=1)
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

    # Check if generated code uses Money or Decimal types and add dependencies
    gen_src_dir = cargo_path.parent / "src"
    uses_money = False
    uses_decimal = False

    if gen_src_dir.exists():
        # Check types.rs and handler files for Money/Decimal usage
        for rust_file in gen_src_dir.rglob("*.rs"):
            try:
                file_content = rust_file.read_text()
                if "rusty_money::Money" in file_content or "Money<" in file_content:
                    uses_money = True
                if "rust_decimal::Decimal" in file_content or "Decimal" in file_content:
                    uses_decimal = True
                if uses_money and uses_decimal:
                    break
            except (OSError, UnicodeDecodeError):
                continue

    # Add rusty-money dependency if Money types are used
    if uses_money and "rusty-money" not in content:
        # Add after tikv-jemallocator line
        if "tikv-jemallocator" in content:
            content = re.sub(
                r"(tikv-jemallocator = \{[^\}]+\}\n)",
                r"\1rusty-money = { workspace = true }\n",
                content,
                count=1,
            )
        else:
            # Add at end of dependencies section
            content = re.sub(
                r"(\[dependencies\][^\[]+)",
                r"\1rusty-money = { workspace = true }\n",
                content,
                count=1,
            )

    # Add rust_decimal dependency if Decimal types are used (if not already present)
    if uses_decimal and "rust_decimal" not in content:
        if "rusty-money" in content:
            content = re.sub(
                r"(rusty-money = \{[^\}]+\}\n)",
                r"\1rust_decimal = { workspace = true }\n",
                content,
                count=1,
            )
        elif "tikv-jemallocator" in content:
            content = re.sub(
                r"(tikv-jemallocator = \{[^\}]+\}\n)",
                r"\1rust_decimal = { workspace = true }\n",
                content,
                count=1,
            )

    cargo_path.write_text(content)


def _create_impl_cargo_toml(cargo_path: Path, service_name: str) -> None:
    """[DEPRECATED] Create impl/Cargo.toml as a binary crate.

    This is a fallback only. Prefer using BRRTRouter's generate-impl-stubs command.
    """
    service_snake = service_name.replace("-", "_")
    gen_crate_name = f"rerp_accounting_{service_snake}_gen"
    impl_crate_name = f"rerp_accounting_{service_snake}"

    # Check if gen crate uses Decimal or Money types
    gen_dir = cargo_path.parent.parent / "gen"
    uses_decimal = False
    uses_money = False

    if gen_dir.exists():
        gen_cargo = gen_dir / "Cargo.toml"
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

    # Build dependencies section
    deps = f"""# Generated crate
{gen_crate_name} = {{ path = "../gen" }}

# BRRTRouter (re-exported from gen crate, but may need direct access)
brrtrouter = {{ workspace = true }}
brrtrouter_macros = {{ workspace = true }}

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
tikv-jemallocator = {{ workspace = true, optional = true }}"""

    # Add rust_decimal if gen crate uses Decimal types
    if uses_decimal:
        deps += "\nrust_decimal = { workspace = true }"

    # Add rusty-money if gen crate uses Money types
    if uses_money:
        deps += "\nrusty-money = { workspace = true }"

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
{deps}
"""

    cargo_path.parent.mkdir(parents=True, exist_ok=True)
    cargo_path.write_text(cargo_content)


def _create_impl_main(gen_main_path: Path, impl_main_path: Path, service_name: str) -> None:
    """[DEPRECATED] Create impl/src/main.rs from gen/src/main.rs with necessary modifications.

    This is a fallback only. Prefer using BRRTRouter's generate-impl-stubs command.

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


def _generate_impl_with_brrtrouter(
    spec_path: Path, impl_dir: Path, service_name: str, project_root: Path
) -> None:
    """Generate impl crate using BRRTRouter's generate-stubs command.

    Uses --component-name to specify the gen crate name directly, avoiding
    the need for directory name conventions.
    """
    from rerp_tooling.gen.brrtrouter import call_brrtrouter_generate_stubs

    # Use gen crate name as component name (BRRTRouter will use this for imports in main.rs)
    # Note: BRRTRouter will generate {component_name}_impl as impl crate name,
    # but we'll fix this in post-processing to match RERP conventions
    service_snake = service_name.replace("-", "_")
    gen_crate_name = f"rerp_accounting_{service_snake}_gen"

    # Ensure impl directory exists
    impl_dir.mkdir(parents=True, exist_ok=True)

    result = call_brrtrouter_generate_stubs(
        spec_path=spec_path,
        impl_dir=impl_dir,
        component_name=gen_crate_name,
        project_root=project_root,
        force=True,
        capture_output=True,
    )

    if result.returncode != 0:
        print(f"⚠️  BRRTRouter impl generation failed: {result.stderr}")
        msg = f"BRRTRouter impl generation failed: {result.stderr}"
        raise RuntimeError(msg)

    # Post-process: Fix crate names to match RERP conventions
    impl_cargo = impl_dir / "Cargo.toml"
    if impl_cargo.exists():
        _fix_impl_cargo_naming(impl_cargo, service_name)

    impl_main = impl_dir / "src" / "main.rs"
    if impl_main.exists():
        _fix_impl_main_naming(impl_main, service_name)

    print("✅ Generated impl crate with BRRTRouter")


def _fix_impl_cargo_naming(cargo_path: Path, service_name: str) -> None:
    """Fix impl Cargo.toml naming to match RERP conventions.

    BRRTRouter generates with component_name, but we need RERP-specific naming.
    """
    if not cargo_path.exists():
        return

    service_snake = service_name.replace("-", "_")
    gen_crate_name = f"rerp_accounting_{service_snake}_gen"
    impl_crate_name = f"rerp_accounting_{service_snake}"

    content = cargo_path.read_text()

    # Replace crate name (BRRTRouter may generate {component}_impl, RERP needs rerp_accounting_{service})
    # Check if it needs fixing
    if f'name = "{impl_crate_name}"' not in content:
        content = re.sub(
            r'name = "[^"]+"',
            f'name = "{impl_crate_name}"',
            content,
            count=1,
        )

    # Replace gen crate dependency path
    # BRRTRouter generates: {component_name} = { path = "../{component_name}" }
    # RERP needs: {gen_crate_name} = { path = "../gen" }
    # Match the first dependency line (the gen crate) and fix both name and path
    # Pattern matches: any_word = { path = "../anything" }
    gen_dep_pattern = r'^(\w+) = \{ path = "\.\./[^"]+" \}'
    if re.search(gen_dep_pattern, content, re.MULTILINE):
        content = re.sub(
            gen_dep_pattern,
            f'{gen_crate_name} = {{ path = "../gen" }}',
            content,
            count=1,
            flags=re.MULTILINE,
        )

    # Add RERP-specific dependencies that BRRTRouter doesn't include
    additional_deps = []

    # Ensure rust_decimal is included if gen uses it
    gen_cargo = cargo_path.parent.parent / "gen" / "Cargo.toml"
    if gen_cargo.exists():
        gen_content = gen_cargo.read_text()
        if "rust_decimal" in gen_content and "rust_decimal" not in content:
            additional_deps.append("rust_decimal = { workspace = true }")
        if "rusty-money" in gen_content and "rusty-money" not in content:
            additional_deps.append("rusty-money = { workspace = true }")

    # Add standard RERP dependencies that BRRTRouter template might not include
    standard_deps = [
        "serde_json = { workspace = true }",
        "serde_yaml = { workspace = true }",
        "config = { workspace = true }",
        "http = { workspace = true }",
        "may = { workspace = true }",
        "may_minihttp = { workspace = true }",
        "anyhow = { workspace = true }",
        "clap = { workspace = true }",
    ]

    for dep in standard_deps:
        dep_name = dep.split()[0]
        if dep_name not in content:
            additional_deps.append(dep)

    # Insert additional dependencies before closing [dependencies] section
    if additional_deps:
        # Find the last dependency line
        lines = content.split("\n")
        deps_end_idx = None
        for i, line in enumerate(lines):
            if line.strip() == "[dependencies]" or line.strip().startswith("[dependencies"):
                # Find where dependencies section ends
                for j in range(i + 1, len(lines)):
                    if lines[j].strip().startswith("[") and not lines[j].strip().startswith("#"):
                        deps_end_idx = j
                        break
                if deps_end_idx is None:
                    deps_end_idx = len(lines)
                break

        if deps_end_idx:
            # Insert additional deps
            for dep in reversed(additional_deps):
                lines.insert(deps_end_idx, dep)
            content = "\n".join(lines)

    cargo_path.write_text(content)


def _fix_impl_main_naming(main_path: Path, service_name: str) -> None:
    """Fix impl main.rs to use correct crate names."""
    if not main_path.exists():
        return

    service_snake = service_name.replace("-", "_")
    gen_crate_name = f"rerp_accounting_{service_snake}_gen"

    content = main_path.read_text()

    # Replace gen crate name in imports
    content = re.sub(
        r"use (\w+)::",
        f"use {gen_crate_name}::",
        content,
    )

    main_path.write_text(content)


def _create_impl_cargo_toml_fallback(cargo_path: Path, service_name: str) -> None:
    """Fallback impl Cargo.toml creation if BRRTRouter fails.

    This should only be used when BRRTRouter's generate-impl-stubs fails.
    """
    _create_impl_cargo_toml(cargo_path, service_name)


def generate_code_with_brrtrouter(spec_path: Path, output_dir: Path, project_root: Path) -> None:
    """Generate gen crate using BRRTRouter's generate command."""
    from rerp_tooling.gen.brrtrouter import call_brrtrouter_generate

    # Check for dependencies config
    deps_config_path = spec_path.parent / "brrtrouter-dependencies.toml"
    deps_config = deps_config_path if deps_config_path.exists() else None

    result = call_brrtrouter_generate(
        spec_path=spec_path,
        output_dir=output_dir,
        project_root=project_root,
        deps_config_path=deps_config,
        capture_output=True,
    )

    if result.returncode != 0:
        msg = f"BRRTRouter generation failed: {result.stderr}"
        raise RuntimeError(msg)

    print("✅ Code generation complete")


def run_bootstrap_microservice(
    service_name: str,
    port: Optional[int],
    project_root: Path,
    add_dependencies_config: bool = False,
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

    # Generate code to gen/ directory
    generate_code_with_brrtrouter(spec_path, gen_dir, project_root)

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

    # Optionally create brrtrouter-dependencies.toml alongside OpenAPI spec
    if add_dependencies_config:
        deps_config_path = spec_path.parent / "brrtrouter-dependencies.toml"
        if not deps_config_path.exists():
            create_dependencies_config_toml(deps_config_path)

    # Generate impl crate using BRRTRouter (removes duplication)
    # BRRTRouter handles: impl/Cargo.toml, impl/src/main.rs, impl/src/controllers/*.rs
    if not (impl_dir / "Cargo.toml").exists():
        _generate_impl_with_brrtrouter(spec_path, impl_dir, service_name, project_root)

    create_dockerfile(service_name, binary_name, port, dockerfile_path)
    update_workspace_cargo_toml(service_name, cargo_toml_path)
    update_tiltfile(service_name, spec_file, binary_name, port, tiltfile_path)

    print(f"✅ Bootstrap complete for {service_name}. Next: tilt up")
    return 0
