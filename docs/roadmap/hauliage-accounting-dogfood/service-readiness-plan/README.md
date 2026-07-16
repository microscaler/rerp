# Hauliage Accounting Service Readiness Work-Through Plan

- **Status**: Active working plan
- **Created**: 2026-07-15
- **First consumer**: Hauliage
- **Primary suites**: Accounting and Documents
- **Canonical layout**: [`CONTRIBUTING.md`](../../../../CONTRIBUTING.md)
- **Parent roadmap**: [Hauliage Accounting Dog-Food Roadmap](../README.md)

## Purpose

This is the executable plan for making the RERP capabilities needed by
Hauliage ready for real dogfooding. It turns the seven strategic goals into a
service-by-service checklist covering:

- `microservices/accounting/invoice`;
- the required services under `microservices/accounting/`;
- `microservices/documents/render`;
- the top-level suite-aware migrator; and
- the public Accounting BFF and generated-client proof.

RERP remains the accounting system of record. Hauliage remains an ordinary API
consumer and must not gain embedded invoice, ledger, tax, payment-allocation,
bank-reconciliation, or reporting logic.

This plan is intentionally narrower than the complete RERP Accounting roadmap.
It delivers the Accounting capabilities Hauliage can exercise first without
pretending that every researched Accounting service is production-ready.

## How To Use This Plan

Status values are:

- **Not started** — no trusted implementation evidence.
- **Structural drift** — code exists but violates the canonical suite/service
  or ownership rules.
- **Scaffold only** — OpenAPI/generated/models exist, but product behavior is
  not delivered.
- **Partial** — real behavior exists, but required acceptance gates remain.
- **Blocked externally** — work requires an official third-party contract,
  credentials, policy decision, or environment.
- **Ready** — every listed acceptance criterion has durable evidence.

For each work session:

1. Audit the current worktree and preserve another contributor's changes.
2. Select the earliest incomplete work package whose dependencies are ready.
3. Implement through the owning service's `impl/` crate; never patch `gen/`.
4. Generate and validate migrations for the owning suite only.
5. Add service tests and suite-level acceptance tests as required.
6. Record the evidence and update the status table in this document.
7. Commit a coherent checkpoint only when explicitly authorized.

Do not mark a service ready merely because it compiles, has an OpenAPI contract,
or returns a generated example response.

## Product Boundary

Hauliage supplies operational facts:

- source-system and idempotency references;
- customer/supplier identity references and commercial presentation facts;
- freight descriptions, quantities, prices, discounts, taxes, and dates;
- delivery, ePOD, dispute, adjustment, and settlement facts; and
- authoritative payment-provider/bank references when cash has actually moved.

RERP owns their accounting consequences:

- legal entities, fiscal periods, sequences, account mappings, and policy;
- invoices, supplier bills, credit/debit notes, and immutable snapshots;
- journals, subledgers, allocations, aging, period controls, and audit history;
- bank statement ingestion, matching, reconciliation, and cash reporting;
- accounting reports; and
- generated accounting document templates, renditions, copies, and storage.

Hauliage must never select its tenant, internal ledger account, fiscal period,
or RLS scope in a request body.

## Non-Negotiable Engineering Constraints

These apply to every work package:

1. Preserve `openapi/<suite>/<service>` and
   `microservices/<suite>/<service>/{gen,impl}`.
2. Every effective database table/view has one `LifeModel` owner and one
   migration provider.
3. Suite foundation entities are reserved for concepts with no natural service
   owner; they never mirror service models.
4. The single migrator lives at `microservices/migrator` and produces/applies
   one explicitly selected suite at a time.
5. Accounting-only installation must not install Documents or another RERP
   suite; Documents-only installation must not install Accounting.
6. OpenAPI is the public source of truth; generated code is disposable.
7. Sesame-validated identity supplies tenant/organization/subject context;
   Lifeguard establishes transaction-scoped RLS.
