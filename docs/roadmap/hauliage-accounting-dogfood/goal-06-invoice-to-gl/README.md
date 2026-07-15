# Goal 6: Public Invoice-To-GL Vertical Slice

- **Status**: Five-route Phase 1 API and immutable basic PDF delivered; HTTPS consumer proof and rich templates pending
- **Runtime decision**: [ADR 001](../../../adrs/001-accounting-runtime-boundary.md)

## Objective

Deliver a reusable public API that turns an idempotent commercial instruction into an immutable invoice, balanced journal, and retrievable document.

## Candidate Public Capabilities

- Create or issue a customer invoice from a source instruction.
- Retrieve invoice, lifecycle status, totals, source links, and journal link.
- Retrieve the rendered document.
- Retrieve the resulting journal entry.
- Create a credit note or reversal through an explicit workflow.

The public API should describe accounting capabilities. It must not expose Hauliage-specific routes or require consumers to understand RERP's table layout.

## Broad Outcomes

- Versioned OpenAPI contract with named schemas and examples.
- Generated server and consumer types through BRRTRouter tooling.
- Atomic posting engine.
- Idempotency and payload-conflict detection.
- Deterministic numbering and rounding.
- Immutable document rendering and storage, expanded by the [Documents generation and rendition PRD](../../../../openapi/documents/DOCUMENTS_ANALYSIS/PRDs/PRD-008-Document-Generation-and-Rendition.md).
- Explainable error responses and audit events.
- SDK-quality documentation for third-party SaaS consumers.

## Initial Acceptance Gates

- A valid request creates exactly one invoice and one balanced journal.
- Retrying the same idempotency key and payload returns the original result.
- Reusing the key with a different payload returns a conflict.
- A failed posting leaves neither a partially posted invoice nor partial journal.
- A posted invoice cannot be mutated through generic CRUD.
- The generated document agrees exactly with persisted totals and tax.
- API consumers can follow source references without accessing internal IDs or tables.
- Contract, unit, PostgreSQL integration, and end-to-end tests cover the behavior.

## Questions To Thrash Out

- Should issue and post be one atomic command or distinct controlled transitions?
- Which resources are public and which remain internal implementation detail?
- How are long-running document generation and eventual consistency represented?
- How should validation failures, period locks, duplicate sources, and policy failures be expressed?
- What API stability and deprecation policy applies before version 1.0?
- Which SDKs should the open-source project publish first?

## Dependencies

- Goal 2 for the public runtime boundary.
- Goal 4 for authenticated tenant scope.
- Goal 5 for accounting invariants.

## First Vertical Slice

The first runtime is the existing invoice implementation process. General
ledger posting is an in-process accounting module and one database transaction,
not a synchronous call to another generated microservice. The public contract
does not expose that implementation choice.

### Command boundary

The initial public command must contain commercial facts only:

- idempotency key and stable source-system reference;
- customer, dates, ISO currency and line descriptions;
- quantities and decimal unit prices; and
- source tax facts only where permitted by configured accounting/tax policy.

Tenant, legal entity, actor, period, numbering and GL account mappings are
resolved inside RERP from validated identity and configuration. A caller may
not choose its tenant or post directly to arbitrary accounts through this API.

### Runtime transaction

1. BRRTRouter validates the Sesame identity and passes its claims.
2. The invoice runtime constructs Lifeguard `SessionContext`.
3. `LifeguardPool::with_session_transaction` pins a primary connection and
   establishes transaction-local RLS context.
4. The runtime resolves legal entity, open period, accounts, numbering and the
   existing idempotency record using typed Lifeguard APIs.
5. `rerp-accounting-core` validates the instruction and returns one posting
   plan containing invoice, journal and audit facts.
6. The runtime persists all facts and the successful idempotency result.
7. Commit makes the result visible; any error rolls the entire operation back.

### Delivered Phase 1 contract

`microservices/accounting/invoice/openapi/phase1.yaml` replaces the broad
research contract on the active invoice process. It uses decimal strings,
omits tenant/company/account selection, separates commands from retrieval and
defines explicit validation, policy, not-found and idempotency-conflict errors.
Only these operations are registered:

- `POST /v1/customer-invoices`;
- `GET /v1/customer-invoices/{id}`;
- `GET /v1/customer-invoices/{id}/journal`; and
- `POST /v1/customer-invoices/{id}/credit-notes`; and
- `GET /v1/customer-invoices/{id}/document`.

Generated example controllers are not registered by the executable. The five
active controllers call the accounting kernel and typed Lifeguard repository.

### Definition of done

Goal 6 is not complete when handlers compile. It is complete only when the
public generated client posts a real invoice through HTTPS, retrieves the same
invoice and journal, proves retry behavior, proves cross-tenant isolation in
PostgreSQL, and contains no generated example response on an active route.

Current evidence also includes immutable content-addressed PDF materialization,
private MinIO storage and short-lived download URLs. HTTPS generated-client
execution and the rich, versioned template capability remain open, so Goal 6
is deliberately not marked complete. The detailed rendering contract, copy
semantics and post-MVP electronic sealing plan are defined in the
[Documents generation and rendition PRD](../../../../openapi/documents/DOCUMENTS_ANALYSIS/PRDs/PRD-008-Document-Generation-and-Rendition.md).
