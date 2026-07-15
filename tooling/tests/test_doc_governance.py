from __future__ import annotations

import json
import sys
from copy import deepcopy
from pathlib import Path

REPOSITORY_ROOT = Path(__file__).resolve().parents[2]
TOOLING_ROOT = REPOSITORY_ROOT / "tooling"
sys.path.insert(0, str(TOOLING_ROOT))

from scripts.check_doc_governance import validate_repository  # noqa: E402


def test_repository_document_governance_is_valid() -> None:
    assert validate_repository(REPOSITORY_ROOT) == []


def _minimal_repository(tmp_path: Path, entries: list[dict[str, object]]) -> Path:
    (tmp_path / "docs" / "adrs").mkdir(parents=True)
    (tmp_path / "docs" / "adrs" / "README.md").write_text("# ADRs\n", encoding="utf-8")
    for entry in entries:
        target = tmp_path / str(entry["path"])
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text("placeholder\n", encoding="utf-8")
    (tmp_path / "docs" / "authority.json").write_text(
        json.dumps({"schema_version": 1, "entries": entries}), encoding="utf-8"
    )
    return tmp_path


def _policy_entry(identifier: str, scope: str, path: str) -> dict[str, object]:
    return {
        "id": identifier,
        "scope": scope,
        "path": path,
        "kind": "policy",
        "status": "ACTIVE",
        "authority": "normative",
        "owner": "test",
        "last_reviewed": "2026-07-15",
        "supersedes": [],
        "superseded_by": [],
    }


def test_rejects_competing_current_normative_authorities(tmp_path: Path) -> None:
    first = _policy_entry("POL-001", "shared.scope", "docs/first.md")
    second = _policy_entry("POL-002", "shared.scope", "docs/second.md")
    errors = validate_repository(_minimal_repository(tmp_path, [first, second]))
    assert any("competing current normative entries" in error for error in errors)


def test_rejects_non_reciprocal_supersession(tmp_path: Path) -> None:
    previous = _policy_entry("POL-001", "old.scope", "docs/old.md")
    previous["status"] = "SUPERSEDED"
    previous["superseded_by"] = ["POL-002"]
    replacement = _policy_entry("POL-002", "new.scope", "docs/new.md")
    errors = validate_repository(_minimal_repository(tmp_path, [previous, replacement]))
    assert any("do not have reciprocal supersession" in error for error in errors)


def test_accepts_reciprocal_supersession(tmp_path: Path) -> None:
    previous = _policy_entry("POL-001", "shared.scope", "docs/old.md")
    previous["status"] = "SUPERSEDED"
    previous["authority"] = "informative"
    previous["superseded_by"] = ["POL-002"]
    replacement = deepcopy(_policy_entry("POL-002", "shared.scope", "docs/new.md"))
    replacement["supersedes"] = ["POL-001"]
    assert validate_repository(_minimal_repository(tmp_path, [previous, replacement])) == []
