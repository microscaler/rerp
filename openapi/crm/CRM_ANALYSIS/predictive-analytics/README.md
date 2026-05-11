# Predictive Analytics

> **Component:** AI/ML-powered lead scoring, forecasting, and recommendations
> **Competitive Landscape:** Salesforce Einstein, Microsoft Copilot, HubSpot AI, SAP AI Core, Zoho Zia

## Pitch

**The Question Every Buyer Asks:** *"Which leads should my team pursue first, and which deals will actually close?"*

In a world where sales reps have limited time and infinite leads, predictive analytics answers: **who to call, when to call, and what to say.** This component covers lead scoring, win probability, revenue forecasting, and AI-driven recommendations. It's the difference between a CRM that records history and a CRM that predicts the future.

---

## Functional Requirement Matrix

| Feature | RERP CRM | Odoo CRM | Salesforce | Microsoft Dynamics 365 | SAP CRM | HubSpot | Zoho CRM |
|---------|----------|----------|------------|------------------------|---------|---------|----------|
| Manual probability field | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Automated probability scoring | Planned | ✅ (Bayesian PLS) | ✅ (Einstein Lead Score) | ✅ (Copilot) | ✅ | ✅ (Lead Scoring) | ✅ (Zia) |
| Predictive lead scoring model | Planned | ✅ (Bayesian) | ✅ (Einstein) | ✅ (Azure ML) | ✅ | ✅ (ML) | ✅ (ML) |
| Scoring by field values | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Historical win-rate computation | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Scoring model retraining | Planned | ✅ (PLS_UPDATE_BATCH) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Probability tooltip/explanation | Planned | ✅ (scores per field) | ✅ | ✅ | ❌ | ✅ | ✅ |
| Lead scoring thresholds | Planned | ❌ | ✅ | ✅ | ❌ | ✅ | ✅ |
| Lead scoring activity | Planned | ✅ (cron trigger) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Win/loss pattern detection | Planned | ✅ (frequency tables) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Next best action AI | Planned | ❌ | ✅ (Einstein Next Best Action) | ✅ (Copilot) | ✅ | ✅ | ✅ |
| Deal risk scoring | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Sentiment analysis on emails | Planned | ❌ | ✅ | ✅ | ❌ | ✅ | ❌ |
| Churn prediction | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Predictive pipeline forecasting | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Anomaly detection in pipeline | Planned | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ |
| Custom scoring model (user-defined) | Planned | ❌ | ✅ | ✅ | ❌ | ❌ | ✅ |
| Scoring model versioning | Planned | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ |

---

## Competitive Positioning

### Where RERP Wins
- **Bayesian probability with explainability** — Odoo's PLS (Predictive Lead Scoring) uses Bayes' Theorem and shows the user exactly which field values contribute to the score. This is explainable AI, not a black box.
- **Rust-level computation** — Scoring 100,000 leads with Bayesian inference in Rust is orders of magnitude faster than Python-based implementations.
- **Self-hosted AI** — No subscription fees for Einstein or Copilot. The model runs on your infrastructure, on your data.

### Where RERP Lags
- **No scoring model deployed** — The infrastructure for PLS exists conceptually (frequency tables, batch update), but no endpoints or schemas are defined.
- **No next-best-action recommendations** — Salesforce Einstein and HubSpot AI go beyond scoring to tell reps what to do next.
- **No NLP/sentiment analysis** — No analysis of email tone, deal sentiment, or communication quality.

---

## Competitive Intelligence Deep Dive

### Salesforce Einstein AI (Enterprise Standard — $100–$330/user/month add-on)
**Einstein Lead Score** uses ML on 200+ data signals to score leads 1–100, with feature importance showing which fields drive the score. **Einstein Opportunity Scoring** predicts deal win probability with confidence intervals. **Einstein Next Best Action** recommends optimal next step (call, email, meeting) based on patterns from 10M+ similar deals. **Einstein Discovery** provides deeper analytics with natural language explanations ("Deals with multi-year contracts close 3x faster"). **Email Insight** analyzes sent/received emails for sentiment, response likelihood, and missing key data. **Activity Capture** automatically logs emails and calendar events, then uses AI to surface insights. Enterprise tier includes **Predictive Lead Scoring** trained on your org's data. Additional cost on top of core Sales Cloud license.

