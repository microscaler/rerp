# CRM-SAL-00: crm/sales — LifeModels and migrations

**Service:** crm/sales · **Database:** `crm`, schema `sales` · **Build step:** 4 · **Depends on:** CRM-DB-01 · **Points:** 7

## Goal

Every table of `openapi/crm/docs/data-model/sales.yaml` exists as a LifeModel in
`microservices/crm/sales/impl/src/models/`, the migrator generates its SQL, the authored row-level-security file is in
place, and the schema applies cleanly to a local Postgres.

## Read first (only these)

- `openapi/crm/docs/data-model/sales.yaml` — the design (columns, keys, checks, indexes, notes).
- `openapi/crm/docs/14-data-model.md` §1 and §2 — rules and the type / attribute table.
- An existing LifeModel for the attribute style: `microservices/accounting/edi/impl/src/models/edi_document.rs`.
- `microservices/crm/migrations/templates/rls_and_infrastructure.sql`.

## Tables (21)

| Table | Purpose | Columns |
|---|---|---|
| `pipelines` | Sales pipelines. | 13 |
| `stages` | Stages of a pipeline. | 17 |
| `leads` | Website leads and their triage (EPIC-32 board). | 29 |
| `deals` | Deals. | 26 |
| `deal_roles` | Roles a contact can have on a deal. | 5 |
| `deal_contacts` | Contacts on a deal, with their role. | 6 |
| `deal_notes` | Notes on a deal. | 8 |
| `deal_files` | Files on a deal (bytes in object storage). | 10 |
| `deal_emails` | Emails logged on a deal. | 22 |
| `deal_activity` | Deal activity feed (append-only). | 8 |
| `deal_settings` | Deal settings, one row per workspace. | 6 |
| `task_types` | Task types. | 8 |
| `task_outcomes` | Task outcomes. | 5 |
| `task_type_outcomes` | Which outcomes a task type offers. | 3 |
| `tasks` | Tasks on deals, contacts or organisations. | 21 |
| `task_settings` | Task settings, one row per workspace. | 6 |
| `calendar_feeds` | iCalendar feeds of tasks (the token is stored hashed). | 12 |
| `saved_response_categories` | Saved response categories. | 4 |
| `saved_responses` | Saved email responses. | 10 |
| `teams` | Sales teams. | 11 |
| `team_members` | Members of a team and their assignment settings. | 13 |

## Steps

1. For each table, create `impl/src/models/<table>.rs` with `#[derive(LifeModel)]`, `#[table_name = "<table>"]`,
   `#[schema_name = "sales"]` (the audit table in crm/workspace uses `audit`), one field per column
   in design order, the attributes from 14-data-model.md §2 (`column_type`, `default_expr`, `foreign_key`, `check`,
   `indexed`, `unique`, `index`), and a doc comment with the table's description. Add `pub mod <table>;` to
   `impl/src/models/mod.rs`.
2. Composite primary keys: mark every key column `#[primary_key]`.
3. Run `python3 openapi/crm/tools/data_model.py --models` until it prints nothing for crm/sales.
4. `cd microservices && cargo run -p rerp_migrator --features crm -- generate --suite crm` — commit the generated
   `microservices/crm/migrations/sales/` files and the updated `apply_order.txt`.
5. Copy the template to `microservices/crm/migrations/sales/0002_rls_and_infrastructure.sql`, replace `{{schema}}`
   with `sales`, add the exceptions the YAML marks (`rls:`, `workspace_scoped: false`), and append the file to
   `apply_order.txt` after the generated ones.
6. Apply both to a local Postgres (`scripts/crm-db.sh apply sales` from CRM-DB-01) and run the isolation test below.

## Acceptance criteria

- AC-1 `data_model.py --models` reports nothing for crm/sales.
- AC-2 The generated SQL and the authored file apply to an empty `crm` database without error, twice (idempotent).
- AC-3 As a non-superuser service role with `crm.workspace_id` set to workspace A, a row inserted for A is visible;
  with B set it is not; with nothing set no row is visible; inserting a row for A while B is set fails.
- AC-4 Every foreign key, unique constraint, check and index in the YAML exists in the database
  (`\d sales.<table>` matches the design).
- AC-5 `cargo clippy -p rerp_crm_sales --all-targets -- -D warnings` passes.

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