8. Product code does not use `execute_unprepared` or an equivalent raw-SQL
   escape hatch.
9. Production HTTP uses the rustls-compatible Microscaler stack and
   `may_minihttp::HttpClient`; tests may not quietly establish a different
   transport contract.
10. Money uses decimal types, explicit currency and rounding policies; binary
    floating point is forbidden.
11. Posted accounting facts are immutable. Corrections are explicit
    credit/debit-note, reversal, allocation-reversal, or reconciliation
    workflows.
12. Every service runs on container port `8080`; Docker/Tilt/Helm use the
    parameterized suite/service build path.

## Current Baseline

This table is an audit starting point, not a substitute for rechecking the
worktree before implementation.

| Area | Current assessment | Readiness gap |
|---|---|---|
| Canonical contribution/layout rules | Documented | Enforce them in code and tooling. |
| Entity ownership | Structural drift | Duplicate Accounting models and suite/service overlap must be reconciled. |
| Top-level migrator | Structural drift | It compiles but retains flat Hauliage paths, all-suite coupling, and repository-root output. |
| Invoice | Partial | Re-audit the corrected worktree; prove generated HTTPS client and handoff to Documents Render. |
| General Ledger | Partial | One authoritative foundation ledger and a four-route tenant-safe read API are delivered; account/period mutation, posting, reversal, opening balances, and scale acceptance remain. |
| Accounts Receivable | Scaffold only | Deliver customer ledger, receipts, allocations, aging, and statements. |
| Accounts Payable | Scaffold only | Deliver supplier bills, approvals, payments, allocations, and aging. |
| Bank Sync | Scaffold only | Deliver bank accounts, statement ingestion, matching, reconciliation, and connector contract. |
| Financial Reports | Scaffold only | Deliver reports derived from authoritative posted/reconciled facts. |
| Accounting BFF | Partial | Publish only delivered operations and fail generation on path/operation clashes. |
| Documents Render | Contract scaffold | Deliver permanent entities, migrations, templates, rendering, MinIO, copies, and recovery. |
| Bank-specific adapter | Blocked externally | Requires official institution API/sandbox agreement and credentials. |

## Dependency And Delivery Order

```text
WP0 Structure + migrations
        |
        v
WP1 General Ledger foundation
      /   \
     v     v
WP2 Invoice   WP4 Accounts Payable
     |
     v
WP3 Accounts Receivable
      \       /
       v     v
      WP5 Banking + reconciliation
              |
              v
        WP6 Financial reporting

WP7 Documents Render runs in parallel after WP0,
then joins Invoice/AR document acceptance.

WP8 BFF + generated-client + Hauliage proof closes the tranche.
```

Work may overlap only where ownership and migration dependencies are already
clear. Generated scaffolding is not a reason to skip this order.

## WP0 — Structural And Migration Reconciliation

**Outcome:** every service has the correct implementation anatomy and every
schema object has one suite/service owner before domain expansion continues.

The audited provider/table inventory and remaining semantic review gates are in
[entity-ownership.md](entity-ownership.md).

### Tasks

- [ ] Audit and checkpoint the current corrected Accounting work without
      overwriting another contributor's changes.
- [x] Inventory every Accounting and Documents `LifeModel` by effective
      schema/table identity and registry provider.
- [x] Assign one owner to duplicated `accounting_accounts`,
      `accounting_legal_entities`, and every other overlap.
- [x] Remove or relocate duplicate model definitions without hand-editing
      generated crates.
- [ ] Preserve each service's `impl` anatomy: controllers, services, models,
      validators, config, seeds, tests, registry, library, and executable.
- [x] Make the top-level migrator require explicit suite selection.
- [x] Identify providers by `(suite, service)` and fail on duplicate tables.
- [x] Write only to `microservices/<suite>/migrations/`.
- [x] Discover seeds at
      `microservices/<suite>/<service>/impl/seeds/` and emit suite-local order.
