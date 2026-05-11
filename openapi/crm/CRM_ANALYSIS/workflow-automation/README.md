# Workflow Automation

> **Component:** Trigger-based actions, rules, and scheduled workflows
> **Competitive Landscape:** Salesforce Flow, Microsoft Power Automate, SAP Process Orchestration, HubSpot Workflows, Zoho Creator

## Pitch

**The Question Every Buyer Asks:** *"Can my CRM automate repetitive tasks, route work to the right people, and enforce business rules — without writing code?"*

A CRM that requires manual data entry for every action is a time sink. A CRM that automates: lead assignment, email triggers, stage transitions, approval workflows, and scheduled follow-ups is a force multiplier. This component covers the rules engine that makes CRM work in the background, not just as a data entry UI.

---

## Functional Requirement Matrix

| Feature | RERP CRM | Odoo CRM | Salesforce | Microsoft Dynamics 365 | SAP CRM | HubSpot | Zoho CRM |
|---------|----------|----------|------------|------------------------|---------|---------|----------|
| Trigger-based actions | Planned | ✅ (ir_cron_data) | ✅ (Flow Builder) | ✅ (Power Automate) | ✅ | ✅ (Workflows) | ✅ (Workflows) |
| Stage-change trigger | Planned | ✅ (stage change) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Field-value trigger | Planned | ✅ (scoring frequency) | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Time-based trigger | Planned | ✅ (cron) | ✅ (Delay) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Workflow definitions | Planned | ✅ (workflow entity) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Rule engine | Planned | ✅ (rule entity) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Scheduled actions | Planned | ✅ (ir_cron_data) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Email trigger on event | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Task creation trigger | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Approval workflows | Planned | ❌ | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ |
| Lead assignment rules | Planned | ✅ (team assignment) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Data validation rules | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Escalation rules | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Cross-service triggers | Planned | ❌ | ✅ (Cross-Object) | ✅ | ✅ | ✅ (Multi-step) | ✅ | ✅ |
| Conditional branching | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Workflow versioning | Planned | ❌ | ✅ (Deployments) | ✅ | ✅ | ✅ | ✅ | ❌ |
| Workflow testing/debug | Planned | ❌ | ✅ (Test Mode) | ✅ | ✅ | ✅ | ✅ | ❌ |
| Bulk action triggers | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Webhook triggers | Planned | ❌ | ✅ (Platform Events) | ✅ (Webhooks) | ✅ | ✅ (Webhooks) | ✅ | ✅ |
| Approval chains | Planned | ❌ | ✅ (Multi-level) | ✅ | ✅ | ❌ | ✅ | ❌ |

---

## Competitive Positioning

### Where RERP Wins
- **Rust-based rule evaluation** — Evaluating thousands of rules against lead data in Rust is instantaneous. Python-based workflow engines (Odoo) evaluate rules sequentially and can be slow at scale.
- **API-defined workflows** — Workflow and rule schemas are OpenAPI-defined. Every client gets the same automation logic automatically.
- **Self-hosted automation** — No Flow or Power Automate subscription. All automation runs on your infrastructure.

### Where RERP Lags
- **Workflow schema exists but has no definition** — The sub-spec defines Workflow, Rule, and Trigger endpoints but schemas are empty. No condition syntax, no action definitions.
- **No email triggers** — "When a lead enters stage 'Proposal', send email template X" is missing.
- **No approval workflows** — No discount approval, no contract approval, no escalation chains.
- **No visual workflow builder** — Salesforce Flow and HubSpot Workflows have drag-and-drop builders. RERP requires API/JSON definition.

---

## Competitive Intelligence Deep Dive

### Salesforce Flow Builder (Enterprise Automation — $25–$330/user/month)
**Flow Builder** is a visual drag-and-drop automation tool with 25+ element types. **Record-Triggered Flows** fire on record create/update/delete with conditional branching. **Scheduled Flows** run on cron-like schedules (daily, hourly, weekly). **Platform-Triggered Flows** fire on platform events, API calls, and external webhooks. **Decision elements** provide complex branching logic (AND/OR conditions). **Action elements** create records, send emails, update fields, call Apex, or invoke external APIs. **Error handling** with try/catch blocks and rollback on failure. **Debug mode** with step-by-step execution and data view at each step. **Subflows** for modular, reusable automation. **Deployment** via Change Sets and DevOps Center with environment promotion (sandbox → production). The depth is unmatched: you can build any business process as a Flow — from simple lead assignment to complex multi-org territory routing.

