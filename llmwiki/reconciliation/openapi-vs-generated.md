# OpenAPI Specs vs Generated Code

> Track discrepancies between OpenAPI source specs and the Rust code generated from them.

**Status:** unverified (needs reconciliation pass)

## Method

For each service:
1. Read `openapi/{suite}/{name}/openapi.yaml`
2. Read generated types in `microservices/{suite}/{name}/gen/src/`
3. Verify: every OpenAPI path has a generated route, every request/response type is present

## Known Issues

- Draft consignments endpoints exist in OpenAPI but return 403 (no security scheme defined, but brrtrouter default auth may be blocking)
- Some services may have stub implementations that don't fully match spec

## OpenAPI Spec Audit

| Suite | Service | Spec exists | Generated crate exists | Impl exists |
|-------|---------|-------------|----------------------|-------------|
| accounting | general-ledger | `openapi/accounting/general-ledger/openapi.yaml` | `gen/` | `impl/` |
| accounting | invoice | `openapi/accounting/invoice/openapi.yaml` | `gen/` | `impl/` |
| ai | llm-wiki | `openapi/ai/llm-wiki/openapi.yaml` | `gen/` | `impl/` |
| auth | identity | `openapi/auth/identity/openapi.yaml` | `gen/` | `impl/` |
| crm | contacts | `openapi/crm/contacts/openapi.yaml` | `gen/` | `impl/` |
| product | catalog | `openapi/product/catalog/openapi.yaml` | `gen/` | `impl/` |
| *(and 64 more)* | | | | |

Total: 71 OpenAPI specs → 71 generated crates + 71 impl crates = 142 crates in workspace.
