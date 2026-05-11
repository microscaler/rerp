# CRM Gap Analysis: RERP vs Odoo CRM Ecosystem

> **Date:** 2026-05-10
> **Scope:** High-level component comparison and gap analysis between RERP CRM and the Odoo CRM ecosystem (Community + Enterprise)

---

## Executive Summary

RERP CRM currently has **19 endpoints** organized into 3 sub-services with a flat CRUD model. Odoo CRM has **21 modules** in Community alone, **4 in Enterprise**, **73+ fields** on the core lead model, and a deeply interconnected architecture spanning lead scoring, revenue forecasting, predictive analytics, and multi-channel engagement.

**The verdict:** RERP CRM is in its infancy. The gap is not just feature breadth — it's architectural depth. Odoo models the entire customer lifecycle as a probabilistic system; RERP models it as a simple entity store.

---

## Current State: RERP CRM

### Architecture
- **Gateway spec** (`openapi/crm/openapi.yaml`) — 34 endpoints, 3 sub-services proxied
- **3 sub-services**, each with 6 CRUD endpoints (list/get/create/update/delete)

| Service | Entities | Endpoints |
|---------|----------|-----------|
| core | leads, contacts, opportunities | 18 |
| automation | workflows, rules, triggers | 18 |
| livechat | agents, chats, messages | 18 |

### Data Models (inferred from endpoint refs)
- **Lead** — referenced schema (empty in spec)
- **Contact** — referenced schema (empty in spec)
- **Opportunity** — referenced schema (empty in spec)
- **Workflow** — referenced schema (empty in spec)
- **Rule** — referenced schema (empty in spec)
- **Trigger** — referenced schema (empty in spec)
- **Agent** — referenced schema (empty in spec)
- **Chat** — referenced schema (empty in spec)
- **Message** — referenced schema (empty in spec)

### Status
- **Schema definitions empty** in sub-specs (likely generated but not populated)
- No relationship modeling between entities
- No pipeline/stage semantics
- No scoring, forecasting, or analytics
- No contact enrichment or lead generation
- No email/SMS integration endpoints
- No reporting or BI endpoints

---

## Current State: Odoo CRM Ecosystem

### Architecture — 25 Modules Total

#### Community (21 modules)

| Module | Purpose | Key Models |
|--------|---------|------------|
| **crm** | Core CRM | crm.lead (70+ fields), crm.stage, crm.team, crm.lost.reason, crm.recurring.plan |
| **crm_livechat** | Livechat-to-lead conversion | discuss.channel extension, chatbot.script extension |
| **crm_mail_plugin** | Email integration in CRM | crm.lead extension |
| **crm_sms** | SMS in CRM | (view-level only) |
| **crm_iap_enrich** | Automated lead enrichment | crm.lead extension, reveal model |
| **crm_iap_mine** | Lead generation/mining | crm.iap.lead.mining.request, crm.iap.lead.role, crm.iap.lead.industry |
| **sale_crm** | Opportunity-to-quotation | sale.order extension, crm.lead conversion |
| **website_crm** | Web contact forms | crm.lead, website.visitor extension |
| **website_crm_livechat** | Livechat sessions for leads | crm.lead extension |
| **website_crm_partner_assign** | Reseller/partner assignment | res.partner.activation, res.partner.grade |
| **website_crm_sms** | SMS to visitors with leads | website.visitor extension |
| **survey_crm** | Survey-to-lead conversion | survey.survey, survey.user_input, crm.lead extension |
| **mass_mailing_crm** | Mass mailing on leads | utm.campaign, mailing.mailing extension |
| **mass_mailing_crm_sms** | SMS mass mailing | utm.campaign extension |
| **event_crm** | Event registration to leads | event.registration, event.lead.rule, event.lead.request |
| **event_crm_sale** | Event-to-opportunity-to-quote | event.registration extension |
| **website_event_crm** | Website event registration | event.registration extension |
| **gamification_sale_crm** | Gamification | (config-level) |
| **test_crm_full** | Test harness | (test-only) |
| **website_crm_iap_reveal** | Website visitor-to-lead AI | crm.reveal.view, crm.reveal.rule, reveal_ip |
| **iap_crm** | IAP integration | crm.lead extension |

#### Enterprise (4 modules)

| Module | Purpose | Key Features |
|--------|---------|-------------|
| **crm_enterprise** | Enhanced pipeline | web_cohort views, web_map views |
| **crm_enterprise_partner_assign** | Enterprise resellers | Partner assignment logic |
| **crm_helpdesk** | Helpdesk-CRM integration | helpdesk.ticket, bidirectional sync |
| **crm_sale_subscription** | Subscription opportunities | crm.lead with subscription generation |

