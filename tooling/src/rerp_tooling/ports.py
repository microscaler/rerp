"""Port registry: assignments, update_config_files, validate, reconcile, fix_duplicates."""

import json
import logging
import re
from collections import defaultdict
from datetime import datetime
from pathlib import Path
from typing import Optional

import yaml

from .discovery import (
    bff_service_to_suite,
    bff_suite_config_path,
    discover_bff_suite_config,
    discover_helm,
    discover_kind_host_ports,
    discover_openapi_bff_localhost,
    discover_openapi_suite_microservice_localhost,
    discover_tiltfile,
    get_bff_service_name_from_config,
    iter_bffs,
    load_suite_services,
    service_to_suite,
    suites_with_bff,
)

log = logging.getLogger(__name__)
START_PORT = 8001
RESERVED_PORTS = [8080]
TILT_MANAGED_RANGE = (8001, 8099)


def _update_openapi_servers(service_name: str, port: int, project_root: Path) -> None:
    """Update OpenAPI server URLs for Swagger 'Try it' to use the correct localhost port."""
    openapi_dir = project_root / "openapi"
    suite = bff_service_to_suite(project_root, service_name)
    if suite is not None:
        path = bff_suite_config_path(project_root, suite)
        if not path.exists():
            return
        with path.open() as f:
            data = yaml.safe_load(f) or {}
        meta = data.setdefault("metadata", {})
        servers = meta.get("servers") or []
        localhost_url = f"http://localhost:{port}"
        found = False
        for s in servers:
            if isinstance(s, dict) and "localhost" in str(s.get("url", "")):
                s["url"] = localhost_url
                found = True
                break
        if not found:
            servers.append({"url": localhost_url, "description": "Local development server (BFF)"})
        meta["servers"] = servers
        with path.open("w") as f:
            yaml.dump(data, f, default_flow_style=False, sort_keys=False)
        print(f"✅ Updated {path} (BFF localhost server -> {localhost_url})")
        return

    suite = service_to_suite(project_root, service_name)
    if not suite:
        return
    spec_path = openapi_dir / suite / service_name / "openapi.yaml"
    if not spec_path.exists():
        return
    with spec_path.open() as f:
        data = yaml.safe_load(f) or {}
    servers = data.get("servers") or []
    localhost_url = f"http://localhost:{port}/api/v1/{suite}/{service_name}"
    desc = "Local development (direct to service, port from port-registry)"
    new_list = []
    replaced = False
    for s in servers:
        if isinstance(s, dict) and "localhost" in str(s.get("url", "")):
            if not replaced:
                new_list.append({"url": localhost_url, "description": desc})
                replaced = True
        else:
            new_list.append(s)
    if not replaced:
        new_list.insert(0, {"url": localhost_url, "description": desc})
    data["servers"] = new_list
    with spec_path.open("w") as f:
        yaml.dump(data, f, default_flow_style=False, sort_keys=False)
    print(f"✅ Updated {spec_path} (localhost server -> {localhost_url})")


