# CRM-AUD-00: crm/audience — LifeModels and migrations

**Service:** crm/audience · **Database:** `crm`, schema `audience` · **Build step:** 1 · **Depends on:** CRM-DB-01 · **Points:** 5

## Goal

Every table of `openapi/crm/docs/data-model/audience.yaml` exists as a LifeModel in
`microservices/crm/audience/impl/src/models/`, the migrator generates its SQL, the authored row-level-security file is in
place, and the schema applies cleanly to a local Postgres.

## Read first (only these)

- `openapi/crm/docs/data-model/audience.yaml` — the design (columns, keys, checks, indexes, notes).
- `openapi/crm/docs/14-data-model.md` §1 and §2 — rules and the type / attribute table.
- An existing LifeModel for the attribute style: `microservices/accounting/edi/impl/src/models/edi_document.rs`.
- `microservices/crm/migrations/templates/rls_and_infrastructure.sql`.

## Tables (15)

| Table | Purpose | Columns |
|---|---|---|
| `contacts` | People the workspace can reach. Email unique per workspace (case-insensitive). | 23 |
| `organisations` | Prospect companies. A customer exists only in RERP crm/accounts (ADR 003). | 14 |
| `tags` | Tag definitions; names unique per workspace. | 7 |
| `contact_tags` | Which contacts carry which tags. | 5 |
| `lists` | Mailing lists (email or SMS) with their consent settings. | 13 |
| `subscriptions` | One contact on one list, with the consent that put them there. | 15 |
| `consent_events` | Append-only history of each subscription status change (the consent evidence). | 10 |
| `suppressions` | Addresses no list may message (hard bounce, complaint, manual, erased). | 10 |
| `segments` | Saved segments: a two-level condition tree over the contact facts. | 9 |
| `contact_notes` | Notes on a contact. | 8 |
| `timeline_items` | The contact timeline, projected from events of every service. | 10 |
| `contact_facts` | Per-contact projection of facts from other services, so a segment is one query. | 15 |
| `bulk_jobs` | Changes applied to many contacts; undoable for 7 days. | 17 |
| `bulk_job_changes` | Before and after values per contact, for undo. | 5 |
| `import_jobs` | Contact imports and exports run by audience. | 14 |

## Steps

1. For each table, create `impl/src/models/<table>.rs` with `#[derive(LifeModel)]`, `#[table_name = "<table>"]`,
   `#[schema_name = "audience"]` (the audit table in crm/workspace uses `audit`), one field per column
   in design order, the attributes from 14-data-model.md §2 (`column_type`, `default_expr`, `foreign_key`, `check`,
   `indexed`, `unique`, `index`), and a doc comment with the table's description. Add `pub mod <table>;` to
   `impl/src/models/mod.rs`.
2. Composite primary keys: mark every key column `#[primary_key]`.
3. Run `python3 openapi/crm/tools/data_model.py --models` until it prints nothing for crm/audience.
4. `cd microservices && cargo run -p rerp_migrator --features crm -- generate --suite crm` — commit the generated
   `microservices/crm/migrations/audience/` files and the updated `apply_order.txt`.
5. Copy the template to `microservices/crm/migrations/audience/0002_rls_and_infrastructure.sql`, replace `{{schema}}`
   with `audience`, add the exceptions the YAML marks (`rls:`, `workspace_scoped: false`), and append the file to
   `apply_order.txt` after the generated ones.
6. Apply both to a local Postgres (`scripts/crm-db.sh apply audience` from CRM-DB-01) and run the isolation test below.

## Acceptance criteria

- AC-1 `data_model.py --models` reports nothing for crm/audience.
- AC-2 The generated SQL and the authored file apply to an empty `crm` database without error, twice (idempotent).
- AC-3 As a non-superuser service role with `crm.workspace_id` set to workspace A, a row inserted for A is visible;
  with B set it is not; with nothing set no row is visible; inserting a row for A while B is set fails.
- AC-4 Every foreign key, unique constraint, check and index in the YAML exists in the database
  (`\d audience.<table>` matches the design).
- AC-5 `cargo clippy -p rerp_crm_audience --all-targets -- -D warnings` passes.

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
