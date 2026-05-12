# Workflow Automation

> **Component:** Trigger-based actions, rules, and scheduled workflows
> **Priority:** P3 — Valuable but non-blocking for initial adoption
> **Odoo Reference:** ir_cron_data, workflow entity, rule entity, crm_lead scheduling, stage-change triggers

---

## The Pitch

**Buyer Question:** *Can my CRM automate repetitive tasks, route work to the right people, and enforce business rules — without writing code?*

A CRM that requires manual data entry for every action is a time sink. A CRM that automates: lead assignment, email triggers, stage transitions, approval workflows, and scheduled follow-ups is a force multiplier. This component covers the rules engine that makes CRM work in the background, not just as a data entry UI.

---

## What This Component Does

1. **Trigger-Based Actions** — When X happens, do Y (stage change, field change, time-based)
2. **Stage-Change Triggers** — "When lead moves to Proposal, send email, create task, notify manager"
3. **Field-Value Triggers** — "When expected_revenue > $100K, require approval"
4. **Time-Based Triggers** — "If no activity in 3 days, send reminder to rep"
5. **Workflow Definitions** — Composable workflows with conditions and actions
6. **Rule Engine** — Simple condition-action pairs (IF field=value THEN action)
7. **Scheduled Actions** — Cron-based recurring tasks (daily digest, weekly review)
8. **Email Triggers** — Automated emails on conditions (welcome, follow-up, nurture)
9. **Task Creation Triggers** — Auto-create follow-up tasks on events
10. **Approval Workflows** — Multi-step approval for discounts, contracts, pricing
11. **Escalation Rules** — "If deal not updated in 7 days, escalate to manager"
12. **Cross-Service Triggers** — "When CRM deal closes, create invoice in Accounting"

---

## Entity Model

### Workflow Entity

A workflow is a named collection of rules that fires on a specific trigger:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Workflow name (e.g., "Proposal Stage Email") |
| `description` | Text | No | Workflow description |
| `is_active` | Boolean | Yes | Enable/disable workflow |
| `trigger_type` | Enum: [STAGE_CHANGE, FIELD_CHANGE, TIME_BASED, WEBHOOK, MANUAL] | Yes | What triggers this workflow |
| `trigger_config` | JSON | Yes | Trigger-specific configuration |
| `action_type` | Enum: [SEND_EMAIL, CREATE_TASK, UPDATE_FIELD, ASSIGN_LEAD, CALL_WEBHOOK, CREATE_RECORD] | Yes | What action to perform |
| `action_config` | JSON | Yes | Action-specific configuration |
| `conditions` | JSON | Yes | Additional conditions (AND/OR logic) |
| `sequence` | Integer | Yes | Execution order (lower = earlier) |
| `created_at` | DateTime | Yes | Creation timestamp |
| `updated_at` | DateTime | No | Last modification |
| `run_count` | Integer | Computed | Times this workflow has fired |
| `last_run` | DateTime | No | Last time it fired |

### Rule Entity

A rule is a condition-action pair within a workflow. A workflow can have multiple rules:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `workflow_id` | Foreign Key: Workflow | Yes | Parent workflow |
| `condition_field` | String (128) | Yes | Field to evaluate (e.g., "stage_id", "expected_revenue") |
| `condition_operator` | Enum: [EQUALS, NOT_EQUALS, GREATER_THAN, LESS_THAN, CONTAINS, NOT_CONTAINS, IS_EMPTY, IS_NOT_EMPTY, IN, NOT_IN] | Yes | Comparison operator |
| `condition_value` | String | Yes | Value to compare against |
| `condition_value_type` | Enum: [TEXT, INTEGER, FLOAT, BOOLEAN, DATE, ENTITY] | Yes | Type of condition_value |
| `action_type` | Enum: [SEND_EMAIL, CREATE_TASK, UPDATE_FIELD, ASSIGN_LEAD, CALL_WEBHOOK, SET_VALUE] | Yes | Action to perform |
| `action_config` | JSON | Yes | Action-specific configuration |
| `is_active` | Boolean | Yes | Enable/disable this rule |
| `sequence` | Integer | Yes | Execution order within workflow |

### Trigger Entity

