"""`rerp` CLI: suite-aware wrapper around BRRTRouter tooling.

RERP is intentionally **not** flat like Hauliage. It is a multi-suite product,
so the canonical layout is:

    openapi/{suite}/{service}/openapi.yaml
    microservices/{suite}/{service}/{gen,impl}

Future LLM sessions: do not "fix" RERP by flattening it to Hauliage's directory
shape. The part we copy from Hauliage is the stable naming model:

    impl crate: rerp_{suite}_{service}
    gen crate:  rerp_{suite}_{service}_gen

The wrapper derives suite and package information from the nested tree whenever
possible. That keeps Tilt/CI building implementation binaries instead of
accidentally targeting generated crates.

Command mapping (Tiltfile -> raw CLI):
    rerp gen suite <suite> --service <name>   -> brrtrouter gen suite <suite> --service <name>
    rerp gen stubs <suite> <name> --force     -> brrtrouter gen stubs <suite> <name> --force
    rerp build microservice <name> [--suite]  -> brrtrouter build <suite>_<name> --package <impl-package>
    rerp docker copy-binary <src> <dest> <bn> -> brrtrouter docker copy-binary <src> <dest> <bn>
    rerp docker build-image-simple ...        -> brrtrouter docker build-image-simple ... --system <suite> --module <name>
    rerp bff generate-system [--system]       -> writes openapi/{suite}/openapi_bff.yaml
"""

import os
import sys
from pathlib import Path

# Prepend raw brrtrouter_tooling so imports resolve (no hauliage workspace patches)
_brrtrouter_tooling = __import__("os").environ.get(
    "BRRTROUTER_TOOLING_ROOT",
    str(Path(__file__).resolve().parent.parent.parent.parent / "BRRTRouter" / "tooling" / "src"),
)
if _brrtrouter_tooling not in sys.path:
    sys.path.insert(0, _brrtrouter_tooling)

from brrtrouter_tooling.cli import gen_cmd, build, docker_cmd, bff  # noqa: E402,I001
from brrtrouter_tooling.bff.config import load_suite_config  # noqa: E402
from brrtrouter_tooling.bff.generate import generate_bff_spec  # noqa: E402


DEFAULT_SUITE = "accounting"
HTTP_METHODS = {"get", "post", "put", "patch", "delete", "options", "head", "trace"}


def _project_root(start=None):
    """Return the RERP project root from a nested cwd.

    The wrapper is called from Tilt, Just, CI, and sometimes an agent shell. Do
    not assume cwd is the repository root; walk upward until both RERP layout
    anchors are present.
    """
    current = Path(start or Path.cwd()).resolve()
    for candidate in (current, *current.parents):
        if (candidate / "openapi").is_dir() and (candidate / "microservices").is_dir():
            return candidate
    return current


def _snake(name):
    return name.replace("-", "_")


def _strip_option(argv, option_name):
    """Remove ``option_name VALUE`` from argv and return ``(value, remaining)``."""
    value = None
    remaining = []
    i = 0
    while i < len(argv):
        item = argv[i]
        if item == option_name and i + 1 < len(argv):
            value = argv[i + 1]
            i += 2
            continue
        if item.startswith(option_name + "="):
            value = item.split("=", 1)[1]
            i += 1
            continue
        remaining.append(item)
        i += 1
    return value, remaining


def _suites_with_bff(project_root=None):
    root = Path(project_root or _project_root())
    openapi = root / "openapi"
    if not openapi.is_dir():
        return []
    return sorted(
        path.name
        for path in openapi.iterdir()
        if path.is_dir() and (path / "bff-suite-config.yaml").exists()
    )


