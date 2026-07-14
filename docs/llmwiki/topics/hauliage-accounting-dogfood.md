# Hauliage Accounting Dog-Food Tranche

- **Status**: partially-verified
- **Source docs**: docs/roadmap/hauliage-accounting-dogfood/README.md, docs/adrs/001-accounting-runtime-boundary.md, openapi/accounting/design/08-implementation-roadmap.md, docs/ACCOUNTING_BUILD_PLAN.md
- **Code anchors**: Cargo.toml, accounting-core/, microservices/Cargo.toml, entities/src/accounting/, Tiltfile, openapi/accounting/
- **Last updated**: 2026-07-14

## Current Execution State

- Restart checkpoint d0412f3 is pushed to origin/main.
- Goal 1, the reproducible development baseline, is active on top of that checkpoint.
- The root workspace now explicitly owns `rerp-entities`; metadata and its 37-entity build pass.
- The deployable service workspace owns and commits its lockfile. Its `may_minihttp` source matches the Microscaler rustls-capable fork, and the invoice implementation compiles.
- BRRTRouter's generator now emits OpenAPI string enums. The EDI and accounting BFF generated crates compile from the current contracts.
- A full service-workspace check reaches the stale BFF implementation and fails with 728 obsolete example-stub errors. Do not repair those stubs as product code; Goal 2 must define the narrow public runtime and remove inactive implementations from build/deploy gates.
- Current CI tests workspace libraries only and therefore misses stale binary implementations. An explicit active-binary gate remains open.
- Tiltfile is intentionally empty at the restart checkpoint; runtime restoration belongs to Goals 2 and 3 after the build topology is trustworthy.
- Goal 5 is active. The root workspace now includes `rerp-accounting-core`, a pure in-process accounting kernel with tested decimal calculation, period locks, balanced customer-invoice posting, full credit notes, idempotency fingerprints and trial balance derivation.
- ADR 001 fixes the first runtime boundary: the existing invoice process owns the invoice-to-GL transaction; GL is an in-process module, not a synchronous generated-service call or a new orchestrator.
- The 37 legacy entities remain schema-only inventory. Nine new `accounting::foundation` models are typed `LifeModel + LifeRecord` persistence surfaces for legal entities, periods, accounts, posted documents/lines, journals/lines, idempotency and audit.
- Generated foundation DDL plus app-owned controls/RLS migrations pass a live PostgreSQL acceptance suite as a non-superuser. The remaining persistence blocker is wiring those typed records into one `with_session_transaction` invoice repository.

## What It Is

The Hauliage dog-food tranche is the immediate execution overlay for RERP accounting. RERP remains a world-class open-source, API-first ERP target with full ledger, tax, treasury, reconciliation, reporting, and controls. The overlay changes delivery order: establish a trustworthy accounting core and prove it through Hauliage before activating broad generated runtime surface.

Hauliage is an external SaaS consumer, not a privileged internal accounting path. It owns freight facts and delegates accounting consequences to RERP's public APIs.

## Seven Goals

1. Reproducible development baseline.
2. Narrow and honest active runtime.
3. Shared-cluster delivery.
4. Sesame tenancy and PostgreSQL RLS.
5. Correct minimum accounting foundation.
6. Public invoice-to-GL vertical slice.
7. Hauliage as the first ordinary consumer.

Each goal has an expandable document under docs/roadmap/hauliage-accounting-dogfood/.

## Important Boundary

RERP must not absorb Hauliage-specific freight concepts into its accounting model. Hauliage supplies source references and commercial facts. RERP owns accounting documents, tax snapshots, journals, periods, audit, and reporting.

The first integration must use the same authenticated, versioned OpenAPI contract intended for other SaaS products and self-hosted users.

## Execution Drift

> **Drift:** The older accounting build plan prioritizes scaffolding every contract-only service before implementing accounting behavior. The dog-food tranche supersedes that execution order while preserving the full product roadmap.

> **Drift:** Current generated implementation surface is not evidence of delivered accounting behavior. Runtime activation must require tested controllers and must not expose generated example responses.

> **Open:** Principal-versus-agent treatment, invoice parties, carrier self-billing, tax point, jurisdiction, and settlement journals require a joint Hauliage/RERP ADR before the accounting model and posting rules are frozen.

> **Drift:** The existing invoice OpenAPI uses binary floating-point money, accepts caller-supplied company scope and exposes generic mutation. It is not the accepted Phase 1 command contract and must be corrected before regeneration.

## Gotchas

- Do not treat quote acceptance as payment or escrow funding.
- Do not treat an escrow status boolean as proof of a bank movement.
- Do not let caller-provided company IDs choose tenant scope.
- Do not create private Hauliage-only RERP endpoints.
- Do not activate advanced services merely because contracts or generated crates exist.

## Cross-References

- [Roadmap overview](../../roadmap/hauliage-accounting-dogfood/README.md)
- [Hauliage reference operating model](./hauliage-reference-operating-model.md)
- [Accounting OpenAPI maturity target](./accounting-openapi-odoo-gap.md)
- [Service implementation and database layout](./service-implementation-and-database-layout.md)
