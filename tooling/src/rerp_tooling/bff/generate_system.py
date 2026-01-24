"""Generate system-level BFF OpenAPI specs from sub-service specs.

Directory-based discovery: openapi/{system}/{service}/openapi.yaml.
Merge into openapi/{system}/openapi.yaml with schema prefixing and $ref updates.
Idempotent, clobber output.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

import yaml


def _to_pascal_case(name: str) -> str:
    return "".join(word.capitalize() for word in name.split("-"))


def discover_sub_services(openapi_dir: Path, system: str) -> dict[str, dict[str, Any]]:
    """Discover sub-services under openapi_dir/system/ with openapi.yaml.

    Returns:
        Dict mapping service name to {"spec": Path, "base_path": str}.
        base_path = /api/v1/{system}/{service}. Empty if system dir missing or no subs.
    """
    system_dir = openapi_dir / system
    if not system_dir.exists() or not system_dir.is_dir():
        return {}

    discovered: dict[str, dict[str, Any]] = {}
    for service_path in sorted(system_dir.iterdir()):
        if not service_path.is_dir():
            continue
        service_name = service_path.name
        if service_name.startswith(".") or service_name == system:
            continue
        spec_file = service_path / "openapi.yaml"
        if not spec_file.exists():
            continue
        discovered[service_name] = {
            "spec": spec_file,
            "base_path": f"/api/v1/{system}/{service_name}",
        }
    return discovered


def list_systems_with_sub_services(openapi_dir: Path) -> list[str]:
    """Return sorted system names that have at least one sub-service with openapi.yaml."""
    if not openapi_dir.exists() or not openapi_dir.is_dir():
        return []
    out: list[str] = []
    for d in sorted(openapi_dir.iterdir()):
        if not d.is_dir() or d.name.startswith("."):
            continue
        if discover_sub_services(openapi_dir, d.name):
            out.append(d.name)
    return out


def _load_spec(p: Path) -> dict[str, Any]:
    with p.open() as f:
        return yaml.safe_load(f)


def _update_refs_in_value(val: Any, old_name: str, new_name: str) -> None:
    if isinstance(val, dict):
        if "$ref" in val:
            ref = val["$ref"]
            # Exact match only: avoid rewriting refs when old_name is a prefix of
            # the schema (e.g. Error must not change #/components/schemas/ErrorResponse)
            if ref == f"#/components/schemas/{old_name}":
                val["$ref"] = f"#/components/schemas/{new_name}"
        for v in val.values():
            _update_refs_in_value(v, old_name, new_name)
    elif isinstance(val, list):
        for it in val:
            _update_refs_in_value(it, old_name, new_name)


def _merge_schemas(
    all_schemas: dict[str, Any],
    service_name: str,
    schemas: dict[str, Any],
) -> None:
    for schema_name, schema_def in schemas.items():
        prefixed = f"{_to_pascal_case(service_name)}{schema_name}"
        all_schemas[prefixed] = schema_def
        if isinstance(schema_def, dict):
            _update_refs_in_value(schema_def, schema_name, prefixed)


def _update_refs_in_paths(paths: dict[str, Any], old_name: str, new_name: str) -> None:
    methods = {"get", "post", "put", "patch", "delete", "options", "head", "trace"}
    for path_def in paths.values():
        if not isinstance(path_def, dict):
            continue
        for method, op in path_def.items():
            if method not in methods or not isinstance(op, dict):
                continue
            if "requestBody" in op:
                _update_refs_in_value(op["requestBody"], old_name, new_name)
            if "responses" in op:
                for r in op["responses"].values():
                    if isinstance(r, dict) and "content" in r:
                        _update_refs_in_value(r["content"], old_name, new_name)


def _schema_name_mapping(
    all_schemas: dict[str, Any],
    sub_services: dict[str, Any],
) -> dict[str, list[str]]:
    mapping: dict[str, list[str]] = {}
    for prefixed in sorted(all_schemas.keys()):
        for sname in sorted(sub_services.keys()):
            prefix = _to_pascal_case(sname)
            if prefixed.startswith(prefix):
                unprefixed = prefixed[len(prefix) :]
                if unprefixed not in mapping:
                    mapping[unprefixed] = []
                mapping[unprefixed].append(prefixed)
    return mapping


def _update_all_refs_in_value(
    val: Any,
    mapping: dict[str, list[str]],
    all_schemas: dict[str, Any],
) -> None:
    if isinstance(val, dict):
        if "$ref" in val:
            ref = val["$ref"]
            if "#/components/schemas/" in ref:
                unprefixed = ref.split("#/components/schemas/")[-1]
                if unprefixed in mapping:
                    for p in mapping[unprefixed]:
                        if p in all_schemas:
                            val["$ref"] = ref.replace(unprefixed, p)
                            break
        for v in val.values():
            _update_all_refs_in_value(v, mapping, all_schemas)
    elif isinstance(val, list):
        for it in val:
            _update_all_refs_in_value(it, mapping, all_schemas)


def _description_from_readme(
    openapi_dir: Path, system: str, system_title: str, default: str
) -> str:
    readme = openapi_dir / system / "README.md"
    if not readme.exists():
        return default
    content = readme.read_text()
    if "## Overview" not in content:
        return default
    section = content.split("## Overview")[1].split("##")[0].strip()
    for line in section.split("\n"):
        line = line.strip()
        if line and not line.startswith("#"):
            return line
    return default


def _merge_one_sub_service(
    sname: str,
    cfg: dict[str, Any],
    all_schemas: dict[str, Any],
    all_tags: set[str],
    all_paths: dict[str, Any],
    merged_params: dict[str, Any],
) -> None:
    spec_path = cfg["spec"] if isinstance(cfg["spec"], Path) else Path(cfg["spec"])
    if not spec_path.exists():
        return
    spec = _load_spec(spec_path)

    if "tags" in spec:
        for t in spec["tags"]:
            all_tags.add(t.get("name", str(t)) if isinstance(t, dict) else str(t))

    if "paths" in spec:
        for path, path_def in spec["paths"].items():
            if isinstance(path_def, dict):
                for m in list(path_def.keys()):
                    if m in (
                        "get",
                        "post",
                        "put",
                        "patch",
                        "delete",
                        "options",
                        "head",
                        "trace",
                    ):
                        op = path_def[m]
                        if isinstance(op, dict) and "x-service" not in op:
                            op["x-service"] = sname
                            op["x-service-base-path"] = cfg["base_path"]
            all_paths[path] = path_def

    if "components" in spec and "schemas" in spec["components"]:
        _merge_schemas(all_schemas, sname, spec["components"]["schemas"])
        for schema_name in spec["components"]["schemas"]:
            prefixed = f"{_to_pascal_case(sname)}{schema_name}"
            _update_refs_in_paths(all_paths, schema_name, prefixed)

    if "components" in spec and "parameters" in spec.get("components", {}):
        for pn, pd in spec["components"]["parameters"].items():
            if pn not in merged_params:
                merged_params[pn] = pd


def _add_error_schema_and_preferred_ref(
    all_schemas: dict[str, Any],
    all_paths: dict[str, Any],
    sub_services: dict[str, dict[str, Any]],
) -> None:
    if "Error" not in all_schemas:
        all_schemas["Error"] = {
            "type": "object",
            "required": ["error", "message"],
            "properties": {
                "error": {"type": "string", "description": "Error code"},
                "message": {
                    "type": "string",
                    "description": "Human-readable error message",
                },
                "details": {
                    "type": "object",
                    "nullable": True,
                    "description": "Additional details",
                    "additionalProperties": True,
                },
            },
        }
    for sname in sorted(sub_services.keys()):
        pe = f"{_to_pascal_case(sname)}Error"
        if pe in all_schemas:
            _update_refs_in_paths(all_paths, "Error", pe)
            all_schemas["Error"] = {"$ref": f"#/components/schemas/{pe}"}
            break


def generate_system_bff_spec(
    openapi_dir: Path,
    system: str,
    output_path: Path | None = None,
) -> None:
    """Generate system BFF OpenAPI at output_path (default: openapi_dir/system/openapi.yaml).

    Discovers sub-services from openapi_dir/system/{service}/openapi.yaml,
    merges paths and schemas (prefixing schemas to avoid clashes), updates $refs,
    writes deterministic YAML. Idempotent; clobbers output.
    If no sub-services, does not write.
    """
    sub_services = discover_sub_services(openapi_dir, system)
    if not sub_services:
        return

    out = output_path if output_path is not None else (openapi_dir / system / "openapi.yaml")
    system_title = system.replace("-", " ").title()
    default_desc = f"System-level API gateway for all {system_title} services"
    system_description = _description_from_readme(openapi_dir, system, system_title, default_desc)

    service_routes = [
        f"- `{cfg['base_path']}/*` → {_to_pascal_case(sname)} Service"
        for sname, cfg in sorted(sub_services.items())
    ]

    bff: dict[str, Any] = {
        "openapi": "3.1.0",
        "info": {
            "title": f"{system_title} API Gateway",
            "description": (
                f"{system_description}. "
                f"This aggregates and proxies requests to {system_title} microservices. "
                f"Single entry point for the {system_title} system.\n\n"
                "All requests proxied to sub-services:\n" + "\n".join(service_routes) + "\n\n"
                "**Note**: Auto-generated. Do not edit manually.\n\n"
                f"**Discovery**: openapi/{system}/"
            ),
            "version": "1.0.0",
        },
        "servers": [{"url": f"/api/v1/{system}", "description": f"{system_title} API Gateway"}],
        "tags": [],
        "paths": {},
        "components": {
            "parameters": {
                "Page": {
                    "name": "page",
                    "in": "query",
                    "schema": {"type": "integer", "minimum": 1, "default": 1},
                },
                "Limit": {
                    "name": "limit",
                    "in": "query",
                    "schema": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 100,
                        "default": 20,
                    },
                },
                "Search": {
                    "name": "search",
                    "in": "query",
                    "schema": {"type": "string"},
                },
            },
            "schemas": {},
        },
    }

    all_schemas = bff["components"]["schemas"]
    all_tags: set[str] = set()
    all_paths: dict[str, Any] = {}
    merged_params = dict(bff["components"]["parameters"])

    for sname, cfg in sorted(sub_services.items()):
        _merge_one_sub_service(sname, cfg, all_schemas, all_tags, all_paths, merged_params)

    bff["components"]["parameters"] = merged_params

    mapping = _schema_name_mapping(all_schemas, sub_services)
    for _sn, schema_def in sorted(all_schemas.items()):
        if isinstance(schema_def, dict):
            _update_all_refs_in_value(schema_def, mapping, all_schemas)

    _add_error_schema_and_preferred_ref(all_schemas, all_paths, sub_services)

    bff["tags"] = sorted([{"name": t} for t in all_tags], key=lambda x: x["name"])
    bff["paths"] = dict(sorted(all_paths.items()))
    bff["components"]["schemas"] = dict(sorted(all_schemas.items()))

    out.parent.mkdir(parents=True, exist_ok=True)
    if out.exists():
        out.unlink()
    with out.open("w") as f:
        yaml.dump(
            bff,
            f,
            sort_keys=False,
            default_flow_style=False,
            allow_unicode=True,
            width=120,
        )
