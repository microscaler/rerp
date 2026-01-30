"""Tests for rerp CLI (main, openapi, bff, ci, ports)."""

import sys
from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest


def _run_main(argv: list[str]) -> int:
    from rerp_tooling.cli.main import main

    with patch.object(sys, "argv", ["rerp", *argv]):
        try:
            main()
            return 0
        except SystemExit as e:
            return e.code if isinstance(e.code, int) else 1


# --- main: no args, unknown command ---


def test_main_no_args_prints_help_and_exits_1(capsys):
    code = _run_main([])
    out, _ = capsys.readouterr()
    assert code == 1
    assert "rerp" in out or "ports" in out or "openapi" in out


def test_main_unknown_command_exits_nonzero(tmp_path, monkeypatch):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["unknown-cmd"])
    assert code != 0


# --- openapi validate ---


def test_openapi_validate_empty_dir(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["openapi", "validate", "--openapi-dir", str(tmp_path)])
    out, _ = capsys.readouterr()
    assert code == 0
    assert "openapi" in out.lower() or "valid" in out.lower() or "nothing" in out.lower()


def test_openapi_validate_one_valid(tmp_path, monkeypatch, capsys):
    (tmp_path / "openapi.yaml").write_text(
        "openapi: 3.1.0\ninfo: {title: T, version: '1.0'}\npaths: {}"
    )
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["openapi", "validate", "--openapi-dir", str(tmp_path)])
    out, _ = capsys.readouterr()
    assert code == 0
    assert "1" in out or "valid" in out.lower()


def test_openapi_validate_one_invalid(tmp_path, monkeypatch, capsys):
    (tmp_path / "openapi.yaml").write_text("{ invalid: ")
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["openapi", "validate", "--openapi-dir", str(tmp_path)])
    out, _ = capsys.readouterr()
    assert code == 1
    assert "invalid" in out.lower() or "error" in out.lower()


def test_openapi_fix_operation_id_casing_no_changes(tmp_path, monkeypatch, capsys):
    (tmp_path / "openapi" / "svc").mkdir(parents=True)
    (tmp_path / "openapi" / "svc" / "openapi.yaml").write_text(
        "paths:\n  /x:\n    get:\n      operationId: list_items\n"
    )
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(
        ["openapi", "fix-operation-id-casing", "--openapi-dir", str(tmp_path / "openapi")]
    )
    out, _ = capsys.readouterr()
    assert code == 0
    assert "No operationId casing changes needed" in out


def test_openapi_fix_operation_id_casing_dry_run(tmp_path, monkeypatch, capsys):
    (tmp_path / "openapi" / "svc").mkdir(parents=True)
    spec = tmp_path / "openapi" / "svc" / "openapi.yaml"
    spec.write_text("paths:\n  /x:\n    get:\n      operationId: getStuff\n")
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(
        [
            "openapi",
            "fix-operation-id-casing",
            "--openapi-dir",
            str(tmp_path / "openapi"),
            "--dry-run",
        ]
    )
    out, _ = capsys.readouterr()
    assert code == 0
    assert "DRY-RUN" in out or "Updated" in out
    assert "getStuff" in spec.read_text()  # unchanged


def test_openapi_fix_operation_id_casing_writes(tmp_path, monkeypatch, capsys):
    (tmp_path / "openapi" / "svc").mkdir(parents=True)
    spec = tmp_path / "openapi" / "svc" / "openapi.yaml"
    spec.write_text("paths:\n  /x:\n    get:\n      operationId: getStuff\n")
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(
        ["openapi", "fix-operation-id-casing", "--openapi-dir", str(tmp_path / "openapi")]
    )
    out, _ = capsys.readouterr()
    assert code == 0
    assert "Updated" in out and "get_stuff" in spec.read_text()


# --- bff generate-system ---


