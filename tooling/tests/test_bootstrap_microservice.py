"""Tests for rerp_tooling.bootstrap.microservice."""

import json
from pathlib import Path

from rerp_tooling.bootstrap.microservice import (
    _get_port_from_registry,
    derive_binary_name,
    load_openapi_spec,
    run_bootstrap_microservice,
    to_pascal_case,
    to_snake_case,
    update_tiltfile,
    update_workspace_cargo_toml,
)


class TestGetPortFromRegistry:
    def test_missing_registry_returns_none(self, tmp_path: Path):
        assert _get_port_from_registry(tmp_path, "x") is None

    def test_reads_assignments(self, tmp_path: Path):
        (tmp_path / "port-registry.json").write_text(json.dumps({"assignments": {"svc": 8001}}))
        assert _get_port_from_registry(tmp_path, "svc") == 8001


class TestRunBootstrapMicroservice:
    def test_returns_1_when_spec_missing(self, tmp_path: Path):
        rc = run_bootstrap_microservice("nosuch", 8001, tmp_path)
        assert rc == 1

    def test_returns_1_when_no_port_and_not_in_registry(self, tmp_path: Path):
        (tmp_path / "openapi" / "accounting" / "x").mkdir(parents=True)
        (tmp_path / "openapi" / "accounting" / "x" / "openapi.yaml").write_text(
            "openapi: 3.1.0\ninfo: {}\n"
        )
        rc = run_bootstrap_microservice("x", None, tmp_path)
        assert rc == 1


class TestToSnakeCase:
    def test_hyphens(self):
        assert to_snake_case("general-ledger") == "general_ledger"

    def test_spaces(self):
        assert to_snake_case("foo bar") == "foo_bar"


class TestToPascalCase:
    def test_hyphens(self):
        assert to_pascal_case("general-ledger") == "GeneralLedger"


class TestDeriveBinaryName:
    def test_from_title(self):
        assert (
            derive_binary_name({"info": {"title": "Invoice Management"}}, "x")
            == "invoice_management_service_api"
        )

    def test_fallback_service_name(self):
        assert derive_binary_name({"info": {}}, "general-ledger") == "general_ledger_service_api"


class TestLoadOpenapiSpec:
    def test_loads_yaml(self, tmp_path: Path):
        (tmp_path / "openapi.yaml").write_text("openapi: 3.1.0\ninfo:\n  title: X\n")
        assert load_openapi_spec(tmp_path / "openapi.yaml")["info"]["title"] == "X"


class TestUpdateWorkspaceCargoToml:
    """Regression: update_workspace_cargo_toml must preserve content after members = [...]."""

    def test_preserves_content_after_members_array(self, tmp_path: Path):
        cargo = tmp_path / "Cargo.toml"
        cargo.write_text(
            "[workspace]\n"
            "members = [\n"
            '    "accounting/foo",\n'
            "]\n"
            'resolver = "2"\n\n'
            "[workspace.package]\n"
            'version = "0.1.0"\n\n'
            "[workspace.dependencies]\n"
            'serde = "1.0"\n'
        )
        update_workspace_cargo_toml("bar", cargo)
        text = cargo.read_text()
        assert '"accounting/bar"' in text
        assert 'resolver = "2"' in text
        assert "[workspace.package]" in text
        assert "[workspace.dependencies]" in text
        assert 'serde = "1.0"' in text

    def test_no_op_if_missing(self, tmp_path: Path):
        update_workspace_cargo_toml("x", tmp_path / "Cargo.toml")
        assert not (tmp_path / "Cargo.toml").exists()

    def test_no_op_if_already_member(self, tmp_path: Path):
        cargo = tmp_path / "Cargo.toml"
        cargo.write_text('[workspace]\nmembers = ["accounting/baz"]\nresolver = "2"\n')
        update_workspace_cargo_toml("baz", cargo)
        assert cargo.read_text().count('"accounting/baz"') == 1


