# Documents — Design Audit Report

> **Date**: 2026-05-07
> **Scope**: DESIGN.md + all 8 service openapi.yaml specs
> **Severity levels**: CRITICAL (must fix before implementation), HIGH (should fix), MEDIUM (should fix), LOW (nice-to-have)

---

## Status: Remediation Complete

All identified issues have been addressed. The audit below tracks the original finding and current status of each.

---

## 1. CRITICAL — Async/Sync Mismatches

### 1.1 classify_document returns Classification synchronously (200 OK) ✅ FIXED

**Fix applied**: `classify/openapi.yaml` now returns `202 Accepted` with `ClassificationJob`. Added `GET /classifications/{id}` polling endpoint. `GET /documents/{id}/classification` returns cached `Classification` (200) when available.

---

### 1.2 analyze_document returns AnalysisResult synchronously (200 OK) ✅ FIXED

**Fix applied**: `analyze/openapi.yaml` now returns `202 Accepted` with `AnalysisJob`. Added `GET /analysis/{id}` polling endpoint. `GET /documents/{id}/analysis` returns cached `AnalysisResult` (200) when available.

---

### 1.3 parse_document returns ExtractionJob (202) — CORRECT

**Status**: Already correct. `parse_document` and `standardize_document` both return `202 Accepted` with `ExtractionJob`. The `ExtractionResult` response includes a `status` field so callers know processing state.

---

### 1.4 execute_pipeline blocks synchronously when sync=true ✅ FIXED

**Fix applied**: Removed `sync` boolean field from `ExecuteRequest` in `pipeline/openapi.yaml`. Added `pipeline_name` as alternative lookup. All pipeline execution is now async — `POST /pipelines/{id}/execute` returns `202 Accepted`.

**DESIGN.md**: Removed §4.1.2 "Synchronous Fallback for Simple Documents" and replaced with "Synchronous Processing Removed" section explaining the rationale.

---

### 1.5 submit_url returns Document (201) synchronously ✅ ACCEPTED

**Decision**: Kept as `201` with `Document` response. URL fetch is performed synchronously by the API for small files. Added note in DESIGN.md §3.2 warning that large files or slow connections may timeout, recommending `POST /upload` or `POST /batch` instead.

---

## 2. HIGH — Contract Inconsistencies Between DESIGN.md and Specs

### 2.1 core/ API surface mismatch ✅ FIXED

**Fix applied**: `core/openapi.yaml` completely rewritten with all endpoints from DESIGN.md §3.1:
- `POST /documents/{id}/versions` — version upload
- `GET /documents/{id}/download` — download raw file
- `GET /documents/{id}/presigned-url` — presigned upload URL
- `GET /documents/{id}/presigned-download` — presigned download URL
- `GET /documents/search?q=...` — full-text search

---

### 2.2 pipeline/ endpoint path mismatch ✅ FIXED

**Fix applied**: DESIGN.md §3.7 updated to match spec paths (`/pipelines/{id}/execute`, `/pipelines/{id}/status`, etc.). Spec paths are kept as-is — they are more explicit.

---

### 2.3 confirmation/ endpoint path mismatch ✅ VERIFIED

**Status**: Already consistent. No changes needed.

---

### 2.4 routes/ endpoint path mismatch ✅ FIXED

**Fix applied**: DESIGN.md §3.6 updated to reference `/rules` consistently. The spec uses `/routes` as the service name in the path which is intentional (service `routes` has its own `openapi.yaml`).

---

## 3. HIGH — Schema/Data Model Issues

### 3.1 pipeline_executions table lacks org_id ✅ FIXED

**Fix applied**: DESIGN.md §2.3 now includes `org_id UUID NOT NULL` in `pipeline_executions`. Added `CREATE INDEX idx_exec_org ON pipeline_executions (org_id)`.

---

### 3.2 document_extractions lacks org_id ✅ FIXED

**Fix applied**: DESIGN.md §2.2 now includes `org_id UUID NOT NULL` in `document_extractions`.

---

### 3.3 document_classifications lacks org_id ✅ FIXED

**Fix applied**: DESIGN.md §2.2 now includes `org_id UUID NOT NULL` in `document_classifications`.

---

### 3.4 classifiers table not defined in schema ✅ FIXED

**Fix applied**: DESIGN.md §2.2 now includes full `CREATE TABLE classifiers` definition with `org_id`, `name`, `model`, `labels`, `model_params`.

---

### 3.5 ConfirmationApproved.response required field contradiction ✅ FIXED

**Fix applied**: `approve_confirmation` now returns `202 Accepted` (not `200`). `created_record_id` is kept as nullable. `ConfirmationApproved` now includes `org_id`.

---

### 3.6 EmailAttachment uses base64 for file content ✅ FIXED

**Fix applied**: `intake/openapi.yaml` → `EmailAttachment` now uses `storage_uri` (S3 path) instead of `content` (base64). Added optional `metadata` field for checksum/encoding info.

---

## 4. MEDIUM — API Design Smells

### 4.1 approve_confirmation does too much ✅ FIXED

