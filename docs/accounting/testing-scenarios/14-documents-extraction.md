# Documents Extraction — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/documents-extraction
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for accounting document intake, classification,
extraction jobs/results, approval with corrections, and linking extracted
documents to invoices and bank statements.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../rules-engines/extraction-classification-engine.md`](../rules-engines/extraction-classification-engine.md)
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

No direct IFRS governs document extraction; it produces the source evidence
that recognition standards require.

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IAS 8 — Accounting Policies, Changes in Estimates and Errors | Complete, accurate source data for policies | <https://www.ifrs.org/issued-standards/list-of-standards/ias-8-accounting-policies-changes-in-accounting-estimates-and-errors/> |
| IFRS 15 — Revenue from Contracts with Customers | Contracts/invoices as revenue evidence | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-15-revenue-from-contracts-with-customers/> |

## Endpoint inventory (9 operations)

Accounting documents (list/create/classify/approve-extraction); extraction
jobs (list/create); extraction results (list); link-invoice;
link-bank-statement.

Document status: UPLOADED → CLASSIFIED → EXTRACTING → REVIEW_REQUIRED →
APPROVED → LINKED (also REJECTED/ARCHIVED). Job status: PENDING, RUNNING,
COMPLETED, FAILED, CANCELLED.

## Use-case scenarios

### UC-DOC-01 — Upload a document

Given a scanned vendor bill `inv-2026-014.pdf`.
When `POST /accounting-documents {file_name: "inv-2026-014.pdf", source_uri: "s3://...", company_id, document_type: VENDOR_BILL, content_type: "application/pdf"}`.
Then created UPLOADED with `uploaded_at` set; a missing file_name or
source_uri is rejected.

### UC-DOC-02 — Classify with confidence

Given the document above.
When `POST /accounting-documents/{id}/classify`.
Then the classifier returns `document_type: VENDOR_BILL` with a confidence
in 0..1 and `reasons[]` explaining the decision; confidence below the review
threshold routes the document to REVIEW_REQUIRED instead of CLASSIFIED;
`force_reclassify=true` re-runs classification on an already-classified
document.

### UC-DOC-03 — Extraction job lifecycle

Given a CLASSIFIED document.
When `POST /extraction-jobs {document_id}`.
Then 202 with job PENDING → RUNNING → COMPLETED; a job for a document in
UPLOADED state is rejected; a FAILED job records the failure reason and can
be retried as a new job.

### UC-DOC-04 — Extraction results with confidence

Given a COMPLETED job on the vendor bill.
Then `extraction-results` lists fields (vendor name, invoice number, date,
total 1,200.00, tax 200.00) each with its confidence; low-confidence fields
are flagged for review.

### UC-DOC-05 — Approve extraction with corrections

Given a REVIEW_REQUIRED extraction where OCR read the total as 1,800.00
but the reviewer corrects it to 1,200.00.
When `POST /documents/{id}/approve-extraction {approved_by, corrections: {"total": "1200.00"}}`.
Then status APPROVED with the corrected fields stored and the correction
recorded (who/when) — the original OCR output remains auditable alongside.

### UC-DOC-06 — Link to invoice

Given an APPROVED vendor bill.
When `POST /documents/{id}/link-invoice {target_id: <ap-invoice>, link_reason: "source evidence"}`.
Then a DocumentLink is created (target_type invoice) and the document
becomes LINKED; linking an unapproved document is rejected; linking twice
to the same target is rejected.

### UC-DOC-07 — Link to bank statement

Given an APPROVED bank statement document.
When `POST /documents/{id}/link-bank-statement {target_id: <statement>, link_reason}`.
Then the statement gains its source-document link (audit trail for
`bank-sync`).

### UC-DOC-08 — Status transitions

Given the states above.
Then UPLOADED → CLASSIFIED/EXTRACTING → REVIEW_REQUIRED → APPROVED →
LINKED, with REJECTED/ARCHIVED as terminals; every transition is guarded
(e.g. approving a non-reviewed extraction, or classifying an ARCHIVED
document, is rejected).

### UC-DOC-09 — Duplicate detection

Given two uploads of the same invoice file (same content hash).
Then the second upload is flagged as a likely duplicate with the first
document's id — never silently processed twice.

### UC-DOC-10 — Boundary validation

Given page 0, limit 101, an unknown document_type, a blank file_name.
Then each is rejected 400; defaults page=1 limit=20.

### UC-DOC-11 — Tenant isolation

Given documents of tenant A and tenant B with the same file_name.
Then tenant B sees only its own documents/jobs/results; cross-tenant
document id → 404.

## Test coverage matrix

All scenarios: **not yet implemented** — contract-only service; all
controllers are stubs (`classify` returns `confidence: 0.0` and an empty
document_type), no services/models, no tests, no migrations.

| Scenario | Existing test | Status |
|---|---|---|
| UC-DOC-01..11 | none | not yet implemented |

## Known drift / gaps

- Contract-only; the approval response schema has **no `corrections` field**
  (corrections only in the request) — spec asymmetry to fix.
- Confidence thresholds, correction versioning, duplicate detection, and
  source-file integrity are open questions in the engine dossier; these
  scenarios fix them as acceptance targets.
- The extraction engine itself (OCR/ML) is external; this service owns the
  orchestration, review, and linkage states only.
