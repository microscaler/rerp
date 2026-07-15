"""Suite-aware runtime discovery, builds, and staged Docker images for RERP."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import shutil
import subprocess
import tempfile
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover - Python 3.10 compatibility
    import tomli as tomllib  # type: ignore[no-redef]


def _relative(path: Path, root: Path) -> str:
    return path.relative_to(root).as_posix()


def _helm_values_path(root: Path, suite: str, service: str) -> Path | None:
    values_dir = root / "helm" / "rerp-microservice" / "values"
    suite_path = values_dir / f"{suite}-{service}.yaml"
    legacy_accounting_path = values_dir / f"{service}.yaml"
    if suite_path.exists():
        return suite_path
    if suite == "accounting" and legacy_accounting_path.exists():
        return legacy_accounting_path
    return None


def describe_service(root: Path, suite: str, service: str) -> dict[str, str]:
    """Describe one service from its checked-in contract and Cargo manifest."""
    root = root.resolve()
    service_root = root / "microservices" / suite / service
    impl_manifest = service_root / "impl" / "Cargo.toml"
    gen_manifest = service_root / "gen" / "Cargo.toml"
    spec = root / "openapi" / suite / service / "openapi.yaml"
    if service == "bff" and not spec.exists():
        spec = root / "openapi" / suite / "openapi_bff.yaml"

    missing = [path for path in (impl_manifest, gen_manifest, spec) if not path.exists()]
    if missing:
        rendered = ", ".join(_relative(path, root) for path in missing)
        raise FileNotFoundError(f"{suite}/{service} is not runtime-ready; missing: {rendered}")

    manifest = tomllib.loads(impl_manifest.read_text())
    package_name = manifest.get("package", {}).get("name")
    binaries = manifest.get("bin") or []
    binary_name = binaries[0].get("name") if binaries else package_name
    if not package_name or not binary_name:
        raise ValueError(f"cannot determine package/binary from {impl_manifest}")

    config_dir = service_root / "impl" / "config"
    if not config_dir.is_dir():
        config_dir = service_root / "gen" / "config"

    helm_values = _helm_values_path(root, suite, service)
    resource_name = service if suite == "accounting" else f"{suite}-{service}"
    result = {
        "suite": suite,
        "service": service,
        "resource_name": resource_name,
        "package_name": str(package_name),
        "binary_name": str(binary_name),
        "image_name": f"rerp-{suite}-{service}",
        "spec_path": _relative(spec, root),
        "service_root": _relative(service_root, root),
        "gen_dir": _relative(service_root / "gen", root),
        "impl_dir": _relative(service_root / "impl", root),
        "config_dir": _relative(config_dir, root),
        "doc_dir": _relative(service_root / "gen" / "doc", root),
        "static_dir": _relative(service_root / "gen" / "static_site", root),
    }
    if helm_values is not None:
        result["helm_values"] = _relative(helm_values, root)
    return result


def discover_services(root: Path, require_helm: bool = True) -> list[dict[str, str]]:
    """Discover suite-nested runtime services without a duplicated service list."""
    root = root.resolve()
    discovered: list[dict[str, str]] = []
    openapi_root = root / "openapi"
    if not openapi_root.is_dir():
        return discovered

    for suite_dir in sorted(path for path in openapi_root.iterdir() if path.is_dir()):
        suite = suite_dir.name
        for service_dir in sorted(path for path in suite_dir.iterdir() if path.is_dir()):
            if not (service_dir / "openapi.yaml").exists():
                continue
            service = service_dir.name
            try:
                descriptor = describe_service(root, suite, service)
            except FileNotFoundError:
                continue
            if require_helm and "helm_values" not in descriptor:
                continue
            discovered.append(descriptor)

        # Suite BFF contracts are generated beside the suite config rather than
        # nested below openapi/<suite>/bff/. Hide that layout exception here so
        # build and deployment consumers receive one descriptor shape.
        if (suite_dir / "openapi_bff.yaml").exists():
            try:
                descriptor = describe_service(root, suite, "bff")
            except FileNotFoundError:
                continue
            if not require_helm or "helm_values" in descriptor:
                discovered.append(descriptor)

    return discovered


def build_microservice(root: Path, suite: str, service: str, release: bool = False) -> int:
    """Build the implementation package for the host architecture."""
    descriptor = describe_service(root, suite, service)
    from brrtrouter_tooling.build.host_aware import detect_host_architecture
    from brrtrouter_tooling.build.workspace_build import build_package_with_options

    return build_package_with_options(
        root,
        workspace_dir="microservices",
        package_name=descriptor["package_name"],
        arch=detect_host_architecture(),
        release=release,
    )


def _resolve(root: Path, value: str | Path) -> Path:
    path = Path(value)
    return path if path.is_absolute() else root / path


def _copy_tree_or_empty(source: Path, destination: Path) -> None:
    if source.is_dir():
        shutil.copytree(source, destination)
    else:
        destination.mkdir(parents=True)


def _docker_architecture() -> str:
    machine = platform.machine().lower()
    if machine in {"x86_64", "amd64"}:
        return "amd64"
    if machine in {"aarch64", "arm64"}:
        return "arm64"
    if machine.startswith("arm"):
        return "arm"
    raise ValueError(f"unsupported Docker host architecture: {machine}")


def _verified_digest(artifact: Path, digest_file: Path) -> str:
    expected_digest = digest_file.read_text().strip().split()[0]
    actual_digest = hashlib.sha256(artifact.read_bytes()).hexdigest()
    if expected_digest != actual_digest:
        raise ValueError(
            f"artifact digest mismatch for {artifact}: expected {expected_digest}, got {actual_digest}"
        )
    return actual_digest


def _verified_copy(artifact: Path, digest_file: Path, destination: Path) -> str:
    actual_digest = _verified_digest(artifact, digest_file)
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(artifact, destination)
    return actual_digest


def _image_tag(image_name: str) -> str:
    final_component = image_name.rsplit("/", 1)[-1]
    return image_name if ":" in final_component else f"{image_name}:tilt"


def build_base_image(root: Path, *, dry_run: bool = False) -> int:
    """Build RERP's local runtime base under its single canonical name."""
    root = root.resolve()
    dockerfile = root / "docker" / "base" / "Dockerfile"
    if not dockerfile.exists():
        print(f"missing base Dockerfile: {dockerfile}", file=os.sys.stderr)
        return 1
    command = [
        "docker",
        "build",
        "--tag",
        "rerp-base:latest",
        "--file",
        str(dockerfile),
        str(root),
    ]
    if dry_run:
        print(" ".join(command))
        return 0
    return subprocess.run(command, cwd=root, check=False).returncode


