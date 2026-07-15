"""Regression tests for the RERP CLI wrapper's suite-aware translations.

RERP intentionally keeps a nested suite layout:

    openapi/{suite}/{service}/openapi.yaml
    microservices/{suite}/{service}/{gen,impl}

These tests protect that layout from being "simplified" back into the flat
Hauliage shape. Hauliage's naming convention still applies, but with a suite
prefix: implementation crates are ``rerp_{suite}_{service}`` and generated
crates are ``rerp_{suite}_{service}_gen``.
"""

import importlib
import sys
from pathlib import Path

import pytest
import yaml

cli = importlib.import_module("rerp_tooling.cli.main")


def _write_manifest(path: Path, package_name: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        f"""[package]
name = "{package_name}"
version = "0.1.0"
edition = "2021"
""",
    )


def test_gen_package_name_uses_suite_prefixed_gen_suffix() -> None:
    assert cli._rerp_gen_package_name("accounting", "general-ledger") == (
        "rerp_accounting_general_ledger_gen"
    )


def test_gen_stubs_forwards_full_gen_command(monkeypatch) -> None:
    forwarded = []
    monkeypatch.setattr(
        cli.gen_cmd,
        "run_gen_argv",
        lambda: forwarded.append(list(sys.argv)),
    )
    monkeypatch.setattr(
        sys,
        "argv",
        ["rerp", "gen", "stubs", "documents", "render", "--force"],
    )

    cli.main()

    assert forwarded == [
        ["brrtrouter", "gen", "stubs", "documents", "render", "--force"]
    ]


def test_microservice_build_is_debug_by_default(monkeypatch, tmp_path: Path) -> None:
    observed = []
    monkeypatch.setattr(
        cli,
        "build_microservice",
        lambda root, suite, service, release: observed.append(
            (root, suite, service, release)
        )
        or 0,
    )

    result = cli._run_microservice_build(
        ["microservice", "render", "--suite", "documents"],
        project_root=tmp_path,
    )

    assert result == 0
    assert observed == [(tmp_path, "documents", "render", False)]


def test_build_microservice_targets_impl_manifest_package(tmp_path: Path) -> None:
    _write_manifest(
        tmp_path / "microservices/accounting/general-ledger/impl/Cargo.toml",
        "rerp_accounting_general_ledger",
    )

    translated = cli._translate_build(
        ["microservice", "general-ledger", "--release"],
        project_root=tmp_path,
    )

    assert translated == [
        "build",
        "accounting_general_ledger",
        "--package",
        "rerp_accounting_general_ledger",
        "--release",
    ]


def test_build_microservice_preserves_existing_impl_suffix(tmp_path: Path) -> None:
    _write_manifest(
        tmp_path / "microservices/accounting/accounts-receivable/impl/Cargo.toml",
        "rerp_accounting_accounts_receivable_impl",
    )

    translated = cli._translate_build(
        ["microservice", "accounts-receivable"],
        project_root=tmp_path,
    )

    assert translated == [
        "build",
        "accounting_accounts_receivable",
        "--package",
        "rerp_accounting_accounts_receivable_impl",
    ]


def test_bff_generate_system_defaults_to_suite_local_output(tmp_path: Path) -> None:
    config = tmp_path / "openapi/accounting/bff-suite-config.yaml"
    config.parent.mkdir(parents=True, exist_ok=True)
    config.write_text("suite: accounting\nbff_service_name: bff\n")

    plans = cli._bff_generate_system_plans([], project_root=tmp_path)

    assert plans == [
        (
            tmp_path / "openapi",
            "accounting",
            tmp_path / "openapi/accounting/openapi_bff.yaml",
        )
    ]


def test_bff_generate_system_reads_all_services_from_suite_config(tmp_path: Path) -> None:
    """`generate-system` must aggregate the suite config, not stale discovery output."""
    accounting = tmp_path / "openapi/accounting"
    for service_name, operation_id in {
        "invoice": "approve_invoice",
        "bank-sync": "auto_match_transactions",
    }.items():
        service_dir = accounting / service_name
        service_dir.mkdir(parents=True, exist_ok=True)
        (service_dir / "openapi.yaml").write_text(
            f"""openapi: 3.1.0
info:
  title: {service_name}
  version: "1.0"
paths:
  /{service_name}:
    get:
      operationId: {operation_id}
      responses:
        "200":
          description: OK
""",
        )

    config = accounting / "bff-suite-config.yaml"
    config.write_text(
        """suite: accounting
bff_service_name: bff
openapi_base_dir: openapi/accounting
output_path: openapi/accounting/openapi_bff.yaml
services:
  invoice:
    base_path: /api/invoice
    spec_path: invoice/openapi.yaml
  bank-sync:
    base_path: /api/bank-sync
    spec_path: bank-sync/openapi.yaml
""",
    )

    cli._run_bff_generate_system(["--suite", "accounting"], project_root=tmp_path)

    generated = yaml.safe_load((accounting / "openapi_bff.yaml").read_text())
    operation_ids = {
        operation.get("operationId")
        for path_item in generated["paths"].values()
        for method, operation in path_item.items()
        if method in {"get", "post", "put", "patch", "delete"}
    }
    assert {"approve_invoice", "auto_match_transactions"} <= operation_ids


def test_bff_generate_system_rejects_duplicate_operation_ids(tmp_path: Path) -> None:
    accounting = tmp_path / "openapi" / "accounting"
    for service_name in ("accounts-payable", "accounts-receivable"):
        service_dir = accounting / service_name
        service_dir.mkdir(parents=True, exist_ok=True)
        (service_dir / "openapi.yaml").write_text(
            f"""openapi: 3.1.0
info:
  title: {service_name}
  version: "1.0"
paths:
  /payments:
    get:
      operationId: list_payments
      responses:
        "200":
          description: OK
""",
        )

    (accounting / "bff-suite-config.yaml").write_text(
        """suite: accounting
openapi_base_dir: openapi/accounting
output_path: openapi/accounting/openapi_bff.yaml
services:
  accounts-payable:
    base_path: /api/accounts-payable
    gateway_path_style: prefixed
    spec_path: accounts-payable/openapi.yaml
  accounts-receivable:
    base_path: /api/accounts-receivable
    gateway_path_style: prefixed
    spec_path: accounts-receivable/openapi.yaml
""",
    )

    with pytest.raises(ValueError, match="Operation ID conflict"):
        cli._run_bff_generate_system(["--suite", "accounting"], project_root=tmp_path)


def test_bff_generate_suite_shorthand_uses_suite_config(tmp_path: Path) -> None:
    translated = cli._translate_bff_generate(["--suite", "accounting"], project_root=tmp_path)

    assert translated == [
        "--suite-config",
        str(tmp_path / "openapi/accounting/bff-suite-config.yaml"),
        "--output",
        str(tmp_path / "openapi/accounting/openapi_bff.yaml"),
        "--base-dir",
        str(tmp_path),
    ]
