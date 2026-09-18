# Document Ingestion & Intake

> **Component:** Document intake layer — capture, validate, deduplicate, and dispatch documents into the processing pipeline
> **Priority:** P0 — Foundation layer; every downstream component (OCR, classification, extraction) depends on this
> **Reference Competitors:** AWS S3 event ingestion / Kinesis Streams, Google Cloud Storage + Pub/Sub, Azure Blob Storage triggers, ABBYY Capture CaptureSuite, Kofax Capture

---

## The Pitch

**Buyer Question:** *Can I accept documents from any source — email, API, SFTP, web upload, scanner — validate them instantly, detect duplicates before they waste processing resources, and hand them off to the right pipeline?*

If your document intake is fragmented — separate workflows for email attachments, FTP drops, web uploads, and scanner feeds — you're bleeding operational cost and creating data silos. Every competitor in the document processing space starts with an ingestion layer because it is the universal entry point. The difference is how unified, fast, and intelligent that layer is. RERP's ingestion component is a single, extensible pipeline that normalizes all document intake into a standard processing envelope.

---

## What This Component Does

Document Ingestion & Intake is the front door to the RERP Document Processing Platform. It handles the complete intake lifecycle:

1. **Multi-Channel Intake** — Documents enter via email (IMAP/POP3 mailbox monitoring), REST API, SFTP drop folders, web form upload, and TWAIN/WIA scanner integration
2. **Format Validation** — On arrival, each document is checked for valid file signatures (magic bytes), MIME type consistency, file size limits, and virus/malware scan via ClamAV or integrated AV service
3. **Content-Type Detection** — File extension is ignored; content-type is detected via magic number inspection (libmagic/libmagic-rs) to prevent spoofed extensions (e.g., .pdf renamed from .exe)
4. **Batch Processing** — Documents can be submitted as individual items or in batches. Batches support grouping by source, destination, priority, and business rule. Batch-level tracking includes total items, processed, failed, pending.
5. **Chunking for Large Documents** — Documents exceeding a configurable size threshold (default 50MB) are automatically chunked into segments. Each segment gets a sequence number and the parent document reference. Chunked documents are reassembled during downstream processing.
6. **Duplicate Detection** — Pre-ingest deduplication computes SHA-256 hash of file content and compares against a configurable retention window (default 90 days). Hash collisions trigger a duplicate alert; content-aware dedup (simhash) catches near-duplicates with minor modifications.
7. **Metadata Extraction on Intake** — Basic metadata is extracted immediately: filename, size, upload timestamp, source channel, uploader identity, detected content-type, page count (for PDF), character count (for text). This metadata is attached to the processing envelope without waiting for OCR or classification.
8. **Routing Dispatch** — Based on configurable rules (source, content-type, sender, tags), each document is routed to the appropriate OCR job, classification queue, or direct archival path.

---

## Entity Model

### DocumentIngestJob

The central entity representing a single ingestion operation.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `batch_id` | UUID | No | Associated batch (nullable for single-doc uploads) |
| `source_id` | UUID | Yes | Foreign key to DocumentSource; identifies intake channel |
| `status` | Enum: [PENDING, VALIDATING, CHUNKING, QUEUED, PROCESSING, COMPLETED, FAILED, DUPLICATE] | Yes | Current lifecycle stage |
| `original_filename` | String (512) | Yes | Original name from the uploader/source |
| `stored_filename` | String (512) | Yes | Server-assigned filename with UUID prefix |
| `file_size_bytes` | Int64 | Yes | File size in bytes |
| `content_type` | String (128) | Yes | Detected MIME type via magic number |
| `file_hash_sha256` | String (64) | Yes | SHA-256 hex digest for dedup and integrity |
| `page_count` | Int32 | No | Page count for PDF/image (set after format validation) |
| `chunk_index` | Int32 | No | Segment number (0 = first chunk, -1 = full doc) |
| `chunk_total` | Int32 | No | Total chunks (-1 = not chunked) |
| `is_duplicate` | Boolean | No | Set to true if content hash matches existing document |
| `duplicate_of_id` | UUID | No | Reference to the original document if duplicate |
| `upload_channel` | Enum: [API, EMAIL, SFTP, WEB, SCANNER] | Yes | Source intake channel |
| `sender_email` | String (255) | No | Email sender (for email intake) |
| `uploader_id` | UUID | No | User who uploaded (for API/web intake) |
| `sftp_path` | String (1024) | No | SFTP drop path (for SFTP intake) |
| `validation_errors` | JSONB | No | Array of validation error objects |
| `routing_rules_matched` | JSONB | No | Array of rule IDs that matched for dispatch |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `updated_at` | DateTime | Yes | Auto-updated on state change |
| `completed_at` | DateTime | No | When status transitions to COMPLETED or FAILED |

