# RERP CRM — design

> **Status:** Current, 2026-11-09. Replaces the earlier 13-service, four-database design (kept in docs/01 to 05 for
> history). **Decided by:** [ADR 003](../../docs/adrs/003-crm-funnel-conversion-boundary.md),
> [ADR 004](../../docs/adrs/004-crm-capture-intake-and-workspaces.md),
> [08-service-decomposition.md](docs/08-service-decomposition.md) (D1 to D14).

## 1. What we are building

A CRM with a marketing suite, built consumer-first: the PriceWhisperer platform portal's CRM screens (Epic 55, 40
stories) were designed and run against a mock, the operations they call (237) were tabulated, and the contracts were
derived from them. The scope is what those screens need: contacts and consent, segments, email and SMS campaigns,
automations, a sales pipeline with tasks, forms and tracking, ads and attribution, reports, dashboards and scores, and
the settings to run it, for one or several brands per organisation.

Out of scope for v1: live chat, subscription billing and MRR (RERP `sales/subscription` and PriceWhisperer billing own
them), enrichment and ICP mining (`intelligence`, deferred), and anything a screen does not need.

## 2. Principles

1. **The contract comes first.** Nine OpenAPI documents and nine AsyncAPI documents are the source of truth; the Rust
   crates are generated from them, and a checker enforces the conventions ([09](docs/09-api-conventions.md)).
2. **Split by the questions a service answers**, not by entity: audience answers "can we reach this person?",
   messaging "what did we send?", automation "what happens next?", sales "what are we selling?", acquisition "where did
   they come from, and what are they worth?", insights "what does it add up to?", workspace "how is this run?".
3. **Events, not shared tables.** A service owns its schema; others learn through `crm.<service>.v1` topics. The few
   synchronous calls form no cycle ([08 §5.2](docs/08-service-decomposition.md)).
4. **The internet touches nothing with a credential.** The edge validates and publishes; services re-validate.
5. **No consent, no contact.** Anonymous input waits in intake until it is confirmed or screened (ADR 004).
6. **Every row belongs to a workspace**, isolated by row-level security, and every write is audited.
7. **The customer master stays in RERP.** A prospect becomes a customer only through a signed conversion (ADR 003).

## 3. Components

| Component | Owns | Runtime |
|---|---|---|
| audience | contacts, organisations, tags, segments and the condition tree, lists, subscriptions and consent history, suppressions, notes, timeline, the per-contact facts projection, bulk jobs | API; segment p95 ≤ 700 ms at 100 k contacts |
| messaging | templates, snippets, assets, campaigns, the send ledger and engagement, senders and domains, SMS senders, tracking domains, postal addresses, list exclusions, providers, mailboxes, social accounts and posts | API and send workers, ≥ 100 k recipients / hour / workspace |
| automation | workflows and step graphs, triggers, the action catalogue, recipes, enrolments, durable timers | API and step engine, exactly once per step, timers within 60 s |
| sales | leads and triage, pipelines and stages, deals with contacts, roles, notes, files, emails; tasks, types, outcomes, calendars; saved responses; teams and assignment; the signed conversion | API |
| acquisition | forms and submissions, tracked sites and events, visitors and clicks, conversions and goals, ad accounts, audiences, creatives, buys, link tagging, conversion relay, attribution | API and adapter workers |
| insights | read models for every report, dashboards and widgets, named scores | event consumers, own analytics database, ≤ 5 min behind |
| workspace | workspaces and memberships, custom field definitions, API keys, webhooks, notification rules, compliance, data requests and erasure, brand kit, jobs, audit log | API and job runner |
| edge | nothing (validates, signs and verifies tokens, publishes) | public, scales on request rate |
| intake | pending submissions only (7 days pending, 30 days decided) | worker and a staff API |

Details per component are in each folder's README and [08 §4](docs/08-service-decomposition.md); entity diagrams in
[08 §6](docs/08-service-decomposition.md); flows and state machines in [08 §7](docs/08-service-decomposition.md).

## 4. Data

Every table, with ERDs: [14-data-model.md](docs/14-data-model.md) (design source: [docs/data-model/](docs/data-model/)).

| Store | Holds |
|---|---|
| CRM store: Postgres on GCP (Crossplane) | one schema per service (audience, messaging, automation, sales, acquisition, workspace), RLS on `workspace_id` everywhere |
| Analytics database | insights' read models, rebuildable from events |
| Intake database | pending submissions, written only by intake |
| RERP tenant database | `crm_accounts` and the conversion ledger (ADR 003) |
| Object storage | assets, deal files, exports (signed URLs) |
| Secret Manager | provider keys, OAuth tokens, ad tokens, webhook and token-signing secrets |

## 5. Interfaces

| Interface | Where | Rules |
|---|---|---|
| HTTP | `/api/v1/crm/<service>/…` behind the gateway; the edge on `t.<workspace domain>` | [09-api-conventions.md](docs/09-api-conventions.md) |
| Events | Pub/Sub `crm.<service>.v1`, envelope with `data` | [10-events.md](docs/10-events.md) |
| Roles | owner, admin, marketer, sales, read-only per workspace | [11-permissions.md](docs/11-permissions.md) |
| Conversion to RERP | `crm-conversions`, signed (`microservices/crm/conversion`) | ADR 003 |

## 6. Reading order

1. This page.
2. [08-service-decomposition.md](docs/08-service-decomposition.md): forces, options, the split, entities, flows,
   decisions.
3. [ADR 003](../../docs/adrs/003-crm-funnel-conversion-boundary.md) and
   [ADR 004](../../docs/adrs/004-crm-capture-intake-and-workspaces.md).
4. [09-api-conventions.md](docs/09-api-conventions.md), then the service folders.
5. [13-implementation-plan.md](docs/13-implementation-plan.md) and [14-data-model.md](docs/14-data-model.md) before writing code, then the
   stories in [docs/crm/](../../docs/crm/README.md);
   [12-portal-migration.md](docs/12-portal-migration.md) before touching the portal.
