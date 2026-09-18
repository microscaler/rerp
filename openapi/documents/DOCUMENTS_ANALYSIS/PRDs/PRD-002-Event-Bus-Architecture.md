# PRD-002: Cross-Component Event Bus Architecture

## Meta

- **Status:** Draft
- **Author:** Engineering Design
- **Created:** 2026-05-11
- **Related:** All 10 components
- **Priority:** P0 — Critical for multi-component data flow
- **Blocks:** Implementation of all components beyond ingestion

## Problem

The 10 components are designed as **silos**. There is no architecture for how they communicate:

- Document Ingestion creates a `Document` and puts it in a `ProcessingJob` (pull-based polling)
- OCR Extraction needs to know when a document is ready for processing
- Classification needs to know when OCR is complete
- Extraction needs to know when classification is done
- Storage needs to know when extraction is complete
- Search Index needs to be updated after extraction
- Workflow needs to know when extraction is complete

The only bridge is `ProcessingJob.stage` in the ingestion component — a **pull-based** polling mechanism. This creates:

1. **Polling loops** — Components must repeatedly check if work is ready (wastes DB connections)
2. **Race conditions** — If OCR reads a Document before ingestion commits it, it gets nothing
3. **No event replay** — If OCR crashes, there's no way to re-process documents that were ready
4. **Tight coupling** — Storage must know about OCR, extraction must know about classification
5. **No async callbacks** — Webhooks fire on completion, but the internal pipeline is synchronous

### Current Flow (Pull-Based)

```
Ingestion → POST /documents → returns jobId
OCR → polls GET /queue/{job_id} every 2s → waits for stage=OCR
Classification → polls GET /queue/{job_id} every 2s → waits for stage=CLASSIFY
Extraction → polls GET /queue/{job_id} every 2s → waits for stage=EXTRACT
```

This is the DocuPipe pattern (upload→jobId→poll). It works for the **external API**, but internally the components need an event-driven architecture for reliability and scalability.

## Solution

Implement a **document processing event bus** that decouples components while maintaining a clear pipeline flow.

### Architecture Decision: Hybrid Model

Use PostgreSQL LISTEN/NOTIFY for **immediate intra-component communication** (same deployment), with a dead-letter queue for failures. For **cross-deployment** (e.g., OCR microservice separate from ingestion microservice), use a message queue.

### Event Types

| Event | Source | Target | Payload |
|-------|--------|--------|---------|
| `document.ingested` | Document Ingestion | Processing Job Manager | document_id, source, checksum_sha256 |
| `job.queued` | Processing Job Manager | OCR | job_id, document_id, stage=OCR |
| `ocr.completed` | OCR | Classification | job_id, document_id, ocr_result_ids[], confidence, model_version |
| `ocr.failed` | OCR | Processing Job Manager | job_id, error_message, retry_count |
| `classification.completed` | Classification | Extraction | job_id, document_id, document_type_ids[], confidence |
| `extraction.completed` | Extraction | Workflow, Search | job_id, document_id, extraction_result_id |
| `extraction.failed` | Extraction | Workflow (exception handler) | job_id, error_message, confidence_scores |
| `storage.saved` | Storage Management | Audit Log | document_id, storage_backend, storage_path, checksum |
| `search.indexed` | Search Discovery | Audit Log | document_id, indexed_at, content_hash |
| `workflow.completed` | Workflow Automation | Reporting | job_id, document_id, duration_ms, steps_completed |
| `workflow.failed` | Workflow Automation | Audit Log | job_id, error_message |

### Event Schema (Common Envelope Format)

All events MUST follow this envelope:

```json
{
  "event_id": "uuid",
  "event_type": "document.ingested",
  "event_version": "1.0",
  "timestamp": "2026-05-11T22:00:00Z",
  "source_component": "document-ingestion",
  "tenant_id": "uuid",
  "correlation_id": "uuid",
  "data": {
    "document_id": "uuid",
    ...
  }
}
```

### Event Bus Implementation

#### Option A: PostgreSQL LISTEN/NOTIFY (Recommended for Phase 1)

PostgreSQL's native LISTEN/NOTIFY is sufficient for in-process event routing within a single PostgreSQL instance:

- **Publisher**: Components execute `NOTIFY 'event_name', '<json_payload>'` after successful operations
- **Subscriber**: A dedicated event dispatcher service listens on channel 'document_events' and routes to subscribers
- **Reliability**: Use a `EventLog` table (similar to AuditLog) for durability. Every event is INSERTed into the table BEFORE NOTIFY fires. Subscribers ACK by UPDATEing the event row.

