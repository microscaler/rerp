# CRM-WSP-00: crm/workspace — LifeModels and migrations

**Service:** crm/workspace · **Database:** `crm`, schema `workspace` · **Build step:** 1 · **Depends on:** CRM-DB-01 · **Points:** 5

## Goal

Every table of `openapi/crm/docs/data-model/workspace.yaml` exists as a LifeModel in
`microservices/crm/workspace/impl/src/models/`, the migrator generates its SQL, the authored row-level-security file is in
place, and the schema applies cleanly to a local Postgres.

## Read first (only these)

- `openapi/crm/docs/data-model/workspace.yaml` — the design (columns, keys, checks, indexes, notes).
- `openapi/crm/docs/14-data-model.md` §1 and §2 — rules and the type / attribute table.
- An existing LifeModel for the attribute style: `microservices/accounting/edi/impl/src/models/edi_document.rs`.
- `microservices/crm/migrations/templates/rls_and_infrastructure.sql`.

## Tables (17)

| Table | Purpose | Columns |
|---|---|---|
| `workspaces` | Workspaces (brands). Not row-level-secured by workspace: membership decides. | 10 |
| `memberships` | Who belongs to a workspace, with which role. | 6 |
| `invitations` | Pending invitations by email. | 10 |
| `custom_fields` | Custom field definitions (values live with the owning entity). | 20 |
| `field_groups` | Groups of custom fields. | 6 |
| `api_keys` | API keys (stored hashed). | 14 |
| `webhooks` | Outgoing webhooks (secret in Secret Manager). | 11 |
| `webhook_delivery_logs` | Webhook deliveries. | 9 |
| `notification_rules` | Notification rules. | 8 |
| `compliance_settings` | Consent, retention and cookie settings, one row per workspace. | 11 |
| `data_requests` | Access and erasure requests. | 12 |
| `data_request_steps` | Each service confirming its part of a data request. | 5 |
| `brand_kits` | Brand kit, one per workspace. | 9 |
| `integrations` | Third-party integrations (credentials in Secret Manager). | 12 |
| `jobs` | Import and export jobs. | 23 |
| `job_errors` | Rows a job could not process. | 6 |
| `audit_entries` | The audit log. Every CRM service inserts here in its own transaction (crm-audit); only crm/workspace reads it. | 13 |

## Steps

1. For each table, create `impl/src/models/<table>.rs` with `#[derive(LifeModel)]`, `#[table_name = "<table>"]`,
   `#[schema_name = "workspace"]` (the audit table in crm/workspace uses `audit`), one field per column
   in design order, the attributes from 14-data-model.md §2 (`column_type`, `default_expr`, `foreign_key`, `check`,
   `indexed`, `unique`, `index`), and a doc comment with the table's description. Add `pub mod <table>;` to
   `impl/src/models/mod.rs`.
2. Composite primary keys: mark every key column `#[primary_key]`.
3. Run `python3 openapi/crm/tools/data_model.py --models` until it prints nothing for crm/workspace.
4. `cd microservices && cargo run -p rerp_migrator --features crm -- generate --suite crm` — commit the generated
   `microservices/crm/migrations/workspace/` files and the updated `apply_order.txt`.
5. Copy the template to `microservices/crm/migrations/workspace/0002_rls_and_infrastructure.sql`, replace `{{schema}}`
   with `workspace`, add the exceptions the YAML marks (`rls:`, `workspace_scoped: false`), and append the file to
   `apply_order.txt` after the generated ones.
6. Apply both to a local Postgres (`scripts/crm-db.sh apply workspace` from CRM-DB-01) and run the isolation test below.

## Acceptance criteria

- AC-1 `data_model.py --models` reports nothing for crm/workspace.
- AC-2 The generated SQL and the authored file apply to an empty `crm` database without error, twice (idempotent).
- AC-3 As a non-superuser service role with `crm.workspace_id` set to workspace A, a row inserted for A is visible;
  with B set it is not; with nothing set no row is visible; inserting a row for A while B is set fails.
- AC-4 Every foreign key, unique constraint, check and index in the YAML exists in the database
  (`\d workspace.<table>` matches the design).
- AC-5 `cargo clippy -p rerp_crm_workspace --all-targets -- -D warnings` passes.

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
