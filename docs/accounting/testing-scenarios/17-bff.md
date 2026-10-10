# Accounting BFF — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/bff
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for the accounting API gateway: routing to the
nine upstream accounting services, identity validation, header handling,
timeouts, and error translation. The BFF holds no business logic and no
database.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

The BFF has no accounting standard of its own — it inherits each downstream
service's compliance surface (see the per-service files and the IFRS
reference linked above). Its acceptance criteria are API-gateway criteria:
correct routing, no permission short-circuits, and faithful error
propagation.

## Endpoint inventory (184 operations, generated `openapi_bff.yaml`, proxy-only)

| Area | Path prefix | Upstream |
|---|---|---|
| Accounts payable | `/accounts-payable/...` (20 ops) | accounts-payable |
| Accounts receivable | `/accounts-receivable/...` (26 ops) | accounts-receivable |
| Asset | `/asset/...` (24 ops) | asset |
| Bank sync | `/bank-sync/...` (23 ops) | bank-sync |
| Budget | `/budget/...` (16 ops) | budget |
| EDI | `/edi/...` (19 ops) | edi |
| Financial reports | `/financial-reports/...` (17 ops) | financial-reports |
| General ledger | `/general-ledger/v1/...` (5 ops) | general-ledger (no base-path rewrite) |
| Invoice | `/invoice/v1/...` (5 ops) | invoice (no base-path rewrite) |

## Use-case scenarios

### UC-BFF-01 — Routing table

Given requests to `/accounts-receivable/ar-agings`, `/asset/assets`, and
`/general-ledger/v1/journal-entries`.
Then each is forwarded to its own upstream with the upstream base path
prepended (`/api/v1/accounting/<name>` for the seven default services, no
prefix for general-ledger/invoice); the GL path arrives upstream as
`/v1/journal-entries`.

### UC-BFF-02 — Unknown prefix

Given `/inventory/stock/...` or `/`.
Then 404 `unknown_upstream` — the BFF never forwards to a service outside
its accounting table.

### UC-BFF-03 — Query string preservation

Given `/asset/assets?page=2&limit=50&status=ACTIVE`.
Then the upstream receives the identical query string with all parameters.

### UC-BFF-04 — Header handling

Given a request with `Authorization`, `Content-Type`, and `Cookie`, plus a
`Connection: keep-alive` hop-by-hop header.
Then the upstream receives Authorization/Content-Type/Cookie, but
connection, keep-alive, proxy-*, te, trailer, transfer-encoding, upgrade,
host, and content-length are stripped.

### UC-BFF-05 — Identity validated, permissions re-checked downstream

Given a valid Sesame token.
Then the BFF builds the identity context (tenant, subject, org, roles,
permissions); given an invalid or tampered token the BFF returns 401;
the BFF **does not** enforce service permissions itself — every upstream
re-checks (asserted by the GL/AP/AR/asset permission tests).

### UC-BFF-06 — Claim consistency

Given claims where `tenant_id` disagrees with the namespaced authorization
tenant, or `sub` ≠ `user_id`.
Then 403 — the BFF refuses to build a context from disagreeing claims.

### UC-BFF-07 — Upstream failure mapping

Given an upstream that is down, slow (> 30 s request / 5 s connect), or
returns an error.
Then the BFF maps to 502 `bad_gateway`, 504 `gateway_timeout`, and passes
the upstream's 4xx/5xx body through untouched (no body rewriting).

### UC-BFF-08 — Method and body forwarding

Given POST `/invoice/v1/customer-invoices` with a JSON body.
Then the upstream receives the same method, JSON body, and content type;
an idempotent retry through the BFF returns the upstream's 200 with
`created=false` byte-for-byte.

### UC-BFF-09 — Passthrough completeness

Given each of the 184 operations in `openapi_bff.yaml`.
Then every operation is reachable through the proxy (routing table covers
all first segments); an operation added to the spec but missing from the
routing table fails the coverage test.

### UC-BFF-10 — Tenant isolation at the edge

Given tenant A and tenant B tokens.
Then all tenant-scoped responses (via upstreams) remain isolated; the BFF
adds no tenant leakage of its own (it forwards headers and returns
upstream bodies verbatim).

## Test coverage matrix

| Scenario | Existing test | Status |
|---|---|---|
| UC-BFF-01 | `proxy.rs::routes_known_prefixes_to_the_right_upstream`, `strips_the_routing_prefix_and_prepends_the_upstream_base_path` | implemented |
| UC-BFF-02 | `proxy.rs::rejects_unknown_prefixes` | implemented |
| UC-BFF-03 | `proxy.rs::strips_the_routing_prefix_and_prepends_the_upstream_base_path` (query passthrough) | implemented |
| UC-BFF-04 | `proxy.rs::hop_by_hop_headers_are_not_forwarded` | implemented |
| UC-BFF-05/06 | `identity.rs::builds_complete_context`, `rejects_tenant_mismatch`, `rejects_unvalidated_request` | implemented |
| UC-BFF-07 | `proxy.rs::upstream_base_defaults_to_the_in_cluster_service` (default base only) | partial — timeout mapping untested |
| UC-BFF-08 | none (needs a live upstream) | not yet implemented |
| UC-BFF-09 | none (spec-to-routing coverage test) | not yet implemented |
| UC-BFF-10 | none | not yet implemented |

## Known drift / gaps

- The spec is generated with 184 operations; there is no automated test
  that the routing table covers every spec operation (UC-BFF-09).
- Upstream failure mapping (502/504) and method/body forwarding have no
  integration tests — they need a mock upstream harness.
- `openapi/accounting/bff/openapi.yaml` does not exist; the only BFF spec
  is the generated `openapi_bff.yaml` (by design).
