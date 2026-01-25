"""Tests for rerp_tooling.release.bump."""

from pathlib import Path

import pytest

from rerp_tooling.release.bump import (
    _cargo_toml_paths,
    _next_version,
    _read_current,
    _replace_in_file,
    run,
)


class TestReadCurrent:
    def test_reads_workspace_package_version(self, tmp_path: Path) -> None:
        (tmp_path / "components").mkdir()
        (tmp_path / "components" / "Cargo.toml").write_text(
            '[workspace]\nmembers = []\n\n[workspace.package]\nversion = "1.2.3"\nedition = "2021"\n'
        )
        assert _read_current(tmp_path / "components" / "Cargo.toml") == "1.2.3"

    def test_reads_v_prefix_strips_to_canonical(self, tmp_path: Path) -> None:
        (tmp_path / "components").mkdir()
        (tmp_path / "components" / "Cargo.toml").write_text(
            '[workspace]\n[workspace.package]\nversion = "v0.1.0"\n'
        )
        assert _read_current(tmp_path / "components" / "Cargo.toml") == "0.1.0"

    def test_missing_raises(self, tmp_path: Path) -> None:
        (tmp_path / "components").mkdir()
        (tmp_path / "components" / "Cargo.toml").write_text("[workspace]\nmembers = []\n")
        with pytest.raises(SystemExit):
            _read_current(tmp_path / "components" / "Cargo.toml")


class TestNextVersion:
    def test_patch(self) -> None:
        assert _next_version("0.1.0", "patch") == "0.1.1"
        assert _next_version("1.2.3", "patch") == "1.2.4"

    def test_minor(self) -> None:
        assert _next_version("0.1.0", "minor") == "0.2.0"
        assert _next_version("1.2.3", "minor") == "1.3.0"

    def test_major(self) -> None:
        assert _next_version("0.1.0", "major") == "1.0.0"
        assert _next_version("1.2.3", "major") == "2.0.0"

    def test_strips_v_prefix(self) -> None:
        assert _next_version("v0.1.0", "patch") == "0.1.1"
        assert _next_version("v1.2.3", "minor") == "1.3.0"


class TestReplaceInFile:
    def test_replaces_in_package_section(self, tmp_path: Path) -> None:
        p = tmp_path / "Cargo.toml"
        p.write_text(
            '[package]\nname = "x"\nversion = "0.1.0"\n\n[dependencies]\nserde = { version = "1.0" }\n'
        )
        assert _replace_in_file(p, "0.1.0", "0.1.1") is True
        assert 'version = "0.1.1"' in p.read_text()
        assert 'version = "1.0"' in p.read_text()  # dep unchanged

    def test_replaces_in_workspace_package_section(self, tmp_path: Path) -> None:
        p = tmp_path / "Cargo.toml"
        p.write_text('[workspace]\nmembers = []\n\n[workspace.package]\nversion = "0.1.0"\n')
        assert _replace_in_file(p, "0.1.0", "0.2.0") is True
        assert 'version = "0.2.0"' in p.read_text()

    def test_does_not_replace_in_dependencies(self, tmp_path: Path) -> None:
        p = tmp_path / "Cargo.toml"
        p.write_text(
            '[package]\nversion = "0.1.0"\n\n[dependencies]\nfoo = { version = "0.1.0" }\n'
        )
        assert _replace_in_file(p, "0.1.0", "0.1.1") is True
        # [package].version updated; [dependencies].foo.version must stay 0.1.0
        text = p.read_text()
        assert text.count('version = "0.1.1"') == 1
        assert 'version = "0.1.0"' in text  # the dep

    def test_returns_false_when_old_not_found(self, tmp_path: Path) -> None:
        p = tmp_path / "Cargo.toml"
        p.write_text('[package]\nversion = "0.2.0"\n')
        assert _replace_in_file(p, "0.1.0", "0.1.1") is False
        assert 'version = "0.2.0"' in p.read_text()

    def test_replaces_v_prefix_and_writes_without_v(self, tmp_path: Path) -> None:
        p = tmp_path / "Cargo.toml"
        p.write_text('[package]\nname = "x"\nversion = "v0.1.0"\n')
        assert _replace_in_file(p, "0.1.0", "0.1.1") is True
        assert 'version = "0.1.1"' in p.read_text()
        assert "v0.1.1" not in p.read_text()


class TestCargoTomlPaths:
    def test_excludes_target(self, tmp_path: Path) -> None:
        (tmp_path / "components").mkdir()
        (tmp_path / "components" / "Cargo.toml").write_text("")
        (tmp_path / "target").mkdir()
        (tmp_path / "target" / "Cargo.toml").write_text("")
        got = _cargo_toml_paths(tmp_path)
        assert len(got) == 1
        assert "target" not in got[0].parts

    def test_excludes_venv(self, tmp_path: Path) -> None:
        (tmp_path / "a").mkdir()
        (tmp_path / "a" / "Cargo.toml").write_text("")
        (tmp_path / ".venv").mkdir()
        (tmp_path / ".venv" / "x").mkdir()
        (tmp_path / ".venv" / "x" / "Cargo.toml").write_text("")
        got = _cargo_toml_paths(tmp_path)
        assert len(got) == 1
        assert ".venv" not in str(got[0])

    def test_includes_root_components_entities_microservices(self, tmp_path: Path) -> None:
        (tmp_path / "Cargo.toml").write_text("[workspace]\n[workspace.package]\nversion = \"0.1.0\"\n")
        (tmp_path / "components").mkdir()
        (tmp_path / "components" / "Cargo.toml").write_text("[workspace]\n[workspace.package]\nversion = \"0.1.0\"\n")
        (tmp_path / "entities").mkdir()
        (tmp_path / "entities" / "Cargo.toml").write_text("[package]\nversion = \"0.1.0\"\n")
        (tmp_path / "microservices").mkdir()
        (tmp_path / "microservices" / "Cargo.toml").write_text("[workspace]\n[workspace.package]\nversion = \"0.1.0\"\n")
        got = _cargo_toml_paths(tmp_path)
        rels = [str(p.relative_to(tmp_path)) for p in got]
        assert "Cargo.toml" in rels
        assert "components/Cargo.toml" in rels
        assert "entities/Cargo.toml" in rels
        assert "microservices/Cargo.toml" in rels
        assert len(got) == 4


class TestRun:
    def test_success_updates_all_matching(self, tmp_path: Path) -> None:
        (tmp_path / "Cargo.toml").write_text('[workspace]\n[workspace.package]\nversion = "0.1.0"\n')
        (tmp_path / "components").mkdir()
        (tmp_path / "components" / "Cargo.toml").write_text(
            '[workspace]\n\n[workspace.package]\nversion = "0.1.0"\n'
        )
        (tmp_path / "microservices").mkdir()
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[workspace]\n\n[workspace.package]\nversion = "0.1.0"\n'
        )
        (tmp_path / "entities").mkdir()
        (tmp_path / "entities" / "Cargo.toml").write_text('[package]\nname = "e"\nversion = "0.1.0"\n')

        rc = run(tmp_path, "patch")

        assert rc == 0
        assert 'version = "0.1.1"' in (tmp_path / "Cargo.toml").read_text()
        assert 'version = "0.1.1"' in (tmp_path / "components" / "Cargo.toml").read_text()
        assert 'version = "0.1.1"' in (tmp_path / "microservices" / "Cargo.toml").read_text()
        assert 'version = "0.1.1"' in (tmp_path / "entities" / "Cargo.toml").read_text()

    def test_fails_without_components(self, tmp_path: Path) -> None:
        assert run(tmp_path, "patch") == 1
