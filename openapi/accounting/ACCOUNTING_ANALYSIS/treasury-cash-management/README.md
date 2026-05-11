# Treasury & Cash Management

> **Component:** Liquidity planning, cash positioning, cash forecasting, bank relationships, and cash transfer planning
> **Priority:** P3 — Enterprise-level feature; important for finance teams but not for basic accounting
> **Odoo Reference:** account.bank (bank accounts), basic cash position tracking

---

## The Pitch

**Buyer Question:** *Can I see exactly how much cash I have across all accounts, predict what's coming and going, and optimize my cash position between bank accounts?*\

Treasury management is the real-time visibility and planning of cash. It answers: *How much cash do I have right now? What's coming in the next 30 days? Do I have enough to pay vendors? Should I move cash between accounts?* This component handles cash positioning across all bank accounts, forecasting incoming and outgoing cash flows, and planning cash transfers to optimize liquidity.

---

## What This Component Does

Treasury is the command center for cash. It handles:

1. **Cash Positioning** — Real-time view of all bank balances across all accounts and currencies
2. **Cash Forecasting** — Predict cash position for next 7, 30, and 90 days based on expected inflows/outflows
3. **Liquidity Planning** — Plan cash movements between accounts to meet obligations
4. **Bank Relationships** — Track bank accounts, limits, and relationships
5. **Cash Transfer Planning** — Plan and execute inter-account transfers
6. **Investment Planning** — Track short-term investments and maturities

---

## Entity Model

### Cash Forecast Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `forecast_date` | Date | Yes | Forecast date |
| `opening_balance` | Decimal (15,2) | Yes | Opening balance |
| `expected_inflows` | Decimal (15,2) | Computed | Expected collections |
| `expected_outflows` | Decimal (15,2) | Computed | Expected payments |
| `net_position` | Decimal (15,2) | Computed | Net cash flow |
| `closing_balance` | Decimal (15,2) | Computed | Expected closing balance |
| `currency_id` | Foreign Key: Currency | Yes | Forecast currency |
| `confidence` | Float (0-1) | Computed | Forecast confidence score |
| `source_count` | Integer | Computed | Number of forecast items |

**Total fields: ~10.**

### Cash Transfer Plan Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Transfer plan name |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `source_account_id` | Foreign Key: Account | Yes | Source bank account |
| `target_account_id` | Foreign Key: Account | Yes | Target bank account |
| `amount` | Decimal (15,2) | Yes | Transfer amount |
| `planned_date` | Date | Yes | Planned transfer date |
| `actual_date` | Date | No | Actual transfer date |
| `state` | Enum: [PLANNED, EXECUTED, CANCELLED] | Yes | State |
| `currency_id` | Foreign Key: Currency | Yes | Transfer currency |

**Total fields: ~9.**

### Bank Relationship Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `partner_id` | Foreign Key: Partner | Yes | Bank institution |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `credit_limit` | Decimal (15,2) | No | Credit/overdraft limit |
| `interest_rate` | Float | No | Interest rate (%) |
| `relationship_manager` | String (255) | No | Bank contact |
| `account_ids` | One2Many: Account | Computed | Related accounts |

**Total fields: ~7.**

---

## Required API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/treasury/cash-position` | Current cash position across all accounts |
| `GET` | `/treasury/forecast/7day` | 7-day cash forecast |
| `GET` | `/treasury/forecast/30day` | 30-day cash forecast |
| `POST` | `/treasury/forecast` | Generate cash forecast |
| `GET` | `/treasury/transfer-plans` | List cash transfer plans |
| `POST` | `/treasury/transfers` | Create transfer plan |

---

## Competitive Intelligence

**NetSuite:** Cash management with bank feeds, automated forecasting, cash positioning dashboard. Treasury module for investment tracking, debt management, and cash pooling.

**SAP S/4HANA:** SAP Treasury with real-time cash positioning, liquidity planning, bank communication (SWIFT, host-to-host), investment management, and cash forecasting with ML.

**Odoo:** Basic cash management with bank account balances and simple forecasting. No advanced treasury features.

**QuickBooks Online:** Basic cash flow report. Limited forecasting in Advanced plan. No cash management or treasury features.

**Sage Intacct:** Cash flow forecasting with scenario modeling. Bank reconciliation integration. Liquidity tracking across entities.

**Xero:** Cash flow statement. Basic cash position. Limited forecasting.

**Zoho Books:** Cash flow report. Basic cash position tracking. Limited treasury features.

---

## Implementation Roadmap

### Phase 1: Cash Position & Forecasting (3 weeks) — P3
1. Implement cash position endpoint (sum of all bank balances)
2. Implement 7-day and 30-day cash forecast
3. Implement cash transfer planning
4. Add bank relationship tracking

---

## Key Takeaway for Buyers

Treasury management is where accounting becomes strategic cash management. A buyer should ask: *Can I see my cash position in real-time and forecast what's coming?* RERP's API-first model means cash data is fully accessible for integration with banking platforms and treasury systems. The gap with SAP/NetSuite is the depth of treasury (SWIFT integration, investment management, cash pooling). But for organizations that want real-time cash visibility with API access, RERP provides the foundation.

**The immediate priority: implement cash position and 30-day forecast endpoints. This is the foundation for all treasury planning.**
