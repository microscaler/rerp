# Documents — Design Documents

Granular design docs for each Documents suite microservice.

- [core](core.md) — Canonical document storage, versions, folders, search
- [intake](intake.md) — Multi-channel ingestion (email, camera, webhook, drag-drop)
- [extract](extract.md) — OCR, table extraction, schema-based field extraction
- [classify](classify.md) — AI document classification, type detection
- [analyze](analyze.md) — LLM-based document Q&A and analysis
- [routes](routes.md) — Routing rules engine, document-type-to-module mapping
- [pipeline](pipeline.md) — Workflow orchestration, state machine, retry logic
- [confirmation](confirmation.md) — User verification and approval workflows

See also: [DESIGN.md](../DESIGN.md) — full system design (DB schema, API contracts, data flow, RLS).

## Audit

- [audit.md](audit.md) — Design audit report with 24 identified issues across all specs, severity ratings, and fix instructions.
