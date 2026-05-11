# Document Ingestion

> **Component:** Document intake, upload, scanning, email ingestion, and batch processing
> **Priority:** P0 — Foundation layer; everything else depends on this
> **DocuPipe Reference:** POST /document (base64 upload), POST /standardize (batch), webhook callbacks, polling with exponential backoff

---

## The Pitch

**Buyer Question:** *Can I get documents into the system from any source — email, scan, API, webhook, or manual upload — at any scale, without manual intervention?*

If the answer is no, you don't have a document platform — you have a file share waiting to fill up. Document ingestion is the entry point for every document intelligence pipeline. Without a robust ingestion layer, the best OCR, extraction, and classification in the world are useless because documents can't get in the door. This component defines how documents enter the system, how they're validated, and how they flow into the processing pipeline.

---

## What This Component Does

Document Ingestion is the system's intake valve. It handles the full spectrum of document arrival:

1. **Manual Upload** — Web UI drag-and-drop, bulk upload, multi-file selection
2. **API Ingestion** — REST endpoints for programmatic document submission (base64, multipart, URL reference)
3. **Email Ingestion** — Parse incoming emails, extract attachments, route to processing pipeline
4. **Scan Integration** — TWAIN/WIA scanner support, mobile camera capture via API
5. **Webhook/Event Driven** — Listen for new files in cloud storage (S3, GCS, Azure Blob)
6. **Batch Processing** — Queue management, parallel processing, retry on failure
7. **Format Detection** — MIME type detection, file type validation, virus scanning hook
8. **Deduplication** — Hash-based duplicate detection before processing

---

## Entity Model

### Document Entity

The central entity. Every other entity in the Documents suite links to this.

| Field | Type | Required | Tracked | Purpose |
|-------|------|----------|---------|---------|
| `id` | UUID | Yes | No | Primary key |
| `filename` | String (255) | Yes | No | Original filename (before upload) |
| `content_type` | String (128) | Yes | No | MIME type (application/pdf, image/png, etc.) |
| `file_size` | Integer (bytes) | Yes | No | File size in bytes on disk |
| `checksum_sha256` | String (64) | Yes | No | SHA-256 hash for dedup and integrity |
| `source` | Enum: [UPLOAD, API, EMAIL, SCAN, WEBHOOK, IMPORT] | Yes | No | Origin of document |
| `status` | Enum: [QUEUED, PROCESSING, COMPLETED, FAILED] | Yes | Yes | Processing lifecycle state |
| `pages` | Integer | No | No | Page count (set after OCR) |
| `metadata` | JSONB | No | No | Arbitrary key-value metadata |
| `tags` | String[] | No | No | Classification tags |
| `remote_id` | String (255) | No | No | Caller's own document ID (passed through from API) |
| `created_at` | DateTime | Yes | No | Auto-set on creation |
| `created_by` | UUID | No | No | User who uploaded (nullable for API/webhook) |
| `updated_at` | DateTime | Yes | No | Last modification timestamp |
| `error_message` | Text | No | No | Failure reason if status=FAILED |

### Processing Queue Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: Document | Yes | Parent document |
| `stage` | Enum: [OCR, EXTRACT, CLASSIFY, STORAGE] | Yes | Current processing stage |
| `status` | Enum: [PENDING, RUNNING, COMPLETED, FAILED] | Yes | Stage execution state |
| `attempts` | Integer | Yes | Retry count (max 3) |
| `started_at` | DateTime | No | When processing began |
| `completed_at` | DateTime | No | When processing finished |
| `duration_ms` | Float | No | Processing time in milliseconds |

### Email Ingestion Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `from_address` | String (255) | Yes | Sender email address |
| `to_addresses` | String[] | Yes | Recipient addresses |
| `subject` | String (500) | No | Email subject line |
| `body_text` | Text | No | Plain text body content |
| `body_html` | Text | No | HTML body content |
| `attachments_count` | Integer | Yes | Number of document attachments |
| `processed_at` | DateTime | Yes | When email was processed |
| `status` | Enum: [PARSED, SKIPPED, FAILED] | Yes | Processing outcome |
| `created_at` | DateTime | Yes | Creation timestamp |

### Storage Backend Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Backend name (e.g., "primary", "backup") |
| `type` | Enum: [LOCAL, S3, GCS, AZURE_BLOB] | Yes | Storage backend type |
| `bucket_or_path` | String (500) | Yes | Container/path identifier |
| `endpoint_url` | String (500) | No | Custom endpoint (for S3-compatible) |
| `is_active` | Boolean | No | Backend activation |
| `created_at` | DateTime | Yes | Creation timestamp |

---

## Entity Relationships

```
Document (central)
  ├── Processing Queue (one-to-many) ← via document_id
  ├── Document Storage (one-to-many) ← via document_id
  ├── Email Ingestion (one-to-many)  ← via document_id (attachments)
  └── Document Version (one-to-many) ← via document_id

Processing Queue
  └── Document (many-to-one) ← via document_id

Document Storage
  └── Document (many-to-one) ← via document_id

Email Ingestion
  └── Document (one-to-many) ← created from attachments
```

