# ADR 002: Documents Owns Generated Document Renditions

- **Status**: ACCEPTED
- **Date**: 2026-07-15
- **Decision owners**: RERP Documents and Accounting
- **Group**: Cross-suite document generation
- **Authority**: Normative
- **Scope**: documents.generated-rendition-ownership
- **Last reviewed**: 2026-07-15
- **Supersedes**: None
- **Superseded by**: None
- **First consumer**: Accounting customer invoices and credit notes

## Context

Accounting has delivered a narrow `rerp-basic-pdf` renderer so the first invoice-to-GL vertical slice can produce and retrieve a real immutable artifact. That implementation proved deterministic rendering, content-addressed MinIO storage, artifact metadata and authorized download URLs.

RERP is still early stage. Building rich templates, general PDF generation, copy stamping, retention and electronic seals inside Accounting would establish the wrong permanent ownership and later require every other suite to duplicate or depend on Accounting for a non-accounting capability.

The Documents suite already claims the canonical document store, versions, links, access control, retention, watermarking and future digital-signature concerns. Its current design is input-oriented and its runtime is not delivered, but generated output is the other half of the same product boundary.

## Decision

The Documents suite owns generic generated-document rendition.

The permanent component is `documents/render`. It owns:

- externally managed and versioned templates and assets;
- constrained template evaluation;
- HTML/CSS-to-PDF and later additional rendition formats;
- render operation idempotency and execution state;
- generated document and derivative/copy metadata;
- object-storage integration, checksums and authorized downloads;
- watermarks, copy stamps and retention metadata; and
- future electronic seals, timestamps and validation evidence.

A source suite owns:

- the authoritative business entity and lifecycle;
- business calculations and statutory facts;
- the decision to issue or correct a document;
- its typed render-model schema;
- the immutable source snapshot; and
- the public domain link from its resource to the generated document.

For invoices, Accounting owns posting, numbering, tax, totals, customer/issuer/payment facts, credit notes and the `POSTED` immutability sentinel. Documents must never recalculate those facts or query mutable customer, organization or accounting data while rendering.

## Interaction Contract

1. The source suite commits its business transaction, frozen render snapshot and durable outbox operation atomically.
2. After commit, a worker submits the complete typed render model to Documents with a stable idempotency key and source lineage.
3. Documents derives tenant and actor scope from validated Sesame identity, resolves an effective published template version against the immutable source issue time, and freezes that selection.
4. Documents creates one immutable `ORIGINAL` rendition, stores it through the Documents core storage boundary, and links it to the source suite/resource/version.
5. The source suite records or projects the Documents identifier without mutating its posted business facts.
6. A source-suite document endpoint may remain as a public façade so consumers do not orchestrate internal suites.

No Documents network call may occur while the source suite holds its database transaction or Lifeguard pool slot. The first implementation may use a post-commit worker and the rustls-compatible `may_minihttp` client; adopting a general event bus is not required by this decision.

## Immutability Model

Documents must distinguish uploaded/versioned source documents from immutable generated renditions:

```text
origin:          INGESTED | GENERATED
mutability:      VERSIONED | IMMUTABLE
rendition_role:  ORIGINAL | COPY
```

Exactly one original is permitted for a source resource, source version, rendition purpose and media type. Ordinary retrieval returns its stored bytes. A copy is a new immutable derivative with explicit lineage and stamp metadata; it is not a replacement version of the original.

## Tenancy and Security

The existing Documents research contracts that accept caller-supplied `org_id` or define local users, passwords and tenant identity are not implementation authority. Before runtime delivery, Documents contracts must align with:

- BRRTRouter validation of Sesame JWT/JWS/JWE identity;
- tenant, organization, subject and service-principal scope derived from validated claims;
- Lifeguard transaction-scoped PostgreSQL RLS;
- no caller override of authenticated tenant scope;
- the rustls/may-compatible Microscaler HTTP stack; and
- private MinIO objects exposed only through authorized short-lived access.

## Deployment Boundary

This ownership decision does not require an additional independently deployed process on day one. `documents/render` is a product and API boundary. Rendering workers may initially share a Documents deployment where safe, but CPU-, memory-, filesystem- and network-constrained rendering must be isolatable and independently scalable without changing consumer contracts.

## Transition

- The Accounting basic renderer remains only as a temporary bridge while Documents core/render reaches end-to-end acceptance.
- No rich template, copy, watermark or trust-service capability will be added to Accounting.
- The rich invoice PDF PRD is owned by Documents, with Accounting identified as its first consumer.
- Accounting cutover occurs only after Documents proves Sesame/RLS isolation, idempotent original creation, MinIO persistence, generated-client access and failure recovery.
- At cutover, transitional Accounting artifact metadata is migrated or reconciled into Documents; a second production renderer is not retained.

## Consequences

### Positive

- Every RERP suite can generate documents through one reusable capability.
- Accounting remains focused on accounting meaning and correctness.
- Template, renderer and trust-service investments are shared across invoices, statements, quotes, contracts, reports and HR documents.
- Generated files receive consistent storage, retention, access, audit and validation controls.
- RERP avoids a later cross-suite migration from Accounting-owned rendering.

### Costs

- Documents core/render must be made production-ready earlier than the input-only roadmap anticipated.
- Accounting’s first delivery now includes a retryable cross-suite handoff and link projection.
- Documents needs an immutable rendition model in addition to its mutable/versioned uploaded-document model.

## Rejected Alternatives

### Keep document rendering in Accounting

Rejected because rendering, templates, files, copies and trust services are reusable document-platform concerns and would be duplicated by other suites.

### Let every suite embed its own renderer

Rejected because it multiplies security, templating, storage, Unicode, PDF, retention and signature implementations and prevents consistent governance.

### Make Documents own invoice facts

Rejected because Documents must not become a business-domain orchestrator. Accounting remains the authority for invoice lifecycle and content.

### Call Documents synchronously inside posting

Rejected because renderer or object-store failure must not hold or partially fail Accounting’s database transaction.

## Related Documents

- `openapi/documents/DOCUMENTS_ANALYSIS/PRDs/PRD-008-Document-Generation-and-Rendition.md`
- `openapi/documents/DESIGN.md`
- `openapi/documents/core/openapi.yaml`
- `docs/adrs/001-accounting-runtime-boundary.md`
- `docs/roadmap/hauliage-accounting-dogfood/goal-06-invoice-to-gl/README.md`
