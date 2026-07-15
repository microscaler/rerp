# Service Implementation And Database Layout

- **Status**: `partially-verified`
- **Source docs**: [`AGENTS.md`](../../../AGENTS.md), [`CONTRIBUTING.md`](../../../CONTRIBUTING.md), [`microservices/accounting/README.md`](../../../microservices/accounting/README.md), Hauliage `AGENTS.md`
- **Code anchors**: `microservices/Cargo.toml`, `microservices/migrator/`, `microservices/accounting/`, `microservices/documents/`, `tooling/src/rerp_tooling/runtime.py`
- **Last updated**: 2026-07-15

## What It Is

RERP adapts Hauliage's complete microservice anatomy beneath a mandatory suite
boundary. Every HTTP service has a generated `gen/` contract crate and its own
deployable, user-owned `impl/` crate. Suite-owned assets such as shared
foundation entities, migrations, SQL contracts, scripts, and cross-service
tests remain inside the suite directory so installing one suite does not
install another suite's schema.

## Directory Model

For a service such as accounting general-ledger:

```text
openapi/accounting/general-ledger/openapi.yaml
microservices/accounting/general-ledger/gen/
microservices/accounting/general-ledger/impl/
microservices/accounting/migrations/general-ledger/
```

Responsibilities:

- `openapi/.../openapi.yaml`: source API contract.
- `gen/`: generated handlers/types/registry/docs. Disposable.
- `impl/`: deployable service library/binary, real controllers, application
  services, service-owned models, validators, configuration, seeds, and tests.
- `<suite>/entities/`: genuinely suite-wide foundation models only.
- `<suite>/migrations/`: the complete migration product for that suite,
  grouped by owning service/foundation provider.

The workspace manifest `microservices/Cargo.toml` may list crates from every
suite to maintain one development lockfile. Workspace membership is not an
installation instruction: build, deployment, and migration tooling must select
suites explicitly.

## Implementation Pattern

Implementation crates should depend on their generated crate and on shared dependencies from the workspace. The intended steady-state package split is:

```text
gen package:  rerp_accounting_general_ledger_gen
impl package: rerp_accounting_general_ledger
```

The implementation binary must register real controllers for operations that
have behavior. Generated stubs are acceptable scaffolding only; they must not
be mistaken for shipped behavior.

When adding or updating a controller:

1. Update `openapi/{suite}/{service}/openapi.yaml` first if the API shape changed.
2. Regenerate `gen/` through `rerp gen suite <suite> --service <service>`.
3. Keep transport adaptation in `impl/src/controllers` and application use cases
   in `impl/src/services`.
4. Ensure `impl/src/main.rs` wires the real handler, unless BRRTRouter generation has taken ownership of that registration.
5. Add focused tests for the real behavior, not just route existence.

The rest of the implementation anatomy follows Hauliage:

- `impl/src/models/`: service-owned Lifeguard entities/views.
- `impl/src/validators/`: validation beyond OpenAPI/database constraints.
- `impl/seeds/`: service-owned idempotent development/reference data.
- `impl/tests/`: unit/integration/BDD/feature/OpenAPI-parity tests.
- `impl/config/`: runtime configuration packaged for the service.
- `impl/build.rs`: generates the entity registry consumed by the migrator.
- `impl/src/impl_registry.rs`: replaces generated example/stub handlers.

Generated docs remain under `gen/doc`/`gen/static_site`; human-owned service
knowledge belongs in the service README and architecture/PRD material under
`docs/`.

## Database And Entity Ownership

Each effective table/view has exactly one `LifeModel` owner and one migration
source:

1. A service-specific entity belongs to that service's `impl/src/models/`.
2. A shared concept with a natural service owner remains with that service;
   consumers reuse the owner library/API rather than copy the model.
3. Only a concept foundational to the whole suite and lacking a natural service
   owner belongs in `microservices/<suite>/entities/`.
4. The same table must never appear in a service registry and the suite registry
   or in two service registries.

