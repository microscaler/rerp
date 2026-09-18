# Opportunity & Revenue Management

> **Component:** Deal tracking, forecasting, recurring revenue, and pipeline analytics
> **Priority:** P1 — Financial model essential for sales teams and CFO trust
> **Odoo Reference:** crm.lead.revenue fields, crm.recurring.plan (20 lines), prorated_revenue computation

---

## The Pitch

**Buyer Question:** *Can I trust my revenue forecast, and can I see exactly where every dollar of pipeline is headed?*

Revenue management is where CRM meets finance. If your pipeline forecast doesn't match reality, your CFO stops trusting the sales team, the sales team stops trusting the CRM, and you're left with a spreadsheet and a bad relationship. This component covers deal value tracking, revenue calculation (probability-weighted), recurring revenue models, forecasting, and the financial data that makes pipeline visible to the C-suite.

---

## What This Component Does

1. **Deal Value** — Every opportunity has an expected_revenue (deal size)
2. **Weighted Revenue** — expected_revenue × probability = prorated_revenue (realistic view of pipeline)
3. **Recurring Revenue** — Monthly/quarterly/annual recurring revenue (MRR/ARR) for subscription deals
4. **Revenue Schedules** — Split recurring revenue across months for financial reporting
5. **Forecasting** — Aggregate pipeline by stage, by rep, by period (month/quarter/year)
6. **Currency** — Multi-currency support with exchange rates
7. **Win Rate Analytics** — Historical conversion rates by period, rep, stage, source
8. **Pipeline Coverage** — Ratio of total pipeline to quota (e.g., 3x coverage = need $3M pipeline for $1M quota)

---

## Entity Model

### Revenue Fields on Lead/Opportunity

These fields live on the unified crm.lead entity (Type = OPPORTUNITY):

| Field | Type | Computed | Purpose |
|-------|------|----------|---------|
| `expected_revenue` | Decimal(15,2) | No | Expected deal value (manual entry) |
| `probability` | Float (0-100) | Computed from Stage | Win probability from current stage |
| `automated_probability` | Float (0-100) | Computed from AI | Bayesian scoring probability |
| `is_automated_probability` | Boolean | No | Use AI score vs manual probability |
| `prorated_revenue` | Decimal(15,2) | Yes | `expected_revenue × probability / 100` |
| `expected_amount` | Decimal(15,2) | Yes | Same as prorated_revenue (Salesforce terminology) |

### Recurring Revenue Model

For subscription-based businesses (SaaS, managed services):

**RecurringPlan Entity**

| Field | Type | Purpose |
|-------|------|---------|
| `id` | UUID | Primary key |
| `name` | String (128) | Plan name (e.g., "Monthly", "Quarterly", "Annual") |
| `number_of_months` | Integer | Duration in months (1, 3, 12) |
| `sequence` | Integer | Display order |
| `is_recurring` | Boolean | Whether this plan recurs |
| `renewal_automatic` | Boolean | Auto-renew at end |

**Revenue Fields Added by RecurringPlan**

| Field | Type | Computed | Purpose |
|-------|------|----------|---------|
| `recurring_plan_id` | Foreign Key: RecurringPlan | No | Revenue plan type |
| `recurring_revenue` | Decimal(15,2) | No | Total recurring deal value |
| `recurring_revenue_monthly` | Decimal(15,2) | Yes | `recurring_revenue / number_of_months` |
| `recurring_revenue_monthly_prorated` | Decimal(15,2) | Yes | `recurring_revenue_monthly × probability / 100` |
| `recurring_revenue_prorated` | Decimal(15,2) | Yes | Total prorated recurring revenue |
| `close_date` | Date | No | Expected deal close date (used for MRR timing) |
| `date_deadline` | Date | No | Same as close_date; deadline for closing |

### Forecast Entity

