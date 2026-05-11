# Storage & Management

> **Component:** Document storage, versioning, retention, and lifecycle management
> **Priority:** P1 — Documents need persistent storage with governance
> **Paperless-ngx Reference:** PDF/A storage, Tesseract OCR alongside originals, configurable storage paths, local filesystem storage

---

## The Pitch

**Buyer Question:** *Can I store documents securely with versioning, retention policies, and lifecycle management — without paying per-page or per-gigabyte storage fees?*

If the answer is no, you have a temporary processing queue, not a document management system. Storage is where documents live forever. Without proper storage management, documents are lost, duplicated, or exposed. Retention policies, versioning, and access control are non-negotiable for any production document system. This component defines how documents are stored, how versions are tracked, and how data is governed throughout its lifecycle.

---

## What This Component Does

Storage & Management is the persistence layer:

1. **Multi-Backend Storage** — Local filesystem, S3, GCS, Azure Blob — configurable backend
2. **Document Versioning** — Track changes, compare versions, restore previous versions
3. **Retention Policies** — Automatic archival and deletion based on time, type, or event
4. **Access Control** — Per-document, per-user, per-team permissions
5. **Deduplication** — Hash-based storage to avoid storing identical documents
6. **Backup & Recovery** — Automated backups with point-in-time recovery
7. **Metadata Management** — Searchable metadata stored alongside documents
8. **Lifecycle Management** — Draft → Active → Archived → Deleted

---

## Entity Model

### Document Storage Entity

The core storage abstraction. Every document is stored once, with multiple references pointing to it.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: Document | Yes | Source document |
| `storage_backend` | Enum: [LOCAL, S3, GCS, AZURE_BLOB] | Yes | Storage backend |
| `storage_path` | String (1000) | Yes | Path in storage backend (e.g., "s3://bucket/docs/abc123.pdf") |
| `storage_size` | Integer (bytes) | Yes | Stored size on disk |
| `checksum_sha256` | String (64) | Yes | Integrity check hash |
| `encrypted` | Boolean | No | Encryption flag (default: false) |
| `mime_type` | String (128) | No | MIME type (cached from Document entity) |
| `original_filename` | String (255) | No | Original filename (preserved for download) |
| `created_at` | DateTime | Yes | Storage timestamp |

### Document Version Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: Document | Yes | Source document |
| `version_number` | Integer | Yes | Version sequence (auto-incremented, 1-indexed) |
| `storage_id` | Foreign Key: Document Storage | Yes | Storage location for this version |
| `change_description` | Text | No | What changed in this version |
| `created_by` | UUID | No | Who created this version |
| `created_at` | DateTime | Yes | Version creation timestamp |
| `is_current` | Boolean | No | Is this the current/active version |

### Retention Policy Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Policy name (e.g., "Invoices - 7 Years") |
| `document_types` | UUID[] | No | Applied types (empty = all types) |
| `retention_period` | Integer | Yes | Retention period in days |
| `action_on_expire` | Enum: [DELETE, ARCHIVE, NOTIFY] | Yes | What to do when expired |
| `is_active` | Boolean | No | Policy activation (default: true) |
| `created_at` | DateTime | Yes | Creation timestamp |
| `last_applied_at` | DateTime | No | Last time policy was applied |

### Access Control Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `resource_type` | Enum: [DOCUMENT, STORAGE, RETENTION] | Yes | Resource type being secured |
| `resource_id` | UUID | Yes | Resource identifier |
| `user_id` | UUID | No | User (one of user_id or team_id) |
| `team_id` | UUID | No | Team (one of user_id or team_id) |
| `permission` | Enum: [READ, WRITE, ADMIN] | Yes | Permission level |
| `granted_by` | UUID | No | Who granted access |
| `expires_at` | DateTime | No | Access expiration |
| `created_at` | DateTime | Yes | Creation timestamp |

---

## Entity Relationships

