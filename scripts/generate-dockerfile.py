#!/usr/bin/env python3
"""
Generate service-specific Dockerfile from template.

Usage:
    python3 scripts/generate-dockerfile.py <system> <module> [port]

Example:
    python3 scripts/generate-dockerfile.py auth idam 8000
"""

import sys
import os
from pathlib import Path

def generate_dockerfile(system: str, module: str, port: int = 8000):
    """Generate a Dockerfile for a specific service."""
    
    # Convert module name to binary name format
    binary_name = f"rerp_{system}_{module.replace('-', '_')}_impl"
    
    # Paths
    template_path = Path("docker/microservices/Dockerfile.template")
    output_path = Path(f"docker/microservices/Dockerfile.{system}_{module}")
    
    if not template_path.exists():
        print(f"❌ Error: Template not found: {template_path}", file=sys.stderr)
        sys.exit(1)
    
    # Read template
    with open(template_path, 'r') as f:
        template = f.read()
    
    # Replace template variables
    dockerfile_content = template.replace("{{service_name}}", f"{system}-{module}")
    dockerfile_content = dockerfile_content.replace("{{binary_name}}", binary_name)
    dockerfile_content = dockerfile_content.replace("{{system}}", system)
    dockerfile_content = dockerfile_content.replace("{{module}}", module)
    dockerfile_content = dockerfile_content.replace("{{port}}", str(port))
    
    # Write output
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, 'w') as f:
        f.write(dockerfile_content)
    
    print(f"✅ Generated: {output_path}")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("usage: generate-dockerfile.py <system> <module> [port]", file=sys.stderr)
        print("  example: generate-dockerfile.py auth idam 8000", file=sys.stderr)
        sys.exit(1)
    
    system = sys.argv[1]
    module = sys.argv[2]
    port = int(sys.argv[3]) if len(sys.argv) > 3 else 8000
    
    generate_dockerfile(system, module, port)
