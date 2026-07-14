# Goal 6: Public Invoice-To-GL Vertical Slice

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
- Immutable document rendering and storage.
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
