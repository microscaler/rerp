#!/usr/bin/env python3
"""
Port Registry Manager for RERP Microservices

This script manages port assignments for RERP services, ensuring no conflicts
and automatically updating configuration files.

Usage:
    assign-port.py assign <service-name>          # Assign next available port
    assign-port.py release <service-name>        # Release a port
    assign-port.py list                          # List all assignments
    assign-port.py query <service-name>          # Get port for a service
    assign-port.py update-configs <service-name> # Update config files with assigned port
    assign-port.py validate [--json]             # Scan all sources; report conflicts
    assign-port.py reconcile [--update-configs]  # Add helm-only services to registry
    assign-port.py fix-duplicates [--dry-run]    # Resolve duplicate helm ports; prefer suite (BFF + bff-suite-config)
"""

import json
import os
import re
import sys
import argparse
from pathlib import Path
from typing import Dict, Iterator, List, Optional, Tuple
from datetime import datetime
from collections import defaultdict

import yaml

# Constants
SCRIPT_DIR = Path(__file__).parent
PROJECT_ROOT = SCRIPT_DIR.parent
REGISTRY_FILE = SCRIPT_DIR / "port-registry.json"
HELM_VALUES_DIR = PROJECT_ROOT / "helm" / "rerp-microservice" / "values"
KIND_CONFIG = PROJECT_ROOT / "kind-config.yaml"
TILTFILE = PROJECT_ROOT / "Tiltfile"
OPENAPI_DIR = PROJECT_ROOT / "openapi"
START_PORT = 8001
RESERVED_PORTS = [8080]  # Ports that should never be assigned
# Tilt port-forwards bind these on the host; kind-config must NOT use hostPort in this range
TILT_MANAGED_RANGE = (8001, 8099)


def _suites_with_bff() -> List[str]:
    """Suites that have a BFF: openapi/{suite}/bff-suite-config.yaml exists."""
    if not OPENAPI_DIR.exists():
        return []
    return [d.name for d in OPENAPI_DIR.iterdir() if d.is_dir() and (d / "bff-suite-config.yaml").exists()]


def _bff_suite_config_path(suite: str) -> Path:
    return OPENAPI_DIR / suite / "bff-suite-config.yaml"


def _openapi_bff_path(suite: str) -> Path:
    return OPENAPI_DIR / suite / "openapi_bff.yaml"


def _service_to_suite(service_name: str) -> Optional[str]:
    """Return the suite that contains openapi/{suite}/{service_name}/openapi.yaml, or None."""
    if not OPENAPI_DIR.exists():
        return None
    for d in OPENAPI_DIR.iterdir():
        if d.is_dir() and (d / service_name / "openapi.yaml").exists():
            return d.name
    return None


def _get_bff_service_name_from_config(data: dict) -> Optional[str]:
    """Read bff_service_name from bff-suite-config (top-level or metadata). Returns None if absent."""
    return data.get("bff_service_name") or (data.get("metadata") or {}).get("bff_service_name")


def _iter_bffs() -> Iterator[Tuple[str, str]]:
    """
    Yield (bff_service_name, suite) for each suite that has bff-suite-config.yaml with
    bff_service_name set. Suites are discovered by listing openapi/ subdirs that contain
    bff-suite-config.yaml.
    """
    for suite in _suites_with_bff():
        path = _bff_suite_config_path(suite)
        try:
            with open(path) as f:
                data = yaml.safe_load(f) or {}
            name = _get_bff_service_name_from_config(data)
            if name:
                yield (name, suite)
        except Exception:
            pass


def _bff_service_to_suite(service_name: str) -> Optional[str]:
    """Return the suite whose BFF has the given registry service name, or None."""
    for bff_svc, suite in _iter_bffs():
        if bff_svc == service_name:
            return suite
    return None


