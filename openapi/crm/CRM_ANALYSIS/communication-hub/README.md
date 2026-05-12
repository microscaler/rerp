# Communication Hub

> **Component:** Email, SMS, calendar, meeting management, and activity timeline
> **Priority:** P3 — Communication layer that keeps CRM as single source of truth
> **Odoo Reference:** mail.thread, calendar.event, mail.alias, crm_mail_plugin

---

## The Pitch

**Buyer Question:** *Can I communicate with my contacts from within the CRM, without ever leaving it?*

If your sales rep has to open Outlook, then switch to their phone, then switch to the CRM to log a call, they won't use the CRM. Communication should happen inside the CRM — email composition, SMS, calendar events, meetings, and activity logging — all linked to the contact or opportunity. This component is the communication layer that keeps the CRM as the single source of truth for every customer interaction.

---

## What This Component Does

1. **Email Composition** — Compose and send emails directly from the CRM
2. **Email Templates** — Reusable templates with merge fields (name, company, deal value)
3. **Email Threading** — Group related emails into conversations per contact
4. **Email Tracking** — Track opens, clicks, bounces (with embedded tracking pixels)
5. **SMS** — Send and receive text messages from within CRM
6. **Calendar Events** — Schedule meetings linked to opportunities or contacts
7. **Activity Timeline** — Chronological view of all interactions per record (emails, calls, meetings, notes)
8. **Follow-up Tasks** — Create and track follow-up actions with due dates
9. **Meeting Scheduling** — Calendly-like availability booking for prospects
10. **Email Templates with Merge** — Dynamic fields pulled from contact/opportunity data
11. **Draft Storage** — Save draft emails to resume later
12. **Bulk Email** — Send bulk communications with tracking

---

## Entity Model

### Email Communication Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `from` | String (255) | Yes | Sender email address |
| `to` | String (1000) | Yes | Recipient(s) — comma-separated for multiple |
| `cc` | String (1000) | No | CC recipients |
| `bcc` | String (1000) | No | BCC recipients |
| `subject` | String (512) | Yes | Email subject line |
| `body` | Text | Yes | Email body (HTML + plain text) |
| `body_html` | Text | No | HTML version of body |
| `status` | Enum: [DRAFT, SENT, DELIVERED, BOUNCED, FAILED] | Yes | Send status |
| `is_template` | Boolean | No | Is this a template (not sent)? |
| `template_id` | Foreign Key: EmailTemplate | No | Associated template (if is_template) |
| `related_lead_id` | Foreign Key: Lead | Yes | Lead this email relates to |
| `related_contact_id` | Foreign Key: Contact | No | Contact this email relates to |
| `related_opportunity_id` | Foreign Key: Opportunity | No | Opportunity this email relates to |
| `parent_message_id` | UUID | No | Parent email in thread |
| `in_reply_to` | String (255) | No | Message-ID being replied to |
| `sent_at` | DateTime | No | When email was sent |
| `delivered_at` | DateTime | No | When email was delivered |
| `opened_at` | DateTime | No | When recipient opened |
| `clicked_at` | DateTime | No | When link was clicked |
| `bounce_reason` | String (512) | No | Reason for bounce |
| `tracking_open_count` | Integer | No | Number of times opened |
| `tracking_link_clicks` | Integer | No | Number of link clicks |
| `create_uid` | Foreign Key: User | No | User who sent |
| `create_date` | DateTime | No | Creation timestamp |

### Email Template Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Template name |
| `subject` | String (512) | Yes | Email subject (with merge fields) |
| `body` | Text | Yes | Email body HTML (with merge fields) |
| `is_default` | Boolean | No | Default template for this type |
| `category` | Enum: [FOLLOW_UP, PROPOSAL, NEGOTIATION, CLOSING, WELCOME, NURTURE] | No | Template category |
| `body_plain` | Text | No | Plain text fallback |
| `company_id` | Foreign Key: Company | No | Multi-company |
| `active` | Boolean | No | Soft delete |

### SMS Communication Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `from` | String (64) | Yes | Sender phone number |
| `to` | String (64) | Yes | Recipient phone number |
| `body` | Text | Yes | SMS body (160 char limit) |
| `status` | Enum: [SENT, DELIVERED, FAILED, RECEIVED] | Yes | Send/receive status |
| `related_lead_id` | Foreign Key: Lead | No | Lead this relates to |
| `related_contact_id` | Foreign Key: Contact | No | Contact this relates to |
| `sent_at` | DateTime | No | When sent |
| `received_at` | DateTime | No | When received (inbound) |
| `is_inbound` | Boolean | No | Was this received (not sent)? |
| `provider` | String (64) | No | SMS provider (Twilio, etc.) |
| `provider_message_id` | String (255) | No | Provider's message ID |

