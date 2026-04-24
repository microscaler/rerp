# OpenAPI Extension Conventions

> RERP's OpenAPI spec conventions, x-* extensions, and codegen hooks.

**Status:** partially-verified

## OpenAPI Structure

Each service has its own spec under `openapi/{suite}/{name}/openapi.yaml`.

## Key Conventions

- OpenAPI 3.1.0 format
- All endpoints use `/api/v1/` prefix
- Security schemes defined at operation level
- Draft endpoints may omit security schemes intentionally

## Security Scheme Quirks

- Draft consignments endpoints have **no security schemes** in OpenAPI
- But brrtrouter may have default auth that blocks them (causes 403)
- This is a known mismatch between spec intent and runtime behavior

## Extension Points

- OpenAPI specs drive brrtrouter-gen code generation
- Suite BFF configs (`bff-suite-config.yaml`) are YAML, not OpenAPI
- No custom `x-*` extensions documented yet — review spec files

## Code Anchors

- Specs: `openapi/{suite}/{name}/openapi.yaml`
- BFF config: `openapi/{suite}/bff-suite-config.yaml`
- Generated output: `microservices/{suite}/{name}/gen/`
