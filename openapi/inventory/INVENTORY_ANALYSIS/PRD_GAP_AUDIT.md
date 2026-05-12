# Inventory Suite — PRD & Gap Audit

> **Date:** 2026-05-12
> **Status:** Blocking — 3 of 6 microservices are non-functional stubs
> **Scope:** Inventory service specs need feature parity with stock/stock-movements/stock-valuations

---

## Current State

Six microservices in the inventory suite. Three are complete, three are stubs:

| Service | Ops | Schemas | Security | Tags | impl | Status |
|---------|-----|---------|----------|------|------|--------|
| stock | 7 | 7 ✓ | ✓ | ✓ | 4/7 | **Operational** |
| stock-movements | 7 | 7 ✓ | ✓ | ✓ | 5/7 | **Operational** |
| stock-valuations | 9 | 7 ✓ | ✓ | ✓ | 5/9 | **Operational** |
| warehouse | 15 | **0** | **NONE** | **0** | **0/15** | **STUB** |
| dropshipping | 10 | **0** | **NONE** | **0** | **0/10** | **STUB** |
| logistics | 15 | **0** | **NONE** | **0** | **0/15** | **STUB** |

---

## Gap Analysis

### Gap 1: Missing Entity Schemas (3 services, BLOCKING)
**Severity:** BLOCKING — No code generation possible without schemas.
**Affected:** warehouse, dropshipping, logistics
**Details:** All paths reference `$ref` to schemas that don't exist in `components.schemas`.
**Fix:** Define all entity schemas (CRUD types, enums, request/response bodies) for each service.

### Gap 2: Missing bearerAuth Security (3 services, HIGH)
**Severity:** HIGH — Endpoints are unsecured; no 401 responses on any operation.
**Affected:** warehouse, dropshipping, logistics
**Details:** No `securitySchemes` defined, no `security` field on spec, no `401` response on any operation.
**Fix:** Add `bearerAuth` security scheme and apply to all operations.

### Gap 3: Incomplete Response Codes (3 services, HIGH)
**Severity:** HIGH — Missing 401 UNAUTHORIZED, 403 FORBIDDEN, 409 CONFLICT, 500 SERVER ERROR.
**Affected:** warehouse, dropshipping, logistics
**Details:** Only 200, 201, 204, 400, 404 present. No 401, 403, 409, 500 on any operation.
**Fix:** Add 401/403/404/409/500 to all operations following BRRTRouter conventions.

### Gap 4: Missing x-brrtrouter-impl Markers (3 services, HIGH)
**Severity:** HIGH — Code generator cannot identify implementation boundaries.
**Affected:** warehouse, dropshipping, logistics (0/40 operations)
**Details:** No `x-brrtrouter-impl` on any mutation (POST/PUT/PATCH).
**Fix:** Add `x-brrtrouter-impl: true` on all write operations.

### Gap 5: Missing Tags (3 services, MEDIUM)
**Severity:** MEDIUM — No semantic grouping in OpenAPI UI.
**Affected:** warehouse, dropshipping, logistics
**Details:** `tags` array is empty on all three specs.
**Fix:** Define semantic tags for each service (e.g., "warehouses", "locations", "transfers").

### Gap 6: Missing Operation Descriptions (3 services, MEDIUM)
**Severity:** MEDIUM — Only `summary` present, no `description` on any operation.
**Affected:** warehouse, dropshipping, logistics
**Details:** `description` field absent from all operations.
**Fix:** Add human-readable descriptions to all 40 operations.

### Gap 7: Missing Request Body Descriptions (3 services, MEDIUM)
**Severity:** MEDIUM — No descriptions on POST/PUT/PATCH requestBodies.
**Affected:** warehouse, dropshipping, logistics
**Details:** 16 requestBodies total, 0 have `description` fields.
**Fix:** Add descriptions to all request bodies.

### Gap 8: Missing Examples (6 services, LOW)
**Severity:** LOW — Swagger UI shows "No example" on responses and requests.
**Affected:** All 6 inventory services
**Details:** Zero response examples, zero request examples across the suite.
**Fix:** Add realistic examples to response and request bodies.

---

## Implementation Plan

### Phase 1: Build warehouse service (15 ops)
1. Define schemas: Warehouse, Location, Transfer + Create/Update request types + BaseEntity
2. Add bearerAuth security
3. Add full response codes (200/201/204/400/401/403/404/409/500)
4. Add x-brrtrouter-impl on mutations
5. Add tags (warehouses, locations, transfers)
6. Add descriptions to all operations + request bodies
7. Add examples

### Phase 2: Build dropshipping service (10 ops)
1. Define schemas: DropshipOrder, VendorOrder + Create/Update request types + BaseEntity
2. Add bearerAuth security
3. Add full response codes
4. Add x-brrtrouter-impl on mutations
5. Add tags (dropship-orders, vendor-orders)
6. Add descriptions to all operations + request bodies
7. Add examples

### Phase 3: Build logistics service (15 ops)
1. Define schemas: Carrier, Shipment, ShippingRate + Create/Update request types + BaseEntity
2. Add bearerAuth security
3. Add full response codes
4. Add x-brrtrouter-impl on mutations
5. Add tags (carriers, shipments, shipping-rates)
6. Add descriptions to all operations + request bodies
7. Add examples

### Phase 4: Examples pass (6 services)
1. Add response examples to stock, stock-movements, stock-valuations (previously uncompleted)
2. Add request examples to all 6 services

---

## Target State

After completion:
- All 6 services at operational parity
- 63 operations total, all with full security, response codes, tags, descriptions, impl markers
- 100% BRRTRouter compliance across the suite
- Full schema coverage for code generation
