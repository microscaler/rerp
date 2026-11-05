# Marketing platform feature map — reverse-engineered from ActiveCampaign

> **Status:** Working document, 2026-11-05. Source: a full walk of an ActiveCampaign trial account
> (every primary section, every secondary panel, the automation trigger and action catalogues, the
> settings tree). Used to curate v1 of RERP's CRM + marketing platform and to keep the Claude Design
> mock honest. Nothing here is vendor-specific in our model; the names are theirs, the shapes are ours.

## 1. Inventory — everything the product exposes

### Contacts
| AC feature | Path | Our service | Status |
|---|---|---|---|
| Contacts list, profile, timeline | `/app/contacts` | `crm/contacts` | in spec |
| Accounts (B2B company grouping of contacts) | `/app/accounts` | `crm/accounts` is the *converted-customer* record (ADR 003); a marketing-side company grouping needs a decision | **decide** |
| Lists | `/app/lists` | `crm/outreach` lists + subscriptions | in spec |
| Segments | `/app/segments` | `crm/contacts` segments, shared `ConditionGroup` | in spec |
| Tags | `/app/tags` | `crm/contacts` tags | in spec |
| Custom fields | `/app/settings/fields/contacts` | `crm/platform` custom fields | in spec |
| Lead scores (rule-based, several named scores, contact and deal) | `/app/scores` | `crm/intelligence` is Bayesian only | **add rule-based scores** |
| List exclusions (block list per list, distinct from suppression) | `/app/list-exclusions` | — | **add** |
| Database sync (external DB → contacts) | `/app/sync-database` | — | defer |
| Import / export with status | `/app/contacts/import-export` | `crm/contacts` import/export jobs | in spec |
| Email subscriptions view (contact × list grid) | `/app/lists/email-subscriptions` | `/contacts/{id}/subscriptions` | in spec |
| Bulk edits | `/app/bulk-edits` | — | **add** (job: segment → add tag / list / field) |
| Archive contact | action | — | **add** |
| Reports: contact performance, field breakdown, tag trends | `/app/reports/...` | `crm/reporting` | partial |
| Reports: nearby contacts (geo) | `/report/#/contacts/nearby` | — | defer |

### Automations
| AC feature | Our service | Status |
|---|---|---|
| Automations list, map, builder | `crm/automation` steps/enrolments/map | in spec |
| **Several start triggers per automation** | one `trigger_type` per workflow | **change: `triggers[]`** |
| Automation messages (every email owned by automations, in one list) | `/templates?kind=AUTOMATION` + usage | **add usage view** |
| Transactional email list | `crm/outreach` transactional | in spec |
| Reports: all automations, jump-to | `crm/reporting` | partial |
| Trigger: on a schedule | `TIME_BASED` | in spec |
| Trigger: subscribes / unsubscribes from list | yes | in spec |
| Trigger: submits a form | yes | in spec |
| Trigger: opens / clicks / bounces | yes | in spec |
| Trigger: **shares / forwards / replies** to an email | `MESSAGE_REPLIED` only | **add FORWARDED, SHARED** (reply needs inbound mail) |
| Trigger: web page visited, event recorded | yes | in spec |
| Trigger: tag added / removed, contact created, field changes | yes | in spec |
| Trigger: **deal field / value / status changes, enters a pipeline, deal stage changes** | `DEAL_CREATED/STAGE_CHANGE/WON/LOST` | **add VALUE, STATUS, FIELD, ENTERED_PIPELINE** |
| Trigger: **account field changes** | — | **add** |
| Trigger: date based, RSS based | `DATE_FIELD` | **add RSS** |
| Trigger: score changes | yes | in spec |
| Trigger: **contact jumped to**, **task completed**, **file downloaded** | — | **add** |
| Trigger: WhatsApp flow completed | — | defer |
| Action: send email | `SEND_MESSAGE` | in spec |
| Action: **send a direct 1:1 email** (from a user's mailbox) | — | **add** (needs connected mailbox; v1 = send via sender identity with user's reply-to) |
| Action: send a transactional email | transactional templates | in spec |
| Action: **internal notification email** (to staff, no consent) | `NOTIFY` | in spec, make it explicit |
| Action: start a WhatsApp flow | — | defer |
| Action: wait, if/else, split | step kinds | in spec |
| Action: **go to** (jump within graph; terminal for its branch) | `GOTO` step | in spec (2026-11-05) |
| Action: **Jump To** (the renamed Goal: scope below/anywhere, continue-or-wait if not met, tracking) | `GOAL` + `GoalConfig` | in spec (2026-11-05) |
| Action: complete a deal task, add contact to account, account note, account owner | `COMPLETE_TASK`, `ADD_TO_ACCOUNT`, `ADD_ACCOUNT_NOTE`, `UPDATE_ACCOUNT_OWNER` | in spec (2026-11-05) |
| Action: format data (copy, date ±, extract phone/URL, math, capitalise) | `FORMAT_DATA` | in spec (2026-11-05), was deferred |
| Workflow settings: entry (once / any / after exit), exit rules (unsubscribe, conversion), unsubscribe scope, access groups, tracking to goal/conversion | `Workflow.reentry/exit_rules/unsubscribe_scope/access_group_ids/tracking` | in spec (2026-11-05) |
| Trigger: runs once / multiple, segment filter per trigger | `WorkflowTrigger.runs/segment` | in spec (2026-11-05) |
| Wait: period / specific day-time pattern in contact tz / date field on-before-after with passed-date rule / conditions with timeout | `WaitConfig` | in spec (2026-11-05) |
| Split: even / percent / test (winner by date or count) / until | `SplitConfig` | in spec (2026-11-05) |
| Action: start / **end another automation** | `START_WORKFLOW` | **add `STOP_WORKFLOW` target** |
| Action: webhook | `CALL_WEBHOOK` | in spec |
| Action: format data | — | defer |
| Action: subscribe / unsubscribe, update field, tag ±, note, score ± | yes | in spec |
| Action: **convert** (fire a conversion) | — | **add** with conversions entity |
| Action: **add a task** | `CREATE_TASK` placeholder | **add tasks entity** |
| Action: **archive contact** | — | **add** |
| Action: add deal, update title/status/owner/value/stage/custom field, deal task/note, deal score, account field | `CREATE_DEAL`, `UPDATE_DEAL_STAGE` | **add the rest** |

