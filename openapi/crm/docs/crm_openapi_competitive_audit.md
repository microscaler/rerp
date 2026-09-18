# RERP CRM OpenAPI Competitive Audit

> **Date:** 2026-05-10  
> **Sources:** RERP OpenAPI specs (gateway + 3 sub-services), Odoo Community/Enterprise CRM modules, vendor live pages (Salesforce, HubSpot, Odoo, SAP, Microsoft)  
> **Scope:** Functional gap analysis, world-class CRM component definition, implementation priorities

---

## Executive Summary

RERP CRM has the right architecture (OpenAPI-first, Rust-based, 3-service split: core, automation, livechat) but its specs define endpoints with **completely empty schemas**. Odoo CRM, by contrast, ships with ~15+ interconnected modules covering leads, teams, scoring, livechat, helpdesk, marketing, subscriptions, AI, and analytics — with concrete field definitions and business logic.

The gap is not architectural — it's schema depth. RERP needs complete entity definitions with fields, relationships, validation, and business logic.

---

## 1. RERP CRM Current State (OpenAPI Audit)

### Service Architecture

| Service | Path | Resources | Schema Status |
|---------|------|-----------|---------------|
| Gateway | `/api/v1/crm/*` | Aggregates all sub-services | References sub-services |
| Core | `/api/v1/crm/core/*` | Leads, Contacts, Opportunities | **Empty** `schemas: {}` |
| Automation | `/api/v1/crm/automation/*` | Workflows, Rules, Triggers | **Empty** `schemas: {}` |
| Livechat | `/api/v1/crm/livechat/*` | Chats, Messages, Agents | **Empty** `schemas: {}` |

### Endpoints Defined (by service)

**Core (6 endpoints):**
- `GET /leads`, `POST /leads`, `GET /leads/{id}`, `PUT /leads/{id}`, `DELETE /leads/{id}`
- `GET /contacts`, `POST /contacts`, `GET /contacts/{id}`, `PUT /contacts/{id}`, `DELETE /contacts/{id}`
- `GET /opportunitys`, `POST /opportunitys`, `GET /opportunitys/{id}`, `PUT /opportunitys/{id}`, `DELETE /opportunitys/{id}`
- **Note:** "opportunitys" (not "opportunities") — naming issue

**Automation (9 endpoints):**
- `GET/POST/GET/PUT/DELETE /workflows/{id}`
- `GET/POST/GET/PUT/DELETE /rules/{id}`
- `GET/POST/GET/PUT/DELETE /triggers/{id}`
- **Note:** No `GET /workflows`, `GET /rules`, `GET /triggers` list endpoints defined — only single-resource CRUD

**Livechat (9 endpoints):**
- `GET/POST/GET/PUT/DELETE /chats/{id}`
- `GET/POST/GET/PUT/DELETE /messages/{id}`
- `GET/POST/GET/PUT/DELETE /agents/{id}`
- **Note:** No list endpoints for agents (only single-resource CRUD)

### Total: 24 endpoint definitions, zero schema definitions

---

## 2. Odoo CRM Module Structure (Live Codebase Analysis)

### Community Modules (17 modules)

| Module | Description |
|--------|-------------|
| `crm` | Core: leads, opportunities, teams, stages, scoring, UTM, recurring plans, activities |
| `crm_iap_enrich` | Lead enrichment via IAP (Intelligent Automation Platform) |
| `crm_iap_mine` | Lead mining/discovery from external data sources |
| `crm_livechat` | Livechat widget with agent assignment |
| `crm_mail_plugin` | Email integration (compose, send, track from CRM) |
| `crm_sms` | SMS integration for leads |
| `event_crm` | Event registration → lead conversion |
| `event_crm_sale` | Event → opportunity conversion |
| `gamification_sale_crm` | Goals, badges, leaderboards |
| `iap_crm` | In-App Purchase CRM integration |
| `mass_mailing_crm` | Mass mailing → lead tracking |
| `mass_mailing_crm_sms` | SMS mass mailing |
| `sale_crm` | Sales pipeline bridge (quotas → opportunities) |
| `survey_crm` | Survey responses → leads |
| `website_crm` | Website contact forms → leads |
| `website_crm_iap_reveal` | Anonymous visitor → lead conversion |
| `website_crm_livechat` | Livechat → lead conversion on website |
| `website_crm_partner_assign` | Partner/reseller lead assignment |
| `website_crm_sms` | SMS on website → lead capture |
| `website_event_crm` | Event registration on website |
| `test_crm_full` | Test fixtures |

