# pipeline/ — Workflow Orchestration

**Path:** `documents/pipeline/`
**API:** `/api/v1/documents/pipeline/`

Orchestrates the multi-step processing flow. Reads from `core/`, triggers extract/classify/analyze services, and writes results back to `core/`.

## Pipeline Stages

1. **Ingest** — Write document to core/
2. **Parse** — OCR + Table Extraction
3. **Classify** — Determine Document Type
4. **Extract** — Schema-based Field Extraction
5. **Route** — Match Routing Rules
6. **Confirm** — User Verification
7. **Create** — API to Target Module
8. **Notify** — Alert Relevant Users

## API Surface

```
POST   /pipelines                      # Create pipeline definition
POST   /pipelines/{id}/execute         # Execute pipeline on document(s)
GET    /pipelines/{id}/status          # Check pipeline status
GET    /pipelines/{id}/result          # Get pipeline output
POST   /documents/confirm/{eid}        # User confirms/rejects extracted data
POST   /documents/confirm/{eid}/correct # User corrects extracted data
POST   /documents/split                # Split multi-page document
POST   /documents/merge                # Merge documents
GET    /pipeline/executions            # List all pipeline executions
GET    /pipeline/executions/{id}       # Get execution details with audit trail
```
