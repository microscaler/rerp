# PRD-003: API Standardization

## Meta

- **Status:** Draft
- **Author:** Engineering Design
- **Created:** 2026-05-11
- **Related:** All 10 components (API endpoints)
- **Priority:** P1 — Required before OpenAPI generation
- **Blocks:** SDK generation, developer experience

## Problem

API endpoints across components have inconsistent patterns:

- **Mixed URL styles:** `/documents/{id}` (resource-oriented) vs `/extract` (action-oriented) vs `/ocr/{id}` (mixed)
- **No pagination:** All list endpoints return unlimited results
- **No idempotency:** All POST endpoints are fire-and-forget
- **No status code documentation:** Responses don't specify 200/201/404/429
- **No versioning:** No strategy for API evolution
- **Inconsistent path params:** `{document_id}` vs `{id}` vs `{job_id}`

## Solution

Define a **unified API contract** that all components follow.

### 1. URL Naming Convention

**Use resource-oriented URLs.** Every noun is a resource. Actions are HTTP methods or sub-resources.

| Pattern | Example | Used By |
|---------|---------|---------|
| `GET /{resource}` | `GET /documents` | List resources |
| `GET /{resource}/{id}` | `GET /documents/{id}` | Get single resource |
| `POST /{resource}` | `POST /documents` | Create resource |
| `PATCH /{resource}/{id}` | `PATCH /documents/{id}` | Update resource |
| `DELETE /{resource}/{id}` | `DELETE /documents/{id}` | Delete resource |
| `POST /{resource}/{id}/action` | `POST /documents/{id}/validate` | Resource-specific action |
| `GET /{resource}/{id}/{child}` | `GET /documents/{id}/versions` | Nested collection |

**All endpoints MUST use `{id}` not `{resource_id}` in path parameters.**

### Current API Non-Conformances

| Component | Non-Conforming Endpoint | Fix |
|-----------|------------------------|-----|
| data-extraction | `POST /extract` | → `POST /extraction-results` |
| data-extraction | `POST /extract/schema/{id}` | → `POST /extraction-results?schema_id={id}` |
| data-extraction | `POST /extract/batch` | → `POST /extraction-results/batch` |
| data-extraction | `POST /extract/standardize` | → `POST /extraction-results/{id}/standardize` |
| data-extraction | `POST /extract/natural-language` | → `POST /extraction-results?mode=natural_language` |
| data-extraction | `POST /extract/table` | → `POST /extraction-results/table` |
| classification | `POST /classify` | → `POST /classifications` |
| classification | `POST /classify/batch` | → `POST /classifications/batch` |
| ocr-extraction | `POST /ocr/process` | → `POST /ocr-results` |
| ocr-extraction | `POST /ocr/process-page` | → `POST /ocr-results` |
| storage-management | `POST /storage/migrate` | → `POST /documents/{id}/migrate` |
| integration-api | `POST /search` | → `POST /search-results` |

### 2. Pagination

All list endpoints MUST support pagination:

```
GET /documents?page=1&limit=50&sort=created_at:desc
GET /documents?offset=0&limit=50
```

**Response format:**
```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "limit": 50,
    "total": 1234,
    "total_pages": 25,
    "has_next": true,
    "has_previous": false
  }
}
```

### 3. Idempotency

All POST endpoints MUST support idempotency keys:

```
POST /documents
Headers: Idempotency-Key: <uuid>
```

- The system stores `(idempotency_key, response, timestamp)` in a new `IdempotencyKey` table
- If the same key is received within 24 hours, return the cached response
- If the key is expired or missing, process normally and cache the result

**IdempotencyKey Entity:**

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `key` | String (64) | Yes | Idempotency key (unique) |
| `tenant_id` | Foreign Key: Tenant | Yes | Tenant scoping |
| `response_status` | Integer | Yes | HTTP status code returned |
| `response_headers` | JSONB | Yes | Response headers |
| `response_body` | JSONB | Yes | Response body |
| `created_at` | DateTime | Yes | Cache timestamp (expire after 24h) |

### 4. Standard Response Formats

