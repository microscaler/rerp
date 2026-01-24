"""Pytest fixtures for RERP tooling tests."""

from pathlib import Path

import pytest


@pytest.fixture
def repo_root() -> Path:
    """Repo root (parent of tooling/). Use for integration tests that need real openapi/, etc."""
    p = Path(__file__).resolve().parents[1]
    assert (p / "pyproject.toml").exists(), f"expected tooling/pyproject.toml at {p}"
    root = p.parent
    assert (root / "tooling" / "pyproject.toml").exists(), f"expected repo root at {root}"
    return root


@pytest.fixture
def tmp_openapi_dir(tmp_path: Path):
    """Temporary openapi-like tree for unit tests. Returns (root, openapi_path)."""
    openapi = tmp_path / "openapi"
    openapi.mkdir()
    return tmp_path, openapi
