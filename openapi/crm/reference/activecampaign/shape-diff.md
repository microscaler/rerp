# Shape diff — reference tenant objects vs `openapi/crm/*`

Working notes from `dump/` (v3 API responses). "Theirs" is the field as the API returns it; "ours" is where
it lands, or what to add. Only differences worth a spec change are listed.

## Contact (`contacts/*.json`) → `crm/contacts.Contact`
| Theirs | Meaning | Ours |
|---|---|---|
| `sms_consent`, `sms_consent_updated_at` | **channel consent at contact level**, not per list | Add `channel_consents[]` {channel, status, at, source} on Contact; list subscriptions stay per list for email. SMS lists (below) still exist but consent is contact-wide. |
| `open_tracking_optout`, `mpp_tracking` | per-contact tracking opt-out; Apple Mail Privacy Protection flag on opens | Add `tracking_optout` on Contact; add `MessageEvent.machine_open: bool` (MPP / proxy opens) so open rates can exclude them |
| `best_send_hour` | predictive send | defer |
| `bounced_hard`, `bounced_soft`, `sentcnt` | counters on the contact | Add read-only `message_stats` {sent, hard_bounces, soft_bounces, last_open_at, last_click_at} on Contact |
| `scoreValues[]` | one value per named score | Supports the rule-based multi-score model (`crm/intelligence` additions) |
| `accountContacts[]`, `organization`, `orgid`, `orgname` | contact ↔ account link with job title | Marketing-side **company** grouping is needed (decision in feature map §1); add `company_id`/`company_name` on Contact and a light `companies` resource in `crm/contacts` distinct from converted `crm/accounts` |
| `anonymized`, `deleted_at` | GDPR state | Add `anonymized_at` on Contact; GDPR request jobs in `crm/platform` |
| `email_domain`, `email_local`, `hash` | derived | ours derives; `hash` = public token for links, we mint per send instead |
| `whatsapp_id`, `whatsapp_username` | channel identity | later, with WhatsApp channel |

## List (`lists/*.json`) → `crm/outreach.MailingList`
| Theirs | Ours |
|---|---|
| `channel` = email \| sms | **Lists are per channel.** Add `channel` to MailingList (default EMAIL). |
| `optinoptout` (double opt-in on), `optinmessageid`, `optoutconf` (confirm on unsubscribe) | `double_opt_in`, `confirmation_template_id` exist; add `unsubscribe_confirmation: bool` |
| `get_unsubscribe_reason` | ask why on unsubscribe | Add `ask_unsubscribe_reason: bool`; `UnsubscribeRequest.reason` exists |
| `sender_name`, `sender_addr1/2/city/state/zip/country`, `sender_url`, `sender_reminder` | CAN-SPAM postal address and "why you got this" line, **per list** | Add `postal_address_id` and `reminder_text` to MailingList; `PostalAddress` entity in `crm/outreach` |
| `require_name`, `p_use_tracking`, `p_use_captcha` | list-level defaults | `track_*` exist on campaign; leave |
| `subscription_notify`, `unsubscription_notify` | staff emails notified | → `NotificationRule` in `crm/platform` |
| `active_subscribers`, `non_deleted_subscribers` | counts | `subscriber_count`, `unsubscribed_count` exist |

## Campaign (`campaigns/*.json`) → `crm/outreach.Campaign`
| Theirs | Ours |
|---|---|
| `type` = single \| split \| recurring \| responder \| reminder \| activerss | Add `RECURRING` (schedule, e.g. `recurring: day1`), `RESPONDER` (send N after subscribe — ours is an automation; keep as automation), `REMINDER` (date-field campaign, e.g. renewal) to `CampaignKind`; RSS exists |
| `split_type` even \| ..., `split_offset` + `split_offset_type`, `split_content`, `split_winner_messageid` | `ABTestConfig` covers; add `split_content: bool` (body vs subject test) |
| `tracklinks` = all \| mime \| html \| text \| none; `trackreads` | ours booleans; fine |
| `embed_images`, `public` (web archive), `mail_cleanup` | add `public_archive: bool`; others no |
| `segmentid` | campaign audience segment | `segment` exists |
| `survey`, `unsubreasons`, `replies`, `forwards`, `socialshares` | engagement counters incl. replies/forwards/shares | `MESSAGE_FORWARDED`/`SHARED` events + report counters |
| `predictive_send`, `has_predictive_content` | ML send-time and content | defer |
| `messages[]` (campaign → message 1..n) | **content is a separate `message` object** (fromname, fromemail, reply2, subject, preheader, html, text, priority, language) reused across campaigns and automations | Ours: `Template` plays this role; keep. Add `priority` and `language_code` to Template. |
| `reminder_field`, `reminder_offset*`, `reminder_type` | date-field driven send | covered by REMINDER kind |

