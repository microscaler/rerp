# Invoice — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/invoice
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for customer invoicing: atomic post
(idempotent), retrieval, journal, PDF artifact, and full credit notes. The
service posts into shared foundation entities and enforces Phase-1 rules
(base-currency-only posting, open-fiscal-period posting, deterministic
document numbering).

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IFRS 15 — Revenue from Contracts with Customers | Revenue recognised on invoice posting; credit notes reverse revenue | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-15-revenue-from-contracts-with-customers/> |
| IFRS 9 — Financial Instruments | Trade receivable recognised at invoice total | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-9-financial-instruments/> |
| IAS 1 — Presentation of Financial Statements | Complete, faithful, balanced postings | <https://www.ifrs.org/issued-standards/list-of-standards/ias-1-presentation-of-financial-statements/> |

## Endpoint inventory (OpenAPI v0.2.0)

| Method | Path | Purpose |
|---|---|---|
| POST | `/v1/customer-invoices` | Issue + post invoice atomically (201 new, 200 idempotent retry, 409 conflict) |
| GET | `/v1/customer-invoices/{id}` | Retrieve posted invoice with lines |
| GET | `/v1/customer-invoices/{id}/journal` | Balanced journal for the invoice |
| GET | `/v1/customer-invoices/{id}/document` | PDF metadata + 5-minute presigned download URL |
| POST | `/v1/customer-invoices/{id}/credit-notes` | Full credit note reversing the invoice |

## Use-case scenarios

### UC-INV-01 — Post a taxed, discounted invoice

Given legal entity with base currency GBP, an OPEN period containing
2026-01-15, AR control account `1100` (role ACCOUNTS_RECEIVABLE), revenue
account `4000` (REVENUE) and tax account `2200` (TAX_PAYABLE).
When posting
`{idempotency_key: "k-1", customer_id, invoice_date: 2026-01-15, due_date: 2026-02-14, currency_code: GBP, lines: [{description: "Consulting", quantity: 10, unit_price: "25.11", discount_percent: "10", tax: {code: "VAT", rate_percent: "20"}}]}`.
Then: gross = round(10×25.11) = 251.10; discount = round(251.10×10%) = 25.11;
net = 225.99; tax = round(225.99×20%) = 45.20; line total = 271.19.
The invoice totals subtotal 225.99, tax 45.20, total 271.19, status POSTED,
document_number `INV-2026-000001`. The journal debits AR 271.19 and credits
revenue 225.99 and tax 45.20 — balanced.

### UC-INV-02 — Idempotent retry returns the committed result

Given UC-INV-01 completed.
When the identical request (same idempotency_key, same fingerprint) is retried.
Then the response is 200 with `created=false` and the **same** invoice id and
document number; no second journal exists.

### UC-INV-03 — Idempotency conflict

Given UC-INV-01 completed.
When the same `idempotency_key` is reused with a different amount.
Then 409 `idempotency_conflict` — the stored fingerprint differs from the
request fingerprint.

### UC-INV-04 — Rounding policy: half away from zero at line level

Given rounding_minor_units=2 and a line quantity 1 × unit_price 1.005.
Then gross rounds to 1.01 (MidpointAwayFromZero), not 1.00; with
rounding_minor_units=3 the same line gross is 1.005.

### UC-INV-05 — Deterministic numbering, collision-safe

Given the legal entity already has `INV-2026-00042` (seed) and the year
sequence.
When a new invoice posts.
Then its number is `INV-2026-00043` — computed from the numeric suffix, so an
oddly-padded seed never produces a duplicate; two concurrent postings for the
same legal entity + year serialize on `pg_advisory_xact_lock` and get distinct
numbers.

### UC-INV-06 — Currency must equal base currency (Phase 1)

