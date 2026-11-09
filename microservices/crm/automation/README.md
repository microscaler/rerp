# crm/automation — what happens next

Implementation of `openapi/crm/automation/openapi.yaml`. Design and purpose: [openapi/crm/automation/README.md](../../../openapi/crm/automation/README.md).

| | |
|---|---|
| Crates | `gen/` (`rerp_crm_automation_gen`, generated, never edited) · `impl/` (`rerp_crm_automation`, binary `automation`) |
| Contract | [openapi.yaml](../../../openapi/crm/automation/openapi.yaml) · [asyncapi.yaml](../../../openapi/crm/automation/asyncapi.yaml) |
| Tables | [data-model/automation.yaml](../../../openapi/crm/docs/data-model/automation.yaml) — database `crm`, schema `automation`; migrations in `../migrations/automation/` |
| Shared code | `../runtime` (`rerp_crm_runtime`) |
| State | Every operation is a stub returning its contract example; real handlers land story by story. |

## Stories

| Story | Title |
|---|---|
| [CRM-AUT-00](../../../docs/crm/stories/CRM-AUT-00-automation-data-model.md) | LifeModels and migrations (11 tables) |
| [CRM-AUT-01](../../../docs/crm/stories/CRM-AUT-01-automation-workflows.md) | Workflows, versions, steps and triggers (27 ops) |
| [CRM-AUT-02](../../../docs/crm/stories/CRM-AUT-02-automation-engine.md) | Enrolments, the step engine and timers (20 ops) |
| [CRM-AUT-EV](../../../docs/crm/stories/CRM-AUT-EV-automation-consumers.md) | Event consumers (30 events) |

## Run locally

```bash
cd microservices
cargo run -p rerp_crm_automation --bin automation -- --spec ../openapi/crm/automation/openapi.yaml --config crm/automation/impl/config/config.yaml
```

Regenerate after a contract change: `tooling/.venv/bin/rerp gen suite crm --service automation` (gen only; never touches impl).
