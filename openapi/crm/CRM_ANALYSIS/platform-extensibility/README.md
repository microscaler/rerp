# Platform & Extensibility

> **Component:** Multi-tenancy, integrations, SDK, APIs, and developer ecosystem
> **Priority:** P4 — Important for enterprise but not for first buyers
> **Odoo Reference:** module system, ORM, access control rules, ir.model.access, ir.attachment

---

## The Pitch

**Buyer Question:** *Can I extend my CRM to fit my unique processes, or do I have to compromise and change my business?*

Every business has processes that don't fit neatly into a CRM's default model. Custom fields, integrations with ERP/finance tools, custom objects, webhook hooks, and developer APIs are what separate a point solution from a platform. A buyer needs to know: *If my business process requires a field that doesn't exist today, can I add it? If I need my CRM to push data to my inventory system, can I connect it?*

---

## What This Component Does

1. **Custom Fields** — Add fields to any entity without code changes
2. **Custom Objects** — Define new entity types (e.g., "Property" for real estate CRM)
3. **API-First Design** — Every operation exposed via REST API, covered by OpenAPI spec
4. **REST API Coverage** — Full CRUD on all entities with filtering, sorting, pagination
5. **GraphQL API** — Flexible queries for complex data fetching (optional)
6. **SDK Generation** — Auto-generate client SDKs from OpenAPI specs (TypeScript, Python, Go, Java)
7. **Webhooks** — Real-time notifications when entities change
8. **OAuth2/OIDC Auth** — Standard authentication with scoped permissions
9. **Rate Limiting** — Configurable per API key/client
10. **Sandbox/Dev Environment** — Isolated development instance with separate data
11. **API Versioning** — Backward-compatible API evolution
12. **Data Import/Export API** — Bulk operations for migration and backup
13. **Third-Party Marketplace** — Distribution for community-built integrations
14. **Plugin/Integration Framework** — Extensible architecture for custom logic
15. **Two-Way ERP Sync** — Bidirectional data sync with ERP systems
16. **Document Management** — Attach files to any record
17. **Single Sign-On (SSO)** — OIDC, SAML for enterprise authentication
18. **Role-Based Permissions** — Granular access control
19. **Field-Level Security** — Control visibility per field per role
20. **Audit Logging** — Who changed what, when (compliance requirement)
21. **Change Data Capture (CDC)** — Real-time change stream for event-driven architecture

---

## Entity Model

### Custom Field Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `entity` | String (64) | Yes | Target entity (crm.lead, crm.contact, etc.) |
| `name` | String (128) | Yes | Field name (machine-readable) |
| `label` | String (255) | Yes | Display label (human-readable) |
| `field_type` | Enum: [STRING, INTEGER, FLOAT, BOOLEAN, DATE, DATETIME, EMAIL, PHONE, URL, TEXT, HTML, SELECT, MANY2ONE, MANY2MANY] | Yes | Data type |
| `options` | JSON | No | Type-specific options (e.g., select values, related entity) |
| `required` | Boolean | No | Is field required? |
| `visible` | Boolean | Yes | Visible in API responses |
| `searchable` | Boolean | Yes | Included in search |
| `default_value` | String | No | Default value |
| `created_by` | Foreign Key: User | Yes | Who created |
| `created_at` | DateTime | Yes | When created |

### Webhook Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Webhook name |
| `url` | String (512) | Yes | Target URL for POST |
| `events` | JSON | Yes | Event types to listen for (create, update, delete on entities) |
| `secret` | String (255) | No | HMAC secret for signature verification |
| `is_active` | Boolean | Yes | Enable/disable |
| `headers` | JSON | No | Custom HTTP headers |
| `retry_count` | Integer | Yes | Max retry attempts |
| `last_triggered` | DateTime | No | When last fired |
| `last_status` | Enum: [PENDING, SUCCESS, FAILED, RETRYING] | No | Last delivery status |
| `created_at` | DateTime | Yes | Creation timestamp |

### Audit Log Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `entity` | String (64) | Yes | Entity type (crm.lead, etc.) |
| `entity_id` | UUID | Yes | Record ID |
| `action` | Enum: [CREATE, UPDATE, DELETE, READ, EXPORT, IMPORT] | Yes | Action performed |
| `user_id` | UUID | Yes | User who performed action |
| `changes` | JSON | Yes | Field changes (before/after) |
| `ip_address` | String (64) | No | Client IP |
| `user_agent` | String (512) | No | Browser/client |
| `created_at` | DateTime | Yes | When action occurred |

