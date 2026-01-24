"""Bootstrap a new microservice crate from an OpenAPI specification.

Creates crate (via BRRTRouter), Dockerfile, config, workspace Cargo.toml, Tiltfile.
Idempotent: safe to run multiple times.
"""

from __future__ import annotations

import json
import os
import re
import subprocess
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
COPY ./microservices/accounting/{service_name}/config /app/config
COPY ./microservices/accounting/{service_name}/doc /app/doc
COPY ./microservices/accounting/{service_name}/static_site /app/static_site

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
    if f'"accounting/{service_name}"' in content:
        return
    m = re.search(r"(members\s*=\s*\[)(.*?)(\])", content, re.DOTALL)
    if not m:
        return
    existing = [x.strip().strip('"') for x in m.group(2).split(",") if x.strip()]
    if f"accounting/{service_name}" in existing:
        return
    existing.append(f"accounting/{service_name}")
    existing.sort()
    new_members = '    "' + '",\n    "'.join(existing) + '",\n'
    new_content = content[: m.start()] + m.group(1) + "\n" + new_members + "]" + content[m.end() :]
    cargo_toml_path.write_text(new_content)
    print(f"✅ Added {service_name} to workspace Cargo.toml")


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
    if m and f"'./microservices/accounting/{service_name}/Cargo.toml'" not in m.group(2):
        deps = [d.strip().strip("'\"") for d in m.group(2).split(",") if d.strip()]
        deps.append(f"./microservices/accounting/{service_name}/Cargo.toml")
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


def generate_code_with_brrtrouter(spec_path: Path, output_dir: Path, project_root: Path) -> None:
    brrtrouter_bin = project_root.parent / "BRRTRouter" / "target" / "debug" / "brrtrouter-gen"
    manifest = project_root.parent / "BRRTRouter" / "Cargo.toml"
    if not manifest.exists():
        msg = f"BRRTRouter not found at {manifest.parent}. Clone at same level as RERP."
        raise FileNotFoundError(msg)
    if brrtrouter_bin.exists():
        cmd = [
            str(brrtrouter_bin),
            "generate",
            "--spec",
            str(spec_path),
            "--output",
            str(output_dir),
            "--force",
        ]
    else:
        cmd = [
            "cargo",
            "run",
            "--manifest-path",
            str(manifest),
            "--bin",
            "brrtrouter-gen",
            "--",
            "generate",
            "--spec",
            str(spec_path),
            "--output",
            str(output_dir),
            "--force",
        ]
    subprocess.run(cmd, check=True, capture_output=True, text=True, cwd=str(project_root))
    print("✅ Code generation complete")


def run_bootstrap_microservice(service_name: str, port: Optional[int], project_root: Path) -> int:
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
    dockerfile_path = project_root / "docker" / "microservices" / f"Dockerfile.{service_name}"
    config_path = crate_dir / "config" / "config.yaml"
    cargo_toml_path = project_root / "microservices" / "Cargo.toml"
    tiltfile_path = project_root / "Tiltfile"

    if not spec_path.exists():
        print(f"❌ OpenAPI spec not found: {spec_path}")
        return 1

    openapi_spec = load_openapi_spec(spec_path)
    binary_name = derive_binary_name(openapi_spec, service_name)
    print(f"🚀 Bootstrapping {service_name} (port {port}, binary {binary_name})")

    generate_code_with_brrtrouter(spec_path, crate_dir, project_root)

    crate_cargo = crate_dir / "Cargo.toml"
    if crate_cargo.exists():
        from rerp_tooling.ci.fix_cargo_paths import run as run_fix_cargo_paths

        run_fix_cargo_paths(crate_cargo, project_root)

    if not config_path.exists():
        create_config_yaml(config_path)
    create_dockerfile(service_name, binary_name, port, dockerfile_path)
    update_workspace_cargo_toml(service_name, cargo_toml_path)
    update_tiltfile(service_name, spec_file, binary_name, port, tiltfile_path)

    print(f"✅ Bootstrap complete for {service_name}. Next: tilt up")
    return 0
