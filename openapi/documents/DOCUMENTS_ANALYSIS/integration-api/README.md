# Integration & API

> **Component:** REST API design, webhook support, SDK generation, and third-party integrations
> **Priority:** P3 — Critical for dev-first positioning
> **DocuPipe Reference:** POST /workflow/on_submit_document (chained operations), webhooks for async callbacks, base64 document uploads
> **Docparser Reference:** GET /v1/parsers, HTTP Basic Auth with API key, rate limiting (60/min single, 30/min batch)

---

## The Pitch

**Buyer Question:** *Can I integrate document processing into my existing applications with type-safe SDKs, webhooks, and API contracts — without vendor lock-in or proprietary formats?*

If the answer is no, you have a point solution, not a platform. The value of document intelligence compounds when it connects to your entire technology stack. Without clean APIs, webhooks, and SDKs, document processing is an island — you have to manually export and import data, which defeats automation. This component defines how the platform exposes its capabilities, how third-party systems integrate with it, and how SDKs are generated and distributed.

---

## What This Component Does

Integration & API is the connectivity layer:

1. **REST API** — Complete REST API with OpenAPI 3.1.0 specifications
2. **SDK Generation** — Automatic SDK generation for TypeScript, Python, Rust, Go
3. **Webhook Support** — Event-driven notifications for workflow completion
4. **API Versioning** — Backward-compatible API versioning strategy
5. **Rate Limiting** — Configurable rate limits per API key/user
6. **Authentication** — API key and OAuth 2.0 authentication
7. **GraphQL Support** — Optional GraphQL endpoint for flexible queries
8. **Integration Marketplace** — Pre-built connectors for popular services

---

## Entity Model

### API Key Entity

The authentication credential for programmatic access.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | API key name (for identification) |
| `key_prefix` | String (8) | Yes | First 8 chars shown to user (never the full key) |
| `key_hash` | String (64) | Yes | SHA-256 hash of API key (never stored plain) |
| `permissions` | String[] | Yes | Allowed permissions (document:read, document:write, etc.) |
| `rate_limit` | Integer | No | Requests per minute (0 = unlimited) |
| `is_active` | Boolean | No | Key activation (default: true) |
| `created_at` | DateTime | Yes | Creation timestamp |
| `last_used_at` | DateTime | No | Last use timestamp |

### Webhook Subscription Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `url` | String (1000) | Yes | Webhook endpoint URL |
| `events` | String[] | Yes | Subscribed event types |
| `secret` | String (64) | No | HMAC signing secret |
| `is_active` | Boolean | No | Subscription activation (default: true) |
| `created_at` | DateTime | Yes | Creation timestamp |
| `last_triggered_at` | DateTime | No | Last trigger timestamp |

### Webhook Event Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `event_type` | String (128) | Yes | Event type |
| `payload` | JSONB | Yes | Event payload data |
| `signature` | String (64) | No | HMAC-SHA256 signature |
| `status` | Enum: [PENDING, SENT, FAILED, RETRYING] | Yes | Delivery status |
| `attempts` | Integer | Yes | Retry count (max 5, exponential backoff) |
| `next_retry_at` | DateTime | No | Next retry timestamp |
| `created_at` | DateTime | Yes | Creation timestamp |
| `delivered_at` | DateTime | No | Delivery timestamp |

### Integration Endpoint Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | Integration name (e.g., "SAP AP") |
| `url` | String (1000) | Yes | Integration endpoint URL |
| `auth_type` | Enum: [NONE, API_KEY, OAUTH2, BASIC] | Yes | Authentication type |
| `payload_template` | JSONB | No | Payload template for data export |
| `is_active` | Boolean | No | Integration activation (default: true) |
| `created_at` | DateTime | Yes | Creation timestamp |

---

## Entity Relationships

