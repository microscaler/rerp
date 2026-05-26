# Platform & Extensibility

> **Component:** Developer SDK (Rust, TypeScript, Python), plugin architecture, custom model training, custom field definitions, webhook events, rate limiting, and documentation generation
> **Priority:** P3 — Developer tools, custom models, plugins for extending the Documents suite
> **Nanonets Reference:** Nanonets custom model builder, Google Custom Document AI, Azure Custom Models, ABBYY SDK, DocuPipe custom schemas

---

## The Pitch

**Buyer Question:** *Can my developers extend the document processing pipeline with custom models, custom fields, and custom connectors — without waiting for vendor releases?*

A document processing system that's rigid and vendor-controlled is a liability. A buyer needs to know: *Can my developers add a custom document type, train a custom extraction model, and integrate with our internal systems — in hours, not months?* This component covers the developer platform that makes RERP Documents extensible: SDKs, plugin architecture, custom models, custom fields, webhook events, API versioning, and automated documentation generation.

---

## What This Component Does

1. **Developer SDKs** — Official SDKs for Rust, TypeScript, and Python with type-safe API clients
2. **Plugin Architecture** — Extensible pipeline with pluggable OCR engines, extraction models, and storage backends
3. **Custom Model Training** — Train custom OCR, classification, and extraction models on your documents
4. **Custom Field Definitions** — Define new extraction fields without code changes
5. **Webhook Events** — Subscribe to document lifecycle events (PROCESSED, FAILED, REVIEWED, APPROVED)
6. **Rate Limiting** — Configurable rate limits per API key, per endpoint, per organization
7. **API Versioning** — Backward-compatible API versioning with deprecation lifecycle
8. **Automated Documentation** — OpenAPI specs auto-generated from code, with interactive documentation
9. **Custom OCR Engine Integration** — Plug in Tesseract, PaddleOCR, or custom OCR models
10. **Batch API** — Process large document batches with async endpoints and progress tracking
11. **Streaming API** — Process documents in real-time with streaming response
12. **Developer Portal** — Self-service API keys, usage analytics, and documentation

---

## Entity Model

### PluginManifest Entity

Defines a plugin that extends the Documents pipeline:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Plugin name |
| `version` | String (32) | Yes | Plugin version (semantic versioning) |
| `type` | Enum: [OCR_ENGINE, EXTRACTION_MODEL, CLASSIFIER, STORAGE_BACKEND, VALIDATOR, TRANSFORMER] | Yes | Plugin type |
| `description` | Text | No | Plugin description |
| `author` | String (128) | No | Plugin author |
| `entry_point` | String (512) | Yes | Entry point for plugin code |
| `config_schema` | JSON | No | Plugin configuration schema |
| `is_active` | Boolean | Yes | Enable/disable plugin |
| `is_system` | Boolean | Yes | System plugin (cannot be disabled) |
| `created_at` | DateTime | Yes | When plugin was created |

### DeveloperAPIKey Entity

API keys for developer access:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `key_hash` | String (64) | Yes | Hashed API key (stored, never raw) |
| `name` | String (128) | Yes | Key name (for identification) |
| `owner_id` | Foreign Key: User | Yes | Who owns this key |
| `permissions` | JSON | Yes | Permission scope (which endpoints can access) |
| `rate_limit` | Integer | No | Requests per minute (NULL = default) |
| `expires_at` | DateTime | No | When key expires (NULL = no expiry) |
| `last_used_at` | DateTime | No | When key was last used |
| `is_active` | Boolean | Yes | Enable/disable key |
| `created_at` | DateTime | Yes | When key was created |

### CustomModel Entity

Custom models trained by users:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Model name |
| `model_type` | Enum: [OCR, CLASSIFICATION, EXTRACTION] | Yes | Model type |
| `status` | Enum: [TRAINING, READY, FAILED, DEPRECATED] | Yes | Training status |
| `training_data_count` | Integer | No | Number of training documents |
| `accuracy` | Float (0-100) | No | Model accuracy on test set |
| `base_model` | String (64) | No | Base model used for training |
| `trained_by` | Foreign Key: User | Yes | Who trained this model |
| `is_active` | Boolean | Yes | Enable/disable model |
| `created_at` | DateTime | Yes | Creation timestamp |
| `trained_at` | DateTime | No | Training completion timestamp |

