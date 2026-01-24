"""Validate OpenAPI YAML specs under openapi_dir."""

from pathlib import Path

import yaml


def validate_specs(openapi_dir: Path) -> list[tuple[Path, Exception]]:
    """
    Find all openapi.yaml under openapi_dir (rglob), load each with yaml.safe_load.
    Returns [(path, error)] for each invalid file (syntax error or root not a dict).
    Returns [] if openapi_dir does not exist or all specs are valid.
    """
    if not openapi_dir.exists():
        return []
    errors: list[tuple[Path, Exception]] = []
    for spec in sorted(openapi_dir.rglob("openapi.yaml")):
        try:
            with spec.open() as f:
                data = yaml.safe_load(f)
            if not isinstance(data, dict):
                errors.append((spec, ValueError(f"root is {type(data).__name__}, expected dict")))
        except (OSError, yaml.YAMLError, KeyError, TypeError, ValueError) as e:
            errors.append((spec, e))
    return errors
