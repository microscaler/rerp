# RERP Docs Catalog

Inventory of source material that this wiki synthesizes. This is not a complete docs index; it lists the documents most relevant to agent decisions and known drift.

## Agent And Wiki Rules

- [`AGENTS.md`](../../AGENTS.md) — Agent rules, suite/BFF model, documentation placement, tooling-only automation policy, Lifeguard index cautions.
- [`docs/README.md`](../README.md) — Human current-authority index and normative-versus-delivered truth hierarchy.
- [`docs/DOCUMENTATION_GOVERNANCE.md`](../DOCUMENTATION_GOVERNANCE.md) — Controlled document lifecycle, authority, retirement, external-source, and supersession policy.
- [`docs/authority.json`](../authority.json) — CI-validated machine registry of current normative and active working authorities.
- [`docs/adrs/README.md`](../adrs/README.md) — Complete ADR register and next identifier.
- [`docs/llmwiki/SCHEMA.md`](./SCHEMA.md) — Wiki source-of-truth order and update workflow.

## Tooling And BRRTRouter

- [`docs/TOOLS_ALIGNMENT_FINDINGS.md`](../TOOLS_ALIGNMENT_FINDINGS.md) — RERP vs Hauliage layout/tooling comparison. Historical claims about the wrapper may predate the 2026-04-25 suite-aware wrapper fix; prefer [`topics/suite-aware-brrtrouter-wrapper.md`](./topics/suite-aware-brrtrouter-wrapper.md) for current behavior.
- [`docs/TOOLING_MIGRATION_PLAN.md`](../TOOLING_MIGRATION_PLAN.md) — Plan to move automation into `tooling/` and expose it through the `rerp` CLI.
- [`tooling/README.md`](../../tooling/README.md) — User-facing `rerp` CLI command reference.

## Delivery Roadmaps

- [`docs/roadmap/hauliage-accounting-dogfood/`](../roadmap/hauliage-accounting-dogfood/) — Current seven-goal execution overlay: recover RERP, prove the public invoice-to-GL core through Hauliage, then resume broader accounting runtime expansion.
- [`openapi/documents/DOCUMENTS_ANALYSIS/PRDs/PRD-008-Document-Generation-and-Rendition.md`](../../openapi/documents/DOCUMENTS_ANALYSIS/PRDs/PRD-008-Document-Generation-and-Rendition.md) — cross-suite PRD for immutable, versioned HTML/CSS document rendering, copy artifacts, and post-MVP electronic sealing and timestamping.
- [`docs/adrs/001-accounting-runtime-boundary.md`](../adrs/001-accounting-runtime-boundary.md) — First-slice decision to post invoice and GL atomically in the existing invoice runtime, with no new deployable or executor abstraction.
- [`microservices/accounting/core/`](../../microservices/accounting/core/) — Pure tested accounting kernel for decimal invoice calculations, journals, credit notes, fingerprints and trial balance.
- [`openapi/accounting/design/08-implementation-roadmap.md`](../../openapi/accounting/design/08-implementation-roadmap.md) — Full accounting product destination across ledger, operations, financial management, advanced accounting, and compliance.
- [`docs/history/plans/ACCOUNTING_BUILD_PLAN.md`](../history/plans/ACCOUNTING_BUILD_PLAN.md) — Historical build inventory and breadth plan; its scaffold-all-services-first order is superseded by the Hauliage dog-food overlay.

## OpenAPI And BFF

- [`docs/OPENAPI_SPEC_AUDIT.md`](../OPENAPI_SPEC_AUDIT.md) — Current maturity/gap analysis across suites.
- [`docs/OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md`](../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md) — Accounting suite benchmark against Odoo Enterprise, including the canonical target state and definition-of-done criteria for world-class accounting engines.
- [`docs/ACCOUNTING_ENTERPRISE_ERP_GAP_ANALYSIS.md`](../ACCOUNTING_ENTERPRISE_ERP_GAP_ANALYSIS.md) — Second-phase accounting gap analysis versus SAP S/4HANA Finance, Microsoft Dynamics 365 Finance, NetSuite, Sage Intacct, Workday, and other enterprise finance systems after the current documented target is delivered.
- [`docs/OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md`](../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md) — Actionable service-by-service mapping from RERP accounting specs to Odoo Enterprise workflow/module anchors and proposed OpenAPI backlog resources.
- [`docs/ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../ACCOUNTING_BDD_FEATURE_BACKLOG.md) — Broad behavior-driven accounting feature backlog; maps maturity themes to high-level Given/When/Then scenarios before granular feature/test explosion.
- [`docs/accounting/rules-engines/`](../accounting/rules-engines/) — Contract-first design dossiers for accounting rules engines, including detailed reconciliation and report expression engine designs plus scaffold dossiers for later engines.
- [`docs/history/architecture-snapshots/SYSTEM_BFF_GENERATION.md`](../history/architecture-snapshots/SYSTEM_BFF_GENERATION.md) — Historical BFF generation architecture.
- [`docs/history/implementation-snapshots/BFF_GENERATION_COMPLETE.md`](../history/implementation-snapshots/BFF_GENERATION_COMPLETE.md) — Historical BFF completion status; mentions retired script paths.
- [`openapi/accounting/bff-suite-config.yaml`](../../openapi/accounting/bff-suite-config.yaml) — Current accounting BFF source config.

## Database And Entities

- [`docs/history/implementation-snapshots/ENTITY_MIGRATION_COMPLETE.md`](../history/implementation-snapshots/ENTITY_MIGRATION_COMPLETE.md) — Historical repository-root entity migration status; paths are superseded.
- [`docs/history/architecture-snapshots/SERVICE_MAPPING.md`](../history/architecture-snapshots/SERVICE_MAPPING.md) — Historical entity-to-service mapping; current ownership is defined by `CONTRIBUTING.md`.
- [`microservices/accounting/entities/README.md`](../../microservices/accounting/entities/README.md) — Current Accounting foundation entity crate.
- [`microservices/accounting/entities/src/`](../../microservices/accounting/entities/src/) — Accounting suite foundation entities; service-specific entities live in the owning service `impl/src/models/`.

## Sibling References

- [`../hauliage/docs/llmwiki/`](../../../hauliage/docs/llmwiki/) — Working flat-layout reference for service/build/database patterns.
- [`../BRRTRouter`](../../../BRRTRouter) — Router and generator implementation.
- [`../lifeguard`](../../../lifeguard) — ORM and migration implementation.