```sql
CREATE TABLE event_log (
    id UUID PRIMARY KEY,
    event_type VARCHAR(128) NOT NULL,
    source_component VARCHAR(64) NOT NULL,
    tenant_id UUID,
    correlation_id UUID,
    data JSONB NOT NULL,
    status VARCHAR(32) DEFAULT 'pending',
    delivery_count INT DEFAULT 0,
    next_retry_at TIMESTAMP,
    delivered_at TIMESTAMP,
    error_message TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);
```

#### Option B: Redis Streams (Phase 2+)

For microservice deployments where components run in separate processes/containers:

- Publisher: `XADD events:* <field> <value>`
- Subscriber: `XREAD GROUP consumers events -`
- ACK: `XACK events-group consumer <id>`
- Dead-letter: Events with 5+ failures moved to `events:dead-letter`

#### Dead Letter Queue

Events that fail delivery 5 times move to a dead letter table. Admin endpoint: `GET /admin/dead-letters` with retry option.

### Processing Job Model Update

Update the `ProcessingJob` entity from the current pull-based model to event-driven:

Current: `stage: Enum: [OCR, EXTRACT, CLASSIFY, STORAGE]`
New: `stage: Enum: [QUEUED, OCR_IN_PROGRESS, CLASSIFY_IN_PROGRESS, EXTRACT_IN_PROGRESS, STORAGE_IN_PROGRESS, COMPLETED, FAILED]`

Transitions are triggered by events:
```
QUEUED → [document.ingested] → OCR_IN_PROGRESS
OCR_IN_PROGRESS → [ocr.completed] → CLASSIFY_IN_PROGRESS
CLASSIFY_IN_PROGRESS → [classification.completed] → EXTRACT_IN_PROGRESS
EXTRACT_IN_PROGRESS → [extraction.completed] → STORAGE_IN_PROGRESS
STORAGE_IN_PROGRESS → [storage.saved] → COMPLETED
Any → [ocr.failed, extraction.failed, etc.] → FAILED
```

### Component Event Contract

Each component declares its events:

**Document Ingestion** publishes:
- `document.ingested` (after successful upload)

**OCR** subscribes to:
- `job.queued` (for stage=OCR)
OCR publishes:
- `ocr.completed` (when OCR finishes)
- `ocr.failed` (when OCR fails)

**Classification** subscribes to:
- `job.queued` (for stage=CLASSIFY)
Classification publishes:
- `classification.completed`
- `classification.failed`

**Extraction** subscribes to:
- `job.queued` (for stage=EXTRACT)
Extraction publishes:
- `extraction.completed`
- `extraction.failed`

**Storage** subscribes to:
- `job.queued` (for stage=STORAGE)
Storage publishes:
- `storage.saved`
- `storage.failed`

**Search** subscribes to:
- `extraction.completed` (when extraction finishes, rebuild search index)
Search publishes:
- `search.indexed` (when index update completes)

**Workflow** subscribes to:
- `document.ingested` (trigger workflow on new documents)
- `extraction.completed` (trigger approval workflows)
Workflow publishes:
- `workflow.completed`
- `workflow.failed`

**Reporting** subscribes to:
- `ocr.completed` (track OCR processing times)
- `extraction.completed` (track extraction quality)
- `workflow.completed` (track workflow performance)
- `classification.completed` (track classification accuracy)

### API Changes

Add event bus endpoints:

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/events` | List recent events with filtering |
| `GET` | `/events/dead-letters` | List dead-letter events (admin) |
| `POST` | `/events/dead-letters/{id}/retry` | Re-publish a dead-letter event |
| `GET` | `/events/types` | List available event types |
| `POST` | `/events/test` | Publish a test event (debug) |

### Migration Plan

1. Create `event_log` table (part of PRD-001 entity registry)
2. Update `ProcessingJob` stage enum to new values
3. Implement event dispatcher (PostgreSQL LISTEN/NOTIFY based)
4. Update each component to publish relevant events
5. Update each component to subscribe to relevant events
6. Replace polling with event-driven processing in Phase 2
7. Add dead-letter handling in Phase 3

## Acceptance Criteria

- [ ] `event_log` table defined in canonical entity registry
- [ ] All 11 event types defined with envelope schema
- [ ] ProcessingJob stage enum updated to event-driven transitions
- [ ] Each component has documented event subscriptions and publications
- [ ] Dead-letter queue exists for failed events
- [ ] Polling replaced with event-driven processing for all 5 stages
- [ ] Event log is queryable with filters by tenant, event_type, status, date range
- [ ] Admin endpoint for dead-letter event management
