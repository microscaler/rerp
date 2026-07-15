# RERP entities and entity-related docs

> **Historical catalog:** This directory documents the earlier repository-root
> entity migration and contains stale paths/import examples. It is not the
> current layout authority. Follow [`CONTRIBUTING.md`](../../CONTRIBUTING.md):
> service-owned models live in
> `microservices/<suite>/<service>/impl/src/models/`, while genuinely shared
> suite foundation models live in `microservices/<suite>/entities/`.

The material was moved from historical Lifeguard examples. Preserve it as
research/history rather than copying its repository-root entity paths.

- **RERP_ODOO_ENTITY_ANALYSIS.md** — Gaps between RERP OpenAPI/entities and Odoo accounting models.
- **Historical service mapping** — the retired root-entity mapping now lives at
  [`../history/architecture-snapshots/SERVICE_MAPPING.md`](../history/architecture-snapshots/SERVICE_MAPPING.md).
  Current ownership is defined by `CONTRIBUTING.md` and suite/service models.
- **general_ledger/MIGRATION_GAPS.md** — Migration/codegen gaps for GL entities (FK, CHECK, etc.) when generating SQL from Lifeguard entities.

See also:
- [ENTITY_AND_OPENAPI_COMPLETION](../ENTITY_AND_OPENAPI_COMPLETION.md)
- [ACCOUNTING_SUITE_ENRICHMENT_PRD](../ACCOUNTING_SUITE_ENRICHMENT_PRD.md) — Gap analysis to reach world-class, top‑5 open‑source accounting.