### Email
| AC feature | Our service | Status |
|---|---|---|
| Campaigns (one-time, A/B, RSS, automated) | `crm/outreach` campaigns | in spec |
| Content manager (image/asset library) | — | **add assets** (small) |
| Templates | versioned templates | in spec |
| **Message variables** (reusable snippets) | — | **add snippets** |
| Form archive | — | n/a |
| Brand kits (logo, colours, fonts → template defaults) | — | **add brand kit** (settings) |
| **Campaign folders** | `folder` string on campaign | in spec (thin) |
| **Resend to unopened** | — | **add** (campaign action) |
| Reports: campaign performance | yes | in spec |
| Reports: email client trends, predictive content, open/read trends | — | defer |

### Deals
| AC feature | Our service | Status |
|---|---|---|
| Deals kanban, pipelines, stages | `crm/pipeline` (single pipeline) | **add multiple pipelines** |
| **Tasks** (type, due, outcome, owner; on deals and contacts) | — | **add tasks entity** |
| Saved responses | — | defer |
| Deal custom fields | `crm/platform` custom fields (entity = deal) | in spec |
| Deal settings: **deal roles**, win probability (ML) | — | roles **add**; ML defer |
| Connected emails (reply from the CRM) | — | defer (inbound mail) |
| Task settings (types, outcomes) | — | with tasks |
| Calendars | — | defer |
| Reports: deals overview, **owner**, **funnel**, sales performance, task overview, **forecast** | `crm/reporting` has pipeline/forecast | **add owner + funnel + forecast weighting by stage probability × close date** |

### Website
| AC feature | Our service | Status |
|---|---|---|
| Forms (inline, floating, modal, hosted) | `crm/marketing` form builder | in spec |
| Pages (landing-page builder) | — | defer |
| Content manager | assets (see Email) | **add** |
| **Site messages** (on-site bar / popup with trigger rules) | — | **add** as `FormStyle.layout = SITE_MESSAGE` + message-only content |
| Site tracking (whitelist, code, event tracking API) | tracked sites + events | in spec |

### Business goals & conversions
| AC feature | Our service | Status |
|---|---|---|
| **Conversions** (named event with value; trigger = page / event / form / deal won / automation action) | — | **add entity** in `crm/marketing` |
| **Business goals** (target, progress, conversion attached) | — | **add** (small, on top of conversions) |
| Marketing revenue report | `CampaignReport.revenue` | **wire to conversions** |

### SMS / WhatsApp
| AC feature | Our service | Status |
|---|---|---|
| SMS broadcast + automation-triggered (US/AU/UK) | `Channel = SMS` on template/campaign/send | **add as a real channel**: provider adapter (Twilio-shaped), STOP/START keyword handling, quiet hours, per-country sender id, SMS consent on `Subscription` |
| WhatsApp flows (conversational qualify/sell) | — | defer (a builder of its own) |
| WhatsApp as a plain channel | `Channel = WHATSAPP` | later, same adapter contract |