`microservices/<suite>/core/` holds pure domain policy. It is neither an entity
repository nor a suite-wide replacement for the services' `impl` crates.

Additional entity rules:

- `#[index = "idx_name(column)"]` and `#[indexed]` must reference fields that exist on the same struct.
- Child/specialized entities do not inherit columns from their parent/base table.
- To query by base-entity fields from a child table, use a join; do not add indexes on columns that do not exist on the child.
- After index/entity changes, run migration generation and apply migrations in a test database.

RERP's shared infrastructure config points services at shared PostgreSQL:

```text
k8s/rerp/rerp-database-env.yaml
helm/rerp-microservice/values/_database-shared-k8s.yaml
```

These stay aligned with the shared cluster owned outside RERP.

## Lifeguard Migration Workflow

There is one top-level migration tool at `microservices/migrator/`. It adapts
Hauliage's service-registry generation pattern, adding suite qualification.

The migrator receives providers identified by `(suite, service)`, loads service
`impl` registries and any suite foundation registry, and produces one migration
set per selected suite. It must reject duplicate effective table ownership and
fail on provider errors.

Current anchors:

- Service entities: `microservices/<suite>/<service>/impl/src/models/`.
- Suite foundation entities: `microservices/<suite>/entities/src/`.
- Migration tool: `microservices/migrator/`.
- Migration output: `microservices/<suite>/migrations/`.
- Service seeds: `microservices/<suite>/<service>/impl/seeds/`.

Generation/application requires explicit suite selection. No command may default
to generating or installing every suite, and no command may recreate a
repository-root `migrations/` directory. Suite-local application and seed order
files cannot reference another suite.

## Database Gotchas From Hauliage

Hauliage exposed several lessons that apply to RERP:

- DB connection pool size multiplies by service count. Keep `DB_POOL_MAX` conservative in dev.
- Table grants/default privileges matter when bootstrap runs as a superuser but apps connect as a product-specific role.
- `ADD COLUMN IF NOT EXISTS ... DEFAULT ...` can silently fail to retrofit defaults on existing columns. Fresh schema generation and retrofit migrations are different problems.
- Lifeguard query methods live on generated `Entity` types, not necessarily on the model struct itself. Verify against the current Lifeguard derive output before writing controller queries.

RERP adds one lesson that Hauliage does not need: a flat, hard-coded migrator is
valid for Hauliage's single suite but not for RERP. Provider identity, output,
seed discovery, installation, and deployment must all include the suite.

## Open Questions

> **Reconciled:** Accounting's effective inventory now has 47 table models with
> 47 unique `(suite, service)` owners. The suite entity crate contains only the
> 10-model posting foundation, the top-level migrator requires explicit suite
> selection, and migration/seed output is suite-local. The current relationship
> diagrams and unenforced logical references are maintained in the
> [Accounting suite README](../../../microservices/accounting/README.md).

> **Open:** General Ledger's `accounts`/`journal_entries` models remain
> physically distinct from the delivered `accounting_accounts`/
> `accounting_journal_entries` foundation. Decide whether they are legitimate
> pre-posting workflow records or obsolete parallel-ledger scaffolding before
> activating those service migrations.

> **Open:** RERP's final database bootstrap/apply flow should be documented once
> the top-level migrator owns `generate`, `validate`, `plan`, `apply`, and
> `status` for explicitly selected suites.

## Cross-References

- [`hauliage-reference-operating-model.md`](./hauliage-reference-operating-model.md) — What RERP should borrow from Hauliage and what it must not copy.
- [`suite-aware-brrtrouter-wrapper.md`](./suite-aware-brrtrouter-wrapper.md) — Wrapper build/codegen rules.
- [`../../../microservices/accounting/README.md`](../../../microservices/accounting/README.md) — Accounting suite ownership and service anatomy.
- [`../../../microservices/accounting/entities/README.md`](../../../microservices/accounting/entities/README.md) — Current Accounting foundation entity crate.