### Microsoft Power Automate (Cross-Platform Automation — $15–$40/user/month)
**Power Automate** integrates with 700+ connectors (Microsoft 365, SAP, Oracle, SharePoint, Salesforce, Slack, etc.). **Cloud Flows** trigger on CRM events (record create, field change, email received). **Desktop Flows** automate desktop applications via RPA. **Business Process Flows** enforce required fields and stage transitions in Dynamics 365. **Approval Flows** route deals, discounts, and contracts for multi-level approval with conditional routing. **Dataverse Triggers** fire on any change to Dataverse tables. **AI Builder** adds document processing, text classification, and prediction to flows. The integration with Microsoft 365 is the differentiator: automate between CRM, SharePoint, Teams, Outlook, and Excel without writing code.

### HubSpot Workflows (SMB Automation — included in Professional/Enterprise tiers)
**Workflows** are visual automation builders with 50+ action types. **Enrollment triggers** include form submit, property change, date-based (birthday, anniversary), and list membership. **Actions** include email, SMS, update property, create task, enroll in sequence, move to pipeline, and webhook. **Sequences** automate drip email campaigns with activity-based triggers (enroll if no reply in 3 days). **Re-enrollment** for recurring automation (quarterly check-in emails). **Smart Goals** automate based on revenue milestones. **Conditional splits** route contacts down different paths based on properties. The simplicity is the strength: "if this, then that" for marketing and sales automation. No code required.

### Zoho Creator/CRM Workflows (Low-Code Automation — $14–$52/user/month)
**Zoho Workflow Rules** trigger on record create, update, delete, and time-based events (days after record creation). **Blueprint** enforces strict stage-by-stage workflows — deals can't skip stages, and each stage can require specific data entry or manager approvals. **Approval Processes** support multi-level chains with conditional routing and delegation. **Function Automation** enables custom logic in Deluge script (Zoho's proprietary scripting language). **Custom Functions** call external APIs and process results. **Integration Hub** provides 200+ pre-built integrations for cross-platform automation. Best value for custom automation needs with enterprise-grade features at fraction of the cost.

---

## RERP CRM Implementation Roadmap

### Phase 1 (Immediate — 2-3 weeks)
1. Define `Workflow` entity: id, name, description, is_active, trigger_type, conditions, actions, created_at, updated_at
2. Define `Rule` entity: id, workflow_id, condition_field, condition_operator, condition_value, action_type, action_data
3. Define `Trigger` entity: id, workflow_id, type (stage_change, field_change, time_based, webhook), schedule, enabled
4. Implement workflow creation endpoint (POST /workflows)
5. Implement rule creation endpoint (POST /rules)

### Phase 2 (3-6 weeks)
1. Implement stage-change trigger (when lead moves to stage X, fire actions)
2. Implement email-trigger action (send template on condition match)
3. Implement task-creation action (create follow-up task)
4. Implement data-validation rule (block stage change if required fields empty)
5. Implement rule evaluation endpoint (test rules against a lead)

### Phase 3 (6-12 weeks)
1. Scheduled actions endpoint (cron-based, configurable intervals)
2. Cross-service triggers (workflow in CRM triggers action in another service)
3. Conditional branching (if/else rules in workflow)
4. Approval workflow chain endpoint
5. Escalation rules (if no action in X days, notify manager)
6. Webhook endpoint (external system triggers CRM workflow)

---

## Key Takeaway for Buyers

Workflow automation is the difference between a passive CRM (records what happened) and an active CRM (makes things happen). A buyer needs to know: *"Can I set up a rule that says 'when a deal reaches Proposal stage and hasn't been updated in 5 days, notify the manager and schedule a follow-up?' — in minutes, not days."* RERP's API-first approach means automation is defined in machine-readable specs, which is great for developers but needs a visual builder for non-technical users. The immediate priority: fill the Workflow/Rule/Trigger schemas and implement stage-change triggers.