### Enterprise Modules (14 modules)

| Module | Description |
|--------|-------------|
| `crm_enterprise` | Advanced views: map view, card scan, business card import |
| `crm_enterprise_partner_assign` | Enterprise partner/reseller assignment |
| `crm_helpdesk` | Lead ↔ ticket conversion (CRM ↔ Helpdesk) |
| `crm_sale_subscription` | CRM → subscription bridge |
| `marketing_automation_crm` | Marketing automation → CRM leads |
| `social_crm` | Social media → lead integration |
| `voip_crm` | VoIP/phone integration |
| `ai_crm` | AI-powered automatic lead creation |
| `ai_crm_livechat` | AI agents in livechat that auto-create leads |
| `appointment_crm` | Appointment scheduling → leads |
| `website_appointment_crm` | Website appointment booking |
| `website_crm_iap_reveal_enterprise` | Enterprise visitor identification |
| `data_merge_crm` | Advanced lead deduplication/merging |
| `spreadsheet_dashboard_crm` | Spreadsheet-based CRM dashboards |

### Core CRM Model: `crm.lead` (200+ fields)

The base `crm.lead` model inherits 7 mixins for comprehensive tracking:

```python
_mail.thread.cc
_mail.thread.blacklist
_mail.thread.phone
_mail.activity.mixin
_utm.mixin
_format.address.mixin
_mail.tracking.duration.mixin
```

Key field categories:
- **Identity:** name, email_from, phone, mobile, title, function, company_name
- **Address:** street, street2, city, state_id, zip, country_id
- **Sales:** user_id, team_id, expected_revenue, probability, priority (1-3), date_deadline
- **Pipeline:** stage_id, stage_name, stage_percent, stage_kanban, stage_sequence
- **Financial:** expected_revenue, recurring_plan, recurring_revenue
- **Tracking:** campaign_id, medium_id, source_id, tag_ids
- **Communication:** email_cc, email_bounce, email_normalized
- **AI/ML:** automated_probability, scoring, lead_properties (polymorphic properties)
- **Relationships:** partner_id (linked contact), company_id (linked account)
- **Metadata:** create_date, create_uid, write_date, write_uid, company_id, color

### Core CRM Models (all models)

| Model | Description |
|-------|-------------|
| `crm.lead` | Lead/Opportunity (unified entity) |
| `crm.team` | Sales team with alias, assignment rules, quotas |
| `crm.team.member` | Team members with user_id, assignment_domain, load |
| `crm.stage` | Pipeline stages with sequence, probability, type (won/lost/open) |
| `crm.lead.scoring.frequency` | Bayesian scoring frequency tables |
| `crm.recurring.plan` | Recurring revenue plan (name, months, sequence) |
| `crm.lead.lost.reason` | Lost reason codes |
| `crm.lead.lost` | Lost reason wizard |
| `mail.activity` | Activity planning (follow-ups, meetings) |
| `digest.digest` | KPI digest emails |

---

## 3. World-Class CRM Component Definitions

Based on the competitive analysis, a world-class CRM consists of **12 components**:

### Component 1: Lead & Contact Management (Foundation)

**What it covers:** Lead creation, contact management, account/company model, lead-to-contact conversion, deduplication, enrichment, GDPR compliance, custom fields, multi-company support.

**Odoo's implementation:**
- Unified `crm.lead` model handles both leads and opportunities
- Lead-to-opportunity wizard: `crm_lead_to_opportunity`
- Merge wizard: `crm_merge_opportunities`
- Partner sync: `PARTNER_FIELDS_TO_SYNC` — contacts sync with leads
- GDPR: `mail.thread.blacklist` mixin, `mail.thread.phone` for consent
- Custom fields: `lead_properties` (polymorphic Properties field)
- UTM tracking: `utm.mixin` (campaign, medium, source)
- Multi-company: `user_company_ids` on team members
- Address formatting: `format.address.mixin`

**RERP's current state:**
- `Lead`, `Contact`, `Opportunity` are separate entities with no relationships
- No account/company model
- No lead-to-opportunity conversion
- No merge/dedup
- No UTM fields
- No GDPR fields
- No custom fields mechanism
- Schemas are completely empty

