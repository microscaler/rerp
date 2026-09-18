# PRD-005: Cross-Component Data Flow & Integration Architecture

## Meta

- **Status:** Draft
- **Author:** Engineering Design
- **Created:** 2026-05-11
- **Related:** All 10 components
- **Priority:** P0 — Critical for avoiding data silos
- **Blocks:** Implementation of multi-component workflows

## Problem

Components are designed in isolation with no cross-component integration architecture:

1. **No document lifecycle diagram** — No visual representation of how a document flows through the system
2. **No event system connecting stages** — `ProcessingQueue.stage` is a dangling bridge entity
3. **No shared state between components** — Storage doesn't know about OCR, extraction doesn't know about classification
4. **No graceful degradation** — What happens when OCR fails? When storage is full? When extraction schema is missing?
5. **No data propagation strategy** — How does extracted data get from extraction to search indexing to workflow routing?

### Current State (Broken)

```
Ingestion creates Document
    │
    ├─→ OCR reads Document? (race condition — ingestion may not have committed)
    │
    ├─→ Classification needs Document? (no trigger mechanism)
    │
    ├─→ Extraction needs Classification? (no data flow)
    │
    ├─→ Storage needs Extraction? (no trigger)
    │
    └─→ Search needs Extraction? (no trigger)
```

Each component has its own "Key Takeaway" that's self-contained. There's no "how does this connect to everything else?"

## Solution

### 1. Document Lifecycle Diagram

Every document goes through this lifecycle:

```
                            ┌─────────────────────────────────────────────────┐
                            │                   DOCUMENT LIFECYCLE             │
                            │                                                  │
   UPLOAD ──────────────▶ [ QUEUED ]                                          │
      │                      │                                                 │
      │                      ▼                                                 │
      │              [ OCR IN PROGRESS ] ──(OCR FAILS)──▶ [ FAILED ]           │
      │                      │                                                 │
      │                      ▼                                                 │
      │            [ CLASSIFY IN PROGRESS ] ──(CLASSIFY FAILS)──▶ [ FAILED ]   │
      │                      │                                                 │
      │                      ▼                                                 │
      │              [ EXTRACT IN PROGRESS ] ──(EXTRACT FAILS)──▶ [ FAILED ]   │
      │                      │                                                 │
      │                      ▼                                                 │
      │            [ STORAGE IN PROGRESS ] ──(STORAGE FAILS)──▶ [ FAILED ]     │
      │                      │                                                 │
      │                      ▼                                                 │
      │                  [ COMPLETED ]                                          │
      │                      │                                                 │
      │                      ├──▶ SEARCH INDEXED (automatic)                   │
      │                      ├──▶ WORKFLOW TRIGGERED (if configured)           │
      │                      └──▶ METRICS RECORDED (automatic)                 │
      │                                                                      │
      └──────────────────────────────────────────────────────────────────────┘
```

### 2. Data Propagation Matrix

When a component completes, it MUST propagate its output to these downstream consumers:

| Component Completion | Propagates To | How | Data Propagated |
|---------------------|---------------|-----|-----------------|
| `Document` created | `ProcessingJob` | Event: `document.ingested` | document_id, source, checksum |
| `OcrResult` created | `SearchIndex` | Event: `ocr.completed` | searchable_content, page_number |
| `DocumentClassification` created | `ExtractionSchema` | Event: `classification.completed` | document_type_id → schema lookup |
| `ExtractionResult` created | `SearchIndex`, `Workflow`, `Reporting` | Event: `extraction.completed` | extracted_data, confidence_scores |
| `DocumentStorage` created | `AuditLog` | Event: `storage.saved` | storage_backend, storage_path |
| `SearchIndex` updated | `AuditLog` | Event: `search.indexed` | indexed_at, content_hash |
| `WorkflowExecution` completed | `Reporting` | Event: `workflow.completed` | duration_ms, steps_completed |
| Any failure | `AuditLog`, `Reporting` | Event: `*.failed` | error_message, component |

### 3. Graceful Degradation Strategies

Each component MUST handle failure modes without bringing down the entire pipeline:

#### OCR Failure
- **What happens:** OCR engine returns error or times out
- **Fallback:** Document status → FAILED, error_message populated
- **Recovery:** Admin can retry: `POST /documents/{id}/retry`
- **Data preserved:** Document and original file remain; OCR result is simply not created

#### Classification Failure
- **What happens:** Classification model unavailable or returns null
- **Fallback:** Document stays in CLASSIFY_IN_PROGRESS, not blocked from extraction
- **Recovery:** Manual classification via UI or API, or default to "uncategorized" type
- **Data preserved:** Document proceeds to extraction with no type assigned

