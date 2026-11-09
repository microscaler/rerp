# CRM-MSG-00: crm/messaging — LifeModels and migrations

**Service:** crm/messaging · **Database:** `crm`, schema `messaging` · **Build step:** 3 · **Depends on:** CRM-DB-01 · **Points:** 6

## Goal

Every table of `openapi/crm/docs/data-model/messaging.yaml` exists as a LifeModel in
`microservices/crm/messaging/impl/src/models/`, the migrator generates its SQL, the authored row-level-security file is in
place, and the schema applies cleanly to a local Postgres.

## Read first (only these)

- `openapi/crm/docs/data-model/messaging.yaml` — the design (columns, keys, checks, indexes, notes).
- `openapi/crm/docs/14-data-model.md` §1 and §2 — rules and the type / attribute table.
- An existing LifeModel for the attribute style: `microservices/accounting/edi/impl/src/models/edi_document.rs`.
- `microservices/crm/migrations/templates/rls_and_infrastructure.sql`.

## Tables (20)

| Table | Purpose | Columns |
|---|---|---|
| `templates` | Message templates (email and SMS); every save creates a version. | 18 |
| `template_versions` | Immutable versions of a template; sends record the version used. | 10 |
| `snippets` | Reusable blocks inserted as {{snippet.key}}. | 6 |
| `assets` | Images in the asset store (bytes in object storage). | 12 |
| `senders` | From identities and their verification. | 12 |
| `sms_senders` | SMS sender ids and numbers. | 8 |
| `tracking_domains` | Custom tracking domains and their DNS checks. | 8 |
| `postal_addresses` | Postal addresses for message footers. | 6 |
| `list_exclusions` | Address patterns excluded from lists. | 8 |
| `campaigns` | Campaigns: email or SMS, one-time, A/B, RSS, recurring or reminder. | 38 |
| `campaign_variants` | A/B variants of a campaign. | 6 |
| `sends` | The send ledger: one row per recipient per message. | 20 |
| `message_events` | Engagement and delivery events per send (append-only). | 14 |
| `tracked_links` | Links rewritten for click tracking; the edge token carries the target. | 9 |
| `mailboxes` | Connected 1:1 mailboxes (tokens in Secret Manager, never here). | 21 |
| `mailbox_messages` | Messages synced from connected mailboxes. | 20 |
| `social_accounts` | Connected social accounts. | 12 |
| `social_posts` | Social posts and their approval. | 15 |
| `social_post_targets` | Where a post is published, per account. | 9 |
| `social_post_audit` | Approval and publishing audit for social posts. | 8 |

## Steps

1. For each table, create `impl/src/models/<table>.rs` with `#[derive(LifeModel)]`, `#[table_name = "<table>"]`,
   `#[schema_name = "messaging"]` (the audit table in crm/workspace uses `audit`), one field per column
   in design order, the attributes from 14-data-model.md §2 (`column_type`, `default_expr`, `foreign_key`, `check`,
   `indexed`, `unique`, `index`), and a doc comment with the table's description. Add `pub mod <table>;` to
   `impl/src/models/mod.rs`.
2. Composite primary keys: mark every key column `#[primary_key]`.
3. Run `python3 openapi/crm/tools/data_model.py --models` until it prints nothing for crm/messaging.
4. `cd microservices && cargo run -p rerp_migrator --features crm -- generate --suite crm` — commit the generated
   `microservices/crm/migrations/messaging/` files and the updated `apply_order.txt`.
5. Copy the template to `microservices/crm/migrations/messaging/0002_rls_and_infrastructure.sql`, replace `{{schema}}`
   with `messaging`, add the exceptions the YAML marks (`rls:`, `workspace_scoped: false`), and append the file to
   `apply_order.txt` after the generated ones.
6. Apply both to a local Postgres (`scripts/crm-db.sh apply messaging` from CRM-DB-01) and run the isolation test below.

## Acceptance criteria

- AC-1 `data_model.py --models` reports nothing for crm/messaging.
- AC-2 The generated SQL and the authored file apply to an empty `crm` database without error, twice (idempotent).
- AC-3 As a non-superuser service role with `crm.workspace_id` set to workspace A, a row inserted for A is visible;
  with B set it is not; with nothing set no row is visible; inserting a row for A while B is set fails.
- AC-4 Every foreign key, unique constraint, check and index in the YAML exists in the database
  (`\d messaging.<table>` matches the design).
- AC-5 `cargo clippy -p rerp_crm_messaging --all-targets -- -D warnings` passes.

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
