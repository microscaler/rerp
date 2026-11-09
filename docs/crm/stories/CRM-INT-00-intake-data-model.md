# CRM-INT-00: crm/intake — LifeModels and migrations

**Service:** crm/intake · **Database:** `crm_intake`, schema `intake` · **Build step:** 2 · **Depends on:** CRM-DB-01 · **Points:** 3

## Goal

Every table of `openapi/crm/docs/data-model/intake.yaml` exists as a LifeModel in
`microservices/crm/intake/impl/src/models/`, the migrator generates its SQL, the authored row-level-security file is in
place, and the schema applies cleanly to a local Postgres.

## Read first (only these)

- `openapi/crm/docs/data-model/intake.yaml` — the design (columns, keys, checks, indexes, notes).
- `openapi/crm/docs/14-data-model.md` §1 and §2 — rules and the type / attribute table.
- An existing LifeModel for the attribute style: `microservices/accounting/edi/impl/src/models/edi_document.rs`.
- `microservices/crm/migrations/templates/rls_and_infrastructure.sql`.

## Tables (2)

| Table | Purpose | Columns |
|---|---|---|
| `submissions` | Pending and recently decided submissions (quarantine; ADR 004). | 25 |
| `rate_limits` | Submission counters per address and IP for screening. | 3 |

## Steps

1. For each table, create `impl/src/models/<table>.rs` with `#[derive(LifeModel)]`, `#[table_name = "<table>"]`,
   `#[schema_name = "intake"]` (the audit table in crm/workspace uses `audit`), one field per column
   in design order, the attributes from 14-data-model.md §2 (`column_type`, `default_expr`, `foreign_key`, `check`,
   `indexed`, `unique`, `index`), and a doc comment with the table's description. Add `pub mod <table>;` to
   `impl/src/models/mod.rs`.
2. Composite primary keys: mark every key column `#[primary_key]`.
3. Run `python3 openapi/crm/tools/data_model.py --models` until it prints nothing for crm/intake.
4. `cd microservices && cargo run -p rerp_migrator --features crm -- generate --suite crm` — commit the generated
   `microservices/crm/migrations/intake/` files and the updated `apply_order.txt`.
5. Copy the template to `microservices/crm/migrations/intake/0002_rls_and_infrastructure.sql`, replace `{{schema}}`
   with `intake`, add the exceptions the YAML marks (`rls:`, `workspace_scoped: false`), and append the file to
   `apply_order.txt` after the generated ones.
6. Apply both to a local Postgres (`scripts/crm-db.sh apply intake` from CRM-DB-01) and run the isolation test below.

## Acceptance criteria

- AC-1 `data_model.py --models` reports nothing for crm/intake.
- AC-2 The generated SQL and the authored file apply to an empty `crm_intake` database without error, twice (idempotent).
- AC-3 As a non-superuser service role with `crm.workspace_id` set to workspace A, a row inserted for A is visible;
  with B set it is not; with nothing set no row is visible; inserting a row for A while B is set fails.
- AC-4 Every foreign key, unique constraint, check and index in the YAML exists in the database
  (`\d intake.<table>` matches the design).
- AC-5 `cargo clippy -p rerp_crm_intake --all-targets -- -D warnings` passes.

## Test cases

| ID | Case | Expected |
|---|---|---|
| TC-1 | Apply migrations to an empty database | No error; every table in the YAML exists |
| TC-2 | Apply again | No error, no change |
| TC-3 | RLS: insert as A, read as B | 0 rows |
| TC-4 | RLS: read with no workspace set | 0 rows |
| TC-5 | RLS: insert A's row while B is set | `new row violates row-level security policy` |
| TC-6 | Violate each CHECK once | Rejected |

## Do not

- Do not add, rename or drop a column that is not in the YAML. If the design is wrong, stop and say so in your report.
- Do not hand-edit generated migration SQL; change the LifeModel and regenerate.
- Do not put a foreign key to another service's table.
