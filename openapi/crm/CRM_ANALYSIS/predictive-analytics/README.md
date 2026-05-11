# Predictive Analytics

> **Component:** AI/ML-powered lead scoring, win probability, and deal recommendations
> **Priority:** P2 — Differentiator that separates record-keeping from prediction
> **Odoo Reference:** crm.lead.scoring.frequency (Bayesian PLS), scoring frequency fields, PLS_UPDATE_BATCH cron

---

## The Pitch

**Buyer Question:** *Which leads should my team pursue first, and which deals will actually close?*

In a world where sales reps have limited time and infinite leads, predictive analytics answers: **who to call, when to call, and what to say.** This component covers lead scoring (who's hot vs cold), win probability (which deals will close), revenue forecasting (how much pipeline), and AI-driven recommendations (next best action). It's the difference between a CRM that records history and a CRM that predicts the future.

---

## What This Component Does

1. **Lead Scoring** — Automated probability (0-100) based on historical data
2. **Explainability** — Show the rep WHY a lead scored high (not a black box)
3. **Deal Risk** — Flag at-risk deals based on inactivity, stage slippage, or unusual patterns
4. **Scoring Model Training** — Update scoring frequencies from won/lost history
5. **Next Best Action** — Recommend the optimal next step based on winning deal patterns
6. **Sentiment Analysis** — Analyze email tone for engagement signals (Phase 3)
7. **Churn Prediction** — Predict which customers will churn (Phase 3)

---

## Entity Model

### Scoring Frequency Entity

The foundation of the Bayesian scoring model. Each row captures: "For team X, when field Y had value Z, it resulted in N won deals and M lost deals."

| Field | Type | Purpose |
|-------|------|---------|
| `id` | UUID | Primary key |
| `variable` | String (128) | Field name being scored (e.g., "email_state", "phone_state", "source_id") |
| `value` | String (255) | Field value (e.g., "correct", "incorrect", "google") |
| `won_count` | Integer | Times this combo led to a won deal |
| `lost_count` | Integer | Times this combo led to a lost deal |
| `team_id` | Foreign Key: Team | Optional team scoping (NULL = global) |
| `total` | Integer | Computed: won_count + lost_count |

**Configuration:** Scoring is applied to specific fields defined in a configuration parameter:

| Variable | Field on Lead | Values |
|----------|--------------|--------|
| `phone_state` | phone validation | "correct", "incorrect", "na" |
| `email_state` | email validation | "correct", "incorrect", "na" |
| `country_id` | country | Country ID (1, 2, 3, ...) |
| `source_id` | lead source | Source ID |
| `lang_id` | language | Language ID |
| `tag_ids` | classification tags | Tag IDs |
| `industry_id` | industry | Industry ID |

### Lead Score Entity

| Field | Type | Purpose |
|-------|------|---------|
| `id` | UUID | Primary key |
| `lead_id` | Foreign Key: Lead | The lead being scored |
| `probability` | Float (0-100) | Computed probability |
| `automated_probability` | Float (0-100) | AI-computed probability (same as probability) |
| `is_automated_probability` | Boolean | Override flag |
| `top_factors` | JSON | Top contributing factors (explainability) |
| `computation_date` | DateTime | When this score was computed |
| `total_records` | Integer | Records used for computation |

### Scoring Score Thresholds

| Threshold | Score Range | Label | Color |
|-----------|------------|-------|-------|
| HOT | 75-100 | Hot lead | Green |
| WARM | 40-74 | Warm lead | Yellow |
| COLD | 0-39 | Cold lead | Gray |

---

## The Bayesian Scoring Algorithm

This is Odoo's PLS (Predictive Lead Scoring) — a simplified Bayesian inference that's fully explainable.

### The Math

```
P(Won | Field=Value) = P(Field=Value | Won) × P(Won) / P(Field=Value)

Where:
  P(Won) = total_won_deals / total_deals (prior probability)
  P(Field=Value | Won) = won_count_for_value / total_won_deals
  P(Field=Value) = (won_count + lost_count) / total_records
```

### Score Computation for a Lead

For a lead with fields F = {email_state=correct, phone_state=correct, source=google}:

```
1. Look up each field value in ScoringFrequency table
2. For each: compute P(Won | Field=Value)
3. Combine all probabilities: score = Π P(Won | Fi=Vi) / Σ P(Won | Fi=Vi)
4. Normalize to 0-100 scale
5. Return top 3 contributing factors for explainability
```

### Example

```
Lead: email=correct, phone=correct, source=google, country=US

ScoringFrequency data:
  email_state=correct, team=ALL: won=800, lost=200 → P(Won|email=correct) = 0.80
  phone_state=correct, team=ALL: won=750, lost=250 → P(Won|phone=correct) = 0.75
  source_id=google, team=ALL: won=500, lost=500 → P(Won|source=google) = 0.50
  country_id=US, team=ALL: won=600, lost=400 → P(Won|country=US) = 0.60

Combined:
  P(Won | email=correct ∧ phone=correct ∧ source=google ∧ country=US)
  ≈ 0.80 × 0.75 × 0.50 × 0.60 / Σ(all_combos)
  = 0.18 / (weighted sum)
  = 72.3 (WARM)

Top factors: email_state=correct (+15%), phone_state=correct (+12%), country=US (+8%)
```

### Retraining the Model

```
PLS_UPDATE_BATCH cron job:
  1. Query all closed (won/lost) leads for the period
  2. For each lead, extract field values for configured scoring fields
  3. Update won_count/lost_count in ScoringFrequency table
  4. Recompute all lead scores from scratch
  5. Store new scores in LeadScore table
```

---

## Required API Endpoints

### Scoring Core

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/leads/{id}/score` | Get score for a single lead |
| `POST` | `/leads/score-batch` | Score all leads (triggers recomputation) |
| `GET` | `/leads/{id}/score/explain` | Top contributing factors with weights |
| `GET` | `/scoring/frequencies` | View scoring frequency data |
| `POST` | `/scoring/frequencies/rebuild` | Rebuild from historical data |

### Score Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/scoring/thresholds` | Get hot/warm/cold thresholds |
| `PUT` | `/scoring/thresholds` | Update thresholds |
| `GET` | `/scoring/model-configuration` | View which fields are scored |
| `PUT` | `/scoring/model-configuration` | Update scoring field configuration |
| `GET` | `/scoring/accuracy` | Historical scoring accuracy report |

### Lead Buckets

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/leads/hot` | List all hot leads (score >= 75) |
| `GET` | `/leads/warm` | List all warm leads (40-74) |
| `GET` | `/leads/cold` | List all cold leads (< 40) |
| `GET` | `/leads/ranked` | All leads sorted by score |

---

## Odoo Technical Patterns to Follow

### Pattern 1: Scoring is Cron-Based, Not Real-Time
Odoo's PLS_UPDATE_BATCH runs as a scheduled cron job (daily by default). It doesn't compute scores on-write because scoring requires scanning the entire won/lost history. This is computationally expensive and best done in batch.

**Recommendation: RERP should implement scoring as a batch operation triggered by cron. The API provides `POST /leads/score-batch` to trigger on-demand.**

### Pattern 2: Explainability via Top Factors
Odoo's scoring returns the top contributing factors (fields that pushed the score up or down). This builds trust — the rep sees WHY a lead is scored 82, not just "82."

**Recommendation: Always return top_factors in scoring responses.**

### Pattern 3: Team-Scoped Scoring
Scoring frequencies can be scoped to a specific team. Team A might score differently from Team B based on their historical data. NULL team_id means global scoring.

**Recommendation: RERP should support team-scoped scoring frequencies.**

---

## Competitive Positioning

### Where RERP Wins
- **Bayesian probability with explainability** — Reps see WHY a lead is scored high. This builds trust, not blind faith in a black box.
- **Rust-level computation** — Scoring 100,000 leads with Bayesian inference in Rust is orders of magnitude faster than Python.
- **Self-hosted AI** — No subscription fees. The model runs on your infrastructure.

### Where RERP Lags
- **No scoring model deployed** — Infrastructure for PLS exists conceptually but no endpoints or schemas defined.
- **No next-best-action** — No recommendation engine.
- **No NLP/sentiment analysis** — No email tone or deal sentiment.

---

## Competitive Intelligence Deep Dive

### Salesforce Einstein AI ($100–$330/user/month add-on)
Einstein Lead Score uses ML on 200+ signals with feature importance. Einstein Opportunity Scoring predicts win probability with confidence intervals. Einstein Next Best Action recommends optimal next step based on patterns from 10M+ similar deals. Email Insight analyzes tone and response likelihood. Einstein Discovery provides natural language explanations.

### Microsoft Copilot for Sales ($30/user/month add-on)
Copilot reads Outlook emails and Teams meetings to generate deal summaries. Copilot Chat lets reps ask "What's blocking this deal?" Meeting Summary generates action items from Teams calls. Predictive Insights flag at-risk deals. Best for Microsoft shops where Copilot is already embedded.

### HubSpot AI (included in Pro/Ent)
Lead Scoring is rules-based with optional AI enhancement. Conversation Intelligence analyzes call recordings. Chatspot is a Copilot-like assistant for deal insights. Deal Predictions flag stalled deals. No separate ML engineering — everything built-in.

### Zoho Zia ($5/user/month add-on)
Lead Scoring, Deal Probability, Email Sentiment, Zia Q&A (natural language analytics), Voice Commands, Anomaly Detection. Best value AI integration.

---

## Implementation Roadmap

### Phase 1: Scoring Foundation (2-3 weeks) — P2
1. Define `LeadScore` entity with lead_id, probability, top_factors, computation_date
2. Define `ScoringFrequency` entity with variable, value, won_count, lost_count, team_id
3. Add `automated_probability` and `is_automated_probability` to Lead entity
4. Implement `POST /leads/score-batch` endpoint
5. Implement scoring frequency rebuild endpoint

### Phase 2: Explainability & Config (2-3 weeks) — P2
1. Implement `GET /leads/{id}/score/explain` (top contributing factors)
2. Implement scoring model configuration endpoint
3. Add hot/warm/cold thresholds
4. Implement lead buckets endpoint (GET /leads/hot, /leads/warm, /leads/cold)
5. Add scoring cron job (scheduled recomputation)

### Phase 3: Advanced Analytics (4-6 weeks) — P2
1. Custom scoring model (user-defined field weights)
2. Deal risk scoring (combine probability with deal age, activity level)
3. Next-best-action recommendations (based on similar winning deals)
4. Anomaly detection (unusual pipeline patterns)
5. Scoring model versioning (track how scores evolve over time)

---

## Key Takeaway for Buyers

Predictive analytics is where CRM becomes a competitive advantage. A buyer doesn't just want to track deals — they want to **win more deals**. RERP's Bayesian approach with explainability is a genuine differentiator: reps can see WHY a lead is scored high, not just a black-box number. The work ahead is building the scoring engine and exposing it via API.