**Success (200):**
```json
{
  "data": { ... },
  "meta": {
    "request_id": "uuid",
    "timestamp": "2026-05-11T22:00:00Z"
  }
}
```

**Success (201 Created):**
```json
{
  "data": { ... },
  "meta": {
    "request_id": "uuid",
    "timestamp": "2026-05-11T22:00:00Z",
    "location": "/documents/{new_id}"
  }
}
```

**Error (4xx/5xx):**
```json
{
  "error": {
    "code": "DOCUMENT_NOT_FOUND",
    "message": "Document with ID abc123 not found",
    "request_id": "uuid",
    "details": { "document_id": "abc123" }
  }
}
```

### 5. Standard Status Codes

| Code | When |
|------|------|
| 200 | GET success, PATCH success |
| 201 | POST create success |
| 202 | POST async (job submitted, polling recommended) |
| 204 | DELETE success (no content) |
| 400 | Invalid request body, missing required fields |
| 401 | Unauthorized (no valid API key or session) |
| 403 | Forbidden (valid credentials, insufficient permissions) |
| 404 | Resource not found |
| 409 | Conflict (duplicate, idempotency key expired) |
| 422 | Validation error (semantic, not syntax) |
| 429 | Rate limit exceeded |
| 500 | Internal server error |

### 6. API Versioning

**Use URL path versioning:** `/api/v1/documents`, `/api/v2/documents`

- Version is part of the base URL, not a header
- Each version is documented in OpenAPI spec with `info.version`
- Backward-compatible changes (adding fields) go in the same version
- Breaking changes require a new version with a migration guide

### 7. API Key Permissions

Standardize permission scopes used by all components:

| Scope | Description |
|-------|-------------|
| `document:read` | Read document metadata and content |
| `document:write` | Create, update, delete documents |
| `document:delete` | Permanently delete documents |
| `ocr:read` | Read OCR results |
| `ocr:write` | Trigger OCR processing |
| `extraction:read` | Read extraction results |
| `extraction:write` | Trigger extraction, manage schemas |
| `classification:read` | Read classifications |
| `classification:write` | Trigger classification, manage types |
| `workflow:read` | Read workflow executions |
| `workflow:write` | Trigger workflows, manage approvals |
| `search:read` | Search documents |
| `admin` | Full access |

### 8. Error Codes

Standard error codes across all components:

| Code | HTTP | Description |
|------|------|-------------|
| `DOCUMENT_NOT_FOUND` | 404 | Document doesn't exist |
| `DOCUMENT_UPLOAD_FAILED` | 400 | File too large, invalid format |
| `OCR_PROCESSING_FAILED` | 500 | OCR engine error |
| `EXTRACTION_SCHEMA_NOT_FOUND` | 404 | Schema ID doesn't exist |
| `EXTRACTION_FAILED` | 500 | Extraction engine error |
| `CLASSIFICATION_FAILED` | 500 | Classification engine error |
| `WORKFLOW_NOT_FOUND` | 404 | Workflow ID doesn't exist |
| `WORKFLOW_EXECUTION_FAILED` | 500 | Workflow engine error |
| `RATE_LIMIT_EXCEEDED` | 429 | API key rate limit |
| `INVALID_API_KEY` | 401 | API key not found or expired |
| `INSUFFICIENT_PERMISSIONS` | 403 | API key lacks required scope |
| `IDEMPOTENCY_KEY_EXPIRED` | 409 | Idempotency key older than 24h |
| `STORAGE_BACKEND_UNAVAILABLE` | 503 | Storage backend connection failed |
| `SEARCH_INDEX_ERROR` | 500 | Search engine connection failed |

## Acceptance Criteria

- [ ] All endpoints use resource-oriented URL patterns (`/resources` not `/actions`)
- [ ] All list endpoints support pagination with `?page=1&limit=50`
- [ ] All POST endpoints accept `Idempotency-Key` header
- [ ] All responses follow the standard success/error format
- [ ] All error codes are defined and consistent across components
- [ ] API versioning strategy documented (`/api/v1/` prefix)
- [ ] Permission scopes are standardized and used by all auth endpoints
- [ ] `IdempotencyKey` entity is in the canonical entity registry