### Settings
| AC feature | Our service | Status |
|---|---|---|
| Account, security, users | Sesame | role scopes only |
| **Addresses** (postal, several, chosen per sender) | `Sender.postal_address` | **add addresses entity** |
| **Brand kits** | — | **add** |
| Appearance | — | n/a |
| **Conversions** | — | **add** (above) |
| Developer (API keys, webhooks) | `crm/platform` | in spec |
| **Domains** (CNAME for tracking / hosted links) | — | **add tracking domain** to senders/settings |
| Import/export status | jobs | in spec |
| Integrations marketplace | `crm/platform` integrations | defer |
| **Data manager** (GDPR export / erase) | — | **add jobs** |
| **Notifications** (which staff get which internal alerts) | — | **add** |
| Saved responses | — | defer |

## 2. Curated v1 additions (ordered by shape change, then size)

Shape changes:
1. ~~`crm/automation.Workflow.triggers[]` — several start triggers per workflow.~~ Done 2026-11-05, with the builder's step configs (shape-diff § Automation builder).
2. `crm/pipeline` — multiple pipelines (`Pipeline` entity; stages belong to a pipeline; deal has `pipeline_id`).
3. SMS as a first-class channel in `crm/outreach` — adapter contract, consent, keywords, quiet hours.

New entities:
4. `Task` (`crm/pipeline` or new `crm/tasks`): subject, type, due, owner, outcome, linked contact/deal; `TASK_COMPLETED` trigger; `CREATE_TASK` action.
5. `Conversion` + `BusinessGoal` (`crm/marketing`): name, value, trigger definition, counts; `CONVERT` action; revenue attribution.
6. `ScoreRule` / `Score` (`crm/intelligence`): named scores (contact, deal), rules with points and decay.
7. `ListExclusion` (`crm/outreach`).
8. `Snippet` (message variables) and `Asset` (content manager) (`crm/outreach`).
9. `PostalAddress`, `BrandKit`, `TrackingDomain`, `NotificationRule`, `DataRequest` (GDPR) (`crm/platform` / `crm/outreach`).
10. `BulkEditJob`, `archive` on contact (`crm/contacts`).

New step/trigger/action enum values:
11. Steps: `GOTO`. Triggers: `MESSAGE_FORWARDED`, `MESSAGE_SHARED`, `FILE_DOWNLOADED`, `ENTERED_PIPELINE`, `DEAL_VALUE_CHANGED`, `DEAL_STATUS_CHANGED`, `DEAL_FIELD_CHANGED`, `ACCOUNT_FIELD_CHANGED`, `TASK_COMPLETED`, `RSS_ITEM`, `JUMPED_TO`. Actions: `SEND_1TO1_EMAIL`, `INTERNAL_NOTIFICATION` (rename of NOTIFY), `CONVERT`, `ARCHIVE_CONTACT`, `UPDATE_DEAL_OWNER`, `UPDATE_DEAL_VALUE`, `UPDATE_DEAL_STATUS`, `UPDATE_DEAL_TITLE`, `UPDATE_DEAL_FIELD`, `ADD_DEAL_NOTE`, `ADJUST_DEAL_SCORE`, `UPDATE_ACCOUNT_FIELD`, `STOP_WORKFLOW` (other workflow).
12. Campaign action: `resend_to_unopened`.
13. Form layout: `SITE_MESSAGE`.

Deferred, deliberately: landing-page builder, WhatsApp flows, win-probability ML, predictive content/sending, connected mailboxes and calendars, saved responses, database sync, geo reports, integrations marketplace, "format data".

## 3. Reference tenant

The ActiveCampaign trial is configured as a reference tenant by script (`openapi/crm/reference/activecampaign/seed.sh`) so that every object shape can be pulled back through its v3 API and diffed field by field against our schemas. Objects seeded: lists, contacts with tags and fields, segments, score rules, one pipeline with stages and deals and tasks, campaigns (one A/B) to a staff-only list, automations (welcome, abandoned signup, hot-lead handoff), forms, site tracking whitelist, conversions and a business goal. SMS and WhatsApp are paywalled on the trial; their adapter contracts come from documentation.

## 4. Mock screens still to add

Tasks (list + on contact/deal), Pipelines settings (several pipelines, stages, roles), Conversions & goals, Score rules editor, List exclusions, Bulk edit, Snippets & assets, Brand kit, Postal addresses, Tracking domain, Notifications, GDPR data requests, Site messages (in form builder), SMS campaign variant of the wizard, multi-trigger in the automation builder, Automation messages view, Forecast / funnel / owner reports.
