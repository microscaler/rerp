# EDI — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/edi
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for EDI document exchange: documents
(ingest/parse/validate/process), mappings, profiles, validation profiles,
submissions (submit/status/retry), acknowledgments, and errors.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

EDI itself is a transport/format layer; it carries the documentation that
accounting standards and jurisdiction e-invoicing law require.

### IFRS (international)

| Authority | Relevance | URL |
|---|---|---|
| IAS 1 / IFRS 15 documentation requirements | EDI payloads evidence posted transactions | <https://www.ifrs.org/issued-standards/list-of-standards/ias-1-presentation-of-financial-statements/> |
| Jurisdiction e-invoicing mandates (Indonesia, Malaysia, Singapore, EU PEPPOL) | Statutory exchange formats and mandates | jurisdiction portals |

## Endpoint inventory (21 operations)

EDI documents CRUD; mappings CRUD; profiles (list/create); validation
profiles (list/create); submissions (list/create/submit/status/retry);
acknowledgments (list); errors (list).

## Use-case scenarios

### UC-EDI-01 — Ingest an EDI document

Given a PEPPOL invoice XML from partner P-9.
When `POST /edi-documents {document_number: "ED-0001", document_type: INVOICE, format_id, sender_id: P-9, document_total_amount: "1200.00", currency_code: USD, raw_content: <xml>}`.
Then created RECEIVED with received_at set; a duplicate `document_number`
is rejected.

### UC-EDI-02 — Parse with a mapping

Given document ED-0001 and a mapping for (format, INVOICE) with field
mappings.
When the document is parsed.
Then status RECEIVED → PARSING → PARSED, `parsed_data` carries the mapped
fields (partner, amount, tax), and parse failures land in ERROR with
`error_message` + `error_details`.

### UC-EDI-03 — Validation profile

Given a validation profile for PEPPOL invoices requiring a positive total
and a valid VAT id.
When the document is validated.
Then status VALIDATED with no errors; a negative total or malformed VAT id
yields ERROR with `field_path` per issue.

### UC-EDI-04 — Process into accounting

Given a VALIDATED invoice document.
When processed.
Then status PROCESSED with `related_invoice_id` pointing at the created
AP/AR invoice (cross-service handoff); processing a non-VALIDATED document
is rejected.

### UC-EDI-05 — Mapping management

Given mappings per (format, document_type).
Then a new document uses the active mapping for its format+type; changing a
mapping affects only future documents (processed documents keep their
parsed_data snapshot); two mappings for the same (format, type) where both
are active is rejected.

### UC-EDI-06 — Submission lifecycle

Given a PROCESSED invoice document and an outbound PEPPOL profile.
When `POST /edi-submissions {document_id, profile_id}`.
Then DRAFT → (submit) 202 → VALIDATING → READY → SUBMITTED → ACCEPTED;
`GET /edi-submissions/{id}/status` polls with `external_reference` and
`submitted_at` once accepted; a REJECTED submission exposes the authority
reason and can be retried (`retry` → 202, retry counter increments).

### UC-EDI-07 — Retry limits

Given a submission that keeps failing validation.
When retried repeatedly.
Then `retry_count` increments with `last_retry_at`; after the configured
maximum the status becomes CANCELLED/REJECTED with the terminal reason
(no unbounded retry loops).

### UC-EDI-08 — Acknowledgments

Given a submitted invoice document.
When the partner returns a functional acknowledgment.
Then an acknowledgment row (type FUNCTIONAL, status ACCEPTED) links to the
document; a REJECTED acknowledgment marks the document ERROR and blocks
PROCESSED until corrected; RECEIPT and TECHNICAL types are tracked
separately.

### UC-EDI-09 — Error reporting

Given parse/validation/submission failures.
When `GET /edi-errors` is called.
Then each error lists `submission_id`, `code`, `message`, and `field_path`
for structured diagnosis.

### UC-EDI-10 — Acknowledgment flag

Given a RECEIVED document.
Then `acknowledgment_sent` stays false until the outbound acknowledgment is
sent (`acknowledgment_sent_at` set); re-sending is idempotent.

### UC-EDI-11 — Boundary validation

Given page 0, limit 101, an unknown document_type/standard, a blank
document_number.
Then each is rejected 400; defaults page=1 limit=20.

### UC-EDI-12 — Tenant isolation

Given documents of tenant A and tenant B with the same document_number.
Then tenant B sees only its own documents/mappings/submissions; cross-tenant
document id → 404.

## Test coverage matrix

| Scenario | Existing test | Status |
|---|---|---|
| UC-EDI-01..12 | none | not yet implemented |

Model/migration coverage exists for `edi_documents`, `edi_formats`,
`edi_mappings`, `edi_acknowledgments` (unique keys, RESTRICT/CASCADE FKs),
but **all 22 controllers are stubs** and no `#[cfg(test)]` modules exist.

## Known drift / gaps

- All controllers are stubs (`// TODO: Implement your business logic
  here`); delete handlers return 204 unconditionally.
- OpenAPI schemas for `EdiProfile`, `EdiValidationProfile`, `EdiSubmission`,
  `EdiError` have **no model/migration** — the contract outpaces the
  persistence layer.
- Document exchange state machine, retry limits, and acknowledgment logic
  are all absent; the scenarios above are the acceptance target.