### CustomFieldDefinition Entity

Custom extraction fields defined by users:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Field name |
| `document_type` | String (64) | No | Applies to document type (NULL = all) |
| `field_type` | Enum: [STRING, INTEGER, FLOAT, DATE, BOOLEAN, ENUM, JSON] | Yes | Field type |
| `extraction_rule` | JSON | No | How to extract this field |
| `validation_rule` | String (512) | No | Validation regex/pattern |
| `required` | Boolean | Yes | Is this field required? |
| `is_auto_extracted` | Boolean | Yes | Auto-extract or manual entry? |
| `created_by` | Foreign Key: User | Yes | Who created this field |
| `created_at` | DateTime | Yes | Creation timestamp |

### WebhookSubscription Entity

Webhook subscriptions for document lifecycle events:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Subscription name |
| `url` | String (1024) | Yes | Webhook URL |
| `events` | JSON | Yes | Event types to subscribe to |
| `secret` | String (64) | Yes | HMAC secret for signature verification |
| `is_active` | Boolean | Yes | Enable/disable subscription |
| `last_triggered_at` | DateTime | No | When last webhook was sent |
| `failure_count` | Integer | No | Number of consecutive failures |
| `created_at` | DateTime | Yes | Creation timestamp |

### RateLimitConfig Entity

Rate limiting configuration:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Config name |
| `scope` | Enum: [GLOBAL, PER_API_KEY, PER_ENDPOINT, PER_ORGANIZATION] | Yes | Rate limit scope |
| `max_requests` | Integer | Yes | Maximum requests |
| `period_seconds` | Integer | Yes | Period in seconds |
| `burst_limit` | Integer | No | Burst allowance |
| `is_active` | Boolean | Yes | Enable/disable config |
| `created_at` | DateTime | Yes | Creation timestamp |

### APIDocumentation Entity

Auto-generated API documentation metadata:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `version` | String (16) | Yes | API version (e.g., "v1", "v2") |
| `openapi_spec_path` | String (1024) | Yes | Path to OpenAPI spec file |
| `is_latest` | Boolean | Yes | Is this the latest version? |
| `deprecated` | Boolean | Yes | Is this version deprecated? |
| `deprecation_date` | DateTime | No | When this version is deprecated |
| `sunset_date` | DateTime | No | When this version will be retired |
| `created_at` | DateTime | Yes | Creation timestamp |

---

## Entity Relationships

```
PluginManifest (extends the pipeline)
  ├── [Config] (via config_schema)                       ← plugin configuration
  └── [Engine] (via entry_point)                        ← plugin code entry point

DeveloperAPIKey
  ├── User (via owner_id)                               ← who owns the key
  └── [RateLimit] (via rate_limit)                       ← per-key rate limit

CustomModel
  ├── User (via trained_by)                              ← who trained it
  └── PluginManifest (via model_type)                    ← registered as a plugin

CustomFieldDefinition
  ├── User (via created_by)                              ← who created it
  └── ExtractionResult (via field_name reference)        ← used in extraction

WebhookSubscription
  ├── User (via owner)                                  ← who created subscription
  └── [Events] (via events array)                       ← subscribed event types

RateLimitConfig
  └── [Scope] (via scope enum)                          ← what this config applies to

APIDocumentation
  └── [OpenAPI Spec] (via openapi_spec_path)            ← generated spec file
```

---

## Required API Endpoints

### Developer Portal

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/developer/docs` | Auto-generated API documentation |
| `GET` | `/developer/docs/{version}` | Get documentation for specific version |
| `GET` | `/developer/openapi-spec` | Download OpenAPI spec |
| `GET` | `/developer/openapi-spec/{version}` | Download spec for specific version |
| `GET` | `/developer/usage` | API usage analytics |

### API Key Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/developer/api-keys` | List all API keys |
| `POST` | `/developer/api-keys` | Create API key |
| `DELETE` | `/developer/api-keys/{id}` | Revoke API key |
| `POST` | `/developer/api-keys/{id}/rotate` | Rotate API key |
| `GET` | `/developer/api-keys/usage` | API key usage analytics |

