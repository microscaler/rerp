# Phase 1: Core Foundation Services

> The foundational services that other services depend on.

**Status:** unverified

## Services

1. **auth/identity** — Authentication, authorization, identity management
2. **infrastructure/observability** — Logging, metrics, tracing
3. **product/catalog** — Product catalog and inventory management
4. *(plus 4 more in core foundation)*

## Dependencies

These services form the bedrock:
- auth is required by virtually all other services
- observability is shared infrastructure
- product/catalog is referenced by sales, purchase, inventory

## Code Anchors
- `openapi/auth/identity/openapi.yaml`
- `openapi/infrastructure/observability/openapi.yaml`
- `openapi/product/catalog/openapi.yaml`
- `microservices/auth/`, `microservices/infrastructure/`, `microservices/product/`