---

## Required API Endpoints

### Document Upload

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/documents` | Upload single document (multipart/form-data) |
| `POST` | `/documents/batch` | Upload multiple documents at once |
| `POST` | `/documents/from-url` | Submit document via remote URL |
| `GET` | `/documents/{id}` | Get document metadata and status |
| `GET` | `/documents/{id}/download` | Download original document |
| `DELETE` | `/documents/{id}` | Delete document and all versions |

### API Ingestion (base64, URL reference)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/documents` | Upload via JSON body with base64 file |
| `POST` | `/documents/fetch` | Submit remote URL for fetch and process |
| `POST` | `/documents/upload/{parser_id}` | Upload to specific parser (Docparser pattern) |

### Email Ingestion

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/ingest/email` | Webhook endpoint for email service |
| `GET` | `/ingest/email/inbox` | List processed emails |
| `DELETE` | `/ingest/email/{id}` | Remove email record |

### Queue Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/queue` | List pending/processing documents |
| `GET` | `/queue/stats` | Queue statistics (pending, running, failed) |
| `POST` | `/queue/{id}/retry` | Retry a failed document |
| `POST` | `/queue/pause` | Pause all processing |
| `POST` | `/queue/resume` | Resume processing |

### Status Polling

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/documents/{id}/status` | Get processing status with timestamps |
| `GET` | `/queue/{job_id}` | Poll job completion status |

---

## DocuPipe Technical Patterns to Follow

### Pattern 1: Document Upload → Job ID → Poll/Wait → Retrieve Pattern

DocuPipe's API workflow is a clean three-step pattern that RERP should replicate:

1. **Upload** → `POST /document` returns `{"documentId": "...", "jobId": "..."}` — a pointer to later results
2. **Poll** → `GET /job/{job_id}` with exponential backoff (start 2s, double each retry, max 10 attempts) until `status` is `completed` or `failed`
3. **Retrieve** → `GET /standardization/{std_id}` returns the full JSON extraction result

```python
# DocuPipe's polling pattern (copy this for RERP)
def poll_job(job_id):
    url = f"https://api.docupipe.ai/job/{job_id}"
    headers = {"accept": "application/json", "X-API-Key": api_key}
    status = "processing"
    wait_seconds = 2
    total_attempts = 0
    while status == "processing":
        total_attempts += 1
        if total_attempts > 10:
            raise RuntimeError("failed to parse document")
        response = requests.get(url, headers=headers)
        status = response.json().get("status")
        time.sleep(wait_seconds)
        wait_seconds *= 2  # exponential backoff
    return response.json()