class PortRegistry:
    """Manages port assignments for RERP services."""

    def __init__(self, registry_file: Path, project_root: Path):
        self.registry_file = registry_file
        self.project_root = project_root
        self.registry = self._load_registry()

    def _load_registry(self) -> dict:
        if not self.registry_file.exists():
            return {
                "version": "1.0",
                "next_port": START_PORT,
                "reserved_ports": RESERVED_PORTS,
                "assignments": {},
                "metadata": {
                    "description": "Port registry for RERP microservices",
                    "last_updated": None,
                    "notes": "Ports start at 8001. Port 8080 is reserved due to conflicts.",
                },
            }
        with self.registry_file.open() as f:
            return json.load(f)

    def _save_registry(self) -> None:
        self.registry["metadata"]["last_updated"] = datetime.now().isoformat()
        with self.registry_file.open("w") as f:
            json.dump(self.registry, f, indent=2)

    def _find_next_available_port(self) -> int:
        assigned = set(self.registry["assignments"].values())
        reserved = set(self.registry.get("reserved_ports", RESERVED_PORTS))
        port = self.registry.get("next_port", START_PORT)
        while port in assigned or port in reserved:
            port += 1
        return port

    def assign_port(
        self,
        service_name: str,
        force: bool = False,
        preferred_port: Optional[int] = None,
    ) -> tuple[int, bool]:
        assignments = self.registry["assignments"]
        assigned = set(assignments.values())
        reserved = set(self.registry.get("reserved_ports", RESERVED_PORTS))
        if service_name in assignments and not force:
            return assignments[service_name], False
        if (
            preferred_port is not None
            and preferred_port not in assigned
            and preferred_port not in reserved
        ):
            port = preferred_port
        else:
            port = self._find_next_available_port()
        assignments[service_name] = port
        self.registry["next_port"] = max(self.registry.get("next_port", START_PORT), port + 1)
        self._save_registry()
        return port, True

    def release_port(self, service_name: str) -> Optional[int]:
        assignments = self.registry["assignments"]
        if service_name not in assignments:
            return None
        port = assignments.pop(service_name)
        self._save_registry()
        return port

    def get_port(self, service_name: str) -> Optional[int]:
        return self.registry["assignments"].get(service_name)

    def list_assignments(self) -> dict[str, int]:
        return self.registry["assignments"].copy()

    def update_config_files(self, service_name: str, port: int, port_only: bool = False) -> None:
        node_port = 31000 + (port - 8000)
        values_file = (
            self.project_root / "helm" / "rerp-microservice" / "values" / f"{service_name}.yaml"
        )
        if not values_file.exists():
            print(f"⚠️  Helm values file not found: {values_file}")
            print("   Create it with: rerp bootstrap microservice <service>")
            return
        with values_file.open() as f:
            values = yaml.safe_load(f) or {}
        if "service" not in values:
            values["service"] = {}
        values["service"]["name"] = service_name
        values["service"]["port"] = port
        values["service"]["containerPort"] = port
        values["service"]["nodePort"] = node_port
        if not port_only:
            if "image" not in values:
                values["image"] = {}
            values["image"]["name"] = f"rerp-{service_name}"
            if "app" not in values:
                values["app"] = {}
            values["app"]["serviceName"] = service_name
            values["app"]["binaryName"] = service_name.replace("-", "_") + "_service_api"
        with values_file.open("w") as f:
            yaml.dump(values, f, default_flow_style=False, sort_keys=False)
        print(f"✅ Updated {values_file}")
        _update_openapi_servers(service_name, port, self.project_root)
        if port_only:
            return
        kind_config = self.project_root / "kind-config.yaml"
        if TILT_MANAGED_RANGE[0] <= port <= TILT_MANAGED_RANGE[1]:
            print(
                f"Info:  Skipping kind-config: port {port} in Tilt-managed range {TILT_MANAGED_RANGE[0]}-{TILT_MANAGED_RANGE[1]} (Tilt port-forwards)"
            )
        elif kind_config.exists():
            with kind_config.open() as f:
                content = f.read()
            if f"hostPort: {port}" not in content:
                lines = content.split("\n")
                insert_index = None
                for i, line in enumerate(lines):
                    if "extraPortMappings:" in line:
                        j = i + 1
                        while j < len(lines) and (
                            lines[j].strip().startswith("#")
                            or lines[j].strip().startswith("-")
                            or "containerPort" in lines[j]
                            or "hostPort" in lines[j]
                            or "protocol" in lines[j]
                        ):
                            j += 1
                        insert_index = j
                        break
                if insert_index is not None:
                    indent = "  "
                    new_lines = [
                        indent + f"# {service_name.replace('-', ' ').title()} Service",
                        indent + f"- containerPort: {node_port}",
                        indent + f"  hostPort: {port}",
                        indent + "  protocol: TCP",
                    ]
                    lines[insert_index:insert_index] = new_lines
                    with kind_config.open("w") as f:
                        f.write("\n".join(lines))
                    print(f"✅ Updated {kind_config}")
            else:
                print("Info:  Port mapping already exists in kind-config.yaml")


def _validate_helm_duplicates(helm: dict[str, int], errors: list[str]) -> None:
    by_port: dict[int, list[str]] = defaultdict(list)
    for svc, port in helm.items():
        by_port[port].append(svc)
    for port, svcs in by_port.items():
        if len(svcs) > 1:
            errors.append(
                f"Duplicate service.port {port} in helm values: {', '.join(sorted(svcs))}"
            )


