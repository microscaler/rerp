"""`rerp` CLI: thin wrapper around ``brrtrouter_tooling.cli`` with RERP-style names.

RERP uses the default suite-nested discovery (`openapi/<suite>/<service>/`)
from ``brrtrouter_tooling.cli``, NOT the hauliage-style flat-layout patches
in ``brrtrouter_tooling.workspace.cli``.

Command mapping (Tiltfile -> raw CLI):
    rerp gen suite <suite> --service <name>  -> brrtrouter gen suite <suite> --service <name>
    rerp gen stubs <suite> <name> --force    -> brrtrouter gen stubs <suite> <name> --force
    rerp build microservice <name>           -> brrtrouter build <suite>_<name> --package rerp_<suite>_<name>
    rerp docker copy-binary <src> <dest> <bn> -> brrtrouter docker copy-binary <src> <dest> <bn>
    rerp docker build-image-simple <img> <tmpl> <hash> <artifact> --service <name>
        -> brrtrouter docker build-image-simple <img> <hash> <artifact> --system <suite> --module <name>
    rerp docker build-base                   -> brrtrouter docker build-base
    rerp bff generate-system                 -> brrtrouter bff generate-system
"""

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


def _translate_build(argv):
    """Translate ``build microservice <name>`` to ``build <suite>_<name> --package rerp_<suite>_<name>``."""
    if not argv:
        return argv
    if argv[0] == "microservice":
        name = argv[1] if len(argv) > 1 else ""
        module = name.replace("-", "_")
        # For rerp, package names are rerp_{suite}_{module}
        # Default to 'accounting' suite if no other indicator
        suite = "accounting"  # TODO: make configurable or derive from context
        return ["build", f"{suite}_{module}", "--package", f"rerp_{suite}_{module}"] + argv[2:]
    return argv


def _translate_docker(argv: list[str]) -> list[str]:
    """Translate ``docker build-image-simple ... --service <name>`` to use --system/--module."""
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

            system = "accounting"
            module = None
            port = None
            binary_name = None
            no_cache = False
            i = 0
            while i < len(rest):
                if rest[i] == "--system" and i + 1 < len(rest):
                    system = rest[i + 1]
                    i += 2
                elif rest[i] == "--service" and i + 1 < len(rest):
                    module = rest[i + 1].replace("-", "_")
                    i += 2
                elif rest[i] == "--module" and i + 1 < len(rest):
                    module = rest[i + 1].replace("-", "_")
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

            new_argv = ["docker", "build-image-simple", image_name, hash_path, artifact_path]
            new_argv.append(f"--system={system}")
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
            # Override package name to use RERP convention: rerp_accounting_<module>
            # instead of the default <snake>_service_api
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

            # Patch the package_name callback to use RERP naming
            import brrtrouter_tooling.cli.gen_cmd as gen_mod

            def _rerp_package_name(suite_name: str, sn: str) -> str:
                """RERP-style package name: rerp_<suite>_<module>."""
                module = sn.replace("-", "_")
                return f"rerp_{suite_name}_{module}"

            # For suite gen, the tooling calls default_gen_package_name(sn) which
            # produces names like "general_ledger_service_api". We override to produce
            # "rerp_accounting_general_ledger" so that impl/main.rs imports match.
            original_default = gen_mod.default_gen_package_name
            gen_mod.default_gen_package_name = _rerp_package_name
            gen_mod._default_package_name = _rerp_package_name

            gen_cmd.run_gen_argv()

            # Restore original
            gen_mod.default_gen_package_name = original_default
            gen_mod._default_package_name = lambda s, n: original_default(n)

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
        sys.argv = ["brrtrouter"] + ["bff"] + rest
        if rest and rest[0] == "generate-system":
            bff.run_bff_generate_system_argv()
        elif rest and rest[0] == "generate":
            bff.run_bff_generate()
        else:
            print("rerp bff: unknown subcommand", file=sys.stderr)
            sys.exit(1)

    elif cmd == "pre-commit":
        _run_pre_commit(rest)

    else:
        print(f'rerp: unknown command "{cmd}"', file=sys.stderr)
        sys.exit(1)
