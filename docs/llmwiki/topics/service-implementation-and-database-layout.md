# Service Implementation And Database Layout

- **Status**: `partially-verified`
- **Source docs**: [`AGENTS.md`](../../../AGENTS.md), [`docs/ENTITY_MIGRATION_COMPLETE.md`](../../ENTITY_MIGRATION_COMPLETE.md), [`docs/entities/SERVICE_MAPPING.md`](../../entities/SERVICE_MAPPING.md), [`docs/TOOLS_ALIGNMENT_FINDINGS.md`](../../TOOLS_ALIGNMENT_FINDINGS.md)
- **Code anchors**: `microservices/Cargo.toml`, `entities/Cargo.toml`, `entities/src/`, `k8s/rerp/rerp-database-env.yaml`, `helm/rerp-microservice/values/_database-shared-kind.yaml`
- **Last updated**: 2026-04-25

## What It Is

RERP services use the same BRRTRouter two-crate model as Hauliage: generated API contract in `gen/`, business logic in `impl/`, database entities through Lifeguard. The RERP-specific twist is that services are nested under a suite and share a top-level `entities` crate.

## Directory Model

For a service such as accounting general-ledger:

```text
openapi/accounting/general-ledger/openapi.yaml
microservices/accounting/general-ledger/gen/
microservices/accounting/general-ledger/impl/
entities/src/accounting/general_ledger/
```

Responsibilities:

- `openapi/.../openapi.yaml`: source API contract.
- `gen/`: generated handlers/types/registry/docs. Disposable.
- `impl/`: deployable binary and real controllers. Preserve human code.
- `entities/`: Lifeguard data model and migration source.

The workspace manifest `microservices/Cargo.toml` lists both `gen` and `impl` crates as members and exposes shared workspace dependencies, including `brrtrouter`, `lifeguard`, and `rerp_entities`.

## Implementation Pattern

Implementation crates should depend on their generated crate and on shared dependencies from the workspace. The intended steady-state package split is:

```text
gen package:  rerp_accounting_general_ledger_gen
impl package: rerp_accounting_general_ledger
```

The implementation binary should register real controllers for operations that have behavior. Generated stubs are acceptable scaffolding only; they should not be mistaken for shipped behavior.

When adding or updating a controller:

1. Update `openapi/{suite}/{service}/openapi.yaml` first if the API shape changed.
2. Regenerate `gen/` through `rerp gen suite <suite> --service <service>`.
3. Put business logic under `impl/src/controllers` or a service module called by controllers.
4. Ensure `impl/src/main.rs` wires the real handler, unless BRRTRouter generation has taken ownership of that registration.
5. Add focused tests for the real behavior, not just route existence.

## Database And Entity Pattern

RERP uses Lifeguard entities in the top-level `entities` crate. Services import `rerp_entities` rather than each service owning a full local model tree.

Key rules from `AGENTS.md`:

- `#[index = "idx_name(column)"]` and `#[indexed]` must reference fields that exist on the same struct.
- Child/specialized entities do not inherit columns from their parent/base table.
- To query by base-entity fields from a child table, use a join; do not add indexes on columns that do not exist on the child.
- After index/entity changes, run migration generation and apply migrations in a test database.

RERP's shared infrastructure config points services at the shared-kind Postgres:

```text
k8s/rerp/rerp-database-env.yaml
helm/rerp-microservice/values/_database-shared-kind.yaml
```

These should stay aligned with the shared Kind cluster owned outside RERP.

## Lifeguard Migration Workflow

The migration source of truth is the entity structs, not hand-written migration edits.

Current anchors:

- Entity definitions: `entities/src/`.
- SQL generator: `entities/src/bin/generate_sql.rs`.
- Migration generator: `entities/src/bin/generate_migrations.rs`.
- Just recipe: `just generate-sql`.

Run from the correct context, and prefer the repo's CLI/just recipes over ad-hoc scripts. The project policy is tooling-only automation: if a new automation need appears, add a `rerp` subcommand or extend existing tooling rather than creating one-off scripts.

## Database Gotchas From Hauliage

Hauliage exposed several lessons that apply to RERP:

- DB connection pool size multiplies by service count. Keep `DB_POOL_MAX` conservative in dev.
- Table grants/default privileges matter when bootstrap runs as a superuser but apps connect as a product-specific role.
- `ADD COLUMN IF NOT EXISTS ... DEFAULT ...` can silently fail to retrofit defaults on existing columns. Fresh schema generation and retrofit migrations are different problems.
- Lifeguard query methods live on generated `Entity` types, not necessarily on the model struct itself. Verify against the current Lifeguard derive output before writing controller queries.

## Open Questions

> **Open:** RERP's final database bootstrap flow should be documented once the `rerp` CLI owns migrations end-to-end. Existing docs mention historical script-style workflows in places; prefer `tooling/` and `just` commands until reconciled.

> **Open:** The exact steady-state placement of service-specific database access helpers in RERP impl crates should be confirmed when business logic controllers are implemented at scale.

## Cross-References

- [`hauliage-reference-operating-model.md`](./hauliage-reference-operating-model.md) — What RERP should borrow from Hauliage and what it must not copy.
- [`suite-aware-brrtrouter-wrapper.md`](./suite-aware-brrtrouter-wrapper.md) — Wrapper build/codegen rules.
- [`../../entities/SERVICE_MAPPING.md`](../../entities/SERVICE_MAPPING.md) — Entity-to-service mapping.
- [`../../../entities/README.md`](../../../entities/README.md) — Entity crate overview.
