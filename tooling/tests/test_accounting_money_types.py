"""Decimal-money gate for Accounting contracts (readiness plan constraint 10).

Money must never be a binary float in an Accounting OpenAPI contract.
Services narrowed to their delivered-first verticals (Epics B1-B4 plus the
already-strict invoice/general-ledger/audit-controls contracts) may contain
no ``number``/``float``/``double`` scalar anywhere in their spec.
Not-yet-narrowed research-surface services are listed explicitly; removing a
name is the contract-narrowing slice for that service. Every service spec
must be classified — an unclassified new service fails until given an
explicit home (no implicit defaults).
"""

from pathlib import Path

import pytest
import yaml

ACCOUNTING_OPENAPI = Path(__file__).resolve().parents[2] / "openapi" / "accounting"

# Contracts narrowed to the delivered-first verticals: zero float tolerance.
STRICT = {
    "accounts-payable",
    "accounts-receivable",
    "audit-controls",
    "bank-sync",
    "financial-reports",
    "general-ledger",
    "invoice",
}
# Research surface pending its narrowing slice (Epics D-I roadmap).
DEFERRED = {
    "asset",
    "budget",
    "consolidation",
    "documents-extraction",
    "edi",
    "lease-accounting",
    "revenue-recognition",
    "tax-compliance",
    "treasury",
}


def service_specs() -> dict[str, Path]:
    return {
        d.name: d / "openapi.yaml"
        for d in ACCOUNTING_OPENAPI.iterdir()
        if d.is_dir() and (d / "openapi.yaml").exists()
    }


def float_paths(node, path: str = "") -> list[str]:
    hits: list[str] = []
    if isinstance(node, dict):
        if node.get("type") == "number" or node.get("format") in ("float", "double"):
            hits.append(path)
        for key, value in node.items():
            hits.extend(float_paths(value, f"{path}/{key}"))
    elif isinstance(node, list):
        for i, value in enumerate(node):
            hits.extend(float_paths(value, f"{path}[{i}]"))
    return hits


def test_every_service_spec_is_classified() -> None:
    unclassified = set(service_specs()) - STRICT - DEFERRED
    assert not unclassified, (
        f"new service contract(s) {sorted(unclassified)} need an explicit "
        "STRICT (float-free) or DEFERRED (pending narrowing slice) home"
    )


@pytest.mark.parametrize("service", sorted(STRICT))
def test_strict_contract_is_float_free(service: str) -> None:
    spec = yaml.safe_load(service_specs()[service].read_text(encoding="utf-8"))
    hits = float_paths(spec)
    assert not hits, (
        f"{service}: binary-float scalars in a delivered-first contract "
        f"(constraint 10): {hits[:10]}"
    )


@pytest.mark.parametrize("service", sorted(DEFERRED))
def test_deferred_service_still_has_a_contract(service: str) -> None:
    # Existence assertion only; the point of the list is that it shrinks.
    assert service_specs()[service].exists()
