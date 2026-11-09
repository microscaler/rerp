# crm/intake — the quarantine (ADR 004)

Implementation of `openapi/crm/intake/openapi.yaml`. Design and purpose: [openapi/crm/intake/README.md](../../../openapi/crm/intake/README.md).

| | |
|---|---|
| Crates | `gen/` (`rerp_crm_intake_gen`, generated, never edited) · `impl/` (`rerp_crm_intake`, binary `intake`) |
| Contract | [openapi.yaml](../../../openapi/crm/intake/openapi.yaml) · [asyncapi.yaml](../../../openapi/crm/intake/asyncapi.yaml) |
| Tables | [data-model/intake.yaml](../../../openapi/crm/docs/data-model/intake.yaml) — database `crm_intake`, schema `intake`; migrations in `../migrations/intake/` |
| Shared code | `../runtime` (`rerp_crm_runtime`) |
| State | Every operation is a stub returning its contract example; real handlers land story by story. |

## Stories

| Story | Title |
|---|---|
| [CRM-INT-00](../../../docs/crm/stories/CRM-INT-00-intake-data-model.md) | LifeModels and migrations (2 tables) |
| [CRM-INT-01](../../../docs/crm/stories/CRM-INT-01-intake-quarantine.md) | The quarantine: screening, confirmation, admission and the staff view (4 ops) |
| [CRM-INT-EV](../../../docs/crm/stories/CRM-INT-EV-intake-consumers.md) | Event consumers (6 events) |

## Run locally

```bash
cd microservices
cargo run -p rerp_crm_intake --bin intake -- --spec ../openapi/crm/intake/openapi.yaml --config crm/intake/impl/config/config.yaml
```

Regenerate after a contract change: `tooling/.venv/bin/rerp gen suite crm --service intake` (gen only; never touches impl).