```
API Key
  ├── Webhook Event (one-to-many)         ← via API key in webhook headers
  └── Integration Endpoint (one-to-many)  ← via API key for auth

Webhook Subscription
  ├── Webhook Event (one-to-many)         ← via subscription_id
  └── API Key (many-to-one)               ← via API key that created subscription

Webhook Event
  └── Webhook Subscription (many-to-one)  ← via subscription_id

Integration Endpoint
  └── API Key (many-to-one)               ← via API key used for auth
```

---

## DocuPipe Technical Patterns to Follow

### Pattern 1: Webhook as First-Class Citizen

DocuPipe uses webhooks as the primary delivery mechanism for async processing results. When a document processing job completes, a webhook is fired immediately — no polling needed. This is the right pattern for document processing because:
- Processing can take seconds to minutes (longer than HTTP request timeout)
- Webhooks eliminate the need for polling loops in client code
- Webhooks reduce API call volume (no repeated GET requests)

```python
# DocuPipe webhook pattern
# Client registers webhook URL
HEADERS = {"accept": "application/json", "X-API-Key": api_key}

# After job completes, DocuPipe fires webhook to registered URL
# with JSON payload containing documentId, jobId, and result
def handle_webhook(request):
    payload = request.json  # {"documentId": "...", "jobId": "...", "result": {...}}
    # Process the result
    return {"status": "received"}, 200
```

**Recommendation: RERP should implement webhooks as the primary delivery mechanism.** Support webhook registration on document creation, workflow completion, and extraction result availability. Use exponential backoff for retries (2s, 4s, 8s, 16s, 32s — max 5 attempts). Sign all webhook payloads with HMAC-SHA256 so recipients can verify authenticity.

### Pattern 2: Rate Limiting with Clear Headers

Docparser implements rate limits at the API level:
- 60 calls/minute for single document results (`GET /v1/results/<parser_id>/<document_id>`)
- 30 calls/minute for batch results (`GET /v1/results/<parser_id>`)
- When limits are exceeded, the API returns 429 Too Many Requests

**Recommendation: RERP should implement rate limiting with standard headers.** Return `X-RateLimit-Limit`, `X-RateLimit-Remaining`, and `X-RateLimit-Reset` headers with every response. When the limit is exceeded, return 429 with a `Retry-After` header. Per-key rate limits allow enterprise customers to request higher limits.

---

## Competitive Intelligence Deep Dive

### DocuPipe: Clean REST API with Chained Operations
DocuPipe has excellent API documentation. REST API with JSON request/response. API key authentication (X-API-Key header). Webhooks for async processing callbacks. SDKs for TypeScript and Python. The API is well-designed and developer-friendly. No GraphQL support. Rate limits are generous on paid plans.

**Key strengths:** Clean REST API, chained operations via workflow endpoint, webhook support
**Key weaknesses:** No GraphQL, no SDK generation, no OpenAPI spec

### AWS Textract: AWS SDK-Driven
Textract uses AWS SDKs (boto3 for Python, aws-sdk for JavaScript, etc.) rather than a direct REST API. The advantage is language-specific SDKs with auto-generated client libraries. The disadvantage is AWS SDK complexity — you're tied to AWS's API design patterns. No webhook support — you must poll or use S3 event notifications.

**Key strengths:** Multi-language SDKs, AWS-native integration
**Key weaknesses:** AWS SDK complexity, no direct REST API, no self-hosted

### Docparser: Developer-Friendly API with SDKs
Docparser provides a clean REST API with JSON request/response. HTTP Basic Auth with API key. Webhook support for parsing completion. Export to JSON, XML, CSV, Excel. Integration with 100+ services via Zapier, Make, and native connectors. Rate limits: 60/min single, 30/min batch. SDKs for PHP, Node.js, Python (third-party), and Salesforce Apex.

**Key strengths:** REST API, SDKs for multiple languages, webhook support
**Key weaknesses:** Older API (last updated 2018), no OpenAPI spec, no GraphQL

