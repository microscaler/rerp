# RERP OpenAPI Specification Audit

Date: 2026-04-24
Scope: All 27 suites, 71 microservices OpenAPI specs
Benchmark: Hauliage (17 suites, 40+ services), world-class ERP systems (Odoo, NetSuite, SAP)

---

## Executive Summary

RERP's 27-suite OpenAPI landscape is **structurally complete but functionally immature**. Only the accounting suite has meaningful spec quality. 26 other suites have skeleton specs with minimal endpoints, no security, no pagination, no error patterns. This is the single biggest gap preventing RERP from being production-ready.

### Scorecard

| Area | RERP Current | Target | Gap |
|------|-------------|--------|-----|
| Suites with BFF config | 1/27 (accounting) | All 27 | Critical |
| Suites with security schemes | 1/27 | All 27 | Critical |
| Suites with tags | 1/27 | All 27 | High |
| Suites with pagination | 1/27 | All 27 | High |
| Suites with error schemas | 1/27 | All 27 | High |
| Suites with health endpoints | 1/27 | All 27 | Medium |
| Endpoints per service (avg) | ~2.6 | 10-20 | Critical |
| Response codes per endpoint | 2-5 | 4-8 (incl 422/500) | Medium |

### Key Gaps

1. **26/27 suites lack BFF configuration** — Only accounting has `bff-suite-config.yaml`. Without BFF configs, the BFF generator cannot produce aggregated specs for 26 suites.

2. **Security schemes only in general-ledger** — 70/71 services have no `securitySchemes` in their OpenAPI. Only `general_ledger` defines `bearerAuth`. Every service needs at minimum:
   - `bearerAuth` (JWT/OAuth2)
   - `bearerAuth` in global `security`
   - Per-operation security overrides where needed (admin endpoints)

3. **No tags across 26/27 suites** — Tags are essential for:
   - API documentation generation
   - BRRTRouter route grouping
   - Access control (role-based path matching)
   - Frontend navigation generation

4. **Pagination only in general-ledger** — List endpoints across all other services lack:
   - `page` (integer, default 1)
   - `limit` (integer, default 20, max 100)
   - Proper `200` response with `type: array` + `items`

5. **Error response patterns missing** — Most endpoints define:
   - 200, 201, 204, 400, 404
   - Missing: 401 (unauthorized), 403 (forbidden), 422 (validation), 500 (server error)

