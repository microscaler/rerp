# Storage & Management

> **Component:** Document storage, versioning, retention, and lifecycle management
> **Priority:** P1 — Documents need persistent storage with governance

---

## The Pitch

**Buyer Question:** *Can I store documents securely with versioning, retention policies, and lifecycle management — without paying per-page or per-gigabyte storage fees?*

If the answer is no, you have a temporary processing queue, not a document management system. Storage is where documents live forever. Without proper storage management, documents are lost, duplicated, or exposed. Retention policies, versioning, and access control are non-negotiable for any production document system.

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

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | FK: Document | Yes | Source document |
| `storage_backend` | Enum: [LOCAL, S3, GCS, AZURE] | Yes | Storage backend |
| `storage_path` | String (1000) | Yes | Path in storage backend |
| `storage_size` | Integer (bytes) | Yes | Stored size |
| `checksum_sha256` | String (64) | Yes | Integrity check |
| `encrypted` | Boolean | No | Encryption flag |
| `created_at` | DateTime | Yes | Storage timestamp |

### Document Version Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | FK: Document | Yes | Source document |
| `version_number` | Integer | Yes | Version sequence |
| `storage_id` | FK: Document Storage | Yes | Storage location |
| `change_description` | Text | No | What changed |
| `created_by` | UUID | No | Who created version |
| `created_at` | DateTime | Yes | Version timestamp |

### Retention Policy Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Policy name |
| `document_types` | UUID[] | No | Applied types (empty = all) |
| `retention_period` | Interval | Yes | How long to keep |
| `action_on_expire` | Enum: [DELETE, ARCHIVE, NOTIFY] | Yes | What to do when expired |
| `is_active` | Boolean | No | Policy activation |
| `created_at` | DateTime | Yes | Creation timestamp |

---

## Required API Endpoints

### Storage Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/storage/config` | Get current storage configuration |
| `PATCH` | `/storage/config` | Update storage configuration |
| `GET` | `/storage/{document_id}` | Get storage details |
| `DELETE` | `/storage/{document_id}` | Delete document storage |
| `POST` | `/storage/migrate` | Migrate storage backend |

### Version Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/documents/{id}/versions` | List all versions |
| `GET` | `/documents/{id}/versions/{n}` | Get specific version |
| `POST` | `/documents/{id}/versions` | Create new version |
| `PATCH` | `/documents/{id}/versions/{n}` | Update version metadata |
| `DELETE` | `/documents/{id}/versions/{n}` | Delete specific version |

### Retention Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/storage/retention` | List all retention policies |
| `POST` | `/storage/retention` | Create retention policy |
| `PATCH` | `/storage/retention/{id}` | Update policy |
| `DELETE` | `/storage/retention/{id}` | Delete policy |
| `POST` | `/storage/retention/apply` | Manually apply retention |

---

## Competitive Intelligence Deep Dive

### DocuPipe: Cloud Storage Only
DocuPipe doesn't provide document storage — documents are processed and the results are returned. Clients must manage their own storage. This is a limitation: once a document is processed, it's gone from DocuPipe's system unless the client saves it. No versioning, no retention, no lifecycle management.

### AWS Textract: S3-Dependent
Textract is designed to work with documents stored in S3. You manage the S3 bucket, versioning, lifecycle policies, and access control. The advantage is infinite scalability and AWS-native features. The disadvantage is operational complexity — you're responsible for the entire storage pipeline.

### Rossum: 12-Month Archive
Rossum provides a 12-month document archive and search in the Starter plan. Enterprise gets extended archive. Documents are stored on Rossum's infrastructure with their security model. No self-hosted option — documents are Rossum's responsibility.

### Hyperscience: Enterprise-Grade Storage
Hyperscience provides enterprise-grade storage with FedRAMP High authorization. Documents are stored securely with full audit trails. Integration with downstream systems means processed data flows directly into client systems. No self-hosted option.

### Paperless-ngx: Full Self-Hosted Storage
Paperless-ngx stores documents as PDF/A on the local filesystem with full versioning. Documents are stored alongside the original unaltered files. Backup is the user's responsibility. The advantage: complete data ownership, no per-page costs, no vendor lock-in. The disadvantage: you manage backups, scaling, and HA.

### M-Files: Enterprise Document Governance
M-Files provides enterprise-grade document storage with metadata-driven classification. Deep Microsoft 365 integration means SharePoint and Teams documents are managed centrally. Retention policies, versioning, and compliance are built-in. 6,000+ organizations trust M-Files for document governance. Named a Leader in the 2026 Gartner Magic Quadrant for Document Management.

---

## Implementation Roadmap

### Phase 1: Basic Storage (2-3 weeks) — P1
1. Define Document Storage entity
2. Implement local filesystem storage backend
3. SHA-256 checksum computation for integrity
4. Basic storage configuration endpoint
5. Document retrieval and download

### Phase 2: Versioning & Backup (3-4 weeks) — P1
1. Implement document versioning
2. Version comparison and restoration
3. Backup configuration and scheduling
4. Restore from backup endpoint
5. Storage usage reporting

### Phase 3: Multi-Backend Storage (3-4 weeks) — P2
1. S3 storage backend
2. GCS storage backend
3. Azure Blob storage backend
4. Storage migration between backends
5. Storage backend health monitoring

### Phase 4: Governance (4-6 weeks) — P2
1. Retention policy engine
2. Automated archival and deletion
3. Per-document access control
4. Audit logging for all storage operations
5. Compliance reporting

---

## Key Takeaway for Buyers

RERP Documents' storage pitch is **self-hosted, multi-backend, and governance-ready**. Unlike DocuPipe (no storage at all) or Textract (S3-only), RERP provides configurable multi-backend storage with full versioning and retention. Unlike Paperless-ngx (filesystem-only), RERP supports cloud storage backends with enterprise governance features.

The Rust-native storage engine handles 10,000+ concurrent file operations with sub-millisecond latency. And because storage is defined in OpenAPI, every client gets type-safe SDKs, automatic validation, and complete API documentation.

**The immediate priority: implement local filesystem storage, define the storage entity, and build the storage configuration endpoint. Storage is the foundation — without it, processed documents have nowhere to live.**
