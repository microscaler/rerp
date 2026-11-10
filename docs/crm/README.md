# CRM implementation

- **Status**: ACTIVE
- **Authority**: Working
- **Owner**: RERP CRM
- **Scope**: crm.implementation
- **Last reviewed**: 2026-11-09
- **Supersedes**: None
- **Superseded by**: None

The work list for building RERP's CRM services from their contracts, sized so a local coding model can take one story
per session under supervision. The design is done and is not changed by these stories: contracts
(`openapi/crm/<service>/openapi.yaml`, `asyncapi.yaml`), tables (`openapi/crm/docs/data-model/`), conventions and
plan (`openapi/crm/docs/09`–`14`), decisions (ADR 003, ADR 004).

- **Coding agent:** read [HANDOVER.md](HANDOVER.md), then take the next story whose dependencies are done.
- **Supervisor:** review each finished story with [SUPERVISION.md](SUPERVISION.md).
- **Consumer stories:** PriceWhisperer `docs/Epics/EPIC_55/` (US_55_01–40) say what each screen needs; every story
  below lists the ones it serves, and those stories link back here.

## State today

- Contracts: 9 services, 511 operations, 75 event types, every example valid (`openapi/crm/tools/check_specs.py`).
- Data model: 128 tables in 8 services (`openapi/crm/tools/data_model.py`), applied to Postgres 16 with row-level security verified.
- Code: `gen/` and `impl/` crates for all nine services and `microservices/crm/runtime`, compiling with
  `clippy -D warnings`; every operation has a stub that returns its contract example. The migrator has a `crm` suite.

## Stories (58, 371 points)

Order: build step, then the table order. A story may start when everything in **Depends on** is done.