- [ ] Separate shared platform prerequisites from Accounting-specific SQL.
- [ ] Add Accounting-only, Documents-only, and combined fresh-install tests.
- [x] Remove or retire competing migration generator entry points after the
      top-level tool is authoritative.

### Acceptance criteria

- [x] No effective table/view is claimed by two registries.
- [ ] Generating Accounting migrations does not touch Documents or a
      repository-root migration path.
- [ ] Generating Documents migrations does not compile/install Accounting as an
      implicit dependency.
- [x] A provider error fails the command; it cannot silently omit a service.
- [ ] Re-running generation with no model change produces no migration drift.
- [ ] Fresh apply and upgrade apply are both covered against native PostgreSQL.

## WP1 — General Ledger Foundation

**Owning service:** `microservices/accounting/general-ledger`

**Outcome:** RERP has one authoritative, tenant-safe double-entry ledger that
all Accounting subledgers post through without bypassing controls.

### Required capabilities

Delivered Phase 1 inspection slice:

- [x] Canonical OpenAPI is narrowed to four honest read operations; broad
      generated-era CRUD and report mocks are not registered.
- [x] Sesame identity and the exact `accounting:ledger:read` permission establish
      tenant, organization, and legal-entity scope for a Lifeguard RLS
      transaction.
- [x] Account and fiscal-period inspection use typed foundation entities and
      bounded filters.
- [x] Immutable journal retrieval validates ordered lines against header totals.
- [x] As-of-date, single-currency trial balance is derived from journal lines and
      validates every source journal before aggregation.
- [x] Canonical regeneration is deterministic and the implementation test suite
      covers identity and accounting-integrity boundaries.

Remaining capability breadth:

- [x] Legal entity and accounting configuration resolution.
- [ ] Chart of accounts and account lifecycle, type, currency, and posting
      controls.
- [ ] Fiscal periods with open, close, reopen, and hard-lock semantics.
- [ ] Journals and deterministic legal-entity/year numbering.
- [ ] Balanced journal entries and lines with typed source references.
- [ ] Posting, reversal, and audit workflows; no mutation of posted entries.
- [ ] Control-account protections for AR, AP, bank, tax, and retained earnings.
- [ ] Trial balance by tenant, legal entity, period, currency, and supported
      dimensions.
- [ ] Explicit opening balance and year-transition policy.

### Acceptance criteria

- [ ] Debit equals credit for every posted journal and transaction.
- [ ] Closed/hard-locked periods reject new posting under concurrency.
- [ ] Reversal links to the original and preserves both audit histories.
- [ ] Another tenant cannot read, post, reverse, or infer ledger facts.
- [x] The delivered as-of-date, single-currency trial balance is derived from
      posted lines and rejects independently invalid source journals.
- [x] No service owns a second copy of the canonical account/journal models.

## WP2 — Customer Invoice Hardening

**Owning service:** `microservices/accounting/invoice`

**Outcome:** an authenticated SaaS consumer can issue, retrieve, credit, and
render immutable customer accounting documents through a stable public API.

### Required capabilities

- [x] Re-audit the existing posting, retrieval, journal, credit-note, and basic
      document routes after structural reconciliation.
- [x] Promote the active OpenAPI contract to the canonical
      `openapi/accounting/invoice/openapi.yaml` path.
- [x] Preserve decimal calculation, explicit rounding, tax snapshots, account
      mapping, period checks, numbering, audit, and idempotency.
- [ ] Add partial credit/debit-note behavior only through explicit workflows.
- [ ] Store the immutable render model and durable Documents Render outbox
      instruction atomically with posting.
- [ ] Replace the temporary Accounting renderer only after Documents Render
      passes cutover acceptance.
- [ ] Prove the generated rustls client through HTTPS, including retry and
      payload-conflict behavior.

### Acceptance criteria

- [x] One valid instruction creates exactly one immutable invoice and balanced
      journal atomically.
