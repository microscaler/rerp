# Hauliage Reference Operating Model For RERP

- **Status**: `partially-verified`
- **Source docs**: [`docs/TOOLS_ALIGNMENT_FINDINGS.md`](../../TOOLS_ALIGNMENT_FINDINGS.md), [`tooling/README.md`](../../../tooling/README.md), Hauliage [`docs/llmwiki/topics/scaffolding-lifecycle.md`](../../../../hauliage/docs/llmwiki/topics/scaffolding-lifecycle.md), Hauliage [`docs/llmwiki/topics/database-architecture.md`](../../../../hauliage/docs/llmwiki/topics/database-architecture.md)
- **Code anchors**: `tooling/src/rerp_tooling/cli/main.py`, `tooling/tests/test_rerp_cli_translations.py`, `Tiltfile`, `microservices/Cargo.toml`, `microservices/<suite>/<service>/impl/`, `microservices/<suite>/entities/`, `microservices/<suite>/migrations/`, `microservices/accounting/scripts/setup-db.sh`
- **Last updated**: 2026-07-16

## What It Is

Hauliage is the nearest working example of the BRRTRouter service loop: OpenAPI specs generate a `gen` crate, hand-written service logic lives in an `impl` crate, Tilt builds the implementation binary, and Kubernetes deploys one container per service. RERP should copy that operating model, but **not** Hauliage's flat directory layout.

RERP is a multi-suite ERP, so the durable translation is:

```text
Hauliage flat:
  openapi/{service}/openapi.yaml
  microservices/{service}/gen
  microservices/{service}/impl

RERP nested:
  openapi/{suite}/{service}/openapi.yaml
  microservices/{suite}/{service}/gen
  microservices/{suite}/{service}/impl
```

## Service Directory Responsibilities

### `gen/`: generated contract crate

The `gen` crate is generated from OpenAPI and should be treated as disposable. It owns request/response types, route metadata, registry scaffolding, OpenAPI docs, and static generated assets.

In Hauliage, generated crates use names like:

```text
hauliage_company_gen
```

In RERP, the suite-aware equivalent is:

```text
rerp_accounting_general_ledger_gen
```

Future agents should not hand-edit `gen/src` to fix business logic. Change the OpenAPI spec and regenerate, or put behavior in `impl/`.

### `impl/`: service behavior crate

The `impl` crate is the deployable service library/binary and the home of real
behavior. It owns controllers, application services, service-specific
Lifeguard models, validators, local config, seeds, and tests. Its `build.rs`
exports the entity registry used by the migration tool, and it depends on the
sibling `gen` contract crate.

The desired RERP naming convention is:

```text
rerp_{suite}_{service}
```

The wrapper currently reads `microservices/{suite}/{service}/impl/Cargo.toml` so it can build migration-state packages like `*_impl` without confusing them with generated crates. That manifest read is compatibility logic, not the desired long-term naming model.

## Register And Overwrite

Hauliage's working pattern is "register generated stubs first, then overwrite selected routes with real implementation handlers." The danger is silent fallback: a controller file may exist, but if `impl/src/main.rs` does not register it, requests can still hit generated mock/example handlers.

For RERP, the lesson is:

- OpenAPI `operationId` defines the handler contract.
- `gen/` provides the generated route/handler shape.
- `impl/` must explicitly wire real controllers until BRRTRouter owns more of this registration.
- Tests should catch mock fallback when product behavior expects real data.

Do not interpret a successful HTTP response as proof that implementation logic ran. Generated mock responses can look successful.

## Build And Deploy Loop

Hauliage's Tilt loop has five useful stages:

1. Lint OpenAPI with `brrtrouter-gen`.
2. Generate `gen/` from OpenAPI.
3. Build the `impl` package.
4. Copy the binary into `build_artifacts/{arch}/{binary}` and build an image.
5. Deploy with Helm and `k8s_resource`.

RERP follows the same sequence, but all paths must include `{suite}`. The wrapper must build the impl package, not the generated package. This is why `rerp build microservice <name>` resolves the suite and reads `impl/Cargo.toml`.

## Database Model From Hauliage

Hauliage uses a shared PostgreSQL server in the `data` namespace and an application database/schema for the product. RERP follows the same shared-cluster idea but with RERP names:

- Shared Postgres service: `postgres.data.svc.cluster.local:5432`.
- RERP Accounting database/runtime config: the suite-owned SOPS profile at
  `deployment-configuration/profiles/dev/rerp/accounting/` in RERP.
- Helm DB override: `helm/rerp-microservice/values/_database-shared-k8s.yaml`.
- Service Lifeguard entities: `microservices/<suite>/<service>/impl/src/models/`.
- Suite foundation entities: `microservices/<suite>/entities/src/`.
- Suite migration products: `microservices/<suite>/migrations/`.

Hauliage's important operational lessons carry over:

- Keep application data isolated by database/schema naming.
- Treat grants and default privileges as part of DB bootstrap, not an afterthought.
- PostgreSQL HA has two authentication planes: bootstrap the application role
  on the elected primary and configure the same custom user in Pgpool's
  SOPS-managed `pool_passwd` source.
- Tilt waits for Flux-owned profile objects and makes workloads depend on the
  successful role/database/schema/migration initializer; it does not apply a
  competing plaintext Secret.
- The shared-cluster product-component inventory generates one RERP Git source
  and the `rerp-accounting` Flux Kustomization for the suite-owned profile. This
  is the migration seam toward Tilt building images while Flux owns Helm
  releases and rollout.
- `DB_POOL_MAX` controls service pool pressure; don't accidentally multiply too-large pools across every microservice.
- Replica routing should be considered disabled unless code proves otherwise.
- Schema changes should originate from Lifeguard entities and migration tooling, not hand edits to generated SQL.

## RERP-Specific Differences

RERP retains Hauliage's service-owned implementation anatomy but nests it under
a suite. Service-specific entities belong to the owning service's
`impl/src/models/`. Only a genuinely suite-wide foundation concept without a
natural service owner belongs in `microservices/<suite>/entities/`. The same
effective table/view must never be declared in both places or by multiple
services.

RERP also has one top-level migrator. Unlike Hauliage's valid single-suite flat
migrator, RERP providers, generation, seed discovery, ordering, application,
and output must include the suite identity. Migration products stay under
`microservices/<suite>/migrations/`; a repository-root migration set is invalid.

RERP also has many planned suites, but currently only accounting has a mature BFF config. Do not hardcode the assumption that every future suite uses `bff` as the service name; read `bff_service_name` from `openapi/{suite}/bff-suite-config.yaml`.

## Gotchas

> **Do not copy Hauliage paths literally.**
> Hauliage's `microservices/company/impl` becomes RERP's `microservices/accounting/general-ledger/impl` shape.

> **Do not build gen crates.**
> If `cargo metadata` shows both `rerp_accounting_general_ledger_gen` and `rerp_accounting_general_ledger`, only the latter is the service binary.

> **Do not encode migration-state package names as policy.**
> Existing `*_impl` or `*_service_api` names are clues that a crate predates the stabilized convention.

## Cross-References

- [`suite-aware-brrtrouter-wrapper.md`](./suite-aware-brrtrouter-wrapper.md) — Wrapper contract that enforces the nested layout.
- [`service-implementation-and-database-layout.md`](./service-implementation-and-database-layout.md) — RERP-specific service/database responsibilities.
- [`../docs-catalog.md`](../docs-catalog.md) — Source docs.
