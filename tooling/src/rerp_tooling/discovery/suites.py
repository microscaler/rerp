"""Suite and BFF discovery from openapi/{suite}/ and bff-suite-config.yaml."""

import logging
from collections.abc import Iterator
from pathlib import Path
from typing import Optional

import yaml

log = logging.getLogger(__name__)

# Optional: allow callers to avoid yaml import if only using path helpers


def _openapi_dir(project_root: Path) -> Path:
    return project_root / "openapi"


def suites_with_bff(project_root: Path) -> list[str]:
    """Suites that have a BFF: openapi/{suite}/bff-suite-config.yaml exists."""
    d = _openapi_dir(project_root)
    if not d.exists():
        return []
    return [x.name for x in d.iterdir() if x.is_dir() and (x / "bff-suite-config.yaml").exists()]


def bff_suite_config_path(project_root: Path, suite: str) -> Path:
    return _openapi_dir(project_root) / suite / "bff-suite-config.yaml"


def openapi_bff_path(project_root: Path, suite: str) -> Path:
    return _openapi_dir(project_root) / suite / "openapi_bff.yaml"


def service_to_suite(project_root: Path, service_name: str) -> Optional[str]:
    """Return the suite that contains openapi/{suite}/{service_name}/openapi.yaml, or None."""
    d = _openapi_dir(project_root)
    if not d.exists():
        return None
    for x in d.iterdir():
        if x.is_dir() and (x / service_name / "openapi.yaml").exists():
            return x.name
    return None


def get_bff_service_name_from_config(data: dict) -> Optional[str]:
    """Read bff_service_name from bff-suite-config (top-level or metadata). Returns None if absent."""
    return data.get("bff_service_name") or (data.get("metadata") or {}).get("bff_service_name")


def iter_bffs(project_root: Path) -> Iterator[tuple[str, str]]:
    """Yield (bff_service_name, suite) for each suite with bff-suite-config.yaml and bff_service_name set."""
    for suite in suites_with_bff(project_root):
        path = bff_suite_config_path(project_root, suite)
        try:
            with path.open() as f:
                data = yaml.safe_load(f) or {}
            name = get_bff_service_name_from_config(data)
            if name:
                yield (name, suite)
        except (OSError, yaml.YAMLError, KeyError, TypeError, ValueError) as e:
            log.debug("Could not parse bff_suite_config %s: %s", path, e)


def bff_service_to_suite(project_root: Path, service_name: str) -> Optional[str]:
    """Return the suite whose BFF has the given registry service name, or None."""
    for bff_svc, suite in iter_bffs(project_root):
        if bff_svc == service_name:
            return suite
    return None


def load_suite_services(project_root: Path) -> set:
    """Services that keep their port in fix-duplicates: BFF names and bff-suite-config services."""
    keep: set = set()
    for bff_svc, _ in iter_bffs(project_root):
        keep.add(bff_svc)
    for suite in suites_with_bff(project_root):
        path = bff_suite_config_path(project_root, suite)
        try:
            with path.open() as f:
                data = yaml.safe_load(f) or {}
            keep |= set((data.get("services") or {}).keys())
        except (OSError, yaml.YAMLError, KeyError, TypeError, ValueError) as e:
            log.debug("Could not parse bff_suite_config %s: %s", path, e)
    return keep


def suite_sub_service_names(project_root: Path, suite: str) -> list[str]:
    """Sub-service names for a suite: openapi/{suite}/{name}/openapi.yaml exists. Sorted. Excludes BFF."""
    d = _openapi_dir(project_root) / suite
    if not d.exists() or not d.is_dir():
        return []
    return sorted(x.name for x in d.iterdir() if x.is_dir() and (x / "openapi.yaml").exists())


def iter_suite_services(project_root: Path) -> Iterator[tuple[str, str]]:
    """Yield (suite, service_name) for every openapi/{suite}/{service_name}/openapi.yaml. No hardcoding."""
    d = _openapi_dir(project_root)
    if not d.exists() or not d.is_dir():
        return
    for suite_dir in sorted(d.iterdir()):
        if not suite_dir.is_dir():
            continue
        suite = suite_dir.name
        for name in suite_sub_service_names(project_root, suite):
            yield (suite, name)


def tilt_service_names(project_root: Path) -> list[str]:
    """Service names that Tilt runs (containers rerp-{name}-dev, images rerp-accounting-{name}:*). Sorted. From bff-suite-config + BFFs."""
    return sorted(load_suite_services(project_root))