class PortRegistry:
    """Manages port assignments for RERP services."""
    
    def __init__(self, registry_file: Path = REGISTRY_FILE):
        self.registry_file = registry_file
        self.registry = self._load_registry()
    
    def _load_registry(self) -> dict:
        """Load the port registry from JSON file."""
        if not self.registry_file.exists():
            # Initialize new registry
            return {
                "version": "1.0",
                "next_port": START_PORT,
                "reserved_ports": RESERVED_PORTS,
                "assignments": {},
                "metadata": {
                    "description": "Port registry for RERP microservices",
                    "last_updated": None,
                    "notes": "Ports start at 8001. Port 8080 is reserved due to conflicts."
                }
            }
        
        with open(self.registry_file, 'r') as f:
            return json.load(f)
    
    def _save_registry(self):
        """Save the port registry to JSON file."""
        self.registry["metadata"]["last_updated"] = datetime.now().isoformat()
        with open(self.registry_file, 'w') as f:
            json.dump(self.registry, f, indent=2)
    
    def _find_next_available_port(self) -> int:
        """Find the next available port, skipping reserved ports."""
        assigned_ports = set(self.registry["assignments"].values())
        reserved = set(self.registry.get("reserved_ports", RESERVED_PORTS))
        
        port = self.registry.get("next_port", START_PORT)
        while port in assigned_ports or port in reserved:
            port += 1
        
        return port
    
    def assign_port(
        self, service_name: str, force: bool = False, preferred_port: Optional[int] = None
    ) -> Tuple[int, bool]:
        """
        Assign a port to a service.

        Args:
            preferred_port: If set and available, use this port when creating a new assignment.
        Returns:
            Tuple of (port_number, was_new_assignment)
        """
        assignments = self.registry["assignments"]
        assigned = set(assignments.values())
        reserved = set(self.registry.get("reserved_ports", RESERVED_PORTS))

        if service_name in assignments and not force:
            return assignments[service_name], False

        if preferred_port is not None and preferred_port not in assigned and preferred_port not in reserved:
            port = preferred_port
        else:
            port = self._find_next_available_port()

        assignments[service_name] = port
        self.registry["next_port"] = max(self.registry.get("next_port", START_PORT), port + 1)
        self._save_registry()
        return port, True
    
    def release_port(self, service_name: str) -> Optional[int]:
        """Release a port assignment for a service."""
        assignments = self.registry["assignments"]
        if service_name not in assignments:
            return None
        
        port = assignments.pop(service_name)
        self._save_registry()
        return port
    
    def get_port(self, service_name: str) -> Optional[int]:
        """Get the assigned port for a service."""
        return self.registry["assignments"].get(service_name)
    
    def list_assignments(self) -> Dict[str, int]:
        """List all port assignments."""
        return self.registry["assignments"].copy()
    
    def update_config_files(self, service_name: str, port: int, port_only: bool = False):
        """
        Update configuration files with the assigned port.

        When port_only=True, only service.port, service.containerPort, service.nodePort
        are updated in helm values; image, app, and kind-config are left unchanged.
        Use for fix-duplicates to avoid overwriting app.binaryName etc.
        """
        # NodePort formula: 31000 + (port - 8000)
        node_port = 31000 + (port - 8000)

        values_file = PROJECT_ROOT / "helm" / "rerp-microservice" / "values" / f"{service_name}.yaml"
        if not values_file.exists():
            print(f"⚠️  Helm values file not found: {values_file}")
            print(f"   Create it with: bootstrap_microservice.py {service_name}")
            return

        with open(values_file, 'r') as f:
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

        with open(values_file, 'w') as f:
            yaml.dump(values, f, default_flow_style=False, sort_keys=False)
        print(f"✅ Updated {values_file}")

        # OpenAPI: update servers so Swagger "Try it" uses the correct localhost port
        _update_openapi_servers(service_name, port)

        if port_only:
            return
        # Update kind-config.yaml: skip for ports in TILT_MANAGED_RANGE (Tilt port-forwards
        # bind these on the host; adding hostPort here would cause "address already in use").
        if TILT_MANAGED_RANGE[0] <= port <= TILT_MANAGED_RANGE[1]:
            print(f"ℹ️  Skipping kind-config: port {port} in Tilt-managed range {TILT_MANAGED_RANGE[0]}-{TILT_MANAGED_RANGE[1]} (Tilt port-forwards)")
        elif KIND_CONFIG.exists():
            with open(KIND_CONFIG, 'r') as f:
                content = f.read()
            if f"hostPort: {port}" not in content:
                # Find insertion point after extraPortMappings: and before containerdConfigPatches
                lines = content.split('\n')
                insert_index = None
                for i, line in enumerate(lines):
                    if 'extraPortMappings:' in line:
                        # Insert after the last mapping under extraPortMappings
                        j = i + 1
                        while j < len(lines) and (lines[j].strip().startswith('#') or lines[j].strip().startswith('-') or 'containerPort' in lines[j] or 'hostPort' in lines[j] or 'protocol' in lines[j]):
                            j += 1
                        insert_index = j
                        break
                if insert_index is not None:
                    indent = "  "
                    new_lines = [
                        indent + f"# {service_name.replace('-', ' ').title()} Service",
                        indent + f"- containerPort: {node_port}",
                        indent + f"  hostPort: {port}",
                        indent + f"  protocol: TCP"
                    ]
                    lines[insert_index:insert_index] = new_lines
                    with open(KIND_CONFIG, 'w') as f:
                        f.write('\n'.join(lines))
                    print(f"✅ Updated {KIND_CONFIG}")
            else:
                print(f"ℹ️  Port mapping already exists in {KIND_CONFIG}")


