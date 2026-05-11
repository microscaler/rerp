# Financial Reporting

> **Component:** The full suite of accounting reports — Balance Sheet, Profit & Loss, Cash Flow, Trial Balance, General Ledger, and custom report builder
> **Priority:** P1 — Management needs these reports to validate that the accounting system is accurate and actionable
> **Odoo Reference:** account.report (financial report builder, 2,000+ lines), account.trial.balance.report (1,000+ lines)

---

## The Pitch

**Buyer Question:** *Can I generate standard accounting reports (Balance Sheet, P&L, Cash Flow) instantly, drill down from any line to source transactions, and build custom reports for management analysis?*\

Financial reporting is the final output of all accounting activity. If your reports are wrong, incomplete, or slow to generate, no one trusts the system. This component delivers the full suite of GAAP/IFRS-standard reports with real-time data from the general ledger, drill-down from any report line to source transactions, and a flexible report builder for management-specific analyses.

---

## What This Component Does

Financial Reporting is the face of accounting — the reports that CFOs, controllers, and board members review. It handles:

1. **Balance Sheet** — Assets = Liabilities + Equity, at a point in time
2. **Profit & Loss (Income Statement)** — Revenue minus expenses, over a period
3. **Cash Flow Statement** — Operating, investing, and financing cash flows
4. **Trial Balance** — All accounts with debit/credit balances, must balance to zero
5. **General Ledger Report** — All journal entries grouped by account, with running balance
6. **Aged Trial Balance** — Receivables and payables by aging period (current, 30, 60, 90+)
7. **Custom Report Builder** — Define reports by account type, date range, dimensions
8. **Multi-Currency Reporting** — Reports in multiple currencies with conversion

---

## Entity Model

### Financial Report Entity

Configurable report definition:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Report name |
| `report_type` | Enum: [BALANCE_SHEET, P&L, CASH_FLOW, TRIAL_BALANCE, GL_REPORT, CUSTOM] | Yes | Standard or custom |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `currency_id` | Foreign Key: Currency | Yes | Report currency |
| `date_from` | Date | No | Start date (for P&L, Cash Flow) |
| `date_to` | Date | No | End date |
| `period_type` | Enum: [MONTHLY, QUARTERLY, YEARLY, CUSTOM] | No | Period grouping |
| `display_accounts` | Enum: [ALL, ACTUAL_ONLY, SHOW_NONE] | No | Show accounts with balances only |
| `compare_to` | Enum: [NONE, PRIOR_PERIOD, BUDGET] | No | Comparison type |
| `compare_date_from` | Date | No | Compare period start |
| `compare_date_to` | Date | No | Compare period end |
| `sort_order` | Enum: [NATURAL, CODE, NAME] | No | Report line sorting |
| `column_count` | Integer | No | Number of data columns |
| `column_names` | JSON | No | Column headers |
| `filter_account_type` | JSON | No | Account type filter |
| `filter_analytic` | JSON | No | Analytic dimension filter |
| `is_custom` | Boolean | No | Is this a custom report? |

**Total fields: ~20.**

### Report Line Entity

Individual line in a financial report:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `report_id` | Foreign Key: Report | Yes | Parent report |
| `code` | String (64) | Yes | Line code (e.g., "ASSET", "REV001") |
| `name` | String (255) | Yes | Line display name |
| `type` | Enum: [HEADER, ACCOUNT, TOTAL, SUBTOTAL, COMPARISON] | Yes | Line type |
| `account_ids` | Many2Many: Account | No | Related accounts |
| `account_type` | Enum | No | Account type filter |
| `compute_formula` | Text | No | Custom formula (for custom reports) |
| `level` | Integer | No | Indentation level |
| `balance` | Decimal (15,2) | Computed | Amount for period |
| `balance_compare` | Decimal (15,2) | Computed | Comparison period amount |
| `variation` | Float (0-1) | Computed | Percentage change |
| `parent_line_id` | Foreign Key: Report Line | No | Parent line |
| `sequence` | Integer | Yes | Display order |
| `is_expanded` | Boolean | No | Expanded in UI? |
| `source_line_count` | Integer | Computed | Number of source lines |

**Total fields: ~17.**

---

## Required API Endpoints

### Standard Reports

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/reports/balance-sheet` | Balance Sheet for period |
| `GET` | `/reports/p&l` | Profit & Loss for period |
| `GET` | `/reports/cash-flow` | Cash Flow Statement |
| `GET` | `/reports/trial-balance` | Trial Balance |
| `GET` | `/reports/general-ledger` | General Ledger report |
| `GET` | `/reports/aged-receivables` | Aged AR report |
| `GET` | `/reports/aged-payables` | Aged AP report |

### Report Configuration

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/reports` | List all report definitions |
| `GET` | `/reports/{id}` | Get report definition |
| `POST` | `/reports` | Create custom report |
| `PATCH` | `/reports/{id}` | Update report definition |
| `DELETE` | `/reports/{id}` | Delete custom report |

