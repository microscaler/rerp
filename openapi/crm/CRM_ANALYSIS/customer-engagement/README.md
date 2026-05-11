# Customer Engagement

> **Component:** Livechat, gamification, subscriptions, helpdesk integration, and renewals
> **Competitive Landscape:** Salesforce Service Cloud, Zendesk, HubSpot Conversations, Microsoft Dynamics, SAP Service, Intercom

## Pitch

**The Question Every Buyer Asks:** *"Does my CRM engage customers throughout their entire lifecycle — not just when they're a lead, but as a customer, advocate, or churn risk?"*

CRM is not a lead management tool. It's a **customer relationship management** tool. After the sale, the relationship continues: support tickets, renewals, upgrades, cross-sells, and advocacy. This component covers the post-sale engagement surface — livechat, helpdesk integration, subscriptions, gamification, and customer success tools.

---

## Functional Requirement Matrix

| Feature | RERP CRM | Odoo CRM | Salesforce | Microsoft Dynamics 365 | SAP CRM | HubSpot | Pipedrive | Zoho CRM |
|---------|----------|----------|------------|------------------------|---------|---------|-----------|----------|
| Livechat widget | Planned | ✅ (livechat sub) | ✅ (Experience) | ✅ | ✅ | ✅ (Chat) | ✅ (Wavebot) | ✅ |
| Livechat agent management | Planned | ✅ (agents entity) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Chat-to-lead conversion | Planned | ✅ (chats entity) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Chat-to-ticket conversion | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Livechat transcript storage | Planned | ✅ (messages entity) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Chatbot/script builder | Planned | ❌ | ✅ (Chatbot Flows) | ✅ | ✅ | ✅ | ❌ | ✅ |
| Helpdesk/ticket integration | Planned | ❌ | ✅ (Service Cloud) | ✅ | ✅ | ✅ (Tickets) | ❌ | ✅ (Desk) |
| Subscription management | Planned | ❌ | ✅ | ✅ | ✅ | ✅ (Subscriptions) | ❌ | ✅ (Subscriptions) |
| Renewal tracking | Planned | ❌ | ✅ (Renewals) | ✅ | ✅ | ✅ | ❌ | ✅ |
| Renewal alerts | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Upgrade/cross-sell prompts | Planned | ❌ | ✅ (Einstein) | ✅ | ✅ | ✅ | ❌ | ✅ |
| Customer health score | Planned | ❌ | ✅ (Einstein) | ✅ | ✅ | ✅ | ❌ | ❌ |
| Gamification | Planned | ❌ | ✅ (Badges/Quota) | ✅ | ✅ | ✅ (Goals) | ❌ | ✅ (Goals) |
| Goal setting & tracking | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Leaderboard | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Achievement/badges | Planned | ❌ | ✅ (Salesforce Badges) | ✅ | ❌ | ❌ | ❌ | ✅ |
| Customer feedback/CSAT | Planned | ❌ | ✅ (Surveys) | ✅ | ✅ | ✅ | ❌ | ✅ |
| Customer feedback/NPS | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Customer portal | Planned | ❌ | ✅ (Experience Cloud) | ✅ | ✅ | ✅ (Portals) | ❌ | ✅ (Customer Portal) |
| Knowledge base integration | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Proactive outreach automation | Planned | ❌ | ✅ (Einstein) | ✅ | ✅ | ✅ (Sequences) | ❌ | ✅ (Workflows) |

---

## Competitive Positioning

### Where RERP Wins
- **Rust-based livechat concurrency** — Handling 10,000+ concurrent chat sessions in Rust is trivial. Python-based chat (Odoo) struggles at scale.
- **API-defined chat interactions** — Every chat message, agent, and session is an OpenAPI-defined entity. Third-party integrations can consume the chat stream directly.
- **Self-hosted customer success** — No separate customer success platform subscription. Build it into the CRM.

### Where RERP Lags
- **No gamification** — No goals, badges, or leaderboards. Sales motivation tools are missing.
- **No subscription/renewal management** — No recurring revenue tracking, renewal alerts, or churn prediction.
- **No helpdesk integration** — No ticket-to-lead sync, no lead-to-ticket sync.
- **No customer health scoring** — No proactive churn risk detection.
- **No customer portal** — Customers can't self-serve through a branded portal.

---

## Competitive Intelligence Deep Dive

### Salesforce Service Cloud + Experience Cloud (Enterprise — $25–$300/user/month)
**Service Cloud** handles tickets, cases, live chat, omnichannel routing, and knowledge base. **Experience Cloud** provides branded customer self-service portals with custom domains and white-labeling. **Einstein Customer 360** provides real-time customer health scoring based on engagement, support interactions, and purchase history. **Omni-Channel Routing** distributes cases across chat, email, phone (via **Einstein Voice**), and social media based on agent skill, availability, and workload. **Knowledge Base** integrated with support cases — reps get inline article suggestions while responding. **CSAT/NPS** surveys automated post-resolution with trend tracking. **Field Service** integration adds on-site visit scheduling and technician dispatch. **Community Portal** lets customers log cases, track status, and search knowledge base without rep involvement. **Omni-Channel** handles 100M+ interactions/year with SLA management and escalation routing.