### Plugin Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/developer/plugins` | List all plugins |
| `POST` | `/developer/plugins` | Register a plugin |
| `PATCH` | `/developer/plugins/{id}` | Update plugin configuration |
| `DELETE` | `/developer/plugins/{id}` | Unregister a plugin |
| `POST` | `/developer/plugins/{id}/test` | Test plugin integration |

### Model Training

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/developer/models` | List all custom models |
| `POST` | `/developer/models/train` | Start model training |
| `GET` | `/developer/models/{id}` | Get model detail and accuracy |
| `GET` | `/developer/models/{id}/training-data` | View training data |
| `DELETE` | `/developer/models/{id}` | Delete/deprecate model |
| `POST` | `/developer/models/{id}/deploy` | Deploy model to production |

### Custom Fields

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/developer/custom-fields` | List all custom fields |
| `POST` | `/developer/custom-fields` | Create custom field definition |
| `PATCH` | `/developer/custom-fields/{id}` | Update custom field |
| `DELETE` | `/developer/custom-fields/{id}` | Delete custom field |
| `GET` | `/developer/custom-fields/template` | Load field template by document type |

### Webhook Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/developer/webhooks` | List all webhook subscriptions |
| `POST` | `/developer/webhooks` | Create webhook subscription |
| `PATCH` | `/developer/webhooks/{id}` | Update webhook subscription |
| `DELETE` | `/developer/webhooks/{id}` | Delete webhook subscription |
| `POST` | `/developer/webhooks/{id}/test` | Test webhook delivery |
| `GET` | `/developer/webhooks/{id}/delivered` | Webhook delivery history |

### Rate Limiting

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/developer/rate-limits` | List all rate limit configs |
| `POST` | `/developer/rate-limits` | Create rate limit config |
| `PATCH` | `/developer/rate-limits/{id}` | Update rate limit config |
| `DELETE` | `/developer/rate-limits/{id}` | Delete rate limit config |
| `GET` | `/developer/rate-limits/current` | Current rate limit status for this API key |

### Batch & Streaming APIs

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/developer/batch/upload` | Upload batch of documents |
| `GET` | `/developer/batch/{id}` | Get batch processing status |
| `GET` | `/developer/batch/{id}/results` | Get batch results |
| `GET` | `/developer/batch/{id}/errors` | Get batch error details |
| `POST` | `/developer/stream/process` | Process document via streaming API |
| `GET` | `/developer/stream/status/{id}` | Get streaming status |

---

## Competitive Positioning

### Where RERP Wins

- **Self-hosted developer platform** — No per-developer licensing. Unlimited developers on your infrastructure.
- **OpenAPI-first documentation** — Every endpoint, entity, and schema is machine-readable. Auto-generated SDKs for any language.
- **Rust-native performance** — SDK performance in Rust is orders of magnitude faster than Python (ABBYY) or Java (Kofax).
- **Zero marginal cost at scale** — 100K API calls cost the same as 1K.
- **Plugin architecture** — Plug in any OCR engine, extraction model, or storage backend without vendor lock-in.

### Where RERP Lags

- **No SDKs deployed** — No official SDKs for Rust, TypeScript, or Python.
- **No plugin system** — No extensible pipeline architecture.
- **No custom model training** — No training pipeline for custom models.
- **No custom fields** — No user-defined extraction fields.
- **No webhook events** — No event-driven integration surface.
- **No rate limiting** — No configurable rate limits.
- **No API versioning** — No backward-compatible API versioning.

---

## Competitive Intelligence Deep Dive

### Nanonets — Custom Model Builder

Nanonets focuses on building and training custom AI models for document processing. Users upload training documents, and Nanonets trains a custom model in hours. The key advantage is ease of use — no code required. Pricing: $0.30/run for data extraction, $0.10/run for classification. The disadvantage is cloud-only — models and training data are stored on Nanonets' infrastructure. RERP's advantage is self-hosted model training with full data control.

### Google Custom Document AI

