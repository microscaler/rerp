#!/usr/bin/env python3
"""
Host-aware build script for RERP microservices.

Detects the current architecture and builds accordingly, with support for
cross-compilation to multiple architectures (amd64, arm64, arm7).

Usage:
    python3 scripts/host-aware-build.py <target> [architecture] [extra cargo args...]

Targets:
    workspace → build all microservices
    <system>_<module> → build specific service (e.g., auth_idam)

Architectures (optional, defaults to host-aware):
    amd64 → x86_64-unknown-linux-musl
    arm64 → aarch64-unknown-linux-musl (Apple Silicon / ARM64)
    arm7  → armv7-unknown-linux-musleabihf (Raspberry Pi)
    all   → build all architectures

Examples:
    python3 scripts/host-aware-build.py workspace
    python3 scripts/host-aware-build.py auth_idam
    python3 scripts/host-aware-build.py auth_idam amd64
    python3 scripts/host-aware-build.py auth_idam all
"""

import sys
import os
import subprocess
import platform
from pathlib import Path
from typing import List, Dict, Optional

# Architecture to Rust target mapping
ARCH_TARGETS: Dict[str, str] = {
    "amd64": "x86_64-unknown-linux-musl",
    "arm64": "aarch64-unknown-linux-musl",
    "arm7": "armv7-unknown-linux-musleabihf",
}

# Architecture to Docker platform mapping
ARCH_PLATFORMS: Dict[str, str] = {
    "amd64": "linux/amd64",
    "arm64": "linux/arm64",
    "arm7": "linux/arm/v7",
}


def detect_host_architecture() -> str:
    """Detect the host architecture."""
    machine = platform.machine().lower()
    if machine in ("x86_64", "amd64"):
        return "amd64"
    elif machine in ("arm64", "aarch64"):
        return "arm64"
    else:
        # Default to amd64 for unknown architectures
        return "amd64"


def should_use_zigbuild() -> bool:
    """Determine if cargo-zigbuild should be used."""
    os_name = platform.system()
    arch = platform.machine()
    # Use zigbuild on macOS, or Linux non-x86_64
    return os_name == "Darwin" or (os_name == "Linux" and arch != "x86_64")


def install_rust_target(rust_target: str) -> bool:
    """Install Rust target if not already installed."""
    try:
        # Check if target is installed
        result = subprocess.run(
            ["rustup", "target", "list", "--installed"],
            capture_output=True,
            text=True,
            check=True,
        )
        if rust_target in result.stdout:
            return True
        
        # Install target
        print(f"📦 Installing Rust target: {rust_target}")
        subprocess.run(
            ["rustup", "target", "add", rust_target],
            check=True,
        )
        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ Error installing Rust target {rust_target}: {e}", file=sys.stderr)
        return False
    except FileNotFoundError:
        print("❌ Error: rustup not found. Please install Rust toolchain.", file=sys.stderr)
        return False


def get_cargo_env(rust_target: str) -> Dict[str, str]:
    """Get environment variables for cargo build based on target."""
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


def build_workspace(rust_target: str, arch_name: str, use_zigbuild: bool, extra_args: List[str]) -> bool:
    """Build entire workspace for a specific architecture."""
    components_dir = Path("components")
    if not (components_dir / "Cargo.toml").exists():
        print(f"❌ Error: Cargo.toml not found in {components_dir}", file=sys.stderr)
        return False
    
    os.chdir(components_dir)
    try:
        if use_zigbuild:
            cmd = ["cargo", "zigbuild", "--target", rust_target, "--workspace", "--release"] + extra_args
        else:
            env = get_cargo_env(rust_target)
            cmd = ["cargo", "build", "--target", rust_target, "--workspace", "--release"] + extra_args
            subprocess.run(cmd, env=env, check=True)
            return True
        
        subprocess.run(cmd, check=True)
        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ Build failed for {arch_name}: {e}", file=sys.stderr)
        return False
    finally:
        os.chdir("..")


