# Document Ingestion

> **Component:** Document intake, upload, scanning, email ingestion, and batch processing
> **Priority:** P0 — Foundation layer; everything else depends on this

---

## The Pitch

**Buyer Question:** *Can I get documents into the system from any source — email, scan, API, webhook, or manual upload — at any scale, without manual intervention?*

If the answer is no, you don't have a document platform — you have a file share waiting to fill up. Document ingestion is the entry point for every document intelligence pipeline. Without a robust ingestion layer, the best OCR, extraction, and classification in the world are useless because documents can't get in the door.

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

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `filename` | String (255) | Yes | Original filename |
| `content_type` | String (128) | Yes | MIME type (application/pdf, image/png, etc.) |
| `file_size` | Integer (bytes) | Yes | File size in bytes |
| `checksum_sha256` | String (64) | Yes | SHA-256 hash for dedup and integrity |
| `status` | Enum: [QUEUED, PROCESSING, COMPLETED, FAILED] | Yes | Processing lifecycle state |
| `source` | Enum: [UPLOAD, API, EMAIL, SCAN, WEBHOOK, IMPORT] | Yes | Where the document came from |
| `metadata` | JSONB | No | Arbitrary key-value metadata |
| `tags` | String[] | No | Classification tags |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `created_by` | UUID | No | User who uploaded |
| `updated_at` | DateTime | Yes | Last modification |
| `error_message` | Text | No | Failure reason if status=FAILED |

### Processing Queue Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | FK: Document | Yes | Parent document |
| `stage` | Enum: [OCR, EXTRACT, CLASSIFY, STORAGE] | Yes | Current processing stage |
| `status` | Enum: [PENDING, RUNNING, COMPLETED, FAILED] | Yes | Stage execution state |
| `attempts` | Integer | Yes | Retry count (max 3) |
| `started_at` | DateTime | No | When processing began |
| `completed_at` | DateTime | No | When processing finished |
| `duration_ms` | Integer | No | Processing time in milliseconds |

### Email Ingestion Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `from_address` | String (255) | Yes | Sender email |
| `subject` | String (500) | No | Email subject line |
| `body_text` | Text | No | Plain text body |
| `attachments_count` | Integer | Yes | Number of attachments |
| `processed_at` | DateTime | Yes | When processed |
| `status` | Enum: [PARSED, SKIPPED, FAILED] | Yes | Processing outcome |

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
| `DELETE` | `/documents/{id}` | Delete document and its data |

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
| `POST` | `/queue/pause` | Pause processing |
| `POST` | `/queue/resume` | Resume processing |

### Batch Operations

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/documents/import/s3` | Import documents from S3 bucket |
| `POST` | `/documents/import/gcs` | Import documents from Google Cloud Storage |
| `POST` | `/documents/import/local` | Import from local filesystem |
| `GET` | `/documents/export/archive` | Export documents as ZIP archive |

---

## Competitive Intelligence Deep Dive

### DocuPipe: Cloud-Native Upload
DocuPipe accepts documents via API (multipart, base64, or URL reference). Supports 30+ formats including PDF, DOCX, XLSX, PNG, JPG, TIFF, and scanned images. No email ingestion — purely API-driven. The simplicity is a feature: submit a document, get JSON back. But the lack of email, scan, or cloud storage connectors limits enterprise workflows.

### AWS Textract: Infrastructure-Heavy
Textract accepts documents via API (S3 URI, base64, or streaming). No built-in document storage — you manage the S3 bucket. Email integration requires separate SES setup. The advantage is native AWS integration: S3 event triggers can automatically invoke Textract. The disadvantage is operational complexity — you're building the pipeline on raw infrastructure primitives.

### Rossum: Email-First Enterprise
Rossum's crown jewel is its intelligent mailbox. Documents arrive via email, API, SFTP, or UI upload. The validation screen handles exceptions before they become problems. The system learns from each user correction, improving extraction accuracy over time. Enterprise-grade: supports email routing rules, DL lists, and mailbox permissions.

### Paperless-ngx: Open-Source Email Ingestion
Paperless-ngx supports email ingestion natively. Configure multiple IMAP accounts, set rules for filtering, and attachments are automatically processed through OCR and classification. The ML-based auto-tagging learns from user corrections. Free and self-hosted — no per-page costs, no API limits.

### M-Files: Context-First Ingestion
M-Files ingests documents into a metadata-driven system. Every document is automatically classified by its content and context (project, person, document type). No rigid folder structure — documents are found by what they ARE, not where they live. Microsoft 365 integration means SharePoint and Teams documents flow in automatically.

---

## Implementation Roadmap

### Phase 1: Basic Upload (2-3 weeks) — P0
1. Define `Document` entity with all fields
2. Implement multipart file upload endpoint
3. MIME type detection and file size validation
4. SHA-256 checksum computation for dedup
5. Basic status tracking (QUEUED → PROCESSING → COMPLETED/FAILED)
6. File storage on local filesystem (S3 backend configurable)

### Phase 2: API & Batch (2-3 weeks) — P0
1. API ingestion endpoint (base64 and URL reference)
2. Batch upload endpoint for multiple files
3. Processing queue entity and management endpoints
4. Retry logic with configurable max attempts
5. Status polling endpoint for async processing

### Phase 3: Email & Cloud (3-4 weeks) — P1
1. Email ingestion webhook endpoint
2. IMAP email connector (configurable accounts)
3. S3/GCS/Azure Blob storage webhook listeners
4. Attachment extraction from emails
5. Email processing rules engine

### Phase 4: Advanced Features (2-3 weeks) — P1
1. Virus scanning integration (ClamAV hook)
2. Format conversion pipeline (Office → PDF/A)
3. Duplicate detection before processing
4. Rate limiting and concurrency controls
5. Audit logging for all ingestion events

---

## Key Takeaway for Buyers

RERP Documents' pitch for ingestion is **any source, any format, zero per-page cost**. Unlike DocuPipe (API-only) or Textract (infrastructure-heavy), RERP provides a unified intake layer that handles email, API, scan, and cloud storage in one system. Unlike Paperless-ngx (which is storage-focused), RERP's ingestion feeds directly into a structured data pipeline with OpenAPI-defined schemas.

The Rust-native processing queue can handle 10,000+ concurrent uploads with sub-second response times — something Python-based competitors struggle with at scale. And because ingestion is OpenAPI-first, every client gets type-safe SDKs, automatic validation, and complete API documentation out of the box.

**The immediate priority: define the Document entity, implement multipart upload, and build the processing queue. Everything else depends on documents getting into the system.**
