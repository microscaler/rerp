"""Tests for rerp_tooling.release.bump."""

from pathlib import Path
from unittest.mock import patch

import pytest

from rerp_tooling.release.bump import (
    _cargo_toml_paths,
    _next_version,
    _read_current,
    _replace_in_file,
    _set_workspace_package_version,
    run,
)


class TestReadCurrent:
    def test_reads_workspace_package_version(self, tmp_path: Path) -> None:
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[workspace]\nmembers = []\n\n[workspace.package]\nversion = "1.2.3"\nedition = "2021"\n'
        )
        assert _read_current(tmp_path / "microservices" / "Cargo.toml") == "1.2.3"

    def test_reads_v_prefix_strips_to_canonical(self, tmp_path: Path) -> None:
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[workspace]\n[workspace.package]\nversion = "v0.1.0"\n'
        )
        assert _read_current(tmp_path / "microservices" / "Cargo.toml") == "0.1.0"

    def test_reads_prerelease_rc(self, tmp_path: Path) -> None:
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[workspace.package]\nversion = "0.39.0-rc.2"\n'
        )
        assert _read_current(tmp_path / "microservices" / "Cargo.toml") == "0.39.0-rc.2"

    def test_missing_raises(self, tmp_path: Path) -> None:
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text("[workspace]\nmembers = []\n")
        with pytest.raises(SystemExit):
            _read_current(tmp_path / "microservices" / "Cargo.toml")

    def test_raises_when_only_package_section(self, tmp_path: Path) -> None:
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[package]\nname = "x"\nversion = "1.0.0"\n'
        )
        with pytest.raises(SystemExit):
            _read_current(tmp_path / "microservices" / "Cargo.toml")

    def test_raises_when_workspace_package_version_malformed(self, tmp_path: Path) -> None:
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[workspace.package]\nversion = "1.2"\nedition = "2021"\n'
        )
        with pytest.raises(SystemExit):
            _read_current(tmp_path / "microservices" / "Cargo.toml")


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

    def test_invalid_version_raises(self) -> None:
        for bad in ("1.2", "1.2.3.4", "x.y.z"):
            with pytest.raises(SystemExit):
                _next_version(bad, "patch")

    def test_invalid_bump_raises(self) -> None:
        with pytest.raises(SystemExit):
            _next_version("1.2.3", "foo")

    # --- rc and release (prerelease) ---

    def test_rc_from_full_version(self) -> None:
        assert _next_version("0.39.0", "rc") == "0.39.0-rc.1"
        assert _next_version("v1.0.0", "rc") == "1.0.0-rc.1"

    def test_rc_bumps_rc_number(self) -> None:
        assert _next_version("0.39.0-rc.2", "rc") == "0.39.0-rc.3"
        assert _next_version("0.39.0-rc.1", "rc") == "0.39.0-rc.2"

    def test_rc_only_supports_rc_n_prerelease(self) -> None:
        with pytest.raises(SystemExit, match=r"rc bump only supports -rc\.N"):
            _next_version("1.2.3-alpha.1", "rc")

    def test_release_promotes_rc_to_full(self) -> None:
        assert _next_version("0.39.0-rc.2", "release") == "0.39.0"
        assert _next_version("v0.39.0-rc.2", "release") == "0.39.0"

    def test_promote_alias_for_release(self) -> None:
        assert _next_version("0.39.0-rc.2", "promote") == "0.39.0"

    def test_release_on_full_version_raises(self) -> None:
        with pytest.raises(SystemExit, match="Already a full release"):
            _next_version("0.39.0", "release")

    def test_patch_minor_major_strip_prerelease_then_bump(self) -> None:
        assert _next_version("0.39.0-rc.2", "patch") == "0.39.1"
        assert _next_version("0.39.0-rc.2", "minor") == "0.40.0"
        assert _next_version("0.39.0-rc.2", "major") == "1.0.0"

    def test_other_prerelease_stripped_then_bump(self) -> None:
        assert _next_version("1.2.3-beta", "patch") == "1.2.4"


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

    def test_replaces_both_package_and_workspace_package_version(self, tmp_path: Path) -> None:
        p = tmp_path / "Cargo.toml"
        p.write_text(
            '[package]\nname = "x"\nversion = "0.1.0"\n\n'
            '[workspace.package]\nversion = "0.1.0"\nedition = "2021"\n'
        )
        assert _replace_in_file(p, "0.1.0", "0.1.1") is True
        text = p.read_text()
        assert text.count('version = "0.1.1"') == 2
        assert 'version = "0.1.0"' not in text

    def test_replaces_prerelease_version(self, tmp_path: Path) -> None:
        p = tmp_path / "Cargo.toml"
        p.write_text('[package]\nname = "x"\nversion = "0.39.0-rc.2"\n')
        assert _replace_in_file(p, "0.39.0-rc.2", "0.39.0-rc.3") is True
        assert 'version = "0.39.0-rc.3"' in p.read_text()