### API Key Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Key name (label) |
| `key_hash` | String (255) | Yes | Hashed API key (never stored in plaintext) |
| `scopes` | JSON | Yes | Allowed scopes (read, write, admin per entity) |
| `rate_limit` | Integer | No | Requests per minute |
| `ip_whitelist` | JSON | No | Allowed IP addresses |
| `is_active` | Boolean | Yes | Enable/disable |
| `last_used` | DateTime | No | When last used |
| `created_at` | DateTime | Yes | Creation timestamp |
| `expires_at` | DateTime | No | Expiration date |

### Integration Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Integration name (e.g., "QuickBooks", "SAP") |
| `type` | Enum: [ERP, ACCOUNTING, MARKETING, SUPPORT, CUSTOM] | Yes | Integration category |
| `config` | JSON | Yes | Configuration (credentials, endpoints) |
| `is_active` | Boolean | Yes | Enable/disable |
| `sync_direction` | Enum: [CRM_TO_EXTERNAL, EXTERNAL_TO_CRM, BIDIRECTIONAL] | Yes | Data flow |
| `last_sync` | DateTime | No | Last sync timestamp |
| `sync_status` | Enum: [IDLE, SYNCING, ERROR] | No | Current sync status |

---

## API Architecture

### REST API Coverage

Every entity in RERP CRM should be accessible via REST:

```
Standard CRUD:
  GET    /entities/{type}         — List records (with filtering)
  GET    /entities/{type}/{id}    — Get single record
  POST   /entities/{type}         — Create record
  PATCH  /entities/{type}/{id}    — Update record
  DELETE /entities/{type}/{id}    — Soft delete

Bulk Operations:
  POST   /entities/{type}/bulk    — Bulk create/update/delete
  GET    /entities/{type}/export   — Export to CSV/JSON
  POST   /entities/{type}/import   — Import from CSV/JSON

Search:
  GET    /entities/{type}/search/{query}  — Full-text search
  POST   /entities/{type}/filter          — Advanced filters

Relationships:
  GET    /entities/{type}/{id}/relationships  — All related entities
```

### Webhook Events

| Event | Payload | Use Case |
|-------|---------|----------|
| `lead.create` | Lead object | Create contact in external system |
| `lead.update` | Lead object + changes | Sync field changes |
| `lead.delete` | Lead ID | Archive in external system |
| `opportunity.won` | Opportunity object | Trigger invoice generation |
| `opportunity.lost` | Opportunity object | Log loss reason in analytics |
| `contact.create` | Contact object | Sync to email marketing tool |
| `subscription.renewal_due` | Subscription + alert | Notify rep of upcoming renewal |

Webhook payload example:
```json
{
  "event": "lead.create",
  "timestamp": "2026-05-10T14:30:00Z",
  "data": {
    "id": "abc123",
    "name": "Acme Corp Deal",
    "email_from": "john@acme.com",
    "expected_revenue": 50000,
    "stage_id": "qualified"
  }
}
```

### SDK Generation

From the OpenAPI spec, generate SDKs for popular languages:

```bash
# Generate TypeScript SDK
openapi-generator-cli generate -i openapi/crm/openapi.yaml -g typescript-axios -o sdk/typescript

# Generate Python SDK
openapi-generator-cli generate -i openapi/crm/openapi.yaml -g python -o sdk/python

# Generate Go SDK
openapi-generator-cli generate -i openapi/crm/openapi.yaml -g go -o sdk/go

# Generate Java SDK
openapi-generator-cli generate -i openapi/crm/openapi.yaml -g java -o sdk/java
```

---

## Required API Endpoints

### Custom Fields

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/custom-fields` | List all custom fields |
| `POST` | `/custom-fields` | Create custom field |
| `PATCH` | `/custom-fields/{id}` | Update custom field |
| `DELETE` | `/custom-fields/{id}` | Delete custom field |
| `GET` | `/entities/{type}/custom-fields` | Custom fields for an entity |

### Webhooks

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/webhooks` | List all webhooks |
| `POST` | `/webhooks` | Create webhook |
| `PATCH` | `/webhooks/{id}` | Update webhook |
| `DELETE` | `/webhooks/{id}` | Delete webhook |
| `POST` | `/webhooks/{id}/test` | Test webhook delivery |
| `GET` | `/webhooks/{id}/logs` | Delivery logs |

### API Keys & Auth

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api-keys` | List API keys |
| `POST` | `/api-keys` | Generate new API key |
| `DELETE` | `/api-keys/{id}` | Revoke API key |
| `GET` | `/auth/validate` | Validate API key |

### Audit & Compliance

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/audit-log` | Audit log entries |
| `GET` | `/audit-log/{entity}/{id}` | Audit log for specific record |
| `GET` | `/audit-log/export` | Export audit log |

### Integrations

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/integrations` | List integrations |
| `POST` | `/integrations` | Configure integration |
| `POST` | `/integrations/{id}/sync` | Trigger manual sync |
| `GET` | `/integrations/{id}/status` | Sync status |

### Data Operations

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/data/import` | Bulk import from CSV/JSON |
| `GET` | `/data/export` | Bulk export to CSV/JSON |
| `GET` | `/data/schemas` | Entity schemas (for import/export validation) |

