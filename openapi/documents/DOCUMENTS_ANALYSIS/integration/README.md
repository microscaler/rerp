# Integration & Routing

> **Component:** ERP/CRM/accounting connectors, webhook-based integration, API-based integration, data transformation/mapping, retry logic, error handling, and audit logging
> **Priority:** P2 — Connect processed documents to downstream business systems
> **UiPath Reference:** UiPath RPA connectors, Kofax integration hub, ABBYY Web Services API, Azure Logic Apps, AWS Lambda + Step Functions

---

## The Pitch

**Buyer Question:** *Can I connect every processed document to the systems that need its data — ERP, CRM, accounting — without writing custom integrations for each one?*

A document processing system that can't deliver its results to downstream systems is a research project, not a product. A buyer needs to know: *Does the data flow automatically to where it belongs, and when it fails, do I get immediate notification with full context?* This component covers all integration patterns — REST APIs, webhooks, message queues, file-based transfers, and pre-built connectors for major ERP/CRM/accounting systems.

---

## What This Component Does

1. **ERP Connectors** — Pre-built integrations for SAP, Oracle NetSuite, Microsoft Dynamics, Sage
2. **CRM Connectors** — Pre-built integrations for Salesforce, HubSpot, Dynamics 365, Zoho
3. **Accounting Connectors** — Pre-built integrations for QuickBooks, Xero, Sage, FreshBooks
4. **Webhook-Based Integration** — Push events to any system via HTTP webhooks
5. **API-Based Integration** — Pull/push data via REST APIs with authentication
6. **Message Queue Integration** — Publish to RabbitMQ, Kafka, AWS SQS, Azure Service Bus
7. **Data Transformation/Mapping** — Map extracted fields to target system schemas
8. **Retry Logic** — Automatic retry on transient failures with exponential backoff
9. **Error Handling** — Dead letter queues, error notifications, manual retry interface
10. **Audit Logging** — Full audit trail of every integration event

---

## Entity Model

### IntegrationConnector Entity

Defines a connector to a downstream system:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Connector name (e.g., "SAP AP Connector") |
| `system_type` | Enum: [ERP, CRM, ACCOUNTING, CUSTOM] | Yes | Type of target system |
| `vendor` | String (64) | No | Vendor name (SAP, Salesforce, QuickBooks, etc.) |
| `endpoint_url` | String (1024) | No | API endpoint URL |
| `auth_type` | Enum: [BASIC, BEARER, OAUTH2, API_KEY, NONE] | No | Authentication method |
| `auth_config` | JSON | No | Auth credentials (encrypted) |
| `is_active` | Boolean | Yes | Enable/disable connector |
| `created_at` | DateTime | Yes | Creation timestamp |
| `last_sync` | DateTime | No | Last successful sync |

### IntegrationEndpoint Entity

A specific endpoint within a connector:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `connector_id` | Foreign Key: IntegrationConnector | Yes | Parent connector |
| `name` | String (128) | Yes | Endpoint name (e.g., "Create Invoice") |
| `method` | Enum: [POST, PUT, PATCH, GET] | No | HTTP method |
| `path` | String (512) | Yes | API path |
| `document_type` | String (64) | No | Triggers on document type (NULL = all) |
| `is_active` | Boolean | Yes | Enable/disable endpoint |

### IntegrationMapping Entity

Maps extracted document fields to target system fields:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `endpoint_id` | Foreign Key: IntegrationEndpoint | Yes | Target endpoint |
| `source_field` | String (128) | Yes | Source field (e.g., "invoice_number") |
| `target_field` | String (128) | Yes | Target field (e.g., "invoiceNumber") |
| `transformation` | Text | No | Transformation rule (e.g., "uppercase", "currency_convert") |
| `default_value` | String (255) | No | Default if source is empty |
| `is_required` | Boolean | No | Must have a value to send |
| `order` | Integer | No | Order in mapping (for multi-field operations) |

### IntegrationJob Entity