### DocumentSource

Defines an intake channel and its configuration.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Human-readable name (e.g., "AP Invoices Email") |
| `type` | Enum: [EMAIL, SFTP, API, WEB, SCANNER] | Yes | Channel type |
| `config` | JSONB | Yes | Channel-specific configuration (see below) |
| `is_active` | Boolean | No | Soft toggle for enable/disable |
| `max_file_size_mb` | Int32 | No | Max file size per document (default 50MB) |
| `allowed_content_types` | String Array | No | Whitelist of accepted MIME types |
| `auto_route_enabled` | Boolean | No | Auto-dispatch to processing pipeline |
| `routing_rule_ids` | UUID Array | No | Associated routing rules |
| `polling_interval_sec` | Int32 | No | For EMAIL/SFTP: check interval in seconds |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `updated_at` | DateTime | Yes | Auto-updated on config change |

**Channel Config Examples:**

- EMAIL: `{"imap_host": "mail.company.com", "imap_port": 993, "imap_user": "docs@company.com", "imap_password_encrypted": "...", "folder": "INBOX", "watch_folder": true}`
- SFTP: `{"host": "sftp.company.com", "port": 22, "username": "rerp-drop", "key_path": "/etc/rerp/sftp_key", "drop_path": "/incoming/documents"}`
- SCANNER: `{"twain_source": "HP_ScanJet_4100", "resolution_dpi": 300, `default_format": "PDF", "auto_detect_duplex": true}`

### DocumentBatch

Groups multiple ingestion jobs for tracking and reporting.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Batch name (e.g., "March Vendor Invoices") |
| `status` | Enum: [ACTIVE, COMPLETED, FAILED, CANCELLED] | Yes | Batch lifecycle state |
| `total_items` | Int32 | No | Total documents expected |
| `processed_items` | Int32 | No | Documents successfully processed |
| `failed_items` | Int32 | No | Documents that failed |
| `duplicate_items` | Int32 | No | Documents identified as duplicates |
| `source_ids` | UUID Array | Yes | Associated document sources |
| `tags` | String Array | No | Freeform tags for grouping |
| `priority` | Enum: [LOW, NORMAL, HIGH, URGENT] | No | Batch processing priority |
| `retention_days` | Int32 | No | Days to retain batch records |
| `created_at` | DateTime | Yes | Auto-set on creation |
| `completed_at` | DateTime | No | When all items processed or batch ended |

---

## Entity Relationships

```
DocumentIngestJob
  ├── DocumentSource (via source_id)        ← intake channel definition
  ├── DocumentBatch (via batch_id)          ← batch grouping (1:N, nullable)
  ├── DocumentIngestJob (via duplicate_of_id) ← self-reference for dedup
  └── ClassificationModel (via routing_rules_matched) ← dispatch target

DocumentSource
  └── DocumentIngestJob × N                 ← one source can have many jobs

DocumentBatch
  └── DocumentIngestJob × N                 ← one batch contains many jobs
```

---

## Required API Endpoints

### DocumentIngestJob CRUD

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/ingestion/jobs` | List ingestion jobs with filters, pagination, sorting |
| `GET` | `/ingestion/jobs/{id}` | Get full job detail including metadata and status |
| `POST` | `/ingestion/jobs` | Upload/submit a document for ingestion |
| `PATCH` | `/ingestion/jobs/{id}` | Update job metadata (tags, priority, notes) |
| `DELETE` | `/ingestion/jobs/{id}` | Cancel job and purge file (if not yet processing) |
| `GET` | `/ingestion/jobs/{id}/download` | Download the original file |

### Batch Operations

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/ingestion/batches` | List batches with status and progress |
| `POST` | `/ingestion/batches` | Create a new batch for grouped ingestion |
| `GET` | `/ingestion/batches/{id}` | Get batch detail with per-job breakdown |
| `POST` | `/ingestion/batches/{id}/upload` | Upload documents to an existing batch |