| Field | Type | Purpose |
|-------|------|---------|
| `id` | UUID | Primary key |
| `opportunity_id` | Foreign Key: Opportunity | The deal |
| `period` | Enum: [MONTH, QUARTER, YEAR] | Time period for forecast |
| `forecast_month` | Integer | Month number (1-12) |
| `forecast_quarter` | Integer | Quarter number (1-4) |
| `forecast_year` | Integer | Year |
| `category` | Enum: [BEST_CASE, MOST_LIKELY, WORST_CASE, PIPELINE] | Forecast category |
| `expected_amount` | Decimal(15,2) | Forecasted revenue for this period |
| `manager_id` | Foreign Key: User | Manager reviewing forecast |
| `confidence` | Float (0-100) | Confidence in this forecast |

---

## Financial Calculations

### Weighted Pipeline Calculation

```
Total Weighted Pipeline = Σ(opportunity.expected_revenue × stage.probability / 100)
                         = Σ(opportunity.prorated_revenue)

Per Rep = Σ(opportunities where user_id = rep)
Per Team = Σ(opportunities where team_id = team)
Per Stage = Σ(opportunities where stage_id = stage)
Per Period = Σ(opportunities where month(close_date) = X)
```

### MRR Calculation

```
New MRR = Σ(new deals closed this month with recurring_revenue / number_of_months)
Churn MRR = Σ(churned deals' recurring_revenue / number_of_months)
Expansion MRR = Σ(upgrades' additional recurring_revenue)
Net MRR = New MRR + Expansion MRR - Churn MRR
```

### Pipeline Coverage Ratio

```
Pipeline Coverage = Total Weighted Pipeline / Sales Quota

Example:
  Quota: $1,000,000
  Weighted Pipeline: $3,000,000
  Coverage: 3.0x (healthy)

  Typical targets: 2.5x - 4.0x depending on close rate
```

---

## Required API Endpoints

### Revenue Operations

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/pipeline/revenue-summary` | Total, weighted, MRR, ARR pipeline |
| `GET` | `/pipeline/weighted` | Σ(amount × probability) by stage/team/rep |
| `POST` | `/opportunities/{id}/update-revenue` | Update expected_revenue, probability, plan |
| `GET` | `/recurring/forecast` | Monthly recurring revenue forecast |

### Forecasting

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/forecasts/monthly` | Monthly revenue forecast by team/rep |
| `GET` | `/forecasts/quarterly` | Quarterly revenue forecast |
| `GET` | `/forecasts/actual-vs-forecast` | Variance analysis |
| `GET` | `/forecasts/pipeline-coverage` | Pipeline vs quota coverage ratio |