A single integration execution:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `connector_id` | Foreign Key: IntegrationConnector | Yes | Target connector |
| `endpoint_id` | Foreign Key: IntegrationEndpoint | Yes | Target endpoint |
| `document_id` | Foreign Key: DocumentStore | No | Related document |
| `extraction_id` | Foreign Key: ExtractionResult | No | Related extraction |
| `payload` | JSON | Yes | Data sent to target system |
| `status` | Enum: [PENDING, SENT, SUCCESS, FAILED, RETRYING, DEAD_LETTER] | No | Job status |
| `response_code` | Integer | No | HTTP response code from target |
| `response_body` | Text | No | Response body from target |
| `error_message` | Text | No | Error description |
| `retry_count` | Integer | No | Number of retries |
| `max_retries` | Integer | No | Maximum retry attempts |
| `created_at` | DateTime | Yes | When job was created |
| `sent_at` | DateTime | No | When sent to target |
| `completed_at` | DateTime | No | When completed |

### IntegrationError Entity

Detailed error records for failed integrations:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `job_id` | Foreign Key: IntegrationJob | Yes | Related integration job |
| `error_type` | Enum: [CONNECTION_ERROR, AUTH_ERROR, VALIDATION_ERROR, TIMEOUT, RATE_LIMITED, UNKNOWN] | Yes | Error category |
| `error_code` | String (64) | No | Error code from target system |
| `error_message` | Text | Yes | Human-readable error |
| `stack_trace` | Text | No | Technical stack trace |
| `action_taken` | Enum: [RETRIED, NOTIFIED, SILENT, ESCALATED] | Yes | Response to error |
| `acknowledged_at` | DateTime | No | When error was acknowledged |
| `resolved_at` | DateTime | No | When error was resolved |

### IntegrationAuditLog Entity

Audit trail for all integration events:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `job_id` | Foreign Key: IntegrationJob | No | Related job (nullable for system events) |
| `action` | String (64) | Yes | Action performed (e.g., "JOB_CREATED", "RETRY_ATTEMPT") |
| `actor` | String (128) | Yes | Who/what performed the action |
| `details` | JSON | No | Additional context |
| `created_at` | DateTime | Yes | When action occurred |

---

## Entity Relationships

```
IntegrationConnector (downstream system connection)
  ├── IntegrationEndpoint (via connector_id)        ← API endpoints
  ├── IntegrationJob (via connector_id)              ← integration executions
  └── IntegrationMapping (via endpoint_id)           ← field mappings

IntegrationEndpoint
  ├── IntegrationConnector (via connector_id)        ← parent connector
  ├── IntegrationMapping (via endpoint_id)           ← field mappings
  └── IntegrationJob (via endpoint_id)               ← executions to this endpoint

IntegrationJob
  ├── IntegrationConnector (via connector_id)        ← target connector
  ├── IntegrationEndpoint (via endpoint_id)          ← target endpoint
  ├── DocumentStore (via document_id)                ← source document
  ├── ExtractionResult (via extraction_id)           ← source extraction
  ├── IntegrationError (via job_id)                  ← errors for this job
  └── IntegrationAuditLog (via job_id)               ← audit trail

IntegrationMapping
  ├── IntegrationEndpoint (via endpoint_id)          ← endpoint being mapped
  └── [Source] (via source_field)                    ← document/extraction field

IntegrationError
  ├── IntegrationJob (via job_id)                    ← failed job

IntegrationAuditLog
  ├── IntegrationJob (via job_id)                    ← related job
```

---

## Required API Endpoints

### Connector Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/integrations/connectors` | List all connectors |
| `GET` | `/integrations/connectors/{id}` | Get connector detail |
| `POST` | `/integrations/connectors` | Create connector |
| `PATCH` | `/integrations/connectors/{id}` | Update connector |
| `DELETE` | `/integrations/connectors/{id}` | Delete connector |
| `POST` | `/integrations/connectors/{id}/test` | Test connector connection |

