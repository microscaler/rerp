"""Generate service-specific Dockerfile from docker/microservices/Dockerfile.template."""

from __future__ import annotations

import sys
from pathlib import Path
from typing import Optional


def generate_dockerfile(
    system: str,
    module: str,
    port: int = 8000,
    project_root: Optional[Path] = None,
    template_path: Optional[Path] = None,
    output_path: Optional[Path] = None,
) -> Path:
    """
    Generate a Dockerfile for a specific service from the template.
    Writes to docker/microservices/Dockerfile.{system}_{module} unless output_path is set.
    Returns the output path.
    """
    root = Path(project_root) if project_root is not None else Path.cwd()
    tpl = template_path or (root / "docker" / "microservices" / "Dockerfile.template")
    out = output_path or (root / "docker" / "microservices" / f"Dockerfile.{system}_{module}")

    if not tpl.exists():
        print(f"❌ Error: Template not found: {tpl}", file=sys.stderr)
        sys.exit(1)

    binary_name = f"rerp_{system}_{module.replace('-', '_')}_impl"
    content = tpl.read_text()
    content = content.replace("{{service_name}}", f"{system}-{module}")
    content = content.replace("{{binary_name}}", binary_name)
    content = content.replace("{{system}}", system)
    content = content.replace("{{module}}", module)
    content = content.replace("{{port}}", str(port))

    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(content)
    print(f"✅ Generated: {out}")
    return out


def run(system: str, module: str, port: int = 8000, project_root: Optional[Path] = None) -> int:
    """CLI entry: generate Dockerfile. Returns 0."""
    generate_dockerfile(system, module, port=port, project_root=project_root)
    return 0
