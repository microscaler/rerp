# RERP CRM suite

The CRM and marketing services. Contracts and design live in [`openapi/crm/`](../../openapi/crm/README.md); the
implementation work list is [`docs/crm/`](../../docs/crm/README.md).

| Path | What |
|---|---|
| `audience/`, `messaging/`, `automation/`, `sales/`, `acquisition/`, `insights/`, `workspace/`, `edge/`, `intake/` | The nine services (`gen/` + `impl/`) |
| `runtime/` | `rerp_crm_runtime`: context, errors, events, audit, idempotency, tokens, conditions |
| `migrations/` | Per-service migrations (`<service>/`), the RLS template (`templates/`), `apply_order.txt`; `0001_accounts.sql` and `0002_accounts_phone.sql` belong to `accounts` in the RERP tenant database |
| `accounts/`, `conversion/` | The customer master and the signed conversion contract (ADR 003), in the RERP tenant database |
| `pipeline/` | Today's leads service against the website's Supabase; replaced by `sales` (CRM-SAL-03) |
| `gen/` | The old CRM gateway crate (earlier 13-service layout); not built |

Databases: `crm` (one schema per service, plus `audit`), `crm_insights`, `crm_intake` — see
[14-data-model.md](../../openapi/crm/docs/14-data-model.md). Migrations: `cargo run -p rerp_migrator --features crm --
generate --suite crm` (once LifeModels exist).