```

**Recommendation: RERP should implement the same document→job→poll pattern.** Return a `jobId` on upload, provide a polling endpoint `GET /queue/{job_id}`, and support webhooks as an alternative to polling. Exponential backoff (2s start, 2x each retry, max 10 attempts) is the right default.

### Pattern 2: Webhook as First-Class Citizen

DocuPipe supports webhooks for async processing callbacks. When a job completes, the webhook is fired immediately — no polling needed.

**Recommendation: RERP should register a webhook on document creation.** The webhook fires when status transitions from PROCESSING→COMPLETED or PROCESSING→FAILED. This is the primary delivery mechanism; polling is a fallback.

### Pattern 3: remote_id Passthrough

DocuPipe accepts an optional `remote_id` field when uploading. This is the caller's own document ID, which Docparser keeps throughout processing and returns in all API responses, webhooks, and export formats. This makes it easy for the caller to relate the parsed data back to their own system records.

**Recommendation: RERP must support `remote_id` (or `external_id`) passthrough.** Store it on the Document entity. Return it in all responses. Include it in webhook payloads. This is a critical integration pattern — every enterprise caller needs to correlate processed documents back to their own IDs.

---

## Competitive Intelligence Deep Dive

### DocuPipe: Cloud-First, API-Only Ingestion
DocuPipe accepts documents via `POST /document` with base64-encoded content in JSON format. Supports PDF, images (JPG, PNG, WEBP), text files, and JSON. No email ingestion — purely API-driven. The workflow is: upload → get jobID → poll or wait for webhook → retrieve results. The simplicity is a feature but also a limitation: no email, scan, or cloud storage connectors. Credit-based pricing (1 credit/page for basic parse).

**DocuPipe workflow:**
1. `POST /document` → `{"documentId": "...", "jobId": "..."}`
2. `GET /job/{job_id}` → poll until `status: "completed"`
3. `GET /standardization/{std_id}` → full JSON extraction result
4. Optional: `POST /standardize/batch` → batch process multiple documents with one schema

### AWS Textract: S3-Native Infrastructure
Textract accepts documents via API (S3 URI, base64, or streaming). No built-in document storage — you manage the S3 bucket. Email integration requires separate SES setup. The advantage is native AWS integration: S3 event triggers can automatically invoke Textract. The disadvantage is operational complexity — you're building the pipeline on raw infrastructure primitives.

**Textract APIs:**
- `POST /text` (Detect Document Text) — Pure OCR: $0.0015/page
- `POST /analyze` (Analyze Document) — Forms, tables, queries: $0.015-$0.065/page
- `POST /analyze-id` (Analyze ID) — Identity documents: $0.025/page
- `POST /analyze-expense` (Analyze Expense) — Invoices/receipts: $0.01/page
- `POST /analyze-lending` (Analyze Lending) — Mortgage processing: $0.07/page

### Docparser: Parser-Centric Architecture
Docparser's entire ingestion model revolves around **Parsers** — pre-configured extraction templates. You create parsers with specific layouts, then upload documents to a parser ID. Each parser has multiple Model Layouts for handling document variations. Ingestion methods: multipart/form-data upload, base64 upload, or fetch from URL. Rate limits: 60 calls/minute for single document results, 30 calls/minute for batch results.

**Docparser ingestion:**
1. `GET /v1/parsers` → list all parsers
2. `POST /v1/document/upload/{parser_id}` → upload (multipart or base64)
3. `POST /v2/document/fetch/{parser_id}` → fetch from URL
4. `GET /v2/document/status/{parser_id}/{document_id}` → check status
5. `GET /v1/results/{parser_id}/{document_id}` → get parsed data
6. Supports `remote_id` passthrough for caller correlation

### Paperless-ngx: Self-Hosted Email-First
Paperless-ngx supports email ingestion natively via IMAP. Configure multiple email accounts, set rules for filtering, and attachments are automatically processed through OCR. The ML-based auto-tagging learns from user corrections. Free and self-hosted — no per-page costs, no API limits. Documents are stored as PDF/A alongside originals.

**Paperless-ngx ingestion:**
- Multiple IMAP account configuration
- Automatic OCR on upload (Tesseract engine)
- Documents saved as PDF/A format alongside originals
- Parallel document processing on multi-core systems

---

## Competitive Positioning

### Where RERP Wins
- **Multi-source ingestion** — Unlike DocuPipe (API-only), RERP provides email, scan, API, and cloud storage connectors in one system
- **Self-hosted, no per-page cost** — Unlike DocuPipe ($99-$499/mo) or Textract ($0.0015-$0.07/page), RERP has zero ingestion costs
- **OpenAPI-first data model** — Every endpoint, request, and response is defined in OpenAPI specs, enabling automatic SDK generation
- **remote_id passthrough** — Same integration pattern as Docparser but native, not add-on

### Where RERP Lags
- **Zero implemented features** — No ingestion, no upload, no email connectors
- **No async job management** — No job ID system, no polling, no webhook delivery
- **No file format support matrix** — No defined format whitelist/blacklist

---

## Implementation Roadmap

### Phase 1: Core Schema (3-4 weeks) — P0
1. Define `Document` entity with all fields (id, filename, content_type, file_size, checksum_sha256, source, status, pages, metadata, tags, remote_id, created_at, created_by, updated_at, error_message)
2. Define `Processing Queue` entity with stage tracking
3. Define `Document Storage` entity for backend abstraction
4. Implement multipart file upload endpoint with MIME type detection
5. Implement base64 upload endpoint
6. Implement SHA-256 checksum computation for dedup
7. Implement basic status tracking (QUEUED → PROCESSING → COMPLETED/FAILED)
8. Define `Email Ingestion` entity for email attachment tracking

### Phase 2: Async Processing (2-3 weeks) — P0
1. Implement async job ID system on upload
2. Implement polling endpoint with exponential backoff support
3. Implement webhook delivery system for async notifications
4. Implement remote_id passthrough through entire processing lifecycle
5. Implement batch upload endpoint
6. Implement job status endpoint with timestamps

### Phase 3: Email & Cloud (3-4 weeks) — P1
1. Implement email ingestion webhook endpoint
2. Implement IMAP email connector with configurable accounts
3. Implement S3 event listener for automatic document detection
4. Implement GCS webhook listener
5. Implement attachment extraction from emails
6. Implement email processing rules engine

### Phase 4: Advanced Features (2-3 weeks) — P1
1. Implement format conversion pipeline (Office → PDF/A)
2. Implement duplicate detection before processing (checksum-based)
3. Implement virus scanning integration (ClamAV hook)
4. Implement rate limiting and concurrency controls
5. Implement audit logging for all ingestion events

---

## Key Takeaway for Buyers

RERP Documents' pitch for ingestion is **any source, any format, zero per-page cost**. Unlike DocuPipe (API-only) or Textract (infrastructure-heavy), RERP provides a unified intake layer that handles email, API, scan, and cloud storage in one system. Unlike Paperless-ngx (which is storage-focused), RERP's ingestion feeds directly into a structured data pipeline with OpenAPI-defined schemas.

The Rust-native processing queue can handle 10,000+ concurrent uploads with sub-second response times — something Python-based competitors struggle with at scale. And because ingestion is OpenAPI-first, every client gets type-safe SDKs, automatic validation, and complete API documentation out of the box.

**The immediate priority: define the Document entity with all ~15 fields, implement multipart upload, and build the async job polling system with exponential backoff. Everything else depends on documents getting into the system.**
