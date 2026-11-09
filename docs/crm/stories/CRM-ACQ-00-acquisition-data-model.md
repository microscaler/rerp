# CRM-ACQ-00: crm/acquisition — LifeModels and migrations

**Service:** crm/acquisition · **Database:** `crm`, schema `acquisition` · **Build step:** 5 · **Depends on:** CRM-DB-01 · **Points:** 8

## Goal

Every table of `openapi/crm/docs/data-model/acquisition.yaml` exists as a LifeModel in
`microservices/crm/acquisition/impl/src/models/`, the migrator generates its SQL, the authored row-level-security file is in
place, and the schema applies cleanly to a local Postgres.

## Read first (only these)

- `openapi/crm/docs/data-model/acquisition.yaml` — the design (columns, keys, checks, indexes, notes).
- `openapi/crm/docs/14-data-model.md` §1 and §2 — rules and the type / attribute table.
- An existing LifeModel for the attribute style: `microservices/accounting/edi/impl/src/models/edi_document.rs`.
- `microservices/crm/migrations/templates/rls_and_infrastructure.sql`.

## Tables (26)

| Table | Purpose | Columns |
|---|---|---|
| `forms` | Web forms: fields, actions, style, embed. | 27 |
| `form_submissions` | Admitted submissions per form (from intake.admitted). | 12 |
| `tracked_sites` | Sites carrying the tracking script. | 11 |
| `event_types` | Custom event names seen or declared. | 6 |
| `visitors` | Visitors (anonymous until identified). | 13 |
| `page_views` | Page views and custom events (append-only, partitioned by month). | 12 |
| `conversion_definitions` | Named, valued conversions. | 10 |
| `goals` | Business goals against a conversion. | 13 |
| `ad_accounts` | Connected ad accounts (tokens in Secret Manager). | 15 |
| `ad_audiences` | Contact audiences synced to ad platforms. | 20 |
| `ad_audience_members` | Who is in an audience and what was uploaded (hashed identifiers only). | 6 |
| `ad_audience_sync_runs` | Sync runs of an audience. | 13 |
| `ad_buys` | Ad buys (campaigns placed through adapters). | 14 |
| `ad_creatives` | Creatives and their platform review state. | 18 |
| `ad_clicks` | Landings with ad click ids. | 16 |
| `ad_link_tag_rules` | Link decoration rules (UTM and click ids). | 13 |
| `ad_conversion_rules` | Which conversions are relayed to which ad accounts. | 18 |
| `ad_conversions` | The conversion relay log. | 21 |
| `touchpoints` | Attribution touchpoints per contact. | 12 |
| `attribution_models` | Attribution models (built-in and custom). | 11 |
| `ad_spend` | Daily spend imported from ad platforms. | 10 |
| `ad_settings` | Ad tracking settings, one row per workspace. | 3 |
| `utm_campaigns` | UTM campaigns. | 6 |
| `utm_mediums` | UTM mediums. | 5 |
| `utm_sources` | UTM sources. | 6 |
| `event_registrations` | Event registrations (parked feature; admitted through intake). | 13 |

## Steps

1. For each table, create `impl/src/models/<table>.rs` with `#[derive(LifeModel)]`, `#[table_name = "<table>"]`,
   `#[schema_name = "acquisition"]` (the audit table in crm/workspace uses `audit`), one field per column
   in design order, the attributes from 14-data-model.md §2 (`column_type`, `default_expr`, `foreign_key`, `check`,
   `indexed`, `unique`, `index`), and a doc comment with the table's description. Add `pub mod <table>;` to
   `impl/src/models/mod.rs`.
2. Composite primary keys: mark every key column `#[primary_key]`.
3. Run `python3 openapi/crm/tools/data_model.py --models` until it prints nothing for crm/acquisition.
4. `cd microservices && cargo run -p rerp_migrator --features crm -- generate --suite crm` — commit the generated
   `microservices/crm/migrations/acquisition/` files and the updated `apply_order.txt`.
5. Copy the template to `microservices/crm/migrations/acquisition/0002_rls_and_infrastructure.sql`, replace `{{schema}}`
   with `acquisition`, add the exceptions the YAML marks (`rls:`, `workspace_scoped: false`), and append the file to
   `apply_order.txt` after the generated ones.
6. Apply both to a local Postgres (`scripts/crm-db.sh apply acquisition` from CRM-DB-01) and run the isolation test below.

## Acceptance criteria

- AC-1 `data_model.py --models` reports nothing for crm/acquisition.
- AC-2 The generated SQL and the authored file apply to an empty `crm` database without error, twice (idempotent).
- AC-3 As a non-superuser service role with `crm.workspace_id` set to workspace A, a row inserted for A is visible;
  with B set it is not; with nothing set no row is visible; inserting a row for A while B is set fails.
- AC-4 Every foreign key, unique constraint, check and index in the YAML exists in the database
  (`\d acquisition.<table>` matches the design).
- AC-5 `cargo clippy -p rerp_crm_acquisition --all-targets -- -D warnings` passes.

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
