"""Host-aware build for RERP: cargo/cross, zigbuild, multi-arch (amd64, arm64, arm7)."""

from __future__ import annotations

import json
import os
import platform
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Optional

ARCH_TARGETS: Dict[str, str] = {
    "amd64": "x86_64-unknown-linux-musl",
    "arm64": "aarch64-unknown-linux-musl",
    "arm7": "armv7-unknown-linux-musleabihf",
}


def detect_host_architecture() -> str:
    machine = platform.machine().lower()
    if machine in ("x86_64", "amd64"):
        return "amd64"
    if machine in ("arm64", "aarch64"):
        return "arm64"
    return "amd64"


def should_use_zigbuild() -> bool:
    os_name = platform.system()
    arch = platform.machine()
    return os_name == "Darwin" or (os_name == "Linux" and arch != "x86_64")


def should_use_cross() -> bool:
    return os.environ.get("RERP_USE_CROSS") == "1"


def _install_rust_target(rust_target: str) -> bool:
    try:
        r = subprocess.run(
            ["rustup", "target", "list", "--installed"],
            capture_output=True,
            text=True,
            check=True,
        )
        if rust_target in r.stdout:
            return True
        print(f"📦 Installing Rust target: {rust_target}")
        subprocess.run(["rustup", "target", "add", rust_target], check=True)
        return True
    except (subprocess.CalledProcessError, FileNotFoundError) as e:
        print(f"❌ Error: {e}", file=sys.stderr)
        return False


def _get_cargo_env(rust_target: str) -> Dict[str, str]:
    env = os.environ.copy()
    if rust_target == "x86_64-unknown-linux-musl":
        env["CC_x86_64_unknown_linux_musl"] = "musl-gcc"
        env["CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER"] = "musl-gcc"
    elif rust_target == "aarch64-unknown-linux-musl":
        env["CC_aarch64_unknown_linux_musl"] = "aarch64-linux-musl-gcc"
        env["CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER"] = "aarch64-linux-musl-gcc"
    elif rust_target == "armv7-unknown-linux-musleabihf":
        env["CC_armv7_unknown_linux_musleabihf"] = "arm-linux-musleabihf-gcc"
        env["CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER"] = "arm-linux-musleabihf-gcc"
    return env


# RERP workspace: microservices/ (root Cargo.toml has members = ["microservices"])
WORKSPACE_DIR = "microservices"

# armv7 cross toolchain does not provide __ffsdi2 (used by tikv-jemalloc-sys); disable jemalloc for arm7
ARM7_TARGET = "armv7-unknown-linux-musleabihf"

# Opt-in jemalloc (tikv) for amd64/arm64; not used for arm7 (musl __ffsdi2 link error)
JEMALLOC_FEATURE_ARGS: List[str] = ["--features", "jemalloc"]


def _workspace_packages(manifest: Path, project_root: Path) -> List[str]:
    """Return workspace member package names (for -p ...). Uses cargo metadata.
    workspace_members are package IDs like 'path+file:///.../gen#rerp_accounting_foo_gen@0.1.0'.
    """
    try:
        r = subprocess.run(
            ["cargo", "metadata", "--manifest-path", str(manifest), "--format-version", "1", "--no-deps"],
            capture_output=True,
            text=True,
            check=True,
            cwd=str(project_root),
        )
        data = json.loads(r.stdout)
        raw = data.get("workspace_members", [])
        out: List[str] = []
        for pid in raw:
            # pid is like "path+file:///.../gen#rerp_accounting_foo_gen@0.1.0" -> extract "rerp_accounting_foo_gen"
            if "#" in pid and "@" in pid:
                out.append(pid.split("#")[1].split("@")[0])
            else:
                out.append(pid)
        return out
    except (subprocess.CalledProcessError, json.JSONDecodeError, FileNotFoundError):
        return []


def _build_workspace(
    project_root: Path,
    rust_target: str,
    arch_name: str,
    use_zigbuild: bool,
    use_cross: bool,
    extra_args: List[str],
    release: bool = True,
) -> bool:
    workspace_dir = project_root / WORKSPACE_DIR
    manifest = workspace_dir / "Cargo.toml"
    if not manifest.exists():
        print(f"❌ Error: Cargo.toml not found in {workspace_dir}", file=sys.stderr)
        return False

    use_jemalloc = rust_target != ARM7_TARGET
    release_args = ["--release"] if release else []

    if use_cross:
        if rust_target == ARM7_TARGET:
            # Cargo does not apply --no-default-features to workspace members when using --workspace;
            # build each package with -p so jemalloc is disabled (avoids __ffsdi2 link error on armv7 musl).
            packages = _workspace_packages(manifest, project_root)
            if not packages:
                print(f"❌ Could not get workspace members for arm7", file=sys.stderr)
                return False
            cmd = [
                "cross", "build", "--manifest-path", str(manifest),
                "--target", rust_target, *release_args, "--no-default-features",
            ]
            for pkg in packages:
                cmd.extend(["-p", pkg])
            cmd.extend(extra_args)
            try:
                subprocess.run(cmd, check=True, cwd=str(project_root))
                return True
            except subprocess.CalledProcessError as e:
                print(f"❌ Build failed for {arch_name}: {e}", file=sys.stderr)
                return False
        cmd = [
            "cross", "build", "--manifest-path", str(manifest),
            "--target", rust_target, "--workspace", *release_args,
        ] + (JEMALLOC_FEATURE_ARGS if use_jemalloc else []) + extra_args
        try:
            subprocess.run(cmd, check=True, cwd=str(project_root))
            return True
        except subprocess.CalledProcessError as e:
            print(f"❌ Build failed for {arch_name}: {e}", file=sys.stderr)
            return False

    if rust_target == ARM7_TARGET:
        # Cargo does not apply --no-default-features to workspace members when using --workspace;
        # build each package with -p so jemalloc is disabled (avoids __ffsdi2 link error on armv7 musl).
        packages = _workspace_packages(manifest, project_root)
        if not packages:
            print(f"❌ Could not get workspace members for arm7", file=sys.stderr)
            return False
        base = ["cargo", "zigbuild"] if use_zigbuild else ["cargo", "build"]
        cmd = base + ["--target", rust_target, *release_args, "--no-default-features"]
        for pkg in packages:
            cmd.extend(["-p", pkg])
        cmd.extend(extra_args)
    else:
        jemalloc = JEMALLOC_FEATURE_ARGS if use_jemalloc else []
        if use_zigbuild:
            cmd = ["cargo", "zigbuild", "--target", rust_target, "--workspace", *release_args] + jemalloc + extra_args
        else:
            cmd = ["cargo", "build", "--target", rust_target, "--workspace", *release_args] + jemalloc + extra_args
    try:
        env = _get_cargo_env(rust_target) if not use_zigbuild else os.environ.copy()
        subprocess.run(cmd, env=env, check=True, cwd=str(workspace_dir))
        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ Build failed for {arch_name}: {e}", file=sys.stderr)
        return False


