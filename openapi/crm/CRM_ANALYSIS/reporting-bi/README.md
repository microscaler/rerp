# Reporting & BI

> **Component:** Dashboards, analytical views, and business intelligence
> **Priority:** P2 — Management needs dashboards to justify CRM cost
> **Odoo Reference:** crm_reporting (1,500 lines), digest_data cron, crm_stage aggregation, pipeline_funnel

---

## The Pitch

**Buyer Question:** *Can I understand my entire sales operation at a glance, and can I drill down from any metric to the underlying data?*

Reporting is where management gets value from the CRM. Sales managers need to see pipeline health, individual performance, conversion rates, and forecast accuracy. Executives need revenue trends, win/loss ratios, and time-to-close. A buyer needs to know: *Can I create the reports I need, or do I need to export to a spreadsheet first?*

---

## What This Component Does

1. **Pipeline Dashboard** — Real-time count and revenue at each stage (Kanban summary)
2. **Conversion Funnel** — Leads → Opportunities → Won, with conversion rates per stage
3. **Revenue Forecast** — Monthly/quarterly revenue projections by rep and team
4. **Win/Loss Analysis** — Historical win rates by rep, stage, source, product
5. **Time-to-Close** — Average days from open to close by stage and rep
6. **Lead Source Effectiveness** — Which sources generate the most revenue (not just leads)
7. **Rep Performance Comparison** — Side-by-side comparison of reps on key metrics
8. **Pipeline Health Score** — Stalled deals, aging pipeline, conversion bottlenecks
9. **Custom Report Builder** — Build ad-hoc reports with filters and groupings
10. **Scheduled Reports** — Automated email delivery of key metrics
11. **KPI Digest** — Daily/weekly summary emails to managers
12. **Data Export** — Export any report to CSV/XLSX

---

## Entity Model

### Pipeline Summary Response

This is a computed response (not a stored entity):

| Field | Type | Purpose |
|-------|------|---------|
| `total_opportunities` | Integer | Total open opportunities |
| `total_revenue` | Decimal | Sum of expected_revenue |
| `total_weighted_revenue` | Decimal | Sum of prorated_revenue |
| `stages` | Array of StageSummary | Per-stage breakdown |

**StageSummary:**
| Field | Type | Purpose |
|-------|------|---------|
| `stage_id` | UUID | Stage reference |
| `stage_name` | String | Stage name |
| `count` | Integer | Number of opportunities in this stage |
| `total_revenue` | Decimal | Sum of expected_revenue |
| `weighted_revenue` | Decimal | Sum of prorated_revenue |
| `avg_days_in_stage` | Float | Average days this stage |

### Conversion Rate Analysis

| Field | Type | Purpose |
|-------|------|---------|
| `period` | Enum: [WEEK, MONTH, QUARTER, YEAR] | Time period |
| `period_start` | Date | Period start |
| `period_end` | Date | Period end |
| `leads_created` | Integer | Leads created in period |
| `opportunities_created` | Integer | Opportunities created |
| `opportunities_won` | Integer | Opportunities won |
| `opportunities_lost` | Integer | Opportunities lost |
| `lead_to_opp_rate` | Float | opportunities_created / leads_created |
| `opp_to_won_rate` | Float | opportunities_won / opportunities_created |
| `overall_conversion_rate` | Float | opportunities_won / leads_created |
| `total_won_revenue` | Decimal | Revenue from won opportunities |

### Rep Performance

| Field | Type | Purpose |
|-------|------|---------|
| `user_id` | UUID | Salesperson |
| `user_name` | String | Display name |
| `team_id` | UUID | Team |
| `period` | String | Time period |
| `leads_created` | Integer | Leads they created |
| `leads_assigned` | Integer | Leads assigned to them |
| `opportunities_created` | Integer | Opportunities created |
| `opportunities_active` | Integer | Open opportunities |
| `opportunities_won` | Integer | Won in period |
| `opportunities_lost` | Integer | Lost in period |
| `revenue_expected` | Decimal | Sum of expected_revenue |
| `revenue_won` | Decimal | Sum of won deal values |
| `weighted_pipeline` | Decimal | Sum of prorated_revenue |
| `avg_deal_size` | Decimal | revenue_won / opportunities_won |
| `avg_sales_cycle_days` | Float | Average time to close |
| `quota_attainment` | Float | revenue_won / quota |
| `activity_count` | Integer | Emails, calls, meetings logged |
| `lead_response_time_hours` | Float | Avg hours from assignment to first contact |

### Win/Loss Analysis

