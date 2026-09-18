# Consignment Draft Model

> The consignment draft entity and its API endpoints.

**Status:** unverified

## Draft Endpoints

- `POST /api/v1/consignments/jobs/drafts` — Create consignment draft
- Draft ID format: `c0000001-0001-4000-8000-000000000001`

## Security Quirk

- OpenAPI spec has **no security schemes** for draft endpoints
- But brrtrouter may apply default auth, causing 403 responses
- This is a known issue between spec intent and runtime behavior

## Implementation Status

- Endpoint returns 403 in dev environment
- Service runs on port 8003 (or should)
- BFF proxy: `/api/v1/consignments/*` → `localhost:8003`

## Code Anchors

- OpenAPI spec: `openapi/consignment/` (or relevant suite path)
- Generated crate: `microservices/consignment/gen/`
- Impl crate: `microservices/consignment/impl/`
- Port registry: `port-registry.json`
