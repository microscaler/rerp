# Opportunity & Revenue Management

> **Component:** Deal tracking, forecasting, and revenue analytics
> **Competitive Landscape:** Salesforce, Microsoft Dynamics, SAP, HubSpot, Zoho, Pipedrive

## Pitch

**The Question Every Buyer Asks:** *"Can I trust my revenue forecast, and can I see exactly where every dollar of pipeline is headed?"*

Revenue management is where CRM meets finance. If your pipeline forecast doesn't match reality, your CFO stops trusting the sales team, the sales team stops trusting the CRM, and you're left with a spreadsheet and a bad relationship. This component covers deal tracking, revenue calculation, forecasting, and recurring revenue models.

---

## Functional Requirement Matrix

| Feature | RERP CRM | Odoo CRM | Salesforce | Microsoft Dynamics 365 | SAP CRM | HubSpot | Zoho CRM | Pipedrive |
|---------|----------|----------|------------|------------------------|---------|---------|----------|-----------|
| Expected revenue field | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Probability-weighted revenue | Planned | ✅ (prorated_revenue) | ✅ (Expected Amount) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Recurring revenue tracking | Planned | ✅ | ✅ (Revenue Schedule) | ✅ | ✅ | ✅ (Recurring Revenue) | ✅ | ❌ |
| Monthly recurring revenue | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Recurring revenue plans | Planned | ✅ (crm.recurring.plan) | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Revenue schedule/segments | Planned | ❌ | ✅ (Revenue Schedules) | ✅ | ✅ | ✅ | ✅ | ❌ |
| Forecast categories | Planned | ❌ | ✅ (Pipeline/Opps Close) | ✅ | ✅ | ✅ | ✅ | ❌ |
| Forecast rollup by manager | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Quota management | Planned | ❌ | ✅ (Quotas) | ✅ | ✅ | ✅ | ✅ | ❌ |
| Revenue attribution | Planned | ✅ (campaign_id) | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Deal staging/rounding | Planned | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Quote generation | Planned | ❌ | ✅ (CPQ) | ✅ (Quotes) | ✅ | ✅ (Quotes) | ✅ | ✅ |
| Contract management | Planned | ❌ | ✅ (CLM) | ✅ (Contracts) | ✅ | ✅ | ✅ | ✅ |
| Multi-currency | Planned | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Discount management | Planned | ❌ | ✅ (Discount Tiers) | ✅ | ✅ | ✅ | ✅ | ❌ |
| Price book integration | Planned | ❌ | ✅ (Price Books) | ✅ | ✅ | ✅ | ✅ | ❌ |
| Revenue recognition | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Win rate by period | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Pipeline coverage ratio | Planned | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |

---

## Competitive Positioning

### Where RERP Wins
- **Rust precision for financial calculations** — No floating-point surprises. Monetary types are handled with exact decimal arithmetic.
- **OpenAPI-defined revenue schemas** — Revenue schedules, forecast categories, and quota structures are machine-readable and auto-generated for all clients.
- **Self-hosted finance integration** — No per-feature pricing for CPQ or revenue management. Everything is in the codebase.

### Where RERP Lags
- **No revenue fields at all** — The entities exist but have no financial data model. This is the most critical gap after pipeline stages.
- **No forecasting** — Without probability * expected_revenue, there's no forecast. Without forecast categories, there's no pipeline visibility.
- **No recurring revenue models** — No subscription revenue, no revenue schedules, no price books.

---

## Competitive Intelligence Deep Dive

### Salesforce Revenue Cloud (Enterprise — $25–$330/user/month)
Salesforce's **Revenue Cloud** is the enterprise gold standard. **Opportunity Amount × Probability = Expected Revenue** computed in real-time. **Revenue Schedules** split revenue across months (12/24/36) for recurring deals. **Price Books** with tiered pricing and discount schedules. **Quotes** with full CPQ (Configure, Price, Quote) — product bundles, option sets, and approval workflows. **Forecasting** with customizable categories (O/P/Closed/Worst/Best/Pipeline) and quota management at every org level. **Einstein Forecasting** uses ML on historical data to predict revenue shortfalls 90 days out. **Revenue Recognition** automates ASC 606/IFRS 15 compliance. The ecosystem is unmatched: 3,000+ AppExchange apps for revenue management.