def build_service(system: str, module: str, rust_target: str, arch_name: str, use_zigbuild: bool, extra_args: List[str]) -> bool:
    """Build a specific service for a specific architecture."""
    # Convert module name to binary name
    binary_name = f"rerp_{system}_{module.replace('-', '_')}_impl"
    crate_path = Path("components") / system / f"{module}_impl"
    
    if not crate_path.exists():
        print(f"❌ Error: Crate not found: {crate_path}", file=sys.stderr)
        return False
    
    components_dir = Path("components")
    os.chdir(components_dir)
    try:
        if use_zigbuild:
            cmd = ["cargo", "zigbuild", "--target", rust_target, "-p", binary_name, "--release"] + extra_args
        else:
            env = get_cargo_env(rust_target)
            cmd = ["cargo", "build", "--target", rust_target, "-p", binary_name, "--release"] + extra_args
            subprocess.run(cmd, env=env, check=True)
            return True
        
        subprocess.run(cmd, check=True)
        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ Build failed for {arch_name}: {e}", file=sys.stderr)
        return False
    finally:
        os.chdir("..")


def build_for_architecture(
    target: str,
    rust_target: str,
    arch_name: str,
    use_zigbuild: bool,
    extra_args: List[str],
) -> bool:
    """Build for a specific architecture."""
    print(f"🔨 Building for {arch_name} ({rust_target})...")
    
    # Install Rust target if needed
    if not install_rust_target(rust_target):
        return False
    
    # Parse target
    if target == "workspace":
        return build_workspace(rust_target, arch_name, use_zigbuild, extra_args)
    else:
        # Parse service name (system_module)
        parts = target.split("_", 1)
        if len(parts) < 2:
            print(f"❌ Error: Service name must be in format <system>_<module>", file=sys.stderr)
            print(f"   Example: auth_idam, accounting_general-ledger", file=sys.stderr)
            return False
        
        system, module = parts
        return build_service(system, module, rust_target, arch_name, use_zigbuild, extra_args)


def determine_architectures(requested_arch: Optional[str]) -> List[str]:
    """Determine which architectures to build."""
    if requested_arch == "all":
        return ["amd64", "arm64", "arm7"]
    elif requested_arch in ARCH_TARGETS:
        return [requested_arch]
    elif requested_arch is None:
        # Host-aware: build for current architecture
        host_arch = detect_host_architecture()
        return [host_arch]
    else:
        print(f"❌ Error: Unknown architecture: {requested_arch}", file=sys.stderr)
        print(f"   Valid architectures: amd64, arm64, arm7, all", file=sys.stderr)
        sys.exit(1)


def main():
    """Main entry point."""
    if len(sys.argv) < 2:
        print("usage: host-aware-build.py <target> [architecture] [extra cargo args...]", file=sys.stderr)
        print("  target: workspace or <system>_<module> (e.g., auth_idam)", file=sys.stderr)
        print("  architecture: amd64, arm64, arm7, or all (default: host-aware)", file=sys.stderr)
        sys.exit(2)
    
    target = sys.argv[1]
    
    # Parse architecture (optional)
    requested_arch: Optional[str] = None
    extra_args_start = 2
    
    if len(sys.argv) > 2 and sys.argv[2] in ("amd64", "arm64", "arm7", "all"):
        requested_arch = sys.argv[2]
        extra_args_start = 3
    
    extra_args = sys.argv[extra_args_start:]
    
    # Determine architectures to build
    build_archs = determine_architectures(requested_arch)
    
    # Determine build tool
    use_zigbuild = should_use_zigbuild()
    
    # Build for each architecture
    success = True
    for arch_name in build_archs:
        rust_target = ARCH_TARGETS[arch_name]
        if not build_for_architecture(target, rust_target, arch_name, use_zigbuild, extra_args):
            success = False
    
    if success:
        print("🎉 All builds complete!")
        sys.exit(0)
    else:
        print("❌ Some builds failed", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