- [x] Same key/same payload returns the original result; changed payload
      conflicts.
- [x] Failed persistence leaves no partial document, journal, audit, or success
      record.
- [x] Credit/debit notes never mutate the original invoice.
- [x] Rendered totals and presentation facts match the frozen posted snapshot.
- [x] No active route returns generated example data.

## WP3 — Accounts Receivable

**Owning service:** `microservices/accounting/accounts-receivable`

**Outcome:** posted customer invoices become an operable customer subledger,
not merely GL rows.

### Required capabilities

- [ ] Customer/counterparty accounting identity and source references.
- [ ] Open-item customer ledger sourced from posted invoices and credits.
- [ ] Receipt recording with authoritative external payment references.
- [ ] Full, partial, and multi-document allocation.
- [ ] Unapplied/on-account cash and later allocation.
- [ ] Allocation reversal without deleting historical facts.
- [ ] Due dates and configurable payment terms.
- [ ] Aging buckets and overdue status at an explicit as-of date.
- [ ] Customer statements through Documents Render.
- [ ] Collections state and promises only after the underlying ledger is
      authoritative.

### Acceptance criteria

- [ ] Invoice, credit, receipt, allocation, and outstanding balance agree.
- [ ] Allocations cannot exceed available receipt or document balance.
- [ ] Reversing an allocation restores both balances and audit linkage.
- [ ] Aging is reproducible for the same as-of date and excludes later events.
- [ ] Customer statement totals agree with the subledger and GL control account.
- [ ] Cross-tenant customer references fail closed.

## WP4 — Accounts Payable

**Owning service:** `microservices/accounting/accounts-payable`

**Outcome:** supplier liabilities and payments are controlled through an
auditable AP subledger.

### Required capabilities

- [ ] Supplier/counterparty accounting identity and source references.
- [ ] Vendor bill capture, validation, posting, and supplier credit notes.
- [ ] Approval state separated from accounting posting state.
- [ ] Payment terms, due dates, holds, and duplicate supplier-invoice detection.
- [ ] Payment proposal/batch with explicit authorization and audit.
- [ ] Full, partial, and multi-document payment allocation.
- [ ] Unapplied supplier payments and allocation reversal.
- [ ] AP aging and supplier statements.
- [ ] Hooks for later purchase-order/receipt matching without making Hauliage
      invent an RERP purchase subsystem in this tranche.

### Acceptance criteria

- [ ] A supplier bill posts one AP control liability and balanced expense/asset/
      tax lines according to configured policy.
- [ ] Duplicate supplier document references are detected in tenant/legal-entity
      scope.
- [ ] Payment authorization cannot be bypassed by a generic status update.
- [ ] Payment allocation and reversal preserve full audit history.
- [ ] AP aging agrees with supplier open items and the GL control account.

## WP5 — Banking, Connectors, And Reconciliation

**Owning service:** `microservices/accounting/bank-sync`

**Outcome:** RERP can prove cash movements from authoritative bank statements,
match them to AR/AP activity, and reconcile accounts without institution logic
leaking into the ledger.

### Connector-neutral capabilities

- [ ] Bank and bank-account registry with legal entity, currency, masked account
      identity, and GL account mapping.
- [ ] Idempotent statement and transaction ingestion using stable provider/file
      identities and payload checksums.
- [ ] Canonical transaction model retaining provider references, booking/value
      dates, amounts, currency, narrative, counterparty, and status.
- [ ] File/mock connector for development and contract acceptance.
- [ ] Connector interface for authentication, account discovery, cursor sync,
      statement retrieval, rate limits, retry, and error normalization.
- [ ] Deterministic matching rules for invoices, supplier bills, payments,
      transfers, fees, interest, and unmatched cash.
- [ ] Manual match/override with reason, actor, timestamp, and before/after facts.
- [ ] Partial, one-to-many, and many-to-one reconciliation.
- [ ] Reconciliation completion, reopen/correction, and audit history.
- [ ] Bank fees, interest, transfers, and exchange differences post explicit
      journals rather than mutate imported transactions.

