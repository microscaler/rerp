# PRD-007: Operational Concerns & Migration Strategy

## Meta

- **Status:** Draft
- **Author:** Engineering Design
- **Created:** 2026-05-11
- **Related:** All 10 components
- **Priority:** P1 — Required for production readiness
- **Blocks:** Production deployment, monitoring, observability

## Problem

The component designs have **no operational concerns** addressed:

1. **No health check endpoints** — No `/health` or `/ready` endpoints for load balancers and orchestration
2. **No database migration strategy** — No schema versioning, no backward compatibility for entity field changes, no migration tooling
3. **No data seeding strategy** — No mechanism for initial document types, schemas, and templates on fresh install
4. **No graceful degradation** — No defined behavior when OCR fails, storage is full, or extraction schema is missing
5. **No monitoring/observability** — No metrics endpoint, no structured logging format, no tracing support
6. **No schema evolution strategy** — Adding/removing entity fields breaks existing API consumers

## Solution

### 1. Health Check Endpoints

Every microservice/component MUST implement these endpoints:

| Method | Endpoint | Response | Purpose |
|--------|----------|----------|---------|
| `GET` | `/health` | `{"status": "ok"}` | Liveness probe (is the process alive?) |
| `GET` | `/ready` | `{"status": "ok"}` or `{"status": "error", "checks": {...}}` | Readiness probe (are dependencies healthy?) |

**`/ready` checks:**

| Check | Target | Failure Signal |
|-------|--------|----------------|
| `database` | PostgreSQL connection | Error: "database connection failed" |
| `storage_backend` | Local filesystem or S3 | Error: "storage backend unreachable" |
| `search_engine` | Elasticsearch/Meilisearch | Error: "search engine unreachable" (not fatal for basic ops) |
| `ocr_engine` | Tesseract binary | Error: "OCR engine not found" |

**Example `/ready` response when all healthy:**
```json
{
  "status": "ok",
  "checks": {
    "database": "ok",
    "storage_backend": "ok",
    "search_engine": "ok",
    "ocr_engine": "ok"
  },
  "version": "1.0.0",
  "uptime_seconds": 86400
}
```

**Example `/ready` response with degraded search:**
```json
{
  "status": "degraded",
  "checks": {
    "database": "ok",
    "storage_backend": "ok",
    "search_engine": "error",
    "ocr_engine": "ok"
  },
  "version": "1.0.0",
  "uptime_seconds": 86400
}
```

### 2. Database Migration Strategy

**Tool:** Use SQL migrations (not ORM auto-migrate). Each migration is a SQL file in `migrations/` directory.

**Naming convention:** `{sequence}_{description}.sql`
- `001_create_tenant_table.sql`
- `002_create_user_table.sql`
- `003_add_document_pages_column.sql`
- `004_create_event_log_table.sql`

**Migration direction:**
- `UP` — forward migration (apply changes)
- `DOWN` — reverse migration (rollback, optional but recommended)

**Schema versioning:**

```sql
-- Migration 001: Create schema version tracker
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TIMESTAMP DEFAULT NOW(),
    description TEXT,
    checksum VARCHAR(64)  -- SHA-256 of migration file for integrity
);

-- On each migration apply, INSERT the version
-- On rollback, DELETE the version
```

**Backward compatibility rules for schema changes:**
1. **Adding columns:** MUST have default value OR be nullable. New default applies to all existing rows.
2. **Removing columns:** MUST rename to `_deprecated_{name}` first, keep for one release cycle, then drop.
3. **Changing column type:** MUST create new column, migrate data, swap, drop old column.
4. **Adding enums:** MUST add new values to end of enum list. Never reorder or remove enum values.
5. **Changing enum values:** NEVER remove or rename enum values. Add new values and migrate data in a separate migration.

### 3. Data Seeding Strategy

On first install, the system MUST seed these baseline resources:

**Seed Script: `seeds/001_document_types.sql`**
```sql
-- Pre-populate common document types (hierarchical)
INSERT INTO document_type (id, name, parent_id, description, icon, color) VALUES
    ('doc-invoice', 'Invoice', NULL, 'Supplier invoices and billing documents', 'receipt', '#2196F3'),
    ('doc-receipt', 'Receipt', NULL, 'Purchase receipts and expense records', 'receipt', '#4CAF50'),
    ('doc-contract', 'Contract', NULL, 'Legal agreements and contracts', 'gavel', '#FF9800'),
    ('doc-id', 'Identity Document', NULL, 'Passports, driver licenses, national IDs', 'person', '#9C27B0'),
    ('doc-invoice', 'Supplier Invoice', 'doc-invoice', 'Invoices from suppliers', 'receipt', '#1E88E5'),
    ('doc-invoice', 'Customer Invoice', 'doc-invoice', 'Invoices sent to customers', 'receipt', '#1565C0'),
    ('doc-receipt', 'Expense Receipt', 'doc-receipt', 'Employee expense receipts', 'receipt', '#388E3C');
```

**Seed Script: `seeds/002_default_admin.sql`**
```sql
-- Create default admin user (password set via env variable on first run)
INSERT INTO user (id, tenant_id, email, password_hash, role, is_active) VALUES
    ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000000', 'admin@localhost', '$2b$12$...', 'ADMIN', true);
```

