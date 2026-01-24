#!/usr/bin/env python3
"""
Bootstrap a new microservice crate from an OpenAPI specification.

This script:
1. Creates the crate directory structure
2. Generates code using BRRTRouter
3. Creates Dockerfile
4. Updates workspace Cargo.toml
5. Updates Tiltfile with lint/gen/deployment resources
6. Creates config/config.yaml template

**IDEMPOTENT**: Can be run multiple times safely - regenerates code but preserves user edits.
"""

import yaml
import re
import sys
import subprocess
from pathlib import Path
from typing import Dict, Any, Optional


def to_snake_case(name: str) -> str:
    """Convert a string to snake_case."""
    # Replace hyphens and spaces with underscores
    name = re.sub(r'[- ]+', '_', name)
    # Insert underscore before uppercase letters (but not at start)
    name = re.sub(r'(?<!^)(?<!_)([A-Z])', r'_\1', name)
    # Convert to lowercase
    return name.lower()


def to_pascal_case(name: str) -> str:
    """Convert a string to PascalCase."""
    return ''.join(word.capitalize() for word in name.split('-'))


def derive_binary_name(openapi_spec: Dict[str, Any], service_name: str) -> str:
    """Derive binary name from OpenAPI spec title or service name."""
    # Try to get from OpenAPI spec title
    title = openapi_spec.get('info', {}).get('title', '')
    if title:
        # Convert title to snake_case
        binary_name = to_snake_case(title)
        # Ensure it ends with _api or _service_api
        if not binary_name.endswith('_api'):
            if binary_name.endswith('_service'):
                binary_name = binary_name + '_api'
            else:
                binary_name = binary_name + '_service_api'
        return binary_name
    
    # Fallback: derive from service name
    return f"{service_name.replace('-', '_')}_service_api"


def load_openapi_spec(spec_path: Path) -> Dict[str, Any]:
    """Load OpenAPI spec from YAML file."""
    with open(spec_path, 'r') as f:
        return yaml.safe_load(f)


def create_dockerfile(service_name: str, binary_name: str, port: int, output_path: Path) -> None:
    """Create Dockerfile for the microservice."""
    dockerfile_content = f"""# Minimal runtime-only Dockerfile for {to_pascal_case(service_name)} Service (Tilt development)
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
    output_path.write_text(dockerfile_content)
    print(f"✅ Created Dockerfile: {output_path}")


def create_config_yaml(output_path: Path) -> None:
    """Create default config.yaml template."""
    config_content = """# BRRTRouter application configuration (YAML)
# This file is generated alongside the example application.
# Adjust values per environment and reload/restart the app.

security:
  # PropelAuth integration (recommended as first provider)
  # See PropelAuth docs: https://docs.propelauth.com/
  propelauth:
    # auth_url: "https://auth.yourdomain.com"   # REQUIRED - your PropelAuth Auth URL
    # audience: "your-api-audience"             # OPTIONAL - set if configured in PropelAuth
    # issuer: "https://auth.yourdomain.com/"    # OPTIONAL - defaults to auth_url
    # jwks_url: "https://auth.yourdomain.com/.well-known/jwks.json"  # OPTIONAL - derived from auth_url by default
    # leeway_secs: 30
    # cache_ttl_secs: 300

  # Static API keys bound to specific OpenAPI security scheme names
  # Example: 'ApiKeyAuth' corresponds to components.securitySchemes.ApiKeyAuth
  api_keys:
    # Development default for Pet Store example: OpenAPI scheme name is 'ApiKeyHeader'
    ApiKeyHeader:
      key: "test123"
      # header_name: "X-API-Key"  # optional override for header-based schemes

  # Remote API key verification by scheme name
  remote_api_keys:
    # ApiKeyAuth:
    #   verify_url: "https://auth.example/verify"
    #   timeout_ms: 500
    #   header_name: "X-API-Key"
    #   cache_ttl_secs: 60

  # Simple bearer signature (dev only) and optional cookie name
  bearer:
    # signature: "sig"
    # cookie_name: "auth_token"

  # Simple oauth2 signature (dev only) and optional cookie name
  oauth2:
    # signature: "sig"
    # cookie_name: "oauth_token"

  # Per-scheme JWKS configuration (production)
  jwks:
    # BearerAuth:
    #   jwks_url: "https://issuer.example/.well-known/jwks.json"
    #   iss: "https://issuer.example/"
    #   aud: "my-audience"
    #   leeway_secs: 30
    #   cache_ttl_secs: 300

