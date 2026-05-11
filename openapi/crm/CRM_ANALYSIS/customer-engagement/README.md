# Customer Engagement

> **Component:** Livechat, gamification, subscriptions, helpdesk, and post-sale lifecycle
> **Priority:** P4 — Post-sale features come after sales CRM is solid
> **Odoo Reference:** crm_livechat (livechat sub), mail_livechat, mail_livechat_server, crm_helpdesk (enterprise), crm_subscription

---

## The Pitch

**Buyer Question:** *Does my CRM engage customers throughout their entire lifecycle — not just when they're a lead, but as a customer, advocate, or churn risk?*

CRM is not a lead management tool. It's a **customer relationship management** tool. After the sale, the relationship continues: support tickets, renewals, upgrades, cross-sells, and advocacy. This component covers the post-sale engagement surface — livechat, helpdesk integration, subscriptions, gamification, and customer success tools. Without this, the CRM dies the day the deal is closed.

---

## What This Component Does

1. **Livechat Widget** — Real-time chat on any website, converting visitors to leads
2. **Chat-to-Lead Conversion** — Every chat session creates a CRM record automatically
3. **Agent Management** — Track agent online status, active sessions, capacity
4. **Chatbot Integration** — AI chatbot resolves common queries before human handoff
5. **Helpdesk/Ticket Integration** — Bidirectional sync between CRM leads and support tickets
6. **Subscription Management** — Track recurring subscriptions, renewal dates, status
7. **Renewal Alerts** — Notify reps when subscriptions are up for renewal
8. **Upgrade/Cross-Sell Prompts** — Suggest upgrades based on usage patterns
9. **Customer Health Score** — Score customers based on engagement, support, usage
10. **Gamification** — Goals, badges, and leaderboards for sales motivation
11. **Goal Setting & Tracking** — Individual and team goals with progress tracking
12. **Achievement/Badges** — Milestone rewards (first deal, $100K month, etc.)
13. **Customer Feedback (CSAT/NPS)** — Post-interaction surveys with trend tracking
14. **Customer Portal** — Branded self-service portal for customers
15. **Knowledge Base** — Searchable FAQ linked to support tickets

---

## Entity Model

### Chat Session Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `visitor_id` | String | No | Anonymous visitor ID |
| `contact_id` | Foreign Key: Contact | No | Converted contact (if any) |
| `lead_id` | Foreign Key: Lead | No | Converted lead (if any) |
| `agent_id` | Foreign Key: User | Yes | Chat agent |
| `status` | Enum: [WAITING, ACTIVE, CLOSED, TRANSFERRED] | Yes | Session status |
| `started_at` | DateTime | Yes | When session started |
| `ended_at` | DateTime | No | When session ended |
| `duration_seconds` | Integer | Computed | Session duration |
| `transcript` | JSON | Yes | Full chat message history |
| `satisfaction_rating` | Integer | No | 1-5 rating from visitor |
| `tags` | JSON | No | Session tags (e.g., ["pricing", "demo"]) |
| `converted_to_lead` | Boolean | No | Whether this became a lead |
| `created_at` | DateTime | Yes | Creation timestamp |

### Chat Message Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `session_id` | Foreign Key: ChatSession | Yes | Parent session |
| `sender_type` | Enum: [AGENT, VISITOR, BOT] | Yes | Who sent |
| `sender_id` | UUID | No | User ID (agent) or bot ID |
| `message` | Text | Yes | Chat message content |
| `is_system` | Boolean | No | System message (e.g., "Agent joined") |
| `sent_at` | DateTime | Yes | When sent |
| `read_at` | DateTime | No | When read by recipient |

### Agent Entity (extends User)

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key (extends User) |
| `is_online` | Boolean | Yes | Current online status |
| `active_sessions` | Integer | Computed | Number of active chats |
| `max_concurrent` | Integer | No | Max concurrent chats (default: 3) |
| `total_chats_today` | Integer | Computed | Chats handled today |
| `avg_response_time_seconds` | Float | Computed | Average response time |
| `satisfaction_avg` | Float | Computed | Average satisfaction rating |
| `queue_position` | Integer | Computed | Position in waiting queue |
| `skills` | JSON | No | Agent skills (e.g., ["technical", "sales"]) |