| Field | Type | Purpose |
|-------|------|---------|
| `period` | String | Time period |
| `total_closed` | Integer | Won + Lost in period |
| `won_count` | Integer | Won in period |
| `lost_count` | Integer | Lost in period |
| `win_rate` | Float | won_count / total_closed |
| `revenue_won` | Decimal | Total won revenue |
| `revenue_lost` | Decimal | Total lost revenue (potential) |
| `lost_reasons` | Array of ReasonBreakdown | Breakdown by reason |

**ReasonBreakdown:**
| Field | Type | Purpose |
|-------|------|---------|
| `reason_id` | UUID | Lost reason |
| `reason_name` | String | Reason name |
| `count` | Integer | Times this reason occurred |
| `percentage` | Float | count / total_lost |
| `revenue_impact` | Decimal | Total revenue lost to this reason |

### Time-to-Close Analysis

| Field | Type | Purpose |
|-------|------|---------|
| `overall_avg_days` | Float | Average from open to close |
| `by_stage` | Array of StageTime | Avg days per stage |
| `by_rep` | Array of RepTime | Avg days per rep |
| `by_source` | Array of SourceTime | Avg days per lead source |
| `by_product` | Array of ProductTime | Avg days per product |
| `by_size` | Array of SizeTime | Avg days by deal size bracket |

### Forecast Entity

| Field | Type | Purpose |
|-------|------|---------|
| `period` | String | Month/quarter/year |
| `team_id` | UUID | Team |
| `user_id` | UUID | Rep |
| `weighted_pipeline` | Decimal | Sum of prorated_revenue |
| `expected_close_revenue` | Decimal | Revenue expected to close |
| `confidence` | Float (0-100) | Forecast confidence |
| `actual_close_revenue` | Decimal | Revenue actually closed |
| `variance` | Decimal | expected - actual |
| `variance_percentage` | Float | (expected - actual) / expected |

### KPI Digest Entity

| Field | Type | Purpose |
|-------|------|---------|
| `id` | UUID | Primary key |
| `name` | String (128) | Digest name (e.g., "Daily Pipeline") |
| `type` | Enum: [DAILY, WEEKLY, MONTHLY] | Frequency |
| `recipient_ids` | Many2Many: User | Who receives it |
| `metrics` | JSON | Which KPIs to include |
| `is_active` | Boolean | Enable/disable |
| `last_sent` | DateTime | When last sent |
| `schedule_cron` | String | Cron expression |

---

## Required API Endpoints

### Pipeline Analytics

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/analytics/pipeline-summary` | Count and revenue per stage |
| `GET` | `/analytics/pipeline-weighted` | Weighted pipeline by team/rep |
| `GET` | `/analytics/pipeline-health` | Stalled deals, aging, bottlenecks |

### Conversion Analytics

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/analytics/conversion-rates` | Lead→Opp→Won by period |
| `GET` | `/analytics/funnel` | Full funnel visualization data |
| `GET` | `/analytics/lead-sources` | Lead volume and revenue by source |

### Performance Analytics

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/analytics/rep-performance` | Individual rep metrics |
| `GET` | `/analytics/team-performance` | Team-level summary |
| `GET` | `/analytics/leaderboard` | Ranked reps by metric |
| `GET` | `/analytics/comparison` | Side-by-side rep/team comparison |

### Win/Loss & Forecast

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/analytics/win-loss` | Historical win/loss analysis |
| `GET` | `/analytics/time-to-close` | Average close duration breakdown |
| `GET` | `/analytics/forecast/monthly` | Monthly revenue forecast |
| `GET` | `/analytics/forecast/quarterly` | Quarterly revenue forecast |
| `GET` | `/analytics/forecast/accuracy` | Forecast vs actual variance |

