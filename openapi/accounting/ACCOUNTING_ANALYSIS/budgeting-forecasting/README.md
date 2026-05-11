# Budgeting & Forecasting

> **Component:** Financial planning — budget creation, version management, variance analysis, rolling forecasts, and scenario modeling
> **Priority:** P2 — Important for enterprise/mid-market but not for basic accounting
> **Odoo Reference:** account.budget (budget management), analytical accounting for variance tracking

---

## The Pitch

**Buyer Question:** *Can I plan my finances with budgets, compare actual results against plan, forecast what's coming, and model "what-if" scenarios?*\

Budgeting and forecasting transform accounting from a historical record-keeping function into a strategic planning tool. Without budgets, you don't know if your spending is on track. Without forecasts, you don't know what's coming. This component handles budget creation (top-down, bottom-up, template-based), variance analysis (actual vs budget), rolling forecasts, and scenario planning.

---

## What This Component Does

Budgeting & Forecasting is the bridge between strategy and execution. It handles:

1. **Budget Creation** — Create budgets from scratch, templates, or previous periods
2. **Version Management** — Multiple budget versions (approved, revised, forecast)
3. **Variance Analysis** — Compare actuals vs budget with automatic variance calculation
4. **Rolling Forecasts** — Continuously update forecasts based on actual performance
5. **Scenario Modeling** — Plan multiple scenarios (optimistic, pessimistic, baseline)
6. **Budget Alerts** — Notify when spending exceeds thresholds
7. **Budget Transfer** — Move budget between periods or accounts

---

## Entity Model

### Budget Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Budget name (e.g., "2025 Operating Budget") |
| `code` | String (16) | Yes | Budget code |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `currency_id` | Foreign Key: Currency | Yes | Budget currency |
| `fiscalyear_id` | Foreign Key: Fiscal Year | Yes | Budget year |
| `state` | Enum: [DRAFT, CONFIRMED, CANCELLED] | Yes | Budget state |
| `date_from` | Date | Yes | Budget start |
| `date_to` | Date | Yes | Budget end |
| `actual_total` | Decimal (15,2) | Computed | Total actuals |
| `budget_total` | Decimal (15,2) | Computed | Total budgeted |
| `variance` | Decimal (15,2) | Computed | Actual - Budget |
| `variance_percentage` | Float | Computed | Variance % |
| `create_uid` | Foreign Key: User | Computed | Creator |
| `create_date` | DateTime | Computed | Created timestamp |

**Total fields: ~14.**

### Budget Line Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `budget_id` | Foreign Key: Budget | Yes | Parent budget |
| `account_id` | Foreign Key: Account | Yes | Account |
| `analytic_account_id` | Foreign Key: Analytic Account | No | Cost center |
| `amount` | Decimal (15,2) | Yes | Budgeted amount |
| `amount_actual` | Decimal (15,2) | Computed | Actual amount |
| `variance` | Decimal (15,2) | Computed | Actual - Budgeted |
| `variance_percentage` | Float | Computed | Variance % |
| `period_id` | Foreign Key: Period | Yes | Budget period |
| `sequence` | Integer | Yes | Display order |

**Total fields: ~10.**

### Budget Version Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Version name |
| `budget_id` | Foreign Key: Budget | Yes | Parent budget |
| `version_number` | Integer | Yes | Version number |
| `type` | Enum: [DRAFT, SUBMITTED, APPROVED, REVISED, FORECAST] | Yes | Version type |
| `approved_by` | Foreign Key: User | No | Approver |
| `approved_date` | DateTime | No | Approval date |
| `is_locked` | Boolean | No | Locked (can't modify) |

**Total fields: ~8.**

---

## Required API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/budgets` | List all budgets |
| `GET` | `/budgets/{id}` | Get budget with lines |
| `POST` | `/budgets` | Create budget |
| `PATCH` | `/budgets/{id}` | Update budget |
| `POST` | `/budgets/{id}/confirm` | Confirm budget |
| `GET` | `/budgets/{id}/variance` | Variance analysis |
| `POST` | `/budgets/{id}/forecast` | Generate forecast |
| `GET` | `/budgets/{id}/variance-report` | Variance report |

---

## Competitive Intelligence

**NetSuite:** EPM suite with planning, budgeting, forecasting, close, and narrative reporting. Multi-dimensional budgeting with what-if scenarios. AI-powered forecasting with machine learning.

**SAP S/4HANA:** Integrated planning with real-time data. Predictive analytics. Group reporting across multi-entity. SAP Analytics Cloud for visualization.

**Odoo:** Budget management with account-level budgets and variance tracking. Simple but effective for SMBs. Enterprise adds analytical accounting integration.

**QuickBooks:** Basic budgeting in Advanced plan. Actual vs budget tracking with alerts. Limited: no rolling forecasts, no scenario modeling.

**Sage Intacct:** Multi-dimensional budgeting with unlimited dimensions. Variance analysis with drill-down. Forecasting with trend analysis.

**Xero:** Basic budgeting in Standard/Premium plans. Simple actual vs budget tracking. Limited customization.

**Zoho Books:** Budget creation and tracking. Variance reports. Good value but limited forecasting capabilities.

---

## Competitive Positioning

### Where RERP Wins
- **API-defined budgets** — Budget structures are machine-readable, enabling automated planning tools.
- **Rust-level computation** — Variance analysis across millions of transactions is instant.
- **Self-hosted scenario modeling** — Run unlimited scenarios with no performance limitations.

### Where RERP Lags
- **No budget entities defined** — Zero fields for budgeting.
- **No variance engine** — No comparison logic between actuals and budget.
- **No forecasting models** — No time-series or predictive forecasting.

---

## Implementation Roadmap

### Phase 1: Core Budget Model (2 weeks) — P2
1. Define `Budget` and `BudgetLine` entities
2. Implement budget creation (account-level, period-level)
3. Implement actuals extraction from GL
4. Calculate variance (actual vs budget)

### Phase 2: Variance Analysis & Reports (2 weeks) — P2
1. Implement variance calculation (amount and percentage)
2. Implement budget variance report
3. Implement alerts for threshold breaches
4. Add version management

---

## Key Takeaway for Buyers

Budgeting transforms accounting from reactive to proactive. A buyer should ask: *Can I plan my finances, track actual performance against plan, and adjust forecasts as reality changes?* RERP's API-first model means budget data is fully accessible for integration with BI tools and planning systems. The gap with NetSuite/SAP is the depth of planning (predictive analytics, group consolidation, narrative reporting). But for organizations that want budget transparency with API access, RERP provides the foundation.

**The immediate priority: define Budget and BudgetLine entities with variance calculation against actuals.**