# ---------------------------------------------------------------------------
# OpenAPI: update servers for correct localhost port (Swagger "Try it")
# ---------------------------------------------------------------------------

def _update_openapi_servers(service_name: str, port: int) -> None:
    """
    Update OpenAPI server URLs so Swagger UI "Try it" points at the correct
    localhost port. Per-suite: updates openapi/{suite}/bff-suite-config.yaml for a
    BFF, or openapi/{suite}/{name}/openapi.yaml for a suite microservice.
    """
    suite = _bff_service_to_suite(service_name)
    if suite is not None:
        path = _bff_suite_config_path(suite)
        if not path.exists():
            return
        with open(path, "r") as f:
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
        with open(path, "w") as f:
            yaml.dump(data, f, default_flow_style=False, sort_keys=False)
        print(f"✅ Updated {path} (BFF localhost server -> {localhost_url})")
        return

    suite = _service_to_suite(service_name)
    if not suite:
        return
    spec_path = OPENAPI_DIR / suite / service_name / "openapi.yaml"
    if not spec_path.exists():
        return
    with open(spec_path, "r") as f:
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
    with open(spec_path, "w") as f:
        yaml.dump(data, f, default_flow_style=False, sort_keys=False)
    print(f"✅ Updated {spec_path} (localhost server -> {localhost_url})")


# ---------------------------------------------------------------------------
# Validate: discover all port usages and report conflicts
# ---------------------------------------------------------------------------

def _discover_helm() -> Dict[str, int]:
    """Discover service.port from helm/rerp-microservice/values/*.yaml."""
    out: Dict[str, int] = {}
    if not HELM_VALUES_DIR.exists():
        return out
    for p in sorted(HELM_VALUES_DIR.glob("*.yaml")):
        try:
            with open(p) as f:
                data = yaml.safe_load(f) or {}
            svc = data.get("service") or {}
            port = svc.get("port")
            name = svc.get("name") or p.stem
            if port is not None:
                out[name] = int(port)
        except Exception:
            pass
    return out


def _discover_kind_host_ports() -> List[Tuple[int, str]]:
    """Discover hostPort from kind-config extraPortMappings. Returns [(host_port, comment_or_container_port)]."""
    out: List[Tuple[int, str]] = []
    if not KIND_CONFIG.exists():
        return out
    try:
        with open(KIND_CONFIG) as f:
            data = yaml.safe_load(f) or {}
        nodes = data.get("nodes") or []
        for n in nodes:
            for m in n.get("extraPortMappings") or []:
                hp = m.get("hostPort")
                if hp is not None:
                    cp = m.get("containerPort", "")
                    out.append((int(hp), str(cp)))
    except Exception:
        pass
    return out


def _discover_tiltfile() -> Dict[str, int]:
    """Discover get_service_port dict from Tiltfile (starlark)."""
    out: Dict[str, int] = {}
    if not TILTFILE.exists():
        return out
    try:
        with open(TILTFILE) as f:
            text = f.read()
        # Match: 'name': 'port' inside the ports = { } block
        m = re.search(r"ports\s*=\s*\{([^}]+)\}", text, re.DOTALL)
        if m:
            for m2 in re.finditer(r"'([^']+)'\s*:\s*'(\d+)'", m.group(1)):
                out[m2.group(1)] = int(m2.group(2))
    except Exception:
        pass
    return out


