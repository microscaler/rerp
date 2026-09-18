# Phase 1 Accounting Foundation Migrations

Apply these files in lexical order after the vendored Sesame RLS contract:

1. `microservices/accounting/sql/rls/v1/install.sql`
2. `0001_generated_entities.sql`
3. `0002_controls_and_rls.sql`
4. `0003_document_artifacts.sql`

`0001` is generated from the typed models in
`microservices/accounting/entities/src/accounting/foundation/`. `0002`
contains database behavior that Lifeguard entity metadata does not yet express:
tenant-consistent composite
foreign keys, detailed accounting checks, immutable posted/audit records and
forced tenant RLS. `0003` adds immutable, RLS-scoped rendered-document metadata.

The runtime database role requires `USAGE` on the schema, table privileges and
`EXECUTE` on the Sesame RLS functions. Role creation/grants belong to deployment
because self-hosted installations choose their own role name.

## Disposable PostgreSQL acceptance

The following verifies fresh apply plus accounting/RLS failure paths against a
local test PostgreSQL. Never point these commands at a persistent database.

```bash
export PGPASSWORD=postgres
dropdb -h 127.0.0.1 -U postgres --if-exists rerp_phase1_test
createdb -h 127.0.0.1 -U postgres rerp_phase1_test
psql -v ON_ERROR_STOP=1 -h 127.0.0.1 -U postgres -d rerp_phase1_test \
  -f microservices/accounting/sql/rls/v1/install.sql
psql -v ON_ERROR_STOP=1 -h 127.0.0.1 -U postgres -d rerp_phase1_test \
  -f microservices/accounting/migrations/foundation/0001_generated_entities.sql
psql -v ON_ERROR_STOP=1 -h 127.0.0.1 -U postgres -d rerp_phase1_test \
  -f microservices/accounting/migrations/foundation/0002_controls_and_rls.sql
psql -v ON_ERROR_STOP=1 -h 127.0.0.1 -U postgres -d rerp_phase1_test \
  -f microservices/accounting/migrations/foundation/0003_document_artifacts.sql
psql -v ON_ERROR_STOP=1 -h 127.0.0.1 -U postgres -d rerp_phase1_test \
  -f microservices/accounting/tests/sql/accounting_foundation_acceptance.sql
dropdb -h 127.0.0.1 -U postgres rerp_phase1_test
psql -h 127.0.0.1 -U postgres -d postgres \
  -c 'DROP ROLE IF EXISTS rerp_phase1_test'
```

The acceptance SQL uses a `NOLOGIN NOSUPERUSER NOBYPASSRLS` role. It proves
context-free fail-closed reads, tenant isolation, cross-tenant FK rejection,
amount/balance constraints, posted immutability and atomic rollback.
