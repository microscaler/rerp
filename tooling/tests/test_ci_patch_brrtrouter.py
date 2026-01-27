"""TDD: tests for rerp_tooling.ci.patch_brrtrouter (rerp ci patch-brrtrouter)."""

from pathlib import Path


class TestFindCargoTomls:
    def test_excludes_target_node_modules_git(self, tmp_path: Path):
        from rerp_tooling.ci.patch_brrtrouter import find_cargo_tomls

        (tmp_path / "a" / "Cargo.toml").parent.mkdir(parents=True, exist_ok=True)
        (tmp_path / "a" / "Cargo.toml").write_text("[package]")
        (tmp_path / "target" / "Cargo.toml").parent.mkdir(parents=True, exist_ok=True)
        (tmp_path / "target" / "Cargo.toml").write_text("[package]")
        (tmp_path / "node_modules" / "x" / "Cargo.toml").parent.mkdir(parents=True, exist_ok=True)
        (tmp_path / "node_modules" / "x" / "Cargo.toml").write_text("[package]")
        (tmp_path / ".git" / "x" / "Cargo.toml").parent.mkdir(parents=True, exist_ok=True)
        (tmp_path / ".git" / "x" / "Cargo.toml").write_text("[package]")
        found = find_cargo_tomls(tmp_path)
        assert len(found) == 1
        rel = found[0].relative_to(tmp_path)
        assert "a" in rel.parts
        assert "target" not in rel.parts
        assert "node_modules" not in rel.parts
        assert ".git" not in rel.parts

    def test_returns_sorted(self, tmp_path: Path):
        from rerp_tooling.ci.patch_brrtrouter import find_cargo_tomls

        (tmp_path / "z" / "Cargo.toml").parent.mkdir(parents=True, exist_ok=True)
        (tmp_path / "z" / "Cargo.toml").write_text("[package]")
        (tmp_path / "a" / "Cargo.toml").parent.mkdir(parents=True, exist_ok=True)
        (tmp_path / "a" / "Cargo.toml").write_text("[package]")
        found = find_cargo_tomls(tmp_path)
        assert found == sorted(found)


class TestFindMatches:
    def test_brrtrouter_path_dep_returns_replacement(self):
        from rerp_tooling.ci.patch_brrtrouter import find_matches

        text = '[dependencies]\nbrrtrouter = { path = "../../BRRTRouter" }'
        m = find_matches(text)
        assert len(m) == 1
        assert "path" in m[0][0] and "BRRTRouter" in m[0][0]
        assert "git" in m[0][1] and "microscaler/BRRTRouter" in m[0][1]
        assert "brrtrouter" in m[0][1]

    def test_brrtrouter_macros_path_dep_returns_replacement(self):
        from rerp_tooling.ci.patch_brrtrouter import find_matches

        text = 'brrtrouter_macros = { path = "../BRRTRouter/brrtrouter_macros" }'
        m = find_matches(text)
        assert len(m) == 1
        assert "brrtrouter_macros" in m[0][1]

    def test_lifeguard_path_dep_returns_replacement(self):
        from rerp_tooling.ci.patch_brrtrouter import find_matches

        text = 'lifeguard = { path = "../../lifeguard" }'
        m = find_matches(text)
        assert len(m) == 1
        assert "path" in m[0][0] and "lifeguard" in m[0][0]
        assert "git" in m[0][1] and "lifeguard" in m[0][1]

    def test_no_path_dep_returns_empty(self):
        from rerp_tooling.ci.patch_brrtrouter import find_matches

        text = '[dependencies]\nserde = "1.0"'
        assert find_matches(text) == []