def _suite_for_service(service_name, project_root=None, explicit_suite=None):
    """Resolve a service to its suite from ``openapi/{suite}/{service}``.

    RERP can have the same service name in different suites in the future
    (especially BFF names like ``bff``). In that case callers should pass
    ``--suite`` or set ``RERP_SUITE``. We still default to accounting for today's
    single-suite checkout so existing Tilt commands remain valid.
    """
    if explicit_suite:
        return explicit_suite
    env_suite = os.environ.get("RERP_SUITE", "").strip()
    if env_suite:
        return env_suite

    root = Path(project_root or _project_root())
    openapi = root / "openapi"
    matches = []
    if openapi.is_dir():
        for suite_dir in sorted(path for path in openapi.iterdir() if path.is_dir()):
            if (suite_dir / service_name / "openapi.yaml").exists():
                matches.append(suite_dir.name)
            elif (suite_dir / "bff-suite-config.yaml").exists() and service_name == "bff":
                matches.append(suite_dir.name)
    if len(matches) == 1:
        return matches[0]
    if DEFAULT_SUITE in matches or (openapi / DEFAULT_SUITE).exists():
        return DEFAULT_SUITE
    return matches[0] if matches else DEFAULT_SUITE


def _package_name_from_manifest(cargo_toml):
    if not cargo_toml.exists():
        return None
    in_package = False
    for raw_line in cargo_toml.read_text().splitlines():
        line = raw_line.strip()
        if line == "[package]":
            in_package = True
            continue
        if line.startswith("[") and line != "[package]":
            in_package = False
        if in_package and line.startswith("name"):
            _, value = line.split("=", 1)
            return value.strip().strip('"')
    return None


def _rerp_impl_package_name(suite, service_name, project_root=None):
    """Cargo package name for the implementation crate.

    Prefer the actual ``impl/Cargo.toml`` name so the wrapper is compatible with
    the current mixed checkout while we converge on the Hauliage convention. The
    fallback is the intended stable convention: ``rerp_{suite}_{service}``.
    """
    root = Path(project_root or _project_root())
    manifest_name = _package_name_from_manifest(
        root / "microservices" / suite / service_name / "impl" / "Cargo.toml"
    )
    return manifest_name or f"rerp_{suite}_{_snake(service_name)}"


def _rerp_gen_package_name(suite, service_name):
    """Generated crate package name.

    This mirrors Hauliage's proven ``<impl>_gen`` shape while preserving RERP's
    suite prefix. Do not regress this to ``*_service_api`` or to the impl crate
    name; both cause imports/builds to drift.
    """
    return f"rerp_{suite}_{_snake(service_name)}_gen"


def _translate_build(argv, project_root=None):
    """Translate ``build microservice`` to the suite implementation package."""
    if not argv:
        return argv
    if argv[0] == "microservice":
        name = argv[1] if len(argv) > 1 else ""
        suite, rest = _strip_option(argv[2:], "--suite")
        suite = _suite_for_service(name, project_root=project_root, explicit_suite=suite)
        module = _snake(name)
        package = _rerp_impl_package_name(suite, name, project_root=project_root)
        return ["build", f"{suite}_{module}", "--package", package] + rest
    return argv