class TestCargoTomlPaths:
    def test_excludes_target(self, tmp_path: Path) -> None:
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text("")
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

    def test_excludes_venv_node_modules_tmp_and_artifacts(self, tmp_path: Path) -> None:
        (tmp_path / "lib").mkdir()
        (tmp_path / "lib" / "Cargo.toml").write_text('[package]\nversion = "0.1.0"\n')
        for skip in ("venv", "node_modules", "node_packages", "tmp", "__pycache__"):
            (tmp_path / skip).mkdir(parents=True)
            (tmp_path / skip / "Cargo.toml").write_text("")
        got = _cargo_toml_paths(tmp_path)
        rels = [str(p.relative_to(tmp_path)) for p in got]
        assert rels == ["lib/Cargo.toml"]
        for skip in ("venv", "node_modules", "node_packages", "tmp", "__pycache__"):
            assert not any(skip in r for r in rels)

    def test_includes_root_microservices_entities(self, tmp_path: Path) -> None:
        (tmp_path / "Cargo.toml").write_text(
            '[workspace]\n[workspace.package]\nversion = "0.1.0"\n'
        )
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[workspace]\n[workspace.package]\nversion = "0.1.0"\n'
        )
        (tmp_path / "entities").mkdir(exist_ok=True)
        (tmp_path / "entities" / "Cargo.toml").write_text('[package]\nversion = "0.1.0"\n')
        got = _cargo_toml_paths(tmp_path)
        rels = [str(p.relative_to(tmp_path)) for p in got]
        assert "Cargo.toml" in rels
        # components/ removed, only microservices/ now
        assert "entities/Cargo.toml" in rels
        assert "microservices/Cargo.toml" in rels
        assert len(got) == 3

    def test_excludes_build(self, tmp_path: Path) -> None:
        (tmp_path / "lib").mkdir()
        (tmp_path / "lib" / "Cargo.toml").write_text('[package]\nversion = "0.1.0"\n')
        (tmp_path / "build").mkdir()
        (tmp_path / "build" / "Cargo.toml").write_text("")
        got = _cargo_toml_paths(tmp_path)
        rels = [str(p.relative_to(tmp_path)) for p in got]
        assert rels == ["lib/Cargo.toml"]
        assert not any("build" in r for r in rels)