def stage_multiarch_context(
    root: Path,
    destination: str | Path,
    artifacts_root: str | Path,
    suite: str,
    service: str,
) -> int:
    """Stage one service's verified multi-architecture release context."""
    root = root.resolve()
    descriptor = describe_service(root, suite, service)
    destination_path = _resolve(root, destination).resolve()
    if destination_path == root or root not in destination_path.parents:
        print("image context destination must be below the repository root", file=os.sys.stderr)
        return 1
    artifacts = _resolve(root, artifacts_root)
    binary = descriptor["binary_name"]
    inputs = {
        architecture: artifacts / architecture / binary
        for architecture in ("amd64", "arm64", "arm")
    }
    missing = [path for path in inputs.values() if not path.exists() or not Path(f"{path}.sha256").exists()]
    if missing:
        print("missing multi-architecture image input: " + ", ".join(map(str, missing)), file=os.sys.stderr)
        return 1

    if destination_path.exists():
        shutil.rmtree(destination_path)
    destination_path.mkdir(parents=True)
    digests = {}
    try:
        for architecture, artifact in inputs.items():
            digests[architecture] = _verified_copy(
                artifact,
                Path(f"{artifact}.sha256"),
                destination_path / architecture / "service",
            )
    except ValueError as error:
        print(str(error), file=os.sys.stderr)
        shutil.rmtree(destination_path)
        return 1

    _copy_tree_or_empty(_resolve(root, descriptor["config_dir"]), destination_path / "config")
    _copy_tree_or_empty(_resolve(root, descriptor["doc_dir"]), destination_path / "doc")
    _copy_tree_or_empty(_resolve(root, descriptor["static_dir"]), destination_path / "static_site")
    shutil.copy2(root / "docker" / "microservices" / "Dockerfile", destination_path / "Dockerfile")
    (destination_path / "artifact-sha256.json").write_text(json.dumps(digests, sort_keys=True) + "\n")
    return 0


def build_service_image(
    root: Path,
    image_name: str,
    dockerfile: str | Path,
    hash_path: str | Path,
    artifact_path: str | Path,
    suite: str,
    service: str,
    *,
    no_cache: bool = False,
) -> int:
    """Build one runtime image from a narrow, service-specific temporary context."""
    root = root.resolve()
    descriptor = describe_service(root, suite, service)
    dockerfile_path = _resolve(root, dockerfile)
    artifact = _resolve(root, artifact_path)
    digest_file = _resolve(root, hash_path)

    for path in (dockerfile_path, artifact, digest_file):
        if not path.exists():
            print(f"missing image input: {path}", file=os.sys.stderr)
            return 1

    try:
        actual_digest = _verified_digest(artifact, digest_file)
    except ValueError as error:
        print(str(error), file=os.sys.stderr)
        return 1

    with tempfile.TemporaryDirectory(prefix="rerp-image-") as temporary:
        context = Path(temporary)
        _verified_copy(artifact, digest_file, context / _docker_architecture() / "service")
        _copy_tree_or_empty(_resolve(root, descriptor["config_dir"]), context / "config")
        _copy_tree_or_empty(_resolve(root, descriptor["doc_dir"]), context / "doc")
        _copy_tree_or_empty(_resolve(root, descriptor["static_dir"]), context / "static_site")

        command = [
            "docker",
            "build",
            "--tag",
            _image_tag(image_name),
            "--file",
            str(dockerfile_path),
            "--build-arg",
            "BASE_IMAGE=rerp-base:latest",
            "--build-arg",
            f"RERP_SUITE={suite}",
            "--build-arg",
            f"RERP_SERVICE={service}",
            "--build-arg",
            f"RERP_ARTIFACT_SHA256={actual_digest}",
        ]
        if no_cache:
            command.append("--no-cache")
        command.append(str(context))
        return subprocess.run(command, cwd=root, check=False).returncode


def _main() -> int:
    parser = argparse.ArgumentParser(description="RERP runtime descriptor helper")
    parser.add_argument("command", choices=["descriptors", "describe"])
    parser.add_argument("--root", type=Path, default=Path.cwd())
    parser.add_argument("--all", action="store_true", help="include services without Helm values")
    parser.add_argument("--suite")
    parser.add_argument("--service")
    args = parser.parse_args()

    if args.command == "descriptors":
        print(json.dumps(discover_services(args.root, require_helm=not args.all), sort_keys=True))
        return 0
    if not args.suite or not args.service:
        parser.error("describe requires --suite and --service")
    print(json.dumps(describe_service(args.root, args.suite, args.service), sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(_main())