Given legal entity base currency GBP.
When posting an invoice with `currency_code: USD`.
Then 422 `accounting_policy_error` ("foreign-currency posting is not enabled
in Phase 1") — nothing is persisted.

### UC-INV-07 — Closed period rejection

Given the invoice date falls in a CLOSED period.
When posting.
Then 422 policy error `ClosedPeriod`; a date in no period at all yields
`DateOutsidePeriod`.

### UC-INV-08 — Boundary validation

Given requests with: `due_date < invoice_date`; zero lines; 1001 lines;
quantity 0; negative unit_price; discount_percent 101; tax rate −1;
idempotency_key of 201 chars; source.resource_id empty.
Then each is rejected 400 `validation_error` with the offending field.

### UC-INV-09 — Credit note reverses the journal exactly

Given posted invoice UC-INV-01 (debit AR 271.19 / credit revenue 225.99 + tax
45.20) in an OPEN period today.
When `POST /v1/customer-invoices/{id}/credit-notes` with reason "cancelled".
Then a `CRN-2026-000001` posts with the exact side-flipped journal
(debit revenue 225.99 + tax 45.20 / credit AR 271.19), `document_type`
CUSTOMER_CREDIT_NOTE, `original_document_id` set; the trial balance over both
documents nets to zero.

### UC-INV-10 — Credit note only against an invoice of the same scope

Given a random UUID, or an invoice of another tenant.
When a credit note is requested against it.
Then 404 / policy error respectively; a credit note of a credit note is
rejected.

### UC-INV-11 — Public JSON never exposes tenant or GL mapping

Given a posted invoice with lines carrying internal `revenue_account_id`,
`tenant_id`, `legal_entity_id`.
When the invoice is retrieved.
Then the response JSON contains none of those internal keys.

### UC-INV-12 — Journal retrieval

Given UC-INV-01.
When `GET /v1/customer-invoices/{id}/journal`.
Then entry_number `JE-2026-...`, total_debit == total_credit == "271.19",
and every line has positive one-sided amounts.

### UC-INV-13 — Deterministic PDF artifact

Given UC-INV-01.
When `GET /v1/customer-invoices/{id}/document` twice.
Then both responses carry the same `sha256` and byte-identical content
(renderer `rerp-basic-pdf` version 1), a valid PDF header `%PDF-1.4` and
`%%EOF` trailer, media_type `application/pdf`, an object key
`accounting/{legal_entity_id}/{document_id}/{sha256}.pdf` containing no
tenant or customer identifiers, and a `download_url` expiring within
5 minutes.

### UC-INV-14 — Object store unavailable

Given the S3-compatible store is unreachable.
When the document endpoint is called.
Then 503 `document_storage_unavailable`; the invoice itself remains intact.

### UC-INV-15 — Concurrent postings get distinct numbers

Given two threads post two different invoices for the same legal entity and
year simultaneously.
Then both succeed with distinct document numbers and no unique-constraint
errors (advisory-lock serialization).

### UC-INV-16 — Fingerprint determinism

Given the same instruction serialized twice.
Then `request_fingerprint` is the same 64-hex SHA-256; changing any field
(customer, amount, tax) changes it. (Also covers UC-INV-03's mechanism.)

## Test coverage matrix

| Scenario | Existing test | Status |
|---|---|---|
| UC-INV-01 | `core/src/lib.rs::posts_taxed_discounted_invoice_and_balanced_journal` | implemented |
| UC-INV-02/03 | `posting.rs::live_post_retry_conflict_retrieve_and_credit` (`#[ignore]`) | implemented, needs PostgreSQL |
| UC-INV-04 | `core/src/lib.rs::rounds_half_away_from_zero_at_line_level` | implemented |
| UC-INV-05 | `posting.rs` live test (concurrent numbering) | partial |
| UC-INV-06 | none | not yet implemented |
| UC-INV-07 | `core/src/lib.rs::rejects_closed_period`, `rejects_date_outside_period` | implemented |
| UC-INV-08 | `core/src/lib.rs::rejects_zero_quantity`, `rejects_discount_above_one_hundred_percent`, `currency_and_idempotency_types_reject_ambiguous_values` | partial |
| UC-INV-09 | `core/src/lib.rs::full_credit_note_reverses_original_journal`, `trial_balance_nets_invoice_and_full_credit_to_zero` | implemented |
| UC-INV-10 | none | not yet implemented |
| UC-INV-11 | `posting.rs::public_invoice_json_does_not_expose_tenant_or_gl_mapping` | implemented |
| UC-INV-12 | `core/src/lib.rs` journal validation | partial |
| UC-INV-13 | `pdf.rs::builds_deterministic_validly_framed_pdf`, `object_store.rs` (vector + path safety), `document.rs::object_key_contains_no_tenant_or_customer_data` | implemented |
| UC-INV-14 | none | not yet implemented |
| UC-INV-15 | `posting.rs` live test (2-thread) | implemented, `#[ignore]` |
| UC-INV-16 | `core/src/lib.rs::request_fingerprint_is_deterministic_and_payload_sensitive` | implemented |

## Known drift / gaps

- `models/invoice.rs` (`invoices`/`invoice_lines`, NUMERIC(19,4), draft
  statuses) is legacy and unused by the posting path — scenarios target the
  foundation-entity flow only.
- Foreign-currency invoicing is deliberately out of Phase 1 (policy error);
  IAS 21 scenario (UC-CS-01 FX) is future work.
- UC-INV-14 (503) and the full UC-INV-02/03 live flow need an integration
  harness; today they run only in the `#[ignore]`d live test.
