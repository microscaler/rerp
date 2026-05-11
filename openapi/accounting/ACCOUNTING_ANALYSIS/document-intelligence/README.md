# Document Intelligence (OCR)

> **Component:** Accounting document ingestion — OCR, classification, extraction, review, approval, and linkage to invoices, bills, and bank statements
> **Priority:** P2 — Document processing is the #1 friction point in accounting data entry
> **Odoo Reference:** account.document module (document storage), third-party OCR integrations, account.invoice (manual data entry)

---

## The Pitch

**Buyer Question:** *Can I capture invoices, bills, bank statements, and receipts from email, file upload, or scanner, and extract the key fields automatically for review and posting?*\

Manual data entry of invoices and bills is the single biggest time sink in accounting. A typical AP clerk spends 40% of their time manually entering invoice data. Document intelligence automates this: upload or forward a PDF, and the system extracts vendor name, invoice number, date, line items, tax, and total — all with a confidence score. The human reviews and approves; the machine does the rest. This component handles the full document lifecycle: ingestion, OCR extraction, document classification, data review, approval, and linkage to accounting records.

---

## What This Component Does

Document Intelligence is the bridge between unstructured documents and structured accounting data. It handles:

1. **Document Ingestion** — Accept documents from email, file upload, scanner, API, or webhook
2. **OCR Extraction** — Extract text, vendor info, invoice number, date, line items, tax, total
3. **Document Classification** — Auto-classify documents (invoice, bill, receipt, bank statement, contract)
4. **Confidence Scoring** — Score extraction confidence; route low-confidence items to manual review
5. **Human-in-the-Loop Review** — Interface for humans to correct extracted data
6. **Linkage** — Auto-create vendor bills or customer invoices from extracted data
7. **Document Storage** — Secure storage of original documents with metadata

---

## Entity Model

### Document Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Document name/reference |
| `document_type` | Enum: [INVOICE, BILL, RECEIPT, BANK_STATEMENT, CONTRACT, OTHER] | Yes | Document classification |
| `source` | Enum: [EMAIL, UPLOAD, SCAN, API, WEBHOOK] | Yes | Document source |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `file_url` | String (512) | Yes | URL to stored document |
| `file_hash` | String (64) | Yes | SHA-256 hash for dedup |
| `mime_type` | String (64) | Yes | File MIME type |
| `page_count` | Integer | No | Number of pages |
| `page_size` | String (32) | No | Dimensions (A4, Letter, etc.) |
| `extracted_text` | Text | No | Full OCR text |
| `extracted_vendor_name` | String (255) | No | Extracted vendor |
| `extracted_invoice_number` | String (128) | No | Extracted invoice number |
| `extracted_invoice_date` | Date | No | Extracted invoice date |
| `extracted_amount_total` | Decimal (15,2) | No | Extracted total |
| `extracted_tax_amount` | Decimal (15,2) | No | Extracted tax |
| `extracted_lines` | JSON | No | Extracted line items |
| `confidence_score` | Float (0-1) | Computed | Overall extraction confidence |
| `review_status` | Enum: [PENDING_REVIEW, APPROVED, REJECTED, IN_PROGRESS] | Yes | Human review status |
| `reviewed_by` | Foreign Key: User | No | Reviewer |
| `reviewed_at` | DateTime | No | Review timestamp |
| `accountable_id` | String (128) | No | Linked entity ID |
| `accountable_type` | String (64) | No | Linked entity type |
| `duplicate_detected` | Boolean | Computed | Is this a duplicate? |
| `create_uid` | Foreign Key: User | Computed | Creator |
| `create_date` | DateTime | Computed | Created timestamp |

**Total fields: ~26.**

### Document Line Entity

Individual extracted line items:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: Document | Yes | Parent document |
| `sequence` | Integer | Yes | Line display order |
| `description` | String (255) | No | Line description |
| `quantity` | Float | No | Quantity |
| `unit_price` | Decimal (15,2) | No | Unit price |
| `amount` | Decimal (15,2) | No | Line amount |
| `tax_amount` | Decimal (15,2) | No | Tax on line |
| `account_id` | Foreign Key: Account | No | Proposed account |
| `confidence_score` | Float (0-1) | Computed | Line-level confidence |
| `product_id` | Foreign Key: Product | No | Matched product |

**Total fields: ~11.**

### OCR Processing Job Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: Document | Yes | Document being processed |
| `provider` | Enum: [Tesseract, AWS_Textract, Google_Vision, Azure_Form_Recognizer, Custom] | Yes | OCR provider |
| `status` | Enum: [QUEUED, PROCESSING, COMPLETED, FAILED] | Yes | Processing status |
| `duration_ms` | Integer | No | Processing time in ms |
| `error_message` | Text | No | Error details |
| `started_at` | DateTime | No | Processing start |
| `completed_at` | DateTime | No | Processing end |

**Total fields: ~8.**

### Document Classifier Model Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Classifier name |
| `type` | Enum: [KEYWORD, ML, HYBRID] | Yes | Classification method |
| `patterns` | JSON | No | Classification patterns/rules |
| `accuracy` | Float (0-1) | Computed | Classifier accuracy |
| `active` | Boolean | Yes | Active? |

**Total fields: ~6.**

---

## Entity Relationships