def _translate_docker(argv: list[str]) -> list[str]:
    """Translate ``docker build-image-simple ... --service <name>`` to suite/module flags."""
    if not argv:
        return argv
    # argv is ['docker', 'build-image-simple', ...] from caller
    if len(argv) < 2:
        return argv
    subcmd = argv[1] if argv[0] == "docker" else argv[0]
    if subcmd == "build-image-simple":
        # Skip 'docker' and 'build-image-simple' prefix
        rest = argv[2:] if argv[0] == "docker" else argv[1:]
        # Raw CLI: build-image-simple <image_name> <hash_path> <artifact_path>
        # RERP: build-image-simple <image_name> <template> <hash_path> <artifact_path>
        # The extra arg is the Dockerfile template — skip it
        if len(rest) >= 4:
            image_name = rest[0]
            # skip rest[1] (template)
            hash_path = rest[2]
            artifact_path = rest[3]
            rest = rest[4:]

            explicit_suite = None
            module = None
            service_name = None
            port = None
            binary_name = None
            no_cache = False
            i = 0
            while i < len(rest):
                if rest[i] == "--system" and i + 1 < len(rest):
                    explicit_suite = rest[i + 1]
                    i += 2
                elif rest[i] == "--suite" and i + 1 < len(rest):
                    explicit_suite = rest[i + 1]
                    i += 2
                elif rest[i] == "--service" and i + 1 < len(rest):
                    service_name = rest[i + 1]
                    module = _snake(service_name)
                    i += 2
                elif rest[i] == "--module" and i + 1 < len(rest):
                    module = _snake(rest[i + 1])
                    i += 2
                elif rest[i] == "--port" and i + 1 < len(rest):
                    port = rest[i + 1]
                    i += 2
                elif rest[i] == "--binary-name" and i + 1 < len(rest):
                    binary_name = rest[i + 1]
                    i += 2
                elif rest[i] == "--no-cache":
                    no_cache = True
                    i += 1
                elif rest[i] == "--prune-dangling":
                    i += 1
                elif rest[i] == "--dev-sync-only":
                    i += 1
                else:
                    i += 1

            suite = _suite_for_service(
                service_name or (module.replace("_", "-") if module else ""),
                explicit_suite=explicit_suite,
            )
            new_argv = ["docker", "build-image-simple", image_name, hash_path, artifact_path]
            new_argv.append(f"--system={suite}")
            if module:
                new_argv.append(f"--module={module}")
            if port:
                new_argv.append(f"--port={port}")
            if binary_name:
                new_argv.append(f"--binary-name={binary_name}")
            if no_cache:
                new_argv.append("--no-cache")
            return new_argv
    return argv


def _bff_generate_system_plans(argv, project_root=None):
    """Return ``(openapi_dir, suite, output_path)`` plans for ``bff generate-system``.

    Raw BRRTRouter can generate system BFFs, but its default output path is not
    RERP's suite-local BFF artifact. This planner keeps the contract explicit:
    every suite writes ``openapi/{suite}/openapi_bff.yaml``.
    """
    root = Path(project_root or _project_root())
    raw_dir, rest = _strip_option(argv, "--openapi-dir")
    explicit_suite, rest = _strip_option(rest, "--system")
    explicit_output, rest = _strip_option(rest, "--output")
    explicit_suite_alias, rest = _strip_option(rest, "--suite")
    suite = explicit_suite or explicit_suite_alias

    if rest:
        print(f"Error: Unknown argument(s): {' '.join(rest)}", file=sys.stderr)
        print(
            "Usage: rerp bff generate-system [--suite <name>|--system <name>] "
            "[--openapi-dir <path>] [--output <path>]",
            file=sys.stderr,
        )
        sys.exit(1)

    openapi_dir = Path(raw_dir).expanduser().resolve() if raw_dir else (root / "openapi").resolve()
    suites = [suite] if suite else _suites_with_bff(root)
    if not suites:
        suites = [DEFAULT_SUITE]

    plans = []
    for suite_name in suites:
        if explicit_output and len(suites) == 1:
            output = Path(explicit_output).expanduser().resolve()
        elif explicit_output:
            print(
                "Error: --output can only be used with a single --suite/--system",
                file=sys.stderr,
            )
            sys.exit(1)
        else:
            output = openapi_dir / suite_name / "openapi_bff.yaml"
        plans.append((openapi_dir, suite_name, output))
    return plans


def _operation_ids_from_spec(spec_path):
    """Return operationIds from an OpenAPI file.

    This is deliberately tiny and local to the wrapper. Its job is not full
    OpenAPI validation; it is a guardrail proving the generated BFF includes the
    operations from every suite-configured service spec.
    """
    import yaml

    with Path(spec_path).open() as f:
        spec = yaml.safe_load(f) or {}

    operation_ids = set()
    for path_item in (spec.get("paths") or {}).values():
        if not isinstance(path_item, dict):
            continue
        for method, operation in path_item.items():
            if method not in HTTP_METHODS or not isinstance(operation, dict):
                continue
            operation_id = operation.get("operationId")
            if operation_id:
                operation_ids.add(operation_id)
    return operation_ids