### Calendar Event Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `title` | String (255) | Yes | Meeting title |
| `description` | Text | No | Meeting description/notes |
| `start` | DateTime | Yes | Meeting start time |
| `end` | DateTime | Yes | Meeting end time |
| `location` | String (255) | No | Meeting location or URL |
| `meeting_url` | String (512) | No | Video conference URL (Zoom, Teams) |
| `status` | Enum: [TENTATIVE, CONFIRMED, CANCELLED, BUSY] | Yes | Meeting status |
| `related_lead_id` | Foreign Key: Lead | No | Lead this relates to |
| `related_contact_id` | Foreign Key: Contact | No | Contact this relates to |
| `related_opportunity_id` | Foreign Key: Opportunity | No | Opportunity this relates to |
| `organizer_id` | Foreign Key: User | Yes | Meeting organizer |
| `attendee_ids` | Many2Many: User | No | Meeting attendees |
| `reminder_minutes` | Integer | No | Minutes before meeting to remind |
| `external_calendar_id` | String (255) | No | External calendar ID (Google, Outlook) |
| `external_event_id` | String (255) | No | External event ID (for sync) |
| `create_uid` | Foreign Key: User | No | Who created |
| `create_date` | DateTime | No | When created |

### Activity/Note Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `activity_type` | Enum: [EMAIL, CALL, MEETING, NOTE, TASK, FOLLOW_UP] | Yes | Activity type |
| `summary` | String (512) | Yes | Brief summary |
| `description` | Text | No | Full description/notes |
| `related_lead_id` | Foreign Key: Lead | Yes | Lead this relates to |
| `related_contact_id` | Foreign Key: Contact | No | Contact this relates to |
| `related_opportunity_id` | Foreign Key: Opportunity | No | Opportunity this relates to |
| `user_id` | Foreign Key: User | Yes | User who logged activity |
| `scheduled_date` | DateTime | No | When activity was supposed to happen |
| `completed_date` | DateTime | No | When activity was completed |
| `create_date` | DateTime | No | When logged |

### Follow-up Task Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `title` | String (255) | Yes | Task title |
| `description` | Text | No | Task details |
| `due_date` | DateTime | Yes | Task due date |
| `priority` | Enum: [LOW, NORMAL, HIGH, URGENT] | Yes | Task priority |
| `status` | Enum: [TODO, IN_PROGRESS, DONE, CANCELLED] | Yes | Task status |
| `assigned_to` | Foreign Key: User | Yes | Who owns the task |
| `related_lead_id` | Foreign Key: Lead | No | Lead this relates to |
| `related_contact_id` | Foreign Key: Contact | No | Contact this relates to |
| `related_opportunity_id` | Foreign Key: Opportunity | No | Opportunity this relates to |
| `reminder_date` | DateTime | No | Reminder trigger date |
| `create_date` | DateTime | No | When created |

---

## Activity Timeline Architecture

The activity timeline is not a UI widget — it's a data model. Every communication event is a record in one of the entities above. The timeline is built by querying all related records ordered by date:

```
Timeline for Lead #123:
  SELECT * FROM ALL communication entities
  WHERE related_lead_id = 123
  ORDER BY date DESC
  
  Results:
    2026-05-10 14:30  Email sent to john@acme.com "Proposal Follow-up"
    2026-05-09 10:00  Meeting: Demo with Acme Corp (Zoom)
    2026-05-08 09:15  Call logged: Discussed pricing
    2026-05-05 16:00  Task: Send proposal by Friday
    2026-05-03 11:00  Email received: "We need to discuss..."
    2026-05-01 09:00  Email sent: "Introduction - RERP Demo"
```

---

## Required API Endpoints

### Email Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/communications/email` | Compose and send email |
| `POST` | `/communications/email/draft` | Save draft email |
| `GET` | `/communications/email/{id}` | Get email details |
| `GET` | `/communications/email/conversation/{id}` | Email thread (parent/children) |
| `GET` | `/communications/email/reply/{id}` | Get reply template |

### Email Templates

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/templates` | List email templates |
| `POST` | `/templates` | Create template |
| `PATCH` | `/templates/{id}` | Update template |
| `POST` | `/templates/{id}/preview` | Preview with merge fields |
| `POST` | `/templates/{id}/send` | Send template to contact |

### SMS Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/communications/sms` | Send SMS |
| `GET` | `/communications/sms/{id}` | Get SMS details |
| `POST` | `/webhooks/sms` | Inbound SMS webhook |

### Calendar & Events

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/calendar/events` | Create meeting event |
| `PATCH` | `/calendar/events/{id}` | Update event |
| `DELETE` | `/calendar/events/{id}` | Cancel event |
| `GET` | `/calendar/upcoming` | Upcoming events for user |
| `POST` | `/calendar/schedule` | Schedule meeting (Calendly-like) |

### Activity Timeline

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/leads/{id}/timeline` | Full activity timeline for lead |
| `GET` | `/contacts/{id}/timeline` | Full activity timeline for contact |
| `GET` | `/opportunities/{id}/timeline` | Full activity timeline for opportunity |
| `POST` | `/activities/log` | Log activity (call, note, task) |