def _discover_bff_suite_config() -> Dict[str, int]:
    """Discover services.*.port from all openapi/{suite}/bff-suite-config.yaml."""
    out: Dict[str, int] = {}
    for suite in _suites_with_bff():
        path = _bff_suite_config_path(suite)
        try:
            with open(path) as f:
                data = yaml.safe_load(f) or {}
            for name, cfg in (data.get("services") or {}).items():
                if isinstance(cfg, dict) and "port" in cfg:
                    out[name] = int(cfg["port"])
        except Exception:
            pass
    return out


def _discover_generate_bff_spec() -> Dict[str, int]:
    """Discover SERVICE_CONFIG / port from scripts/generate_bff_spec.py."""
    out: Dict[str, int] = {}
    p = PROJECT_ROOT / "scripts" / "generate_bff_spec.py"
    if not p.exists():
        return out
    try:
        with open(p) as f:
            text = f.read()
        for m in re.finditer(r"'([^']+)'\s*:\s*\{[^}]*'port'\s*:\s*(\d+)", text):
            out[m.group(1)] = int(m.group(2))
    except Exception:
        pass
    return out


def _discover_openapi_bff_localhost() -> Dict[str, Tuple[int, str]]:
    """
    Extract localhost port from openapi/{suite}/openapi_bff.yaml for each BFF discovered via
    _iter_bffs (from bff-suite-config). Returns {bff_service_name: (port, suite)}.
    """
    out: Dict[str, Tuple[int, str]] = {}
    for bff_svc, suite in _iter_bffs():
        path = _openapi_bff_path(suite)
        if not path.exists():
            continue
        try:
            with open(path) as f:
                data = yaml.safe_load(f) or {}
            for s in data.get("servers") or []:
                if isinstance(s, dict):
                    u = str(s.get("url", ""))
                    if "localhost" in u:
                        m = re.search(r":(\d+)(?:/|$)", u)
                        if m:
                            out[bff_svc] = (int(m.group(1)), suite)
                        break
        except Exception:
            pass
    return out


def _discover_openapi_suite_microservice_localhost() -> Dict[str, Tuple[str, int]]:
    """Extract localhost ports from openapi/{suite}/{name}/openapi.yaml. Returns {service_name: (suite, port)}."""
    out: Dict[str, Tuple[str, int]] = {}
    if not OPENAPI_DIR.exists():
        return out
    for suite_dir in OPENAPI_DIR.iterdir():
        if not suite_dir.is_dir():
            continue
        for sdir in suite_dir.iterdir():
            if not sdir.is_dir():
                continue
            spec = sdir / "openapi.yaml"
            if not spec.exists():
                continue
            try:
                with open(spec) as f:
                    data = yaml.safe_load(f) or {}
                for s in data.get("servers") or []:
                    if isinstance(s, dict):
                        u = str(s.get("url", ""))
                        if "localhost" in u:
                            m = re.search(r":(\d+)(?:/|$)", u)
                            if m:
                                out[sdir.name] = (suite_dir.name, int(m.group(1)))
                            break
            except Exception:
                pass
    return out