def test_bff_generate_system_no_subs(tmp_path, monkeypatch, capsys):
    (tmp_path / "openapi" / "x").mkdir(parents=True)
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(
        ["bff", "generate-system", "--system", "x", "--openapi-dir", str(tmp_path / "openapi")]
    )
    out, _ = capsys.readouterr()
    assert code == 0
    assert "sub-services" in out or "No sub" in out or "⚠" in out


def test_bff_generate_system_with_subs(tmp_path, monkeypatch, capsys):
    (tmp_path / "openapi" / "s" / "v").mkdir(parents=True)
    (tmp_path / "openapi" / "s" / "v" / "openapi.yaml").write_text(
        "openapi: 3.1.0\ninfo: {title: T, version: '1.0'}\npaths: {}"
    )
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(
        ["bff", "generate-system", "--system", "s", "--openapi-dir", str(tmp_path / "openapi")]
    )
    out, _ = capsys.readouterr()
    assert code == 0
    assert "✅" in out or "Generated" in out
    assert (tmp_path / "openapi" / "s" / "openapi.yaml").exists()


# --- ci patch-brrtrouter ---


def test_ci_patch_brrtrouter_dry_run_no_matches(tmp_path, monkeypatch, capsys):
    (tmp_path / "Cargo.toml").write_text('[package]\nname = "x"\nversion = "0.1.0"\n')
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["ci", "patch-brrtrouter", "--dry-run"])
    out, _ = capsys.readouterr()
    assert code == 0
    # No BRRTRouter/lifeguard path deps: "No Cargo.toml...nothing to patch" or dry-run message
    assert "No Cargo.toml" in out or "nothing" in out.lower() or "Dry-run" in out


def test_ci_patch_brrtrouter_audit(tmp_path, monkeypatch, capsys):
    (tmp_path / "Cargo.toml").write_text('[package]\nname = "x"\n')
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["ci", "patch-brrtrouter", "--audit"])
    out, _ = capsys.readouterr()
    assert code == 0
    assert "Audit" in out or "Cargo.toml" in out


def test_ci_fix_cargo_paths_no_deps(tmp_path, monkeypatch, capsys):
    (tmp_path / "microservices" / "accounting" / "svc").mkdir(parents=True)
    cargo = tmp_path / "microservices" / "accounting" / "svc" / "Cargo.toml"
    cargo.write_text('[package]\nname = "svc"\nversion = "0.1.0"\n')
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["ci", "fix-cargo-paths", str(cargo)])
    out, _ = capsys.readouterr()
    assert code == 0
    assert "No changes" in out or "Fixed" in out


def test_docker_generate_dockerfile(tmp_path, monkeypatch, capsys):
    (tmp_path / "docker" / "microservices").mkdir(parents=True)
    (tmp_path / "docker" / "microservices" / "Dockerfile.template").write_text(
        "FROM x\nENV BIN={{binary_name}} PORT={{port}}\n"
    )
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["docker", "generate-dockerfile", "auth", "idam", "--port", "8000"])
    out, _ = capsys.readouterr()
    assert code == 0
    assert "Generated" in out
    gen = tmp_path / "docker" / "microservices" / "Dockerfile.auth_idam"
    assert gen.exists()
    assert "rerp_auth_idam_impl" in gen.read_text()


def test_docker_validate_build_artifacts_missing_exits_1(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["docker", "validate-build-artifacts"])
    out, err = capsys.readouterr()
    assert code == 1
    assert "Missing" in (out + err) or "build_artifacts" in (out + err)


def test_docker_validate_build_artifacts_success_exits_0(tmp_path, monkeypatch, capsys):
    from rerp_tooling.build.constants import get_binary_names

    # Minimal openapi layout so discovery returns names
    (tmp_path / "openapi" / "accounting" / "general-ledger").mkdir(parents=True)
    (tmp_path / "openapi" / "accounting" / "general-ledger" / "openapi.yaml").write_text(
        "openapi: 3.1.0\ninfo: {}\nservers:\n  - url: http://localhost:8001/api\n"
    )
    binary_names = get_binary_names(tmp_path)
    for arch in ("amd64", "arm64", "arm"):
        d = tmp_path / "build_artifacts" / arch
        d.mkdir(parents=True)
        for name in binary_names.values():
            (d / name).write_bytes(b"\x7fELF")
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["docker", "validate-build-artifacts"])
    out, _ = capsys.readouterr()
    assert code == 0
    assert "amd64" in out and "arm64" in out and "arm" in out