```
account.document (documents)
  ├── account.document.line (line_ids)    ← Extracted line items
  ├── account.ocr.job (processing_jobs)   ← OCR processing jobs
  ├── account.bill (linked bills)         ← Linked vendor bills
  ├── account.invoice (linked invoices)   ← Linked customer invoices
  └── account.bank.statement.line (linked)← Linked bank statement lines

account.document
  └── res.partner (via extracted_vendor_name) ← Vendor matching
```

---

## Required API Endpoints

### Document Ingestion

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/documents/upload` | Upload document file |
| `POST` | `/documents/from-email` | Ingest document from email |
| `POST` | `/documents/webhook` | Receive document via webhook |
| `GET` | `/documents` | List documents with filters |
| `GET` | `/documents/{id}` | Get document detail |
| `GET` | `/documents/{id}/extract` | Get extracted data |

### OCR Processing

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/documents/{id}/ocr` | Trigger OCR processing |
| `GET` | `/documents/{id}/ocr-status` | Check processing status |
| `POST` | `/documents/batch-ocr` | Trigger OCR for multiple documents |
| `POST` | `/documents/{id}/rerun-ocr` | Re-run OCR with different provider |

### Document Classification

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/documents/classify` | Classify document type |
| `POST` | `/documents/classify-batch` | Classify multiple documents |
| `GET` | `/documents/{id}/classification` | Get classification result |

### Human Review

| Method | Endpoint | Description |
|--------|----------|-------------|
| `PATCH` | `/documents/{id}/review` | Submit human review with corrections |
| `POST` | `/documents/{id}/approve` | Approve extracted data |
| `POST` | `/documents/{id}/reject` | Reject and return for re-extraction |

### Linkage

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/documents/{id}/create-bill` | Create vendor bill from extracted data |
| `POST` | `/documents/{id}/create-invoice` | Create customer invoice from extracted data |
| `POST` | `/documents/{id}/link-bank` | Link to bank statement line |
| `GET` | `/documents/{id}/linked-entities` | View linked accounting entities |

### Deduplication

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/documents/detect-duplicates` | Detect duplicate documents |
| `POST` | `/documents/{id}/mark-duplicate` | Mark as duplicate |

---

## Competitive Intelligence

**Odoo:** Enterprise document module with file upload and storage. Basic OCR via third-party integrations. No native intelligent extraction. Manual data entry from documents remains the norm.

**QuickBooks Online:** Receipt capture via mobile app with basic OCR. Auto-extract vendor, date, amount. Link to expenses. Limited invoice scanning.

**NetSuite:** Bill Capture with AI-driven invoice data extraction. Supports 100+ form layouts. Vendor portal for self-service invoice submission. Auto-creation of AP bills from extracted data.

**SAP S/4HANA:** SAP Document Management (DMS) with intelligent scanning. SAP Readycrop auto-classifies documents. AI-powered extraction via SAP AI Core. Integration with SAP Business Network for EDI invoices.

**Sage Intacct:** Document attachment to transactions. Basic file storage. No native OCR. Third-party integrations (AvidXchange, HighRadius) add intelligent document processing.

**Xero:** Photo invoices — mobile app captures receipts with basic OCR. Manual data entry still required for most invoices.

**Zoho Books:** Zoho OCR for invoice scanning. Extracts vendor, date, amount. Create bills from extracted data. Good integration with Zoho ecosystem.

---

## Implementation Roadmap

### Phase 1: Document Ingestion & Storage (2 weeks) — P2
1. Define `Document` entity with metadata and file storage
2. Implement document upload, email ingestion, and webhook endpoints
3. Implement SHA-256 deduplication on document hash
4. Store original documents with metadata

### Phase 2: OCR Pipeline (3 weeks) — P2
1. Define `OCRJob` entity for processing tracking
2. Implement OCR provider abstraction (Tesseract, AWS Textract, Google Vision)
3. Implement text extraction from PDFs and images
4. Implement field extraction (vendor, invoice number, date, total, lines)

### Phase 3: Classification & Review (2 weeks) — P2
1. Define `DocumentClassifier` entity
2. Implement document type classification
3. Implement human review interface (API) with correction support
4. Link approved documents to vendor bills or customer invoices

---

## Competitive Positioning

### Where RERP Wins
- **API-first document processing** — Documents flow via API, not email or GUI. Structured data from day one.
- **Multi-provider OCR** — Switch between OCR providers (Tesseract, AWS Textract, Google Vision) via configuration. No vendor lock-in.
- **Self-hosted, no per-document fees** — No OCR API subscription costs. Unlimited documents.

### Where RERP Lags
- **No document entities defined** — Zero fields for document management.
- **No OCR pipeline** — No text extraction or field parsing.
- **No human review workflow** — No interface for manual correction.

---

## Key Takeaway for Buyers

Document intelligence eliminates the most expensive and error-prone step in accounting: manual data entry. A buyer should ask: *Can I forward invoices to my system and have them auto-populated for review and posting?* RERP's API-first model means documents arrive as structured API calls, not unstructured email attachments. The gap with SAP/NetSuite is the depth of AI/ML (form layout adaptation, vendor-specific parsers). But for organizations that want document processing with full API control and no per-document fees, RERP provides the foundation.

**The immediate priority: define Document entity with file storage and SHA-256 deduplication. This is the input layer for everything else.**
