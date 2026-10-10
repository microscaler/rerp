# Accounts Payable — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/accounts-payable
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for vendor invoices, approvals, payments,
commitments, payment batches (with export files), and three-way matching.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IAS 1 — Presentation of Financial Statements | Trade payables presented gross; commitments disclosed | <https://www.ifrs.org/issued-standards/list-of-standards/ias-1-presentation-of-financial-statements/> |
| IFRS 9 — Financial Instruments | Payables at amortised cost; payment is derecognition | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-9-financial-instruments/> |
| IAS 37 — Provisions, Contingent Liabilities and Contingent Assets | Distinguish payable (recognised) vs contingent | <https://www.ifrs.org/issued-standards/list-of-standards/ias-37-provisions-contingent-liabilities-and-contingent-assets/> |
| IAS 21 — Effects of Changes in Foreign Exchange Rates | FX-denominated vendor invoices revalued at settlement | <https://www.ifrs.org/issued-standards/list-of-standards/ias-21-the-effects-of-changes-in-foreign-exchange-rates/> |

## Endpoint inventory (20 operations)

Vendor invoices CRUD + approvals + register-payment + three-way-match;
vendor payments CRUD; commitments; cash-flow forecast; payment batches
(create/approve/export-payment-file).

## Use-case scenarios

### UC-AP-01 — Create vendor invoice with auto-approval

Given an active legal entity and vendor `V-1`.
When creating `{invoice_id: "EXT-1001", vendor_id, currency_code: USD, amount: "1200.00", tax_amount: "200.00", invoice_date: 2026-01-05, terms: NET_30, auto_approve: true}`.
Then: `net_amount = 1000.00` (1200 − 200), `outstanding_amount = 1200.00`,
status/approval_status APPROVED, payment_status UNPAID, due_date
2026-02-04, and an approval record exists with the caller as approver.

### UC-AP-02 — Duplicate external invoice id is rejected

Given UC-AP-01 exists for `EXT-1001`.
When another create uses `invoice_id: "EXT-1001"` in the same tenant.
Then 409 Conflict — the (tenant, invoice_id) composite unique key holds.

### UC-AP-03 — Due date derivation from terms

Given invoice_date 2026-01-05.
Then terms NET_30 → due 2026-02-04; NET_15 → 2026-01-20;
DUE_ON_RECEIPT → same day; an explicit due_date always wins over terms.

### UC-AP-04 — Approval workflow state machine

Given a PENDING_APPROVAL invoice.
When actions arrive in order APPROVE → APPROVE (duplicate) or
REJECT → REJECT, or any action on a POSTED/PAID invoice.
Then the second APPROVE/REJECT is rejected; actions on POSTED/PAID invoices
are rejected; REQUEST_INFO/DELEGATE return the invoice to PENDING.

### UC-AP-05 — Forward-looking aging buckets

Given due_date 2026-02-01 and today 2026-03-03 (31 days later).
Then `days_until_due = -31` and the bucket is `31-60`; boundaries: −1 →
`1-30`, −31 → `31-60`, −61 → `61-90`, −91 → `90+`, not-yet-due → CURRENT.
(AP aging is forward-looking on `days_until_due`, unlike AR's
`days_overdue`.)

### UC-AP-06 — Schedule and disburse a payment

Given an APPROVED invoice with outstanding 1200.00.
When `POST /vendor-payments {vendor_id, amount: "1200.00", currency_code: USD, payment_method: ACH, payment_date: 2026-02-10, auto_disburse: true}`.
Then the payment is COMPLETED with `actual_payment_date` set, number
`AP-20260210-<8 hex>`, and after allocation the invoice is PAID with
outstanding 0.00.

### UC-AP-07 — Payment allocation caps at outstanding

Given invoice outstanding 500.00 and a payment of 700.00 applied to it.
Then applied_amount caps at 500.00 (the remaining 200 stays unapplied), the
invoice becomes PAID, and the payment's `applied_amount` accumulates
correctly across multiple invoices.