def test_docker_copy_artifacts_unknown_arch_exits_1(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["docker", "copy-artifacts", "x64"])
    out, err = capsys.readouterr()
    assert code == 1
    assert "Unknown" in (out + err) or "x64" in (out + err)


def test_docker_copy_artifacts_missing_binary_exits_1(tmp_path, monkeypatch, capsys):
    # Discovery expects one service; we do not create its binary so run fails
    (tmp_path / "openapi" / "accounting" / "general-ledger").mkdir(parents=True)
    (tmp_path / "openapi" / "accounting" / "general-ledger" / "openapi.yaml").write_text(
        "openapi: 3.1.0\ninfo: {}\nservers:\n  - url: http://localhost:8001/api\n"
    )
    (tmp_path / "microservices" / "target" / "x86_64-unknown-linux-musl" / "release").mkdir(
        parents=True
    )
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["docker", "copy-artifacts", "amd64"])
    out, err = capsys.readouterr()
    assert code == 1
    assert "not found" in (out + err) or "Binary" in (out + err)


def test_docker_build_base_dry_run(tmp_path, monkeypatch, capsys):
    (tmp_path / "docker" / "base").mkdir(parents=True)
    (tmp_path / "docker" / "base" / "Dockerfile").write_text("FROM alpine\n")
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["docker", "build-base", "--dry-run"])
    out, _ = capsys.readouterr()
    assert code == 0
    assert "dry-run" in out.lower() or "would" in out.lower()


def test_docker_copy_binary_missing_source_exits_1(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["docker", "copy-binary", "missing", "out/b", "b"])
    err = capsys.readouterr().err
    assert code == 1
    assert "not found" in err or "missing" in err


def test_docker_copy_binary_success_exits_0(tmp_path, monkeypatch, capsys):
    (tmp_path / "src").mkdir()
    (tmp_path / "src" / "bin").write_bytes(b"x")
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["docker", "copy-binary", "src/bin", "out/bin", "bin"])
    _out, _ = capsys.readouterr()
    assert code == 0
    assert (tmp_path / "out" / "bin").exists()
    assert (tmp_path / "out" / "bin.sha256").exists()


def test_docker_build_image_simple_missing_hash_exits_1(tmp_path, monkeypatch, capsys):
    (tmp_path / "art").write_bytes(b"x")
    (tmp_path / "Dockerfile").write_text("FROM alpine\n")
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(
        [
            "docker",
            "build-image-simple",
            "img",
            "Dockerfile",
            "missing.sha256",
            "art",
        ]
    )
    err = capsys.readouterr().err
    assert code == 1
    assert "Hash" in err or "hash" in err or "not found" in err


def test_docker_copy_multiarch_unknown_arch_exits_1(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["docker", "copy-multiarch", "auth", "idam", "x64"])
    err = capsys.readouterr().err
    assert code == 1
    assert "Unknown" in err or "x64" in err


def test_docker_copy_multiarch_success_exits_0(tmp_path, monkeypatch, capsys):
    (tmp_path / "microservices" / "target" / "x86_64-unknown-linux-musl" / "release").mkdir(
        parents=True
    )
    (
        tmp_path
        / "microservices"
        / "target"
        / "x86_64-unknown-linux-musl"
        / "release"
        / "rerp_auth_idam_impl"
    ).write_bytes(b"x")
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["docker", "copy-multiarch", "auth", "idam", "amd64"])
    assert code == 0
    assert (tmp_path / "build_artifacts" / "auth_idam" / "amd64" / "rerp_auth_idam_impl").exists()