class TestRun:
    def test_success_updates_all_matching(self, tmp_path: Path) -> None:
        (tmp_path / "Cargo.toml").write_text(
            '[workspace]\n[workspace.package]\nversion = "0.1.0"\n'
        )
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[workspace]\n\n[workspace.package]\nversion = "0.1.0"\n'
        )
        (tmp_path / "entities").mkdir()
        (tmp_path / "entities" / "Cargo.toml").write_text(
            '[package]\nname = "e"\nversion = "0.1.0"\n'
        )

        rc = run(tmp_path, "patch")

        assert rc == 0
        assert 'version = "0.1.1"' in (tmp_path / "Cargo.toml").read_text()
        assert 'version = "0.1.1"' in (tmp_path / "microservices" / "Cargo.toml").read_text()
        assert 'version = "0.1.1"' in (tmp_path / "entities" / "Cargo.toml").read_text()

    def test_fails_without_microservices(self, tmp_path: Path) -> None:
        assert run(tmp_path, "patch") == 1

    def test_root_updated_when_drifted_from_microservices(self, tmp_path: Path) -> None:
        # Root has 0.1.0, microservices has 0.2.0: main loop won't replace root; _set_workspace_package_version does.
        (tmp_path / "Cargo.toml").write_text(
            '[workspace]\nmembers = ["microservices"]\n\n[workspace.package]\nversion = "0.1.0"\n'
        )
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[workspace]\n\n[workspace.package]\nversion = "0.2.0"\n'
        )
        (tmp_path / "entities").mkdir()
        (tmp_path / "entities" / "Cargo.toml").write_text(
            '[package]\nname = "e"\nversion = "0.2.0"\n'
        )

        rc = run(tmp_path, "patch")

        assert rc == 0
        assert 'version = "0.2.1"' in (tmp_path / "Cargo.toml").read_text()
        assert 'version = "0.2.1"' in (tmp_path / "microservices" / "Cargo.toml").read_text()

    def test_invalid_bump_raises_system_exit(self, tmp_path: Path) -> None:
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[workspace]\n[workspace.package]\nversion = "0.1.0"\n'
        )
        (tmp_path / "Cargo.toml").write_text(
            '[workspace]\n[workspace.package]\nversion = "0.1.0"\n'
        )
        with pytest.raises(SystemExit):
            run(tmp_path, "invalid")

    def test_appends_to_github_output_when_set(self, tmp_path: Path) -> None:
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[workspace]\n[workspace.package]\nversion = "0.1.0"\n'
        )
        (tmp_path / "Cargo.toml").write_text(
            '[workspace]\n[workspace.package]\nversion = "0.1.0"\n'
        )
        gh_out = tmp_path / "github_output.txt"
        with patch.dict("os.environ", {"GITHUB_OUTPUT": str(gh_out)}, clear=False):
            rc = run(tmp_path, "patch")
        assert rc == 0
        assert gh_out.is_file()
        assert "version=0.1.1\n" in gh_out.read_text()

    def test_run_bump_rc(self, tmp_path: Path) -> None:
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[workspace]\n[workspace.package]\nversion = "0.39.0-rc.2"\n'
        )
        (tmp_path / "Cargo.toml").write_text(
            '[workspace]\n[workspace.package]\nversion = "0.39.0-rc.2"\n'
        )
        (tmp_path / "entities").mkdir()
        (tmp_path / "entities" / "Cargo.toml").write_text(
            '[package]\nname = "e"\nversion = "0.39.0-rc.2"\n'
        )
        rc = run(tmp_path, "rc")
        assert rc == 0
        assert 'version = "0.39.0-rc.3"' in (tmp_path / "microservices" / "Cargo.toml").read_text()
        assert 'version = "0.39.0-rc.3"' in (tmp_path / "entities" / "Cargo.toml").read_text()

    def test_run_release_promotes_rc_to_full(self, tmp_path: Path) -> None:
        (tmp_path / "microservices").mkdir(exist_ok=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[workspace]\n[workspace.package]\nversion = "0.39.0-rc.2"\n'
        )
        (tmp_path / "Cargo.toml").write_text(
            '[workspace]\n[workspace.package]\nversion = "0.39.0-rc.2"\n'
        )
        (tmp_path / "entities").mkdir()
        (tmp_path / "entities" / "Cargo.toml").write_text(
            '[package]\nname = "e"\nversion = "0.39.0-rc.2"\n'
        )
        rc = run(tmp_path, "release")
        assert rc == 0
        assert 'version = "0.39.0"' in (tmp_path / "microservices" / "Cargo.toml").read_text()
        assert 'version = "0.39.0"' in (tmp_path / "entities" / "Cargo.toml").read_text()


class TestSetWorkspacePackageVersion:
    def test_changes_when_different(self, tmp_path: Path) -> None:
        p = tmp_path / "Cargo.toml"
        p.write_text('[workspace.package]\nversion = "0.1.0"\n')
        assert _set_workspace_package_version(p, "0.2.0") is True
        assert 'version = "0.2.0"' in p.read_text()

    def test_returns_false_when_already_equal(self, tmp_path: Path) -> None:
        p = tmp_path / "Cargo.toml"
        p.write_text('[workspace.package]\nversion = "0.2.0"\n')
        assert _set_workspace_package_version(p, "0.2.0") is False
        assert 'version = "0.2.0"' in p.read_text()

    def test_changes_prerelease_to_full(self, tmp_path: Path) -> None:
        p = tmp_path / "Cargo.toml"
        p.write_text('[workspace.package]\nversion = "0.39.0-rc.2"\n')
        assert _set_workspace_package_version(p, "0.39.0") is True
        assert 'version = "0.39.0"' in p.read_text()
