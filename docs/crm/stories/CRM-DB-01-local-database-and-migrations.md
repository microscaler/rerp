# CRM-DB-01: Local CRM databases and the migration pipeline

**Paths:** `microservices/crm/scripts/crm-db.sh`, `microservices/crm/migrations/` · **Build step:** 0 · **Points:** 3

## Goal

One command creates the three CRM databases locally with a role per service, applies the CRM migrations in order, and
resets them, so every data-model story can be verified the same way.

## Read first

- `openapi/crm/docs/14-data-model.md` §1–§2.
- `microservices/accounting/migrations/apply_order.txt` and `scripts/setup-db.sh` (how accounting does it).

## Provide

`microservices/crm/scripts/crm-db.sh` with:

- `up` — create databases `crm`, `crm_insights`, `crm_intake` on `$CRM_PG_URL` (default the local Tilt Postgres); one
  login role per service (`crm_audience`, ..., `crm_intake`) owning its schema, no superuser, no BYPASSRLS; schema
  `audit` with INSERT for every service role and SELECT for `crm_workspace` only.
- `apply [service]` — apply `migrations/<service>/` files in `apply_order.txt` order to the service's database.
- `reset` — drop and recreate.
- `psql <service>` — a shell as that service's role.

## Acceptance criteria

- AC-1 `up && apply` on an empty server succeeds; running `apply` again changes nothing.
- AC-2 A service role cannot read another service's schema.
- AC-3 Works on ms02 and on a Mac with the Tilt Postgres.
