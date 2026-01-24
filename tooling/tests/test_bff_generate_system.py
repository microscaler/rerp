"""TDD: tests for rerp_tooling.bff.generate_system (rerp bff generate-system)."""

from pathlib import Path

# --- _update_refs_in_value ---


class TestUpdateRefsInValue:
    """Exact ref match: old_name must not match when it is a prefix of the schema (e.g. Error vs ErrorResponse)."""

    def test_exact_match_rewritten(self) -> None:
        from rerp_tooling.bff.generate_system import _update_refs_in_value

        val = {"$ref": "#/components/schemas/Error"}
        _update_refs_in_value(val, "Error", "ServiceError")
        assert val["$ref"] == "#/components/schemas/ServiceError"

    def test_prefix_of_schema_name_not_rewritten(self) -> None:
        from rerp_tooling.bff.generate_system import _update_refs_in_value

        val = {"$ref": "#/components/schemas/ErrorResponse"}
        _update_refs_in_value(val, "Error", "ServiceError")
        assert val["$ref"] == "#/components/schemas/ErrorResponse"

    def test_unrelated_schema_unchanged(self) -> None:
        from rerp_tooling.bff.generate_system import _update_refs_in_value

        val = {"$ref": "#/components/schemas/Foo"}
        _update_refs_in_value(val, "Error", "ServiceError")
        assert val["$ref"] == "#/components/schemas/Foo"


# --- discover_sub_services ---