### Microsoft Dynamics 365 (Finance Integration — $65–$200/user/month)
Dynamics integrates with **Dynamics 365 Finance & Operations** — quotes flow directly into invoices, orders, and general ledger. **Revenue management** tracks revenue recognition over time with automatic journal entries. **Quote management** includes pricing formulas, discount rules, and margin analysis. **Customer Insights** enriches deal data with financial signals (payment history, credit score). **Sales Hub** includes deal profit/loss tracking per opportunity. Best for organizations needing CRM-to-ERP continuity with full financial audit trails.

### SAP CRM/S/4HANA (Manufacturing B2B — custom pricing)
SAP's CRM handles **complex pricing matrices**, **volume discounts**, **contract-based pricing**, and **multi-channel revenue** (online, retail, wholesale). Deep integration with SAP S/4HANA means every quote flows directly into production planning, inventory allocation, and order fulfillment. **Margin analysis** per deal shows cost-of-goods-sold in real-time. **Revenue recognition** is native — no add-on needed. Best for manufacturing and distribution where CRM directly drives production.

### HubSpot (SMB Revenue — Free → $1,800+/month for Enterprise)
HubSpot's **Recurring Revenue** module tracks MRR/ARR from subscription deals with built-in churn tracking. **Quotes** are simple — create, send via email, accept with e-signature. **Deal pipelines** show revenue at each stage with real-time weighted pipeline calculation. **Forecasting** uses historical close rates per rep to predict revenue. **Revenue Analytics** shows trends over time. The simplicity is the selling point: no CPQ complexity, just "what's the deal worth?" For SMB to mid-market, this is sufficient.

### Zoho CRM (Value Revenue — $14–$52/user/month)
Zoho's **Forecasting** supports quota management, pipeline forecasting, and team-level rollups. **Revenue forecasting** uses historical data to predict monthly/quarterly revenue. **Price Books** support multi-tier pricing per product and customer segment. **Discount approval workflows** require manager sign-off for discounts above thresholds. **Deal tracking** includes margin calculations (revenue - cost). **Zoho Subscriptions** (integrated app) handles recurring revenue, renewals, and dunning management. Best value for mid-market with complex pricing.

### Pipedrive (Simple Revenue — $15–$99/user/month)
Pipedrive shows **Revenue in Pipeline** (total deal value) and **Weighted Revenue** (stage probability × amount). **Revenue widgets** display monthly trends. That's it for native revenue management. No quotes, no recurring revenue, no forecasting. For teams where revenue calculation is handled in a separate tool (QuickBooks, Xero), this is sufficient.

---

## RERP CRM Implementation Roadmap

### Phase 1 (Immediate — 2 weeks)
1. Define revenue fields on Opportunity: `expected_revenue: Monetary`, `probability: Float`, `close_date: Date`
2. Compute `prorated_revenue = expected_revenue * probability / 100`
3. Add `expected_revenue` and `probability` to Lead entity
4. Implement pipeline revenue summary endpoint (total/weighted by stage)

### Phase 2 (2-4 weeks)
1. Define `RecurringPlan` entity: name, number_of_months, sequence
2. Add `recurring_plan_id` and `recurring_revenue` to Opportunity
3. Compute `recurring_revenue_monthly` and `recurring_revenue_monthly_prorated`
4. Implement revenue forecasting endpoint (by month/quarter)

### Phase 3 (4-8 weeks)
1. Multi-currency support (base_currency, exchange_rate)
2. Quote generation endpoint (Opportunity → Quote)
3. Price book endpoint (products, prices, tiers)
4. Forecast categories endpoint
5. Quota management (per rep, per period)

---

## Key Takeaway for Buyers

Revenue management is the bridge between sales and finance. A buyer needs to trust that when their sales team says "$5M in pipeline," the number is real — not a sales rep's guess. RERP's advantage is precision and transparency. The OpenAPI-defined revenue model means every client sees the same numbers, calculated the same way, at sub-millisecond latency. But the work is ahead: the financial data model doesn't exist yet.