### Microsoft Copilot for Sales (AI Assistant — $30/user/month add-on)
Copilot reads Outlook emails, Teams meetings, and Dynamics 365 notes to generate **deal summaries**, **next step recommendations**, and **competitive intelligence**. **Copilot Chat** lets reps ask "What's blocking this deal?" and get answers from the full data context (emails, meetings, notes, opportunities). **Meeting Summary** generates action items and follow-ups from Teams calls automatically. **Email Composition** drafts personalized emails based on contact history and deal context. **Predictive Insights** flag at-risk deals based on engagement patterns. **Competitive Win/Loss** analysis identifies win drivers from historical data. Best for Microsoft shops where Copilot is already embedded in Teams/Outlook — the marginal cost is the Copilot add-on.

### HubSpot AI (SMB-Focused — included in Professional/Enterprise tiers)
**Lead Scoring** is simple: assign points for actions (page views, email opens, form fills, deal stage changes). No ML training required — rules-based with optional AI enhancement. **Conversation Intelligence** (part of Sales Hub) analyzes call recordings for keywords, sentiment, and talk-to-listen ratio. **Chatspot** is a Copilot-like assistant for deal insights — ask "What's the status of deal X?" in natural language. **Content Recommendations** suggest next best content based on contact engagement. **Deal Predictions** flag deals at risk based on stalled activity. **AI Writing Assistant** drafts emails, notes, and meeting summaries. No separate ML engineering — everything is built-in.

### Zoho Zia (Value AI — $5/user/month add-on)
**Zia** provides lead scores, deal predictions, email sentiment, voice commands, and anomaly detection. **Lead Scoring** uses ML from historical won/lost data. **Deal Probability** predicts win rate per opportunity. **Email Sentiment** analyzes email tone for engagement signals. **Zia Insights** surface patterns in sales data ("Your Q3 conversion rate drops 40% after July"). **Zia Q&A** lets managers ask natural language questions about pipeline ("Show me deals at risk this quarter"). **Voice Commands** ("Zia, create a task for John to call Acme Corp"). **Anomaly Detection** flags unusual patterns (sudden pipeline drop, atypical deal duration). Best value AI integration at $5/user/month.

---

## RERP CRM Implementation Roadmap

### Phase 1 (Immediate — 2-3 weeks)
1. Define `LeadScore` entity: lead_id, probability, automated_probability, computation_date
2. Define `ScoringFrequency` entity: field, value, won_count, lost_count, team_id
3. Add automated_probability and is_automated_probability to Lead/Opportunity schemas
4. Implement scoring computation endpoint (compute scores for all leads)
5. Implement scoring frequency rebuild endpoint (update from historical data)

### Phase 2 (3-6 weeks)
1. Implement scoring tooltip/explanation endpoint (return top contributing factors)
2. Implement scoring model configuration (which fields to use, thresholds)
3. Add scoring cron job (scheduled recomputation)
4. Implement lead scoring thresholds (Hot/Warm/Cold buckets)
5. Add probability tooltip to Lead/Opportunity detail endpoint

### Phase 3 (6-12 weeks)
1. Custom scoring model (user-defined field weights)
2. Deal risk scoring (combining probability with deal age, activity level)
3. Next-best-action recommendations (based on similar winning deals)
4. Anomaly detection in pipeline (unusual patterns, stalled deals)
5. Scoring model versioning (track how scores evolve)

---

## Key Takeaway for Buyers

Predictive analytics is where CRM becomes a competitive advantage. A buyer doesn't just want to track deals — they want to **win more deals**. RERP's Bayesian approach with explainability is a genuine differentiator: reps can see WHY a lead is scored high, not just a black-box number. This builds trust in the system. The work ahead is building the scoring engine and exposing it via API. Once deployed, RERP can compete with Einstein-level analytics without the enterprise price tag.
