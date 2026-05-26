# Document Storage & Versioning

> **Component:** Document repository with version control, metadata tagging, full-text search, deduplication, retention policies, and access control
> **Priority:** P1 — Store, version, and retrieve documents at enterprise scale
> **ABBYY Reference:** ABBYY Capture Center content management, Kofax Capture Center, DocumentDB

---

## The Pitch

**Buyer Question:** *Can I store every document, track every version, find anything instantly, and control who sees what — without paying per-gigabyte or per-document?*

A document processing system that can extract data but can't reliably store, version, and retrieve documents is a leaky bucket. You need a robust document repository that serves as the single source of truth for all business documents — linked to records, contacts, deals, and employees. This component covers storage, versioning, search, deduplication, retention, and access control.

---

## What This Component Does

1. **Document Repository** — Centralized storage for all document types (PDFs, images, scans, office docs)
2. **Version Control** — Every modification creates a new version; track who changed what and when
3. **Metadata Tagging** — Attach structured metadata (document type, date, source, classification, extraction results)
4. **Full-Text Search** — Search across document content, not just filenames or metadata
5. **Deduplication** — Detect and merge duplicate documents (by hash or content similarity)
6. **Retention Policies** — Automatic archival and deletion based on configured retention rules
7. **Access Control** — Role-based access control (RBAC) with fine-grained permissions per document
8. **Document Linking** — Link documents to business records (invoices, contracts, employee profiles, deals)
9. **Storage Optimization** — Compression, deduplication, tiered storage (hot/warm/cold)

---

## Entity Model

### DocumentStore Entity

The root entity that represents a single document in the repository:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `filename` | String (512) | Yes | Original filename |
| `content_type` | String (255) | Yes | MIME type (application/pdf, image/png, etc.) |
| `size_bytes` | Integer (64) | Yes | File size in bytes |
| `sha256_hash` | String (64) | Yes | SHA-256 hash for dedup and integrity |
| `upload_source` | Enum: [API, EMAIL, SFTP, WEB, SCANNER, BATCH] | No | How the document entered the system |
| `uploaded_by` | Foreign Key: User | Yes | User who uploaded |
| `classification_id` | Foreign Key: DocumentClass | No | Classification result |
| `status` | Enum: [PENDING, PROCESSING, COMPLETED, ERROR, ARCHIVED] | No | Processing status |
| `is_deleted` | Boolean | No | Soft delete flag |
| `deleted_at` | DateTime | No | Deletion timestamp |
| `created_at` | DateTime | Yes | Upload timestamp |
| `updated_at` | DateTime | Yes | Last modification timestamp |

### DocumentVersion Entity

Tracks every version of a document:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: DocumentStore | Yes | Parent document |
| `version_number` | Integer | Yes | Sequential version number (1, 2, 3, ...) |
| `content_bytes` | Binary | Yes | Document content (or reference to storage) |
| `storage_path` | String (1024) | No | Path in storage backend (S3, local, etc.) |
| `checksum` | String (64) | Yes | Content hash for this version |
| `comment` | Text | No | Reason for this version |
| `created_by` | Foreign Key: User | Yes | User who created this version |
| `created_at` | DateTime | Yes | Version creation timestamp |

### DocumentMetadata Entity

Structured metadata attached to documents:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: DocumentStore | Yes | Parent document |
| `key` | String (128) | Yes | Metadata key (e.g., "invoice_number", "supplier_name") |
| `value` | Text | Yes | Metadata value |
| `source` | Enum: [MANUAL, AUTOMATED, EXTRACTION, CLASSIFICATION] | No | How metadata was set |
| `confidence` | Float (0-100) | No | Confidence score (for automated metadata) |
| `created_at` | DateTime | Yes | When metadata was added |

### DocumentLink Entity

Links documents to business records:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: DocumentStore | Yes | Document being linked |
| `entity_type` | String (64) | Yes | Target entity type (accounting.invoice, crm.lead, hr.employee) |
| `entity_id` | UUID | Yes | Target entity ID |
| `link_type` | Enum: [SOURCE, ATTACHMENT, CONTRACT, IDENTITY, RECEIPT] | No | Type of link |
| `created_at` | DateTime | Yes | When link was created |

### RetentionPolicy Entity

Defines automatic retention and deletion rules:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Policy name (e.g., "Invoices 7 Years") |
| `description` | Text | No | Policy description |
| `entity_type` | String (64) | Yes | Applies to which entity types |
| `document_type` | String (64) | No | Applies to which document types (NULL = all) |
| `retention_days` | Integer | Yes | Days to retain after processing |
| `action_on_expiry` | Enum: [DELETE, ARCHIVE, REDACT] | Yes | Action when retention expires |
| `is_active` | Boolean | Yes | Enable/disable policy |
| `created_at` | DateTime | Yes | Policy creation timestamp |