| Step | Story | Title | Depends on | Points | Serves (Epic 55) | Status |
|---|---|---|---|---|---|---|
| 0 | [CRM-DB-01](stories/CRM-DB-01-local-database-and-migrations.md) | Local CRM databases and the migration pipeline | — | 3 | — | ✅ |
| 0 | [CRM-PLT-01](stories/CRM-PLT-01-gcp-platform.md) | CRM platform resources on GCP (gcp-infrastructure) | — | 8 | — | ⬜ |
| 0 | [CRM-RT-01](stories/CRM-RT-01-context-and-errors.md) | request context and errors | — | 5 | — | ✅ |
| 0 | [CRM-RT-02](stories/CRM-RT-02-events.md) | events: envelope, outbox, relay, consumer | CRM-RT-01, CRM-DB-01 | 8 | — | ✅ |
| 0 | [CRM-RT-03](stories/CRM-RT-03-audit-and-idempotency.md) | audit and idempotency | CRM-RT-01, CRM-WSP-00 | 5 | — | ⬜ |
| 1 | [CRM-RT-05](stories/CRM-RT-05-conditions.md) | the condition tree | CRM-AUD-00 | 8 | — | ⬜ |
| 1 | [CRM-AUD-00](stories/CRM-AUD-00-audience-data-model.md) | LifeModels and migrations | CRM-DB-01 | 5 | — | ✅ |
| 1 | [CRM-AUD-01](stories/CRM-AUD-01-audience-contacts.md) | Contacts, organisations and notes | CRM-AUD-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 9 | US_55_03, US_55_04, US_55_05, US_55_11, US_55_18, US_55_19, US_55_22, US_55_23, US_55_35 | ⬜ |
| 1 | [CRM-AUD-02](stories/CRM-AUD-02-audience-lists.md) | Lists, subscriptions, consent and suppressions | CRM-AUD-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 6 | US_55_03, US_55_04, US_55_06, US_55_08, US_55_14, US_55_29, US_55_32 | ⬜ |
| 1 | [CRM-AUD-03](stories/CRM-AUD-03-audience-segments.md) | Tags, segments and bulk jobs | CRM-AUD-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 7 | US_55_03, US_55_04, US_55_07, US_55_08, US_55_29, US_55_32 | ⬜ |
| 1 | [CRM-AUD-EV](stories/CRM-AUD-EV-audience-consumers.md) | event consumers | CRM-RT-02, CRM-AUD-00 | 8 | — | ⬜ |
| 1 | [CRM-WSP-00](stories/CRM-WSP-00-workspace-data-model.md) | LifeModels and migrations | CRM-DB-01 | 5 | — | ⬜ |
| 1 | [CRM-WSP-01](stories/CRM-WSP-01-workspace-workspaces.md) | Workspaces, memberships and the team facade | CRM-WSP-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 5 | US_55_22, US_55_29 | ⬜ |
| 1 | [CRM-WSP-02](stories/CRM-WSP-02-workspace-fields.md) | Custom fields and field groups | CRM-WSP-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 5 | US_55_20, US_55_24, US_55_29 | ⬜ |
| 1 | [CRM-WSP-03](stories/CRM-WSP-03-workspace-settings.md) | Compliance, brand kit and notification rules | CRM-WSP-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 4 | US_55_29 | ⬜ |
| 1 | [CRM-WSP-04](stories/CRM-WSP-04-workspace-access.md) | API keys, webhooks and integrations | CRM-WSP-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 6 | US_55_29 | ⬜ |
| 1 | [CRM-WSP-05](stories/CRM-WSP-05-workspace-jobs.md) | Import and export jobs, data requests and the audit log | CRM-WSP-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 5 | US_55_03, US_55_07, US_55_10, US_55_27, US_55_29 | ⬜ |
| 1 | [CRM-WSP-EV](stories/CRM-WSP-EV-workspace-consumers.md) | event consumers | CRM-RT-02, CRM-WSP-00 | 3 | — | ⬜ |
| 2 | [CRM-RT-04](stories/CRM-RT-04-tokens.md) | signed tokens for the edge | CRM-RT-01 | 3 | — | ⬜ |
| 2 | [CRM-EDG-01](stories/CRM-EDG-01-edge-capture.md) | Forms, surveys, registrations and confirmation links | CRM-RT-01, CRM-RT-02, CRM-RT-04 | 3 | — | ⬜ |
| 2 | [CRM-EDG-02](stories/CRM-EDG-02-edge-tracking.md) | Pixels, click redirects, unsubscribe, preferences, tracking and ad clicks | CRM-RT-01, CRM-RT-02, CRM-RT-04 | 6 | — | ⬜ |
| 2 | [CRM-EDG-03](stories/CRM-EDG-03-edge-webhooks.md) | Provider and ad-platform webhooks and OAuth callbacks | CRM-RT-01, CRM-RT-02, CRM-RT-04 | 3 | — | ⬜ |
| 2 | [CRM-INT-00](stories/CRM-INT-00-intake-data-model.md) | LifeModels and migrations | CRM-DB-01 | 3 | — | ⬜ |
| 2 | [CRM-INT-01](stories/CRM-INT-01-intake-quarantine.md) | The quarantine: screening, confirmation, admission and the staff view | CRM-INT-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 3 | — | ⬜ |
| 2 | [CRM-INT-EV](stories/CRM-INT-EV-intake-consumers.md) | event consumers | CRM-RT-02, CRM-INT-00 | 4 | — | ⬜ |
| 3 | [CRM-AUT-00](stories/CRM-AUT-00-automation-data-model.md) | LifeModels and migrations | CRM-DB-01 | 3 | — | ⬜ |
| 3 | [CRM-AUT-01](stories/CRM-AUT-01-automation-workflows.md) | Workflows, versions, steps and triggers | CRM-AUT-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 11 | US_55_02, US_55_03, US_55_12, US_55_13, US_55_14 | ⬜ |
| 3 | [CRM-AUT-02](stories/CRM-AUT-02-automation-engine.md) | Enrolments, the step engine and timers | CRM-AUT-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 8 | US_55_03, US_55_04, US_55_12, US_55_13, US_55_19 | ⬜ |
| 3 | [CRM-AUT-EV](stories/CRM-AUT-EV-automation-consumers.md) | event consumers | CRM-RT-02, CRM-AUT-00 | 9 | — | ⬜ |
| 3 | [CRM-MSG-00](stories/CRM-MSG-00-messaging-data-model.md) | LifeModels and migrations | CRM-DB-01 | 6 | — | ⬜ |
| 3 | [CRM-MSG-01](stories/CRM-MSG-01-messaging-content.md) | Templates, snippets, assets and postal addresses | CRM-MSG-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 7 | US_55_06, US_55_08, US_55_11, US_55_12, US_55_29, US_55_33 | ⬜ |
| 3 | [CRM-MSG-02](stories/CRM-MSG-02-messaging-sending-setup.md) | Senders, SMS senders, tracking domains, list exclusions and providers | CRM-MSG-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 6 | US_55_06, US_55_08, US_55_29 | ⬜ |
| 3 | [CRM-MSG-03](stories/CRM-MSG-03-messaging-campaigns.md) | Campaigns, sends and the send worker | CRM-MSG-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 7 | US_55_02, US_55_08, US_55_10, US_55_24, US_55_32 | ⬜ |
| 3 | [CRM-MSG-04](stories/CRM-MSG-04-messaging-reports.md) | Campaign and deliverability reports | CRM-MSG-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 4 | US_55_02, US_55_10, US_55_24 | ⬜ |
| 3 | [CRM-MSG-05](stories/CRM-MSG-05-messaging-mailboxes.md) | Connected mailboxes (1:1 email) | CRM-MSG-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 4 | US_55_26 | ⬜ |
| 3 | [CRM-MSG-06](stories/CRM-MSG-06-messaging-social.md) | Social accounts, posts, approval and engagement | CRM-MSG-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 8 | US_55_17 | ⬜ |
| 3 | [CRM-MSG-EV](stories/CRM-MSG-EV-messaging-consumers.md) | event consumers | CRM-RT-02, CRM-MSG-00 | 5 | — | ⬜ |
| 4 | [CRM-SAL-00](stories/CRM-SAL-00-sales-data-model.md) | LifeModels and migrations | CRM-DB-01 | 7 | — | ⬜ |
| 4 | [CRM-SAL-01](stories/CRM-SAL-01-sales-pipelines.md) | Pipelines, stages and deal settings | CRM-SAL-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 9 | US_55_18, US_55_19, US_55_20, US_55_22 | ⬜ |
| 4 | [CRM-SAL-02](stories/CRM-SAL-02-sales-deals.md) | Deals, deal contacts, notes, files, emails and status | CRM-SAL-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 10 | US_55_04, US_55_05, US_55_18, US_55_19, US_55_22 | ⬜ |
| 4 | [CRM-SAL-03](stories/CRM-SAL-03-sales-leads.md) | Leads and triage | CRM-SAL-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 5 | US_55_18, US_55_21 | ⬜ |
| 4 | [CRM-SAL-04](stories/CRM-SAL-04-sales-tasks.md) | Tasks, task types, outcomes, calendars and saved responses | CRM-SAL-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 16 | US_55_19, US_55_20, US_55_22 | ⬜ |
| 4 | [CRM-SAL-05](stories/CRM-SAL-05-sales-teams.md) | Teams, members and assignment | CRM-SAL-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 5 | — | ⬜ |
| 4 | [CRM-SAL-EV](stories/CRM-SAL-EV-sales-consumers.md) | event consumers | CRM-RT-02, CRM-SAL-00 | 5 | — | ⬜ |
| 5 | [CRM-ACQ-00](stories/CRM-ACQ-00-acquisition-data-model.md) | LifeModels and migrations | CRM-DB-01 | 8 | — | ⬜ |
| 5 | [CRM-ACQ-01](stories/CRM-ACQ-01-acquisition-forms.md) | Forms, submissions, tracked sites and visitors | CRM-ACQ-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 6 | US_55_14, US_55_15 | ⬜ |
| 5 | [CRM-ACQ-02](stories/CRM-ACQ-02-acquisition-conversions.md) | Conversions, goals, UTM and marketing analytics | CRM-ACQ-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 10 | US_55_15, US_55_16, US_55_24 | ⬜ |
| 5 | [CRM-ACQ-03](stories/CRM-ACQ-03-acquisition-ad-accounts.md) | Ad accounts, audiences and sync | CRM-ACQ-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 8 | US_55_31, US_55_32, US_55_33, US_55_34 | ⬜ |
| 5 | [CRM-ACQ-04](stories/CRM-ACQ-04-acquisition-ad-buying.md) | Ad buys and creatives | CRM-ACQ-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 6 | US_55_31, US_55_33, US_55_35 | ⬜ |
| 5 | [CRM-ACQ-05](stories/CRM-ACQ-05-acquisition-ad-tracking.md) | Ad tracking, conversion relay and attribution | CRM-ACQ-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 10 | US_55_31, US_55_34, US_55_35 | ⬜ |
| 5 | [CRM-ACQ-EV](stories/CRM-ACQ-EV-acquisition-consumers.md) | event consumers | CRM-RT-02, CRM-ACQ-00 | 6 | — | ⬜ |
| 5 | [CRM-INS-00](stories/CRM-INS-00-insights-data-model.md) | LifeModels and migrations | CRM-DB-01 | 5 | — | ⬜ |
| 5 | [CRM-INS-01](stories/CRM-INS-01-insights-read-models.md) | Read models and deal, pipeline and forecast reports | CRM-INS-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 8 | US_55_24 | ⬜ |
| 5 | [CRM-INS-02](stories/CRM-INS-02-insights-marketing-reports.md) | Contact, email, automation, task and revenue reports | CRM-INS-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 8 | US_55_13, US_55_24 | ⬜ |
| 5 | [CRM-INS-03](stories/CRM-INS-03-insights-dashboards.md) | Dashboards and widgets | CRM-INS-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 5 | US_55_25 | ⬜ |
| 5 | [CRM-INS-04](stories/CRM-INS-04-insights-scores.md) | Named scores | CRM-INS-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 | 4 | US_55_23, US_55_29, US_55_32 | ⬜ |
| 5 | [CRM-INS-EV](stories/CRM-INS-EV-insights-consumers.md) | event consumers | CRM-RT-02, CRM-INS-00 | 14 | — | ⬜ |
| — | [CRM-PTL-01](stories/CRM-PTL-01-portal-cutover.md) | Move the portal and the mock to the new contracts | — | 8 | — | ⬜ |

Status: ⬜ to do · 🔄 in progress · 👀 in review · ✅ merged. The supervisor updates it on merge.

## Reports

Each finished story has `reports/<story-id>.md` (template in HANDOVER.md).