### Core Model: crm.lead (73 fields)

The community `crm` module defines `CrmLead` with these field categories:

| Category | Fields |
|----------|--------|
| **Identity** | name, contact_name, partner_name, function, email_from, phone, website, street, street2, zip, city, state_id, country_id |
| **Contact Info** | email_from, email_normalized, email_domain_criterion, phone, phone_sanitized, phone_state, email_state |
| **Ownership** | user_id, user_company_ids, team_id, company_id |
| **Classification** | type, priority, stage_id, tag_ids, color, lead_properties |
| **Financial** | expected_revenue, prorated_revenue, recurring_revenue, recurring_plan, recurring_revenue_monthly, recurring_revenue_monthly_prorated, recurring_revenue_prorated |
| **Timing** | date_open, date_closed, date_deadline, date_last_stage_update, date_conversion, day_open, day_close, date_automation_last |
| **Probability** | probability, automated_probability, is_automated_probability, won_status |
| **Outcome** | lost_reason_id, commercial_partner_id |
| **Partner** | partner_id, partner_is_blacklisted, is_partner_visible, partner_email_update, partner_phone_update |
| **Activities** | calendar_event_ids, meeting_display_date, meeting_display_label |
| **Duplicates** | duplicate_lead_ids, duplicate_lead_count |
| **Tracking (UTM)** | campaign_id, medium_id, source_id (referred) |
| **Language** | lang_id, lang_code, lang_active_count |
| **Lifecycle** | active |

### Supporting Models

| Model | Purpose |
|-------|---------|
| **crm.stage** | Pipeline stages with probability, is_won flag, rotting_threshold_days, requirements |
| **crm.team** | Sales teams with alias, auto-assignment rules, lead/opportunity modes |
| **crm.team.member** | Team members with assignment domains, lead quotas |
| **crm.lost.reason** | Lost reason codes with lead counts |
| **crm.recurring.plan** | Recurring revenue plans (number of months, sequence) |
| **crm.lead.scoring.frequency** | Predictive scoring frequency tables |
| **crm.lead.scoring.frequency.field** | Scoring field definitions |
| **res.partner (extended)** | opportunity_ids, opportunity_count on partners |
| **utm.campaign (extended)** | use_leads flag, crm_lead_count |
| **calendar.event (extended)** | opportunity_id on events |
| **mail.activity** | Activity scheduling linked to leads |
| **digest.digest (extended)** | KPIs: kpi_crm_lead_created, kpi_crm_opportunities_won |
| **res.config.settings** | CRM config: auto-assignment, predictive scoring, lead mining, memberships |

### Key Feature Areas

1. **Predictive Lead Scoring (PLS)** — Bayesian probability computation based on won/lost historical data per field value
2. **Revenue Forecasting** — expected_revenue * probability = prorated_revenue; recurring revenue with prorated monthly calculations
3. **Pipeline Management** — Kanban stages with probability assignment, won/lost semantics, rotting detection
4. **Auto-Assignment** — Round-robin or domain-based lead assignment to team members with quotas
5. **UTM Tracking** — Campaign, medium, source tracking at the lead level
6. **Lead Enrichment** — IAP-powered company/contact data enrichment from email/website
7. **Lead Mining** — Automated lead discovery from databases
8. **Lead Merge** — Deduplication wizard
9. **Lead-to-Opportunity** — Conversion with partner association
10. **Web Integration** — Contact forms, livechat-to-lead, visitor-to-lead AI
11. **Mass Mailing** — CRM leads as mailing targets
12. **SMS** — SMS from leads and to visitors
13. **Email Plugin** — Email composition from CRM, bounce tracking
14. **Events** — Event registration auto-converted to leads
15. **Surveys** — Survey responses converted to leads
16. **Gamification** — CRM goal tracking
17. **Reporting** — Pivot tables, graphs, activity reports, opportunity analysis
18. **Duplicate Detection** — duplicate_lead_ids, duplicate_lead_count
19. **GDPR** | Blacklist management, partner visibility controls
20. **Multi-company** | user_company_ids, company_id
21. **Activities** | calendar_event_ids, mail.activity integration
22. **Map/Cohort Views** | Enterprise: web_map, web_cohort visualizations

---

## Gap Analysis

### CRITICAL GAPS (Architecture-level)

