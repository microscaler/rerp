"""Setup local Docker registry for Kind (localhost:5001). Replaces setup-kind-registry.sh."""

from __future__ import annotations

import subprocess
from pathlib import Path

REG_NAME = "kind-registry"
REG_PORT = "5001"


def run(project_root: Path) -> int:
    """Create/start kind-registry, connect to kind network, optional ConfigMap. Returns 0 or 1."""
    # 1. Create or start registry
    inspect = subprocess.run(["docker", "inspect", REG_NAME], capture_output=True, text=True)
    if inspect.returncode != 0:
        print(f"📦 Creating local registry: {REG_NAME} (host port {REG_PORT})")
        subprocess.run(
            [
                "docker",
                "run",
                "-d",
                "--restart=always",
                "-p",
                f"127.0.0.1:{REG_PORT}:5000",
                "--network",
                "bridge",
                "--name",
                REG_NAME,
                "registry:2",
            ],
            check=True,
        )
        print(f"   Created and started {REG_NAME}")
    else:
        state = subprocess.run(
            ["docker", "inspect", "-f", "{{.State.Running}}", REG_NAME],
            capture_output=True,
            text=True,
        )
        if (state.stdout or "").strip() != "true":
            print(f"📦 Starting existing registry: {REG_NAME}")
            subprocess.run(["docker", "start", REG_NAME], check=True)
            print(f"   Started {REG_NAME}")
        else:
            print(f"📦 Registry already running: {REG_NAME}")

    # 2. Connect to kind network
    net = subprocess.run(
        ["docker", "network", "inspect", "kind"],
        capture_output=True,
        text=True,
    )
    if net.returncode != 0:
        print("⚠️  Docker network 'kind' not found. Create a Kind cluster first:")
        print("   kind create cluster --config kind-config.yaml")
        return 1
    nets = subprocess.run(
        [
            "docker",
            "inspect",
            "-f",
            "{{json .NetworkSettings.Networks.kind}}",
            REG_NAME,
        ],
        capture_output=True,
        text=True,
    )
    if (nets.stdout or "").strip() == "null":
        subprocess.run(["docker", "network", "connect", "kind", REG_NAME], check=True)
        print(f"🔗 Connected {REG_NAME} to kind")
    else:
        print("🔗 Registry already on kind network")

    # 3. ConfigMap (optional)
    if subprocess.run(["kubectl", "cluster-info"], capture_output=True).returncode == 0:
        cm = f"""apiVersion: v1
kind: ConfigMap
metadata:
  name: local-registry-hosting
  namespace: kube-public
data:
  localRegistryHosting.v1: |
    host: "localhost:{REG_PORT}"
    help: "https://kind.sigs.k8s.io/docs/user/local-registry/"
"""
        subprocess.run(
            ["kubectl", "apply", "-f", "-"],
            input=cm,
            capture_output=True,
            text=True,
        )
    print(f"✅ Local registry ready: push images to localhost:{REG_PORT}/<image>:<tag>")
    return 0