def validate(registry: "PortRegistry", json_out: bool = False) -> int:
    """
    Scan registry, helm, kind, Tiltfile, bff-suite-config, generate_bff_spec.
    Report conflicts and mismatches. Returns 0 if ok, 1 if conflicts.
    """
    reg = registry.list_assignments()
    helm = _discover_helm()
    kind_ports = _discover_kind_host_ports()
    tilt = _discover_tiltfile()
    bff = _discover_bff_suite_config()
    gen = _discover_generate_bff_spec()

    errors: List[str] = []
    warnings: List[str] = []

    # 1) Duplicate port in helm (two services same port)
    by_port: Dict[int, List[str]] = defaultdict(list)
    for svc, port in helm.items():
        by_port[port].append(svc)
    for port, svcs in by_port.items():
        if len(svcs) > 1:
            errors.append(f"Duplicate service.port {port} in helm values: {', '.join(sorted(svcs))}")

    # 2) Duplicate hostPort in kind
    kind_host = [p for p, _ in kind_ports]
    if len(kind_host) != len(set(kind_host)):
        seen = set()
        for p, _ in kind_ports:
            if p in seen:
                errors.append(f"Duplicate hostPort {p} in kind-config.yaml")
            seen.add(p)

    # 3) kind hostPort in Tilt-managed range (would conflict with Tilt port-forwards)
    for hp, _ in kind_ports:
        if TILT_MANAGED_RANGE[0] <= hp <= TILT_MANAGED_RANGE[1]:
            warnings.append(
                f"kind-config hostPort {hp} is in Tilt-managed range {TILT_MANAGED_RANGE[0]}-{TILT_MANAGED_RANGE[1]}; "
                "Tilt port-forwards also bind these. Remove from kind extraPortMappings to avoid 'address already in use'."
            )

    # 4) Registry vs helm mismatch (only for services in both)
    for svc, port in helm.items():
        r = reg.get(svc)
        if r is not None and r != port:
            errors.append(f"Port mismatch: registry has {svc}={r}, helm has {port}")

    # 5) Registry vs Tiltfile
    for svc, port in tilt.items():
        r = reg.get(svc)
        if r is not None and r != port:
            errors.append(f"Port mismatch: registry has {svc}={r}, Tiltfile has {port}")

    # 6) Helm vs Tiltfile (for services that use both)
    for svc, port in helm.items():
        t = tilt.get(svc)
        if t is not None and t != port:
            errors.append(f"Port mismatch: helm has {svc}={port}, Tiltfile has {t}")

    # 7) BFF suite vs helm/tilt (warn only)
    for svc, port in bff.items():
        if helm.get(svc) is not None and helm[svc] != port:
            warnings.append(f"bff-suite-config {svc}= {port} differs from helm {helm[svc]}")
        if tilt.get(svc) is not None and tilt[svc] != port:
            warnings.append(f"bff-suite-config {svc}= {port} differs from Tiltfile {tilt[svc]}")

    # 8) generate_bff_spec vs helm/tilt (warn)
    for svc, port in gen.items():
        if helm.get(svc) is not None and helm[svc] != port:
            warnings.append(f"generate_bff_spec {svc}= {port} differs from helm {helm[svc]}")
        if tilt.get(svc) is not None and tilt[svc] != port:
            warnings.append(f"generate_bff_spec {svc}= {port} differs from Tiltfile {tilt[svc]}")

    # 8b) bff-suite-config missing bff_service_name (needed for BFF discovery)
    for suite in _suites_with_bff():
        try:
            with open(_bff_suite_config_path(suite)) as f:
                data = yaml.safe_load(f) or {}
            if not _get_bff_service_name_from_config(data):
                warnings.append(
                    f"openapi/{suite}/bff-suite-config.yaml has no bff_service_name (top-level or metadata). "
                    "Add it so assign-port can discover this suite's BFF."
                )
        except Exception:
            pass

    # 9) bff-suite-config metadata.servers localhost vs registry (per suite)
    for bff_svc, suite in _iter_bffs():
        path = _bff_suite_config_path(suite)
        if not path.exists() or bff_svc not in reg:
            continue
        try:
            with open(path) as f:
                bsc = yaml.safe_load(f) or {}
            for s in (bsc.get("metadata") or {}).get("servers") or []:
                if isinstance(s, dict) and "localhost" in str(s.get("url", "")):
                    m = re.search(r":(\d+)(?:/|$)", str(s.get("url", "")))
                    if m and int(m.group(1)) != reg[bff_svc]:
                        errors.append(
                            f"openapi/{suite}/bff-suite-config.yaml metadata.servers localhost port {m.group(1)} differs from registry {bff_svc}={reg[bff_svc]}. "
                            f"Run: assign-port.py update-configs {bff_svc}"
                        )
                    break
        except Exception:
            pass

    # 10) openapi/{suite}/openapi_bff.yaml localhost vs registry (per BFF; regenerate from bff-suite-config if (9) fixed)
    obff = _discover_openapi_bff_localhost()
    for bff_svc, (port, suite) in obff.items():
        if bff_svc not in reg or reg[bff_svc] == port:
            continue
        errors.append(
            f"openapi/{suite}/openapi_bff.yaml localhost server port {port} differs from registry {bff_svc}={reg[bff_svc]}. "
            f"Regenerate: bff-generator generate-spec --config openapi/{suite}/bff-suite-config.yaml --output openapi/{suite}/openapi_bff.yaml"
        )

    # 11) openapi/{suite}/{name}/openapi.yaml localhost vs registry
    oacc = _discover_openapi_suite_microservice_localhost()
    for svc, (suite, port) in oacc.items():
        r = reg.get(svc)
        if r is not None and r != port:
            errors.append(
                f"openapi/{suite}/{svc}/openapi.yaml localhost server port {port} differs from registry {svc}={r}. "
                f"Run: assign-port.py update-configs {svc}"
            )

    if json_out:
        obj = {"ok": len(errors) == 0, "errors": errors, "warnings": warnings}
        print(json.dumps(obj, indent=2))
        return 0 if len(errors) == 0 else 1

    if errors or warnings:
        for e in errors:
            print(f"❌ {e}")
        for w in warnings:
            print(f"⚠️  {w}")
    if errors:
        print("\n💡 Run: ./scripts/assign-port.py list   and   ./scripts/assign-port.py validate")
        return 1

    if warnings:
        print("\n✅ No hard conflicts; see warnings above.")
        return 0

    print("✅ No port conflicts found.")
    return 0


