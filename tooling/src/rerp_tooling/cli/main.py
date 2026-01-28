"""`rerp` CLI entry point."""

import os
import sys
from pathlib import Path

from . import bff as bff_cli
from . import bootstrap as bootstrap_cli
from . import build as build_cli
from . import ci as ci_cli
from . import docker as docker_cli
from . import gen as gen_cli
from . import openapi as openapi_cli
from . import ports as ports_cli
from . import pre_commit as pre_commit_cli
from . import release as release_cli
from . import tilt as tilt_cli


def _get_project_root() -> Path:
    root = Path(os.environ.get("RERP_PROJECT_ROOT", ".")).resolve()
    if str(root) == ".":
        root = Path.cwd()
    return root


def _get_registry_path(project_root: Path) -> Path:
    if os.environ.get("RERP_PORT_REGISTRY"):
        return Path(os.environ["RERP_PORT_REGISTRY"]).resolve()
    return project_root / "port-registry.json"


def main() -> None:
    parser = __build_parser()
    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        sys.exit(1)

    project_root = _get_project_root()
    registry_path = _get_registry_path(project_root)

    if args.command == "ports":
        if not args.ports_cmd:
            print("rerp ports: missing subcommand")
            print(
                "  assign, release, list, query, update-configs, validate, reconcile, fix-duplicates"
            )
            print("  Use: rerp ports --help")
            sys.exit(1)
        exit_code = ports_cli.run_ports(args, project_root, registry_path)
        sys.exit(exit_code)

    if args.command == "openapi":
        if not getattr(args, "openapi_cmd", None):
            print("rerp openapi: missing subcommand")
            print("  validate, fix-operation-id-casing")
            print("  Use: rerp openapi --help")
            sys.exit(1)
        exit_code = openapi_cli.run_openapi(args, project_root)
        sys.exit(exit_code)

    if args.command == "ci":
        if not getattr(args, "ci_cmd", None):
            print("rerp ci: missing subcommand")
            print("  patch-brrtrouter, fix-cargo-paths, is-tag, get-latest-tag, validate-version")
            print("  Use: rerp ci --help")
            sys.exit(1)
        ci_cli.run_ci(args, project_root)

    if args.command == "bff":
        if not getattr(args, "bff_cmd", None):
            print("rerp bff: missing subcommand")
            print("  generate-system")
            print("  Use: rerp bff --help")
            sys.exit(1)
        bff_cli.run_bff(args, project_root)
        return

    if args.command == "gen":
        if not getattr(args, "gen_cmd", None):
            print("rerp gen: missing subcommand")
            print("  suite")
            print("  Use: rerp gen --help")
            sys.exit(1)
        gen_cli.run_gen(args, project_root)
        return

    if args.command == "docker":
        if not getattr(args, "docker_cmd", None):
            print("rerp docker: missing subcommand")
            print(
                "  generate-dockerfile, copy-artifacts, copy-binary, build-image-simple, copy-multiarch, build-multiarch, build-base"
            )
            print("  Use: rerp docker --help")
            sys.exit(1)
        docker_cli.run_docker(args, project_root)

    if args.command == "build":
        build_cli.run_build(args, project_root)

    if args.command == "bootstrap":
        if not getattr(args, "bootstrap_cmd", None):
            print("rerp bootstrap: missing subcommand")
            print("  microservice")
            print("  Use: rerp bootstrap --help")
            sys.exit(1)
        bootstrap_cli.run_bootstrap(args, project_root)
        return

    if args.command == "tilt":
        if not getattr(args, "tilt_cmd", None):
            print("rerp tilt: missing subcommand")
            print("  setup-kind-registry, setup-persistent-volumes, setup, teardown, logs")
            print("  Use: rerp tilt --help")
            sys.exit(1)
        tilt_cli.run_tilt(args, project_root)
        return

    if args.command == "release":
        if not getattr(args, "release_cmd", None):
            print("rerp release: missing subcommand")
            print("  bump, generate-notes")
            print("  Use: rerp release --help")
            sys.exit(1)
        release_cli.run_release(args, project_root)
        return

    if args.command == "pre-commit":
        if not getattr(args, "pre_commit_cmd", None):
            print("rerp pre-commit: missing subcommand")
            print("  microservices-fmt")
            print("  Use: rerp pre-commit --help")
            sys.exit(1)
        pre_commit_cli.run_pre_commit(args, project_root)
        return

    # Placeholder for future: brrtrouter
    print(
        f"rerp {args.command}: not yet implemented. Use 'rerp ports', 'rerp openapi', 'rerp ci', 'rerp bff', 'rerp docker', 'rerp build', 'rerp bootstrap', 'rerp gen', 'rerp release', or 'rerp tilt'."
    )
    sys.exit(1)