def test_tilt_setup_mocked_exits_0(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    with (
        patch("shutil.which", return_value="/usr/bin/docker"),
        patch("subprocess.run", return_value=MagicMock(returncode=0)),
    ):
        code = _run_main(["tilt", "setup"])
    assert code == 0


def test_tilt_teardown_mocked_exits_0(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    with patch("subprocess.run", return_value=MagicMock(returncode=0)):
        code = _run_main(["tilt", "teardown"])
    assert code == 0


def test_tilt_setup_persistent_volumes_mocked_exits_0(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    with patch("subprocess.run", return_value=MagicMock(returncode=0)):
        code = _run_main(["tilt", "setup-persistent-volumes"])
    assert code == 0


def test_tilt_logs_tilt_missing_exits_1(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    with patch("shutil.which", return_value=None):
        code = _run_main(["tilt", "logs", "general-ledger"])
    assert code == 1


def test_tilt_setup_kind_registry_mocked_exits_0(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    with patch("subprocess.run") as m:
        m.side_effect = [
            MagicMock(returncode=0),
            MagicMock(returncode=0, stdout="true"),
            MagicMock(returncode=0),
            MagicMock(returncode=0, stdout="{}"),
            MagicMock(returncode=0),
            MagicMock(returncode=0),
        ]
        code = _run_main(["tilt", "setup-kind-registry"])
    assert code == 0


# --- build microservices / microservice ---


def test_build_microservice_unknown_exits_1(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["build", "microservice", "nosuch"])
    out, err = capsys.readouterr()
    assert code == 1
    assert "unknown" in (out + err) or "nosuch" in (out + err) or "Valid" in (out + err)


def test_build_microservices_missing_manifest_exits_1(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["build", "microservices", "amd64"])
    out, err = capsys.readouterr()
    assert code == 1
    assert "not found" in (out + err) or "Cargo.toml" in (out + err)


# --- ports ---


def test_ports_list_empty_registry(tmp_path, monkeypatch, capsys):
    (tmp_path / "port-registry.json").write_text(
        '{"version":"1.0","assignments":{},"next_port":8001,"reserved_ports":[8080],"metadata":{}}'
    )
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["ports", "list"])
    out, _ = capsys.readouterr()
    assert code == 0
    assert "No port" in out or "Total: 0" in out or "assignments" in out.lower()


def test_ports_query_missing(tmp_path, monkeypatch, capsys):
    (tmp_path / "port-registry.json").write_text(
        '{"version":"1.0","assignments":{},"next_port":8001,"reserved_ports":[8080],"metadata":{}}'
    )
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["ports", "query", "nosuch"])
    out, _ = capsys.readouterr()
    assert code == 1
    assert "no assigned port" in out or "nosuch" in out


def test_ports_assign_and_list(tmp_path, monkeypatch, capsys):
    (tmp_path / "port-registry.json").write_text(
        '{"version":"1.0","assignments":{},"next_port":8001,"reserved_ports":[8080],"metadata":{}}'
    )
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    code = _run_main(["ports", "assign", "svc1"])
    out, _ = capsys.readouterr()
    assert code == 0
    assert "Assigned" in out or "8001" in out

    code2 = _run_main(["ports", "list"])
    out2, _ = capsys.readouterr()
    assert code2 == 0
    assert "svc1" in out2 or "8001" in out2


# --- openapi run_openapi and _run_validate (direct) ---


def test_run_openapi_validate_missing_dir(tmp_path, monkeypatch):
    from rerp_tooling.cli.openapi import run_openapi

    class A:
        openapi_cmd = "validate"
        openapi_dir = tmp_path / "nonexistent"

    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    # run_openapi expects project_root; we use openapi_dir override so it doesn't matter
    rc = run_openapi(A(), tmp_path)
    assert rc == 0


# --- bff run_bff (direct) ---


def test_run_bff_generate_system_unknown_cmd():
    from rerp_tooling.cli.bff import run_bff

    class A:
        bff_cmd = "unknown"

    with pytest.raises(ValueError, match="bff"):
        run_bff(A(), Path("/tmp"))


# --- ci run_ci (direct) ---


def test_run_ci_patch_brrtrouter(tmp_path, monkeypatch):
    from rerp_tooling.cli.ci import run_ci

    (tmp_path / "Cargo.toml").write_text('[package]\nname = "x"\n')

    class A:
        ci_cmd = "patch-brrtrouter"
        dry_run = True
        audit = False

    with pytest.raises(SystemExit) as exc:
        run_ci(A(), tmp_path)
    assert exc.value.code == 0


# --- release (bump, generate-notes) ---


def test_release_generate_notes_requires_version(tmp_path, monkeypatch, capsys):
    from rerp_tooling.cli.release import run_release

    class A:
        release_cmd = "generate-notes"
        version = None
        since_tag = None
        template = None
        output = None
        model = None
        provider = None

    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    with pytest.raises(SystemExit) as exc:
        run_release(A(), tmp_path)
    assert exc.value.code == 1
    assert "version" in capsys.readouterr().err.lower()


def _fake_http_resp(payload: dict):
    import json

    class _Resp:
        def read(self) -> bytes:
            return json.dumps(payload).encode()

        def __enter__(self) -> "_Resp":
            return self

        def __exit__(self, *a: object) -> None:
            pass

    return _Resp()


OPENAI_BASIC = {"choices": [{"message": {"content": "OpenAI basic response for 1.0.0"}}]}
ANTHROPIC_BASIC = {"content": [{"type": "text", "text": "Anthropic basic response for 1.0.0"}]}


def test_release_generate_notes_provider_openai(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    monkeypatch.setenv("OPENAI_API_KEY", "sk-fake")
    out = tmp_path / "notes.md"
    with (
        patch("rerp_tooling.release.notes._get_commits_since", return_value=["feat: a"]),
        patch("urllib.request.urlopen", return_value=_fake_http_resp(OPENAI_BASIC)),
    ):
        code = _run_main(
            [
                "release",
                "generate-notes",
                "-v",
                "1.0.0",
                "--since-tag",
                "v0.9.0",
                "-o",
                str(out),
                "--provider",
                "openai",
            ]
        )
    assert code == 0
    assert "OpenAI basic response for 1.0.0" in out.read_text()


def test_release_generate_notes_provider_anthropic(tmp_path, monkeypatch, capsys):
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    monkeypatch.setenv("ANTHROPIC_API_KEY", "skant-fake")
    out = tmp_path / "notes.md"
    with (
        patch("rerp_tooling.release.notes._get_commits_since", return_value=["feat: a"]),
        patch("urllib.request.urlopen", return_value=_fake_http_resp(ANTHROPIC_BASIC)),
    ):
        code = _run_main(
            [
                "release",
                "generate-notes",
                "-v",
                "1.0.0",
                "--since-tag",
                "v0.9.0",
                "-o",
                str(out),
                "--provider",
                "anthropic",
            ]
        )
    assert code == 0
    assert "Anthropic basic response for 1.0.0" in out.read_text()


def test_release_generate_notes_provider_via_env_anthropic(tmp_path, monkeypatch, capsys):
    """RELEASE_NOTES_PROVIDER=anthropic without --provider uses Anthropic path."""
    monkeypatch.setenv("RERP_PROJECT_ROOT", str(tmp_path))
    monkeypatch.setenv("ANTHROPIC_API_KEY", "skant-fake")
    monkeypatch.setenv("RELEASE_NOTES_PROVIDER", "anthropic")
    out = tmp_path / "notes.md"
    with (
        patch("rerp_tooling.release.notes._get_commits_since", return_value=["feat: a"]),
        patch("urllib.request.urlopen", return_value=_fake_http_resp(ANTHROPIC_BASIC)),
    ):
        code = _run_main(
            [
                "release",
                "generate-notes",
                "-v",
                "1.0.0",
                "--since-tag",
                "v0.9.0",
                "-o",
                str(out),
            ]
        )
    assert code == 0
    assert "Anthropic basic response for 1.0.0" in out.read_text()
