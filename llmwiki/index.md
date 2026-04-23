# RERP LLM Wiki Index

## Core

- [Schema](./SCHEMA.md)
- [Log](./log.md)
- [Docs Catalog](./docs-catalog.md)

## Architectural Flows

- [Request lifecycle: OpenAPI spec → BRRTRouter → service impl](./flows/runtime-request-flow.md)
- [Code generation: brrtrouter-gen pipeline](./flows/code-generation-flow.md)
- [Suite BFF aggregation](./flows/suite-bff-flow.md)
- [Lifecycle: Tilt → Dev environment](./flows/lifeguard-entity-migration.md)

## Reconciliation

- [OpenAPI specs vs generated code](./reconciliation/openapi-vs-generated.md)
- [Microservice stubs vs implementations](./reconciliation/stubs-vs-implementations.md)

## Reference

- [Suite organization and BFF mapping](./reference/suites-and-bff.md)
- [Lifeguard entity conventions](./reference/lifeguard-entities.md)
- [Port registry and Tilt configuration](./reference/tilt-and-ports.md)
- [OpenAPI extension conventions](./reference/openapi-extensions.md)
- [Codebase entry points by suite](./reference/codebase-entry-points.md)

## Entities

- [Base invoice entity and child relationships](./entities/invoice-model.md)
- [Consignment draft model](./entities/consignment-draft.md)
- [Organization and company model](./entities/organization-model.md)

## Topics

- [71-service microservice matrix](./topics/microservice-matrix.md)
- [Phase 1: Core Foundation services](.//topics/phase1-core-foundation.md)
- [Phase 2: Business Operations services](./topics/phase2-business-ops.md)
- [Suite BFF generation](./topics/suite-bff-generation.md)
- [Lifeguard entity migration workflow](./topics/lifeguard-entity-migration.md)
- [Tooling/rerp CLI commands](./topics/rerp-cli-tooling.md)
- [CI automation (GitHub Actions)](./topics/ci-automation.md)
- [Container builds and multi-arch](./topics/multi-arch-builds.md)
- [Frontend architecture and vite proxy](./topics/frontend-architecture.md)
- [Sibling repos and wikis](./topics/sibling-repos-and-wikis.md) — Hauliage, Lifeguard, BRRTRouter