def _validate_kind_ports(
    kind_ports: list[tuple[int, str]], errors: list[str], warnings: list[str]
) -> None:
    kind_host = [p for p, _ in kind_ports]
    if len(kind_host) != len(set(kind_host)):
        seen: set[int] = set()
        for p, _ in kind_ports:
            if p in seen:
                errors.append("Duplicate hostPort in kind-config.yaml")
            seen.add(p)
    for hp, _ in kind_ports:
        if TILT_MANAGED_RANGE[0] <= hp <= TILT_MANAGED_RANGE[1]:
            warnings.append(
                f"kind-config hostPort {hp} is in Tilt-managed range {TILT_MANAGED_RANGE[0]}-{TILT_MANAGED_RANGE[1]}; "
                "Tilt port-forwards also bind these. Remove from kind extraPortMappings to avoid 'address already in use'."
            )


def _validate_registry_helm_tilt_bff(
    reg: dict[str, int],
    helm: dict[str, int],
    tilt: dict[str, int],
    bff: dict[str, int],
    errors: list[str],
    warnings: list[str],
) -> None:
    for svc, port in helm.items():
        r = reg.get(svc)
        if r is not None and r != port:
            errors.append(f"Port mismatch: registry has {svc}={r}, helm has {port}")
    for svc, port in tilt.items():
        r = reg.get(svc)
        if r is not None and r != port:
            errors.append(f"Port mismatch: registry has {svc}={r}, Tiltfile has {port}")
    for svc, port in helm.items():
        t = tilt.get(svc)
        if t is not None and t != port:
            errors.append(f"Port mismatch: helm has {svc}={port}, Tiltfile has {t}")
    for svc, port in bff.items():
        if helm.get(svc) is not None and helm[svc] != port:
            warnings.append(f"bff-suite-config {svc}= {port} differs from helm {helm[svc]}")
        if tilt.get(svc) is not None and tilt[svc] != port:
            warnings.append(f"bff-suite-config {svc}= {port} differs from Tiltfile {tilt[svc]}")


def _validate_bff_suite_configs(
    project_root: Path, reg: dict[str, int], errors: list[str], warnings: list[str]
) -> None:
    for suite in suites_with_bff(project_root):
        try:
            with bff_suite_config_path(project_root, suite).open() as f:
                data = yaml.safe_load(f) or {}
            if not get_bff_service_name_from_config(data):
                warnings.append(
                    f"openapi/{suite}/bff-suite-config.yaml has no bff_service_name (top-level or metadata). "
                    "Add it so rerp ports can discover this suite's BFF."
                )
        except (OSError, yaml.YAMLError, KeyError, TypeError, ValueError) as e:
            log.debug("Could not read bff_suite_config %s: %s", suite, e)

    for bff_svc, suite in iter_bffs(project_root):
        path = bff_suite_config_path(project_root, suite)
        if not path.exists() or bff_svc not in reg:
            continue
        try:
            with path.open() as f:
                bsc = yaml.safe_load(f) or {}
            for s in (bsc.get("metadata") or {}).get("servers") or []:
                if isinstance(s, dict) and "localhost" in str(s.get("url", "")):
                    m = re.search(r":(\d+)(?:/|$)", str(s.get("url", "")))
                    if m and int(m.group(1)) != reg[bff_svc]:
                        errors.append(
                            f"openapi/{suite}/bff-suite-config.yaml metadata.servers localhost port {m.group(1)} differs from registry {bff_svc}={reg[bff_svc]}. "
                            f"Run: rerp ports update-configs {bff_svc}"
                        )
                    break
        except (
            OSError,
            yaml.YAMLError,
            KeyError,
            TypeError,
            ValueError,
            re.error,
        ) as e:
            log.debug("Could not read bff_suite_config %s: %s", path, e)


def _validate_openapi_localhost(project_root: Path, reg: dict[str, int], errors: list[str]) -> None:
    obff = discover_openapi_bff_localhost(project_root)
    for bff_svc, (port, suite) in obff.items():
        if bff_svc not in reg or reg[bff_svc] == port:
            continue
        errors.append(
            f"openapi/{suite}/openapi_bff.yaml localhost server port {port} differs from registry {bff_svc}={reg[bff_svc]}. "
            f"Regenerate: bff-generator generate-spec --config openapi/{suite}/bff-suite-config.yaml --output openapi/{suite}/openapi_bff.yaml"
        )

    oacc = discover_openapi_suite_microservice_localhost(project_root)
    for svc, (suite, port) in oacc.items():
        r = reg.get(svc)
        if r is not None and r != port:
            errors.append(
                f"openapi/{suite}/{svc}/openapi.yaml localhost server port {port} differs from registry {svc}={r}. "
                f"Run: rerp ports update-configs {svc}"
            )