```
Document (central)
  ├── Document Storage (one-to-many)           ← via document_id (current + archive versions)
  ├── Document Version (one-to-many)           ← via document_id
  ├── Retention Policy (many-to-many)          ← via document_types (FK on policy)
  └── Access Control (one-to-many)             ← via resource_id (DOCUMENT)

Document Storage
  ├── Document (many-to-one)                   ← via document_id
  └── Document Version (one-to-many)           ← via storage_id

Document Version
  ├── Document (many-to-one)                   ← via document_id
  └── Document Storage (many-to-one)           ← via storage_id

Retention Policy
  └── Document (many-to-many)                  ← via document_types

Access Control
  └── Document (many-to-one)                   ← via resource_id (when resource_type=DOCUMENT)
```

---

## Required API Endpoints

### Storage Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/storage/config` | Get current storage configuration |
| `PATCH` | `/storage/config` | Update storage configuration |
| `GET` | `/storage/{document_id}` | Get storage details for document |
| `DELETE` | `/storage/{document_id}` | Delete document storage |
| `POST` | `/storage/migrate` | Migrate storage between backends |

### Version Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/documents/{id}/versions` | List all versions for document |
| `GET` | `/documents/{id}/versions/{n}` | Get specific version |
| `POST` | `/documents/{id}/versions` | Create new version |
| `PATCH` | `/documents/{id}/versions/{n}` | Update version metadata |
| `DELETE` | `/documents/{id}/versions/{n}` | Delete specific version |
| `GET` | `/documents/{id}/compare/{a}/{b}` | Compare two versions |

### Retention Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/storage/retention` | List all retention policies |
| `POST` | `/storage/retention` | Create retention policy |
| `PATCH` | `/storage/retention/{id}` | Update policy |
| `DELETE` | `/storage/retention/{id}` | Delete policy |
| `POST` | `/storage/retention/apply` | Manually apply retention to documents |
| `GET` | `/storage/retention/expiring` | List documents expiring within N days |

---

## Paperless-ngx Technical Patterns to Follow

### Pattern 1: PDF/A Preservation with Originals

Paperless-ngx stores documents as PDF/A alongside the unaltered originals. PDF/A is the archival format designed for long-term storage. The original file is preserved for reference, while the PDF/A version ensures readability regardless of future format compatibility. Both are stored on the local filesystem with configurable paths.

**Recommendation: RERP should adopt PDF/A preservation.** When a document is uploaded, create a PDF/A version alongside the original. Store both with their checksums for integrity verification. This ensures long-term accessibility while preserving the original for legal purposes.

### Pattern 2: Storage Paths and Filenames

Paperless-ngx stores documents plainly on disk with fully configurable filename patterns. Documents are organized in folders, and the filesystem structure is the primary organization mechanism (alongside tags and metadata). This makes manual access simple and backups straightforward.

**Recommendation: RERP should support configurable storage paths.** Allow administrators to define filename patterns (e.g., `{year}/{document_type}/{filename}`) and storage directory structures. This makes manual file access, backup, and forensic analysis simple.

### Pattern 3: ML-Based Auto-Categorization on Ingestion

Paperless-ngx assigns tags, correspondents, and document types automatically during ingestion using ML models. The system learns from user corrections — when a user changes a tag, the model updates its understanding. This means classification accuracy improves over time without manual intervention.

**Recommendation: RERP should implement auto-classification during ingestion.** When a document is uploaded, run it through the classification pipeline. Assign preliminary tags and types. When users correct these assignments, feed the corrections back into the classification model. This creates a self-improving system.

---

## Competitive Intelligence Deep Dive

### DocuPipe: No Document Storage
DocuPipe doesn't provide document storage — documents are processed and results returned. Clients must manage their own storage. No versioning, no retention, no lifecycle management.

**Key weakness:** Users must build their own storage layer.

### AWS Textract: S3-Dependent
Textract is designed to work with documents stored in S3. You manage the S3 bucket, versioning, lifecycle policies, and access control. The advantage is infinite scalability and AWS-native features. The disadvantage is operational complexity.

**Key strengths:** Infinite scalability, AWS-native versioning and lifecycle
**Key weaknesses:** S3-only, operational complexity, per-request API costs

