# OpenAPI Enrichment Analysis

**Date**: 2026-04-24
**Objective**: Assess current state of RERP OpenAPI specifications and define the enrichment strategy to reach world-class ERP API quality.
**Reference**: Hauliage (`/home/casibbald/Workspace/microscaler/hauliage/openapi/`) used as working reference for quality bar.

---

## Executive Summary

RERP has **98 OpenAPI spec files** across **27 suites** with a total of **~65,000 lines** of YAML. However, **~70% (62 of 98) are pure CRUD stubs** with zero data models. The architecture is sound (multi-suite, multi-service, BFF-per-suite) but the specs are functionally empty -- they describe *where* resources live, not *what* resources are.

This document maps the gap between current state and world-class ERP API quality, then defines a prioritized enrichment plan.

---

## 1. Current State Assessment

### 1.1 Scale

| Metric | Value |
|---|---|
| Total OpenAPI files | 98 |
| Suite-level specs | 27 (one per suite, acting as BFF gateways) |
| Sub-service specs | 52 (actual microservice specs) |
| Accounting sub-services | 9 (most mature suite) |
| Total endpoints | ~727 across all specs |
| Total data schemas | ~199 across all specs |
| Total lines of YAML | ~65,000 |

### 1.2 Quality Distribution

By **schema richness** (the most important signal of functional depth):

| Level | Count | % | Characterization |
|---|---|---|---|
| **Empty (0 schemas)** | 62 | 63% | Pure CRUD: list/create/get/update/delete for 1-3 resources |
| **Low (1-3 schemas)** | 26 | 27% | Minimal models, often just Error or single DTO |
| **Medium (4-15 schemas)** | 9 | 9% | Substantive models (accounting sub-services, HR suite) |
| **High (16+ schemas)** | 1 | 1% | accounting/openapi.yaml suite BFF with 87 schemas |

By **feature richness**:

| Feature | Count | % |
|---|---|---|
| Has state machine / phase transitions | 0 | 0% |
| Has nested resource paths (depth >= 3) | 9 | 9% |
| Has action endpoints (publish, approve, cancel) | 0 | 0% |
| Has tags on operations | 0 | 0% |
| Has request body schemas | 0 | 0% |
| Has response schemas | 0 | 0% |
| Has examples | 0 | 0% |

### 1.3 What Current Specs Actually Look Like

Every sub-service follows the identical pattern:

```yaml
paths:
  GET /leads:
    summary: List leads
    operationId: list_leads
    parameters:    # <-- generic pagination params, no type info
    responses:
      200: List of leads  # <-- no response schema!
  
  POST /leads:
    summary: Create lead
    operationId: create_lead
    responses:
      201: Lead created  # <-- no response schema!
      400: Invalid request
  
  GET /leads/{id}: ...
  PUT /leads/{id}: ...
  DELETE /leads/{id}: ...
```