### Endpoint Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/integrations/endpoints` | List all endpoints |
| `POST` | `/integrations/endpoints` | Create endpoint |
| `PATCH` | `/integrations/endpoints/{id}` | Update endpoint |
| `DELETE` | `/integrations/endpoints/{id}` | Delete endpoint |

### Mapping Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/integrations/mappings` | List all field mappings |
| `POST` | `/integrations/mappings` | Create field mapping |
| `PATCH` | `/integrations/mappings/{id}` | Update field mapping |
| `DELETE` | `/integrations/mappings/{id}` | Delete field mapping |
| `GET` | `/integrations/mappings/template` | Load mapping template by vendor |

### Integration Jobs

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/integrations/jobs` | List all integration jobs |
| `GET` | `/integrations/jobs/{id}` | Get job detail |
| `POST` | `/integrations/jobs/{id}/retry` | Retry a failed job |
| `POST` | `/integrations/jobs/{id}/cancel` | Cancel a pending job |
| `GET` | `/integrations/jobs/pending` | List pending jobs |
| `GET` | `/integrations/jobs/failed` | List failed jobs |
| `GET` | `/integrations/jobs/dead-letter` | List dead-letter queue |

### Webhooks

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/integrations/webhooks` | List all webhooks |
| `POST` | `/integrations/webhooks` | Create webhook subscription |
| `PATCH` | `/integrations/webhooks/{id}` | Update webhook |
| `DELETE` | `/integrations/webhooks/{id}` | Delete webhook |
| `POST` | `/integrations/webhooks/{id}/test` | Test webhook delivery |

### Error Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/integrations/errors` | List all integration errors |
| `GET` | `/integrations/errors/{id}` | Get error detail |
| `POST` | `/integrations/errors/{id}/resolve` | Mark error as resolved |
| `POST` | `/integrations/errors/{id}/acknowledge` | Acknowledge error |
| `GET` | `/integrations/errors/unresolved` | List unresolved errors |

