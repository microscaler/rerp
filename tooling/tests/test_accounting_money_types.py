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
# Frozen baseline of the deferred set (Epic B5 close). The set may only
# SHRINK as services are narrowed into STRICT; moving a service out of
# STRICT unnoticed is a gate bypass and fails. This is a LITERAL snapshot
# on purpose (PR #129 review): copying live DEFERRED at import time would
# make the never-grows assertion vacuously true.
DEFERRED_BASELINE = frozenset(
    {
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
)

# The aggregate BFF is generated from all service contracts, so deferred
# services legitimately contribute float fields there. Scoped honesty:
# money fields from already-STRICT contracts must arrive decimal.
BFF_SPEC = ACCOUNTING_OPENAPI / "openapi_bff.yaml"
BFF_GEN_SRC = ACCOUNTING_OPENAPI.parents[1] / "microservices" / "accounting" / "bff" / "gen" / "src"
STRICT_MONEY_FIELDS_IN_BFF = {"match_tolerance", "description_similarity_threshold"}


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


def test_deferred_set_never_grows() -> None:
    # A STRICT service renamed into DEFERRED would hide its float fields
    # from the strict check; the deferred set is frozen at its B5 baseline
    # and may only shrink as narrowing slices land.
    grown = set(DEFERRED) - DEFERRED_BASELINE
    assert not grown, (
        f"service(s) {sorted(grown)} entered DEFERRED; deferred entries may only "
        "be removed (by narrowing into STRICT), never added or relabelled in"
    )


def test_strict_money_fields_survive_decimal_into_the_bff() -> None:
    # Scoped check (per PR #116 review): the BFF aggregate keeps floats from
    # deferred services by design, but money fields defined in already-STRICT
    # contracts must not arrive as binary floats in the generated aggregate.
    spec = yaml.safe_load(BFF_SPEC.read_text(encoding="utf-8"))
    hits = [
        path
        for path in float_paths(spec)
        if path.rsplit("/", 1)[-1].split("[")[0] in STRICT_MONEY_FIELDS_IN_BFF
    ]
    assert not hits, f"STRICT-contract money fields are floats in the BFF: {hits}"


def test_strict_money_fields_survive_decimal_into_the_generated_bff_crate() -> None:
    offenders: list[str] = []
    for rs_file in BFF_GEN_SRC.rglob("*.rs"):
        text = rs_file.read_text(encoding="utf-8")
        for line_no, line in enumerate(text.splitlines(), start=1):
            if "f64" not in line and "f32" not in line:
                continue
            if any(field in line for field in STRICT_MONEY_FIELDS_IN_BFF):
                offenders.append(f"{rs_file.name}:{line_no}: {line.strip()}")
    assert not offenders, (
        "STRICT-contract money fields became binary floats in the generated "
        f"BFF crate (regenerate from the updated contracts): {offenders}"
    )