### Report Execution

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/reports/{id}/run` | Execute report with filters |
| `POST` | `/reports/{id}/export` | Export report to CSV/Excel |
| `GET` | `/reports/{id}/drill-down/{line_code}` | Drill down from report line |
| `GET` | `/reports/{id}/source-transactions/{line_code}` | Source transactions for line |

### Comparison Reports

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/reports/p&l?compare=prior-year` | P&L with prior year comparison |
| `GET` | `/reports/p&l?compare=budget` | P&L with budget variance |
| `GET` | `/reports/balance-sheet?compare=prior-year` | BS with prior year comparison |

---

## Competitive Positioning

### Where RERP Wins
- **API-first report generation** — Reports are data endpoints, not GUI pages. Easy to integrate into dashboards, BI tools, and automated reports.
- **Rust-level computation** — Balance Sheet and P&L across 1 million journal entries compute in milliseconds.
- **OpenAPI-defined report structure** — Report layouts are machine-readable. No proprietary report formats.

### Where RERP Lags
- **No report entities defined** — Zero fields for report generation.
- **No formula engine** — No way to define report line calculations.
- **No drill-down capability** — No link from report line to source transactions.

---

## Competitive Intelligence Deep Dive

### Oracle NetSuite: SuiteAnalytics
NetSuite's **SuiteAnalytics** provides real-time financial reports with drill-down to transactions. **Customizable dashboards** with drag-and-drop. **Predictive analytics** with AI-powered forecasts. **Consolidated reporting** across subsidiaries with currency conversion. **Bolt-on ERP** (now part of NetSuite) for planning and budgeting integration.

### SAP S/4HANA: Real-Time Reporting
SAP's **Universal Journal (ACDOCA)** powers real-time reporting with zero aggregation lag. **Fiori apps** for CFO dashboards, P&L analysis, and cash position. **Group reporting** for consolidated results. **Controlling integration** for management accounting. **Predictive accounting** with ML-powered forecasts.

### Odoo: Simple but Effective
Odoo's financial reports are straightforward: P&L, Balance Sheet, and Trial Balance are standard. **Report builder** for custom reports. **Comparison views** for prior periods. **Drill-down** to journal entries. Simple enough for SMBs; limited customization for complex needs.

### QuickBooks Online: Basic Reports
QBO provides standard reports: Balance Sheet, P&L, Cash Flow, Transaction List, Sales by Customer, Expense by Vendor. **Customizable date ranges and filters**. **Export to CSV/Excel**. **Fathom** (add-on) for advanced analytics and benchmarking.

### Sage Intacct: Enterprise Reporting
Sage Intacct offers **flexible reporting** with drag-and-drop report builder. **Real-time consolidation** across entities. **Dimension-based reporting** (100+ dimensions). **Schedule and distribute** reports automatically. **API access** to all report data. **Cash flow forecasting** integrated with reporting.

### Xero: Basic but Clean
Xero's reports are clean and simple: P&L, Balance Sheet, Cash Flow, Aged Receivables/Payables. **Comparison periods** for variance analysis. **Export to CSV/Excel/PDF**. Limited customization — you work within predefined report templates.

### Zoho Books: Value Reports
Zoho Books provides standard reports: P&L, Balance Sheet, Cash Flow, Trial Balance, Transaction Register. **Variance analysis** (actual vs budget). **Scheduled reports** via email. **Custom report builder** for ad-hoc analysis. Good depth for the price ($15-$125/mo).

---

## Implementation Roadmap

### Phase 1: Core Report Engine (2-3 weeks) — P1
1. Define `FinancialReport` entity with report types and filters
2. Implement Balance Sheet report (assets = liabilities + equity)
3. Implement P&L report (revenue - expenses = net income)
4. Implement Trial Balance report (debit = credit verification)
5. Implement General Ledger report (entries grouped by account)

### Phase 2: Advanced Reports & Drill-Down (3 weeks) — P1
1. Implement Cash Flow Statement (operating, investing, financing)
2. Implement Aged Trial Balance (AR and AP)
3. Implement drill-down from report line to source transactions
4. Add multi-currency report support
5. Implement report export (CSV, Excel)

### Phase 3: Comparison & Custom Reports (3 weeks) — P2
1. Implement comparison reports (prior period, budget variance)
2. Define `ReportLine` entity with formulas and hierarchies
3. Implement custom report builder
4. Add scheduled report delivery
5. Build REST API for all report data

---

## Key Takeaway for Buyers

Financial reporting is how management validates that the accounting system works. A buyer should ask: *Can I generate accurate Balance Sheet, P&L, and Cash Flow statements instantly, and drill down from any line to see the source transactions?* RERP's API-first model means all reports are available as data endpoints — perfect for BI tools, dashboards, and automated reporting. The gap with SAP/NetSuite is the depth of analytics (predictive, group reporting, dimension-based). But for organizations that want clean, accurate, fast reports with API access, RERP delivers the foundation.

**The immediate priority: implement Balance Sheet and P&L reports with drill-down to journal entries. These are the two reports management looks at first.**