### Reports

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/reports/build` | Build custom report with filters |
| `GET` | `/reports/{id}` | Get saved report |
| `POST` | `/reports/schedule` | Schedule report delivery |
| `POST` | `/reports/export` | Export report to CSV/XLSX |
| `GET` | `/digests/{id}` | View KPI digest |

---

## Odoo Technical Patterns to Follow

### Pattern 1: crm_reporting as a Separate Model
Odoo's `crm.reporting` model defines computed fields for analytics — count, expected_revenue, weighted, days, etc. It's not a report endpoint; it's a model that aggregates crm.lead data with GROUP BY operations. This is the key insight: **the analytics model is the source of truth for all reports**.

**Recommendation: RERP should define a `crm_reporting` view or computed entity that aggregates crm.lead data. All report endpoints read from this model.**

### Pattern 2: GROUP BY is the Core Operation
Every report in Odoo CRM is essentially a SQL GROUP BY query on crm.lead:
```sql
SELECT stage_id, COUNT(*), SUM(expected_revenue), SUM(prorated_revenue)
FROM crm.lead
WHERE active = true AND type = 'opportunity'
GROUP BY stage_id
```

**Recommendation: RERP's Rust backend should expose a generic GROUP BY endpoint that takes entity, group_by fields, and aggregations as parameters. This is infinitely more flexible than hardcoded report endpoints.**

### Pattern 3: Digest Cron
Odoo uses a daily cron job (`digest_data`) that generates digest emails with KPIs. The digest configuration is a separate entity that defines recipients and metrics.

**Recommendation: RERP should implement a digest cron that queries the analytics model and sends scheduled emails.**

---

## Competitive Positioning

### Where RERP Wins
- **Rust-level query performance** — Aggregating 1 million leads across stages, teams, and periods in Rust is instant. Python-based reports (Odoo) take seconds or minutes for large datasets.
- **OpenAPI-defined reports** — Report schemas are machine-readable. BI tools can consume the CRM API directly.
- **Self-hosted BI** — No separate analytics subscription. The CRM IS the analytics platform.

### Where RERP Lags
- **No reporting endpoints** — No pivot, graph, funnel, or KPI endpoints exist.
- **No dashboard** — No widget-based dashboard surface.
- **No ad-hoc query** — No way for users to build custom reports without API knowledge.

---

## Competitive Intelligence Deep Dive

### Salesforce Analytics Cloud ($75–$300/user/month add-on)
**Einstein Analytics** provides predictive insights. **Dashboards** with real-time data refresh. **Discovery** lets users ask natural language questions. **CRM Analytics** includes 100+ pre-built templates. **Dataflows** transform and blend data from 3,000+ sources. **Predictive metrics** show forecast confidence intervals.

### Microsoft Power BI ($10–$20/user/month)
**Power BI** integrates directly with Dynamics 365 data — no ETL layer needed. **DAX** formulas for complex calculations. **Natural language Q&A** for ad-hoc queries. **Embedded analytics** in Power Apps. **AI Insights** surface anomalies. Best for Microsoft-centric organizations.

### HubSpot Reporting (included in tiers)
**Dashboard** with pre-built widgets: pipeline, deals, activities, sources, revenue trends. **Custom dashboard** builder with drag-and-drop widgets. **Report builder** for custom reports. **KPI tracking** with historical trends. **AI-powered insights** surface anomalies.

---


### ServiceNow: Performance Analytics & Custom Dashboards
ServiceNow provides **Performance Analytics** (predictive insights, trend analysis, predictive alerting) and custom dashboards within the Service Operations Workspace. Reporting is built on RaptorDB for ultra-fast workflow performance. Sales-specific reporting includes pipeline visibility, forecast accuracy, and revenue attribution. **Gap vs. Salesforce:** No Einstein Analytics (Tableau CRM) with ML-powered insights, no CRM Analytics with forecast management. **Gap vs. Microsoft:** No Power BI integration with D365 Sales data. **Gap vs. HubSpot:** No native dashboards with drag-and-drop customization. **Unique strength:** Cross-functional reporting — pipeline data is visible alongside service metrics, IT performance, and HR data on unified dashboards. Process Mining tracks actual workflow execution for continuous improvement. AI Control Tower monitors AI agent performance alongside business outcomes.
## Implementation Roadmap

### Phase 1: Pipeline Analytics (1-2 weeks)
1. Implement `pipeline-summary` endpoint (count/revenue/weighted by stage)
2. Implement `conversion-rates` endpoint (lead→opp→won)
3. Implement `rep-performance` endpoint
4. Implement `lead-sources` endpoint (volume and revenue)
5. Add pipeline health score endpoint

### Phase 2: Advanced Analytics (2-3 weeks)
1. Implement `win-loss` analysis endpoint
2. Implement `time-to-close` endpoint (by stage, rep, source)
3. Implement `funnel` visualization data endpoint
4. Implement `forecast` monthly/quarterly endpoint
5. Implement `forecast-accuracy` (actual vs expected)

### Phase 3: Custom Reports (3-4 weeks)
1. Implement generic GROUP BY report builder endpoint
2. Implement saved report storage
3. Implement scheduled report delivery
4. Implement KPI digest cron
5. Implement CSV/XLSX export

### Phase 4: Advanced BI (3-4 weeks)
1. Cohort analysis endpoint
2. Territory/map analysis endpoint
3. Anomaly detection endpoint
4. Custom calculated fields endpoint
5. Ad-hoc query builder endpoint

---

## Key Takeaway for Buyers

Reporting is where the investment in CRM pays off for management. RERP's advantage is performance and API accessibility — Rust-level query speed on any dataset size. The disadvantage is that no reporting surface exists today. The first endpoints to build: pipeline summary and conversion rates. Those two give management immediate value.