Triggers are the event hooks that start workflows:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `workflow_id` | Foreign Key: Workflow | Yes | Associated workflow |
| `type` | Enum: [STAGE_CHANGE, FIELD_CHANGE, TIME_BASED, WEBHOOK, ON_CREATE, ON_UPDATE, ON_DELETE] | Yes | Trigger type |
| `entity` | String (64) | Yes | Entity to watch (crm.lead, crm.contact, etc.) |
| `field` | String (128) | No | Field to watch (for field-based triggers) |
| `schedule` | String (64) | No | Cron expression (for time-based) |
| `enabled` | Boolean | Yes | Enable/disable trigger |
| `last_fired` | DateTime | No | Last time trigger fired |

---

## Trigger-Action Patterns

### Pattern 1: Stage-Change → Email

```
Workflow: "Proposal Stage Follow-Up"
Trigger: stage_change
Condition: stage_id = "proposal" AND days_in_stage > 5
Action: send_email
Config: {
  "template_id": "follow_up_proposal",
  "recipient": "{{lead.email_from}}",
  "subject": "Following up on our proposal",
  "merge_fields": {
    "name": "{{lead.name}}",
    "expected_revenue": "{{lead.expected_revenue}}"
  }
}
```

### Pattern 2: Time-Based → Task

```
Workflow: "Stale Lead Reminder"
Trigger: time_based (every 24h via cron)
Condition: lead.days_since_last_activity > 3 AND won_status = PENDING
Action: create_task
Config: {
  "title": "Follow up with {{lead.name}}",
  "assigned_to": "{{lead.user_id}}",
  "due_date": "tomorrow",
  "related_lead_id": "{{lead.id}}"
}
```

### Pattern 3: Field-Value → Approval

```
Workflow: "Discount Approval"
Trigger: field_change
Condition: expected_revenue < discount_amount AND discount > 10
Action: create_task (approval required)
Config: {
  "title": "Discount approval needed for {{lead.name}}",
  "assigned_to": "{{lead.team_id.manager_id}}",
  "due_date": "in 2 days"
}
```

### Pattern 4: On-Create → Assignment

```
Workflow: "New Lead Assignment"
Trigger: on_create (entity: crm.lead)
Condition: team_id = NULL
Action: assign_lead
Config: {
  "algorithm": "round_robin",
  "team_id": "auto_detect_from_source"
}
```

---

## Required API Endpoints

### Workflow Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/workflows` | List all workflows |
| `GET` | `/workflows/{id}` | Get workflow detail |
| `POST` | `/workflows` | Create workflow |
| `PATCH` | `/workflows/{id}` | Update workflow |
| `DELETE` | `/workflows/{id}` | Delete workflow |
| `POST` | `/workflows/{id}/run` | Manually trigger workflow |
| `GET` | `/workflows/{id}/run-history` | History of workflow executions |

### Rule Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/workflows/{id}/rules` | List rules for workflow |
| `POST` | `/workflows/{id}/rules` | Add rule to workflow |
| `PATCH` | `/rules/{id}` | Update rule |
| `DELETE` | `/rules/{id}` | Delete rule |
| `POST` | `/rules/test` | Test rule against a lead |

### Trigger Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/triggers` | List all triggers |
| `POST` | `/triggers/{id}/fire` | Manually fire trigger |
| `GET` | `/triggers/run-history` | Execution history |

### Scheduled Actions

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/actions/schedule` | Schedule one-time action |
| `POST` | `/actions/recurring` | Schedule recurring action |
| `GET` | `/actions/upcoming` | Upcoming scheduled actions |
| `DELETE` | `/actions/{id}` | Cancel scheduled action |

### Webhooks

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/webhooks` | List webhooks |
| `POST` | `/webhooks` | Create webhook |
| `POST` | `/webhooks/{id}/test` | Test webhook |
| `GET` | `/webhooks/{id}/delivered` | Delivery history |

---

## Cron Automation (Scheduled Tasks)

These are system-level cron jobs that run on a schedule:

| Cron Job | Schedule | Purpose |
|----------|----------|---------|
| `assign_leads` | Daily at 9:00 | Auto-assign unassigned leads |
| `run_workflows` | Every 5 minutes | Check and fire workflow triggers |
| `send_digests` | Daily at 8:00 | Send KPI digest emails |
| `check_rotting` | Every hour | Flag stalled deals |
| `recompute_scores` | Daily at 2:00 | Rebuild scoring frequencies |
| `cleanup_old_data` | Weekly | Archive old activities |

---

## Competitive Positioning

### Where RERP Wins
- **Rust-based rule evaluation** — Evaluating thousands of rules against lead data is instantaneous. Python-based workflows (Odoo) evaluate sequentially and can be slow at scale.
- **API-defined workflows** — Workflow and rule schemas are OpenAPI-defined. Every client gets the same automation logic.
- **Self-hosted automation** — No Flow or Power Automate subscription. All automation runs on your infrastructure.

