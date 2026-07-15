# ADR 001: First Accounting Runtime Boundary

- **Status**: ACCEPTED
- **Date**: 2026-07-14
- **Decision owners**: RERP Accounting
- **Group**: Accounting runtime and data consistency
- **Authority**: Normative
- **Scope**: accounting.runtime-boundary
- **Last reviewed**: 2026-07-15
- **Supersedes**: None
- **Superseded by**: None

## Issue

RERP has broad generated invoice, general-ledger, AR and AP service surfaces but
no delivered persistence behavior. Issuing a customer invoice and its journal
is one accounting invariant and must be atomic. Implementing it as synchronous
calls between generated services would introduce a distributed transaction,
partial-result recovery and another abstraction before there is a demonstrated
independent scaling or ownership boundary.

## Decision

The first invoice-to-GL slice runs inside the existing invoice implementation
process. General-ledger posting is an in-process domain module. Invoice,
invoice lines, journal, journal lines, idempotency result and audit event are
written on one primary database connection in one Lifeguard
`with_session_transaction` closure with Sesame-derived RLS context.

`rerp-accounting-core` is a pure library holding deterministic calculations and
invariants. It is not a deployable, network service, repository framework or
executor wrapper. Lifeguard's base pool/executor remains the only RLS execution
abstraction.

The public API describes accounting capabilities and does not reveal whether
invoice and GL modules share a process. The boundary may be revisited only when
measured load, ownership or availability needs justify independent deployment
and an explicit consistency protocol exists.

## Assumptions

- RERP uses PostgreSQL as the accounting system of record.
- Sesame/BRRTRouter establish authenticated identity before business logic.
- Lifeguard provides typed CRUD and pinned transaction-scoped RLS execution.
- Hauliage is the first ordinary API consumer, not a privileged internal path.
- The first slice values accounting correctness and recoverability over broad
  service activation.

## Alternatives considered

### Invoice service calls general-ledger service synchronously

Rejected for Phase 1. HTTP success/failure cannot atomically commit both
services. Compensation would be accounting behavior in its own right and is
unnecessary while both modules use the same database and release cadence.

### Add an accounting orchestrator service

Rejected. It adds a third runtime and another abstraction without removing the
distributed transaction or owning a distinct business capability.

### Put all behavior in generated controllers

Rejected. Generated code is disposable and HTTP handlers should adapt
contracts, not own reusable accounting rules.

### Put domain behavior in the entity crate

Rejected. Lifeguard entities describe persistence and migrations. Keeping pure
accounting policy in a separate library avoids making schema types the service
layer while adding no runtime boundary.

## Implications

- The active runtime initially includes only invoice endpoints whose behavior
  is implemented and tested.
- Generic GL endpoints may remain contract-visible but cannot be advertised as
  delivered until backed by real behavior.
- Database models and migrations remain Lifeguard-owned and RLS policies remain
  app-owned migrations.
- No raw SQL exception is introduced. Missing ORM capability is extended in
  Lifeguard rather than bypassed in RERP.
- A future service split requires a new ADR addressing idempotency, ordering,
  failure recovery, audit continuity and reconciliation of partial delivery.

## Related requirements

- Goal 4: Sesame tenancy and RLS.
- Goal 5: correct minimum accounting foundation.
- Goal 6: atomic public invoice-to-GL vertical slice.