### First bank adapter gate

Stanbic or Standard Chartered in Zimbabwe/South Africa are candidate first
institutions, not assumed integrations. Before implementing a production
adapter, obtain and record:

- [ ] official API/product documentation for the exact institution and country;
- [ ] sandbox/onboarding agreement and test credentials;
- [ ] supported account, balance, statement, payment, webhook, and cursor
      capabilities;
- [ ] authentication, certificate, IP allow-list, signing, and key-rotation
      requirements;
- [ ] rate limits, retry rules, data-retention constraints, and support contacts;
      and
- [ ] confirmation that the integration is an approved API or file channel—no
      screen scraping.

Credentials and customer bank data never enter the repository or test fixtures.

### Acceptance criteria

- [ ] Re-importing the same statement cannot duplicate transactions or journals.
- [ ] A changed payload under the same provider identity fails visibly.
- [ ] Matching does not itself claim cash settlement until authoritative bank
      evidence exists.
- [ ] Reconciled bank balance, imported statement balance, and GL balance can be
      explained for an explicit date.
- [ ] Connector outage/retry cannot double-post receipts or payments.
- [ ] Tenant and legal-entity boundaries apply to accounts, transactions,
      matching, and reports.

## WP6 — Financial Reporting

**Owning service:** `microservices/accounting/financial-reports`

**Outcome:** Hauliage can inspect trustworthy financial state without querying
RERP tables or reconstructing accounting in its own code.

### Required reports

- [ ] Trial balance.
- [ ] Profit and loss.
- [ ] Balance sheet.
- [ ] Cash movement and bank cash position.
- [ ] Customer ledger and AR aging.
- [ ] Supplier ledger and AP aging.
- [ ] Reconciliation status and unmatched cash.
- [ ] Tax/VAT summary sufficient for the approved initial jurisdiction policy.

### Acceptance criteria

- [ ] Reports use posted/reconciled authoritative facts only.
- [ ] Every report has explicit tenant, legal entity, currency, period/as-of
      date, and comparison semantics.
- [ ] P&L and balance-sheet totals reconcile to the same trial balance.
- [ ] AR/AP reports reconcile to their GL control accounts or expose an explicit
      exception.
- [ ] Re-running a historical report with unchanged facts is deterministic.
- [ ] Report data and exported documents pass RLS and authorization tests.

Full consolidation, advanced budgeting, lease accounting, and statutory filing
engines are separate roadmap tranches.

## WP7 — Documents Render

**Owning component:** `microservices/documents/render`

**Outcome:** Accounting documents use a reusable Documents-suite rendition
service rather than a permanent renderer embedded in Accounting.

### Required capabilities

- [ ] Documents-owned entity registry and suite-local migrations.
- [ ] External HTML/CSS template bundles with constrained, documented fields.
- [ ] Template validation, immutable published versions, effective-date
      selection, retirement, and preview.
- [ ] Frozen typed render model supplied by the source suite; Documents does not
      recalculate accounting or query mutable customer facts.
- [ ] Deterministic original rendition with stable idempotency and checksum.
- [ ] Private content-addressed MinIO storage and authorized short-lived access.
- [ ] Explicit `ORIGINAL` and `COPY` artifacts with lineage and copy/regeneration
      stamp metadata.
- [ ] Unicode fonts, pagination, repeating headers/footers, tables, totals, and
      controlled page breaks.
- [ ] Sandboxed rendering with bounded CPU, memory, filesystem, network, input,
      and output.
- [ ] Retry, dead-letter/recovery, observability, and orphan-object cleanup.

### Acceptance criteria

- [ ] The same source version/template version/media type produces one original.
- [ ] Ordinary retrieval returns stored bytes rather than regenerating.
- [ ] A copy is a new immutable derivative and never replaces the original.
- [ ] Accounting can post successfully while rendering is unavailable; the
      durable outbox retries after commit.