### Audit & Analytics

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/integrations/audit` | Full audit trail |
| `GET` | `/integrations/analytics/success-rate` | Success rate by connector |
| `GET` | `/integrations/analytics/latency` | Average response time by endpoint |
| `GET` | `/integrations/analytics/volume` | Integration volume by type |

---

## Competitive Positioning

### Where RERP Wins

- **Self-hosted integration layer** — No per-connector licensing. Unlimited connectors on your infrastructure.
- **OpenAPI-defined field mappings** — Every mapping is machine-readable. New connectors can be auto-generated from OpenAPI specs.
- **Rust-level integration throughput** — Processing 10,000 integration jobs in Rust is instantaneous.
- **Zero marginal cost at scale** — Sending 100K integration requests costs the same as 10K.
- **Connector agnostic** — No vendor lock-in to specific integration platforms.

### Where RERP Lags

- **No pre-built connectors** — No SAP, Salesforce, or QuickBooks connectors deployed.
- **No webhook system** — No event-driven integration surface.
- **No retry logic** — No automatic retry with exponential backoff.
- **No error dead-letter queue** — No dead-letter queue for failed integrations.
- **No audit logging** — No full audit trail of integration events.

---

## Competitive Intelligence Deep Dive

### UiPath RPA — Point-to-Point Integration

UiPath's Document Understanding excels at document processing but relies on UiPath's broader RPA platform for integration. UiPath provides 400+ pre-built connectors for ERP, CRM, and accounting systems. The key differentiator is deep enterprise integration — once a document is processed, UiPath RPA can push data to any system with a GUI or API. However, this requires UiPath licensing ($25+/user/month) and significant automation infrastructure. For organizations already invested in UiPath, integration is seamless. For everyone else, it's a heavy dependency.

### Kofax Integration Hub

Kofax (Tungsten Automation) provides integration capabilities as part of its process orchestration platform. Kofax Connect provides pre-built adapters for SAP, Oracle, Salesforce, and other enterprise systems. Integration is configured through Kofax's workflow designer — documents flow automatically from capture through extraction to target systems. The key differentiator is enterprise-grade integration orchestration with full audit trail. Cost: part of $100K+ enterprise license.

### ABBYY Web Services API

ABBYY provides a Web Services API for integration with external systems. It supports HTTP/HTTPS, SOAP, and REST interfaces. ABBYY FlexiCapture can push extracted data to external databases, ERP systems, or any system with an API. The key differentiator is deep integration with the ABBYY ecosystem and support for complex enterprise architectures (multi-server, distributed). However, integration requires ABBYY-specific development effort and SDK knowledge.

### Azure Logic Apps + Document Intelligence

Microsoft's approach is to use Logic Apps for integration orchestration. Document Intelligence sends extracted data to Logic Apps, which route it to target systems (Dynamics 365, SAP via connectors, custom APIs). The key advantage is tight Microsoft ecosystem integration — Dynamics 365, Power Automate, Azure services. Pricing: Logic Apps ~$0.05/run. For Microsoft-centric organizations, this is the natural choice.

### AWS Lambda + Step Functions + Textract

AWS's approach is serverless: Textract sends extracted data to Lambda functions, which route to target systems via API calls. Step Functions orchestrates complex workflows (e.g., extract → validate → route to SAP → notify Slack). The key advantage is infinite scalability and pay-per-use pricing. However, it requires significant AWS development effort and Lambda functions don't scale well for high-throughput document processing (>100K documents/day).

---

## Implementation Roadmap

### Phase 1: Core Integration Framework (2-3 weeks) — P2

1. Define `IntegrationConnector`, `IntegrationEndpoint`, `IntegrationJob` entities
2. Implement connector CRUD endpoints
3. Implement job creation on document processing completion
4. Implement basic HTTP POST integration (generic REST connector)
5. Implement job status tracking (PENDING, SENT, SUCCESS, FAILED)
6. Implement error recording on failure

### Phase 2: Field Mapping & Transformation (2-3 weeks) — P2

1. Define `IntegrationMapping` entity
2. Implement mapping engine (map extraction fields to target fields)
3. Implement transformation rules (uppercase, currency convert, date format)
4. Implement mapping template loading by vendor (pre-configured SAP/Dynamics/QuickBooks templates)
5. Add required field validation before sending

### Phase 3: Retry, Dead-Letter & Error Handling (2-3 weeks) — P2

1. Implement retry logic with exponential backoff (configurable max retries)
2. Implement dead-letter queue for permanently failed jobs
3. Implement error notification (email, webhook, UI)
4. Implement manual retry interface for dead-letter jobs
5. Add error categorization (CONNECTION_ERROR, AUTH_ERROR, VALIDATION_ERROR, etc.)

### Phase 4: Webhooks & Event-Driven Integration (3-4 weeks) — P2

1. Implement webhook subscription system
2. Implement webhook delivery with retry
3. Add event types (DOCUMENT_PROCESSED, EXTRACTION_FAILED, REVIEW_COMPLETE)
4. Implement webhook signature verification (HMAC)
5. Add webhook delivery analytics (success rate, latency)

### Phase 5: Pre-Built Connectors (4-6 weeks) — P2

1. Implement SAP connector (IDoc or REST API)
2. Implement QuickBooks Online connector (OAuth2 + REST API)
3. Implement Salesforce connector (REST API)
4. Implement Xero connector (OAuth2 + REST API)
5. Implement generic CSV/JSON file export connector
6. Implement message queue connector (RabbitMQ, Kafka)

---

## Key Takeaway for Buyers

Integration is where document processing becomes a business enabler. A buyer needs to know: *When a document is processed, does the data flow automatically to where it belongs — and when it fails, do I get immediate notification with full context?* RERP's advantage is self-hosted integration with zero per-connector licensing, OpenAPI-defined field mappings that auto-generate from specs, and configurable retry/dead-letter logic. The immediate priority: define the IntegrationConnector, IntegrationEndpoint, and IntegrationJob entities, implement basic HTTP POST integration, and build the retry and dead-letter queue system. Everything else builds on this foundation.
