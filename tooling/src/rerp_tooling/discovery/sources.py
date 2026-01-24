"""Discover port usages from helm, kind-config, Tiltfile, bff-suite-config, openapi."""

import logging
import re
from pathlib import Path

import yaml

log = logging.getLogger(__name__)


def _openapi_dir(project_root: Path) -> Path:
    return project_root / "openapi"


def _iter_bffs(project_root: Path):
    from .suites import iter_bffs

    return iter_bffs(project_root)


def discover_helm(project_root: Path) -> dict[str, int]:
    """Discover service.port from helm/rerp-microservice/values/*.yaml."""
    out: dict[str, int] = {}
    d = project_root / "helm" / "rerp-microservice" / "values"
    if not d.exists():
        return out
    for p in sorted(d.glob("*.yaml")):
        try:
            with p.open() as f:
                data = yaml.safe_load(f) or {}
            svc = data.get("service") or {}
            port = svc.get("port")
            name = svc.get("name") or p.stem
            if port is not None:
                out[name] = int(port)
        except Exception as e:  # noqa: BLE001
            log.warning("Could not parse helm values %s: %s", p, e)
    return out


def discover_kind_host_ports(project_root: Path) -> list[tuple[int, str]]:
    """Discover hostPort from kind-config extraPortMappings. Returns [(host_port, comment_or_container_port)]."""
    out: list[tuple[int, str]] = []
    p = project_root / "kind-config.yaml"
    if not p.exists():
        return out
    try:
        with p.open() as f:
            data = yaml.safe_load(f) or {}
        for n in data.get("nodes") or []:
            for m in n.get("extraPortMappings") or []:
                hp = m.get("hostPort")
                if hp is not None:
                    cp = m.get("containerPort", "")
                    out.append((int(hp), str(cp)))
    except Exception as e:  # noqa: BLE001
        log.warning("Could not parse kind-config %s: %s", p, e)
    return out


def discover_tiltfile(project_root: Path) -> dict[str, int]:
    """Discover get_service_port dict from Tiltfile (starlark)."""
    out: dict[str, int] = {}
    p = project_root / "Tiltfile"
    if not p.exists():
        return out
    try:
        with p.open() as f:
            text = f.read()
        m = re.search(r"ports\s*=\s*\{([^}]+)\}", text, re.DOTALL)
        if m:
            for m2 in re.finditer(r"'([^']+)'\s*:\s*'(\d+)'", m.group(1)):
                out[m2.group(1)] = int(m2.group(2))
    except Exception as e:  # noqa: BLE001
        log.warning("Could not parse Tiltfile %s: %s", p, e)
    return out


def discover_bff_suite_config(project_root: Path) -> dict[str, int]:
    """Discover services.*.port from all openapi/{suite}/bff-suite-config.yaml."""
    from .suites import bff_suite_config_path, suites_with_bff

    out: dict[str, int] = {}
    for suite in suites_with_bff(project_root):
        path = bff_suite_config_path(project_root, suite)
        try:
            with path.open() as f:
                data = yaml.safe_load(f) or {}
            for name, cfg in (data.get("services") or {}).items():
                if isinstance(cfg, dict) and "port" in cfg:
                    out[name] = int(cfg["port"])
        except Exception as e:  # noqa: BLE001
            log.warning("Could not parse bff_suite_config %s: %s", path, e)
    return out


def discover_openapi_bff_localhost(project_root: Path) -> dict[str, tuple[int, str]]:
    """Extract localhost port from openapi/{suite}/openapi_bff.yaml per BFF. Returns {bff_service_name: (port, suite)}."""
    from .suites import openapi_bff_path

    out: dict[str, tuple[int, str]] = {}
    for bff_svc, suite in _iter_bffs(project_root):
        path = openapi_bff_path(project_root, suite)
        if not path.exists():
            continue
        try:
            with path.open() as f:
                data = yaml.safe_load(f) or {}
            for s in data.get("servers") or []:
                if isinstance(s, dict):
                    u = str(s.get("url", ""))
                    if "localhost" in u:
                        m = re.search(r":(\d+)(?:/|$)", u)
                        if m:
                            out[bff_svc] = (int(m.group(1)), suite)
                        break
        except (
            OSError,
            yaml.YAMLError,
            KeyError,
            TypeError,
            ValueError,
            re.error,
        ) as e:
            log.debug("Could not parse openapi_bff %s: %s", path, e)
    return out


def discover_openapi_suite_microservice_localhost(
    project_root: Path,
) -> dict[str, tuple[str, int]]:
    """Extract localhost ports from openapi/{suite}/{name}/openapi.yaml. Returns {service_name: (suite, port)}."""
    out: dict[str, tuple[str, int]] = {}
    d = _openapi_dir(project_root)
    if not d.exists():
        return out
    for suite_dir in d.iterdir():
        if not suite_dir.is_dir():
            continue
        for sdir in suite_dir.iterdir():
            if not sdir.is_dir():
                continue
            spec = sdir / "openapi.yaml"
            if not spec.exists():
                continue
            try:
                with spec.open() as f:
                    data = yaml.safe_load(f) or {}
                for s in data.get("servers") or []:
                    if isinstance(s, dict):
                        u = str(s.get("url", ""))
                        if "localhost" in u:
                            m = re.search(r":(\d+)(?:/|$)", u)
                            if m:
                                out[sdir.name] = (suite_dir.name, int(m.group(1)))
                            break
            except (
                OSError,
                yaml.YAMLError,
                KeyError,
                TypeError,
                ValueError,
                re.error,
            ) as e:
                log.debug("Could not parse openapi %s: %s", spec, e)
    return out
