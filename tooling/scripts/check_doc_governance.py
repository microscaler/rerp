#!/usr/bin/env python3
"""Validate RERP's current-document authority and ADR lifecycle contracts."""

from __future__ import annotations

import json
import re
import sys
from collections import defaultdict
from datetime import date
from pathlib import Path, PurePosixPath
from typing import Any

ALLOWED_STATUSES = {
    "PROPOSED",
    "ACCEPTED",
    "SUPERSEDED",
    "REJECTED",
    "DRAFT",
    "IN_REVIEW",
    "APPROVED",
    "IMPLEMENTING",
    "DELIVERED",
    "ABANDONED",
    "ACTIVE",
    "CURRENT_ANALYSIS",
    "HISTORICAL_SNAPSHOT",
}
ALLOWED_AUTHORITIES = {"normative", "working", "informative"}
ALLOWED_KINDS = {"adr", "analysis", "design", "mode", "openapi", "policy", "prd", "roadmap"}
CURRENT_NORMATIVE_STATUSES = {"ACCEPTED", "APPROVED", "IMPLEMENTING", "DELIVERED", "ACTIVE"}
REQUIRED_ENTRY_FIELDS = {
    "id",
    "scope",
    "path",
    "kind",
    "status",
    "authority",
    "owner",
    "last_reviewed",
    "supersedes",
    "superseded_by",
}
REQUIRED_ADR_METADATA = {
    "status",
    "date",
    "decision owners",
    "group",
    "authority",
    "scope",
    "last reviewed",
    "supersedes",
    "superseded by",
}
METADATA_RE = re.compile(r"^- \*\*(?P<key>[^*]+)\*\*: (?P<value>.+?)\s*$")
ADR_FILE_RE = re.compile(r"^[0-9]{3}-.+\.md$")


def repository_root() -> Path:
    return Path(__file__).resolve().parents[2]