**Gap:** Critical. This is the foundation of any CRM. Everything else depends on it.

---

### Component 2: Pipeline & Stage Management

**What it covers:** Configurable stages, probabilities, won/lost states, stage history, rotting/slippage alerts, stage requirements, multi-pipeline per team, stage transition rules.

**Odoo's implementation:**
- `crm.stage` model with: sequence, name, probability, type (open/won/lost), fold, requirement fields
- Stage kanban state: `stage_kanban` for drag-and-drop position
- Stage change tracking: `date_last_stage_update` automatically computed
- Stage requirements: `crm_stage` has `required` fields that must be filled before entering
- Stage probability: auto-computed `stage_percent`
- Lost reasons: `crm.lead.lost.reason` with reason codes
- Stage color: visual kanban indicator
- Multi-pipeline: `crm.team` can have different pipelines

**RERP's current state:**
- No Stage entity exists
- No probability on leads/opportunities
- No won/lost semantics
- No stage history
- No rotting detection
- No stage transition validation

**Gap:** Critical. No pipeline = no CRM. This is the core user-facing feature.

---

### Component 3: Opportunity & Revenue Management

**What it covers:** Revenue fields, probability-weighted revenue, recurring revenue, revenue schedules, forecast categories, quota management, multi-currency, discount management, price books.

**Odoo's implementation:**
- `crm.lead.expected_revenue`: monetary field
- `crm.lead.probability`: float 0-100
- `crm.recurring.plan`: linked recurring revenue plans
- `crm.lead.recurring_revenue`: computed monthly recurring
- `crm.lead.prorated_revenue`: expected_revenue × probability / 100
- `crm.lead.date_deadline`: close date
- `crm.lead.pls_automatic_probability`: AI-computed win probability
- `crm.lead.pls_probability`: manual probability override

**RERP's current state:**
- No revenue fields on any entity
- No probability
- No close dates
- No recurring revenue
- No multi-currency

**Gap:** Critical for B2B sales. Revenue tracking is what makes CRM a business tool, not just a contact list.

---

### Component 4: Predictive Analytics

**What it covers:** Bayesian lead scoring, win probability, historical win-rate computation, scoring model retraining, probability tooltips, lead scoring thresholds, anomaly detection.

**Odoo's implementation:**
- `crm.lead.scoring.frequency` model: field, value, won_count, lost_count
- `crm_pls_fields` config parameter: fields to score on (phone_state, email_state, stage, country, source, lang, tags)
- `_cron_update_automated_probabilities`: scheduled job to recompute PLS
- `crm.lead.pls_automatic_probability`: computed probability
- `crm.lead.pls_probability`: manual override
- `crm.lead.scoring_explanation`: shows which fields drive the score

**RERP's current state:**
- No scoring infrastructure
- No frequency tables
- No automated probability
- No prediction models

