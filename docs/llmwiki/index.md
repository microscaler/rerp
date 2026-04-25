# RERP LLM Wiki — Index

Content catalog for RERP's llm-wiki. Read this first when starting a documentation, tooling, or architecture task.

## Core Operational

- [`SCHEMA.md`](./SCHEMA.md) — Wiki purpose, source-of-truth order, page conventions, and ingest/query/session workflow.
- [`log.md`](./log.md) — Chronological append-only activity log.
- [`docs-catalog.md`](./docs-catalog.md) — Inventory of source docs and sibling references the wiki synthesizes.

## Topics

- [`topics/suite-aware-brrtrouter-wrapper.md`](./topics/suite-aware-brrtrouter-wrapper.md) — RERP's suite-nested BRRTRouter wrapper contract: keep nested layout, use Hauliage naming semantics, build impl crates, write suite-local BFF specs.
- [`topics/hauliage-reference-operating-model.md`](./topics/hauliage-reference-operating-model.md) — What RERP should borrow from Hauliage: two-crate service loop, build/deploy stages, database lessons, and what not to copy.
- [`topics/service-implementation-and-database-layout.md`](./topics/service-implementation-and-database-layout.md) — RERP-specific responsibilities for `openapi/`, `gen/`, `impl/`, shared `entities/`, Lifeguard migrations, and database config.
- [`topics/accounting-openapi-odoo-gap.md`](./topics/accounting-openapi-odoo-gap.md) — Accounting OpenAPI maturity target versus Odoo Enterprise: generated BFF coverage, new accounting service contracts, enrichment targets, accounting engines, BDD backlog, reconciliation, reporting, payments, documents, and localization.

## Entities

*(No entity pages yet.)*

## Reconciliation

- [`reconciliation/legacy-root-llmwiki-location.md`](./reconciliation/legacy-root-llmwiki-location.md) — Root `llmwiki/` is historical source material; `docs/llmwiki/` is canonical per `AGENTS.md`.

## Planned Gaps

- [ ] Reconcile older BFF automation docs that mention deleted `scripts/generate_system_bff.py` with current `tooling/`-only policy.
- [ ] Add a topic for RERP suite/BFF expansion once non-accounting suites gain `bff-suite-config.yaml`.

## Cross-References

- [`../../AGENTS.md`](../../AGENTS.md) — Agent rules for RERP.
- [`../TOOLS_ALIGNMENT_FINDINGS.md`](../TOOLS_ALIGNMENT_FINDINGS.md) — RERP vs Hauliage tooling/layout analysis.
- [`../TOOLING_MIGRATION_PLAN.md`](../TOOLING_MIGRATION_PLAN.md) — Migration from scripts into the `rerp` CLI.
- BRRTRouter sibling repo: [`../../../BRRTRouter`](../../../BRRTRouter).
- Hauliage sibling repo: [`../../../hauliage`](../../../hauliage).
