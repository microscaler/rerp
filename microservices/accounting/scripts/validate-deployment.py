#!/usr/bin/env python3
"""Validate the Flux-owned RERP Accounting deployment from Tilt or CI.

This is intentionally a passive acceptance watcher: it never applies a
manifest or forces a Flux reconciliation. It extracts the useful contract from
the former Skaffold watch script—selected image, deployed image, rollout and
health—and adds the Accounting foundation gates.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any


DEV_TAG = re.compile(r"^dev-[0-9]+$")
SERVICES = {
    "general-ledger": "rerp-accounting-general-ledger",
    "invoice": "rerp-accounting-invoice",
}


class Pending(RuntimeError):
    """The desired deployment state has not converged yet."""


@dataclass(frozen=True)
class Options:
    kubeconfig: Path
    timeout: int
    poll_interval: float
    app_namespace: str
    data_namespace: str
    flux_namespace: str
    skip_http_health: bool


def kubectl(options: Options, *arguments: str, json_output: bool = False) -> Any:
    command = ["kubectl", "--kubeconfig", str(options.kubeconfig), *arguments]
    completed = subprocess.run(command, capture_output=True, text=True, check=False)
    if completed.returncode != 0:
        detail = completed.stderr.strip() or completed.stdout.strip()
        raise Pending(detail or f"kubectl exited {completed.returncode}")
    if not json_output:
        return completed.stdout
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise Pending(f"kubectl returned invalid JSON: {error}") from error


def get_object(options: Options, namespace: str, kind: str, name: str) -> dict[str, Any]:
    return kubectl(
        options,
        "--namespace",
        namespace,
        "get",
        kind,
        name,
        "--output",
        "json",
        json_output=True,
    )


def condition_is_true(resource: dict[str, Any], condition_type: str) -> bool:
    return any(
        condition.get("type") == condition_type and condition.get("status") == "True"
        for condition in resource.get("status", {}).get("conditions", [])
    )


def require_flux_ready(options: Options, name: str) -> None:
    resource = get_object(options, options.flux_namespace, "kustomization", name)
    if not condition_is_true(resource, "Ready"):
        raise Pending(f"Flux Kustomization {name} is not Ready")


def require_job_complete(options: Options, name: str) -> dict[str, Any]:
    resource = get_object(options, options.data_namespace, "job", name)
    if not condition_is_true(resource, "Complete"):
        raise Pending(f"Job {options.data_namespace}/{name} is not Complete")
    return resource


def selected_image(options: Options, policy: str) -> str:
    resource = get_object(options, options.flux_namespace, "imagepolicy", policy)
    if not condition_is_true(resource, "Ready"):
        raise Pending(f"ImagePolicy {policy} is not Ready")
    latest = resource.get("status", {}).get("latestRef", {})
    image = latest.get("name")
    tag = latest.get("tag")
    if not image or not tag:
        raise Pending(f"ImagePolicy {policy} has no selected image")
    if not DEV_TAG.fullmatch(tag):
        raise Pending(f"ImagePolicy {policy} selected unexpected tag {tag!r}")
    return f"{image}:{tag}"


def pod_template_image(resource: dict[str, Any]) -> str:
    containers = resource.get("spec", {}).get("template", {}).get("spec", {}).get("containers", [])
    if len(containers) != 1 or not containers[0].get("image"):
        raise Pending("expected exactly one pod-template container image")
    return containers[0]["image"]


def require_deployment_ready(resource: dict[str, Any], name: str) -> None:
    metadata = resource.get("metadata", {})
    spec = resource.get("spec", {})
    status = resource.get("status", {})
    desired = spec.get("replicas", 1)
    if status.get("observedGeneration", 0) < metadata.get("generation", 0):
        raise Pending(f"Deployment {name} has not observed its latest generation")
    if status.get("updatedReplicas", 0) != desired or status.get("availableReplicas", 0) != desired:
        raise Pending(f"Deployment {name} rollout is not available ({status.get('availableReplicas', 0)}/{desired})")


def require_http_health(options: Options, service: str) -> None:
    path = (
        f"/api/v1/namespaces/{options.app_namespace}/services/"
        f"http:{service}:8080/proxy/health"
    )
    kubectl(options, "get", "--raw", path)


def validate_once(options: Options) -> None:
    require_flux_ready(options, "rerp-accounting")
    require_flux_ready(options, "rerp-accounting-services")

    database_job = require_job_complete(options, "rerp-accounting-db-init")
    require_job_complete(options, "rerp-accounting-object-store-init")
    expected_database_image = selected_image(options, "rerp-accounting-db-init")
    actual_database_image = pod_template_image(database_job)
    if actual_database_image != expected_database_image:
        raise Pending(
            "database bootstrap image has not converged: "
            f"deployed={actual_database_image}, selected={expected_database_image}"
        )

    for service, policy in SERVICES.items():
        expected_image = selected_image(options, policy)
        deployment = get_object(options, options.app_namespace, "deployment", service)
        actual_image = pod_template_image(deployment)
        if actual_image != expected_image:
            raise Pending(
                f"{service} image has not converged: "
                f"deployed={actual_image}, selected={expected_image}"
            )
        require_deployment_ready(deployment, service)
        if not options.skip_http_health:
            require_http_health(options, service)


def wait_for_acceptance(options: Options) -> None:
    deadline = time.monotonic() + options.timeout
    previous = ""
    while True:
        try:
            validate_once(options)
            print("RERP Accounting deployment acceptance passed")
            return
        except Pending as pending:
            message = str(pending)
            if message != previous:
                print(f"Waiting: {message}", flush=True)
                previous = message
            if time.monotonic() >= deadline:
                raise TimeoutError(
                    f"Accounting deployment did not converge within {options.timeout}s: {message}"
                ) from pending
            time.sleep(options.poll_interval)


def parse_args() -> Options:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--kubeconfig", required=True, type=Path)
    parser.add_argument("--timeout", type=int, default=600)
    parser.add_argument("--poll-interval", type=float, default=5.0)
    parser.add_argument("--app-namespace", default="rerp")
    parser.add_argument("--data-namespace", default="data")
    parser.add_argument("--flux-namespace", default="flux-system")
    parser.add_argument("--skip-http-health", action="store_true")
    args = parser.parse_args()
    if args.timeout < 1 or args.poll_interval <= 0:
        parser.error("timeout and poll interval must be positive")
    if not args.kubeconfig.is_file():
        parser.error(f"kubeconfig does not exist: {args.kubeconfig}")
    return Options(**vars(args))


def main() -> int:
    try:
        wait_for_acceptance(parse_args())
    except (Pending, TimeoutError) as error:
        print(f"Acceptance failed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