### Tasks

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/tasks` | Create follow-up task |
| `GET` | `/tasks/upcoming` | Upcoming tasks for user |
| `PATCH` | `/tasks/{id}/complete` | Mark task as done |
| `GET` | `/tasks/overdue` | Overdue tasks |

---

## Odoo Technical Patterns to Follow

### Pattern 1: mail.thread for Email Logging
Odoo's `mail.thread` mixin automatically logs emails to the CRM when sent via Outlook integration. Every sent/received email creates a mail.message record linked to the lead/opportunity.

**Recommendation: RERP should log all emails sent via the API as activity timeline records automatically. No separate "email sent" step needed.**

### Pattern 2: Calendar Events as First-Class Entities
Odoo has a separate `calendar.event` model that's linked to CRM via `crm_lead.calendar_event_ids`. This allows calendar data to be shared across all modules (CRM, Project, Sales), not just CRM.

**Recommendation: RERP should define CalendarEvent as a standalone entity that can be linked to any CRM record.**

### Pattern 3: Activity Timeline as Aggregated View
Odoo's timeline is not a single table — it's an aggregation of mail.message, calendar.event, and mail_activity records, all ordered by date.

**Recommendation: RERP should implement the timeline as an API endpoint that queries all communication entities and aggregates them by date. The UI renders the aggregated result.**

---

## Competitive Positioning

### Where RERP Wins
- **Async Rust email processing** — Non-blocking email composition and sending via async SMTP/SMTPS. No blocking I/O.
- **Activity timeline as API-first** — Every communication event is a structured, queryable record. No need for a UI-specific timeline.
- **Self-hosted communication** — No SendGrid or Twilio subscription fees. Use your own SMTP server.

### Where RERP Lags
- **No email integration** — No email composition or template endpoints.
- **No calendar integration** — No two-way calendar sync with external calendars.
- **No meeting scheduling** — Calendly-style booking is a standard expectation.
- **No email tracking** — No open/click tracking without third-party integrations.

---

## Competitive Intelligence Deep Dive

### Salesforce (Email + Activity Capture — $25–$330/user/month)
**Einstein Activity Capture** automatically logs emails and calendar events. **Email Templates** with merge fields. **Conversations** show email threading per contact. **Task and Event** objects link activities to deals. **Email-to-Case** converts inbound emails to support tickets.

### Microsoft Dynamics (Outlook Integration — $65–$200/user/month)
**Email Router** integrates with Exchange Server. **Outlook Plugin** logs emails and calendar events automatically. **Templates** with dynamic fields. **Letter** entities for print communications (proposals, contracts). Best for organizations that live in Outlook.

### HubSpot (Unified Inbox — $20–$1,800+/month)
**Unified Inbox** aggregates email, SMS, chat, and call history. **Meeting Scheduler** lets prospects book meetings directly. **Email Tracking** shows opens, clicks, engagement in real-time. **Sequences** automate drip campaigns. All communications in one place, zero context switching.

---


### ServiceNow: Omnichannel Communication via Service Cloud
ServiceNow's communication capabilities are primarily **service-oriented** — email, chat, voice, and social are unified under Customer Service Management. The ServiceNow Otto AI assistant handles inbound communications autonomously. **IntegrationHub** pre-built connectors link communication channels (Slack, Teams, Zoom, Five9, NiCE). **Zoom integration** enables AI Companion triggers that launch actionable workflows in Now Assist. **Five9 integration** combines ServiceNow CSM with CCaaS for real-time transcription, unified routing, and consolidated agent workspace. **Gap vs. Salesforce:** No Conga/Slack deep integration for sales emails. **Gap vs. HubSpot:** No Conversations inbox (email, chat, SMS, social in one place). **Gap vs. Microsoft:** No Teams native integration for sales collaboration. **Unique strength:** Communication is workflow-embedded — every email, call, or chat is a workflow event that can trigger automated follow-ups, case creation, or opportunity updates.
## Implementation Roadmap

### Phase 1: Core Communication (2-3 weeks)
1. Define `EmailCommunication`, `SmsCommunication`, `CalendarEvent` entities
2. Add `calendar_event_ids` (Many2Many) to Lead/Opportunity
3. Implement email composition endpoint (POST /communications/email)
4. Implement calendar event creation endpoint
5. Implement SMS send endpoint

### Phase 2: Templates & Timeline (2-3 weeks)
1. Define `EmailTemplate` entity with merge field support
2. Implement `ActivityTimeline` endpoint (aggregate all comms for a record)
3. Implement email template preview/merge endpoint
4. Implement draft email storage
5. Implement activity log endpoint (calls, notes, tasks)

### Phase 3: Tracking & Scheduling (3-4 weeks)
1. Calendar integration (Google Calendar, Outlook sync)
2. Meeting scheduling endpoint (availability booking)
3. Email open/click tracking (tracking pixel webhooks)
4. Call logging endpoint (VoIP integration)
5. Bulk email from CRM endpoint

---

## Key Takeaway for Buyers

Communication is the glue that holds the CRM together. A buyer needs to know: *Can I email a prospect, log a call, schedule a meeting, and send an SMS — all from the CRM?* RERP's API-first approach means every communication can be automated (e.g., "when a lead enters stage X, send email Y"). The tradeoff: the developer has to build the integration. For teams who need out-of-the-box email templates and Outlook sync, RERP's pitch is "we'll build exactly what you need."
