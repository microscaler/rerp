# Goal 5: Correct Minimum Accounting Foundation

## Objective

Create the smallest reusable model that can issue an accounting document and post a correct double-entry journal without blocking RERP's full product evolution.

## Minimum Candidate Model

- Accounting tenant and legal entity.
- Counterparty with source-system references and legal/tax identity.
- Chart of accounts and accounts.
- Journal and fiscal period.
- Invoice and immutable invoice-line snapshots.
- Tax code/rate snapshot sufficient for the first supported jurisdiction.
- Journal entry and journal-entry lines.
- Source event and idempotency link.
- Immutable rendered-document metadata and checksum.
- Audit events for state transitions.

The final model must come from accounting invariants and the Hauliage accounting-policy ADR, not from copying current placeholder entities.

## Broad Outcomes

- Decimal money and explicit ISO currency.
- Tenant-scoped identifiers and uniqueness.
- Draft, issue/post, credit/debit note, void, and reversal semantics.
- Atomic invoice-to-journal posting.
- Fiscal-period and account controls.
- Lifeguard-owned models and migrations.
- Extensibility through typed dimensions and source references rather than arbitrary JSON as the primary domain model.

## Initial Acceptance Gates

- Fresh-install and upgrade migrations apply through the supported migrator.
- An unbalanced entry cannot be posted.
- Posted documents and entries cannot be edited or deleted.
- Closed periods reject posting.
- Lines cannot debit and credit simultaneously.
- Invoice totals equal their persisted line and tax snapshots.
- Reversal and credit-note flows preserve the original audit chain.
- The initial trial balance can be derived from posted journal lines.
- No execute_unprepared path is required by product code.

## Questions To Thrash Out

- What belongs in the shared entity crate versus service-local models?
- Does the first slice need separate AR/AP subledgers or only source-linked control accounts?
- How are legal document sequences configured per tenant, entity, jurisdiction, and year?
- Which tax calculations belong in the initial core and which require localization modules?
- Are balances derived, projected, or both?
- Which dimensions are first-class at launch?

## Dependencies

- The accounting-policy ADR.
- Goal 4 for non-null tenancy and RLS.
