"""Registry-wiring gate for Accounting impl binaries (Epic C1').

The generated ``impl_registry.rs`` dispatches by operationId string. If a
spec gains an operation without an impl controller — or a controller is
deleted without narrowing the spec — the arm silently vanishes and the
route 404s in the field. This gate asserts three-way agreement for every
service that has an impl registry:

    spec operationIds  ==  registry arms  ==  controller files

It is the wiring analogue of the runtime-honesty gate (NFR-DOG-013): that
one checks what a controller DOES, this one checks that the ops the
contract PROMISES all have a controller.
"""

import re
from pathlib import Path

import pytest
import yaml

ROOT = Path(__file__).resolve().parents[2]
ACCOUNTING = ROOT / "microservices" / "accounting"
OPENAPI = ROOT / "openapi" / "accounting"

# Honest proxy suite entry: its registry arms dispatch generated upstream
# proxies, not impl controllers; excluded by name, like the honesty gate.
PROXY = {"bff"}

ARM_RE = re.compile(r'^\s*"[a-z0-9_]+" =>', re.M)


def registry_services() -> list[Path]:
    return sorted(
        registry.parent.parent.parent
        for registry in ACCOUNTING.glob("*/impl/src/impl_registry.rs")
        if registry.parent.parent.parent.name not in PROXY
    )


def spec_operation_ids(service: Path) -> set[str]:
    spec = yaml.safe_load((OPENAPI / service.name / "openapi.yaml").read_text(encoding="utf-8"))
    return {
        op["operationId"]
        for item in spec["paths"].values()
        for op in item.values()
        if isinstance(op, dict) and "operationId" in op
    }


def registry_arms(service: Path) -> set[str]:
    text = (service / "impl" / "src" / "impl_registry.rs").read_text(encoding="utf-8")
    return {m.strip(' "=>') for m in ARM_RE.findall(text)}


def controller_files(service: Path) -> set[str]:
    return {
        p.stem for p in (service / "impl" / "src" / "controllers").glob("*.rs") if p.stem != "mod"
    }


@pytest.mark.parametrize("svc", registry_services(), ids=lambda p: p.name)
def test_registry_arms_match_spec_operations(svc: Path) -> None:
    spec_ops = spec_operation_ids(svc)
    arms = registry_arms(svc)
    assert spec_ops, f"{svc.name}: spec has no operations"
    missing = spec_ops - arms
    ghost = arms - spec_ops
    assert not missing and not ghost, (
        f"{svc.name}: registry/spec drift — ops with no dispatch arm: {sorted(missing)}; "
        f"arms with no spec operation (orphan controllers?): {sorted(ghost)}"
    )


@pytest.mark.parametrize("svc", registry_services(), ids=lambda p: p.name)
def test_controllers_match_registry_arms(svc: Path) -> None:
    arms = registry_arms(svc)
    files = controller_files(svc)
    assert files, f"{svc.name}: no controllers on disk"
    assert files == arms, (
        f"{svc.name}: controller files and registry arms disagree — "
        f"files-only: {sorted(files - arms)}; arms-only: {sorted(arms - files)}"
    )