### Intake Source Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/ingestion/sources` | List all configured intake sources |
| `POST` | `/ingestion/sources` | Register a new intake channel |
| `PATCH` | `/ingestion/sources/{id}` | Update source configuration |
| `DELETE` | `/ingestion/sources/{id}` | Disable and remove an intake source |
| `POST` | `/ingestion/sources/{id}/test` | Test connectivity (EMAIL: send test email, SFTP: list files) |

### Dedup & Validation

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/ingestion/jobs/check-duplicate` | Check if a file content hash exists (dry-run) |
| `GET` | `/ingestion/jobs/duplicates` | List all duplicate pairs within retention window |
| `POST` | `/ingestion/jobs/{id}/validate` | Manually re-run validation checks |

### Bulk Operations

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/ingestion/jobs/bulk-upload` | Upload multiple files via multipart |
| `POST` | `/ingestion/jobs/bulk-verify` | Batch hash check for dedup before ingest |

---

## Competitive Positioning

### Where RERP Wins

- **Unified intake pipeline** — Single API accepts from all channels. Competitors (AWS S3, Azure Blob, GCS) require separate integrations for each channel. RERP normalizes them.
- **Built-in deduplication** — SHA-256 + simhash content-aware dedup at ingestion time prevents waste downstream. AWS S3 requires external Lambda for dedup. Azure requires Logic Apps.
- **Open-source self-hosted** — No per-document pricing. Process 10 or 10 million documents without incremental cost. ABBYY Capture, Kofax, and Docuware charge per scanned page or per document processed.
- **Rust-native performance** — File I/O, hash computation, and validation in Rust are orders of magnitude faster than Python-based ingestion layers used by many competitors.

### Where RERP Lags

- **Scanner integration (TWAIN/WIA)** — Kofax Capture and ABBYY Capture have decades of scanner driver integration. RERP's TWAIN/WIA support is a greenfield effort.
- **Email parsing complexity** — ABBYY Capture and Kofax handle complex email scenarios (forwarded threads, multiple attachments, embedded files in emails) out of the box. RERP needs to build this.
- **Enterprise SSO for intake portals** — ABBYY and Kofax integrate with Active Directory, Okta, and SAML for web intake portals. RERP needs to build SSO for the ingestion web interface.
- **Compliance certifications** — ABBYY and Kofax have SOC 2, HIPAA, and FedRAMP. As open-source, RERP doesn't have these certifications (but avoids the certification cost).

---

## Competitive Intelligence Deep Dive

### AWS S3 Event Ingestion + Kinesis

AWS's approach is event-driven: files land in S3, S3 triggers a Lambda, which queues events to Kinesis. It's highly scalable but architecturally complex — you manage S3 buckets, Lambda functions, Kinesis streams, IAM roles, and monitoring dashboards. Pricing: S3 storage (~$0.023/GB/month), Lambda compute (~$0.20/1M requests), Kinesis (~$0.014/GB ingested). **Total cost for 1M documents/year: ~$50-200 depending on size and Lambda calls.** The hidden cost is engineering time to wire everything together. RERP's ingestion is a single component; AWS's is a distributed system you build yourself.

### Google Cloud Storage + Pub/Sub

Similar to AWS: GCS triggers Pub/Sub messages that fan out to processing services. GCS offers free egress from Cloud Run (within same region). Pricing: GCS Standard (~$0.02/GB/month), Pub/Sub (~$0.05/GB published). **Total cost for 1M documents/year: ~$30-150.** Google's Document AI can be triggered automatically but costs $1.50/1,000 pages for basic parsing. RERP provides the ingestion layer without forcing you into Google's paid AI layer.

### Azure Blob Storage Triggers

Azure's blob storage triggers Azure Functions via blob change events. Azure Functions has a free tier (1M executions/month). Pricing: Blob Storage LRS (~$0.02/GB/month), Functions (~$0.000016/execution). **Total cost for 1M documents/year: ~$20-100.** However, Azure's advantage is deep Office 365 integration — email attachments from Exchange can be captured via Microsoft Flow without any custom code. RERP must build its own email-to-doc pipeline.