class TestDiscoverSubServices:
    def test_system_dir_missing_returns_empty(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import discover_sub_services

        got = discover_sub_services(tmp_path, "nosuch")
        assert got == {}

    def test_no_subdirs_with_openapi_returns_empty(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import discover_sub_services

        (tmp_path / "foo").mkdir()
        got = discover_sub_services(tmp_path, "foo")
        assert got == {}

    def test_one_subdir_with_openapi_returns_service(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import discover_sub_services

        (tmp_path / "sys" / "svc").mkdir(parents=True)
        (tmp_path / "sys" / "svc" / "openapi.yaml").write_text(
            "openapi: 3.1.0\ninfo: {title: X, version: '1.0'}\npaths: {}"
        )
        got = discover_sub_services(tmp_path, "sys")
        assert list(got.keys()) == ["svc"]
        assert got["svc"]["spec"] == tmp_path / "sys" / "svc" / "openapi.yaml"
        assert got["svc"]["base_path"] == "/api/v1/sys/svc"

    def test_two_subdirs_returns_both(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import discover_sub_services

        for s in ["a", "b"]:
            (tmp_path / "s" / s).mkdir(parents=True)
            (tmp_path / "s" / s / "openapi.yaml").write_text("openapi: 3.1.0\ninfo: {}\npaths: {}")
        got = discover_sub_services(tmp_path, "s")
        assert set(got.keys()) == {"a", "b"}
        assert got["a"]["base_path"] == "/api/v1/s/a"
        assert got["b"]["base_path"] == "/api/v1/s/b"

    def test_skips_hidden_dirs(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import discover_sub_services

        (tmp_path / "s" / ".hidden").mkdir(parents=True)
        (tmp_path / "s" / ".hidden" / "openapi.yaml").write_text(
            "openapi: 3.1.0\ninfo: {}\npaths: {}"
        )
        (tmp_path / "s" / "ok").mkdir()
        (tmp_path / "s" / "ok" / "openapi.yaml").write_text("openapi: 3.1.0\ninfo: {}\npaths: {}")
        got = discover_sub_services(tmp_path, "s")
        assert list(got.keys()) == ["ok"]

    def test_skips_dirs_without_openapi_yaml(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import discover_sub_services

        (tmp_path / "s" / "no-spec").mkdir(parents=True)
        (tmp_path / "s" / "with-spec").mkdir()
        (tmp_path / "s" / "with-spec" / "openapi.yaml").write_text(
            "openapi: 3.1.0\ninfo: {}\npaths: {}"
        )
        got = discover_sub_services(tmp_path, "s")
        assert list(got.keys()) == ["with-spec"]

    def test_base_path_uses_system_and_service(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import discover_sub_services

        (tmp_path / "accounting" / "general-ledger").mkdir(parents=True)
        (tmp_path / "accounting" / "general-ledger" / "openapi.yaml").write_text(
            "openapi: 3.1.0\ninfo: {}\npaths: {}"
        )
        got = discover_sub_services(tmp_path, "accounting")
        assert got["general-ledger"]["base_path"] == "/api/v1/accounting/general-ledger"


# --- list_systems_with_sub_services ---


class TestListSystemsWithSubServices:
    def test_empty_openapi_dir_returns_empty(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import list_systems_with_sub_services

        assert list_systems_with_sub_services(tmp_path) == []

    def test_system_with_sub_services_included(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import list_systems_with_sub_services

        (tmp_path / "x" / "a").mkdir(parents=True)
        (tmp_path / "x" / "a" / "openapi.yaml").write_text("openapi: 3.1.0\ninfo: {}\npaths: {}")
        assert list_systems_with_sub_services(tmp_path) == ["x"]

    def test_system_without_sub_services_excluded(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import list_systems_with_sub_services

        (tmp_path / "empty").mkdir()
        (tmp_path / "with_subs" / "a").mkdir(parents=True)
        (tmp_path / "with_subs" / "a" / "openapi.yaml").write_text(
            "openapi: 3.1.0\ninfo: {}\npaths: {}"
        )
        got = list_systems_with_sub_services(tmp_path)
        assert got == ["with_subs"]

    def test_multiple_systems_sorted(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import list_systems_with_sub_services

        for sys in ["b", "a", "c"]:
            (tmp_path / sys / "x").mkdir(parents=True)
            (tmp_path / sys / "x" / "openapi.yaml").write_text(
                "openapi: 3.1.0\ninfo: {}\npaths: {}"
            )
        assert list_systems_with_sub_services(tmp_path) == ["a", "b", "c"]


# --- generate_system_bff_spec ---


def _minimal_spec(paths: dict | None = None, schemas: dict | None = None) -> str:
    p = paths if paths is not None else {}
    s = schemas if schemas is not None else {}
    import yaml

    o = {"openapi": "3.1.0", "info": {"title": "T", "version": "1.0"}, "paths": p}
    if s:
        o["components"] = {"schemas": s}
    return yaml.dump(o, sort_keys=False)


class TestGenerateSystemBffSpec:
    def test_no_sub_services_does_not_write(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import generate_system_bff_spec

        out = tmp_path / "s" / "openapi.yaml"
        generate_system_bff_spec(tmp_path, "s", output_path=out)
        assert not out.exists()

    def test_one_sub_service_writes_valid_spec(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import generate_system_bff_spec

        (tmp_path / "sys" / "svc").mkdir(parents=True)
        (tmp_path / "sys" / "svc" / "openapi.yaml").write_text(
            _minimal_spec(
                paths={
                    "/items": {
                        "get": {"summary": "List", "responses": {"200": {"description": "ok"}}}
                    }
                }
            )
        )
        out = tmp_path / "sys" / "openapi.yaml"
        generate_system_bff_spec(tmp_path, "sys", output_path=out)
        assert out.exists()
        import yaml

        data = yaml.safe_load(out.read_text())
        assert data["openapi"] == "3.1.0"
        assert "paths" in data
        assert "/items" in data["paths"]
        assert "components" in data
        assert "schemas" in data["components"]

    def test_output_path_default_when_none(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import generate_system_bff_spec

        (tmp_path / "x" / "a").mkdir(parents=True)
        (tmp_path / "x" / "a" / "openapi.yaml").write_text(_minimal_spec())
        generate_system_bff_spec(tmp_path, "x", output_path=None)
        default = tmp_path / "x" / "openapi.yaml"
        assert default.exists()

    def test_idempotent_second_run_same_content(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import generate_system_bff_spec

        (tmp_path / "s" / "v").mkdir(parents=True)
        (tmp_path / "s" / "v" / "openapi.yaml").write_text(_minimal_spec())
        out = tmp_path / "s" / "openapi.yaml"
        generate_system_bff_spec(tmp_path, "s", output_path=out)
        c1 = out.read_text()
        generate_system_bff_spec(tmp_path, "s", output_path=out)
        c2 = out.read_text()
        assert c1 == c2

    def test_schemas_prefixed_with_service_pascal(self, tmp_path: Path) -> None:
        from rerp_tooling.bff.generate_system import generate_system_bff_spec

        (tmp_path / "s" / "my-svc").mkdir(parents=True)
        (tmp_path / "s" / "my-svc" / "openapi.yaml").write_text(
            _minimal_spec(
                schemas={"Item": {"type": "object", "properties": {"id": {"type": "string"}}}}
            )
        )
        out = tmp_path / "s" / "openapi.yaml"
        generate_system_bff_spec(tmp_path, "s", output_path=out)
        import yaml

        data = yaml.safe_load(out.read_text())
        schemas = data["components"]["schemas"]
        # Original 'Item' should be prefixed as MySvcItem
        assert "MySvcItem" in schemas
        assert schemas["MySvcItem"].get("type") == "object"