| # | Gap | Severity | Description |
|---|-----|----------|-------------|
| 1 | **No schema definitions** | CRITICAL | Sub-specs reference schemas but schemas dict is empty. The spec is incomplete. |
| 2 | **No entity relationships** | CRITICAL | Leads, contacts, opportunities are independent resources. No many-to-one, no cascading, no join semantics. |
| 3 | **No pipeline/stage model** | CRITICAL | No stages, no pipeline flow, no won/lost semantics. A CRM without a pipeline is just a database. |
| 4 | **No probability/scoring** | CRITICAL | No probability field, no automated scoring, no forecasting. The core of a CRM is opportunity prediction. |
| 5 | **No revenue model** | CRITICAL | No expected_revenue, no prorated_revenue, no recurring revenue concepts. |

### MAJOR GAPS (Feature-level)

| # | Gap | Description |
|---|-----|-------------|
| 6 | **No lead-contact-opportunity lifecycle** | No conversion, no merge, no type-based routing. |
| 7 | **No team/assignment model** | No crm.team, no team members, no auto-assignment. |
| 8 | **No UTM/tracking model** | No campaign, medium, source tracking for leads. |
| 9 | **No activity management** | No calendar events, no mail activity on leads. |
| 10 | **No duplicate detection** | No deduplication logic, no merge wizard. |
| 11 | **No enrichment/mining** | No IAP enrichment, no lead generation from databases. |
| 12 | **No web integration** | No contact form capture, no visitor-to-lead. |
| 13 | **No reporting/analytics** | No pivot, graph, analytical views. No KPI dashboards. |
| 14 | **No email integration** | No email composition from CRM, no bounce tracking. |
| 15 | **No SMS integration** | No SMS from leads, no SMS to visitors. |
| 16 | **No event/survey integration** | No event registration to lead conversion. |
| 17 | **No gamification** | No goal tracking for sales teams. |
| 18 | **No GDPR tools** | No blacklist, no partner visibility controls. |

### MINOR GAPS (Enhancement-level)

| # | Gap | Description |
|---|-----|-------------|
| 19 | **No partner enrichment** | No reseller/partner assignment from web forms. |
| 20 | **No subscription generation** | No opportunity-to-subscription conversion. |
| 21 | **No helpdesk integration** | No bidirectional CRM-helpdesk sync. |
| 22 | **No map/cohort views** | Enterprise features for territory visualization. |
| 23 | **No digest/KPI reports** | No automated CRM digest emails with KPIs. |
| 24 | **No recurring revenue plans** | No crm.recurring.plan model for deferred revenue. |

---

## Components of a World-Leading CRM

Based on the Odoo analysis and industry best practices, a world-leading CRM should have these **12 component domains**:

### 1. Lead & Contact Management
- Unified lead/contact/account entity model with type-based routing
- Lead-to-contact-to-opportunity conversion pipeline
- Lead merge/deduplication
- Partner/contact enrichment (IAP/API-powered)
- Blacklist/GDPR compliance tools

### 2. Pipeline & Stage Management
- Configurable pipeline stages with probability assignment
- Won/lost semantics with reason codes
- Stage rotting/detention detection
- Stage requirements and validation rules
- Multi-pipeline support (per team/product)

### 3. Opportunity & Revenue Management
- Expected revenue with prorated calculations
- Recurring revenue with monthly/annual plans
- Probability-weighted pipeline value
- Revenue forecasting (by team, product, period)
- Opportunity-to-quotation conversion

### 4. Predictive Analytics
- Bayesian lead scoring (predictive probability)
- Historical win-rate analysis by field/value
- Automated probability recomputation
- Probability tooltips with contributing factors
- Pipeline health scoring

### 5. Sales Team Management
- Team/org hierarchy
- Member assignment with domains and quotas
- Auto-assignment (round-robin, load-balanced)
- Team alias for email-to-lead
- Activity metrics per member

### 6. Marketing Integration
- UTM tracking (campaign, medium, source)
- Mass mailing integration
- Web form capture
- Event registration to lead conversion
- Survey response to lead conversion
- Livechat-to-lead conversion

### 7. Communication Hub
- Email composition from CRM records
- SMS from/to CRM records
- Activity scheduling (calendar events)
- Bounce tracking and email validation
- Meeting management linked to opportunities

### 8. Intelligence & Enrichment
- Company/contact data enrichment
- Automated lead discovery/mining
- Visitor-to-lead AI (web analytics)
- Role/industry/seniority detection
- Predictive scoring (as above)