### StorageIndex Entity

Full-text search index for document content:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: DocumentStore | Yes | Document being indexed |
| `search_text` | Text | Yes | Extracted text for full-text search |
| `vector_embedding` | Vector | No | Vector embedding for semantic search |
| `indexed_at` | DateTime | Yes | When indexing occurred |
| `status` | Enum: [PENDING, INDEXED, ERROR] | No | Indexing status |

---

## Entity Relationships

```
DocumentStore (central document record)
  ├── DocumentVersion (via document_id)          ← version history
  ├── DocumentMetadata (via document_id)          ← structured metadata
  ├── DocumentLink (via document_id)              ← links to business records
  └── StorageIndex (via document_id)              ← search index

DocumentVersion
  ├── DocumentStore (via document_id)              ← parent document
  └── User (via created_by)                        ← who created version

DocumentMetadata
  ├── DocumentStore (via document_id)              ← parent document
  └── User (via extracted_by, if automated)        ← who extracted metadata

DocumentLink
  ├── DocumentStore (via document_id)              ← linked document
  └── [Dynamic] (via entity_type + entity_id)      ← target business record

RetentionPolicy
  └── DocumentStore (via policy_id filter)         ← which documents match policy
```

---

## Required API Endpoints

### Document CRUD

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/documents` | List documents with filters, pagination, search |
| `GET` | `/documents/{id}` | Get document detail with all versions |
| `POST` | `/documents` | Upload a new document |
| `PATCH` | `/documents/{id}` | Update document metadata |
| `DELETE` | `/documents/{id}` | Soft-delete document |
| `GET` | `/documents/search/fulltext` | Full-text search across document content |
| `GET` | `/documents/search/semantic` | Semantic/vector search |
| `GET` | `/documents/{id}/dedup` | Check for duplicate documents |

### Version Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `PUT` | `/documents/{id}/version` | Upload new version of document |
| `GET` | `/documents/{id}/versions` | List all versions |
| `GET` | `/documents/{id}/versions/{version}` | Get specific version content |
| `DELETE` | `/documents/{id}/versions/{version}` | Delete a version |
| `GET` | `/documents/{id}/diff` | Compare two versions |

### Metadata Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/documents/{id}/metadata` | List all metadata for a document |
| `POST` | `/documents/{id}/metadata` | Add metadata to a document |
| `PATCH` | `/documents/{id}/metadata/{key}` | Update metadata field |
| `DELETE` | `/documents/{id}/metadata/{key}` | Remove metadata field |

### Document Linking

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/documents/{id}/links` | List all links for a document |
| `POST` | `/documents/{id}/link` | Link document to a business record |
| `DELETE` | `/documents/{id}/link/{link_id}` | Remove a link |
| `GET` | `/documents/by-link/{entity}/{id}` | Get all documents linked to an entity |

### Retention & Compliance

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/retention/policies` | List all retention policies |
| `POST` | `/retention/policies` | Create retention policy |
| `PATCH` | `/retention/policies/{id}` | Update retention policy |
| `DELETE` | `/retention/policies/{id}` | Delete retention policy |
| `POST` | `/retention/enforce` | Run retention enforcement |
| `GET` | `/retention/audit` | Audit trail of retention actions |

