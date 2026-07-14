# Goal 5: Correct Minimum Accounting Foundation

- **Status**: Phase 1 runtime delivered; broader foundation active
- **First implementation**: [`rerp-accounting-core`](../../../../accounting-core/README.md)

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

## Phase 1: Controlled Invoicing And Ledger

“Just beyond bean counter” means the first phase is useful for operating a real
SaaS receivables flow, not merely capable of storing debits and credits.

### Functional requirements

| ID | Requirement | First delivery state |
|---|---|---|
| FR-ACCT-001 | Accept tenant and legal-entity context only from authenticated execution context. | Delivered: validated Sesame claims become the complete Lifeguard session context; request bodies cannot select scope. |
| FR-ACCT-002 | Validate ISO currency, positive quantity, non-negative unit price, discount and tax bounds. | Delivered and unit tested. |
| FR-ACCT-003 | Calculate decimal line, discount and tax snapshots under an explicit rounding policy. | Delivered; midpoint-away-from-zero and configurable minor units. |
| FR-ACCT-004 | Reject posting outside an open fiscal period. | Delivered and unit tested. |
| FR-ACCT-005 | Produce one immutable customer invoice and balanced journal as one posting plan. | Delivered through the kernel and one atomic runtime transaction. |
| FR-ACCT-006 | Debit receivables and credit line revenue plus optional tax liability using configured accounts. | Delivered without Hauliage-specific account assumptions. |
| FR-ACCT-007 | Produce a full credit note which links to and exactly reverses a posted customer invoice. | Delivered and unit tested across fiscal periods. |
| FR-ACCT-008 | Derive a tenant/legal-entity trial balance from validated posted journal lines. | Delivered and unit tested. |
| FR-ACCT-009 | Detect retry conflicts using tenant-scoped idempotency key and deterministic request fingerprint. | Delivered and proven against live PostgreSQL for retry and changed-payload conflict. |
| FR-ACCT-010 | Persist invoice, lines, journal, lines, source/idempotency and audit event in one RLS transaction. | Delivered through typed Lifeguard records and `with_session_transaction`; live rollback/RLS acceptance passes. |
| FR-ACCT-011 | Allocate tenant/legal-entity document sequences without gaps caused by rolled-back work being exposed. | Delivered: transaction advisory locking serializes legal-entity/year allocation and rollback exposes no consumed row. |
| FR-ACCT-012 | Make posted documents and entries immutable; corrections use credit/reversal workflows. | Delivered in the kernel, database controls and four-route public API. |

### Non-functional requirements

| ID | Requirement | Acceptance criterion |
|---|---|---|
| NFR-ACCT-001 | Determinism | Identical authenticated context and payload produce the same totals, journal and fingerprint. |
| NFR-ACCT-002 | Precision | No binary floating-point type enters core money or tax calculations. |
| NFR-ACCT-003 | Tenant safety | Cross-tenant or cross-legal-entity reporting/reversal input fails closed before producing a result. |
| NFR-ACCT-004 | Atomicity | A database failure at any persistence step leaves no invoice, journal, idempotency success or audit fragment. |
| NFR-ACCT-005 | Bounded work | An instruction contains 1–1,000 lines; validation and posting are linear in line count. |
| NFR-ACCT-006 | Auditability | Posted values retain line pricing, discount, tax code/rate, account mapping, source reference, actor and timestamp. |
| NFR-ACCT-007 | Portability | The accounting kernel has no HTTP, database, cloud or Hauliage dependency. |
| NFR-ACCT-008 | Failure semantics | Validation and policy failures are typed errors; generated example success responses are forbidden. |
| NFR-ACCT-009 | Testability | Every accounting invariant has unit coverage and persistence receives live PostgreSQL/RLS integration coverage. |

### Delivered persistence foundation

The active foundation is deliberately separate from the 37 legacy
schema-inventory entities:

- nine `LifeModel + LifeRecord` models under
  `entities/src/accounting/foundation/`;
- generated base DDL plus an app-owned controls/RLS migration;
- `tenant_id` and `legal_entity_id` carried directly on every accounting row;
- composite foreign keys preventing cross-tenant references even when UUIDs are
  known;
- forced PostgreSQL RLS on all nine tables;
- immutable posted document, journal, line and audit tables; and
- a live non-superuser acceptance suite under
  `tests/sql/accounting_foundation_acceptance.sql`.

### Delivered runtime evidence

The invoice process now exposes only four Phase 1 capabilities from
`microservices/accounting/invoice/openapi/phase1.yaml`: post and retrieve a
customer invoice, retrieve its journal, and post a full credit note. The
implementation resolves legal entity, open period and control accounts inside
the RLS transaction and never accepts those internal choices from the caller.

The ignored live Rust acceptance test
`live_post_retry_conflict_retrieve_and_credit` runs as a non-superuser against
a disposable PostgreSQL database. It proves initial posting, balanced journal,
same-payload retry, changed-payload conflict, retrieval and full credit. The SQL
acceptance remains responsible for cross-tenant, constraint, immutability and
forced-rollback failure paths.

Remaining Phase 1 hardening is explicit: an HTTPS generated-client proof and
immutable rendered-document storage. These are not hidden behind generated
example responses. The live acceptance also proves that two simultaneous
postings receive distinct document numbers.

### Phase 1 acceptance scenario

Given an authenticated tenant and legal entity, an open fiscal period, a
customer, configured AR/revenue/tax accounts and a unique source instruction:

1. posting returns one numbered customer invoice and one numbered journal;
2. persisted invoice totals equal the sum of immutable line snapshots;
3. journal debits equal credits and link back to the invoice;
4. repeating the same key and payload returns the original result;
5. repeating the key with changed commercial facts returns a conflict;
6. a failure before commit makes the whole attempt invisible;
7. a later full credit note posts in its own open period and nets the trial
   balance effect of the original document to zero; and
8. another tenant cannot read, report, credit or infer either document.

## Explicit deferrals

Phase 1 does not guess principal/agent status, carrier self-billing,
pass-through treatment, jurisdiction tax rules, cash settlement, bank
reconciliation, partial credit notes or foreign-currency revaluation. The core
accepts configured posting accounts and tax snapshots so those policies can be
added without rewriting its invariants.