### Win/Loss Analytics

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/analytics/win-rate` | Historical win rate by period/rep/stage |
| `GET` | `/analytics/time-to-close` | Average days from open to close |
| `GET` | `/analytics/source-effectiveness` | Win rate and revenue by lead source |
| `GET` | `/analytics/deal-velocity` | Average days per stage |

---

## Odoo Technical Patterns to Follow

### Pattern 1: Average-Aggregated Float for Probability
Odoo's `probability` field uses `avg_aggregate=True` which means it's included in group-by aggregations. When you group opportunities by team, the probability is averaged. This is important for accurate pipeline forecasting.

**Recommendation: RERP should define probability as a Float with aggregation support.**

### Pattern 2: Computed Monetary Fields
Odoo uses `compute=` methods for `prorated_revenue`, `recurring_revenue_monthly`, etc. These are stored (not just computed on read) so they can be indexed and queried efficiently.

**Recommendation: RERP should define these as computed fields stored in the database, not derived on-read. This enables aggregation queries without runtime computation.**

### Pattern 3: RecurringPlan as Configuration
Odoo's `crm.recurring.plan` is a configuration entity — it defines reusable plan types. Opportunities reference it, but the plan itself is independent. This allows "Annual Plan" to be used across thousands of opportunities without duplication.

**Recommendation: RERP should define RecurringPlan as a separate entity, seeded with common plans (Monthly, Quarterly, Annual), referenced by opportunities.**

---

## Competitive Positioning

### Where RERP Wins
- **Rust precision for financial calculations** — No floating-point surprises. Decimal types handle monetary values exactly.
- **OpenAPI-defined revenue schemas** — Revenue schedules, forecast categories, and quota structures are machine-readable for all clients.
- **Self-hosted finance integration** — No per-feature pricing for CPQ or revenue management. Everything in the codebase.

### Where RERP Lags
- **No revenue fields at all** — Zero financial data model in any entity.
- **No forecasting** — No probability × expected_revenue computation.
- **No recurring revenue** — No subscription models.

---

## Competitive Intelligence Deep Dive

### Salesforce Revenue Cloud ($25–$330/user/month)
Enterprise gold standard. Revenue Cloud includes Revenue Schedules (split across 12/24/36 months), Price Books with tiered pricing, Quotes with full CPQ, Forecasting with categories (O/P/Closed/Worst/Best), and Quota management. Einstein Forecasting uses ML to predict shortfalls 90 days out. ASC 606/IFRS 15 revenue recognition is native.

### Microsoft Dynamics ($65–$200/user/month)
Integrates with Finance & Operations — quotes flow directly into invoices and general ledger. Revenue recognition tracks over time with automatic journal entries. Margin analysis per deal shows COGS.

### HubSpot ($20–$1,800+/month)
Recurring Revenue module tracks MRR/ARR with churn tracking. Quotes are simple — create, send via email, accept with e-signature. Forecasting uses historical close rates. Simplicity is the selling point.

### Zoho ($14–$52/user/month)
Forecasting with quota management and team rollups. Price Books with multi-tier pricing. Discount approval workflows. Zoho Subscriptions handles recurring revenue and dunning.

---


### ServiceNow: CPQ-Integrated Opportunities
ServiceNow Opportunity Management integrates directly with the Product Catalog and Pricing Engine for real-time pricing. Opportunities flow through: Opportunity → Quote → Order. **Logik.ai CPQ** (acquired 2025) provides AI-powered configuration, guided selling, and millisecond quote response. Keysight reports 40% quote processing reduction. CORT eliminated order processing delays; ATN International cut order processing from hours to <1 minute. Pure Storage unified quote-to-cash on one platform. **Revenue forecasting** uses pipeline analysis combined with historical performance via Now Assist AI. **Unique features:** Sales credit allocations (transparent revenue attribution across team members), Channel Partner management (Full permissions vs. Tracking-only), Needs Analysis templates for AI-driven product recommendations. **Gap vs. Salesforce:** No Revenue Objects, no CPQ Cloud with advanced discounting. **Gap vs. Microsoft:** No D365 Sales with embedded financial models.
## Implementation Roadmap

### Phase 1: Revenue Fields (1-2 weeks) — P1
1. Add `expected_revenue`, `probability`, `close_date` to Lead entity
2. Compute `prorated_revenue = expected_revenue × probability / 100`
3. Implement `pipeline/revenue-summary` endpoint (total, weighted by stage/team)
4. Add `date_deadline` for forecast timing

### Phase 2: Recurring Revenue (2-3 weeks) — P1
1. Define `RecurringPlan` entity with seed data (Monthly, Quarterly, Annual)
2. Add `recurring_plan_id`, `recurring_revenue`, `close_date` to Lead
3. Compute MRR and prorated MRR on Lead
4. Implement `recurring/forecast` endpoint
5. Implement pipeline coverage ratio calculation

### Phase 3: Forecasting (2-3 weeks) — P1
1. Define `Forecast` entity with period, category, manager
2. Implement monthly/quarterly forecast endpoints
3. Implement actual-vs-forecast variance endpoint
4. Add forecast by team/rep rollup
5. Pipeline coverage ratio endpoint

### Phase 4: Advanced Revenue (3-4 weeks) — P2
1. Multi-currency support with exchange rates
2. Quote generation endpoint (Opportunity → Quote)
3. Price book endpoint (products, prices, tiers)
4. Win rate by period endpoint
5. Deal velocity tracking (avg days per stage)

---

## Key Takeaway for Buyers

Revenue management is the bridge between sales and finance. A buyer needs to trust that when their sales team says "$5M in pipeline," the number is real — not a guess. RERP's advantage is precision and transparency through OpenAPI-defined revenue models. The immediate priority: add expected_revenue, probability, and prorated_revenue to the Lead entity, then compute the weighted pipeline.
