# Documentation Governance

- **Status**: `verified`
- **Source docs**: `docs/README.md`, `docs/DOCUMENTATION_GOVERNANCE.md`, `docs/authority.json`, `docs/adrs/README.md`, `CONTRIBUTING.md`
- **Code anchors**: `tooling/scripts/check_doc_governance.py`, `tooling/tests/test_doc_governance.py`, `.github/workflows/ci.yml`
- **Last updated**: 2026-07-15

## What it is

RERP separates normative product/architecture truth from delivered runtime
truth. Accepted ADRs and approved requirements define intent; runtime, code,
schema, contracts, and tests prove current delivery. A mismatch is explicit
drift rather than an implicit change of authority.

`docs/README.md` is the human current-authority index.
`docs/authority.json` is its CI-validated machine registry. The LLM wiki remains
a derived synthesis and cannot make a stale document authoritative.

## Lifecycle

- ADRs use `PROPOSED`, `ACCEPTED`, `SUPERSEDED`, or `REJECTED`.
- Requirements/designs use draft, review, approved, implementing, delivered,
  superseded, or abandoned states.
- Analyses and implementation reports are dated evidence and become historical
  snapshots rather than silently expiring.
- Accepted ADRs are not rewritten. A material change creates a new ADR with
  reciprocal supersession metadata.
- Legacy documents are retired in place first so links and reasoning survive.

## Enforcement

`tooling/scripts/check_doc_governance.py` verifies registry fields and paths,
controlled statuses, one current normative authority per scope, reciprocal
supersession, complete ADR registration, ADR metadata, and ADR index coverage.
The tooling CI job runs the validator explicitly, and tests cover authority
clashes and supersession behavior.

> **Open:** Legacy planning and `*_COMPLETE.md` files are not automatically
> declared current. They should be classified incrementally when touched or
> when a targeted documentation audit identifies likely conflicts.

## Cross-references

- [`docs/README.md`](../../README.md)
- [`docs/DOCUMENTATION_GOVERNANCE.md`](../../DOCUMENTATION_GOVERNANCE.md)
- [`docs/adrs/README.md`](../../adrs/README.md)
- [`docs/llmwiki/SCHEMA.md`](../SCHEMA.md)