def _build_service(
    project_root: Path,
    system: str,
    module: str,
    rust_target: str,
    arch_name: str,
    use_zigbuild: bool,
    use_cross: bool,
    extra_args: List[str],
    release: bool = True,
) -> bool:
    # RERP: microservices/<system>/<module>/impl (e.g. microservices/accounting/general-ledger/impl)
    # Package name in impl/Cargo.toml is rerp_<system>_<module_snake> (no _impl suffix)
    package_name = f"rerp_{system}_{module.replace('-', '_')}"
    crate = project_root / WORKSPACE_DIR / system / module / "impl"
    manifest = project_root / WORKSPACE_DIR / "Cargo.toml"
    if not crate.exists():
        print(f"❌ Error: Crate not found: {crate}", file=sys.stderr)
        return False

    use_jemalloc = rust_target != ARM7_TARGET
    release_args = ["--release"] if release else []

    if use_cross:
        args = (JEMALLOC_FEATURE_ARGS if use_jemalloc else []) + list(extra_args)
        if rust_target == ARM7_TARGET:
            args.insert(0, "--no-default-features")  # avoid jemalloc __ffsdi2 link error on armv7 musl
        cmd = [
            "cross", "build", "--manifest-path", str(manifest),
            "-p", package_name, "--target", rust_target, *release_args,
        ] + args
        try:
            subprocess.run(cmd, check=True, cwd=str(project_root))
            return True
        except subprocess.CalledProcessError as e:
            print(f"❌ Build failed for {arch_name}: {e}", file=sys.stderr)
            return False

    arm7_no_jemalloc = rust_target == ARM7_TARGET
    jemalloc = [] if arm7_no_jemalloc else JEMALLOC_FEATURE_ARGS
    if use_zigbuild:
        cmd = ["cargo", "zigbuild", "--target", rust_target, "-p", package_name, *release_args] + (
            ["--no-default-features"] if arm7_no_jemalloc else []
        ) + jemalloc + extra_args
    else:
        cmd = ["cargo", "build", "--target", rust_target, "-p", package_name, *release_args] + (
            ["--no-default-features"] if arm7_no_jemalloc else []
        ) + jemalloc + extra_args
    try:
        env = _get_cargo_env(rust_target) if not use_zigbuild else os.environ.copy()
        subprocess.run(cmd, env=env, check=True, cwd=str(project_root / WORKSPACE_DIR))
        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ Build failed for {arch_name}: {e}", file=sys.stderr)
        return False


def _build_for_arch(
    project_root: Path,
    target: str,
    rust_target: str,
    arch_name: str,
    use_zigbuild: bool,
    use_cross: bool,
    extra_args: List[str],
    release: bool = True,
) -> bool:
    print(f"🔨 Building for {arch_name} ({rust_target})...")
    if not use_cross and not _install_rust_target(rust_target):
        return False
    if target == "workspace":
        return _build_workspace(
            project_root, rust_target, arch_name, use_zigbuild, use_cross, extra_args, release
        )
    parts = target.split("_", 1)
    if len(parts) < 2:
        print("❌ Error: Service name must be <system>_<module> (e.g., auth_idam)", file=sys.stderr)
        return False
    system, module = parts
    return _build_service(
        project_root, system, module, rust_target, arch_name, use_zigbuild, use_cross, extra_args, release
    )


def _determine_architectures(requested: Optional[str]) -> List[str]:
    if requested == "all":
        return ["amd64", "arm64", "arm7"]
    if requested in ARCH_TARGETS:
        return [requested]
    if requested is None:
        return [detect_host_architecture()]
    print(f"❌ Unknown architecture: {requested}. Valid: amd64, arm64, arm7, all", file=sys.stderr)
    sys.exit(1)


def run(
    target: str,
    arch: Optional[str] = None,
    extra_args: Optional[List[str]] = None,
    project_root: Optional[Path] = None,
    release: bool = True,
) -> int:
    """Run host-aware build. Returns 0 on success, 1 on failure."""
    root = Path(project_root) if project_root is not None else Path.cwd()
    extra = extra_args or []
    archs = _determine_architectures(arch)
    use_zigbuild = should_use_zigbuild()
    use_cross = should_use_cross()
    ok = True
    for a in archs:
        if not _build_for_arch(
            root, target, ARCH_TARGETS[a], a, use_zigbuild, use_cross, extra, release
        ):
            ok = False
    if ok:
        print("🎉 All builds complete!")
        return 0
    print("❌ Some builds failed", file=sys.stderr)
    return 1