class TestUpdateTiltfile:
    """Regression: update_tiltfile must include m.group(3) in all three regex replacements."""

    def test_ports_dict_preserves_closing_brace(self, tmp_path: Path) -> None:
        # Pattern (ports\s*=\s*\{)(.*?)(\s*\}) — group 3 is \s*}. No DOTALL, so use one-line dict.
        tilt = tmp_path / "Tiltfile"
        tilt.write_text(
            "def get_service_port(name):\n"
            "    ports = { 'a': '8001' }\n"
            "    return ports.get(name, '8080')\n"
        )
        update_tiltfile("b", "b/openapi.yaml", "b_api", 8002, tilt)
        text = tilt.read_text()
        assert "'b': '8002'" in text
        assert "return ports.get" in text
        # dict must close before "return" — ensure we didn't drop m.group(3) =\s*}
        before_return = text.split("return ports.get")[0]
        assert before_return.rstrip().endswith("}"), (
            "ports dict must be closed with } before return"
        )

    def test_resource_deps_preserves_labels(self, tmp_path: Path) -> None:
        # Pattern (resource_deps=\[)(.*?)(\]\s*labels=\['microservices-build'\])
        tilt = tmp_path / "Tiltfile"
        tilt.write_text(
            "local_resource('x', 'echo', resource_deps=['a-service-gen']\n"
            "    labels=['microservices-build'],\n"
            ")\n"
        )
        update_tiltfile("b", "b/openapi.yaml", "b_api", 8002, tilt)
        text = tilt.read_text()
        assert "labels=['microservices-build']" in text
        assert "'b-service-gen'" in text

    def test_deps_preserves_resource_deps(self, tmp_path: Path) -> None:
        # Pattern (deps=\[)(.*?)(\]\s*resource_deps=) — group 3 is ]\s*resource_deps=.
        # Regex requires ] then only \s then resource_deps= (no comma in between).
        tilt = tmp_path / "Tiltfile"
        tilt.write_text(
            "local_resource('build-x', 'echo',\n"
            "    deps=['./microservices/accounting/a/Cargo.toml']\n"
            "    resource_deps=['accounting-all-gens'],\n"
            ")\n"
        )
        update_tiltfile("b", "b/openapi.yaml", "b_api", 8002, tilt)
        text = tilt.read_text()
        assert "resource_deps=" in text
        assert "'./microservices/accounting/b/Cargo.toml'" in text

    def test_binary_names_new_entry_sorts_alphabetically_and_has_consistent_indent(
        self, tmp_path: Path
    ) -> None:
        # Regression: new entries were appended with 4 leading spaces while
        # existing were stripped. ASCII space (32) < apostrophe (39), so
        # indented new entries sorted to the top. Fix: keep normalized
        # (stripped) form for sort, apply indent when emitting.
        tilt = tmp_path / "Tiltfile"
        tilt.write_text(
            "BINARY_NAMES = {\n"
            "    'accounts-receivable': 'ar_api',\n"
            "    'bank-sync': 'bank_sync',\n"
            "}\n"
        )
        update_tiltfile("asset", "asset/openapi.yaml", "asset_api", 8006, tilt)
        text = tilt.read_text()
        # 'asset' must be between 'accounts-receivable' and 'bank-sync'
        ar_pos = text.find("'accounts-receivable'")
        asset_pos = text.find("'asset'")
        bank_pos = text.find("'bank-sync'")
        assert ar_pos < asset_pos < bank_pos, (
            "asset must sort between accounts-receivable and bank-sync"
        )
        # All entries must have 4-space indent (not 0 from strip, not 8)
        assert "    'accounts-receivable': 'ar_api'," in text
        assert "    'asset': 'asset_api'," in text
        assert "    'bank-sync': 'bank_sync'," in text

    def test_ports_new_entry_sorts_alphabetically_and_has_consistent_indent(
        self, tmp_path: Path
    ) -> None:
        # Same regression as BINARY_NAMES: new port entries had 8 spaces,
        # sorted to top. Fix: normalized form for sort, 8-space indent when emitting.
        tilt = tmp_path / "Tiltfile"
        tilt.write_text(
            "def get_service_port(name):\n"
            "    ports = {\n"
            "        'accounts-receivable': '8003',\n"
            "        'bank-sync': '8005',\n"
            "    }\n"
            "    return ports.get(name, '8080')\n"
        )
        update_tiltfile("asset", "asset/openapi.yaml", "asset_api", 8006, tilt)
        text = tilt.read_text()
        ar_pos = text.find("'accounts-receivable'")
        asset_pos = text.find("'asset'")
        bank_pos = text.find("'bank-sync'")
        assert ar_pos < asset_pos < bank_pos, (
            "asset must sort between accounts-receivable and bank-sync"
        )
        assert "        'accounts-receivable': '8003'," in text
        assert "        'asset': '8006'," in text
        assert "        'bank-sync': '8005'," in text