def validate(registry: PortRegistry, project_root: Path, json_out: bool = False) -> int:
    """Scan registry, helm, kind, Tiltfile, bff-suite-config, etc.; report conflicts. Return 0 if ok, 1 if conflicts."""
    reg = registry.list_assignments()
    helm = discover_helm(project_root)
    kind_ports = discover_kind_host_ports(project_root)
    tilt = discover_tiltfile(project_root)
    bff = discover_bff_suite_config(project_root)

    errors: list[str] = []
    warnings: list[str] = []

    _validate_helm_duplicates(helm, errors)
    _validate_kind_ports(kind_ports, errors, warnings)
    _validate_registry_helm_tilt_bff(reg, helm, tilt, bff, errors, warnings)
    _validate_bff_suite_configs(project_root, reg, errors, warnings)
    _validate_openapi_localhost(project_root, reg, errors)

    if json_out:
        print(
            json.dumps(
                {"ok": len(errors) == 0, "errors": errors, "warnings": warnings},
                indent=2,
            )
        )
        return 0 if len(errors) == 0 else 1

    for e in errors:
        print(f"❌ {e}")
    for w in warnings:
        print(f"⚠️  {w}")
    if errors:
        print("\n💡 Run: rerp ports list   and   rerp ports validate")
        return 1
    if warnings:
        print("\n✅ No hard conflicts; see warnings above.")
        return 0
    print("✅ No port conflicts found.")
    return 0


def reconcile(registry: PortRegistry, project_root: Path, update_configs: bool = False) -> int:
    """Add helm-only services to registry (using helm port)."""
    helm = discover_helm(project_root)
    reg = registry.list_assignments()
    assigned_ports = set(reg.values())
    reserved = set(registry.registry.get("reserved_ports", RESERVED_PORTS))
    for name, port in sorted(helm.items()):
        if name not in reg:
            if port in assigned_ports or port in reserved:
                print(
                    f"⚠️  {name}: helm has port {port} but it is already taken; skip adding. Run validate."
                )
                continue
            registry.assign_port(name, force=False, preferred_port=port)
            print(f"✅ Added to registry: {name} = {port}")
            assigned_ports.add(port)
            if update_configs:
                registry.update_config_files(name, port)
        else:
            if reg[name] != port:
                print(
                    f"⚠️  {name}: registry={reg[name]}, helm={port} (no change; fix manually if needed)"
                )
    return 0


def fix_duplicates(registry: PortRegistry, project_root: Path, dry_run: bool = False) -> int:
    """Resolve duplicate service.port in helm; prefer suite (BFF + bff-suite-config)."""
    helm = discover_helm(project_root)
    by_port: dict[int, list[str]] = defaultdict(list)
    for svc, port in helm.items():
        by_port[port].append(svc)
    suite_keepers = load_suite_services(project_root)
    reg = registry.list_assignments()
    dupes = [(p, svcs) for p, svcs in sorted(by_port.items()) if len(svcs) > 1]
    if not dupes:
        print("✅ No duplicate helm ports found.")
        return 0
    print(f"Resolving {len(dupes)} duplicate port(s)...")
    for port, svcs in dupes:
        in_keep = [s for s in svcs if s in suite_keepers]
        keeper = sorted(in_keep)[0] if in_keep else sorted(svcs)[0]
        losers = [s for s in svcs if s != keeper]
        need_p_for_k = reg.get(keeper) != port
        if need_p_for_k:
            for s in svcs:
                if s != keeper and reg.get(s) == port:
                    if not dry_run:
                        registry.release_port(s)
                        reg = registry.list_assignments()
                    print(f"  release {s} (free {port} for {keeper})")
            if not dry_run:
                registry.assign_port(keeper, force=False, preferred_port=port)
                registry.update_config_files(keeper, port, port_only=True)
                reg = registry.list_assignments()
            print(f"  assign {keeper} = {port}")
        for loser in losers:
            if reg.get(loser) is not None:
                if not dry_run:
                    registry.release_port(loser)
                    reg = registry.list_assignments()
                print(f"  release {loser}")
            if not dry_run:
                p2, _ = registry.assign_port(loser, force=False, preferred_port=None)
                registry.update_config_files(loser, p2, port_only=True)
                reg = registry.list_assignments()
                print(f"  assign {loser} = {p2}  (update helm)")
            else:
                print(f"  assign {loser} = (next available)")
    if dry_run:
        print("  (dry-run; no changes written)")
    return 0
