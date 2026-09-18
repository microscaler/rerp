# Accounting Rules Engine Design Dossiers

Date: 2026-04-25

Purpose: define how RERP accounting rules engines are designed, reviewed, implemented, and tested. These dossiers sit between the broad BDD backlog and service implementation. They keep rule behavior explainable before it becomes business logic.

Related docs:

- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)
- [`../../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md`](../../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md)
- [`../../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md`](../../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md)
- [`../../llmwiki/topics/accounting-openapi-odoo-gap.md`](../../llmwiki/topics/accounting-openapi-odoo-gap.md)

## Design Principle

Rules engines are explainable decision systems, not hidden conditionals. Every run should be reproducible from:

- source data snapshot
- rule version
- execution options
- actor or system trigger
- service version

Every output must explain which rule produced it, why it matched, what source records it used, and what side effects were created or intentionally avoided.

## Engine Dossier Index

Build order:

1. [`reconciliation-rules-engine.md`](./reconciliation-rules-engine.md) — first, because it unlocks bank matching, write-offs, exchange differences, and unreconcile flows.
2. [`report-expression-engine.md`](./report-expression-engine.md) — second, because it unlocks report definitions, drill-down, exports, and statutory reporting foundations.
3. [`tax-compliance-rules-engine.md`](./tax-compliance-rules-engine.md) — after tax-compliance scaffolding.
4. [`extraction-classification-engine.md`](./extraction-classification-engine.md) — after documents-extraction scaffolding.
5. [`consolidation-elimination-engine.md`](./consolidation-elimination-engine.md) — after consolidation scaffolding.
6. [`revenue-recognition-engine.md`](./revenue-recognition-engine.md) — after revenue-recognition scaffolding.
7. [`lease-accounting-engine.md`](./lease-accounting-engine.md) — after lease-accounting scaffolding.
8. [`audit-controls-policy-engine.md`](./audit-controls-policy-engine.md) — after audit-controls scaffolding or when cross-service controls justify central ownership.

## Standard Dossier Structure

Each engine dossier should include:

1. **Purpose and owner**: the owning service, boundaries, and what the engine must not own.
2. **Current contract anchors**: OpenAPI paths, schemas, generated BFF paths, and BDD slices.
3. **Inputs**: source records, required fields, optional context, and service dependencies.
4. **Outputs**: decisions, suggestions, postings, artifacts, events, or errors.
5. **Rule model**: rule types, predicates, actions, priority, versioning, and status.
6. **Execution lifecycle**: draft, validate, simulate, run, approve, post, reverse, archive.
7. **Explainability**: match reasons, source lines, confidence, rule version, and trace ids.
8. **Auditability**: immutable events, actors, timestamps, before/after state, and replay data.
9. **Failure modes**: conflicts, stale source records, missing data, lock-date blocks, duplicate runs.
10. **OpenAPI contract target**: resources and request/response schemas needed before implementation.
11. **BDD acceptance slices**: feature scenarios that can become `.feature` files or integration tests.
12. **Persistence and events**: entities, projections, outbox/domain events, and idempotency keys.
13. **Rollout strategy**: feature flags, dry-run mode, version migrations, and backwards compatibility.

## Documentation Rules

- Document rule behavior before coding a rules engine.
- Prefer typed OpenAPI schemas over anonymous request bodies.
- Include dry-run and explain endpoints for engines that create accounting side effects.
- Keep rule ownership inside the service that owns the source lifecycle.
- Use extension services for jurisdiction-specific behavior rather than hardcoding local law into core services.
- Add BDD acceptance slices before adding persistence migrations or handlers.

## Pre-Implementation Gate

Before implementing an engine:

- The owning service must have a generated BFF contract.
- Required request/response schemas must be named and service-prefixed in the generated BFF.
- The BDD slice must identify fixtures, inputs, outputs, and expected side effects.
- Runtime scaffolding must exist for any new microservice owner: `gen`, `impl`, Helm values, Dockerfile, workspace registration, and generated docs.
- The engine must define how to run in dry-run mode without posting accounting side effects.