Google's Custom Document AI lets users train custom extraction models on their own documents. Users provide labeled training documents, and Google trains a custom extractor. The key advantage is Google's ML infrastructure and accuracy. However, models run in Google Cloud, and custom training costs $0.50/1000 pages for the custom processor. RERP's advantage is self-hosted training and zero per-page costs.

### Azure Custom Models

Azure Document Intelligence allows custom model training via the Azure AI Studio. Users upload labeled training documents, and Azure trains a custom extractor. The key advantage is Microsoft ecosystem integration. However, custom models cost $0.03/page, and training requires Azure ML infrastructure. RERP's advantage is self-hosted training with zero marginal cost.

### ABBYY SDK

ABBYY provides SDKs for C++, C#, Java, and Python that embed ABBYY recognition directly into custom applications. The key advantage is offline recognition — no API calls needed. However, SDKs are tied to the ABBYY license ($100K+) and don't support custom model training. RERP's advantage is open-source, extensible SDKs with custom model support.

### DocuPipe — Custom Schemas

DocuPipe supports custom data extraction schemas — users define which fields to extract, and DocuPipe's LLM extracts them. The key advantage is zero training required — the LLM understands the schema without examples. However, it's cloud-only and API-consumed. RERP's advantage is self-hosted deployment with the same LLM-native extraction.

---

## Implementation Roadmap

### Phase 1: Developer Portal Foundation (2-3 weeks) — P3

1. Define `DeveloperAPIKey` entity with hashed key storage
2. Implement API key CRUD endpoints
3. Implement auto-generated OpenAPI spec endpoint
4. Implement API documentation endpoint (served from OpenAPI spec)
5. Implement API usage analytics endpoint
6. Add API key rotation endpoint

### Phase 2: Rate Limiting & API Versioning (2-3 weeks) — P3

1. Define `RateLimitConfig` entity
2. Implement rate limiting middleware (sliding window algorithm)
3. Implement rate limit status endpoint
4. Define `APIDocumentation` entity
5. Implement API versioning (backward-compatible version prefixes)
6. Add deprecation lifecycle (warnings, sunset dates)

### Phase 3: Webhook Events (2-3 weeks) — P3

1. Define `WebhookSubscription` entity with HMAC signature
2. Implement webhook subscription CRUD endpoints
3. Implement webhook event dispatch (PROCESSED, FAILED, REVIEWED, APPROVED)
4. Implement webhook delivery with retry (exponential backoff)
5. Implement webhook signature verification
6. Add webhook delivery history endpoint

### Phase 4: Plugin Architecture (3-4 weeks) — P3

1. Define `PluginManifest` entity with type system
2. Implement plugin registration and lifecycle (load, unload, reload)
3. Implement plugin configuration schema validation
4. Implement plugin testing endpoint
5. Add plugin types: OCR_ENGINE, EXTRACTION_MODEL, CLASSIFIER, STORAGE_BACKEND
6. Implement plugin discovery (scan for available plugins)

### Phase 5: Custom Models & Fields (4-6 weeks) — P3

1. Define `CustomModel` and `CustomFieldDefinition` entities
2. Implement custom field CRUD endpoints
3. Implement model training pipeline (upload training data → train → evaluate → deploy)
4. Implement model accuracy reporting
5. Add field template loading by document type
6. Implement model versioning (train, deploy, rollback)

### Phase 6: Batch & Streaming APIs (3-4 weeks) — P3

1. Implement batch upload endpoint (POST /developer/batch/upload)
2. Implement batch progress tracking endpoint
3. Implement batch results and error endpoints
4. Implement streaming API endpoint for real-time processing
5. Add streaming status endpoint
6. Implement batch scheduling (process batches at off-peak hours)

---

## Key Takeaway for Buyers

Extensibility is where a document processing system becomes a platform, not just a product. A buyer needs to know: *Can my developers extend the pipeline with custom models, custom fields, and custom connectors — without waiting for vendor releases?* RERP's advantage is self-hosted developer platform with unlimited developers, OpenAPI-first auto-generated SDKs for any language, and a plugin architecture that lets you plug in any OCR engine or extraction model. The immediate priority: define the DeveloperAPIKey entity, implement auto-generated OpenAPI specs, and build the developer documentation endpoint. Everything else builds on this foundation.
