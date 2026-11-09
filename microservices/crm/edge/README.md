# crm/edge — the public surface

Implementation of `openapi/crm/edge/openapi.yaml`. Design and purpose: [openapi/crm/edge/README.md](../../../openapi/crm/edge/README.md).

| | |
|---|---|
| Crates | `gen/` (`rerp_crm_edge_gen`, generated, never edited) · `impl/` (`rerp_crm_edge`, binary `edge`) |
| Contract | [openapi.yaml](../../../openapi/crm/edge/openapi.yaml) · [asyncapi.yaml](../../../openapi/crm/edge/asyncapi.yaml) |
| Tables | none: the edge holds no database credentials (ADR 004) |
| Shared code | `../runtime` (`rerp_crm_runtime`) |
| State | Every operation is a stub returning its contract example; real handlers land story by story. |

## Stories

| Story | Title |
|---|---|
| [CRM-EDG-01](../../../docs/crm/stories/CRM-EDG-01-edge-capture.md) | Forms, surveys, registrations and confirmation links (4 ops) |
| [CRM-EDG-02](../../../docs/crm/stories/CRM-EDG-02-edge-tracking.md) | Pixels, click redirects, unsubscribe, preferences, tracking and ad clicks (12 ops) |
| [CRM-EDG-03](../../../docs/crm/stories/CRM-EDG-03-edge-webhooks.md) | Provider and ad-platform webhooks and OAuth callbacks (3 ops) |

## Run locally

```bash
cd microservices
cargo run -p rerp_crm_edge --bin edge -- --spec ../openapi/crm/edge/openapi.yaml --config crm/edge/impl/config/config.yaml
```

Regenerate after a contract change: `tooling/.venv/bin/rerp gen suite crm --service edge` (gen only; never touches impl).
