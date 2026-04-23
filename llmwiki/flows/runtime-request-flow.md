# Runtime Request Flow

> How an HTTP request travels from the outside world to service impl code in RERP.

**Status:** partially-verified

## Overview

```
Client → Tilt ingress → brrtrouter service → OpenAPI path match → impl handler → DB (Lifeguard) → Response
```

## Steps

1. **Ingress:** Request enters via Tilt-managed service (Tilt port from `port-registry.json`).
2. **BRRTRouter dispatch:** The `brrtrouter` router matches `openapi/{suite}/{service}/openapi.yaml` paths against registered routes.
3. **Route lookup:** `brrtrouter` uses `RouteMeta` (generated from OpenAPI) to find the handler.
4. **Security middleware:** Auth/validation middleware runs if security schemes are defined in the OpenAPI spec. Draft endpoints have **no security schemes** in consignments suite.
5. **Handler dispatch:** Request body parsed (JSON via `serde`), typed into generated impl signature.
6. **Business logic:** Service `impl/` code processes the request, calls Lifeguard entities for DB ops.
7. **Response:** Typed response serialized to JSON, status code returned.

## Code Anchors

- Router dispatch: `microservices/` workspace → each service's `gen/src/lib.rs`
- Service server setup: each service's `impl/src/server.rs` (or equivalent)
- Lifeguard entities: `entities/src/`
- Tilt dev environment: port 10352 (Tilt on port), `DB_POOL_MAX` controls DB pool size

## Key Details

- **BFF layer:** In dev, the frontend bypasses the BFF entirely via vite proxy (`frontend/vite.config.js`). The BFF is non-functional and a future redesign target.
- **Proxy map:** `/api/v1/organizations/*` → 8009, `/api/v1/consignments/*` → 8003, `/api/v1/fleet/*` → 8002.
- **DB pool:** All 9 services share a single Postgres pool. `DB_POOL_MAX` defaults to 10 in dev.
- **Replica routing:** Disabled (hardcoded empty vec in `parse_replica_config()`).