def _validate_bff_operation_coverage(suite_config_path, bff_output_path, project_root=None):
    """Fail if generated BFF omits operationIds from configured service specs."""
    root = Path(project_root or _project_root())
    config = load_suite_config(Path(suite_config_path), base_dir=root)
    services = (config.get("_resolved") or {}).get("services") or {}

    expected = set()
    for service in services.values():
        expected |= _operation_ids_from_spec(service["spec_path"])

    actual = _operation_ids_from_spec(bff_output_path)
    missing = sorted(expected - actual)
    if missing:
        raise RuntimeError(
            "Generated BFF is missing operationIds from configured service specs: "
            + ", ".join(missing)
        )


def _run_bff_generate_system(argv, project_root=None):
    """Generate suite BFFs from bff-suite-config.yaml and verify coverage."""
    root = Path(project_root or _project_root())
    for openapi_dir, suite, output in _bff_generate_system_plans(argv, project_root=root):
        suite_config = openapi_dir / suite / "bff-suite-config.yaml"
        if not suite_config.exists():
            print(f"Error: Suite config not found: {suite_config}", file=sys.stderr)
            sys.exit(1)

        print(f"🔄 Generating RERP {suite} BFF spec from {suite_config} -> {output}")
        generate_bff_spec(suite_config_path=suite_config, output_path=output, base_dir=root)
        _validate_bff_operation_coverage(suite_config, output, project_root=root)


def _translate_bff_generate(argv, project_root=None):
    """Translate ``rerp bff generate --suite`` to raw suite-config flags."""
    root = Path(project_root or _project_root())
    suite, rest = _strip_option(argv, "--suite")
    if not suite:
        return argv

    has_suite_config = any(
        item == "--suite-config" or item.startswith("--suite-config=") for item in rest
    )
    has_output = any(item == "--output" or item.startswith("--output=") for item in rest)
    has_base_dir = any(item == "--base-dir" or item.startswith("--base-dir=") for item in rest)

    translated = []
    if not has_suite_config:
        translated += ["--suite-config", str(root / "openapi" / suite / "bff-suite-config.yaml")]
    translated += rest
    if not has_output:
        translated += ["--output", str(root / "openapi" / suite / "openapi_bff.yaml")]
    if not has_base_dir:
        translated += ["--base-dir", str(root)]
    return translated


def _run_pre_commit(argv):
    """Handle pre-commit hooks: microservices-fmt."""
    sub = argv[0] if argv else ""
    if sub == "microservices-fmt":
        # Run cargo fmt in microservices/ and rustfmt in entities/
        import subprocess as sp

        # microservices/ — use cargo fmt which handles all workspace crates
        ms = Path(__file__).resolve().parent.parent.parent.parent / "microservices"
        if ms.exists():
            result = sp.run(["cargo", "fmt", "--all"], cwd=str(ms), capture_output=True, text=True)
            if result.returncode != 0:
                print(f"cargo fmt in microservices/ failed:\n{result.stderr}", file=sys.stderr)
                sys.exit(1)

        # entities/ — use rustfmt directly (cargo fmt fails with "multiple workspace roots")
        ent = Path(__file__).resolve().parent.parent.parent.parent / "entities"
        if ent.exists():
            for rs_file in ent.rglob("*.rs"):
                sp.run(["rustfmt", str(rs_file)], capture_output=True)
        sys.exit(0)
    else:
        print(f"rerp pre-commit: unknown subcommand '{sub}'")
        print("  Available: microservices-fmt", file=sys.stderr)
        sys.exit(1)


