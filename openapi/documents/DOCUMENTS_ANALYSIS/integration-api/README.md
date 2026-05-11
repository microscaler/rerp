# Integration & API

> **Component:** REST API design, webhook support, SDK generation, and third-party integrations
> **Priority:** P3 — Critical for dev-first positioning

---

## The Pitch

**Buyer Question:** *Can I integrate document processing into my existing applications with type-safe SDKs, webhooks, and API contracts — without vendor lock-in or proprietary formats?*

If the answer is no, you have a point solution, not a platform. The value of document intelligence compounds when it connects to your entire technology stack. Without clean APIs, webhooks, and SDKs, document processing is an island — you have to manually export and import data, which defeats automation.

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

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (255) | Yes | API key name |
| `key_prefix` | String (8) | Yes | First 8 chars for identification |
| `key_hash` | String (64) | Yes | Hashed API key (never stored plain) |
| `permissions` | String[] | Yes | Allowed permissions |
| `rate_limit` | Integer | No | Requests per minute |
| `is_active` | Boolean | No | Key activation |
| `created_at` | DateTime | Yes | Creation timestamp |
| `last_used_at` | DateTime | No | Last use timestamp |

### Webhook Subscription Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `url` | String (1000) | Yes | Webhook endpoint URL |
| `events` | String[] | Yes | Subscribed event types |
| `secret` | String (64) | No | HMAC signing secret |
| `is_active` | Boolean | No | Subscription activation |
| `created_at` | DateTime | Yes | Creation timestamp |

### Webhook Event Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `event_type` | String (128) | Yes | Event type |
| `payload` | JSONB | Yes | Event payload |
| `status` | Enum: [PENDING, SENT, FAILED, RETRYING] | Yes | Delivery status |
| `attempts` | Integer | Yes | Retry count |
| `created_at` | DateTime | Yes | Creation timestamp |

---

## Required API Endpoints

### API Key Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/keys` | List API keys |
| `POST` | `/api/keys` | Create API key |
| `DELETE` | `/api/keys/{id}` | Revoke API key |
| `GET` | `/api/keys/{id}/usage` | Get key usage statistics |

### Webhook Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/webhooks` | List webhook subscriptions |
| `POST` | `/webhooks` | Create webhook subscription |
| `PATCH` | `/webhooks/{id}` | Update webhook |
| `DELETE` | `/webhooks/{id}` | Delete webhook |
| `POST` | `/webhooks/{id}/test` | Test webhook delivery |
| `GET` | `/webhooks/{id}/events` | Get webhook event history |

### API Versioning

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/version` | Get current API version |
| `GET` | `/api/changelog` | API changelog |
| `GET` | `/api/supported-versions` | List supported versions |

---

## Competitive Intelligence Deep Dive

### DocuPipe: Clean REST API
DocuPipe has excellent API documentation at readme.io. REST API with JSON request/response. API key authentication. Webhooks for async processing callbacks. SDKs for TypeScript and Python. The API is well-designed and developer-friendly. No GraphQL support. Rate limits are generous on paid plans.

### AWS Textract: Infrastructure API
Textract uses AWS SDKs (boto3 for Python, aws-sdk for JavaScript, etc.) rather than a direct REST API. The advantage is language-specific SDKs with auto-generated client libraries. The disadvantage is AWS SDK complexity — you're tied to AWS's API design patterns. No webhook support — you must poll or use S3 event notifications.

### Docparser: Developer-Friendly API
Docparser provides a clean REST API with JSON request/response. API key authentication. Webhook support for parsing completion. Export to JSON, XML, CSV, Excel. Integration with 100+ services via Zapier, Make, and native connectors. The API is well-documented with code examples in multiple languages.

### Rossum: Enterprise API
Rossum provides a comprehensive REST API with full CRUD operations. OAuth 2.0 authentication. Webhook support for workflow events. API rate limits are configured per enterprise contract. Integration with SAP, Coupa, Workday, and Oracle via API. No public SDKs — enterprise support includes custom integration assistance.

### Hyperscience: Python-First API
Hyperscience's API is designed for Python developers. Human-readable Python code for custom extraction logic. REST API with JSON request/response. OAuth 2.0 authentication. Webhook support for event notifications. The API is designed to be extensible — developers can add custom logic without modifying the core platform.

### Adobe PDF Services: Developer SDKs
Adobe provides SDKs for JavaScript, Python, and Java. REST API with Document Transaction model (each PDF operation costs 1 transaction). Free tier: 500 transactions/month. The API is well-documented with interactive examples. No webhook support — operations are synchronous.

### Paperless-ngx: Open API
Paperless-ngx provides a REST API with Swagger/OpenAPI documentation. Token-based authentication. No webhooks — you must poll for changes. The API supports CRUD operations on all entities (documents, tags, correspondents, types). Community-maintained SDKs for Python and JavaScript.

---

## Implementation Roadmap

### Phase 1: Core API (2-3 weeks) — P3
1. Define API Key entity with secure storage
2. Implement API key authentication
3. Rate limiting per API key
4. Basic REST API documentation (OpenAPI spec)
5. API key usage tracking

### Phase 2: Webhooks & Events (3-4 weeks) — P3
1. Define Webhook Subscription entity
2. Implement webhook delivery system
3. HMAC signature verification for webhook payloads
4. Retry logic with exponential backoff
5. Webhook event history and monitoring

### Phase 3: SDK Generation (2-3 weeks) — P3
1. Generate TypeScript SDK from OpenAPI spec
2. Generate Python SDK from OpenAPI spec
3. Generate Rust SDK from OpenAPI spec
4. SDK versioning aligned with API versioning
5. SDK documentation with examples

### Phase 4: Advanced Features (3-4 weeks) — P4
1. OAuth 2.0 authentication support
2. GraphQL endpoint for flexible queries
3. API versioning with migration guides
4. Integration marketplace (pre-built connectors)
5. API analytics and usage dashboard

---

## Key Takeaway for Buyers

RERP Documents' API pitch is **OpenAPI-first, multi-language SDKs, and self-hosted**. Unlike Textract (AWS SDKs only) or Rossum (enterprise API only), RERP provides developer-friendly APIs with automatic SDK generation for TypeScript, Python, and Rust. Unlike Adobe (per-transaction pricing), RERP has zero per-API-call costs for self-hosted use.

The OpenAPI-first approach means every API change is version-controlled, documented, and automatically generates type-safe SDKs. The Rust-native API delivers sub-millisecond latency for all endpoints. And because the API is fully defined in OpenAPI, every client gets complete API documentation, automatic validation, and tooling that works out of the box.

**The immediate priority: define the API Key entity, implement API key authentication, and build the webhook delivery system. APIs are the bridge between document processing and business applications.**