### Hyperscience: API-First with Custom Code Blocks
Hyperscience treats integrations as configurable Blocks within the workflow engine. Integration Blocks enable data to flow between systems — RPA platforms, databases, content management systems, and custom applications. API-first architecture with Python SDK for custom code blocks. Supports AWS, Google, Azure, on-premises, and FedRAMP High deployments.

**Key strengths:** API-first, custom code blocks, multi-deployment
**Key weaknesses:** Enterprise-only, no self-hosted option

### Paperless-ngx: Open API
Paperless-ngx provides a REST API with Swagger/OpenAPI documentation. Token-based authentication. No webhooks — you must poll for changes. The API supports CRUD operations on all entities (documents, tags, correspondents, types). Community-maintained SDKs for Python and JavaScript.

**Key strengths:** OpenAPI documentation, token auth, community SDKs
**Key weaknesses:** No webhooks, polling-only, community-maintained SDKs

---

## Competitive Positioning

### Where RERP Wins
- **OpenAPI-first with auto-generated SDKs** — Unlike DocuPipe (no OpenAPI) or Docparser (no OpenAPI), RERP generates SDKs from OpenAPI specs
- **Multi-language SDK generation** — Unlike Docparser (PHP, Node, third-party Python), RERP generates TypeScript, Python, Rust, and Go from a single spec
- **Webhooks + polling** — Unlike Paperless-ngx (polling only), RERP supports both webhooks and polling

### Where RERP Lags
- **No SDK generation** — Not yet implemented
- **No webhook delivery** — Not yet implemented
- **No API documentation** — Not yet implemented

---

## Implementation Roadmap

### Phase 1: Core API (2-3 weeks) — P3
1. Define `API Key` entity with secure key storage (hash only)
2. Implement API key authentication middleware
3. Rate limiting per API key (Redis-backed counter)
4. Basic REST API documentation (OpenAPI spec generation)
5. API key usage tracking and audit logging

### Phase 2: Webhooks & Events (3-4 weeks) — P3
1. Define `Webhook Subscription` entity with event filtering
2. Implement webhook delivery system with HTTP POST
3. HMAC-SHA256 signature verification for webhook payloads
4. Retry logic with exponential backoff (2s, 4s, 8s, 16s, 32s)
5. Webhook event history and monitoring dashboard

### Phase 3: SDK Generation (2-3 weeks) — P3
1. Implement OpenAPI spec generation from RERP models
2. Generate TypeScript SDK from OpenAPI spec (openapi-typescript-codegen)
3. Generate Python SDK from OpenAPI spec (openapi-python-client)
4. Generate Rust SDK from OpenAPI spec (openapi-rust)
5. Publish SDKs to npm, PyPI, crates.io with version alignment

### Phase 4: Advanced Features (3-4 weeks) — P4
1. OAuth 2.0 authentication support
2. GraphQL endpoint for flexible queries (GraphQL schema from OpenAPI)
3. API versioning with migration guides
4. Integration marketplace (pre-built connectors for SAP, Workday, etc.)
5. API analytics and usage dashboard

---

## Key Takeaway for Buyers

RERP Documents' API pitch is **OpenAPI-first, multi-language SDKs, and self-hosted**. Unlike DocuPipe (no OpenAPI) or Textract (AWS SDKs only), RERP provides developer-friendly APIs with automatic SDK generation for TypeScript, Python, and Rust. Unlike Adobe (per-transaction pricing), RERP has zero per-API-call costs for self-hosted use.

The OpenAPI-first approach means every API change is version-controlled, documented, and automatically generates type-safe SDKs. The Rust-native API delivers sub-millisecond latency for all endpoints. And because the API is fully defined in OpenAPI, every client gets complete API documentation, automatic validation, and tooling that works out of the box.

**The immediate priority: define the API Key entity, implement API key authentication, and build the webhook delivery system. APIs are the bridge between document processing and business applications.**