**Gap:** Differentiator. RERP can do this in Rust (much faster than Odoo's Python). But it doesn't exist yet.

---

### Component 5: Sales Team Management

**What it covers:** Teams, members, territories, auto-assignment, round-robin, load-based assignment, domain-based routing, quotas, role-based access control.

**Odoo's implementation:**
- `crm.team` model with: name, alias_id, use_leads, use_opportunities, assignment_enabled
- `crm.team.member` with: user_id, team_id, assignment_domain, assignment_max (max leads), lead_day_count
- Assignment logic: load-based, domain-based, round-robin
- Lead count tracking per team member per day
- Team alias: email-to-lead via `alias_contact`
- User company support: `user_company_ids`

**RERP's current state:**
- No Team entity
- No TeamMember entity
- No team_id on Leads/Opportunities
- No assignment logic
- No quotas

**Gap:** Critical for any organization with more than 1 salesperson.

---

### Component 6: Marketing Integration

**What it covers:** UTM tracking, web forms, visitor-to-lead conversion, event registration, survey responses, mass mailing, SMS, email marketing, lead nurturing, landing pages.

**Odoo's implementation:**
- `utm.mixin` on crm.lead: campaign_id, medium_id, source_id
- `website_crm`: website forms → leads
- `website_crm_iap_reveal`: anonymous visitor → identified lead
- `event_crm`: event registration → lead
- `survey_crm`: survey response → lead
- `mass_mailing_crm`: mass mailing → lead tracking
- `crm_mail_plugin`: email tracking on CRM communications
- `iap_crm`: lead intelligence integration

**RERP's current state:**
- No UTM fields
- No web form capture
- No visitor tracking
- No event/survey integration
- No mass mailing

**Gap:** Important but not blocking for initial adoption. Can start with API-only form capture.

---

### Component 7: Communication Hub

**What it covers:** Email composition, templates, tracking, SMS, calendar events, meeting scheduling, call logging, activity timeline, follow-up tasks.

**Odoo's implementation:**
- `mail.activity.mixin`: activity planning and tracking
- `mail.thread.cc`: carbon copy support
- `mail.thread.phone`: phone number validation and tracking
- `mail.thread.blacklist`: opt-out management
- `crm_mail_plugin`: email from within CRM
- Calendar events linked to leads via `calendar_event_ids`
- Activity timeline: chronological `mail.message` log per lead

**RERP's current state:**
- Livechat service exists (chats, messages, agents)
- No email integration
- No calendar integration
- No activity timeline
- No follow-up/task system

**Gap:** Important but can be phased. Livechat is partially implemented.

---

### Component 8: Intelligence & Enrichment

**What it covers:** Email-based enrichment, website-based intelligence, company data, lead discovery, role/industry/seniority detection, auto-fill from partial data, data verification.

**Odoo's implementation:**
- `crm_iap_enrich`: enrichment via IAP (Intelligent Automation Platform)
- `crm_iap_mine`: lead mining/discovery
- `website_crm_iap_reveal`: identify anonymous visitors
- Fields populated from enrichment: industry, company_size, website, email, phone, social profiles, tech stack
- `crm.lead.scoring.frequency` also feeds enrichment data for scoring

**RERP's current state:**
- No enrichment integrations
- No auto-fill
- No data verification
- No LinkedIn integration

**Gap:** Nice-to-have for initial launch. Can be added post-launch.

---

### Component 9: Reporting & BI

**What it covers:** Pivot tables, charts, funnel visualization, custom reports, scheduled delivery, dashboard widgets, KPI digests, trend analysis, win/loss analysis, time-to-close, conversion rates.

**Odoo's implementation:**
- `report/crm_opportunity_report`: pivot table report
- `report/crm_activity_report`: activity analysis
- `crm_helper_templates`: dashboard helper templates
- `digest.digest`: scheduled KPI email digests
- `digest_data`: predefined digest metrics (revenue, activities, conversion)
- Static views: xls/pdf report generation

**RERP's current state:**
- No reporting endpoints
- No dashboard
- No pivot/table views
- No KPI digests

**Gap:** Important for management buy-in. Start with pipeline summary and conversion rates.

---

### Component 10: Workflow Automation

**What it covers:** Trigger-based actions, stage-change triggers, field-value triggers, time-based triggers, rule engine, scheduled actions, email triggers, task creation, approval workflows, lead assignment rules, data validation, escalation, webhooks.

**Odoo's implementation:**
- `ir_cron_data`: scheduled cron jobs (scoring recompute, follow-up scheduling)
- `mail.activity`: activity-based triggers
- Stage-change → email trigger: built-in via mail.message on stage transitions
- Lead assignment: built into `crm.team` assignment logic
- Data validation: stage `required` fields
- Workflow: Odoo's general workflow engine (ir_act_server)

**RERP's current state:**
- Workflow, Rule, Trigger endpoints exist but schemas are empty
- No condition syntax defined
- No action definitions
- No stage-change trigger implementation

**Gap:** Important for reducing manual work. Start with stage-change triggers.

---

### Component 11: Customer Engagement

**What it covers:** Livechat widget, agent management, chat-to-lead conversion, chat-to-ticket conversion, chatbot scripts, helpdesk integration, subscription management, renewal tracking, gamification, customer health scoring.

**Odoo's implementation:**
- `crm_livechat`: livechat widget with agent assignment
- `chat` entity: agent_id, contact_id, status, started_at, ended_at, transcript
- `message` entity: chat messages between agent and visitor
- `agent` entity: online status, active sessions, max concurrent
- `crm_helpdesk`: lead ↔ ticket conversion
- `crm_sale_subscription`: CRM → subscription bridge
- `gamification_sale_crm`: goals, badges, leaderboards

**RERP's current state:**
- Livechat is partially implemented (agents, chats, messages exist as endpoints)
- No chat-to-lead conversion
- No helpdesk integration
- No subscriptions
- No gamification
- No customer health scoring

**Gap:** Partially addressed by livechat service. Subscriptions and helpdesk are post-sale features.

---

### Component 12: Platform & Extensibility

**What it covers:** Custom fields, custom objects, REST API, GraphQL, SDK, webhooks, OAuth2/OIDC, rate limiting, sandbox/dev environment, API versioning, data import/export, third-party marketplace, plugin framework.

**Odoo's implementation:**
- Module system: `.py` modules with manifest
- Custom fields: `lead_properties` (polymorphic)
- Webhooks: supported via external modules
- OAuth2: native Odoo authentication
- API: full CRUD on all models via XML-RPC/JSON-RPC
- Custom objects: enterprise module
- Sandbox: developer/enterprise edition environments
- Integration: 500+ connector modules

**RERP's current state:**
- OpenAPI-first (major advantage over Odoo)
- Schema regeneration from OpenAPI specs
- No custom objects
- No webhooks
- No sandbox
- No marketplace
- Rate limiting not defined

**Gap:** OpenAPI-first is a genuine competitive advantage. Webhooks and custom fields should be Phase 2 priorities.

---

## 4. Gap Analysis Summary

### Critical Gaps (must implement for v1.0)

| Gap | RERP | Odoo | Priority |
|-----|------|------|----------|
| Complete entity schemas | Empty | 200+ fields | P0 |
| Stage/Pipeline model | Missing | Full stage model | P0 |
| Lead-Opportunity relationship | Separate entities | Unified model | P0 |
| Revenue/financial fields | None | expected_revenue, probability | P0 |
| Team/Member model | Missing | Full team model | P0 |
| Lead-to-Opportunity conversion | None | Wizard | P0 |

### High Priority Gaps (v1.1-v1.5)

| Gap | RERP | Odoo | Priority |
|-----|------|------|----------|
| UTM tracking | None | campaign/medium/source | P1 |
| Activity timeline | None | mail.message per entity | P1 |
| Email integration | None | mail_plugin | P1 |
| Reporting endpoints | None | pivot/report/digest | P1 |
| Workflow automation | Empty schemas | cron/activity triggers | P1 |
| Webhooks | None | External module | P1 |

### Medium Priority Gaps (v2.0+)

| Gap | RERP | Odoo | Priority |
|-----|------|------|----------|
| Lead scoring | None | Bayesian PLS | P2 |
| Enrichment | None | IAP Enrich/Mine | P2 |
| Web forms | None | website_crm | P2 |
| Subscription management | None | crm_sale_subscription | P2 |
| Gamification | None | gamification_sale_crm | P2 |
| Helpdesk integration | None | crm_helpdesk | P2 |
| Multi-currency | None | Enterprise feature | P2 |

---

## 5. RERP vs Odoo: Head-to-Head

### Where RERP Has Architecture Advantages

1. **OpenAPI-first schemas** — Odoo's model definitions are Python code. RERP's are machine-readable YAML. This enables:
   - Automatic SDK generation for any language
   - Contract testing between services
   - API documentation that never gets stale
   - Client code generation at build time

2. **Rust performance** — The Axum + async I/O stack delivers sub-millisecond latency. Odoo's Python/PostgreSQL stack can handle 10K+ requests/sec but at higher latency per request.

3. **Independent service deployment** — RERP's 3-service split (core, automation, livechat) allows independent scaling. Odoo is a monolith (even with modular architecture, it's one process).