## Automation (`automations/*.json`) → `crm/automation.Workflow`
| Theirs | Ours |
|---|---|
| `multientry` | ours `reentry` enum; fine |
| `exit_on_unsubscribe`, `exit_on_conversion` | **add** `stop_on_unsubscribe: bool`, `stop_on_conversion_id` |
| `dedup_configuration` | dedupe contacts entering | covered by reentry |
| `entered`, `exited` | counters | `active_enrolments`, `completed_enrolments` exist; add `entered_total` |
| `screenshot` | graph thumbnail | nice-to-have, skip |
| triggers are a sub-resource (not exposed at `/automationTriggers`) | several per automation | `triggers[]` on Workflow (feature map §2.1) |

## Score (`scores/*.json`)
`reltype` = contact \| deal, `status`, rules via UI only. → `crm/intelligence`: `Score {id, name, entity: CONTACT|DEAL, status}` + `ScoreRule {score_id, conditions, points, decay_days}`; contact carries `score_values[]`.

## Segment (`segments/*.json`)
`logic` and \| or, `seriesid` (segment owned by an automation trigger), `hidden`. → ours: `ConditionGroup.op`; add `owner_workflow_id` + `hidden` so trigger segments do not clutter the list.

## Conversion (`conversions/*.json`)
`name`, `value`, `value_option` (fixed \| from field), `currency`, `limit` + `enforcelimit` (count once per contact), `field_data`. → `crm/marketing.Conversion {name, value_mode: FIXED|FIELD, value, value_field, currency, once_per_contact}`; `CONVERT` action; `Workflow.stop_on_conversion_id`.

## Deal / pipeline (`deals`, `dealGroups`, `dealStages`, `dealTasktypes`, `dealRoles`)
| Theirs | Ours |
|---|---|
| `dealGroup` (pipeline): `currency`, `allgroups/allusers` (visibility), `autoassign`, `win_probability_initialize_date` | **add `Pipeline`** in `crm/pipeline`: name, currency, visibility, auto-assign; stages carry `pipeline_id` |
| `dealStage`: `order`, `color`, `width`, `dealOrder` (card sort), `cardRegion1..5` (card layout) | ours has sequence/color/probability; add `card_fields[]` (what the kanban card shows) |
| `deal.status` 0 open / 1 won / 2 lost; `winProbability`; `nextdate` (next task due); `nextdealid`; `account`/`customerAccount`; `fields[]` | ours `won_status`; add `next_task_at`, `pipeline_id`, `owner_id`, `company_id`; custom fields via platform |
| `dealTasktypes` with `outcomes[]`, `defduration` | **Task types with outcomes** → `Task`, `TaskType {title, default_minutes, outcomes[]}` |
| `dealRoles` | **add `DealRole`** and `deal.contacts[] {contact_id, role_id}` (several contacts per deal) |

## Message (`messages/*.json`) and template (`templates/*.json`)
Message = sendable content with sender; template = starting layout. Ours merges both into versioned `Template` with `kind`; keep, add `priority`, `language_code`, `reply_to` override.

## Not reachable on the trial / by API
Forms, site tracking domains, goals, addresses, brandings details, saved responses, calendars: created in the UI; dumped when present.

## Automation builder (UI only; nothing of this is in the v3 API) → `crm/automation`

Walked 2026-11-05 in `/app/builder/1`. Each row is a dialog as the product shows it; "ours" is where it landed in the
spec that day.