def _read_registry(root: Path) -> tuple[dict[str, Any], list[str]]:
    registry_path = root / "docs" / "authority.json"
    try:
        data = json.loads(registry_path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        return {}, ["docs/authority.json does not exist"]
    except json.JSONDecodeError as exc:
        return {}, [f"docs/authority.json is invalid JSON: {exc}"]
    if not isinstance(data, dict):
        return {}, ["docs/authority.json must contain a JSON object"]
    return data, []


def _metadata(path: Path) -> dict[str, str]:
    result: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8").splitlines()[:40]:
        match = METADATA_RE.match(line)
        if match:
            result[match.group("key").strip().lower()] = match.group("value").strip().strip("`")
    return result


def _valid_relative_path(value: str) -> bool:
    candidate = PurePosixPath(value)
    return bool(value) and not candidate.is_absolute() and ".." not in candidate.parts


def validate_repository(root: Path | None = None) -> list[str]:
    root = (root or repository_root()).resolve()
    data, errors = _read_registry(root)
    if errors:
        return errors

    if data.get("schema_version") != 1:
        errors.append("docs/authority.json schema_version must be 1")

    entries = data.get("entries")
    if not isinstance(entries, list):
        return errors + ["docs/authority.json entries must be a list"]

    by_id: dict[str, dict[str, Any]] = {}
    by_path: dict[str, dict[str, Any]] = {}
    current_normative_by_scope: dict[str, list[str]] = defaultdict(list)
    registered_adrs: set[str] = set()

    for index, raw_entry in enumerate(entries):
        label = f"authority entry #{index + 1}"
        if not isinstance(raw_entry, dict):
            errors.append(f"{label} must be an object")
            continue
        missing = sorted(REQUIRED_ENTRY_FIELDS - raw_entry.keys())
        if missing:
            errors.append(f"{label} is missing fields: {', '.join(missing)}")
            continue

        entry_id = raw_entry["id"]
        if not isinstance(entry_id, str) or not entry_id:
            errors.append(f"{label} has an invalid id")
            continue
        if entry_id in by_id:
            errors.append(f"duplicate authority id: {entry_id}")
        by_id[entry_id] = raw_entry
        label = entry_id

        status = raw_entry["status"]
        authority = raw_entry["authority"]
        kind = raw_entry["kind"]
        scope = raw_entry["scope"]
        target = raw_entry["path"]

        if status not in ALLOWED_STATUSES:
            errors.append(f"{label} has unsupported status {status!r}")
        if authority not in ALLOWED_AUTHORITIES:
            errors.append(f"{label} has unsupported authority {authority!r}")
        if kind not in ALLOWED_KINDS:
            errors.append(f"{label} has unsupported kind {kind!r}")
        if not isinstance(scope, str) or not scope or " " in scope:
            errors.append(f"{label} has invalid scope {scope!r}")
        if not isinstance(raw_entry["owner"], str) or not raw_entry["owner"].strip():
            errors.append(f"{label} owner must be a non-empty string")
        if not isinstance(target, str) or not _valid_relative_path(target):
            errors.append(f"{label} path must be a repository-relative path without '..'")
        elif not (root / target).is_file():
            errors.append(f"{label} target does not exist: {target}")
        elif target in by_path:
            errors.append(f"duplicate authority path: {target}")
        else:
            by_path[target] = raw_entry

        try:
            date.fromisoformat(raw_entry["last_reviewed"])
        except (TypeError, ValueError):
            errors.append(f"{label} last_reviewed must be an ISO date")

        for field in ("supersedes", "superseded_by"):
            if not isinstance(raw_entry[field], list) or not all(
                isinstance(item, str) and item for item in raw_entry[field]
            ):
                errors.append(f"{label} {field} must be a list of authority IDs")

        if authority == "normative" and status in CURRENT_NORMATIVE_STATUSES:
            current_normative_by_scope[scope].append(entry_id)
        if raw_entry["kind"] == "adr" and isinstance(target, str):
            registered_adrs.add(target)

    for scope, identifiers in current_normative_by_scope.items():
        if len(identifiers) > 1:
            errors.append(
                f"scope {scope!r} has competing current normative entries: "
                + ", ".join(sorted(identifiers))
            )

    for entry_id, entry in by_id.items():
        for predecessor in entry.get("supersedes", []):
            target = by_id.get(predecessor)
            if target is None:
                errors.append(f"{entry_id} supersedes unknown authority ID {predecessor}")
            elif entry_id not in target.get("superseded_by", []):
                errors.append(f"{entry_id} and {predecessor} do not have reciprocal supersession")
        for successor in entry.get("superseded_by", []):
            target = by_id.get(successor)
            if target is None:
                errors.append(f"{entry_id} is superseded by unknown authority ID {successor}")
            elif entry_id not in target.get("supersedes", []):
                errors.append(f"{entry_id} and {successor} do not have reciprocal supersession")
        if entry.get("status") == "SUPERSEDED" and not entry.get("superseded_by"):
            errors.append(f"{entry_id} is SUPERSEDED but has no superseded_by authority ID")

    adr_directory = root / "docs" / "adrs"
    adr_index = (adr_directory / "README.md").read_text(encoding="utf-8")
    for path in sorted(adr_directory.glob("*.md")):
        if path.name in {"000-ADR-template.md", "README.md"} or not ADR_FILE_RE.match(path.name):
            continue
        relative = path.relative_to(root).as_posix()
        if relative not in registered_adrs:
            errors.append(f"ADR is not registered in docs/authority.json: {relative}")
            continue
        if path.name not in adr_index:
            errors.append(f"ADR is not linked from docs/adrs/README.md: {path.name}")

        entry = by_path[relative]
        metadata = _metadata(path)
        missing_metadata = sorted(REQUIRED_ADR_METADATA - metadata.keys())
        if missing_metadata:
            errors.append(f"{relative} is missing ADR metadata: {', '.join(missing_metadata)}")
            continue
        if metadata["status"].upper() != entry["status"]:
            errors.append(f"{relative} status does not match docs/authority.json")
        if metadata["authority"].lower() != entry["authority"]:
            errors.append(f"{relative} authority does not match docs/authority.json")
        if metadata["scope"] != entry["scope"]:
            errors.append(f"{relative} scope does not match docs/authority.json")
        if metadata["last reviewed"] != entry["last_reviewed"]:
            errors.append(f"{relative} last reviewed date does not match docs/authority.json")

    return errors


def main() -> int:
    errors = validate_repository()
    if errors:
        print("Documentation governance validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print("Documentation governance validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