http:
  # Enable HTTP/1.1 keep-alive (default true in generated apps for testing)
  keep_alive: true
  timeout_secs: 5
  max_requests: 5000
"""
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(config_content)
    print(f"✅ Created config.yaml: {output_path}")


def update_workspace_cargo_toml(service_name: str, cargo_toml_path: Path) -> None:
    """Add service to workspace members in Cargo.toml."""
    if not cargo_toml_path.exists():
        print(f"⚠️  Warning: {cargo_toml_path} not found, skipping workspace update")
        return
    
    content = cargo_toml_path.read_text()
    
    # Check if already added
    if f'"accounting/{service_name}"' in content:
        print(f"ℹ️  Service {service_name} already in workspace Cargo.toml")
        return
    
    # Find the members section and add the new service
    # Pattern: members = [ ... ]
    pattern = r'(members\s*=\s*\[)(.*?)(\])'
    match = re.search(pattern, content, re.DOTALL)
    
    if match:
        members_start = match.start(1)
        members_content = match.group(2)
        members_end = match.end(3)
        
        # Parse existing members
        existing_members = [m.strip().strip('"') for m in members_content.split(',') if m.strip()]
        
        # Add new member if not present
        if f'accounting/{service_name}' not in existing_members:
            existing_members.append(f'accounting/{service_name}')
            existing_members.sort()  # Keep sorted
            
            # Rebuild members list
            new_members = '    "' + '",\n    "'.join(existing_members) + '",\n'
            new_content = content[:members_start] + match.group(1) + '\n' + new_members + ']'
            
            cargo_toml_path.write_text(new_content)
            print(f"✅ Added {service_name} to workspace Cargo.toml")
        else:
            print(f"ℹ️  Service {service_name} already in workspace Cargo.toml")
    else:
        print(f"⚠️  Warning: Could not find members section in {cargo_toml_path}")


def update_tiltfile(service_name: str, spec_file: str, binary_name: str, port: int, tiltfile_path: Path) -> None:
    """Update Tiltfile with lint, gen, and deployment resources."""
    if not tiltfile_path.exists():
        print(f"⚠️  Warning: {tiltfile_path} not found, skipping Tiltfile update")
        return
    
    content = tiltfile_path.read_text()
    original_content = content
    
    # 1. Add to BINARY_NAMES dict
    binary_names_pattern = r"(BINARY_NAMES\s*=\s*\{)(.*?)(\})"
    match = re.search(binary_names_pattern, content, re.DOTALL)
    if match:
        binary_names_content = match.group(2)
        if f"'{service_name}':" not in binary_names_content and f'"{service_name}":' not in binary_names_content:
            # Add new entry (keep sorted)
            entries = []
            for line in binary_names_content.split('\n'):
                line = line.strip()
                if line and not line.startswith('#'):
                    entries.append(line)
            
            # Add new entry
            new_entry = f"    '{service_name}': '{binary_name}',"
            entries.append(new_entry)
            entries.sort()
            
            # Rebuild dict
            new_binary_names = match.group(1) + '\n' + '\n'.join(entries) + '\n}'
            content = content[:match.start()] + new_binary_names + content[match.end():]
            print(f"✅ Added {service_name} to BINARY_NAMES")
    
    # 2. Add lint resource call
    lint_call = f"create_microservice_lint('{service_name}', '{spec_file}')"
    if lint_call not in content:
        # Find the section with existing lint calls - look for the last one
        lint_section_pattern = r"(create_microservice_lint\('ftebe'[^\n]+\n)"
        lint_match = re.search(lint_section_pattern, content)
        if lint_match:
            # Add after the last lint call
            insert_pos = lint_match.end()
            content = content[:insert_pos] + lint_call + '\n' + content[insert_pos:]
            print(f"✅ Added lint resource for {service_name}")
        else:
            # Try to find any lint call and add after it
            any_lint_pattern = r"(create_microservice_lint\([^\n]+\n)"
            any_lint_match = list(re.finditer(any_lint_pattern, content))
            if any_lint_match:
                insert_pos = any_lint_match[-1].end()
                content = content[:insert_pos] + lint_call + '\n' + content[insert_pos:]
                print(f"✅ Added lint resource for {service_name}")
    
    # 3. Add gen resource call
    gen_call = f"create_microservice_gen('{service_name}', '{spec_file}', '{service_name}')"
    if gen_call not in content:
        # Find the section with existing gen calls - look for the last one
        gen_section_pattern = r"(create_microservice_gen\('ftebe'[^\n]+\n)"
        gen_match = re.search(gen_section_pattern, content)
        if gen_match:
            # Add after the last gen call
            insert_pos = gen_match.end()
            content = content[:insert_pos] + gen_call + '\n' + content[insert_pos:]
            print(f"✅ Added gen resource for {service_name}")
        else:
            # Try to find any gen call and add after it
            any_gen_pattern = r"(create_microservice_gen\([^\n]+\n)"
            any_gen_match = list(re.finditer(any_gen_pattern, content))
            if any_gen_match:
                insert_pos = any_gen_match[-1].end()
                content = content[:insert_pos] + gen_call + '\n' + content[insert_pos:]
                print(f"✅ Added gen resource for {service_name}")
    
    # 4. Add to build-workspace deps
    build_workspace_pattern = r"(resource_deps=\[)(.*?)(\]\s*labels=\['microservices-build'\])"
    build_match = re.search(build_workspace_pattern, content, re.DOTALL)
    if build_match:
        deps_content = build_match.group(2)
        if f"'{service_name}-service-gen'" not in deps_content:
            # Add to deps list
            deps = [d.strip().strip("'\"") for d in deps_content.split(',') if d.strip()]
            deps.append(f'{service_name}-service-gen')
            deps.sort()
            new_deps = build_match.group(1) + "'" + "', '".join(deps) + "',\n    "
            content = content[:build_match.start()] + new_deps + content[build_match.end():]
            print(f"✅ Added {service_name} to build-workspace deps")
    
    # Also add Cargo.toml to deps
    cargo_deps_pattern = r"(deps=\[)(.*?)(\]\s*resource_deps=)"
    cargo_match = re.search(cargo_deps_pattern, content, re.DOTALL)
    if cargo_match:
        cargo_deps_content = cargo_match.group(2)
        if f"'./microservices/accounting/{service_name}/Cargo.toml'" not in cargo_deps_content:
            cargo_deps = [d.strip().strip("'\"") for d in cargo_deps_content.split(',') if d.strip()]
            cargo_deps.append(f"./microservices/accounting/{service_name}/Cargo.toml")
            cargo_deps.sort()
            new_cargo_deps = cargo_match.group(1) + "'" + "',\n        '".join(cargo_deps) + "',\n    "
            content = content[:cargo_match.start()] + new_cargo_deps + content[cargo_match.end():]
            print(f"✅ Added {service_name} Cargo.toml to build-workspace deps")
    
    # 5. Add to get_service_port function
    port_pattern = r"(ports\s*=\s*\{)(.*?)(\s*\})"
    port_match = re.search(port_pattern, content)
    if port_match:
        ports_content = port_match.group(2)
        if f"'{service_name}':" not in ports_content:
            # Add port entry
            port_entry = f"        '{service_name}': '{port}',"
            # Insert before the closing brace, keeping alphabetical order
            lines = [l.strip() for l in ports_content.split('\n') if l.strip() and not l.strip().startswith('#')]
            lines.append(port_entry)
            lines.sort()
            new_ports = port_match.group(1) + '\n' + '\n'.join(lines) + '\n    '
            content = content[:port_match.start()] + new_ports + content[port_match.end():]
            print(f"✅ Added port {port} for {service_name}")
    
    # 6. Add deployment call
    deployment_call = f"create_microservice_deployment('{service_name}')"
    if deployment_call not in content:
        # Find the section with existing deployment calls - look for the last one
        deployment_section_pattern = r"(create_microservice_deployment\('bff'[^\n]+\n)"
        deployment_match = re.search(deployment_section_pattern, content)
        if deployment_match:
            # Add after the last deployment call
            insert_pos = deployment_match.end()
            content = content[:insert_pos] + deployment_call + '\n' + content[insert_pos:]
            print(f"✅ Added deployment resource for {service_name}")
        else:
            # Try to find any deployment call and add after it
            any_deployment_pattern = r"(create_microservice_deployment\([^\n]+\n)"
            any_deployment_match = list(re.finditer(any_deployment_pattern, content))
            if any_deployment_match:
                insert_pos = any_deployment_match[-1].end()
                content = content[:insert_pos] + deployment_call + '\n' + content[insert_pos:]
                print(f"✅ Added deployment resource for {service_name}")
    
    if content != original_content:
        tiltfile_path.write_text(content)
        print(f"✅ Updated Tiltfile")
    else:
        print(f"ℹ️  Tiltfile already contains all necessary entries for {service_name}")


def generate_code_with_brrtrouter(spec_path: Path, output_dir: Path) -> None:
    """Generate code using BRRTRouter."""
    print(f"🔄 Generating code with BRRTRouter...")
    
    # Get project root (assuming script is in scripts/ directory)
    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    
    # Try to use the built binary first (faster)
    # BRRTRouter is at the same level as RERP (../BRRTRouter from RERP root)
    brrtrouter_bin = project_root.parent / "BRRTRouter" / "target" / "debug" / "brrtrouter-gen"
    
    if not brrtrouter_bin.exists():
        # Fall back to cargo run
        brrtrouter_manifest = project_root.parent / "BRRTRouter" / "Cargo.toml"
        if not brrtrouter_manifest.exists():
            print(f"❌ Error: BRRTRouter not found at {brrtrouter_manifest.parent}")
            print(f"   Please ensure BRRTRouter is cloned at the same level as RERP")
            raise FileNotFoundError(f"BRRTRouter Cargo.toml not found: {brrtrouter_manifest}")
        
        cmd = [
            "cargo", "run", "--manifest-path", str(brrtrouter_manifest),
            "--bin", "brrtrouter-gen", "--",
            "generate",
            "--spec", str(spec_path),
            "--output", str(output_dir),
            "--force"
        ]
    else:
        cmd = [
            str(brrtrouter_bin),
            "generate",
            "--spec", str(spec_path),
            "--output", str(output_dir),
            "--force"
        ]
    
    try:
        result = subprocess.run(cmd, check=True, capture_output=True, text=True, cwd=project_root)
        print(f"✅ Code generation complete")
        if result.stdout:
            print(result.stdout)
    except subprocess.CalledProcessError as e:
        print(f"❌ Error generating code: {e}")
        if e.stderr:
            print(f"Error output: {e.stderr}")
        raise


def fix_cargo_toml_paths(cargo_toml_path: Path) -> None:
    """Fix BRRTRouter paths in generated Cargo.toml."""
    if not cargo_toml_path.exists():
        return
    
    # Run the fix script
    fix_script = Path("scripts/fix_cargo_toml_paths.py")
    if fix_script.exists():
        subprocess.run(["python3", str(fix_script), str(cargo_toml_path)], check=False)
    else:
        print(f"⚠️  Warning: {fix_script} not found, skipping Cargo.toml path fix")


def bootstrap_microservice(service_name: str, spec_file: str, port: int) -> None:
    """Bootstrap a new microservice."""
    print(f"🚀 Bootstrapping microservice: {service_name}")
    print(f"   Spec: {spec_file}")
    print(f"   Port: {port}")
    
    # Paths
    project_root = Path(__file__).parent.parent  # scripts/ -> rerp/
    spec_path = project_root / "openapi" / "accounting" / service_name / "openapi.yaml"
    crate_dir = project_root / "microservices" / "accounting" / service_name
    dockerfile_path = project_root / "docker" / "microservices" / f"Dockerfile.{service_name}"
    config_path = crate_dir / "config" / "config.yaml"
    cargo_toml_path = project_root / "microservices" / "Cargo.toml"
    tiltfile_path = project_root / "Tiltfile"
    
    # Validate spec file exists
    if not spec_path.exists():
        print(f"❌ Error: OpenAPI spec not found: {spec_path}")
        sys.exit(1)
    
    # Load OpenAPI spec to get binary name
    openapi_spec = load_openapi_spec(spec_path)
    binary_name = derive_binary_name(openapi_spec, service_name)
    print(f"   Binary name: {binary_name}")
    
    # 1. Generate code with BRRTRouter (creates crate directory and files)
    generate_code_with_brrtrouter(spec_path, crate_dir)
    
    # 2. Fix Cargo.toml paths
    cargo_toml = crate_dir / "Cargo.toml"
    if cargo_toml.exists():
        fix_cargo_toml_paths(cargo_toml)
    
    # 3. Create config.yaml (if not exists, BRRTRouter might have created it)
    if not config_path.exists():
        create_config_yaml(config_path)
    
    # 4. Create Dockerfile
    create_dockerfile(service_name, binary_name, port, dockerfile_path)
    
    # 5. Update workspace Cargo.toml
    update_workspace_cargo_toml(service_name, cargo_toml_path)
    
    # 6. Update Tiltfile
    update_tiltfile(service_name, spec_file, binary_name, port, tiltfile_path)
    
    print(f"\n✅ Bootstrap complete for {service_name}!")
    print(f"   Next steps:")
    print(f"   1. Review generated code in: {crate_dir}")
    print(f"   2. Implement business logic in controllers/")
    print(f"   3. Update config/config.yaml as needed")
    print(f"   4. Run 'tilt up' to start development")


def main():
    """Main entry point."""
    if len(sys.argv) < 2:
        print("Usage: bootstrap_microservice.py <service-name> [port]")
        print("")
        print("Example:")
        print("  bootstrap_microservice.py general-ledger")
        print("  bootstrap_microservice.py invoice 8002")
        print("")
        print("Arguments:")
        print("  service-name: Short name for the service (e.g., 'general-ledger', 'invoice')")
        print("  port:         Optional port number (if not provided, will use port registry)")
        print("")
        print("Note: OpenAPI spec is expected at: openapi/accounting/<service-name>/openapi.yaml")
        sys.exit(1)
    
    service_name = sys.argv[1]
    
    # Get port from registry or command line
    port = None
    if len(sys.argv) >= 3:
        try:
            port = int(sys.argv[2])
        except ValueError:
            print(f"❌ Error: Invalid port number: {sys.argv[2]}")
            sys.exit(1)
    else:
        # Try to get port from registry
        import json
        registry_file = Path(__file__).parent / "port-registry.json"
        if registry_file.exists():
            with open(registry_file, 'r') as f:
                registry = json.load(f)
                port = registry.get("assignments", {}).get(service_name)
        
        if not port:
            print(f"⚠️  Warning: No port found for {service_name} in registry")
            print(f"   Assign a port first: ./scripts/assign-port.py assign {service_name} --update-configs")
            print(f"   Or provide port as second argument")
            sys.exit(1)
    
    # Spec file is determined by service name
    spec_file = f"{service_name}/openapi.yaml"
    
    try:
        bootstrap_microservice(service_name, spec_file, port)
    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == '__main__':
    main()