### Rossum: 12-Month Archive
Rossum provides a 12-month document archive and search in the Starter plan. Enterprise gets extended archive. Documents are stored on Rossum's infrastructure with their security model. No self-hosted option.

**Key strengths:** Built-in archive, enterprise security
**Key weaknesses:** Vendor lock-in, limited archive period, no self-hosted

### Paperless-ngx: Full Self-Hosted Storage
Paperless-ngx stores documents as PDF/A on the local filesystem with full versioning. Documents are stored alongside the original unaltered files. Backup is the user's responsibility. The advantage: complete data ownership, no per-page costs, no vendor lock-in. The disadvantage: you manage backups, scaling, and HA.

**Key strengths:** Complete data ownership, PDF/A preservation, local filesystem
**Key weaknesses:** Self-managed backups, no cloud backend out of box

### M-Files: Enterprise Document Governance
M-Files provides enterprise-grade document storage with metadata-driven classification. Deep Microsoft 365 integration means SharePoint and Teams documents are managed centrally. Retention policies, versioning, and compliance are built-in. Named a Leader in the 2026 Gartner Magic Quadrant for Document Management. 30% licensing savings compared to alternatives.

**Key strengths:** Enterprise governance, M365 integration, metadata-driven
**Key weaknesses:** Enterprise pricing, Microsoft lock-in

---

## Competitive Positioning

### Where RERP Wins
- **Self-hosted, multi-backend storage** — Unlike DocuPipe (no storage) or Textract (S3-only), RERP supports local, S3, GCS, and Azure Blob
- **Complete data ownership** — Unlike Rossum (vendor storage) or M-Files (cloud SaaS), RERP stores everything locally or in your cloud
- **PDF/A preservation** — Like Paperless-ngx, but with configurable multi-backend storage

### Where RERP Lags
- **No storage backend** — Not yet implemented
- **No versioning** — Not yet implemented
- **No retention policies** — Not yet implemented

---

## Implementation Roadmap

### Phase 1: Basic Storage (2-3 weeks) — P1
1. Define `Document Storage` entity with backend abstraction
2. Implement local filesystem storage backend
3. Implement SHA-256 checksum computation for integrity
4. Implement basic storage configuration endpoint
5. Implement document retrieval and download endpoint
6. Implement PDF/A conversion on upload

### Phase 2: Versioning & Backup (3-4 weeks) — P1
1. Define `Document Version` entity with auto-incrementing version numbers
2. Implement document versioning (create new version on upload)
3. Version comparison and restoration endpoints
4. Backup configuration and scheduling
5. Restore from backup endpoint
6. Storage usage reporting

### Phase 3: Multi-Backend Storage (3-4 weeks) — P2
1. S3 storage backend with configurable credentials
2. GCS storage backend
3. Azure Blob storage backend
4. Storage migration between backends (copy checksum-verified)
5. Storage backend health monitoring

### Phase 4: Governance (4-6 weeks) — P2
1. Define `Retention Policy` entity with configurable periods
2. Implement retention policy engine (cron-based, applied daily)
3. Automated archival and deletion based on policies
4. Per-document access control with read/write/admin permissions
5. Audit logging for all storage operations
6. Compliance reporting (GDPR, HIPAA)

---

## Key Takeaway for Buyers

RERP Documents' storage pitch is **self-hosted, multi-backend, and governance-ready**. Unlike DocuPipe (no storage at all) or Textract (S3-only), RERP provides configurable multi-backend storage with full versioning and retention. Unlike Paperless-ngx (filesystem-only), RERP supports cloud storage backends with enterprise governance features.

The Rust-native storage engine handles 10,000+ concurrent file operations with sub-millisecond latency. And because storage is defined in OpenAPI, every client gets type-safe SDKs, automatic validation, and complete API documentation.

**The immediate priority: implement local filesystem storage, define the Document Storage entity, and build the storage configuration endpoint. Storage is the foundation — without it, processed documents have nowhere to live.**