#### Extraction Failure
- **What happens:** Extraction engine returns error or schema not found
- **Fallback:** Document proceeds to storage WITHOUT extraction (document still stored)
- **Recovery:** Re-extraction: `POST /extraction-results/{id}/re-extract`
- **Data preserved:** Document and OCR result preserved; extraction skipped

#### Storage Failure
- **What happens:** Storage backend unreachable (S3 down, disk full)
- **Fallback:** Document marked as FAILED, queued for retry when backend recovers
- **Recovery:** Automatic retry after 5 minutes, then 15 minutes, then 1 hour (exponential backoff, max 3 retries)
- **Data preserved:** Document metadata and OCR result preserved in DB; file storage deferred

#### Search Index Failure
- **What happens:** Search engine (Elasticsearch/Meilisearch) unreachable
- **Fallback:** Document processing continues; search index update queued in `event_log`
- **Recovery:** Background worker retries search indexing every 60 seconds until success
- **Data preserved:** Document processing NOT blocked by search failure

### 4. Shared State: The Document Event Log

Add a `DocumentEventLog` entity that serves as the canonical audit trail of every document's journey through the pipeline:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `document_id` | Foreign Key: Document | Yes | Source document |
| `event_type` | Enum: [INGESTED, OCR_STARTED, OCR_COMPLETED, CLASSIFY_STARTED, CLASSIFY_COMPLETED, EXTRACT_STARTED, EXTRACT_COMPLETED, STORAGE_STARTED, STORAGE_COMPLETED, SEARCH_INDEXED, WORKFLOW_STARTED, WORKFLOW_COMPLETED, FAILED] | Yes | Pipeline stage |
| `component` | String (64) | Yes | Component that emitted the event |
| `status` | Enum: [STARTED, COMPLETED, FAILED] | Yes | Event outcome |
| `details` | JSONB | No | Stage-specific details (ocr confidence, schema_id, storage_backend, etc.) |
| `created_at` | DateTime | Yes | When event occurred |

This is NOT the same as `AuditLog` (which tracks user actions and security events). `DocumentEventLog` tracks the internal pipeline state of each document.

### 5. Schema-to-Workflow Data Mapping

When a classification completes and identifies a document type, the system MUST look up the associated extraction schema and trigger extraction:

```sql
-- This query runs when classification completes
SELECT es.schema_id, es.schema
FROM extraction_schema es
JOIN document_type dt ON dt.extraction_schema_id = es.id
WHERE dt.id = $1 AND es.is_active = true;
```

The result feeds into the extraction engine, which uses the JSON Schema to guide field extraction.

### 6. Component Dependency Graph

Explicit dependency ordering for implementation:

```
Phase 1 (Foundational):
  ┌─────────────────────────────────────────┐
  │ Tenant, User, Team (PRD-001 entities)   │
  │ Document (canonical entity)             │
  │ ProcessingJob (event-driven)            │
  │ EventLog (PRD-002)                      │
  │ DocumentEventLog (new)                  │
  └─────────────────────────────────────────┘
         │
Phase 2 (Core Pipeline):
  ┌─────────────────────────────────────────┐
  │ Document Ingestion (creates Document)   │
  │ OCR Extraction (reads Document,         │
  │   creates OcrResult)                    │
  │ Storage Management (stores document)    │
  └─────────────────────────────────────────┘
         │
Phase 3 (Intelligence):
  ┌─────────────────────────────────────────┐
  │ Classification (reads OcrResult,        │
  │   creates DocumentClassification)        │
  │ Data Extraction (reads OcrResult +      │
  │   Classification, creates Extraction    │
  │   Result)                               │
  │ Search Discovery (reads OcrResult +     │
  │   Extraction Result, creates SearchIndex)│
  └─────────────────────────────────────────┘
         │
Phase 4 (Automation):
  ┌─────────────────────────────────────────┐
  │ Workflow Automation (reads Extraction   │
  │   Result, creates WorkflowExecution)     │
  └─────────────────────────────────────────┘
         │
Phase 5 (Governance):
  ┌─────────────────────────────────────────┐
  │ Security & Compliance (all components)  │
  │ Integration & API (all components)      │
  │ Reporting & Analytics (all components)  │
  └─────────────────────────────────────────┘
```

## Acceptance Criteria

- [ ] Document lifecycle diagram documented in PRD-005
- [ ] Data propagation matrix defined with all 8 propagation rules
- [ ] `DocumentEventLog` entity defined in canonical entity registry
- [ ] Graceful degradation strategies defined for all 5 failure modes
- [ ] Component dependency graph shows phased implementation order
- [ ] Schema-to-workflow data mapping query documented
- [ ] Each component README references the lifecycle diagram and its position in it