### Workflow-level settings (side panel → Settings)
| Theirs | Ours |
|---|---|
| Description | `Workflow.description` |
| "Track the progress": add to Business Goal / add Conversion | `Workflow.tracking {goal_id, conversion_id}` |
| "A contact can enter": only once / any number of times / only after they have exited | `ReentryRule` ONCE / CONCURRENT / MULTIPLE (+ our RESTART) |
| "Contacts exit when": reach the end (always), unsubscribe from any email in the automation, any tracked conversion occurs | `Workflow.exit_rules {on_unsubscribe, on_conversion, on_goal_reached}` |
| "If a contact unsubscribes from an email": the lists associated to a trigger or action / all lists | `Workflow.unsubscribe_scope` TRIGGER_LISTS / ALL_LISTS (+ NONE) |
| "User groups who can access": admin only / groups | `Workflow.access_group_ids[]` |
| Side panel tabs: Activity logs (per action, AI summaries), Messages (every message in this automation) | activity = enrolment step records; messages = `/templates?kind=AUTOMATION&workflow_id=` |
| "Add contacts to this automation": specific contacts (test/replay) or a segment (bulk) | `POST /workflows/{id}/enrol {contact_ids[]|segment_id}` exists |

### Start triggers
| Theirs | Ours |
|---|---|
| Several triggers, "Add another start trigger" | `Workflow.triggers[]` |
| Per trigger: Runs once / multiple times | `WorkflowTrigger.runs` |
| Per trigger, Advanced: "Segment the contacts entering this automation" (condition tree) | `WorkflowTrigger.segment` |
| Subscribes to a list: Any list / a list | `config.list_id` absent = any |

### Step dialogs
| Step | Options | Ours |
|---|---|---|
| Wait | (1) a set period: N minutes/hours/days/weeks; (2) until a specific day and/or time: any-or-given month, day, year, time, "in the contact's time zone" or account's; (3) until a custom date field matches: field, on/before/after the field's date, and "if the date has already passed: continue / exit"; (4) until specific conditions are met: condition tree, "from a saved segment", optional "continue if not met within a period" | `WaitConfig.mode` DURATION / UNTIL_DATETIME (`until` or `until_pattern`) / UNTIL_DATE_FIELD (`relation`, `if_date_passed`) / UNTIL_CONDITION (`timeout_seconds`) |
| If/Else | the same condition tree; yes/no branches | `conditions`, `yes_step_id`, `no_step_id` |
| Split | Split paths: evenly across N paths, or by percentage. Split test: until a date or until N contacts split, N paths, winner conditions. Split until: everyone to path A until a date or N contacts, then B | `SplitConfig.mode` EVEN / PERCENT / TEST / UNTIL |
| Go to | pick any action in the automation; adding it **removes every action below it** (terminal for the branch) | `StepKind.GOTO`, `goto_step_id`, no `next_step_id` |
| Jump To (formerly Goal) | name; conditions; "when this action is below the contact's position / anywhere"; "if the contact does not meet the conditions: continue anyway / wait until met"; add tracking | `StepKind.GOAL` + `GoalConfig {scope, if_not_met, track}` |
| Start / End another automation | pick automation | `START_WORKFLOW` / `STOP_WORKFLOW {workflow_id}` |
| Webhook | URL only (payload is the contact) | `CALL_WEBHOOK {url}` |
| Format data | copy field values; add/remove time from a date field; extract phone numbers or URLs; perform math; capitalize a field | `FORMAT_DATA {op ...}` |
| AI Actions Library | AI agents as steps | deferred |
| Send an email | create new (scratch/template) / start from an existing email (past campaign or this automation) / save and design later | `SEND_MESSAGE {template_id}`; "design later" = a step with no template (graph validation TEMPLATE_KIND warns, workflow cannot activate) |
| Send a direct 1:1 email | plain text through a connected account | `SEND_1TO1_EMAIL` (v1: sender identity + user reply-to) |
| Send a transactional email | needs a Postmark connection; message settings | `SEND_TRANSACTIONAL` on our own transactional templates |
| Send an internal notification email | subject (merge fields), send to (up to 5 addresses), from (a verified sender), include "a link to the contact and contact's field values", rich-text body, preview | `INTERNAL_NOTIFICATION {subject, to[], from_sender_id, include_contact_link, body}` |
| Start a WhatsApp flow / SMS | paywalled on the trial | deferred / SMS via outreach channel |
| Subscribe / Unsubscribe | one or more lists | `{list_ids[]}` |
| Convert | pick a conversion (or create one inline) | `CONVERT {conversion_id}` |
| Update a contact | field values | `UPDATE_FIELD` |
| Add / remove a tag, add a note | — | exist |
| Add a task (new) | title, description, task type, assign to (account owner / user), due after N day(s) | `CREATE_TASK` |
| Adjust a contact score | needs a contact score to exist ("add a score" inline) | `ADJUST_SCORE {score_id, points}` |
| Archive a contact | — | `ARCHIVE_CONTACT` |
| CRM: add a deal; update title / status / owner / value / stage / custom field; add a deal task; add a deal note; complete a deal task; adjust a deal score; update an account field; add an account note; update an account owner; add contact to account | `CREATE_DEAL`, `UPDATE_DEAL_*`, `CREATE_TASK {deal: true}`, `ADD_DEAL_NOTE`, `COMPLETE_TASK`, `ADJUST_DEAL_SCORE`, `UPDATE_ACCOUNT_FIELD`, `ADD_ACCOUNT_NOTE`, `UPDATE_ACCOUNT_OWNER`, `ADD_TO_ACCOUNT` |

