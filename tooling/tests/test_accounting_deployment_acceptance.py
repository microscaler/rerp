"""Unit tests for the passive Accounting deployment acceptance watcher."""

from __future__ import annotations

import importlib.util
import sys
from pathlib import Path

import pytest
import yaml

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "microservices/accounting/scripts/validate-deployment.py"
SPEC = importlib.util.spec_from_file_location("accounting_deployment_acceptance", SCRIPT)
assert SPEC and SPEC.loader
acceptance = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = acceptance
SPEC.loader.exec_module(acceptance)


def test_selected_image_uses_flux_v1_latest_ref_name(monkeypatch: pytest.MonkeyPatch) -> None:
    resource = {
        "status": {
            "conditions": [{"type": "Ready", "status": "True"}],
            "latestRef": {
                "name": "10.177.76.220:5000/rerp-accounting-invoice",
                "tag": "dev-1784194020656608828",
            },
        }
    }
    monkeypatch.setattr(acceptance, "get_object", lambda *_args: resource)
    options = acceptance.Options(Path("kubeconfig"), 1, 0.1, "rerp", "data", "flux-system", False)

    assert acceptance.selected_image(options, "invoice") == (
        "10.177.76.220:5000/rerp-accounting-invoice:dev-1784194020656608828"
    )


def test_selected_image_rejects_non_dev_tag(monkeypatch: pytest.MonkeyPatch) -> None:
    resource = {
        "status": {
            "conditions": [{"type": "Ready", "status": "True"}],
            "latestRef": {"name": "registry/image", "tag": "latest"},
        }
    }
    monkeypatch.setattr(acceptance, "get_object", lambda *_args: resource)
    options = acceptance.Options(Path("kubeconfig"), 1, 0.1, "rerp", "data", "flux-system", False)

    with pytest.raises(acceptance.Pending, match="unexpected tag"):
        acceptance.selected_image(options, "invoice")


def test_deployment_readiness_requires_current_available_generation() -> None:
    resource = {
        "metadata": {"generation": 4},
        "spec": {"replicas": 1},
        "status": {"observedGeneration": 4, "updatedReplicas": 1, "availableReplicas": 1},
    }
    acceptance.require_deployment_ready(resource, "invoice")

    resource["status"]["availableReplicas"] = 0
    with pytest.raises(acceptance.Pending, match="rollout is not available"):
        acceptance.require_deployment_ready(resource, "invoice")


def test_catalog_requires_suspension_and_no_deployment(monkeypatch: pytest.MonkeyPatch) -> None:
    release = {
        "metadata": {"labels": {"delivery.rerp.io/status": "scaffold-only"}},
        "spec": {"suspend": True},
    }
    monkeypatch.setattr(acceptance, "SUSPENDED_SERVICES", ("accounts-payable",))
    monkeypatch.setattr(acceptance, "get_object", lambda *_args: release)
    monkeypatch.setattr(acceptance, "kubectl", lambda *_args, **_kwargs: "")
    options = acceptance.Options(Path("kubeconfig"), 1, 0.1, "rerp", "data", "flux-system", False)

    acceptance.require_catalog_suspended(options)

    release["spec"]["suspend"] = False
    with pytest.raises(acceptance.Pending, match="is not suspended"):
        acceptance.require_catalog_suspended(options)


def test_flux_catalog_covers_every_non_active_accounting_descriptor() -> None:
    values_dir = ROOT / "helm/rerp-microservice/values"
    source_services = {
        path.parent.name
        for path in (ROOT / "openapi/accounting").glob("*/openapi.yaml")
        if (values_dir / f"{path.parent.name}.yaml").is_file()
    }
    if (values_dir / "bff.yaml").is_file():
        source_services.add("bff")
    expected = source_services - set(acceptance.SERVICES)

    catalog_path = (
        ROOT / "deployment-configuration/profiles/dev/rerp/accounting/catalog/releases.yaml"
    )
    releases = list(yaml.safe_load_all(catalog_path.read_text(encoding="utf-8")))
    actual = {release["metadata"]["name"] for release in releases}

    assert actual == expected == set(acceptance.SUSPENDED_SERVICES)
    assert all(release["spec"]["suspend"] is True for release in releases)
    assert all(
        release["metadata"]["labels"]["delivery.rerp.io/status"] == "scaffold-only"
        for release in releases
    )