4. **Self-hosted, zero per-seat cost** — Unlike Odoo Enterprise, no per-user licensing.

### Where Odoo Has Feature Advantages

1. **Complete feature set** — 31 modules covering every CRM aspect. RERP has 24 endpoints with empty schemas.

2. **Rich field definitions** — 200+ fields on crm.lead with business logic. RERP has zero field definitions.

3. **Mature business logic** — Lead scoring, team assignment, activity planning, UTM tracking all implemented. RERP has none.

4. **Ecosystem** — 500+ Odoo modules across ERP, accounting, inventory, HR, POS, e-commerce. RERP is CRM-only.

5. **UI/UX** — Odoo has a complete web UI with Kanban views, pivot tables, dashboards. RERP has API endpoints with no UI.

---

## 6. Implementation Priority Matrix

### Phase 1: Foundation (Core Schemas + Pipeline) — Weeks 1-4

**Goal:** Make RERP usable for a small sales team tracking leads through a pipeline.

1. Define complete `Lead` schema with fields matching Odoo's crm.lead basics
   - name, email_from, phone, mobile, title, function, company_name, user_id, team_id
   - expected_revenue, probability, date_deadline, priority
   - campaign_id, medium_id, source_id (UTM)
   - stage_id, tag_ids, partner_id, company_id
   - create_date, write_date, color, description
   