def main():
    if len(sys.argv) < 2:
        print("rerp: missing subcommand", file=sys.stderr)
        print("Available: gen, build, docker, bff, pre-commit", file=sys.stderr)
        sys.exit(1)

    cmd = sys.argv[1]
    rest = sys.argv[2:]

    if cmd == "gen":
        if rest and rest[0] == "suite":
            # Override BRRTRouter's raw default package names.
            #
            # RERP is suite-nested, but should follow Hauliage's proven naming
            # split: impl package plus a separate `<impl>_gen` generated crate.
            # This is the guardrail that prevents future regenerations from
            # turning gen crates back into `<module>_service_api` or, worse,
            # giving gen crates the same name as implementation binaries.
            suite = rest[1] if len(rest) > 1 else ""
            service = None
            extra = []
            i = 2
            while i < len(rest):
                if rest[i] == "--service" and i + 1 < len(rest):
                    service = rest[i + 1]
                    i += 2
                else:
                    extra.append(rest[i])
                    i += 1

            # Rebuild args with suite name, passing --service if provided
            # run_gen_argv() reads sys.argv[2] as the subcommand, so we need:
            # sys.argv[0] = 'brrtrouter', sys.argv[1] = 'gen', sys.argv[2] = 'suite'
            new_rest = ["brrtrouter", "gen", "suite", suite]
            if service:
                new_rest += ["--service", service]
            new_rest += extra
            sys.argv = new_rest

            # Patch the package-name callbacks used by raw `brrtrouter gen suite`.
            # Keep this local to the call so other BRRTRouter tooling remains
            # untouched when the process continues.
            import brrtrouter_tooling.cli.gen_cmd as gen_mod

            def _one_arg_gen_package_name(sn: str) -> str:
                return _rerp_gen_package_name(suite, sn)

            original_default = gen_mod.default_gen_package_name
            original_default_package = gen_mod._default_package_name
            gen_mod.default_gen_package_name = _one_arg_gen_package_name
            gen_mod._default_package_name = _rerp_gen_package_name

            try:
                gen_cmd.run_gen_argv()
            finally:
                gen_mod.default_gen_package_name = original_default
                gen_mod._default_package_name = original_default_package

        elif rest and rest[0] in ("stubs", "generate", "generate-stubs"):
            # For stubs, we also need to pass the correct component-name
            suite = rest[1] if len(rest) > 1 else ""
            service = rest[2] if len(rest) > 2 else None
            force = "--force" in rest
            sync = "--sync" in rest

            # Rebuild args
            new_rest = ["stubs", suite]
            if service:
                new_rest.append(service)
            if force:
                new_rest.append("--force")
            if sync:
                new_rest.append("--sync")
            sys.argv = ["brrtrouter"] + new_rest
            gen_cmd.run_gen_argv()
        else:
            print("rerp gen: unknown subcommand", file=sys.stderr)
            sys.exit(1)

    elif cmd == "build":
        if rest and rest[0] == "microservice":
            sys.argv = ["brrtrouter"] + _translate_build(rest)
            build.run_build_argv()
        else:
            sys.argv = ["brrtrouter"] + ["build"] + rest
            build.run_build_argv()

    elif cmd == "docker":
        if rest:
            sys.argv = ["brrtrouter"] + _translate_docker(["docker"] + rest)
            docker_cmd.run_docker_argv()
        else:
            print("rerp docker: missing subcommand", file=sys.stderr)
            sys.exit(1)

    elif cmd == "bff":
        if rest and rest[0] == "generate-system":
            _run_bff_generate_system(rest[1:])
        elif rest and rest[0] == "generate":
            sys.argv = ["brrtrouter", "bff", "generate"] + _translate_bff_generate(rest[1:])
            bff.run_bff_generate()
        else:
            print("rerp bff: unknown subcommand", file=sys.stderr)
            sys.exit(1)

    elif cmd == "pre-commit":
        _run_pre_commit(rest)

    else:
        print(f'rerp: unknown command "{cmd}"', file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