### Storage Analytics

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/storage/usage` | Total storage usage by type |
| `GET` | `/storage/duplicates` | List duplicate documents |
| `GET` | `/storage/size-breakdown` | Storage by document type, date range |
| `GET` | `/storage/tier-status` | Hot/warm/cold tier distribution |

---

## Competitive Positioning

### Where RERP Wins

- **Zero marginal cost at scale** — Once self-hosted, storing 1TB of documents costs the same as 1GB. Competitors charge $0.023/GB/month (AWS S3 Standard) + processing fees.
- **OpenAPI-defined storage schema** — Every entity, field, and relationship is machine-readable. BI tools can query storage metrics directly via API.
- **Rust-level search performance** — Full-text search across 1 million documents in Rust is instantaneous. Python-based search (ABBYY) can be slow at scale.
- **Self-hosted, no vendor lock-in** — No storage egress fees, no API rate limits, no data egress fees. Full control over storage backend (local, S3, Azure Blob, GCS).

### Where RERP Lags

- **No storage backend** — No document repository with versioning.
- **No full-text search** — No indexing pipeline for document content.
- **No deduplication** — No hash-based or content-based duplicate detection.
- **No retention engine** — No automatic archival and deletion.
- **No access control** — No RBAC for document access.

---

## Competitive Intelligence Deep Dive

### ABBYY Content Management

ABBYY's content management provides document storage with versioning, metadata, and access control. It integrates tightly with FlexiCapture for end-to-end document workflows. Enterprise customers include McDonald's, Siemens, and DHL. The key differentiator is deep enterprise integration but at significant cost ($100K+ license). ABBYY supports on-premises, cloud (Azure), and SDK deployments. Storage is managed internally — no direct integration with external storage backends.

### Kofax Capture Center

Kofax positions itself as a process orchestration platform with document intelligence at its core. Its Capture Center provides document storage with versioning, metadata, and workflow automation. The key differentiator is enterprise workflow automation — Kofax integrates with SAP, Oracle, and other ERP systems. Storage is managed internally with limited external integration. Kofax is an Everest Group Peak Matrix Leader in Intelligent Document Processing.

### AWS S3 + DocumentDB

AWS's approach is to use S3 for object storage and DocumentDB (MongoDB-compatible) for metadata. This is the most flexible approach — you control storage costs, retention, and access control. However, it requires significant development effort to build search, deduplication, and versioning on top of raw storage. AWS Textract handles extraction separately. Total cost: ~$0.023/GB/month for S3 + Textract processing fees.

### Google Cloud Storage + Document AI

Google's approach mirrors AWS — Cloud Storage for objects, Document AI for extraction, Cloud SQL/DocumentDB for metadata. The key advantage is tight integration with Google's AI/ML ecosystem (Vertex AI, BigQuery). However, storage and search must be built externally. Google Document AI charges $0.0015-$0.10/page for extraction.

### Azure Blob Storage + Document Intelligence

Microsoft's approach is similar — Blob Storage for objects, Document Intelligence for extraction, Cosmos DB/SQL for metadata. The key advantage is integration with Microsoft ecosystem (Power Automate, Dataverse, Dynamics 365). Document Intelligence charges $0.01-$0.03/page.

---

## Implementation Roadmap

### Phase 1: Core Document Store (2-3 weeks) — P1

1. Define `DocumentStore` entity: id, filename, content_type, size_bytes, sha256_hash, upload_source, status, created_at, updated_at
2. Define `DocumentVersion` entity: id, document_id, version_number, content_bytes, storage_path, checksum, comment, created_by
3. Implement document upload endpoint (POST /documents)
4. Implement version creation on re-upload (PUT /documents/{id}/version)
5. Implement document retrieval endpoint (GET /documents/{id})
6. Seed default storage configuration (local filesystem or S3)

### Phase 2: Metadata & Linking (2-3 weeks) — P1

1. Define `DocumentMetadata` entity: id, document_id, key, value, source, confidence
2. Define `DocumentLink` entity: id, document_id, entity_type, entity_id, link_type
3. Implement metadata CRUD endpoints
4. Implement document linking endpoints
5. Implement linked document listing (GET /documents/by-link/{entity}/{id})
6. Extract metadata automatically from extraction results

### Phase 3: Search & Dedup (3-4 weeks) — P1

1. Define `StorageIndex` entity for full-text search
2. Implement text extraction from documents (integrate with OCR service)
3. Implement full-text search endpoint (POST /documents/search/fulltext)
4. Implement SHA-256 hash-based deduplication
5. Implement duplicate detection endpoint (GET /documents/{id}/dedup)
6. Add semantic search via vector embeddings (Phase 4)

### Phase 4: Retention & Compliance (3-4 weeks) — P1

1. Define `RetentionPolicy` entity: id, name, description, entity_type, retention_days, action_on_expiry
2. Implement retention policy CRUD endpoints
3. Implement retention enforcement cron job (DELETE/ARCHIVE/REDACT expired documents)
4. Implement audit logging for all retention actions
5. Add soft-delete with configurable retention period
6. Implement storage analytics (usage, duplicates, size breakdown)

### Phase 5: Advanced Features (4-6 weeks) — P1

1. Implement version diffing (compare two versions)
2. Implement tiered storage (hot/warm/cold)
3. Implement storage compression and optimization
4. Add RBAC for document access control
5. Implement semantic/vector search via embedding service
6. Add storage quota management and alerts

---

## Key Takeaway for Buyers

Document storage and versioning is the foundation of any document processing system. A buyer needs to know: *Can I store every document, track every version, find anything instantly, and control who sees what?* RERP's advantage is self-hosted deployment with zero marginal cost at scale — once you have the infrastructure, storing 1TB costs the same as 1GB. The open API means every storage metric is queryable via API, and BI tools can consume storage analytics directly. The immediate priority: define the DocumentStore and DocumentVersion entities, implement upload/retrieval endpoints, and build the full-text search index. Everything else depends on this foundation.