- [ ] Another tenant cannot enumerate metadata, objects, templates, previews,
      or download URLs.
- [ ] Invoice, credit-note, customer-statement, and supplier-statement examples
      pass visual and data fidelity checks.

Electronic sealing, qualified timestamps, PAdES long-term validation, and
visible notarisation panels remain post-MVP unless a jurisdiction/customer
requirement promotes them.

## WP8 — Accounting BFF, Generated Client, And Dogfood Proof

**Owning services:** `microservices/accounting/bff` plus the public service
contracts selected by the Accounting suite configuration.

**Outcome:** Hauliage can consume the complete ready slice as an ordinary
external SaaS tenant without private routes or database knowledge.

### Tasks

- [ ] Aggregate only selected Accounting service contracts.
- [ ] Namespace legitimate AP/AR overlaps and fail BFF generation on unresolved
      path/operation/schema clashes.
- [ ] Expose only implemented operations; no generated mock/example handlers.
- [ ] Generate the rustls-compatible consumer and prove HTTPS, identity,
      timeout, redirect, retry, and error behavior.
- [ ] Provide contract fixtures/mock server so RERP work can continue while the
      Hauliage worktree is owned by another contributor.
- [ ] Add consumer-driven contract tests for the intended Hauliage requests and
      responses without freight-specific RERP endpoints.
- [ ] Add shared-cluster smoke, observability, and rollback evidence.

### Acceptance scenarios

#### Order to cash

- [ ] Hauliage submits one idempotent invoice instruction.
- [ ] RERP posts the invoice and balanced GL journal.
- [ ] AR exposes the open customer item and aging.
- [ ] Documents produces and stores the original invoice.
- [ ] A bank statement transaction is ingested and allocated to the invoice.
- [ ] Reconciliation proves the cash movement.
- [ ] AR, GL, bank, P&L, balance sheet, and statement outputs agree.

#### Procure to pay

- [ ] A supplier bill is captured, approved, and posted.
- [ ] AP exposes the open supplier item and aging.
- [ ] A controlled supplier payment is recorded from authoritative evidence.
- [ ] The bank transaction is matched and reconciled.
- [ ] AP, GL, bank, P&L, and balance sheet outputs agree.

#### Failure and isolation

- [ ] Retries cannot duplicate documents, journals, receipts, payments,
      allocations, bank transactions, renditions, or reports.
- [ ] Forced failure at every transaction boundary leaves no partial accounting
      state.
- [ ] Cross-tenant and cross-legal-entity access fails at API and database layers.
- [ ] Logs, metrics, traces, and errors do not expose secrets, bank payloads, or
      another tenant's identifiers.

## Cross-Cutting Non-Functional Acceptance

| ID | Requirement | Acceptance |
|---|---|---|
| NFR-DOG-001 | Correctness | Every posted journal balances; subledgers reconcile to control accounts. |
| NFR-DOG-002 | Atomicity | Each accounting command commits all facts/audit/idempotency or none. |
| NFR-DOG-003 | Tenant safety | API and native PostgreSQL tests prove fail-closed RLS isolation. |
| NFR-DOG-004 | Determinism | Same immutable facts and policy produce the same calculation/report/render result. |
| NFR-DOG-005 | Idempotency | Stable retries return the original result; payload drift conflicts. |
| NFR-DOG-006 | Precision | Decimal money, explicit currency and rounding; no floating point. |
| NFR-DOG-007 | Auditability | Every state transition records actor, time, reason, source, and lineage. |
| NFR-DOG-008 | Transport security | Production/generated clients use rustls-compatible HTTPS with bounded timeouts. |
| NFR-DOG-009 | Installation isolation | Selected suite migrations/images only; no implicit all-suite install. |
| NFR-DOG-010 | Recovery | Post-commit work is durable/retryable and operators can inspect/recover failures. |
| NFR-DOG-011 | Observability | Structured logs, metrics and traces identify suite/service/operation without leaking protected data. |
| NFR-DOG-012 | Performance | Bounded queries/work per page/batch; load tests establish initial SLO baselines before production claims. |
| NFR-DOG-013 | Contract honesty | Active APIs never return generated examples as product behavior. |
| NFR-DOG-014 | Portability | Self-hosted core accounting and migrations do not require a proprietary RERP SaaS path. |

