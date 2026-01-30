"""Tests for discovery.services: get_package_names, get_binary_names, get_service_ports (no hardcoding)."""

from pathlib import Path

from rerp_tooling.discovery.services import (
    get_binary_names,
    get_package_names,
    get_service_ports,
)


def _make_openapi_spec(project_root: Path, suite: str, service: str, port: int = 8001) -> None:
    d = project_root / "openapi" / suite / service
    d.mkdir(parents=True)
    (d / "openapi.yaml").write_text(
        f"openapi: 3.1.0\ninfo: {{}}\nservers:\n  - url: http://localhost:{port}/api/v1/{suite}/{service}\n"
    )


def test_get_package_names_empty(tmp_path: Path) -> None:
    assert get_package_names(tmp_path) == {}


def test_get_package_names_derived(tmp_path: Path) -> None:
    _make_openapi_spec(tmp_path, "accounting", "general-ledger")
    _make_openapi_spec(tmp_path, "accounting", "invoice")
    got = get_package_names(tmp_path)
    assert got == {
        "general-ledger": "rerp_accounting_general_ledger",
        "invoice": "rerp_accounting_invoice",
    }


def test_get_binary_names_empty(tmp_path: Path) -> None:
    assert get_binary_names(tmp_path) == {}


def test_get_binary_names_derived(tmp_path: Path) -> None:
    _make_openapi_spec(tmp_path, "accounting", "general-ledger")
    got = get_binary_names(tmp_path)
    assert got == {"general-ledger": "general_ledger"}


def test_get_package_names_includes_bff(tmp_path: Path) -> None:
    """BFF from bff-suite-config is included so brrtrouter-gen gets --package-name (e.g. rerp_accounting_bff_gen)."""
    _make_openapi_spec(tmp_path, "accounting", "general-ledger")
    (tmp_path / "openapi" / "accounting").mkdir(parents=True, exist_ok=True)
    (tmp_path / "openapi" / "accounting" / "bff-suite-config.yaml").write_text(
        "suite: accounting\nbff_service_name: bff\noutput_path: openapi/accounting/openapi_bff.yaml\n"
    )
    got = get_package_names(tmp_path)
    assert got["general-ledger"] == "rerp_accounting_general_ledger"
    assert got["bff"] == "rerp_accounting_bff"


def test_get_binary_names_includes_bff(tmp_path: Path) -> None:
    (tmp_path / "openapi" / "accounting").mkdir(parents=True, exist_ok=True)
    (tmp_path / "openapi" / "accounting" / "bff-suite-config.yaml").write_text(
        "suite: accounting\nbff_service_name: bff\n"
    )
    got = get_binary_names(tmp_path)
    assert got["bff"] == "bff"


def test_get_service_ports_from_openapi(tmp_path: Path) -> None:
    _make_openapi_spec(tmp_path, "accounting", "general-ledger", port=8001)
    _make_openapi_spec(tmp_path, "accounting", "invoice", port=8002)
    got = get_service_ports(tmp_path)
    assert got["general-ledger"] == "8001"
    assert got["invoice"] == "8002"


def test_get_service_ports_empty(tmp_path: Path) -> None:
    assert get_service_ports(tmp_path) == {}