def reconcile(registry: "PortRegistry", update_configs: bool = False) -> int:
    """
    Sync registry from helm values: add any service in helm that is missing in registry
    (using the port from helm). Report if registry has a different port. Returns 0.
    """
    helm = _discover_helm()
    reg = registry.list_assignments()
    assigned_ports = {p for p in reg.values()}
    reserved = set(registry.registry.get("reserved_ports", RESERVED_PORTS))
    for name, port in sorted(helm.items()):
        if name not in reg:
            if port in assigned_ports or port in reserved:
                print(f"⚠️  {name}: helm has port {port} but it is already taken; skip adding. Run validate.")
                continue
            registry.assign_port(name, force=False, preferred_port=port)
            print(f"✅ Added to registry: {name} = {port}")
            assigned_ports.add(port)
            if update_configs:
                registry.update_config_files(name, port)
        else:
            if reg[name] != port:
                print(f"⚠️  {name}: registry={reg[name]}, helm={port} (no change; fix manually if needed)")
    return 0


def _load_suite_services() -> set:
    """Services that keep their port in fix-duplicates: all suite microservices (from any bff-suite-config) and all BFF service names (from bff_service_name)."""
    keep: set = set()
    for bff_svc, _ in _iter_bffs():
        keep.add(bff_svc)
    for suite in _suites_with_bff():
        path = _bff_suite_config_path(suite)
        try:
            with open(path) as f:
                data = yaml.safe_load(f) or {}
            keep |= set((data.get("services") or {}).keys())
        except Exception:
            pass
    return keep


def fix_duplicates(registry: "PortRegistry", dry_run: bool = False) -> int:
    """
    Resolve duplicate service.port in helm: for each duplicate group, the suite
    service (from any bff-suite-config or BFF) keeps the port; others get the next
    available. Updates registry and helm (port-only for losers to preserve
    app.binaryName etc.). Returns 0.
    """
    helm = _discover_helm()
    by_port: Dict[int, List[str]] = defaultdict(list)
    for svc, port in helm.items():
        by_port[port].append(svc)
    suite_keepers = _load_suite_services()
    reg = registry.list_assignments()
    dupes = [(p, svcs) for p, svcs in sorted(by_port.items()) if len(svcs) > 1]
    if not dupes:
        print("✅ No duplicate helm ports found.")
        return 0
    print(f"Resolving {len(dupes)} duplicate port(s)...")
    for port, svcs in dupes:
        # Prefer suite service (BFF or bff-suite-config member); else first alphabetically
        in_keep = [s for s in svcs if s in suite_keepers]
        keeper = sorted(in_keep)[0] if in_keep else sorted(svcs)[0]
        losers = [s for s in svcs if s != keeper]
        need_p_for_k = (reg.get(keeper) != port)
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
        for L in losers:
            if reg.get(L) is not None:
                if not dry_run:
                    registry.release_port(L)
                    reg = registry.list_assignments()
                print(f"  release {L}")
            if not dry_run:
                p2, _ = registry.assign_port(L, force=False, preferred_port=None)
                registry.update_config_files(L, p2, port_only=True)
                reg = registry.list_assignments()
                print(f"  assign {L} = {p2}  (update helm)")
            else:
                print(f"  assign {L} = (next available)")
    if dry_run:
        print("  (dry-run; no changes written)")
    return 0