## Explicit External Decisions And Dependencies

The following do not block WP0 or generic service implementation, but they gate
specific production policies/adapters:

- [ ] Accounting-policy ADR: principal versus agent, invoice parties, carrier
      self-billing, commission/pass-through/insurance treatment, delivery and
      dispute accounting events.
- [ ] Initial jurisdiction and approved tax/VAT policy, tax point, document
      fields, currencies, exchange-rate source, and rounding.
- [ ] Official first-bank product/API/sandbox selection and credentials.
- [ ] Production object-storage retention and encryption policy.
- [ ] Initial report presentation and statutory export expectations.

When these decisions are unavailable, implement configurable domain-neutral
contracts and tested mock/file adapters. Do not invent jurisdiction or bank
behavior.

## Explicit Deferrals

These existing Accounting research areas are not automatically activated by
this tranche:

- fixed assets and depreciation;
- budgeting and forecasting beyond basic report comparisons;
- EDI beyond a demonstrated Hauliage requirement;
- multi-entity consolidation and eliminations;
- full treasury, liquidity forecasting, hedging, and investment management;
- full revenue-recognition schedules;
- lease accounting;
- broad tax-compliance filing/localization engines;
- advanced audit-controls/rules engines beyond required posting/approval/audit
  controls; and
- electronic notarisation/sealing beyond the Documents post-MVP plan.

They remain part of RERP's world-class ERP destination and should be promoted
when Hauliage or another consumer has a concrete requirement and the preceding
accounting foundation is ready.

## Definition Of Done

This work-through plan is complete only when:

- [ ] WP0 through WP8 acceptance criteria are satisfied or explicitly replaced
      by an accepted ADR;
- [ ] Accounting and Documents install independently and together;
- [ ] the public generated client completes order-to-cash and procure-to-pay
      acceptance through HTTPS;
- [ ] AR, AP, bank, GL, and reports reconcile for the representative scenarios;
- [ ] immutable documents are produced by Documents Render and recover after
      transient failure;
- [ ] cross-tenant, retry, concurrency, rollback, and generated-example failure
      paths are proven;
- [ ] the APIs and models contain no freight-only assumptions; and
- [ ] Hauliage can replace its mock contract with the same public RERP endpoints
      without embedding accounting behavior.

## Evidence Log

Add concise entries as work packages advance:

| Date | Work package | Evidence | Result |
|---|---|---|---|
| 2026-07-15 | Plan creation | Canonical suite/service layout, current invoice-to-GL baseline, Documents ADR/PRD, and service scaffold audit | Active plan established |

## Related Documents

- [Parent dogfood roadmap](../README.md)
- [Goal 5 — Accounting foundation](../goal-05-accounting-foundation/README.md)
- [Goal 6 — Invoice to GL](../goal-06-invoice-to-gl/README.md)
- [Goal 7 — Hauliage integration](../goal-07-hauliage-integration/README.md)
- [Accounting runtime ADR](../../../adrs/001-accounting-runtime-boundary.md)
- [Documents ownership ADR](../../../adrs/002-document-generation-ownership.md)
- [Documents generation and rendition PRD](../../../../openapi/documents/DOCUMENTS_ANALYSIS/PRDs/PRD-008-Document-Generation-and-Rendition.md)
- [Service implementation and database layout](../../../llmwiki/topics/service-implementation-and-database-layout.md)
- [Hauliage reference operating model](../../../llmwiki/topics/hauliage-reference-operating-model.md)
