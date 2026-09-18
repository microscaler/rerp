# RERP Documentation History

- **Status**: ACTIVE
- **Authority**: Informative
- **Owner**: RERP maintainers
- **Scope**: repository.documentation-history
- **Last reviewed**: 2026-07-15
- **Supersedes**: None
- **Superseded by**: None

This directory preserves superseded concepts and dated delivery evidence. Its
contents are useful for understanding why RERP changed, but they are not current
product, architecture, API, or implementation authority.

Start at the [RERP Documentation Authority Index](../README.md) for current
concepts and follow the
[Documentation Governance policy](../DOCUMENTATION_GOVERNANCE.md) when moving
additional material here.

## Contents

| Directory | Purpose |
|---|---|
| [`conceptual-bootstrap/`](./conceptual-bootstrap/) | Superseded early repository and service-layout plans |
| [`implementation-snapshots/`](./implementation-snapshots/) | Dated completion and status reports that describe earlier repository states |
| [`architecture-snapshots/`](./architecture-snapshots/) | Superseded design and ownership mappings |
| [`audits/`](./audits/) | Completed point-in-time audits |
| [`plans/`](./plans/) | Superseded execution-order plans retained for breadth and rationale |

### Conceptual bootstrap

- [RERP Preparation Plan](./conceptual-bootstrap/RERP_PREPARATION_PLAN.md)

### Implementation snapshots

- [Entity Migration Complete](./implementation-snapshots/ENTITY_MIGRATION_COMPLETE.md)
- [Preparation Implementation Complete](./implementation-snapshots/IMPLEMENTATION_COMPLETE.md)
- [Preparation Implementation Status](./implementation-snapshots/IMPLEMENTATION_STATUS.md)
- [OpenAPI Examples Implementation Status](./implementation-snapshots/OPENAPI_EXAMPLES_IMPLEMENTATION_STATUS.md)
- [BFF Generation Complete](./implementation-snapshots/BFF_GENERATION_COMPLETE.md)
- [OpenAPI Generation Complete](./implementation-snapshots/OPENAPI_GENERATION_COMPLETE.md)

### Architecture snapshots and audits

- [Original System BFF Generation](./architecture-snapshots/SYSTEM_BFF_GENERATION.md)
- [Retired Entity Service Mapping](./architecture-snapshots/SERVICE_MAPPING.md)
- [Microservice Matrix Audit](./audits/MICROSERVICE_MATRIX_AUDIT.md)

### Superseded plans

- [Accounting Suite Build Plan](./plans/ACCOUNTING_BUILD_PLAN.md)

## Archive rule

A document moves here only when:

1. its replacement or current authority is identified;
2. its status is explicitly historical or superseded;
3. current inbound links are repaired;
4. the authority index no longer presents it as current; and
5. relevant LLM wiki pages are reconciled.

Accepted ADRs remain in `docs/adrs/` even after supersession. Postmortems and
other document classes with an already-explicit historical location do not move
here merely because they are old.
