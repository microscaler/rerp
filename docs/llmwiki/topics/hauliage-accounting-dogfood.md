# Hauliage Accounting Dog-Food Tranche

- **Status**: partially-verified
- **Source docs**: docs/roadmap/hauliage-accounting-dogfood/README.md, docs/roadmap/hauliage-accounting-dogfood/service-readiness-plan/README.md, openapi/documents/DOCUMENTS_ANALYSIS/PRDs/PRD-008-Document-Generation-and-Rendition.md, docs/adrs/001-accounting-runtime-boundary.md, docs/adrs/002-document-generation-ownership.md, openapi/accounting/design/08-implementation-roadmap.md, docs/history/plans/ACCOUNTING_BUILD_PLAN.md
- **Code anchors**: microservices/migrator/, microservices/accounting/, microservices/documents/render/, Tiltfile, openapi/accounting/, openapi/documents/
- **Last updated**: 2026-07-16

## Current Execution State

- Restart checkpoint d0412f3 is pushed to origin/main.
- Goal 1, the reproducible development baseline, is active on top of that checkpoint.
- The root workspace now explicitly owns `rerp-entities`; metadata and its 37-entity build pass.
- The deployable service workspace owns and commits its lockfile. Its `may_minihttp` source matches the Microscaler rustls-capable fork, and the invoice implementation compiles.
- BRRTRouter's generator now emits OpenAPI string enums. The EDI and accounting BFF generated crates compile from the current contracts.
- A full service-workspace check still fails in broad inactive AP, banking, GL,
  EDI, and BFF example stubs whose response shapes drifted during generation.
  Do not repair those stubs as product code; each owning work package must
  narrow its public runtime and remove inactive implementations from
  build/deploy gates.
- Current CI tests workspace libraries only and therefore misses stale binary implementations. An explicit active-binary gate remains open.
- Tilt now discovers and deploys the delivered Accounting runtimes, waits for
  the RERP-owned SOPS Accounting dev profile, initializes the database through
  the elected PostgreSQL HA primary, and prevents services from starting before
  database bootstrap succeeds.
- Goal 5 Phase 1 runtime is delivered. The root workspace includes `rerp-accounting-core`, a pure in-process accounting kernel with tested decimal calculation, period locks, balanced customer-invoice posting, full credit notes, idempotency fingerprints and trial balance derivation.
- ADR 001 fixes the first runtime boundary: the existing invoice process owns the invoice-to-GL transaction; GL is an in-process module, not a synchronous generated-service call or a new orchestrator.
- The 37 legacy entities remain schema-only inventory. Nine new `accounting::foundation` models are typed `LifeModel + LifeRecord` persistence surfaces for legal entities, periods, accounts, posted documents/lines, journals/lines, idempotency and audit.
- Generated foundation DDL plus app-owned controls/RLS migrations pass a live PostgreSQL acceptance suite as a non-superuser.
- The invoice executable registers five honest Phase 1 routes generated from canonical `openapi/accounting/invoice/openapi.yaml`; generated examples are not active.
- Validated Sesame claims become the complete Lifeguard context. Legal entity, period and control accounts are resolved inside one pinned RLS transaction.
- A live non-superuser Rust acceptance proves post, balanced journal, same-payload retry, changed-payload conflict, retrieve and full credit note.
- Database setup now installs the vendored Sesame RLS contract before accounting controls and grants the complete explicit v1 helper set to the runtime role.
- Database and object-store credentials have moved out of RERP manifests into
  separate least-privilege SOPS Secrets. Pgpool consumes the matching RERP
  custom-user credential from the platform PostgreSQL HA profile.
- Legal-entity/year advisory locking serializes document and journal numbering; the live acceptance proves two simultaneous postings receive distinct numbers.
- The fifth Phase 1 route materializes a deterministic basic PDF from the immutable invoice snapshot, stores it content-addressed in private MinIO, records immutable artifact metadata, and returns a short-lived authorized download URL.
- The basic renderer is a delivery baseline, not the target template system: it has a hard-coded ASCII layout and incomplete issuer/customer presentation facts.
- Documents PRD-008 defines externalized HTML/CSS templates, a constrained Jinja-compatible field language, immutable template versions, frozen render models, explicit copy artifacts, and post-MVP PAdES electronic sealing/timestamping. Accounting owns invoice facts and the frozen snapshot; `documents/render` owns rendition production and storage.
- Goal 6 remains active for HTTPS generated-client proof and the rich template-driven document capability.
- The canonical Invoice OpenAPI was restored after suite restructuring had
  accidentally regenerated the broad research contract over the five-route
  runtime. Regeneration is now deterministic and targeted tests compile again.
- General Ledger no longer owns the five undelivered predecessor tables. The
  RLS-protected `accounting_*` foundation is the only authoritative ledger.
- The canonical General Ledger API is now a four-route read-only slice for
  accounts, fiscal periods, immutable journal retrieval, and an as-of-date
  single-currency trial balance. The runtime requires
  `accounting:ledger:read`, installs the Sesame identity as Lifeguard RLS
  context, and never registers generated mocks.
- Trial balance is computed from immutable lines after every selected journal
  passes independent header/line and debit/credit integrity checks. Account and
  period mutation, generic posting/reversal, opening balances, dimensions, and
  scale acceptance remain WP1 work.

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

The [service readiness work-through plan](../../roadmap/hauliage-accounting-dogfood/service-readiness-plan/README.md)
is the active checklist across structural migration work, GL, Invoice, AR, AP,
banking/connectors, reporting, Documents Render, BFF aggregation, and generated
client dogfood proof.

## Important Boundary

RERP must not absorb Hauliage-specific freight concepts into its accounting model. Hauliage supplies source references and commercial facts. RERP owns accounting documents, tax snapshots, journals, periods, audit, and reporting.

The first integration must use the same authenticated, versioned OpenAPI contract intended for other SaaS products and self-hosted users.

## Execution Drift

> **Drift:** The older accounting build plan prioritizes scaffolding every contract-only service before implementing accounting behavior. The dog-food tranche supersedes that execution order while preserving the full product roadmap.

> **Drift:** Current generated implementation surface is not evidence of delivered accounting behavior. Runtime activation must require tested controllers and must not expose generated example responses.

> **Open:** Principal-versus-agent treatment, invoice parties, carrier self-billing, tax point, jurisdiction, and settlement journals require a joint Hauliage/RERP ADR before the accounting model and posting rules are frozen.

> **Resolved drift:** The active Phase 1 invoice OpenAPI uses decimal strings,
> derives scope internally and exposes only post/retrieve/journal/full-credit
> capabilities. The old broad contract remains historical research, not an
> active runtime surface.

## Gotchas

- Do not treat quote acceptance as payment or escrow funding.
- Do not treat an escrow status boolean as proof of a bank movement.
- Do not let caller-provided company IDs choose tenant scope.
- Do not create private Hauliage-only RERP endpoints.
- Do not activate advanced services merely because contracts or generated crates exist.

## Cross-References

- [Roadmap overview](../../roadmap/hauliage-accounting-dogfood/README.md)
- [Service readiness work-through plan](../../roadmap/hauliage-accounting-dogfood/service-readiness-plan/README.md)
- [Document generation and rendition PRD](../../../openapi/documents/DOCUMENTS_ANALYSIS/PRDs/PRD-008-Document-Generation-and-Rendition.md)
- [Hauliage reference operating model](./hauliage-reference-operating-model.md)
- [Accounting OpenAPI maturity target](./accounting-openapi-odoo-gap.md)
- [Service implementation and database layout](./service-implementation-and-database-layout.md)