### ABBYY Capture CaptureSuite

ABBYY Capture is the enterprise scanner workflow leader. Key differentiator: it runs on-premise, connects directly to scanners via TWAIN/ISIS drivers, and provides a visual workflow designer (drag-and-drop rule builder) for intake validation and routing. It handles scanning, validation, indexing, and delivery in one package. Pricing: $5,000-50,000+ per server license + $2,000-10,000/year maintenance. **For a 100-user org: ~$75,000 total cost of ownership (TCO) over 3 years.** RERP's advantage: the entire pipeline (ingestion + OCR + classification + extraction) is a single integrated system vs. ABBYY's point solutions stitched together.

### Kofax Capture

Kofax is ABBYY's main competitor for scanner-based intake. Kofax's strength is in high-volume enterprise document processing: banks, insurance, and government. It offers "Kofax TotalAgility" for end-to-end document lifecycle management. Pricing: ~$10,000-100,000+ per license depending on user count and modules. **TCO over 3 years: $50,000-300,000.** Kofax excels at complex pre-scan workflows (multi-stage validation, human review gates). RERP's advantage: open-source extensibility vs. Kofax's proprietary SDK, and no per-document fee.

---

## Implementation Roadmap

### Phase 1: Core Ingestion (Weeks 1-4) — P0

1. Define `DocumentIngestJob`, `DocumentSource`, `DocumentBatch` entities in OpenAPI spec with all fields from entity model
2. Implement REST API for single-document upload via API endpoint
3. Implement file signature validation (magic bytes) and MIME type detection via libmagic
4. Implement SHA-256 hash computation and dedup check against retention store
5. Implement basic SFTP drop folder source polling (cron-based)
6. Define ingestion status machine with transitions and validation rules
7. Generate Rust server stubs from OpenAPI spec

### Phase 2: Channel Expansion (Weeks 5-8) — P0

1. Implement email intake source (IMAP polling with attachment extraction)
2. Implement web upload portal (HTML form with drag-and-drop)
3. Implement batch ingestion API (multipart upload, batch tracking)
4. Add chunking for large documents (>50MB configurable)
5. Implement content-type spoofing detection
6. Add virus scan integration (ClamAV socket connection)
7. Configure routing rules engine (source-based dispatch to OCR/classification)

### Phase 3: Advanced Features (Weeks 9-12) — P1

1. Implement TWAIN/WIA scanner integration (desktop bridge service)
2. Add simhash-based near-duplicate detection for modified files
3. Implement batch-level progress tracking and reporting
4. Add webhook notifications for ingestion events (job completed, failed, duplicate)
5. Implement metadata extraction on intake (page count, character count)
6. Add SSO integration for web intake portal (SAML/OIDC)

### Phase 4: Scale & Hardening (Weeks 13-16) — P1

1. Implement distributed ingestion worker pool (multi-node)
2. Add rate limiting per source and per user
3. Implement ingestion audit trail (who uploaded what, when, from where)
4. Add integration tests for all intake channels
5. Performance benchmark: target 1,000 documents/minute sustained ingestion rate

---

## Key Takeaway for Buyers

RERP's Document Ingestion & Intake is the universal entry point that unifies every document channel into a single, high-performance pipeline. Unlike AWS/Azure/Google which require you to architect and maintain a distributed ingestion system, or ABBYY/Kofax which charge $50,000-300,000 for on-premise scanner workflows, RERP delivers a complete, open-source ingestion layer at zero marginal cost per document.

The competitive moat isn't in ingestion alone — every platform has one. It's in the fact that RERP's ingestion speaks the same language as the rest of the Document Processing Platform: the same entities, the same routing rules, the same metadata envelope. When a document lands, it doesn't just get stored — it's instantly prepared for OCR, classification, and extraction without any handoff friction.

**For buyers: If you're tired of stitching together AWS S3 + Lambda + Kinesis or paying ABBYY $75K for scanner intake, RERP's ingestion layer is the consolidated, self-hosted alternative that just works.**