class TestPatchFile:
    def test_dry_run_does_not_write(self, tmp_path: Path):
        from rerp_tooling.ci.patch_brrtrouter import patch_file

        f = tmp_path / "Cargo.toml"
        f.write_text('[dependencies]\nbrrtrouter = { path = "../../BRRTRouter" }')
        changed, matches = patch_file(f, dry_run=True, audit=False)
        assert changed is True
        assert len(matches) == 1
        assert "path" in f.read_text() and "BRRTRouter" in f.read_text()

    def test_audit_does_not_write(self, tmp_path: Path):
        from rerp_tooling.ci.patch_brrtrouter import patch_file

        f = tmp_path / "Cargo.toml"
        orig = '[dependencies]\nbrrtrouter = { path = "../../BRRTRouter" }'
        f.write_text(orig)
        changed, matches = patch_file(f, dry_run=False, audit=True)
        assert changed is True
        assert len(matches) == 1
        assert f.read_text() == orig

    def test_patch_replaces_path_with_git(self, tmp_path: Path):
        from rerp_tooling.ci.patch_brrtrouter import patch_file

        f = tmp_path / "Cargo.toml"
        f.write_text('[dependencies]\nbrrtrouter = { path = "../../BRRTRouter" }')
        changed, _ = patch_file(f, dry_run=False, audit=False)
        assert changed is True
        txt = f.read_text()
        assert "path" not in txt or "BRRTRouter" not in txt
        assert "git" in txt and "microscaler/BRRTRouter" in txt

    def test_patch_idempotent_second_run_no_matches(self, tmp_path: Path):
        from rerp_tooling.ci.patch_brrtrouter import patch_file

        f = tmp_path / "Cargo.toml"
        f.write_text('[dependencies]\nbrrtrouter = { path = "../../BRRTRouter" }')
        patch_file(f, dry_run=False, audit=False)
        first = f.read_text()
        changed2, matches2 = patch_file(f, dry_run=False, audit=False)
        assert changed2 is False
        assert matches2 == []
        assert f.read_text() == first


class TestRun:
    def test_run_no_matches(self, tmp_path: Path, capsys):
        from rerp_tooling.ci.patch_brrtrouter import run

        (tmp_path / "Cargo.toml").write_text('[package]\nname = "x"\n')
        run(tmp_path, dry_run=False, audit=False)
        out, _ = capsys.readouterr()
        assert "No Cargo.toml" in out or "nothing" in out.lower()

    def test_run_audit_with_matches(self, tmp_path: Path, capsys):
        from rerp_tooling.ci.patch_brrtrouter import run

        (tmp_path / "Cargo.toml").write_text(
            '[dependencies]\nbrrtrouter = { path = "../../BRRTRouter" }\n'
        )
        run(tmp_path, dry_run=False, audit=True)
        out, _ = capsys.readouterr()
        assert "Audit" in out and "Cargo.toml" in out

    def test_run_dry_run_with_matches(self, tmp_path: Path, capsys):
        from rerp_tooling.ci.patch_brrtrouter import run

        (tmp_path / "Cargo.toml").write_text(
            '[dependencies]\nbrrtrouter = { path = "../../BRRTRouter" }\n'
        )
        run(tmp_path, dry_run=True, audit=False)
        out, _ = capsys.readouterr()
        assert "Dry-run" in out or "would replace" in out
        assert (
            "path" in (tmp_path / "Cargo.toml").read_text()
            or "BRRTRouter" in (tmp_path / "Cargo.toml").read_text()
        )

    def test_run_patch_and_cargo_update_mocked(self, tmp_path: Path, capsys, monkeypatch):
        import subprocess

        from rerp_tooling.ci import patch_brrtrouter

        (tmp_path / "microservices").mkdir()
        (tmp_path / "microservices" / "Cargo.toml").write_text(
            '[package]\nname = "m"\n[dependencies]\nbrrtrouter = { path = "../../BRRTRouter" }\n'
        )
        called = []

        def fake_run(cmd, *args, **kwargs):
            called.append(cmd)
            from unittest.mock import MagicMock

            m = MagicMock()
            m.returncode = 0
            m.stderr = ""
            m.check_returncode = MagicMock()
            return m

        monkeypatch.setattr(subprocess, "run", fake_run)
        patch_brrtrouter.run(tmp_path, dry_run=False, audit=False)
        out, _ = capsys.readouterr()
        assert "Patched" in out or "cargo update" in out.lower()
        assert any("cargo" in str(c) for c in called) or "Ran cargo update" in out