2. Define complete `Contact` schema
   - name, email, phone, mobile, title, function, company_id
   - street, street2, city, state, zip, country
   - lead_id (back-reference), customer_id (flag)
   
3. Define complete `Opportunity` schema
   - Same as Lead but with additional financial fields
   - expected_revenue, probability, prorated_revenue
   - recurring_plan_id, recurring_revenue
   
4. Define `Stage` entity
   - name, sequence, probability, is_won, is_lost, type (open/won/lost)
   - rotting_threshold_days, required_fields (JSON)
   
5. Add `stage_id` to Lead and Opportunity
   - Implement probability auto-assignment by stage
   
6. Fix endpoint naming: `opportunitys` → `opportunities`

### Phase 2: Team & Automation — Weeks 5-8

**Goal:** Make RERP usable for a sales organization with multiple reps.

1. Define `Team` entity
   - name, description, alias_id, use_leads, use_opportunities
   - assignment_enabled, assignment_method (round_robin/load_based/domain)
   
2. Define `TeamMember` entity
   - team_id, user_id, assignment_domain, assignment_max, lead_day_count
   
3. Implement lead auto-assignment
   - Round-robin: assign to next available member
   - Load-based: assign to member with fewest leads
   - Domain-based: assign by email domain match
   
4. Define Workflow, Rule, Trigger schemas
   - Workflow: name, description, is_active, trigger_type, conditions, actions
   - Rule: condition_field, condition_operator, condition_value, action_type
   - Trigger: type (stage_change, field_change, time_based, webhook)
   
5. Implement stage-change trigger (fire actions when lead moves to stage X)

### Phase 3: Communication & Reporting — Weeks 9-12

**Goal:** Provide sales management with visibility and communication tools.

1. Implement activity timeline
   - List all communications (chats, emails, calls) per entity
   
2. Define reporting endpoints
   - GET /pipeline/summary: stages, counts, revenue by stage
   - GET /analytics/conversion_rates: lead→opportunity→won rates
   - GET /analytics/rep_performance: deals per rep, win rates
   
3. Implement email communication entity
   - from, to, subject, body, status, sent_at, related_lead_id
   
4. Implement KPI digest endpoint
   - Weekly email summary of pipeline, activities, conversion rates

### Phase 4: Intelligence & Advanced Features — Weeks 13-20

**Goal:** Add predictive features that differentiate RERP.

1. Implement lead scoring (Bayesian)
   - Frequency tables for field→outcome correlation
   - Automated probability computation
   - Score explanation (show contributing factors)
   
2. Implement enrichment endpoint
   - POST /leads/{id}/enrich (call Clearbit/ZoomInfo)
   - Batch enrichment for existing leads
   
3. Implement webhook system
   - POST /webhooks (register webhook)
   - Emit webhook events on lead/opportunity changes
   
4. Implement custom fields system
   - POST /entities/{type}/custom-fields
   - CRUD for custom field values

### Phase 5: Customer Success — Weeks 21-28

**Goal:** Extend CRM beyond the sale.

1. Subscription management
   - Subscription entity with renewal tracking
   
2. Helpdesk integration
   - Lead ↔ ticket conversion endpoint
   
3. Gamification
   - Goals, badges, leaderboards

---

## 7. Key Takeaway

RERP CRM has the right foundation — OpenAPI-first architecture, Rust-based performance, and a clean service split. But it's essentially a skeleton with 24 endpoint definitions and zero schemas. 

Odoo ships with a complete, working CRM with 31 modules and 200+ field definitions. The gap is in schema depth and business logic implementation, not architecture.

**The immediate priority is Phase 1:** define complete schemas for Lead, Contact, Opportunity, and Stage. Once these are filled in, RERP becomes a usable CRM for small sales teams. Everything else builds on this foundation.

RERP's OpenAPI-first advantage means that once schemas are defined, every client (web UI, mobile app, API consumers) gets the full data model automatically — something Odoo's Python-model approach cannot match without code generation.