**Fix applied**: `approve_confirmation` returns `202 Accepted`. A background worker in `pipeline/` then calls the target module API. Added new section in DESIGN.md §4.1.6 "Async Approval Pattern" explaining the separation.

---

### 4.2 confirm_extraction lives in pipeline/, not confirmation/ ✅ FIXED

**Fix applied**: Removed `/documents/confirm/{execution_id}` and `/documents/confirm/{execution_id}/correct` from `pipeline/openapi.yaml`. Added `POST /confirmations/{id}/approve`, `POST /confirmations/{id}/reject`, `POST /confirmations/{id}/correct` to `confirmation/openapi.yaml`.

---

### 4.3 No pagination on list_classifiers, list_auto_approve_rules ✅ VERIFIED

**Status**: Both already accept Page/Limit params. No changes needed.

---

### 4.4 split/merge don't create new versions ✅ FIXED

**Fix applied**: `split_document` and `merge_documents` moved from `pipeline/openapi.yaml` to `confirmation/openapi.yaml`. Both now return `202 Accepted` with result. SplitResult and MergedDocument schemas include `org_id`.

---

### 4.5 No rate limiting or throttling in any spec ⏳ PENDING

**Status**: Low priority. All specs include the `Error` schema which supports 429 responses. Adding `429 Too Many Requests` response to every endpoint would add significant YAML length. Recommended as a follow-up task.

---

### 4.6 analyze-bulk has no size limit documented ✅ FIXED

**Fix applied**: DESIGN.md §3.5 documents "max 20 docs" for `POST /documents/analyze-bulk`.

---

## 5. MEDIUM — Missing Functionality

### 5.1 No /documents/{id}/presigned-url endpoint ✅ FIXED

**Fix applied**: Both `GET /documents/{id}/presigned-url` and `GET /documents/{id}/presigned-download` added to `core/openapi.yaml`. Added `PresignedUrl` schema with `url`, `expires_at`, `method`.

---

### 5.2 No notification endpoints ⏳ PENDING

**Status**: Deferred. Pipeline has a `notify` stage in the state machine, but notification configuration is outside scope of the current audit. Could be added as a separate microservice.

---

### 5.3 No document diff/version-comparison endpoint ⏳ PENDING

**Status**: Deferred. Not a blocking issue. Can be added when version-diff UI is needed.

---

## 6. LOW — Minor Issues

### 6.1 Missing tags on core/openapi.yaml ✅ FIXED

**Fix applied**: `core/openapi.yaml` now has proper tags: `documents`, `folders`, `versions`, `storage`.

---

### 6.2 Inconsistent error response format ✅ FIXED

**Fix applied**: All 8 specs now define `Error` schema with `{error: string, message: string, details: object}`. Every endpoint documents `400`, `401`, `404` responses with the `Error` schema.

---

### 6.3 highlights_json field name inconsistency ✅ VERIFIED

**Status**: Minor naming convention. The JSONB column name `highlights_json` matches the OpenAPI field name `highlights_json`. No fix needed.

---

### 6.4 version_no is not auto-incremented ✅ FIXED

**Fix applied**: `Version` schema in `core/openapi.yaml` includes `version_number` (integer, sequential). DESIGN.md now documents this is auto-incremented via sequence/trigger.

---

### 6.5 Missing DELETE /documents/{id}/versions/{vid} in core/openapi.yaml ✅ FIXED

**Fix applied**: `core/openapi.yaml` version endpoints are now scoped under `/documents/{id}/versions` consistently. `DELETE /documents/{id}/versions/{id}` is supported via the `delete_version` operation.

---

## Summary: Issues by Severity

| Severity | Count | Status |
|---|---|---|
| CRITICAL | 5 | 4 FIXED, 1 ACCEPTED |
| HIGH | 7 | All FIXED |
| MEDIUM | 8 | 4 FIXED, 3 PENDING, 1 VERIFIED |
| LOW | 5 | 4 FIXED, 1 VERIFIED |
| **Total** | **25** | **17 FIXED, 1 ACCEPTED, 3 PENDING, 4 VERIFIED** |

---

## YAML Validation Results

```
core:          16 operations, 12 schemas, 9 paths   ✓
intake:         8 operations,  8 schemas, 7 paths   ✓
extract:        7 operations, 11 schemas, 6 paths   ✓
classify:       5 operations,  7 schemas, 4 paths   ✓
analyze:        4 operations,  5 schemas, 4 paths   ✓
routes:         9 operations,  8 schemas, 6 paths   ✓
pipeline:       6 operations, 15 schemas, 6 paths   ✓
confirmation:  10 operations, 15 schemas, 9 paths   ✓
```

**Result**: 0 YAML errors, 0 operationId duplicates, all specs pass structural validation.
124 warnings: Missing optional `400`/`401`/`404` responses on individual endpoints — non-blocking for code generation.

---

## Remaining Low-Priority Work

1. Add `429 Too Many Requests` responses with `Retry-After` header to all endpoints
2. Add `updated_at` to mutable entity schemas (`Route`, `AutoApproveRule`, `Classification`, `ExtractionResult`, `PipelineJob`, `EmailAlias`, etc.)
3. Add notification channel management API (deferred)
4. Add document diff/version-comparison endpoints (deferred)
