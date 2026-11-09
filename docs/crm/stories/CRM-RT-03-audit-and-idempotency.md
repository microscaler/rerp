# CRM-RT-03: rerp_crm_runtime — audit and idempotency

**Crate:** `microservices/crm/runtime` (modules `audit`, `idempotency`) · **Build step:** 0 · **Depends on:** CRM-RT-01, CRM-WSP-00 · **Points:** 5

## Goal

Every write in every CRM service is audited in the same transaction, and side-effecting commands honour
`Idempotency-Key`.

## Read first

- `openapi/crm/docs/data-model/workspace.yaml` → `audit_entries` (schema `audit`).
- `openapi/crm/docs/09-api-conventions.md` §7; the `idempotency_keys` table in the RLS template.
- PriceWhisperer Epic 55 US_55_39 (AC-5).

## API to provide

```rust
// audit.rs
pub enum Action { Create, Update, Delete, StateChange, Erase, Export }
/// One audit.audit_entries row. `before`/`after` are JSON objects; only changed keys are stored.
pub fn record(tx, ctx: &RequestContext, service: &str, entity: &str, entity_id: &str, action: Action,
              before: Option<&serde_json::Value>, after: Option<&serde_json::Value>) -> Result<(), ApiError>;
pub fn changed_fields(before: &Value, after: &Value) -> Value;   // {"field": {"before": .., "after": ..}}

// idempotency.rs
pub enum Claim { Fresh, Replay { status: u16, body: Value } }
/// Hash the method, path and body; claim the key (insert) or return the stored response; 409 idempotency_mismatch
/// when the key exists with another hash.
pub fn claim(tx, ctx: &RequestContext, key: &str, request_hash: &str) -> Result<Claim, ApiError>;
pub fn store(tx, ctx: &RequestContext, key: &str, status: u16, body: &Value) -> Result<(), ApiError>;
```

## Acceptance criteria

- AC-1 Updating two fields writes one audit row with exactly those two fields before and after.
- AC-2 The audit insert uses the service's own role; that role cannot SELECT from `audit.audit_entries`.
- AC-3 Same key, same body within 24 h → stored response, no second effect; same key, other body → 409.
- AC-4 Expired keys (> 24 h) are treated as fresh.

## Verify

```bash
cd microservices && cargo test -p rerp_crm_runtime && cargo clippy -p rerp_crm_runtime --all-targets -- -D warnings
```