### UC-AP-08 — Register payment against one invoice

Given an APPROVED invoice (currency USD, outstanding 900.00).
When `register-payment {amount: "900.00", currency_code: USD, ...}`.
Then a COMPLETED payment is created and allocated; an amount above
outstanding, a currency mismatch, or a zero/negative amount is rejected.

### UC-AP-09 — Commitments = approved and unpaid

Given invoices: A APPROVED/UNPAID, B APPROVED/PARTIAL, C PENDING_APPROVAL,
D APPROVED/PAID.
When listing `/commitments`.
Then A and B appear, C and D do not; the cash-flow forecast reports
`net_cash_flow = total_payments − total_commitments` (implementation sign;
the spec description states the reverse — see drift note).

### UC-AP-10 — Payment batch single-currency rule

Given APPROVED invoices in USD (1200.00) and EUR (500.00).
When creating a batch with both invoice ids and `currency_code: USD`.
Then the request is rejected; a USD-only batch is accepted, starts DRAFT, and
approval creates one SCHEDULED payment per item (oldest due date first).

### UC-AP-11 — Batch export file

Given an APPROVED batch.
When `export-payment-file {file_format: ISO20022}`.
Then a `PaymentFileExport` with status QUEUED/FILE_GENERATING is returned
(202); exporting a non-approved batch fails.

### UC-AP-12 — Three-way match

Given invoice with stored `purchase_order_id` PO-9 and two receipt ids.
When `three-way-match {purchase_order_id: "PO-9", receipt_ids: [R1, R2]}`.
Then `matched=true`, matching_status MATCHED, stored in metadata. A wrong PO
id or empty receipts → NOT_MATCHED / PARTIAL with variances.
(Note: `tolerance_amount`/`tolerance_percent` are accepted but currently
ignored — see drift.)

### UC-AP-13 — Update/delete status guards

Given a COMPLETED payment and a non-DRAFT invoice.
When updating the payment amount or deleting either record.
Then 409 — payments can only change while editable, invoices only while
DRAFT; `amount` may never drop below `tax_amount` on update.

### UC-AP-14 — Boundary validation

Given `currency_code: usd`, `amount: -5`, `amount: NaN`, page 0, limit 101.
Then each is rejected 400 `validation_error`; defaults page=1 limit=20.

### UC-AP-15 — Tenant isolation

Given tenant A and tenant B share vendor `V-1` with different invoices.
When tenant B lists vendor invoices.
Then only tenant B's rows appear; `company_id` must equal the caller's legal
entity or the call is a Policy error.

## Test coverage matrix

| Scenario | Existing test | Status |
|---|---|---|
| UC-AP-01 | none (needs DB) | not yet implemented |
| UC-AP-02 | none | not yet implemented |
| UC-AP-03 | `payables.rs::due_dates_follow_terms` | implemented |
| UC-AP-04 | none | not yet implemented |
| UC-AP-05 | `payables.rs::aging_buckets_follow_overdue_days`, `days_until_due_counts_down_and_negative_when_overdue` | implemented |
| UC-AP-06..08 | none (allocation logic exists, untested) | not yet implemented |
| UC-AP-09 | none | not yet implemented |
| UC-AP-10/11 | none | not yet implemented |
| UC-AP-12 | none (logic exists, tolerance ignored) | not yet implemented |
| UC-AP-13 | none | not yet implemented |
| UC-AP-14 | `payables.rs::validates_boundary_inputs` | implemented |
| UC-AP-15 | none | not yet implemented |

## Known drift / gaps

- Three-way match parses **neither** `tolerance_amount` nor
  `tolerance_percent` — the spec fields are accepted and ignored.
- `cash-flow-forecast.net_cash_flow` sign is opposite to the spec
  description (impl: payments − commitments).
- AP money has no rounding on parse (unlike AR's 2 dp and asset's 4 dp);
  stored as NUMERIC(19,4).
- The `ap_agings` table exists but no endpoint populates or reads it in the
  current impl (aging is computed on the fly).
