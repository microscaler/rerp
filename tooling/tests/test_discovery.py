"""Tests for rerp_tooling.discovery (suites, sources)."""

from pathlib import Path


class TestSuites:
    def test_suites_with_bff_empty_when_no_bff_config(self, tmp_path: Path):
        from rerp_tooling.discovery import suites_with_bff

        (tmp_path / "openapi" / "x").mkdir(parents=True)
        assert suites_with_bff(tmp_path) == []

    def test_suites_with_bff_returns_suites_with_config(self, tmp_path: Path):
        from rerp_tooling.discovery import suites_with_bff

        (tmp_path / "openapi" / "acct").mkdir(parents=True)
        (tmp_path / "openapi" / "acct" / "bff-suite-config.yaml").write_text(
            "bff_service_name: bff\nservices: {}"
        )
        (tmp_path / "openapi" / "other").mkdir(parents=True)
        # no bff-suite-config in other
        assert suites_with_bff(tmp_path) == ["acct"]

    def test_iter_bffs_yields_bff_name_and_suite(self, tmp_path: Path):
        from rerp_tooling.discovery import iter_bffs

        (tmp_path / "openapi" / "acct").mkdir(parents=True)
        (tmp_path / "openapi" / "acct" / "bff-suite-config.yaml").write_text(
            "bff_service_name: bff\nservices:\n  general-ledger: {}"
        )
        out = list(iter_bffs(tmp_path))
        assert out == [("bff", "acct")]

    def test_bff_suite_config_path(self, tmp_path: Path):
        from rerp_tooling.discovery import bff_suite_config_path

        p = bff_suite_config_path(tmp_path, "acct")
        assert p == tmp_path / "openapi" / "acct" / "bff-suite-config.yaml"

    def test_service_to_suite_finds_spec(self, tmp_path: Path):
        from rerp_tooling.discovery import service_to_suite

        (tmp_path / "openapi" / "acct" / "general-ledger").mkdir(parents=True)
        (tmp_path / "openapi" / "acct" / "general-ledger" / "openapi.yaml").write_text(
            "openapi: 3.0.3"
        )
        assert service_to_suite(tmp_path, "general-ledger") == "acct"

    def test_service_to_suite_none_when_missing(self, tmp_path: Path):
        from rerp_tooling.discovery import service_to_suite

        assert service_to_suite(tmp_path, "nonexistent") is None

    def test_suite_sub_service_names_empty_when_no_suite_dir(self, tmp_path: Path):
        from rerp_tooling.discovery import suite_sub_service_names

        assert suite_sub_service_names(tmp_path, "acct") == []

    def test_suite_sub_service_names_returns_dirs_with_openapi_yaml(self, tmp_path: Path):
        from rerp_tooling.discovery import suite_sub_service_names

        (tmp_path / "openapi" / "acct" / "svc-a").mkdir(parents=True)
        (tmp_path / "openapi" / "acct" / "svc-a" / "openapi.yaml").write_text("")
        (tmp_path / "openapi" / "acct" / "svc-b").mkdir(parents=True)
        (tmp_path / "openapi" / "acct" / "svc-b" / "openapi.yaml").write_text("")
        (tmp_path / "openapi" / "acct" / "no-spec").mkdir(parents=True)
        assert suite_sub_service_names(tmp_path, "acct") == ["svc-a", "svc-b"]

    def test_tilt_service_names_includes_bff_and_services(self, tmp_path: Path):
        from rerp_tooling.discovery import tilt_service_names

        (tmp_path / "openapi" / "acct").mkdir(parents=True)
        (tmp_path / "openapi" / "acct" / "bff-suite-config.yaml").write_text(
            "bff_service_name: bff\nservices:\n  a: {}\n  b: {}"
        )
        assert tilt_service_names(tmp_path) == ["a", "b", "bff"]

    def test_tilt_service_names_empty_when_no_bff_config(self, tmp_path: Path):
        from rerp_tooling.discovery import tilt_service_names

        assert tilt_service_names(tmp_path) == []
