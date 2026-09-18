# Documents Render

`documents/render` is the RERP product/API boundary for creating governed documents from immutable facts owned by another suite.

## Ownership

- Source suites own business state, legal decisions, identifiers, calculations, and the frozen typed render model.
- Documents owns template bundles and publication, rendering, artifacts, source/template/snapshot lineage, derivative copies, and future electronic seals and timestamps.
- Documents core remains the canonical document/version store; render registers generated objects there rather than creating a competing store.
- Tenant and actor scope come from validated Sesame identity and Lifeguard RLS context. Requests must not select a tenant.

## Layout

- `gen/` is generated from `openapi/documents/render/openapi.yaml`.
- `impl/` is the hand-owned runtime boundary.
- The component listens on port `8080` when it is activated.

## Current status

The API contract and compileable generated scaffold exist so implementation starts in its permanent home. Generated controller bodies are placeholders and this component must not be added to Tilt, Helm, or a shared environment until persistence, template validation, sandboxed rendering, object-storage registration, Sesame authorization, Lifeguard RLS, and truthful failure handling replace them.

See [PRD-008](../../../openapi/documents/DOCUMENTS_ANALYSIS/PRDs/PRD-008-Document-Generation-and-Rendition.md) and [ADR 002](../../../../docs/adrs/002-document-generation-ownership.md).