def __build_parser():
    import argparse

    p = argparse.ArgumentParser(
        prog="rerp",
        description="RERP development tooling: ports, build, docker, tilt, brrtrouter, bff, gen",
    )
    sub = p.add_subparsers(dest="command", help="Command")

    # --- ports ---
    pp = sub.add_parser(
        "ports",
        help="Port registry: assign, list, validate, update-configs, reconcile, fix-duplicates",
    )
    pp_sub = pp.add_subparsers(dest="ports_cmd")

    pa = pp_sub.add_parser("assign", help="Assign a port to a service")
    pa.add_argument("service_name", help="Service name (e.g. general-ledger)")
    pa.add_argument("--force", action="store_true", help="Force reassignment even if port exists")
    pa.add_argument(
        "--port",
        type=int,
        metavar="N",
        help="Use this port (must be free) instead of next available",
    )
    pa.add_argument(
        "--update-configs",
        action="store_true",
        help="Update config files after assignment",
    )

    pr = pp_sub.add_parser("release", help="Release a port assignment")
    pr.add_argument("service_name", help="Service name")

    pp_sub.add_parser("list", help="List all port assignments")

    pq = pp_sub.add_parser("query", help="Get port for a service")
    pq.add_argument("service_name", help="Service name")

    pu = pp_sub.add_parser("update-configs", help="Update config files with assigned port")
    pu.add_argument("service_name", help="Service name")

    pv = pp_sub.add_parser(
        "validate",
        help="Scan registry, helm, kind, Tiltfile, bff-suite-config; report conflicts",
    )
    pv.add_argument("--json", action="store_true", help="Output JSON")

    prc = pp_sub.add_parser(
        "reconcile", help="Add services from helm values to registry (using helm port)"
    )
    prc.add_argument(
        "--update-configs",
        action="store_true",
        help="Run update-configs for each added service",
    )

    pf = pp_sub.add_parser(
        "fix-duplicates",
        help="Resolve duplicate service.port in helm; prefer suite (BFF + bff-suite-config)",
    )
    pf.add_argument("--dry-run", action="store_true", help="Only print planned changes")

    # --- openapi ---
    po = sub.add_parser("openapi", help="OpenAPI: validate, generate, fix-operation-id-casing")
    po_sub = po.add_subparsers(dest="openapi_cmd")
    pov = po_sub.add_parser(
        "validate", help="Validate all openapi.yaml under openapi/ (or --openapi-dir)"
    )
    pov.add_argument(
        "--openapi-dir",
        type=Path,
        metavar="DIR",
        help="Override openapi/ directory (default: project root / openapi)",
    )

    pof = po_sub.add_parser(
        "fix-operation-id-casing",
        help="Convert operationId from camelCase to snake_case in openapi.yaml files",
    )
    pof.add_argument(
        "--openapi-dir",
        type=Path,
        metavar="DIR",
        help="Directory to search (default: project root / openapi)",
    )
    pof.add_argument("--dry-run", action="store_true", help="Only print changes, do not write")
    pof.add_argument("--verbose", "-v", action="store_true", help="Print each file and conversion")

    # --- ci ---
    pci = sub.add_parser(
        "ci",
        help="CI/build: patch-brrtrouter (git deps for CI), fix-cargo-paths (local path deps), is-tag (check if ref is tag), get-latest-tag (GitHub API), validate-version (prevent downgrades)",
    )
    pci_sub = pci.add_subparsers(dest="ci_cmd")
    pci_p = pci_sub.add_parser(
        "patch-brrtrouter",
        help="Replace path deps for BRRTRouter and lifeguard with git; run cargo update",
    )
    pci_p.add_argument(
        "--dry-run",
        action="store_true",
        help="Show changes only, do not write or run cargo",
    )
    pci_p.add_argument("--audit", action="store_true", help="List Cargo.toml and matches only")
    pci_f = pci_sub.add_parser(
        "fix-cargo-paths",
        help="Fix brrtrouter/brrtrouter_macros path deps in a Cargo.toml to ../BRRTRouter (local dev)",
    )
    pci_f.add_argument("cargo_toml", type=Path, metavar="PATH", help="Path to Cargo.toml")
    pci_sub.add_parser(
        "is-tag",
        help="Check if GITHUB_REF is a release tag (refs/tags/v*). Prints 'true' or 'false' to stdout.",
    )
    pci_sub.add_parser(
        "get-latest-tag",
        help="Get latest release tag from GitHub API. Prints version (without 'v' prefix) to stdout, or empty if no releases.",
    )
    pci_v = pci_sub.add_parser(
        "validate-version",
        help="Validate version to prevent downgrades. Requires --current and --latest, or GITHUB_REPOSITORY/GITHUB_TOKEN.",
    )
    pci_v.add_argument("--current", required=True, help="Current version to validate")
    pci_v.add_argument("--latest", help="Latest version from GitHub (or use GITHUB_API)")
    pci_v.add_argument(
        "--allow-same", action="store_true", help="Allow same version (for patch releases)"
    )

    # --- bff ---
    pb = sub.add_parser("bff", help="BFF OpenAPI: generate-system from sub-service specs")
    pb_sub = pb.add_subparsers(dest="bff_cmd")
    pbg = pb_sub.add_parser(
        "generate-system",
        help="Merge openapi/{system}/{service}/openapi.yaml into system BFF",
    )
    pbg.add_argument(
        "--system",
        metavar="NAME",
        help="System name (e.g. accounting); default: all systems with sub-services",
    )
    pbg.add_argument(
        "--output",
        "-o",
        type=Path,
        metavar="PATH",
        help="Output path (default: openapi/{system}/openapi.yaml)",
    )
    pbg.add_argument(
        "--openapi-dir",
        type=Path,
        metavar="DIR",
        help="Override openapi/ directory (default: project root / openapi)",
    )

    # --- docker ---
    pd = sub.add_parser(
        "docker",
        help="Docker: generate-dockerfile, copy-artifacts, copy-binary, build-image-simple, copy-multiarch, build-multiarch, build-base, unpack-build-bins, validate-build-artifacts",
    )
    pd_sub = pd.add_subparsers(dest="docker_cmd")
    pdub = pd_sub.add_parser(
        "unpack-build-bins",
        help="Extract rerp-binaries-*.zip into microservices/target (from Multi-Arch artifacts in tmp/buildBins)",
    )
    pdub.add_argument(
        "--input-dir",
        default="tmp/buildBins",
        metavar="DIR",
        help="Directory with rerp-binaries-amd64.zip, rerp-binaries-arm64.zip, rerp-binaries-arm7.zip (default: tmp/buildBins)",
    )
    pdg = pd_sub.add_parser(
        "generate-dockerfile",
        help="Generate docker/microservices/Dockerfile.{system}_{module} from template",
    )
    pdg.add_argument("system", help="System name (e.g. accounting, auth)")
    pdg.add_argument("module", help="Module name (e.g. general-ledger, idam)")
    pdg.add_argument(
        "--port",
        type=int,
        default=8000,
        metavar="N",
        help="Service port (default: 8000)",
    )
    pd_sub.add_parser(
        "validate-build-artifacts",
        help="Check build_artifacts/{amd64,arm64,arm} contain expected microservice binaries (after download from Multi-Arch)",
    )
    pdc = pd_sub.add_parser(
        "copy-artifacts",
        help="Copy microservice binaries from microservices/target to build_artifacts/{arch}",
    )
    pdc.add_argument("arch", help="Architecture: amd64, arm64, or arm7")
    pdb = pd_sub.add_parser(
        "build-base",
        help="Build docker/base/Dockerfile as rerp-base:latest (local; use base-images.yml to publish)",
    )
    pdb.add_argument(
        "--push",
        action="store_true",
        help="Also push to ghcr.io/$GHCR_OWNER/rerp-base:latest (requires login)",
    )
    pdb.add_argument("--dry-run", action="store_true", help="Print commands only, do not build")
    pdcb = pd_sub.add_parser(
        "copy-binary",
        help="Copy binary to dest and write SHA256 to dest.sha256 (replaces copy-microservice-binary-simple.sh)",
    )
    pdcb.add_argument(
        "source",
        help="Source binary path (e.g. microservices/target/.../general_ledger)",
    )
    pdcb.add_argument("dest", help="Destination path (e.g. build_artifacts/amd64/general_ledger)")
    pdcb.add_argument("binary_name", help="Binary name for hash file (e.g. general_ledger)")
    pdbi = pd_sub.add_parser(
        "build-image-simple",
        help="Build image:tilt and push or kind load (replaces build-microservice-docker-simple.sh)",
    )
    pdbi.add_argument(
        "image_name", help="Image name (e.g. localhost:5001/rerp-accounting-general-ledger)"
    )
    pdbi.add_argument("hash_path", help="Path to .sha256 file")
    pdbi.add_argument("artifact_path", help="Path to binary artifact")
    # Support both old (static Dockerfile) and new (template-based) approaches
    pdbi.add_argument("--dockerfile", help="Path to static Dockerfile (legacy mode)")
    pdbi.add_argument("--system", help="System name (e.g. accounting) - for template mode")
    pdbi.add_argument("--module", help="Module/service name (e.g. invoice) - for template mode")
    pdbi.add_argument("--port", type=int, help="Service port (e.g. 8002) - for template mode")
    pdbi.add_argument("--binary-name", help="Binary name (e.g. invoice) - for template mode")
    pdcm = pd_sub.add_parser(
        "copy-multiarch",
        help="Copy component binaries to build_artifacts/{system}_{module}/{arch} (replaces copy-multiarch-binary.sh)",
    )
    pdcm.add_argument("system", help="System name (e.g. auth)")
    pdcm.add_argument("module", help="Module name (e.g. idam)")
    pdcm.add_argument(
        "arch",
        nargs="?",
        default="all",
        help="amd64, arm64, arm7, or all (default: all)",
    )
    pdbm = pd_sub.add_parser(
        "build-multiarch",
        help="Build and push multi-arch Docker images for a component (replaces build-multiarch-docker.sh)",
    )
    pdbm.add_argument("system", help="System name (e.g. auth)")
    pdbm.add_argument("module", help="Module name (e.g. idam)")
    pdbm.add_argument("image_name", help="Image name (e.g. rerp/auth-idam)")
    pdbm.add_argument("--tag", default="latest", help="Image tag (default: latest)")
    pdbm.add_argument("--push", action="store_true", help="Push images to registry")

    # --- build (host-aware cargo/cross; microservices/ workspace) ---
    pbd = sub.add_parser(
        "build",
        help="Build: workspace, <system>_<module>, microservices, or microservice",
    )
    pbd.add_argument(
        "target",
        help="workspace, <system>_<module> (microservices), microservices, or microservice",
    )
    pbd.add_argument(
        "arch",
        nargs="?",
        default=None,
        help="arch (amd64|arm64|arm7|all) or service name for microservice",
    )
    pbd.add_argument(
        "--release",
        action="store_true",
        help="Release build (microservices/microservice; microservices always release)",
    )

    # --- gen ---
    pgen = sub.add_parser(
        "gen",
        help="Regenerate services from OpenAPI specs using BRRTRouter",
    )
    pgen_sub = pgen.add_subparsers(dest="gen_cmd")
    pgens = pgen_sub.add_parser(
        "suite",
        help="Regenerate all services in a suite (or a specific service with --service)",
    )
    pgens.add_argument("suite", help="Suite name (e.g. accounting, hr, sales)")
    pgens.add_argument(
        "--service",
        help="Regenerate only this specific service (optional)",
    )

    # --- bootstrap ---
    pbo = sub.add_parser(
        "bootstrap",
        help="Bootstrap a microservice from OpenAPI (BRRTRouter, Dockerfile, Cargo, Tiltfile)",
    )
    pbo_sub = pbo.add_subparsers(dest="bootstrap_cmd")
    pbom = pbo_sub.add_parser(
        "microservice",
        help="Bootstrap accounting microservice from openapi/accounting/<name>/openapi.yaml",
    )
    pbom.add_argument("service_name", help="Service name (e.g. general-ledger, invoice)")
    pbom.add_argument(
        "port",
        nargs="?",
        type=int,
        default=None,
        help="Port (default: from port registry)",
    )
    pbom.add_argument(
        "--add-dependencies-config",
        action="store_true",
        help="Create brrtrouter-dependencies.toml alongside OpenAPI spec",
    )

    # --- release ---
    prl = sub.add_parser(
        "release", help="Release: bump, generate-notes (Cargo.toml, GitHub Release)"
    )
    prl_sub = prl.add_subparsers(dest="release_cmd")
    prlb = prl_sub.add_parser(
        "bump",
        help="Bump version from microservices/Cargo.toml; update all [package]/[workspace.package].version in repo",
    )
    prlb.add_argument(
        "bump",
        nargs="?",
        default="patch",
        choices=["patch", "minor", "major", "rc", "release"],
        help="Bump: patch|minor|major; rc=bump -rc.N; release=promote 0.39.0-rc.2 -> 0.39.0 (default: patch)",
    )
    prlgn = prl_sub.add_parser(
        "generate-notes",
        help="Generate release notes from commits since last tag via OpenAI or Anthropic; write to --output for gh-release body",
    )
    prlgn.add_argument("--version", "-v", required=True, help="Release version (e.g. 1.2.3)")
    prlgn.add_argument("--output", "-o", help="Write notes to file (default: stdout)")
    prlgn.add_argument("--template", "-t", help="Custom template path (default: built-in)")
    prlgn.add_argument("--since-tag", help="Git ref for 'since' (default: previous tag)")
    _p = (os.environ.get("RELEASE_NOTES_PROVIDER") or "anthropic").strip().lower()
    _prov_default = _p if _p in ("openai", "anthropic") else "anthropic"
    prlgn.add_argument(
        "--provider",
        choices=["openai", "anthropic"],
        default=_prov_default,
        help="AI provider: openai or anthropic (default: RELEASE_NOTES_PROVIDER or anthropic)",
    )
    prlgn.add_argument(
        "--model",
        help="Model: OpenAI (default: gpt-4o-mini or OPENAI_MODEL) or Anthropic (default: claude-sonnet-4-5-20250929 or ANTHROPIC_MODEL)",
    )

    # --- tilt ---
    pt = sub.add_parser(
        "tilt",
        help="Tilt/Kind: setup-kind-registry, setup-persistent-volumes, setup, teardown, logs",
    )
    pt_sub = pt.add_subparsers(dest="tilt_cmd")
    pt_sub.add_parser(
        "setup-kind-registry",
        help="Create/start kind-registry and connect to kind network",
    )
    pt_sub.add_parser(
        "setup-persistent-volumes", help="Apply k8s data/monitoring persistent-volumes"
    )
    pt_sub.add_parser("setup", help="Create dirs and docker volumes; check docker/tilt")
    ptt = pt_sub.add_parser(
        "teardown",
        help="Tilt down, stop containers, optional --remove-images/--remove-volumes/--system-prune",
    )
    ptt.add_argument("--remove-images", action="store_true", help="Remove rerp-* images")
    ptt.add_argument(
        "--remove-volumes",
        action="store_true",
        help="Remove postgres/redis/prometheus/grafana volumes",
    )
    ptt.add_argument("--system-prune", action="store_true", help="Run docker system prune -f")
    ptl = pt_sub.add_parser("logs", help="Tail tilt logs for a component")
    ptl.add_argument("component", help="Component name (e.g. general-ledger)")

    # --- pre-commit ---
    ppc = sub.add_parser(
        "pre-commit",
        help="Pre-commit hooks: microservices-fmt (cargo fmt when microservices/ changes)",
    )
    ppc_sub = ppc.add_subparsers(dest="pre_commit_cmd")
    ppc_sub.add_parser(
        "microservices-fmt",
        help="If microservices/ changed vs HEAD, run cargo fmt in microservices and entities",
    )

    return p
