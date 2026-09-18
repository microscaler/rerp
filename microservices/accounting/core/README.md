# RERP Accounting Core

This crate is the in-process accounting kernel for RERP's first invoice-to-GL
slice. It is a library, not another service or executor abstraction.

It owns deterministic accounting decisions that must be identical whether an
instruction arrives through HTTP, a test fixture, or a future import path:

- decimal line calculation and an explicit rounding policy;
- invoice validation and immutable posted snapshots;
- balanced customer-invoice journals;
- full credit notes that preserve the original audit chain;
- deterministic request fingerprints for idempotency conflict detection; and
- trial-balance derivation from posted journal lines.

It deliberately does not own authentication, HTTP, database transactions,
number allocation, document rendering, or Hauliage-specific commercial policy.
The invoice runtime supplies authenticated Sesame/Lifeguard context, allocated
identifiers and account mappings, then persists the returned posting plan in a
single RLS-scoped transaction.

## First-phase policy boundary

The current kernel supports a customer invoice with an accounts-receivable
control account, one revenue account per line, and an optional tax-liability
account per line. The account mapping is input to the kernel. Principal/agent,
commission, pass-through, self-billing and jurisdiction-specific tax decisions
remain blocked on the Hauliage accounting-policy ADR; they are not guessed here.