### Where RERP Lags
- **Workflow schema exists but has no definition** — Schemas are empty. No condition syntax, no action definitions.
- **No email triggers** — "When lead enters Proposal, send email template X" is missing.
- **No approval workflows** — No discount approval, contract approval, escalation chains.
- **No visual workflow builder** — Salesforce Flow and HubSpot Workflows have drag-and-drop builders.

---

## Competitive Intelligence Deep Dive

### Salesforce Flow Builder ($25–$330/user/month)
**Flow Builder** is a visual drag-and-drop tool with 25+ element types. **Record-Triggered Flows** fire on create/update/delete. **Scheduled Flows** run on cron-like schedules. **Decision elements** provide complex branching. **Error handling** with try/catch and rollback. **Debug mode** with step-by-step execution. **Subflows** for reusable automation. Unmatched depth.

### Microsoft Power Automate ($15–$40/user/month)
**Power Automate** integrates with 700+ connectors. **Cloud Flows** trigger on CRM events. **Business Process Flows** enforce required fields. **Approval Flows** route deals, discounts, contracts. **AI Builder** adds document processing. Integration with Microsoft 365 is the differentiator.

### HubSpot Workflows (included in Pro/Ent)
**Workflows** are visual automation builders with 50+ action types. **Enrollment triggers** include form submit, property change, date-based. **Actions** include email, SMS, update property, create task. **Sequences** automate drip campaigns. **Conditional splits** route contacts differently. Simple "if this, then that" for marketing and sales.

---


### ServiceNow: The Killer Feature — Enterprise Workflow Automation
**This is ServiceNow's dominant competitive advantage.** ServiceNow built the industry's best workflow automation platform through Flow Designer (no-code visual workflow builder), IntegrationHub (pre-built connectors for 200+ apps), and AI Agent Fabric (multi-agent orchestration). Every customer interaction triggers workflows: lead capture → qualification → nurturing → opportunity creation → quote → order → fulfillment → service. **AI Agents** execute these workflows autonomously (not just assist humans). At Knowledge 2025, ServiceNow unveiled autonomous CRM where AI agents handle end-to-end sales and service processes: instant inquiry resolution, complex case routing, cross-department coordination (sales + IT + HR + field service). **Logik.ai CPQ** added to the workflow: quote generation, configuration validation, pricing. **Gap vs. Salesforce:** No Flow Builder with comparable enterprise scope. **Gap vs. Zapier:** Zapier is point-to-point; ServiceNow is enterprise workflow orchestration across silos. **Unique strength:** Workflow Data Fabric + AI Control Tower = autonomous, governed, cross-functional automation that pure-play CRMs cannot replicate.
## Implementation Roadmap

### Phase 1: Core Engine (2-3 weeks)
1. Define `Workflow`, `Rule`, `Trigger` entities
2. Implement workflow creation endpoint (POST /workflows)
3. Implement rule creation endpoint (POST /rules)
4. Implement basic stage-change trigger
5. Implement rule evaluation engine (evaluate conditions against a lead)

### Phase 2: Actions & Triggers (2-3 weeks)
1. Implement email-trigger action (send template on condition match)
2. Implement task-creation action
3. Implement field-update action (set value on lead)
4. Implement lead-assignment action
5. Implement cron-based scheduled trigger

### Phase 3: Advanced (3-4 weeks)
1. Cross-service triggers (CRM → Accounting, CRM → Marketing)
2. Conditional branching (if/else rules in workflow)
3. Approval workflow chain endpoint
4. Escalation rules (if no action in X days, notify manager)
5. Webhook endpoint (external system triggers CRM workflow)

### Phase 4: Builder UI Support (3-4 weeks)
1. Workflow versioning (track changes over time)
2. Workflow testing/debug endpoint (test mode)
3. Bulk action triggers (process many records at once)
4. Error handling and retry logic
5. Workflow performance monitoring (execution time, success rate)

---

## Key Takeaway for Buyers

Workflow automation is the difference between a passive CRM and an active CRM. A buyer needs to know: *Can I set up a rule that says 'when a deal reaches Proposal and hasn't been updated in 5 days, notify the manager' — in minutes, not days?* RERP's API-first approach means automation is defined in machine-readable specs. The immediate priority: fill the Workflow/Rule/Trigger schemas and implement stage-change triggers.
