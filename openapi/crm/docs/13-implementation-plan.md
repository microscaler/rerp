# 13 — CRM implementation plan

> **Status:** Plan, 2026-11-09. **Inputs:** the nine service contracts, [08](08-service-decomposition.md) (split,
> entities, flows), [ADR 004](../../../docs/adrs/004-crm-capture-intake-and-workspaces.md), PriceWhisperer Epic 55
> (stories, acceptance criteria and test cases). **Answers:** what has to exist before the first handler is written,
> in which order the services are built, and when a service counts as done.

---

## 1. Where things stand

| Piece | State |
|---|---|
| Contracts | Nine `openapi.yaml` (511 operations) and nine `asyncapi.yaml` (75 event types), all checked by `tools/check_specs.py`: conventions, valid examples on every request and response, events tied to operations. |
| Generated crates | `microservices/crm/<service>/gen` for all nine, in the Cargo workspace, `cargo clippy -D warnings` clean. |
| Implementation crates | `microservices/crm/<service>/impl` for all nine (lib + binary, `services/`, `validators/`, `models/` with the Lifeguard registry) and `microservices/crm/runtime`; every operation has a stub that returns its contract example, so the portal can run against RERP before real handlers exist. The migrator has a `crm` suite. |
| Data model | 128 tables in [14-data-model.md](14-data-model.md) (source `data-model/<service>.yaml`), applied to Postgres 16 with row-level security verified; LifeModels are the first story of each service. |
| Stories | [docs/crm/](../../../docs/crm/README.md): 58 stories for a local coding model under supervision. |
| Implemented today | `crm/pipeline` (leads, stages, conversion, against the website's Supabase), `crm/accounts` (customer master and conversion intake, ADR 003), `crm/conversion` (signed conversion contract and Pub/Sub). |
| Portal | Every CRM screen built against the mock; moves per [12-portal-migration.md](12-portal-migration.md). |
| Old folders | `contacts`, `outreach`, `social`, `platform`, `teams`, `tasks`, `marketing`, `reporting`, `intelligence`, `engagement`, `livechat`: deprecated, kept until the service that replaces each is implemented. |

## 2. Before the first service

### 2.1 Platform (gcp-infrastructure, Crossplane; nothing by hand)

| Resource | For |
|---|---|
| Cloud SQL Postgres instance `crm` with databases `crm` (schemas audience, messaging, automation, sales, acquisition, workspace), `crm_insights`, `crm_intake` | ADR 004 §1, §2; 08 §5.4 |
| One Postgres role per service, owning only its schema; RLS forced on every table | 08 §5.4, US_55_39 |
| Pub/Sub topics `crm.<service>.v1` (one per service, nine), subscriptions `<consumer>-<publisher>` with filters on `event_type`, dead-letter topics, retry policy | [10-events.md](10-events.md) |
| GKE Autopilot Deployments (one per component), Kubernetes service accounts bound by Workload Identity to GCP service accounts with exactly their topics, subscriptions and database | ADR 004 §7 |
| Secret Manager entries for provider keys, OAuth tokens, ad developer tokens, webhook signing secrets, the token-signing key the edge and intake share | 08 §5.4 |
| Object storage bucket (assets, deal files, exports) with signed URLs | messaging, sales, workspace |
| The tracking host per workspace domain (`t.<domain>`) routed to the edge, with managed certificates | edge |
| Scaling: the edge on request rate; workers on subscription backlog | ADR 004 §7 |

### 2.2 Shared crates (in `microservices/crm/`)

| Crate | Does | Used by |
|---|---|---|
| `crm-context` | Reads the verified JWT and `X-Workspace-Id`, checks membership (cached, dropped on `member.changed`), checks `x-required-role`, opens the transaction with the RLS session variables set, maps errors to `ErrorResponse` with `reason`. Grown from `crm/accounts`' `auth.rs` and `identity.rs`. | every service but the edge |
| `crm-events` | The envelope type, an outbox table and relay (publish after commit), an idempotent consumer (processed `event_id` table, ack after commit, nack to retry, dead-letter after 10). Grown from `crm/conversion`'s Pub/Sub code. | every service |
| `crm-audit` | Writes the audit row (changed fields before and after) in the same transaction. | every service with writes |
| `crm-idempotency` | Stores `Idempotency-Key` → first response for 24 hours. | services with side-effecting commands |
| `crm-conditions` | The condition tree types, validation, and its translation to SQL over audience's facts table; a single-contact evaluator for automation. | audience, automation, messaging (preview), workspace (export filters) |
| `crm-tokens` | Signs and verifies the edge's tokens (open, click, unsubscribe, preferences, confirm, calendar) with the shared key. | edge, messaging, intake, sales |

### 2.3 Practices

- Database access through Lifeguard pools, as in `crm/accounts`.
- Migrations per service under `microservices/crm/migrations/<service>/`, each table with `workspace_id`, an RLS
  policy, and the audit trigger or helper.
- Tests: contract tests generated from each spec's examples (each operation returns a body that validates), isolation
  tests (workspace A cannot see B: TC-1 of US_55_39 for every service), role-matrix tests from
  [11-permissions.md](11-permissions.md), consumer idempotency tests (deliver every event twice).

## 3. Order

From [08 §10.3](08-service-decomposition.md), with the platform work in front:

| Step | Service | First slice (what proves it) | Unblocks |
|---|---|---|---|
| 0 | platform + shared crates | A hello-world service on Autopilot with Workload Identity, writing an audited row under RLS and publishing through the outbox | everything |
| 1 | **workspace** | Workspaces and memberships, roles facade (`GET /team`), custom fields and field groups, audit log | the `X-Workspace-Id` check every other service relies on |
| 1 | **audience** | Contacts, organisations, tags, lists, subscriptions and consent history, suppressions; segments over the facts table | messaging, automation, sales, acquisition |
| 2 | **edge** + **intake** | Form submit → intake → confirmation email → confirm → `intake.admitted` → audience admits the contact (08 §7.3, ADR 004 acceptance) | the website forms leave Supabase |
| 3 | **messaging** | Templates, senders, a campaign to a list end to end with opens and clicks through the edge; transactional send for intake's confirmation | automation |
| 3 | **automation** | Triggers on `subscription.changed` and `tag.added`, SEND_MESSAGE and WAIT steps, enrolments with retry | the welcome series |
| 4 | **sales** | Moves `crm/pipeline` onto the CRM store: leads, deals, stages, tasks; `PATCH /deals/{id}/status` with the signed conversion | retires `pipeline` against Supabase |
| 5 | **acquisition** | Forms, tracked sites, conversions and goals; then ad accounts, audiences and the conversion relay | insights' revenue reports |
| 5 | **insights** | Read models for the deal and contact reports, dashboards, named scores with `score.changed` | — |

Each step retires the deprecated folders it replaces (08 §10.5) and moves the portal's calls for those screens off the
mock.

## 4. Done, per service

- Every operation implemented (`x-brrtrouter-impl: true` registered), or explicitly left on the generated mock with a
  reason in the service README.
- Isolation, role-matrix, contract and consumer-idempotency tests green in CI.
- Its events published through the outbox and consumed idempotently; dead-letter topics alerting to the pager channel
  (failures only).
- The Epic 55 stories it covers meet their acceptance criteria, including the non-functional ones (08 §4 runtime
  targets).
- Deployed to pp through Flux; the portal screens it serves run against it.

## 5. Open questions (to settle before the step that needs them)

| # | Question | Needed by |
|---|---|---|
| Q1 | Email and SMS provider for the first release (adapter interface is in messaging's `SendingProviders`). | step 3 |
| Q2 | Tracking host per workspace (`t.<brand domain>`), or one shared host with the workspace in the token. | step 2 |
| Q3 | Retention of engagement events and send ledgers (default proposal: 25 months, then aggregates only). | step 3 |
| Q4 | Which ad platforms in the first acquisition release (Meta and Google proposed; 07-ad-buying-landscape.md). | step 5 |
| Q5 | Whether the website's existing Supabase capture moves to the edge in step 2 or keeps feeding intake for a while (ADR 004 allows both). | step 2 |