### HubSpot Conversations (SMB Engagement — $20–$1,800+/month)
**Chat** is built into the CRM — no separate chat tool, no JS widget to manage separately. **Chatflows** create automated conversation paths with qualification questions before routing to a human. **Conversations View** aggregates all customer interactions (email, chat, call, meeting) in chronological order. **Tickets** convert chats and emails into support cases with SLA tracking. **Sequences** automate proactive outreach based on customer behavior (e.g., "customer viewed pricing page 3 times → send intro email"). **Customer Portal** is part of HubSpot CMS — branded self-service with case tracking. **Ticketing** supports priority levels, assignment rules, and SLA timers. **AI-powered suggestions** recommend next best action for support reps. The integration between sales, marketing, and service is seamless because they're the same product.

### Zendesk (Customer Service Leader — $19–$115/user/month)
**Zendesk** is the leading customer service platform. **AI-powered routing** (Zendesk AI) classifies and routes tickets to the right agent. **Self-Service Portal** with knowledge base and community forums. **Customer Feedback** module tracks NPS/CSAT with automated surveys and trend dashboards. **Knowledge Base** with article analytics (which articles solve problems, which don't). **Advanced Analytics** with real-time agent performance, queue management, and SLA reporting. **Zendesk CRM** now includes lead management, deal tracking, and pipeline views — filling the gap between service and sales. **Voice** integrates with Twilio for inbound/outbound calling. **Messaging** supports WhatsApp, Facebook Messenger, and SMS. Best for organizations where customer service is the primary engagement surface.

### Intercom (Messaging-First Engagement — $49–$189+/month)
**Intercom** is the messaging platform for customer engagement. **Messenger** provides livechat with real-time typing indicators and instant response. **Fin AI** is an AI chatbot that resolves common queries without human intervention (70%+ deflection rate for simple queries). **Steps** creates in-app messaging and onboarding sequences based on user behavior. **Conversations** aggregates all messaging (chat, email, push, in-app) in one thread per user. **Segmentation** enables targeted outreach based on user behavior, company, plan, and engagement level. **Customer Success** provides health scores (usage-based, behavioral, and survey-derived) and churn prediction. **Fin Agent** assists human agents with suggested responses based on conversation context. Best for SaaS companies who want proactive customer engagement and product-led growth.

### Pipedrive (Minimal Engagement — $15–$99/user/month)
Pipedrive has **no native engagement features**. **Wavebot** is a basic chat widget for collecting leads (not for ongoing conversation). **Activities** track calls and meetings but don't include livechat or messaging. No helpdesk, no subscriptions, no gamification, no health scoring. The philosophy is: "manage your pipeline, use other tools for everything else." Connects to Zendesk, Intercom, and 500+ other tools via Zapier for engagement features.

---

## RERP CRM Implementation Roadmap

### Phase 1 (Immediate — 2-3 weeks)
1. Define `ChatSession` entity: id, agent_id, contact_id, status, started_at, ended_at, transcript
2. Enhance `Agent` entity: is_online, active_sessions, max_concurrent
3. Implement chat session endpoint (POST /chats, GET /chats/{id}/transcript)
4. Implement agent status endpoint (PUT /agents/{id}/status)
5. Implement chat-to-lead conversion endpoint

### Phase 2 (3-6 weeks)
1. Define `Subscription` entity: id, opportunity_id, status, start_date, end_date, renewal_date, value
2. Define `RenewalAlert` entity: id, subscription_id, alert_date, status, assigned_to
3. Implement renewal tracking endpoint (GET /subscriptions/{id}/status)
4. Implement renewal alert endpoint (POST /renewals/alerts, GET /renewals/upcoming)
5. Implement renewal notification endpoint

### Phase 3 (6-12 weeks)
1. Gamification: Define `Goal` entity and `Badge` entity
2. Implement goal tracking endpoint (GET /goals, PUT /goals/{id}/progress)
3. Implement leaderboard endpoint (GET /leaderboard)
4. Helpdesk integration endpoint (bidirectional lead-ticket sync)
5. Customer health score endpoint (GET /customers/{id}/health)

---

## Key Takeaway for Buyers

Customer engagement extends the CRM beyond the sale. A buyer needs to know: *"Can I manage renewals, track customer health, and provide support — without buying a separate customer success platform?"* RERP's advantage is that everything is in one system: leads become customers, customers become renewals, renewals become referrals. The tradeoff: building a customer success layer from scratch takes engineering cycles that Salesforce, HubSpot, and Zendesk have years of head start on. The highest-priority engagement features: **livechat** (already partially implemented) and **subscription/renewal tracking** (critical for SaaS businesses).