Condition tree vocabulary on the trial (wait / if-else / jump-to / split-test / trigger segment): contact details
(email, email domain, first name, last name), contact list status (subscribed, not subscribed, unsubscribed from a
list), custom contact fields, actions (submitted form), predictive geography / attribution (paywalled). Operators
shown: Is (+ the usual negations and comparisons per field type). Condition groups AND/OR, and "add conditions from a
saved segment". Our `ConditionGroup` (crm/contacts) is a superset: tags, site visits, engagement, deals, scores.

Canvas: a note per step (the "1" badge), per-step stats on email steps (sent / open rate / click rate), draft-campaign
warnings beside email steps, minimap, zoom, export image; a graph-level validity (Active cannot be switched on with an
undesigned email).

## Campaign editor (UI) → `crm/outreach.Campaign`

Walked 2026-11-05 (`/app/campaigns/new`, campaigns 11 "Reference: date-based renewal reminder" and 13 "Reference:
split test" left as drafts for the dump).

| Theirs | Ours |
|---|---|
| Type picker: Standard (from scratch / template in the designer), Split Testing (test subjects and/or from information; or subjects, from and email content), Automated (hands off to the builder), RSS Triggered, Auto Responder (send after subscribe), Date Based (birthdays, anniversaries, renewals) | `CampaignKind` SINGLE / AB_TEST (`split_content`) / RSS / REMINDER; Automated and Auto Responder are workflows (`WorkflowRecipe`), not campaigns |
| One page, not a wizard: message (subject, preheader, from name, from email, "use as reply-to"), design (create with email designer / existing), recipients, delivery, tracking, address, archive; header has Save and exit / Test and preview / Send now | Our wizard steps map one to one; `Campaign.reply_to` defaults to from |
| Recipients: subscribed to list(s) (multi-select, "Select All / None") → send to: all active subscribers of the selected lists, a segment, a tag, or "send using custom conditions"; "Calculate recipients" count; "Translate to a contact's language" (paywalled) | `Campaign.list_ids[]`, `audience {mode: ALL\|SEGMENT\|TAG\|CONDITIONS, segment_id, tag, conditions}`; `POST /campaigns/{id}/audience` returns the count |
| Date Based delivery: sends on / before / after the contact's `<date field>` (any date or datetime field incl. Subscription Date), yearly / once only | `Campaign.reminder {date_field, relation ON\|BEFORE\|AFTER, offset_days, repeat YEARLY\|ONCE}` |
| Split test settings: do not determine a winner / determine and send using the winner by best click rate or best open rate after N hours/days/weeks/months; email ratios per variant; at least 2 variants (tabs "Test 1", "+") | `ABTestConfig {variants[], winner_metric CLICKS\|OPENS, decide_after_seconds, auto_send_winner, ratios[]}` |
| Schedule (optional): Predictive Send (each contact's optimal time within 24 h) or an exact date + time in the workspace timezone | `schedule {mode NOW\|AT\|PREDICTIVE, at, timezone}`; PREDICTIVE deferred |
| Tracking: open/read tracking (+ "Automate": automations to start when opened), reply tracking, link tracking (+ "Customize": per link enable/disable, automations to start on click, a name for reports), Google Analytics (+ UTM campaign name) | `track_opens`, `track_clicks`, `track_replies`, `links[] {url, tracked, report_name, start_workflow_ids[]}`, `on_open_workflow_ids[]`, `utm {campaign}` |
| Physical mailing address (required, per list; CAN-SPAM) | `postal_address_id` (per list, overridable per campaign) |
| Campaign archive: public / private | `public_archive: bool` |
| Pre-send checklist ("the following areas need to be updated before sending": list, address, subject, design, A/B needs 2 variants) | `GET /campaigns/{id}/readiness` → `[{code, message}]`; `schedule`/`send` refuse while non-empty |
| AI helpers: "how have similar campaigns performed", "how does my subject line work", "is my email ready to send" | deferred |

## Form builder (UI) → `crm/marketing.Form`

Walked 2026-11-05; form 1 "Reference: pricing enquiry" (inline, subscribes to Newsletter) left for the dump.

| Theirs | Ours |
|---|---|
| Create: name; style inline / floating bar / floating box / modal; one or more form actions: subscribe to a list, subscribe to an SMS list, add a tag, add to a deal (pipeline + stage + owner), email results (to addresses); AI "generate a form in seconds" | `Form.layout` INLINE / BAR / BOX / MODAL (+ SITE_MESSAGE); `Form.actions[] {kind: SUBSCRIBE_LIST\|SUBSCRIBE_SMS_LIST\|ADD_TAG\|CREATE_DEAL\|EMAIL_RESULTS, ...}` replaces a single `list_id` |
| Fields panel: standard (header, HTML and text, full name, first name, last name, email, CAPTCHA, phone, image, subscribe/unsubscribe, list selector), custom contact fields (searchable, "+" creates one), account fields | `Form.fields[] {kind: HEADER\|HTML\|FULL_NAME\|FIRST_NAME\|LAST_NAME\|EMAIL\|CAPTCHA\|PHONE\|IMAGE\|SUBSCRIPTION_TOGGLE\|LIST_SELECTOR\|CUSTOM\|ACCOUNT_FIELD, field_id?, label, placeholder, required, options}` |
| Style: layout vertical / horizontal; max width; padding; form and page background; borders (colour, stroke, style), corners, shadow; per-field and button styles; fonts | `Form.style` object (opaque JSON, rendered by our embed script) |
| Options: on submit → show thank-you text or open URL; form actions (edit/remove/add); accepting submissions (pause); allow blank fields to overwrite existing data; block personal email providers; block email domains (comma list, `*.example.com`) | `Form.on_submit {mode: THANK_YOU\|REDIRECT, text, url}`, `Form.status` ACTIVE / PAUSED, `allow_blank_overwrite`, `block_free_mail`, `blocked_domains[]` |
| Code editor toggle (edit the HTML directly) | `Form.custom_html` (optional; replaces the rendered fields) |
| "Trigger Automation" (start an automation on submit) | `FORM_SUBMITTED` trigger on a workflow; the form page lists the workflows that use it |
| Integrate: simple embed (`<div class="_form_1">` + script), full embed (inline CSS + HTML), link (hosted page), Facebook, WordPress; suggested follow-up automations | `GET /forms/{id}/embed` → `{script, html, hosted_url}`; public `POST /public/forms/{id}/submit` |
| Toast "Your form is saved"; forms auto-save | — |

## Site tracking settings (UI) → `crm/marketing` tracking

Walked 2026-11-05 (`/app/settings/tracking`); tracking enabled, `pricewhisperer.ai` (all pages) whitelisted, event
tracking enabled with event `trial_activated`, for the dump (`siteTrackingDomains`).

| Theirs | Ours |
|---|---|
| Site tracking on/off (workspace-wide); "has not been used recently" warning | `GET/PUT /tracking/settings {site_tracking: bool, event_tracking: bool}`; `TrackedSite.last_event_at` drives the warning |
| Whitelist: website URL + "include all website pages" (unchecked = that page/subdomain only; `*` wildcard); per-row site-tracking checkbox; remove | `TrackedSite.origin`, `include_all_pages`, `enabled` |
| Tracking code: one snippet for the account (`setAccount`, `setTrackByDefault true`, `process`) | `GET /tracking/snippet`; one `tracking_key` per workspace |
| Event tracking on/off; event key (regenerate); whitelisted events (add / remove) | `event_tracking` flag, `event_key` (regenerable), `EventTypeDef.declared/enabled` |

## Recipes (the automation library) → `crm/automation` recipes

Walked 2026-11-05: library of ~13 industry categories (Software has 20), each recipe a card with description, a
"Read more" dialog and "Use recipe", plus "Import recipe" from a URL. Importing creates an inactive automation with
placeholders and opens a **Setup Wizard** ("Step 1/1 Select list"), or "Skip setup". Unbound references render as
"Related object is unavailable, please make a selection". Every step carries recipe **notes** (badge count); email
steps show "This campaign is currently in draft" until designed and "An address is needed in order to send this
email" when the sender has no postal address; the automation cannot be set Active until both are resolved.

Imported for the dump (automations 4-7): Free Trial Email Automation Workflow, Expired Trial Email Automation
Workflow, Upgrade Incentive and Outreach, Re-engagement Campaign Triggered by "Disengaged" Tag. Their graphs are
transcribed in `automation/recipes/*.yaml`; the behaviour they imply (goto as join, if/else before each message,
wait-until with timeout then if/else, tag chaining between automations, conditions evaluated on arrival) is in
`automation/features/*.feature`.

| Theirs | Ours |
|---|---|
| Recipe library by industry; import from URL; AI "generate an automation" | `GET /recipes` (`industry`), `POST /recipes/import`; AI deferred |
| Placeholders per recipe (list, tag, page URL, message), setup wizard, skip setup | `RecipeParameter`, `instantiate.bindings`, `GET /workflows/{id}/setup`, `PATCH /workflows/{id}/bindings`, `Workflow.unbound_parameters`, validation `UNBOUND_PARAMETER` |
| Notes on every step and on the trigger | `WorkflowStep.notes[]`, `Workflow.notes[]`, `POST .../notes` |
| Draft-email and missing-address warnings; Active refused | validation `TEMPLATE_NOT_READY`, `NO_POSTAL_ADDRESS`; activate → 422 |
| Go to draws a dashed edge to its target; No branch on the right | `GOTO` step; editor hint only |

### Trigger settings forms (one hidden dialog per type, read from the builder page)
list: any/one · form: any/one · pipeline/stage: pipeline, from stage, to stage · deal status changes to open/won/lost ·
owner: any / specific (equal / not equal) · tag · field changes: ever / to value · jump-to goal: achieved / skipped ·
date field (subscription date, creation date, custom dates, deal created/updated/forecast close) + format + timezone
(contact's / account) + N days/weeks/months/years on/before/after + daily/weekdays/weekends + hour · RSS feed +
timezone + hourly/daily/weekdays/weekends + time · score + threshold above/below · email any/one (+ link any/one,
list filter) · 1:1 email any / automated / direct · social network. → `WorkflowTrigger.config` per type.

## Deals menu (UI walk, 2026-11-05) → `crm/pipeline`, new `crm/tasks`, `crm/reporting`, `crm/platform`

Walked every entry of the Deals menu (Deals, Tasks, Saved Responses; Manage: Fields, Deal Settings, Connected
Emails, Task Settings, Calendar, Import/Export Status; Reports: Deal Overview, Deal Owner Overview, Funnel, Sales
Performance, Task Overview, Deal Forecast). Objects created on the tenant to pull shapes back: task outcomes
(Reached/positive, No answer/neutral, Not interested/negative), saved response "Trial follow-up", calendar feed
"Sales tasks".

| Screen | What it holds | Ours |
|---|---|---|
| Deals (kanban / list) | per-pipeline board, stage columns with count + value, card fields from stage settings, filters (owner, status, stage, value, custom fields), sort by `dealOrder`, bulk actions (owner, stage, status, tag, delete), "Add deal" (title, contact, account, value, currency, pipeline, stage, owner, custom fields) | `GET /pipelines`, `GET /deals?pipeline_id&stage_id&status&owner_id&q`, `POST /deals/bulk`, `Stage.card_fields`, `Stage.deal_order` |
| Deal detail | header (title, value, stage pill, status Won/Lost buttons), AI prompts, About panel (pipeline, stage, status, owner, win probability, expected close, custom field groups), tabs Notes / Files / Emails, open tasks, contacts with roles + primary star, recent activity, "Send an email" (connected mailbox) | `Deal` (+`contacts[] {contact_id, role_id, is_primary}`, `win_probability`, `expected_close_date`), `/deals/{id}/notes|files|emails|activity|contacts`, `PATCH /deals/{id}/stage|status` |
| Tasks | list with tabs (Mine / All / Overdue / Today / Upcoming / Completed), columns type, title, related contact/deal, due, assignee, outcome; "Add task" (type, title, related to, due date + time, assignee, reminder, note, "start automation when added/completed") | `crm/tasks` `Task` (`related_kind`, `reminders[]`, `automation` hook ADDED/COMPLETED), `/tasks/{id}/complete` with `outcome_id`, `/tasks/{id}/reopen` |
| Saved Responses | categorised templates with merge tags, used from task emails and 1:1 sends | `SavedResponse` (+`category_id`), `/saved-responses/{id}/render` |
| Manage → Fields | deal custom fields in groups: text, textarea, number, money (currency), date, datetime, dropdown, listbox (multi), required, default, visible on card | `crm/platform` custom fields entity=deal (`FieldType` +MONEY, MULTISELECT, TEXTAREA; `group`, `sequence`, `show_on_card`, `required`, `default_value`) |
| Manage → Deal Settings | pipelines (name, currency, visibility all users / groups, auto-assign round-robin), stages (name, colour, width, card region 1-5, deal sort), deal roles, win probability (ML on/off, initialised date), deal value brackets for reports | `Pipeline`, `Stage`, `DealRole`, `DealSettings {win_probability_mode, value_brackets[], default_pipeline_id}` |
| Manage → Connected Emails | per-user mailbox connect (Google / Microsoft OAuth, IMAP), sync since date, only known contacts vs all, log to deals, open/click tracking, signature, share with team | `crm/platform` `Mailbox`, `/mailboxes/oauth/callback`, `/mailboxes/{id}/sync|send|messages` |
| Manage → Task Settings | task types (ordered, default duration, allowed outcomes), outcomes (label + sentiment), "require outcome when completing" | `TaskType.outcome_ids`, `Outcome.sentiment`, `TaskSettings.require_outcomes`, `/types/reorder` |
| Manage → Calendar | per-user iCal feed URLs: all tasks / deal tasks / contact tasks, filter by pipeline and assignee, include alerts, regenerate URL | `CalendarFeed {scope, pipeline_ids, assignee_ids, include_alerts, url}`, `/calendars/{id}/regenerate`, `GET /public/calendars/{token}.ics` |
| Manage → Import/Export Status | list of import/export runs: kind, entity, file, status, rows total/processed/errors, started/finished, download result / error report, cancel | `crm/platform` `Job`, `/jobs`, `/jobs/{id}/errors`, `/jobs/export` |
| Reports → Deal Overview | totals (open/won/lost count + value, win rate), deal value histogram by bracket, owner table, stage table, deals created over time; filters pipeline / owner / date / currency | `GET /deals/overview` → `DealOverviewReport` |
| Reports → Deal Owner Overview | one row per owner: created, won, lost, value won, avg value, avg days to close, tasks completed | `GET /deals/owner-overview` |
| Reports → Funnel | per pipeline: deals entered each stage, % progressing to the next, avg days in stage, lost from stage | `GET /deals/funnel?pipeline_id` → `FunnelStage.progression_pct` |
| Reports → Sales Performance | matrix of deals by stage per owner, won/lost per owner, period selector | `GET /deals/sales-performance` |
| Reports → Task Overview | open / overdue / completed per rep and per type, outcome sentiment per rep | `GET /tasks/overview` → `TaskOverviewReport` |
| Reports → Deal Forecast | expected revenue per month from expected close dates weighted by stage probability (or ML win probability), quota line, forecast vs actual accuracy for past periods | `GET /deals/forecast?basis&granularity&periods` → `DealForecastReport.accuracy[]` |

Behaviour worth keeping: a deal's **status** (Won/Lost) is separate from its **stage**; winning does not move the
stage. **Win probability** is either the stage's probability or an ML estimate per deal (we expose `basis`).
**Tasks** complete with an outcome whose sentiment rolls up to the Task Overview; completion can start an automation.
**Calendar** feeds are signed public URLs, regenerable. **Connected emails** are per user and only log mail to
known contacts unless widened. Import/export jobs are async and keep per-row errors.
