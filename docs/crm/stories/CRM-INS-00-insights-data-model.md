# CRM-INS-00: crm/insights — LifeModels and migrations

**Service:** crm/insights · **Database:** `crm_insights`, schema `insights` · **Build step:** 5 · **Depends on:** CRM-DB-01 · **Points:** 5

## Goal

Every table of `openapi/crm/docs/data-model/insights.yaml` exists as a LifeModel in
`microservices/crm/insights/impl/src/models/`, the migrator generates its SQL, the authored row-level-security file is in
place, and the schema applies cleanly to a local Postgres.

## Read first (only these)

- `openapi/crm/docs/data-model/insights.yaml` — the design (columns, keys, checks, indexes, notes).
- `openapi/crm/docs/14-data-model.md` §1 and §2 — rules and the type / attribute table.
- An existing LifeModel for the attribute style: `microservices/accounting/edi/impl/src/models/edi_document.rs`.
- `microservices/crm/migrations/templates/rls_and_infrastructure.sql`.

## Tables (16)

| Table | Purpose | Columns |
|---|---|---|
| `dashboards` | Dashboards. | 7 |
| `dashboard_widgets` | Widgets on a dashboard with their saved query. | 11 |
| `saved_reports` | Saved reports. | 7 |
| `report_schedules` | Scheduled report and widget emails. | 9 |
| `scores` | Named, rule-based scores. | 9 |
| `score_rules` | Rules of a score. | 10 |
| `entity_scores` | Current score per contact or deal. | 7 |
| `score_hits` | Rule hits that make up a score (for explain and decay). | 8 |
| `rm_deals` | Read model, rebuilt from events (never written by the API). | 13 |
| `rm_deal_stage_changes` | Read model, rebuilt from events (never written by the API). | 6 |
| `rm_contacts` | Read model, rebuilt from events (never written by the API). | 9 |
| `rm_messages` | Read model, rebuilt from events (never written by the API). | 14 |
| `rm_enrolments` | Read model, rebuilt from events (never written by the API). | 8 |
| `rm_tasks` | Read model, rebuilt from events (never written by the API). | 8 |
| `rm_conversions` | Read model, rebuilt from events (never written by the API). | 8 |
| `rm_list_daily` | Read model, rebuilt from events (never written by the API). | 7 |

## Steps

1. For each table, create `impl/src/models/<table>.rs` with `#[derive(LifeModel)]`, `#[table_name = "<table>"]`,
   `#[schema_name = "insights"]` (the audit table in crm/workspace uses `audit`), one field per column
   in design order, the attributes from 14-data-model.md §2 (`column_type`, `default_expr`, `foreign_key`, `check`,
   `indexed`, `unique`, `index`), and a doc comment with the table's description. Add `pub mod <table>;` to
   `impl/src/models/mod.rs`.
2. Composite primary keys: mark every key column `#[primary_key]`.
3. Run `python3 openapi/crm/tools/data_model.py --models` until it prints nothing for crm/insights.
4. `cd microservices && cargo run -p rerp_migrator --features crm -- generate --suite crm` — commit the generated
   `microservices/crm/migrations/insights/` files and the updated `apply_order.txt`.
5. Copy the template to `microservices/crm/migrations/insights/0002_rls_and_infrastructure.sql`, replace `{{schema}}`
   with `insights`, add the exceptions the YAML marks (`rls:`, `workspace_scoped: false`), and append the file to
   `apply_order.txt` after the generated ones.
6. Apply both to a local Postgres (`scripts/crm-db.sh apply insights` from CRM-DB-01) and run the isolation test below.

## Acceptance criteria

- AC-1 `data_model.py --models` reports nothing for crm/insights.
- AC-2 The generated SQL and the authored file apply to an empty `crm_insights` database without error, twice (idempotent).
- AC-3 As a non-superuser service role with `crm.workspace_id` set to workspace A, a row inserted for A is visible;
  with B set it is not; with nothing set no row is visible; inserting a row for A while B is set fails.
- AC-4 Every foreign key, unique constraint, check and index in the YAML exists in the database
  (`\d insights.<table>` matches the design).
- AC-5 `cargo clippy -p rerp_crm_insights --all-targets -- -D warnings` passes.

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