### Subscription Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `opportunity_id` | Foreign Key: Opportunity | Yes | Related closed-won deal |
| `customer_id` | Foreign Key: Contact | Yes | Customer (linked to opportunity's contact) |
| `status` | Enum: [ACTIVE, EXPIRED, CANCELLED, TRIAL, PAST_DUE] | Yes | Subscription status |
| `plan_id` | Foreign Key: Product/Plan | Yes | Subscription plan |
| `plan_name` | String (128) | Yes | Plan display name |
| `start_date` | Date | Yes | Subscription start |
| `end_date` | Date | Yes | Subscription end |
| `renewal_date` | Date | Yes | Next renewal date |
| `renewal_automatic` | Boolean | Yes | Auto-renew at end? |
| `value` | Decimal(15,2) | Yes | Subscription value (total) |
| `monthly_value` | Decimal(15,2) | Computed | MRR = value / months |
| `currency_id` | Foreign Key: Currency | No | Currency |
| `billing_cycle` | Enum: [MONTHLY, QUARTERLY, ANNUALLY] | Yes | Billing frequency |
| `payment_method` | String (255) | No | Payment method on file |
| `seats` | Integer | No | Number of seats/licenses |
| `created_at` | DateTime | Yes | When created |
| `cancelled_at` | DateTime | No | When cancelled (if applicable) |
| `cancellation_reason` | String (255) | No | Why cancelled |
| `churn_risk_score` | Float | Computed | Churn probability (0-100) |

### Renewal Alert Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `subscription_id` | Foreign Key: Subscription | Yes | Related subscription |
| `alert_type` | Enum: [RENEWAL_REMINDER, PRICE_CHANGE, CONTRACT_END, CANCELLATION_WARNING] | Yes | Alert type |
| `alert_date` | DateTime | Yes | When alert fires |
| `status` | Enum: [PENDING, SENT, ACKNOWLEDGED, RESOLVED] | Yes | Alert status |
| `assigned_to` | Foreign Key: User | Yes | Rep responsible |
| `message` | Text | Yes | Alert message |
| `sent_at` | DateTime | No | When notification sent |
| `created_at` | DateTime | Yes | When created |

### Gamification: Goal Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Goal name (e.g., "Q1 Revenue") |
| `type` | Enum: [REVENUE, DEALS, CALLS, MEETINGS, LEADS] | Yes | Goal type |
| `target_value` | Decimal(15,2) | Yes | Target amount |
| `start_date` | Date | Yes | Period start |
| `end_date` | Date | Yes | Period end |
| `user_id` | Foreign Key: User | No | Individual goal (NULL = team goal) |
| `team_id` | Foreign Key: Team | No | Team goal (NULL = org goal) |
| `currency_id` | Foreign Key: Currency | No | Currency for monetary goals |
| `is_active` | Boolean | Yes | Enable/disable |
| `completed_value` | Decimal(15,2) | Computed | Current progress |
| `completion_percentage` | Float | Computed | progress / target * 100 |

### Gamification: Badge Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Badge name (e.g., "First Deal") |
| `description` | Text | Yes | Badge description |
| `icon` | String (64) | No | Emoji/icon |
| `criteria` | JSON | Yes | Achievement criteria |
| `is_active` | Boolean | Yes | Enable/disable |

### Gamification: User Badge (Achievement)

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `user_id` | Foreign Key: User | Yes | Who earned it |
| `badge_id` | Foreign Key: Badge | Yes | Which badge |
| `earned_at` | DateTime | Yes | When earned |

---

## Customer Health Score Calculation

A composite score combining multiple signals:

```
Health Score = w1 × engagement_score + w2 × support_score + w3 × usage_score + w4 × payment_score

Where:
  engagement_score = recent meetings * 0.3 + emails * 0.2 + logins * 0.5
  support_score = ticket_count_inverted * 0.4 + resolution_time * 0.3 + satisfaction * 0.3
  usage_score = feature_adoption_rate * 0.5 + login_frequency * 0.5
  payment_score = on_time_payments * 0.6 + dunning_events_inverted * 0.4

Result: 0-100, where > 70 = healthy, 40-70 = at-risk, < 40 = churn risk
```

---

## Required API Endpoints

### Livechat

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/chats/start` | Start new chat session |
| `POST` | `/chats/{id}/message` | Send chat message |
| `GET` | `/chats/{id}/transcript` | Get full transcript |
| `POST` | `/chats/{id}/close` | Close chat session |
| `GET` | `/chats/active` | List active sessions |
| `POST` | `/chats/convert` | Convert chat to lead |

### Agent Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `PUT` | `/agents/{id}/status` | Set agent online/offline |
| `GET` | `/agents/status` | All agent statuses |
| `GET` | `/agents/{id}/metrics` | Agent performance metrics |
| `GET` | `/agents/queue` | Current waiting queue |

### Subscriptions & Renewals

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/subscriptions` | List all subscriptions |
| `GET` | `/subscriptions/{id}/status` | Subscription detail with status |
| `POST` | `/renewals/alerts` | Create renewal alert |
| `GET` | `/renewals/upcoming` | Upcoming renewals (next 30/60/90 days) |
| `POST` | `/subscriptions/{id}/renew` | Renew subscription |
| `POST` | `/subscriptions/{id}/cancel` | Cancel subscription |
| `GET` | `/subscriptions/mrr` | Monthly recurring revenue |
| `GET` | `/subscriptions/churn` | Churn rate and churned customers |

### Gamification

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/goals` | List all goals |
| `POST` | `/goals` | Create goal |
| `PUT` | `/goals/{id}/progress` | Update goal progress |
| `GET` | `/goals/{id}/progress` | Get goal progress |
| `GET` | `/leaderboard` | Ranked list of users |
| `GET` | `/users/{id}/badges` | User's earned badges |
| `GET` | `/badges` | Available badges |

### Customer Health

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/customers/{id}/health` | Customer health score |
| `GET` | `/customers/at-risk` | List at-risk customers |
| `GET` | `/customers/health-trends` | Health score history |

---

## Odoo Technical Patterns to Follow

### Pattern 1: Livechat as a Separate Microservice
Odoo's livechat is in a separate `mail_livechat` module, not embedded in CRM. It manages sessions, messages, and agents as separate entities, with optional conversion to CRM leads.

**Recommendation: RERP already has a `livechat` sub-service in its OpenAPI spec. This component belongs there. The CRM core should reference livechat sessions but not own them.**

### Pattern 2: Gamification as Optional Feature
Gamification (goals, badges) is optional in Odoo — it's in the enterprise edition. The core CRM doesn't require it.

**Recommendation: RERP should make gamification an optional module that plugs into the core CRM entities.**

### Pattern 3: Subscription Management as a Bridge to Accounting
In Odoo Enterprise, subscriptions bridge CRM → Sales → Invoicing → Accounting. A closed deal becomes a subscription, which generates recurring invoices, which post to the general ledger.

**Recommendation: RERP's subscription entity should have a `related_invoice_ids` field for future accounting integration. The billing cycle should map to invoice generation in the Accounting microservice.**

---

## Competitive Positioning

### Where RERP Wins
- **Rust-based livechat concurrency** — Handling 10,000+ concurrent chats is trivial in Rust. Python (Odoo) struggles at scale.
- **API-defined chat interactions** — Every chat message and session is OpenAPI-defined. Third-party integrations can consume the chat stream.
- **Self-hosted customer success** — No separate platform subscription. Build it into the CRM.

### Where RERP Lags
- **No gamification** — No goals, badges, or leaderboards.
- **No subscription/renewal management** — No recurring revenue tracking.
- **No helpdesk integration** — No ticket-to-lead sync.
- **No customer health scoring** — No proactive churn risk detection.
- **No customer portal** — No self-service for customers.

---

## Competitive Intelligence Deep Dive

### Salesforce Service Cloud + Experience Cloud ($25–$300/user/month)
**Service Cloud** handles tickets, cases, live chat, omnichannel routing. **Experience Cloud** provides branded self-service portals. **Einstein Customer 360** provides real-time health scoring. **Omni-Channel Routing** distributes cases across channels. **Knowledge Base** with inline article suggestions. **CSAT/NPS** surveys with trend tracking.

### HubSpot Conversations ($20–$1,800+/month)
**Chat** built into CRM — no separate tool. **Chatflows** create automated paths. **Tickets** convert chats to support cases. **Sequences** automate proactive outreach. **AI-powered suggestions** for reps. Seamless sales-marketing-service integration.

### Intercom (Messaging-First — $49–$189+/month)
**Messenger** provides livechat with real-time typing. **Fin AI** resolves common queries (70%+ deflection). **Segmentation** enables targeted outreach. **Customer Success** provides health scores and churn prediction. Best for SaaS companies.

---

## Implementation Roadmap

### Phase 1: Livechat Core (2-3 weeks)
1. Define `ChatSession`, `ChatMessage` entities
2. Enhance `Agent` entity: is_online, active_sessions, max_concurrent
3. Implement chat session endpoints (POST /chats, GET /chats/{id}/transcript)
4. Implement agent status endpoint
5. Implement chat-to-lead conversion endpoint

### Phase 2: Subscriptions & Renewals (2-3 weeks)
1. Define `Subscription` entity with renewal tracking
2. Define `RenewalAlert` entity
3. Implement renewal tracking endpoint
4. Implement renewal alert endpoint (GET /renewals/upcoming)
5. Implement renewal notification endpoint

### Phase 3: Gamification & Health (3-4 weeks)
1. Define `Goal`, `Badge`, `UserBadge` entities
2. Implement goal tracking endpoint
3. Implement leaderboard endpoint
4. Implement customer health score endpoint
5. Implement at-risk customer detection

### Phase 4: Helpdesk & Portal (3-4 weeks)
1. Helpdesk integration endpoint (bidirectional lead-ticket sync)
2. Customer feedback (CSAT/NPS) survey endpoint
3. Customer portal endpoint (self-service)
4. Knowledge base integration
5. Proactive outreach automation

---

## Key Takeaway for Buyers

Customer engagement extends the CRM beyond the sale. A buyer needs to know: *Can I manage renewals, track customer health, and provide support — without buying a separate platform?* RERP's advantage is that everything is in one system: leads become customers, customers become renewals, renewals become referrals. The highest-priority features: **livechat** (already partially implemented) and **subscription/renewal tracking** (critical for SaaS businesses).