---

## Competitive Positioning

### Where RERP Wins
- **OpenAPI-first from day one** — Every entity, endpoint, schema, and parameter is machine-readable. Enables automatic SDK generation, API testing, documentation, and contract testing.
- **Rust performance at scale** — Sub-millisecond API latency. Bulk operations on 100,000+ records complete in seconds.
- **Self-hosted extensibility** — No app marketplace approval process. Write integration and deploy.
- **No vendor lock-in** — Own the code, data, and infrastructure. No per-API-call pricing.

### Where RERP Lags
- **No custom objects** — Salesforce lets you model any entity without code. RERP requires spec changes.
- **No webhook system** — No way for external systems to react to CRM events.
- **No sandbox** — No isolated development environment.
- **No app marketplace** — No ecosystem of pre-built integrations.
- **No audit logging** — No trail for compliance (SOC 2, HIPAA, GDPR).

---

## Competitive Intelligence Deep Dive

### Salesforce Platform (The Platform King — $25–$330/user/month)
**AppExchange** has 7,000+ vetted apps. **Custom Objects** with 10,000+ fields per object. **Apex** for server-side logic. **Sandbox** environments for isolated testing. **Change Sets** and **DevOps Center** for CI/CD. **API** covers 95%+ of platform with REST, SOAP, Bulk, Streaming, GraphQL. **Platform Events** for real-time event-driven architecture. The ecosystem is the moat.

### Microsoft Power Platform ($5–$40/user/month add-on)
**Power Apps** for no-code/low-code apps. **Power Automate** for 700+ connector workflows. **Dataverse** as unified data platform. **Teams integration** for real-time collaboration. **Azure integration** for AI/ML and serverless. The advantage: if you can use Excel, you can build a Power App.

### HubSpot App Marketplace (Free → $1,800+/month)
**2,000+ integrations** from accounting to ecommerce to support. **Webhooks** for real-time events. **CRM SDK** with TypeScript, Python, JS libraries. **Developer Hub** with docs and sandboxes. **Private Apps** for internal integrations. Low barrier: build, list, tap into 188,000+ customers.

---


### ServiceNow: The Platform Moat
ServiceNow is not a CRM product — it's an **AI-powered platform** that includes CRM as one of many modules. **App Engine** lets you build custom apps without code. **IntegrationHub** connects to 200+ systems (SAP, Oracle, Salesforce, Slack, Teams, Zoom, Five9, Xactly). **AI Agent Fabric** enables agent-to-agent communication with third parties (Adobe, Box, Accenture, Cisco, Google Cloud, IBM, Microsoft). **RaptorDB** provides high-speed data layer for workflow performance. **Gartner Magic Quadrant Leader** (2nd consecutive year). **Gap vs. Salesforce:** Salesforce AppExchange has more apps, but ServiceNow's native integrations are deeper. **Gap vs. Microsoft:** No equivalent to Microsoft's ecosystem breadth. **Unique strength:** Platform convergence — ITSM + CSM + HRSD + SecOps + CRM + ITOM all on one platform with shared data, shared AI, shared governance. This is the structural advantage over siloed CRMs.
## Implementation Roadmap

### Phase 1: API Foundation (2-3 weeks)
1. Add custom fields endpoint (POST /custom-fields)
2. Implement CRUD for custom field values
3. Add rate limiting to all endpoints
4. Implement OAuth2 client credentials flow
5. Add API key management endpoint

### Phase 2: Webhooks & Audit (2-3 weeks)
1. Define webhook entity and endpoint
2. Implement webhook trigger on entity create/update/delete
3. Implement audit logging (who changed what, when)
4. Add API versioning (header or path)
5. Implement change data capture endpoint

### Phase 3: Extensibility (3-4 weeks)
1. Custom objects system (user-defined entity model)
2. SDK generation pipeline (TypeScript, Python, Go, Java)
3. Sandbox/dev environment
4. Data import/export API (CSV, JSON, bulk)
5. Two-way ERP sync endpoint

### Phase 4: Enterprise Features (3-4 weeks)
1. SSO integration (OIDC, SAML)
2. Field-level security
3. GraphQL API (optional)
4. App marketplace infrastructure
5. Plugin/integration framework

---

## Key Takeaway for Buyers

Platform extensibility separates a product from a platform. RERP's OpenAPI-first architecture is a genuine advantage: the entire data model is machine-readable, enabling SDK generation, contract testing, and automatic documentation. The path forward: start with API-first extensibility (custom fields, webhooks, audit logs), then build up to custom objects and the sandbox environment.