def main():
    parser = argparse.ArgumentParser(
        description="Manage port assignments for RERP microservices",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )
    
    subparsers = parser.add_subparsers(dest='command', help='Command to execute')
    
    # Assign command
    assign_parser = subparsers.add_parser('assign', help='Assign a port to a service')
    assign_parser.add_argument('service_name', help='Service name (e.g., general-ledger)')
    assign_parser.add_argument('--force', action='store_true', help='Force reassignment even if port exists')
    assign_parser.add_argument('--port', type=int, metavar='N', help='Use this port (must be free) instead of next available')
    assign_parser.add_argument('--update-configs', action='store_true', help='Update config files after assignment')
    
    # Release command
    release_parser = subparsers.add_parser('release', help='Release a port assignment')
    release_parser.add_argument('service_name', help='Service name')
    
    # List command
    subparsers.add_parser('list', help='List all port assignments')
    
    # Query command
    query_parser = subparsers.add_parser('query', help='Get port for a service')
    query_parser.add_argument('service_name', help='Service name')
    
    # Update configs command
    update_parser = subparsers.add_parser('update-configs', help='Update config files with assigned port')
    update_parser.add_argument('service_name', help='Service name')

    # Validate: scan all sources and report conflicts
    validate_parser = subparsers.add_parser('validate', help='Scan registry, helm, kind, Tiltfile, bff-suite-config; report conflicts')
    validate_parser.add_argument('--json', action='store_true', help='Output JSON')

    # Reconcile: add helm-only services to registry with their helm port
    reconcile_parser = subparsers.add_parser('reconcile', help='Add services from helm values to registry (using helm port); report mismatches')
    reconcile_parser.add_argument('--update-configs', action='store_true', help='Run update-configs for each added service')

    # Fix-duplicates: prefer suite services (BFF + bff-suite-config), others get next
    fix_parser = subparsers.add_parser('fix-duplicates', help='Resolve duplicate service.port in helm; prefer suite (BFF + bff-suite-config members)')
    fix_parser.add_argument('--dry-run', action='store_true', help='Only print planned changes')
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        sys.exit(1)
    
    registry = PortRegistry()
    
    if args.command == 'assign':
        port, is_new = registry.assign_port(
            args.service_name,
            force=args.force,
            preferred_port=getattr(args, 'port', None),
        )
        if is_new:
            node_port = 31000 + (port - 8000)
            print(f"✅ Assigned port {port} to service '{args.service_name}'")
            print(f"   NodePort: {node_port}")
        else:
            print(f"ℹ️  Service '{args.service_name}' already has port {port}")
        
        if args.update_configs or is_new:
            registry.update_config_files(args.service_name, port)
    
    elif args.command == 'release':
        port = registry.release_port(args.service_name)
        if port:
            print(f"✅ Released port {port} from service '{args.service_name}'")
        else:
            print(f"⚠️  Service '{args.service_name}' has no assigned port")
    
    elif args.command == 'list':
        assignments = registry.list_assignments()
        if assignments:
            print("\nPort Assignments:")
            print("-" * 50)
            for service, port in sorted(assignments.items()):
                node_port = 31000 + (port - 8000)
                print(f"  {service:30} Port: {port:4}  NodePort: {node_port}")
            print("-" * 50)
            print(f"\nTotal: {len(assignments)} services")
        else:
            print("No port assignments found.")
    
    elif args.command == 'query':
        port = registry.get_port(args.service_name)
        if port:
            node_port = 31000 + (port - 8000)
            print(f"Service: {args.service_name}")
            print(f"  Port: {port}")
            print(f"  NodePort: {node_port}")
        else:
            print(f"⚠️  Service '{args.service_name}' has no assigned port")
            print(f"   Assign one with: {sys.argv[0]} assign {args.service_name}")
            sys.exit(1)
    
    elif args.command == 'update-configs':
        port = registry.get_port(args.service_name)
        if port:
            registry.update_config_files(args.service_name, port)
        else:
            print(f"⚠️  Service '{args.service_name}' has no assigned port")
            print(f"   Assign one first with: {sys.argv[0]} assign {args.service_name}")
            sys.exit(1)

    elif args.command == 'validate':
        sys.exit(validate(registry, json_out=getattr(args, 'json', False)))

    elif args.command == 'reconcile':
        sys.exit(reconcile(registry, update_configs=getattr(args, 'update_configs', False)))

    elif args.command == 'fix-duplicates':
        sys.exit(fix_duplicates(registry, dry_run=getattr(args, 'dry_run', False)))


if __name__ == '__main__':
    main()