Key deficiencies:
- **No schemas** for request bodies or responses
- **No tags** on operations (spec has a `tags: []` at top level but operations aren't tagged)
- **No security schemes** anywhere
- **No examples** on any field or response
- **No parameter types** (query params show as `? in ? (any)`)
- **No error schemas** (each operation just says `400: Invalid request`)
- **No validation constraints** (minLength, pattern, enum, format)
- **Spelling errors**: `opportunitys` (should be `opportunities`), `entrys` (should be `entries`)

### 1.4 What Hauliage Does Better

The Hauliage `consignments` spec (1,091 lines, 13 schemas, 11 endpoints) shows what a functional spec looks like:

| RERP Stub | Hauliage Reference |
|---|---|
| `GET /leads` | `GET /jobs` with 10 typed query params (status, page, limit, carrier, origin, destination, date_from, date_to, weight_min, weight_max) |
| `POST /leads` | `POST /jobs` with CreateJobRequest schema (13 properties, 4 required) |
| No state management | `PUT /jobs/{id}/phase` - state machine transitions |
| No sub-workflows | `POST /jobs/drafts` -> `POST /jobs/drafts/{id}/publish` (draft->publish workflow) |
| No document handling | `GET /jobs/{id}/documents` + `POST /jobs/{id}/documents` |
| No telemetry | `GET /jobs/{id}/telemetry` - live GPS tracking |
| No AI features | `POST /ai/tidy-text` - AI-assisted text formatting |
| 1 generic Error schema | 13 domain schemas (Job, CreateJobRequest, JobDraft, TrackingDocument, etc.) |
| 1 server | 2 servers (dev + proxy) |

The Hauliage `company` spec (776 lines, 12 schemas) shows nested resource patterns:

```
GET  /organizations/me                    -- current org profile
GET  /organizations/me/team               -- team members
POST /organizations/me/team               -- invite member
GET  /organizations/me/dashboard          -- metrics overlay
GET  /organizations/me/preferences        -- settings
GET  /organizations/me/fleet              -- vehicle categories
GET  /organizations/me/addresses          -- address book
POST /organizations/me/addresses          -- save address
GET  /organizations/me/compliance/documents -- compliance files
```

### 1.5 What's Actually Good

- **Architecture is correct**: Multi-suite, multi-service, BFF-per-suite. This is enterprise-grade design.
- **Accounting suite is the most mature**: 87 schemas in the suite BFF, 6-12 schemas per sub-service. The models (Invoice, InvoiceLine, CreateInvoiceRequest, etc.) are reasonably structured.
- **Sub-service granularity is right**: 9 accounting microservices, 7 HR microservices, etc. This matches the RERP_MUSINGS modular vision.
- **BFF generation infrastructure exists**: `bff-suite-config.yaml` files with proper service routing.
- **Port registry and service discovery are in place**.

---

## 2. Gap Analysis: What World-Class ERP APIs Need

Based on analysis of Odoo, ERPNext, SAP, and the Hauliage reference, a world-class ERP API needs:

### 2.1 Data Model Completeness

**Current**: 199 schemas across 98 files (avg 2 per spec). Many have 0.
**Target**: 30-80 schemas per complex service (e.g., accounting, HR, manufacturing).

Every resource endpoint needs:
1. **Domain entity schema** -- full model with all properties
2. **Create request schema** -- input validation, required/optional fields
3. **Update request schema** -- partial update support
4. **List response schema** -- paginated wrapper with metadata
5. **Detail response schema** -- hydrated entity with nested collections
6. **Error response schema** -- consistent error envelope
7. **Relationship schemas** -- for referenced entities (foreign keys)

### 2.2 Endpoint Richness

**Current**: CRUD-only (GET/POST/PUT/DELETE). No business logic.
**Target**: Business operation endpoints alongside CRUD.

Types of endpoints needed:
- **CRUD**: list, create, get, update, delete (baseline)
- **State machine transitions**: `PUT /invoices/{id}/approve`, `PUT /invoices/{id}/cancel`
- **Actions**: `POST /orders/{id}/submit`, `POST /invoices/{id}/send`, `POST /jobs/{id}/assign`
- **Bulk operations**: `POST /products/bulk-update`, `POST /employees/bulk-import`
- **Nested resources**: `GET /customers/{id}/orders`, `GET /orders/{id}/lines`
- **Aggregations**: `GET /accounting/reports/pnl`, `GET /hr/reports/payroll-summary`
- **Search**: `POST /products/search`, `POST /customers/search` with complex query filters
- **File operations**: upload, download, attach
- **Webhook/event endpoints**: `POST /webhooks/{id}/test`, `GET /events/{id}/logs`
- **AI/intelligence endpoints**: `POST /products/suggest-price`, `POST /invoices/extract-data`

### 2.3 Query & Filtering

**Current**: Generic pagination params only.
**Target**: Domain-specific query capabilities.

Examples:
- **Filtering**: `GET /invoices?status=pending&customer_id=xxx&date_from=2024-01-01`
- **Sorting**: `GET /products?sort=name,asc&page=2&limit=50`
- **Field selection**: `GET /customers?fields=name,email,phone`
- **Search**: `POST /customers/search` with full-text query
- **Range queries**: `GET /products?price_min=10&price_max=100`
- **Status filters**: `GET /jobs?status=open,published`
- **Aggregation queries**: `GET /inventory/summaries?by=warehouse`

### 2.4 Security & Authentication

**Current**: Zero security schemes defined anywhere.
**Target**: Full OAuth2/JWT security model.

Every spec needs:
1. `components.securitySchemes` with JWT bearer token scheme
2. `security` at spec level (global default)
3. Per-operation security overrides for public endpoints
4. Role-based access annotations
5. Tenant isolation (multi-tenancy) patterns

### 2.5 Schema Constraints & Validation

**Current**: No constraints on any fields.
**Target**: Full validation layer (layer 1 of 3-layer defense).

Every schema needs:
- `required` arrays
- `enum` values for status/state fields
- `pattern` for IDs, codes, phone numbers
- `format` for dates, emails, UUIDs, amounts
- `minimum`/`maximum` for numeric fields
- `minLength`/`maxLength` for strings
- `uniqueItems` for arrays
- `description` on every property
- `example` values on key fields

### 2.6 Documentation & Discovery

**Current**: Title and description at spec level only. No examples.
**Target**: Rich, discoverable, self-documenting API.

Every spec needs:
- `tags` array with descriptions for every resource
- `summary` on every operation
- `description` on every operation (business context)
- `requestBody` with `$ref` to schema + description
- `responses` with `$ref` to schema for 200/201
- Error response schemas for 400, 401, 403, 404, 409, 422, 500
- `examples` section with realistic data
- `callbacks` for webhook subscriptions
- `x-code-samples` for SDK generation

---

## 3. Enrichment Strategy: Prioritized Work Plan

### Phase 1: Foundation (Weeks 1-4)

**Goal**: Make the 62 "empty" specs have basic data models.

#### 1.1 Standardize the error model
Add to every spec:
```yaml
components:
  schemas:
    Error:
      type: object
      required: [code, message]
      properties:
        code:
          type: string
          description: Machine-readable error code
        message:
          type: string
          description: Human-readable error message
        details:
          type: object
          description: Additional error context
```

#### 1.2 Add basic entity schemas to all CRUD stubs
For each empty spec, generate:
- **Entity schema**: All properties the resource has (UUID, timestamps, relationships)
- **Create request**: Same as entity minus ID, with required fields marked
- **Update request**: All optional fields
- **List response**: `{ items: [], total: 0, page: 0, per_page: 0 }`

#### 1.3 Add security schemes to all specs
```yaml
components:
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
      bearerFormat: JWT
```

#### 1.4 Add tags to all operations
Each resource gets its own tag. Example for CRM core:
```yaml
tags:
  - name: leads
    description: Lead management -- potential sales opportunities
  - name: contacts
    description: Contact entities (people)
  - name: opportunities
    description: Sales pipeline opportunities
```

### Phase 2: Domain Enrichment (Weeks 5-12)

**Goal**: Add business logic endpoints to core enterprise services.

#### 2.1 Priority services (highest business impact)
1. **Accounting** (already has models, needs operations): state machines, approvals, reports, journal entries, reconciliations
2. **Sales** (quotation + order): quote-to-cash workflow, price lists, promotions, order status machine
3. **Inventory** (core + warehouse): stock movements, transfers, adjustments, cycle counts
4. **HR** (core + payroll): employee lifecycle, leave approval workflows, payroll processing
5. **Purchase** (core + vendor): PO lifecycle, vendor catalog, RFQ workflow

#### 2.2 Business operation patterns to implement

Each service needs a standard set of operations beyond CRUD:

| Pattern | Example | Purpose |
|---|---|---|
| State machine | `PUT /invoices/{id}/approve` | Lifecycle transitions |
| Bulk action | `POST /invoices/approve` | Batch operations |
| Nested list | `GET /customers/{id}/invoices` | Resource relationships |
| Search | `POST /products/search` | Complex queries |
| Report | `GET /accounting/reports/pnl` | Aggregation |
| Export | `GET /invoices/export?format=pdf` | Data extraction |
| Webhook | `POST /webhooks/{id}/test` | Event subscription |

### Phase 3: Advanced Features (Weeks 13-20)

**Goal**: Add enterprise-grade features that differentiate from competitors.

#### 3.1 Cross-service operations
- **Order fulfillment**: `POST /sales/orders/{id}/fulfill` (triggers inventory check, warehouse pick, shipping)
- **Invoice reconciliation**: `POST /accounting/invoices/{id}/reconcile` (links payment, updates ledger)
- **Customer onboarding**: `POST /crm/leads/{id}/convert` (creates account, contact, and initial order)

#### 3.2 Analytics and BI endpoints
- Aggregation endpoints for dashboards
- Real-time metrics
- Scheduled report generation
- Export to CSV/PDF/Excel

#### 3.3 Integration patterns
- Webhook subscriptions (`POST /webhooks`, `POST /webhooks/{id}/events`)
- API versioning (`/api/v1/`, `/api/v2/`)
- Rate limiting indicators
- Batch endpoints (`POST /batch`)

#### 3.4 AI/ML integration points
- `POST /products/suggest-price` (pricing intelligence)
- `POST /invoices/extract` (OCR + NLP invoice processing)
- `POST /products/describe` (AI-generated product descriptions)
- `POST /customers/score` (lead scoring)

### Phase 4: Polish & Enterprise Grade (Weeks 21-28)

**Goal**: Reach world-class API documentation and developer experience.

#### 4.1 Complete schema enrichment
- Add examples to every property on every schema
- Add constraints (min, max, pattern, enum, format)
- Add descriptions to every endpoint
- Add `x-code-samples` for SDK generation

#### 4.2 OpenAPI best practices
- Consistent pagination patterns across all specs
- Consistent error response format
- HATEOAS links where appropriate
- Schema `$ref` reuse to avoid duplication
- Proper `409 Conflict` and `422 Unprocessable Entity` handling

#### 4.3 Developer experience
- OpenAPI Studio/Swagger UI setup with example data
- Postman collection generation
- SDK generation configs
- API changelog/versioning strategy

---

## 4. Reference: Target Schema Count by Service

Based on Odoo and ERPNext models, here's a reasonable target for schema counts:

| Service | Current | Target | Gap | Priority |
|---|---|---|---|---|
| **accounting** (suite BFF) | 87 | 120 | +33 | Done |
| **accounting/general-ledger** | 9 | 25 | +16 | P1 |
| **accounting/invoice** | 6 | 20 | +14 | P1 |
| **accounting/accounts-payable** | 12 | 18 | +6 | P1 |
| **accounting/accounts-receivable** | 12 | 18 | +6 | P1 |
| **accounting/bank-sync** | 12 | 20 | +8 | P2 |
| **accounting/budget** | 9 | 15 | +6 | P2 |
| **accounting/asset** | 10 | 15 | +5 | P2 |
| **accounting/financial-reports** | 8 | 25 | +17 | P2 |
| **accounting/edi** | 8 | 15 | +7 | P3 |
| **hr/core** | 345 lines | 20+ schemas | +15 | P1 |
| **hr/payroll** | 345 lines | 15+ schemas | +12 | P1 |
| **hr/recruitment** | 345 lines | 12+ schemas | +9 | P2 |
| **hr/leave** | 345 lines | 10+ schemas | +7 | P2 |
| **hr/appraisal** | 345 lines | 8+ schemas | +5 | P3 |
| **hr/skills** | 345 lines | 6+ schemas | +3 | P3 |
| **hr/attendance** | 241 lines | 8+ schemas | +5 | P3 |
| **sales/core** | 345 lines | 15+ schemas | +12 | P1 |
| **sales/order** | 345 lines | 20+ schemas | +16 | P1 |
| **sales/quotation** | 345 lines | 15+ schemas | +12 | P1 |
| **sales/subscription** | 345 lines | 12+ schemas | +9 | P2 |
| **sales/loyalty** | 345 lines | 10+ schemas | +7 | P3 |
| **inventory/core** | 345 lines | 18+ schemas | +14 | P1 |
| **inventory/warehouse** | 345 lines | 15+ schemas | +11 | P1 |
| **inventory/logistics** | 345 lines | 12+ schemas | +9 | P2 |
| **inventory/dropshipping** | 241 lines | 10+ schemas | +7 | P3 |
| **purchase/core** | 241 lines | 12+ schemas | +9 | P1 |
| **purchase/vendor** | 345 lines | 12+ schemas | +9 | P1 |
| **manufacturing/core** | 345 lines | 15+ schemas | +12 | P2 |
| **manufacturing/bom** | 345 lines | 12+ schemas | +9 | P2 |
| **manufacturing/production-planning** | 345 lines | 15+ schemas | +12 | P2 |
| **manufacturing/repair** | 241 lines | 10+ schemas | +7 | P3 |
| **manufacturing/subcontracting** | 241 lines | 8+ schemas | +5 | P3 |
| **crm/core** | 345 lines | 15+ schemas | +12 | P2 |
| **crm/automation** | 345 lines | 10+ schemas | +7 | P2 |
| **crm/livechat** | 345 lines | 8+ schemas | +5 | P3 |
| **product/catalog** | 516 lines | 20+ schemas | +14 | P1 |
| **product/pricing** | 386 lines | 12+ schemas | +8 | P1 |
| **product/tax** | 275 lines | 10+ schemas | +6 | P1 |
| **auth/idam** | 420 lines | 15+ schemas | +10 | P1 |
| **auth/rbac** | 406 lines | 15+ schemas | +10 | P1 |
| **infrastructure/gateway** | 295 lines | 10+ schemas | +7 | P1 |
| **infrastructure/integration-platform** | 420 lines | 15+ schemas | +10 | P2 |

---

## 5. Quality Checklist for Each Spec

Before marking a spec as "enriched", verify:

### Level 1: Basic Completeness (all specs)
- [ ] `components.securitySchemes` defined with JWT bearer auth
- [ ] `tags` array defined with descriptions
- [ ] All operations tagged
- [ ] All responses have `$ref` to schema (not just descriptions)
- [ ] All `POST`/`PUT` operations have `requestBody` with `$ref`
- [ ] Standard `Error` response schema defined
- [ ] `servers` array with dev and production URLs

### Level 2: Domain Completeness (business services)
- [ ] All domain entities have full schemas with properties
- [ ] Create/Update request schemas with required fields
- [ ] List response schemas with pagination metadata
- [ ] Status/state enums defined with valid values
- [ ] Field constraints (min, max, pattern, format)
- [ ] Example values on key fields
- [ ] `description` on every operation
- [ ] `summary` on every operation
- [ ] Relationships expressed via `$ref`

### Level 3: Business Logic (core services)
- [ ] State machine transitions defined
- [ ] Bulk operation endpoints
- [ ] Nested resource endpoints
- [ ] Search/filter/query patterns
- [ ] Report/aggregation endpoints
- [ ] File operations (upload/download/attach)
- [ ] Webhook/event endpoints

### Level 4: Enterprise Grade (all services)
- [ ] Examples on every property (top 20% of fields minimum)
- [ ] Consistent pagination across all list endpoints
- [ ] Consistent error envelope (400, 401, 403, 404, 409, 422, 500)
- [ ] HATEOAS links where appropriate
- [ ] `x-code-samples` for common operations
- [ ] API versioning documented
- [ ] OpenAPI Studio/Swagger UI configured with example data

---

## 6. Automation Opportunities

### 6.1 Code Generation
Use the pattern from Hauliage's `bff-generator` to create an `openapi-enricher` tool that:
1. Takes a service spec + entity model definitions
2. Generates enriched CRUD endpoints with proper schemas
3. Adds security schemes, tags, and standard responses
4. Outputs a complete spec ready for business logic enrichment

### 6.2 Schema Inference
From the Lifeguard entity models in `microservices/*/impl/src/models/`, auto-generate:
- OpenAPI schemas from Rust structs
- Field constraints from annotations (`#[column_type]`, `#[nullable]`, etc.)
- Relationships from `#[belongs_to]`, `#[has_many]`

### 6.3 Consistency Checks
Add CI validation that:
- Verifies all specs have security schemes
- Checks for consistent error response schemas
- Validates that every POST/PUT has a requestBody schema
- Ensures tags are defined and used
- Reports on schema count per spec

---

## 7. Comparison to Market Leaders

| Feature | RERP (Current) | RERP (Target) | Odoo | ERPNext |
|---|---|---|---|---|
| API-first design | Partial (spec exists) | Full | Partial | Partial |
| REST endpoints | CRUD only | Full lifecycle | CRUD + actions | CRUD + actions |
| Data models | 2 avg per spec | 15+ avg per spec | Rich | Moderate |
| Validation | None | Full constraint layer | Partial | Partial |
| Security | None | JWT/OAuth2 | JWT | Token-based |
| Documentation | Title only | Rich docs + examples | Good | Basic |
| State machines | None | Full lifecycle | Partial | Partial |
| Bulk operations | None | Supported | Limited | Limited |
| Search | None | Full-text + filters | Basic | Basic |
| Reports/BI | None | Aggregate endpoints | Limited | Basic |

---

## 8. Immediate Next Steps

1. **Pick 3 priority services** to enrich first (recommend: `sales/order`, `inventory/warehouse`, `hr/core`)
2. **Define the enrichment pattern** using Hauliage consignments as the reference template
3. **Build a spec enricher tool** that standardizes the basic enrichment (security, tags, error schemas, CRUD schemas)
4. **Manual business logic enrichment** on the 3 chosen services
5. **Iterate and replicate** to the remaining 95 specs

---

*This analysis should be treated as a living document. Update it as enrichment progresses.*
