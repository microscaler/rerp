# Tax Compliance Rules Engine

Status: scaffold dossier, implementation deferred

Owner service: `tax-compliance`

Runtime gate: `tax-compliance` is contract-only until its Rust crates, Helm values, Dockerfile, workspace registration, and generated docs exist.

## Purpose

The tax compliance rules engine determines tax-period obligations, validates return working files, prepares filing payloads, and explains statutory calculations by jurisdiction.

## Ownership

Owns:

- tax rules by jurisdiction and effective date
- tax period readiness checks
- return validation
- filing payload preparation
- audit pack evidence

Does not own:

- invoice tax calculation at transaction entry
- financial report expression evaluation
- EDI submission transport
- jurisdiction-specific plugins that should live as localization extensions

## Initial Contract Anchors

- `/tax-periods`
- `/tax-rules`
- `/tax-returns`
- `/tax-returns/{id}/validate`
- `/tax-returns/{id}/submit`
- `/tax-payments`
- `/tax-audit-packs`

## Design Questions Before Implementation

- Which tax rule concepts are core and which are localization extensions?
- How are tax return lines linked back to GL, invoice, and payment source records?
- How does the engine preserve rule versions for filed returns?
- What is the dry-run contract for filing validation?

## Required BDD Slices

- Validate a draft tax return without submitting it.
- Explain each tax return line back to source records and rule versions.
- Submit a validated return through a localization adapter.
- Produce an audit pack that remains immutable after filing.