### 9. Reporting & BI
- Pivot table views for pipeline analysis
- Graph/chart views (funnel, timeline, distribution)
- Activity reports
- KPI dashboards with digest emails
- Custom report builder

### 10. Workflow Automation
- Trigger-based actions (stage change, field update, email send)
- Rule engine for lead assignment and routing
- Workflow definitions with conditions
- Scheduled actions (cron-based)

### 11. Customer Engagement
- Livechat widget with agent management
- Chat-to-lead conversion
- Visitor tracking and attribution
- Gamification for sales teams
- Subscription and renewal management

### 12. Platform & Extensibility
- Multi-company support
- Multi-language support
- API-first design (OpenAPI)
- Webhook/event system
- Plugin/integration architecture
- Role-based access control

---

## Recommended Next Steps

1. **Define complete schemas** for all 9 entities — this is the foundation
2. **Implement the entity relationship model** — leads belong to contacts, opportunities belong to leads, etc.
3. **Build the pipeline stage model** — with probability, won/lost, rotting
4. **Add the probability/scoring model** — starting with manual probability, then automated
5. **Implement revenue fields** — expected_revenue, prorated_revenue, recurring_revenue
6. **Design the crm.team model** — with members, alias, and auto-assignment
7. **Add UTM tracking** — campaign, medium, source on leads
8. **Implement lead enrichment** — API-first enrichment endpoint
9. **Build reporting endpoints** — pipeline summary, conversion rates, forecast
10. **Add activity management** — calendar events and mail activity on leads

---

## Architecture Recommendation

RERP's current 3-service split (core/automation/livechat) is a reasonable starting point but needs expansion. The Odoo ecosystem shows that CRM touches almost every business function. A recommended evolution:

```
openapi/crm/
├── openapi.yaml              # Gateway spec
├── core/                     # Already exists: leads, contacts, opportunities
│   ├── openapi.yaml
│   └── (needs: stages, teams, members, lost_reasons, recurring_plans)
├── automation/               # Already exists: workflows, rules, triggers
│   ├── openapi.yaml
│   └── (needs: webhooks, scheduled_actions, lead_scoring)
├── livechat/                 # Already exists: agents, chats, messages
│   ├── openapi.yaml
│   └── (needs: sessions, integrations, visitor_tracking)
├── engagement/               # NEW: email, SMS, web forms
│   ├── openapi.yaml
│   └── (needs: emails, sms, web_forms, visitors)
├── intelligence/             # NEW: enrichment, mining, scoring
│   ├── openapi.yaml
│   └── (needs: enrichment, mining_requests, scoring_models)
├── reporting/                # NEW: analytics and dashboards
│   ├── openapi.yaml
│   └── (needs: pipeline_summary, forecasts, conversions)
└── marketing/                # NEW: campaigns, events, surveys
    ├── openapi.yaml
    └── (needs: campaigns, mailing_lists, event_registrations)
```

This gives **7 services** vs the current 3, aligning with the component domains above and allowing parallel development.

---

## Odoo CRM Module Dependency Map

For reference, here's how Odoo CRM modules depend on each other:

```
crm (core)
├── crm_livechat (extends crm)
├── crm_mail_plugin (extends crm)
├── crm_sms (extends crm)
├── crm_iap_enrich (extends crm)
├── crm_iap_mine (extends crm)
├── sale_crm (extends crm + sale)
├── website_crm (extends crm + website)
├── website_crm_livechat (extends crm + livechat)
├── website_crm_partner_assign (extends crm + website)
├── website_crm_sms (extends crm + website)
├── survey_crm (extends crm + survey)
├── mass_mailing_crm (extends crm + mass_mailing)
├── mass_mailing_crm_sms (extends crm + mass_mailing)
├── event_crm (extends crm + event)
├── event_crm_sale (extends event_crm + sale)
├── website_event_crm (extends event_crm + website)
├── gamification_sale_crm (extends gamification + sale + crm)
├── website_crm_iap_reveal (extends crm + website + iap)
└── iap_crm (extends crm + iap)

crm_enterprise (extends crm + web_cohort + web_map)
├── crm_enterprise_partner_assign (extends crm_enterprise + website_crm_partner_assign)
├── crm_helpdesk (extends crm + helpdesk)
└── crm_sale_subscription (extends crm + sale_subscription)
```

This shows the **interconnected nature** of a mature CRM — it's not a single module but an ecosystem. RERP should design its API to support similar composability from the start.