6. **Endpoint coverage is minimal** — Average ~2.6 endpoints per service. World-class ERP services have 10-20+ endpoints each. For example:
   - General Ledger (RERP's most mature): 21 endpoints ✓
   - Invoice (RERP): 4 endpoints ✗ (should be 15+)
   - Consignments (Hauliage): 11 endpoints

---

## Detailed Suite Analysis

### Accounting Suite (1 BFF config, 9 services, 141 endpoints) — **MATURE**

| Service | Endpoints | Security | Tags | Pagination | Error Schemas | Status |
|---------|-----------|----------|------|------------|---------------|--------|
| general-ledger | 21 | ✓ bearerAuth | ✓ | ✓ | ✓ | Production-ready |
| invoice | 4 | ✗ | ✗ | ✗ | ✗ | Skeleton |
| accounts-receivable | 6 | ✗ | ✗ | ✗ | ✗ | Skeleton |
| accounts-payable | 6 | ✗ | ✗ | ✗ | ✗ | Skeleton |
| bank-sync | 5 | ✗ | ✗ | ✗ | ✗ | Skeleton |
| asset | 6 | ✗ | ✗ | ✗ | ✗ | Skeleton |
| budget | 5 | ✗ | ✗ | ✗ | ✗ | Skeleton |
| edi | 6 | ✗ | ✗ | ✗ | ✗ | Skeleton |
| financial-reports | 6 | ✗ | ✗ | ✗ | ✗ | Skeleton |
| bff | N/A | N/A | N/A | N/A | N/A | Configured |

**Accounting issues:** Only general-ledger is production-ready. 8 services are skeleton specs needing:
- Security schemes + global security
- Tags for route grouping
- Pagination on list endpoints
- 422/401/403/500 error responses
- Health/ready endpoints

### 26 Other Suites — **SKELTON**

All 26 non-accounting suites have the same fundamental problems:

| Suite | Services | Endpoints | BFF Config | Security | Tags | Pagination |
|-------|----------|-----------|------------|----------|------|------------|
| ai | 2 | 30 | ✗ | ✗ | ✗ | ✗ |
| analytics | 3 | 45 | ✗ | ✗ | ✗ | ✗ |
| appointments | 1 | 15 | ✗ | ✗ | ✗ | ✗ |
| approvals | 1 | 15 | ✗ | ✗ | ✗ | ✗ |
| auth | 2 | 37 | ✗ | ✗ | ✗ | ✗ |
| automation | 1 | 15 | ✗ | ✗ | ✗ | ✗ |
| crm | 3 | 45 | ✗ | ✗ | ✗ | ✗ |
| data | 1 | 15 | ✗ | ✗ | ✗ | ✗ |
| documents | 1 | 15 | ✗ | ✗ | ✗ | ✗ |
| esg | 1 | 10 | ✗ | ✗ | ✗ | ✗ |
| field-service | 1 | 15 | ✗ | ✗ | ✗ | ✗ |
| helpdesk | 2 | 30 | ✗ | ✗ | ✗ | ✗ |
| hr | 7 | 100 | ✗ | ✗ | ✗ | ✗ |
| infrastructure | 2 | 31 | ✗ | ✗ | ✗ | ✗ |
| inventory | 4 | 55 | ✗ | ✗ | ✗ | ✗ |
| iot | 1 | 15 | ✗ | ✗ | ✗ | ✗ |
| localization | 2 | 30 | ✗ | ✗ | ✗ | ✗ |
| manufacturing | 5 | 65 | ✗ | ✗ | ✗ | ✗ |
| marketing | 3 | 45 | ✗ | ✗ | ✗ | ✗ |
| marketplace | 2 | 25 | ✗ | ✗ | ✗ | ✗ |
| pos | 2 | 30 | ✗ | ✗ | ✗ | ✗ |
| product | 3 | 52 | ✗ | ✗ | ✗ | ✗ |
| project | 2 | 25 | ✗ | ✗ | ✗ | ✗ |
| purchase | 2 | 25 | ✗ | ✗ | ✗ | ✗ |
| sales | 5 | 70 | ✗ | ✗ | ✗ | ✗ |
| website | 3 | 45 | ✗ | ✗ | ✗ | ✗ |

---

## Recommended OpenAPI Spec Template

Every RERP microservice spec should include:

### 1. Security (mandatory)

```yaml
components:
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
      bearerFormat: JWT
    apiKeyAuth:
      type: apiKey
      in: header
      name: X-API-Key

security:
  - bearerAuth: []
```

### 2. Tags (mandatory for BRRTRouter grouping)

```yaml
# Each operation needs:
tags:
  - <suite-name>          # e.g., accounting, sales
  - <service-name>        # e.g., general-ledger
```

### 3. Pagination (on all list/search endpoints)

```yaml
parameters:
  - name: page
    in: query
    schema:
      type: integer
      default: 1
      minimum: 1
  - name: limit
    in: query
    schema:
      type: integer
      default: 20
      minimum: 1
      maximum: 100
  - name: sort
    in: query
    schema:
      type: string
      default: created_at
  - name: order
    in: query
    schema:
      type: string
      enum: [asc, desc]
      default: desc
```

### 4. Error Responses (on all mutations)

```yaml
responses:
  '400':
    description: Bad request
    content:
      application/json:
        schema:
          $ref: '#/components/schemas/ErrorResponse'
  '401':
    description: Unauthorized
  '403':
    description: Forbidden
  '404':
    description: Not found
  '422':
    description: Validation error
    content:
      application/json:
        schema:
          $ref: '#/components/schemas/ValidationError'
  '500':
    description: Internal server error
```

### 5. Error Schemas (shared across all specs)

```yaml
components:
  schemas:
    ErrorResponse:
      type: object
      properties:
        error:
          type: string
        message:
          type: string
        code:
          type: string
        details:
          type: array
          items:
            type: object
    
    ValidationError:
      type: object
      properties:
        error:
          type: string
          enum: [VALIDATION_ERROR]
        message:
          type: string
        field_errors:
          type: array
          items:
            type: object
            properties:
              field:
                type: string
              message:
                type: string
```

### 6. Health/Ready Endpoints

```yaml
paths:
  /health:
    get:
      tags:
        - <service-name>
      summary: Health check
      operationId: getHealth
      responses:
        '200':
          description: Service is healthy
          content:
            application/json:
              schema:
                type: object
                properties:
                  status:
                    type: string
                  uptime_seconds:
                    type: number
        '503':
          description: Service unhealthy
```

---

## Prioritized Work Plan

### Phase 1: Foundation (Week 1-2)
1. **Add security schemes to ALL 27 suite specs** — Standardize on `bearerAuth` JWT
2. **Add tags to ALL 71 services** — Consistent naming: `{suite-name}` + `{service-name}`
3. **Create shared error schemas** — `ErrorResponse`, `ValidationError` as reusable components
4. **Add BFF configs to all 26 remaining suites** — Follow accounting pattern

### Phase 2: Endpoint Enrichment (Week 3-6)
5. **Add pagination to all list endpoints** — Standard pagination parameters
6. **Add error response codes to all mutations** — 400, 401, 403, 404, 422, 500
7. **Add health endpoints to all services** — `/health` GET
8. **Enrich accounting services** (invoice, accounts-receivable, etc.) — 4-6 endpoints → 15+ endpoints each

### Phase 3: World-Class Patterns (Week 7-10)
9. **Add idempotency keys** to all POST/PUT/DELETE endpoints
10. **Add OpenAPI extensions** for:
    - `x-resource-name` — Frontend identification
    - `x-sortable-fields` — List view generation
    - `x-filterable-fields` — Search/filter UI generation
11. **Cross-service references** — `$ref` to shared schemas across suites
12. **Add webhook event schemas** — For integration points

### Phase 4: Validation & Testing (Week 11-12)
13. **Run brrtrouter-gen lint on all specs**
14. **Generate stubs for all 71 services**
15. **Validate BFF aggregation for all suites**
16. **Frontend generator integration test**

---

## Comparison: RERP vs World-Class ERP OpenAPI Patterns

### Odoo Enterprise (Reference)
- Each model has CRUD + search + action endpoints (20-40 per model)
- Full pagination with `domain`, `fields`, `order`, `limit`, `offset`
- Per-operation security via `groups` attribute
- Rich error responses with field-level validation
- Webhook event definitions

### NetSuite SuiteTalk (Reference)
- SOAP-based but pattern-relevant
- Standardized query API (`Query`, `QueryAll`, `Search`)
- Full CRUD + bulk operations
- Complex error hierarchies
- Session-based auth + OAuth2

### RERP Target (After Phase 3)
- Each service: 15-30 endpoints (CRUD + search + actions + exports)
- Standard pagination + filtering + sorting
- Bearer auth with per-operation scope support
- Structured error responses with field validation
- Webhook event definitions for integrations

---

## Notes on Hauliage Comparison

Hauliage's specs are also surprisingly bare compared to Odoo/NetSuite:
- No security schemes in consignments, fleet, etc.
- No tags in most specs
- No pagination on list endpoints
- Minimal error response patterns

This suggests hauliage is **similarly immature at the OpenAPI level** but has compensated with:
- BFF layer that handles auth at the proxy level
- Frontend that doesn't rely on OpenAPI-driven generation as much
- More mature service implementations (Rust code)

RERP should aim to be **ahead of hauliage at the spec layer** since the frontend is OpenAPI-driven (via vite proxy / BFF generation).