**Seed Script: `seeds/003_default_storage.sql`**
```sql
-- Create default local storage backend
INSERT INTO storage_backend_config (id, name, type, bucket_or_path, is_active) VALUES
    ('00000000-0000-0000-0000-000000000001', 'Local Storage', 'LOCAL', '/var/lib/rerp/documents', true);
```

### 4. Graceful Degradation (Expanded)

Detailed failure mode handling:

#### OCR Engine Unavailable
- **Detection:** `Tesseract` binary not found or OCR POST returns 500
- **Impact:** Document stays in QUEUED status, not PROCESSING
- **User visible:** "OCR engine unavailable. Document queued for processing."
- **Recovery:** Service restart re-checks OCR engine availability
- **Monitoring:** Health check `/ready` returns `ocr_engine: error`

#### Storage Backend Full
- **Detection:** Disk space < 10% remaining
- **Impact:** New document uploads return 503 Service Unavailable
- **User visible:** "Storage full. Please contact administrator."
- **Recovery:** Automated archival of old documents based on retention policy
- **Monitoring:** Alert at 80% usage, critical at 90%

#### Search Engine Down
- **Detection:** Health check `search_engine` returns error
- **Impact:** Document processing continues, search indexing is queued
- **User visible:** Search returns partial results (documents indexed before failure)
- **Recovery:** Background worker retries indexing every 60 seconds
- **Monitoring:** Alert on search engine connection failure

#### Extraction Schema Missing
- **Detection:** ExtractionResult.schema_id references non-existent schema
- **Impact:** Extraction step skipped, document proceeds to storage
- **User visible:** "No extraction schema found. Document stored without data extraction."
- **Recovery:** Admin creates schema, triggers re-extraction
- **Monitoring:** Alert on extraction failures due to missing schema

#### Event Bus Down
- **Detection:** Event dispatcher process crashed or PostgreSQL NOTIFY channel full
- **Impact:** Events written to `event_log` table with status='pending'
- **User visible:** None (internal failure, documents still processed)
- **Recovery:** Restart event dispatcher, it replays all pending events from `event_log`
- **Monitoring:** Alert on event_log.pending count > 100

### 5. Monitoring & Observability

#### Structured Logging Format

All components MUST use JSON structured logging:

```json
{
  "timestamp": "2026-05-11T22:00:00Z",
  "level": "INFO",
  "component": "document-ingestion",
  "request_id": "uuid",
  "tenant_id": "uuid",
  "event": "document.ingested",
  "document_id": "uuid",
  "filename": "invoice.pdf",
  "file_size": 12345,
  "duration_ms": 42.5
}
```

**Log levels:**
- DEBUG — verbose internal state (only in dev)
- INFO — normal operations (document.ingested, extraction.completed)
- WARN — recoverable issues (low confidence, retry scheduled)
- ERROR — failures requiring attention (extraction failed, storage unavailable)
- CRITICAL — system-level failures (database down, all storage backends down)

#### Metrics Endpoint

Every component MUST expose `/metrics` in Prometheus format:

```
# HELP rerp_documents_documents_total Total documents processed
# TYPE rerp_documents_documents_total counter
rerp_documents_documents_total{status="completed"} 1234
rerp_documents_documents_total{status="failed"} 42

# HELP rerp_documents_ocr_duration_seconds OCR processing time
# TYPE rerp_documents_ocr_duration_seconds histogram
rerp_documents_ocr_duration_seconds_bucket{le="0.5"} 1000
rerp_documents_ocr_duration_seconds_bucket{le="1.0"} 1200
rerp_documents_ocr_duration_seconds_bucket{le="+Inf"} 1234

# HELP rerp_documents_events_pending Events waiting in queue
# TYPE rerp_documents_events_pending gauge
rerp_documents_events_pending 3
```

**Standard metrics per component:**
- `rerp_{component}_documents_total` — count by status
- `rerp_{component}_duration_seconds` — histogram of processing times
- `rerp_{component}_events_pending` — gauge of pending events
- `rerp_{component}_errors_total` — count by error code

#### Request ID Propagation

Every HTTP request gets a unique `request_id` UUID that is:
1. Generated on entry (or passed in `X-Request-ID` header)
2. Logged in all structured log entries
3. Propagated across event bus events (in `correlation_id` field)
4. Included in all response `meta.request_id` fields

This enables full request tracing across components.

### 6. Schema Evolution Strategy

When adding/removing entity fields, follow this process:

1. **Add migration file** — SQL file that adds the column with a default value
2. **Update canonical entity registry** — Add the new field to the entity definition
3. **Update component README** — Document the new field in the entity table
4. **Update OpenAPI spec** — Add the new field to the schema
5. **Add migration note** — If the change requires data migration, document it in the migration file

**Backward compatibility guarantee:** The API always accepts requests with unknown fields (ignored) and always returns consistent response shapes regardless of schema version. New fields are optional in the schema and default to null/empty.

## Acceptance Criteria

- [ ] `/health` and `/ready` endpoints implemented in all 10 components
- [ ] `/ready` checks for database, storage backend, search engine, OCR engine
- [ ] Migration directory structure defined with naming convention
- [ ] `schema_version` table defined in canonical entity registry
- [ ] Backward compatibility rules documented (no column drops without deprecation)
- [ ] Seed scripts created for document types, admin user, default storage
- [ ] Graceful degradation strategies documented for all 5 failure modes
- [ ] Structured logging format defined and documented
- [ ] `/metrics` endpoint specification defined with standard metrics
- [ ] Request ID propagation strategy documented across all components
