# General Ledger & Journal Management

> **Component:** The core double-entry bookkeeping engine — chart of accounts, journal entries, debit/credit enforcement, fiscal periods
> **Priority:** P0 — Foundation layer; every other accounting module depends on this
> **Odoo Reference:** account.account (1,200+ lines), account.journal (500 lines), account.move (3,000+ lines)

---

## The Pitch

**Buyer Question:** *Can I maintain a complete, audit-ready double-entry general ledger that enforces accounting fundamentals — every transaction balanced, every entry traceable, every period closed with confidence?*\

If the answer is no, you don't have an accounting system — you have a spreadsheet. The general ledger is the spine of all financial management. Without a proper double-entry model, nothing else works: no bank reconciliation (no entries to match), no financial reporting (no data to aggregate), no tax compliance (no transaction history), no consolidation (no parent-child ledger hierarchy). This component defines the chart of accounts structure, the journal entry model with debit/credit enforcement, fiscal period management, and the audit trail that makes your books defensible.

---

## What This Component Does

General Ledger & Journal Management is the engine room of accounting. It handles:

1. **Chart of Accounts** — Define the hierarchy of accounts (Assets, Liabilities, Equity, Revenue, Expenses) with proper classification
2. **Journal Entries** — Double-entry bookkeeping with mandatory balance (debits = credits) on every entry
3. **Fiscal Periods** — Define open, closed, and locked periods; prevent post-period entries
4. **Audit Trail** — Every journal entry is immutable once posted; changes go through reversals
5. **Multi-Currency** — Record transactions in foreign currency with automatic exchange rate application
6. **Analytic Accounting** — Tag entries with cost centers, departments, projects for segmented reporting
7. **Recurring Entries** — Automate recurring journal entries (monthly depreciation, rent accrual)
8. **Adjusting Entries** — End-of-period accruals, deferrals, and reclassifications

---

## Entity Model

### Chart of Accounts Entity

The COA defines every account used in the business:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `code` | String (64) | Yes | Account code (e.g., "1010", "4000"); unique, indexed |
| `name` | String (255) | Yes | Account name (e.g., "Cash - Operating", "Sales Revenue") |
| `account_type` | Enum: [ASSET_LIABILITY_EQUITY, REVENUE, EXPENSE, EQUITY] | Yes | Classification for financial statement placement |
| `parent_id` | Foreign Key: Account | No | Parent account (hierarchy); null = top-level |
| `child_ids` | One2Many: Account | Computed | Sub-accounts |
| `currency_id` | Foreign Key: Currency | No | Account-level currency; null = company currency |
| `currency_rate` | Float | No | Currency rate for this account |
| `balance` | Decimal (15,2) | Computed | Current balance (sum of all debit/credit lines) |
| `debit` | Decimal (15,2) | Computed | Sum of debit amounts |
| `credit` | Decimal (15,2) | Computed | Sum of credit amounts |
| `active` | Boolean | No | Soft delete (default: true) |
| `internal_type` | Enum: [NORMAL, REVERSE, MERGE] | No | Balance direction |
| `reconcile` | Boolean | No | Allow bank reconciliation |
| `account_subtype` | Enum: [BANK, CASH, RECEIVABLE, PAYABLE, CURRENT, FIXED, INVENTORY, INVESTMENT, CAPITAL, EXPENSE, TAX, EQUITY] | Yes | Detailed classification |
| `tag_ids` | Many2Many: Tag | No | Classification tags (operating, non-operating, etc.) |
| `tax_code` | String (64) | No | Tax code mapping |
| `sequence` | Integer | No | Display order in COA |
| `central_posting` | Boolean | No | Allow direct postings (false = through sub-accounts only) |
| `closed` | Boolean | No | Close account (default: false) |
| `account_group_id` | Foreign Key: Group | No | Group for reporting |
| `group_id` | Foreign Key: Group | No | Group for analysis |
| `special_account` | Boolean | No | Special account (e.g., suspense, clearing) |

**Total fields: ~22.** This is the backbone. Every journal line references one account here.

### Journal Entity

Journals categorize entries by transaction type:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Journal name (e.g., "Sales Invoices", "Bank - Chase") |
| `code` | String (16) | Yes | Journal code (e.g., "SAL", "BANK"); unique |
| `type` | Enum: [SALE, PURCHASE, BANK, CASH, GENERAL, MISC] | Yes | Determines default accounts and behavior |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `currency_id` | Foreign Key: Currency | No | Journal currency; null = company currency |
| `account_journal_partner_id` | Foreign Key: Partner | No | Default counterparty |
| `default_credit_account_id` | Foreign Key: Account | No | Default credit account |
| `default_debit_account_id` | Foreign Key: Account | No | Default debit account |
| `restrict_mode` | Enum: [READ_ONLY, FULL] | No | Control user modifications |
| `lock_date` | Date | No | Date after which entries cannot be modified |
| `lock_date_total` | Date | No | Date after which entries cannot be added or deleted |
| `entry_sequence` | Integer | No | Display order |
| `color` | Integer | No | Kanban color (1-16) |
| `active` | Boolean | No | Soft delete |

**Total fields: ~15.** Each journal entry belongs to exactly one journal.

### Journal Entry (Move) Entity

The core transaction record — every financial event is an entry:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key (move number) |
| `name` | String (128) | Yes | Entry reference number (auto-generated, e.g., "SAL/2025/0042") |
| `date` | Date | Yes | Entry date (posting date) |
| `period_id` | Foreign Key: Period | Computed | Fiscal period based on date |
| `journal_id` | Foreign Key: Journal | Yes | Source journal |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `currency_id` | Foreign Key: Currency | No | Transaction currency |
| `exchange_rate` | Float | No | Exchange rate (auto from currency) |
| `state` | Enum: [DRAFT, POSTED, CANCELLED] | Yes | Entry lifecycle state |
| `line_ids` | One2Many: Line | Yes | Debit/credit lines (must balance) |
| `total_debit` | Decimal (15,2) | Computed | Sum of all debit lines |
| `total_credit` | Decimal (15,2) | Computed | Sum of all credit lines |
| `difference` | Decimal (15,2) | Computed | total_debit - total_credit (must be 0) |
| `ref` | String (255) | No | Optional reference/memo |
| `narration` | Text | No | Entry description |
| `posted_by` | Foreign Key: User | Computed | User who posted the entry |
| `posted_at` | DateTime | Computed | When entry was posted |
| `move_type` | Enum: [ENTRY, AUTO, AUTO_REVERSE, CORRECTION, CLOSURE] | Yes | Entry type |
| `activity_type` | Enum: [MANUAL, AUTO, BATCH, IMPORT, API] | Yes | Source of entry |
| `activity_status` | Enum: [PENDING, PROCESSED, ERROR] | No | Processing status |
| `activity_date` | Date | No | Expected activity date |
| `auto_id` | Foreign Key: Auto Move | No | Original auto-entry |
| `parent_move_id` | Foreign Key: Move | No | Parent entry (for compound entries) |
| `date_maturity` | Date | No | Maturity date (for financial instruments) |
| `payment_reference` | String (255) | No | Payment reference |
| `payment_date` | Date | No | Payment date |
| `is_tax_recoverable` | Boolean | No | Is this entry tax recoverable? |
| `tax_adjustment_date` | Date | No | Date for tax adjustment |
| `is_move_valid` | Boolean | Computed | True if entry validates (balances, required fields present) |
| `color` | Integer | No | Kanban color (1-16) |
| `is_internal_transfer` | Boolean | No | Is this an internal transfer (no P&L impact)? |
| `create_uid` | Foreign Key: User | Computed | User who created |
| `create_date` | DateTime | Computed | Creation timestamp |
| `write_uid` | Foreign Key: User | Computed | User who last modified |
| `write_date` | DateTime | Computed | Last modification timestamp |

**Total fields: ~30.** Every financial event is captured here.

### Journal Entry Line Entity

Individual debit/credit lines that make up a journal entry:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `move_id` | Foreign Key: Move | Yes | Parent entry |
| `account_id` | Foreign Key: Account | Yes | Account being debited or credited |
| `debit` | Decimal (15,2) | No | Debit amount (one of debit/credit must be > 0) |
| `credit` | Decimal (15,2) | No | Credit amount (one of debit/credit must be > 0) |
| `currency_id` | Foreign Key: Currency | No | Line-level currency |
| `amount_currency` | Decimal (15,2) | No | Amount in foreign currency |
| `currency_rate` | Float | No | Exchange rate for this line |
| `company_currency_id` | Foreign Key: Currency | Computed | Company currency |
| `company_currency_value` | Decimal (15,2) | Computed | Amount in company currency |
| `product_id` | Foreign Key: Product | No | Associated product |
| `name` | String (255) | No | Line description |
| `sequence` | Integer | Yes | Display order within entry |
| `account_id` | Foreign Key: Account | Yes | Account for this line |
| `partner_id` | Foreign Key: Partner | No | Associated partner (customer/vendor) |
| `reconciled` | Boolean | No | Is this line fully reconciled? |
| `reconcile_model_id` | Foreign Key: Model | No | Reconciliation model used |
| `reconcile_partial_id` | Foreign Key: Partial | No | Reconciliation group |
| `tax_line_id` | Foreign Key: Line | No | Tax line (if this is a tax line) |
| `tax_ids` | Many2Many: Tax | No | Taxes applied to this line |
| `tax_amount` | Decimal (15,2) | No | Tax amount for this line |
| `analytic_distribution` | JSON | No | Analytic account distribution (key-value pairs) |
| `date_maturity` | Date | No | Due date for this line |
| `balance` | Decimal (15,2) | Computed | Signed amount (debit positive, credit negative) |
| `amount_residual` | Decimal (15,2) | Computed | Outstanding/unreconciled amount |

**Total fields: ~20.** The actual financial data lives in these lines.

### Fiscal Period Entity

Control when entries can be posted:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (64) | Yes | Period name (e.g., "Jan 2025", "Q1 2025") |
| `code` | String (16) | Yes | Period code (e.g., "01/2025") |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `fiscalyear_id` | Foreign Key: Fiscal Year | Yes | Parent fiscal year |
| `date_from` | Date | Yes | First day of period |
| `date_to` | Date | Yes | Last day of period |
| `state` | Enum: [DRAFT, OPEN, CLOSED] | Yes | Period state |
| `is_adjacent_to_closed` | Boolean | Computed | True if next period is closed |
| `provisional_entries` | Boolean | No | Allow provisional entries |

**Total fields: ~10.** Periods group fiscal years.

### Fiscal Year Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (32) | Yes | Year name (e.g., "2025") |
| `code` | String (4) | Yes | Year code |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `date_from` | Date | Yes | Start of fiscal year |
| `date_to` | Date | Yes | End of fiscal year |
| `state` | Enum: [DRAFT, OPEN, CLOSED] | Yes | Year state |
| `period_ids` | One2Many: Period | Computed | Child periods |

**Total fields: ~8.**

### Recurring Entry Template Entity

Automate recurring journal entries:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Template name (e.g., "Monthly Depreciation - Equipment") |
| `journal_id` | Foreign Key: Journal | Yes | Target journal |
| `line_ids` | One2Many: Template Line | Yes | Lines to copy (accounts, amounts) |
| `interval` | Enum: [DAILY, WEEKLY, MONTHLY, QUARTERLY, YEARLY] | Yes | Frequency |
| `interval_number` | Integer | Yes | Number of intervals (e.g., every 3 months) |
| `number` | Integer | No | Total entries to create (-1 = indefinite) |
| `next_move_date` | Date | Computed | Next entry date |
| `last_move_date` | Date | Computed | Last generated entry date |
| `last_move_id` | Foreign Key: Move | Computed | Last generated entry |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `active` | Boolean | No | Soft delete |

**Total fields: ~13.**

---

## Entity Relationships

```
account.account (chart of accounts)
  ├── account.account (parent_id)              ← Hierarchy: top-level → sub-accounts
  ├── account.move.line (account_id)           ← Journal lines reference accounts
  └── account.journal (default_*/account_id)   ← Journal defaults

account.journal (journals)
  ├── account.move (journal_id)                ← Entries belong to journals
  └── account.move.template (journal_id)       ← Recurring templates

account.move (journal entries)
  ├── account.move.line (move_id)              ← Lines make up entries
  ├── account.fiscal.period (computed via date) ← Period assignment
  └── account.fiscal.year (via period)         ← Year assignment

account.move.line (entry lines)
  ├── account.account (account_id)             ← Account reference
  ├── account.partner (partner_id)             ← Customer/vendor linkage
  ├── account.tax (tax_ids)                    ← Tax lines
  ├── account.analytic.account (analytic_distribution) ← Cost center tagging
  └── account.reconciliation.model (reconcile_model_id) ← Auto-reconciliation rules

account.fiscal.year
  └── account.fiscal.period (period_ids)       ← Years contain periods

account.move.template (recurring entries)
  └── account.move (generated entries)         ← Auto-generates move entries
```

---

## Required API Endpoints

### Chart of Accounts CRUD

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/accounts` | List all accounts with hierarchy |
| `GET` | `/accounts/{id}` | Get account detail with balance |
| `POST` | `/accounts` | Create account |
| `PATCH` | `/accounts/{id}` | Update account |
| `DELETE` | `/accounts/{id}` | Archive account (only if no balances) |
| `GET` | `/accounts/tree` | Get full COA tree hierarchy |
| `GET` | `/accounts/search/{code}` | Search by account code |
| `GET` | `/accounts/similarity` | Find similar account codes |

### Journal Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/journals` | List all journals |
| `GET` | `/journals/{id}` | Get journal detail |
| `POST` | `/journals` | Create journal |
| `PATCH` | `/journals/{id}` | Update journal |
| `DELETE` | `/journals/{id}` | Archive journal (only if no entries) |
| `PUT` | `/journals/{id}/lock` | Set lock dates |

### Journal Entry CRUD

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/entries` | List entries with filters |
| `GET` | `/entries/{id}` | Get entry detail with lines |
| `POST` | `/entries` | Create draft entry |
| `PATCH` | `/entries/{id}` | Update draft entry |
| `POST` | `/entries/{id}/post` | Post entry (validate balance, lock period) |
| `POST` | `/entries/{id}/cancel` | Cancel posted entry (create reversal) |
| `DELETE` | `/entries/{id}` | Delete draft entry |
| `POST` | `/entries/batch` | Create batch of entries |
| `POST` | `/entries/validate` | Validate entry balance without posting |

### Entry Lines

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/entries/{id}/lines` | Add a debit/credit line |
| `PATCH` | `/entries/{id}/lines/{line_id}` | Update line |
| `DELETE` | `/entries/{id}/lines/{line_id}` | Remove line |
| `POST` | `/entries/{id}/reconcile` | Reconcile lines |

### Periods & Years

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/periods` | List fiscal periods |
| `POST` | `/periods` | Create period |
| `PATCH` | `/periods/{id}` | Update period (state: open/closed) |
| `GET` | `/years` | List fiscal years |
| `POST` | `/years` | Create fiscal year |

### Reporting

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/entries/report/ledger` | General ledger report |
| `GET` | `/entries/report/general-ledger` | Trial balance |
| `GET` | `/entries/report/aged-trial-balance` | Aged trial balance |
| `GET` | `/accounts/{id}/history` | Account activity history |
| `GET` | `/entries/report/period-summary` | Period summary report |

### Recurring Entries

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/templates` | List recurring entry templates |
| `POST` | `/templates` | Create template |
| `PATCH` | `/templates/{id}` | Update template |
| `DELETE` | `/templates/{id}` | Delete template |
| `POST` | `/templates/{id}/run` | Manually trigger entry generation |
| `POST` | `/templates/run-all` | Run all active templates |

---

## Odoo Technical Patterns to Follow

### Pattern 1: Double-Entry Balance Enforcement
Odoo enforces that every journal entry has equal debits and credits before allowing `post()`. The `difference` field must be 0. This is a hard constraint — you cannot post an unbalanced entry.

**Recommendation: RERP must enforce `SUM(debit) = SUM(credit)` on every entry before posting. Reject POST requests with `difference != 0`.**

### Pattern 2: Immutable Posted Entries
Odoo never modifies posted entries. Corrections go through reversal entries. You cannot delete a posted entry. You can only cancel it (creates a reverse entry with opposite debits/credits).

**Recommendation: RERP should mark entries as immutable once `POSTED`. All corrections require creating a new entry with `move_type = "CORRECTION"`.**

### Pattern 3: Fiscal Period Locking
Odoo's `lock_date` prevents modification of entries before that date. `lock_date_total` prevents adding/deleting entries. Closed periods cannot be reopened through the UI (requires technical access).

**Recommendation: RERP should enforce `lock_date` on entry creation — reject entries with `date < lock_date`. Period closure should be managed via API endpoint only.**

### Pattern 4: Multi-Company Entries
Odoo's `account.move` has a `company_id` field. Entries can belong to any single company. Inter-company entries are handled through dedicated inter-company journals and automated journal entries.

**Recommendation: RERP should support multi-company from day one. Every entry and line carries a `company_id`. Inter-company transactions use dedicated journals with auto-generation rules.**

### Pattern 5: Analytic Distribution
Odoo's analytic accounts allow tagging entries with dimensions (department, project, cost center). A single journal line can distribute across multiple analytic accounts with percentages.

**Recommendation: RERP should support `analytic_distribution` as a JSON key-value mapping on each line. This enables segmented reporting without complicating the chart of accounts.**

---

## Competitive Positioning

### Where RERP Wins
- **OpenAPI-first, machine-readable data model** — Every entity, field, and relationship is defined in OpenAPI specs. No other accounting platform exposes its data model this cleanly.
- **Rust-level transaction speed** — Balancing 100,000 journal lines in Rust is instantaneous. Python-based systems (Odoo) can be slow with large datasets.
- **Self-hosted, no vendor lock-in** — No per-seat pricing, no rate limits, no data egress fees. Full infrastructure control.
- **API-native, not GUI-native** — Accounting operations are performed via API, making integration trivial. No UI workaround needed.

### Where RERP Lags
- **Zero entity fields defined** — The entire double-entry model is missing. This is the critical first deliverable.
- **No balance enforcement** — Without the entry/line schema, no balance validation is possible.
- **No fiscal period management** — Periods and locking are fundamental to accounting workflows.
- **No audit trail** — Posted entries must be immutable; corrections must go through reversals.

---

## Competitive Intelligence Deep Dive

### Oracle NetSuite: OneWorld Multi-Book
NetSuite's **OneWorld** supports unlimited subsidiaries with different currencies, fiscal calendars, and reporting currencies. **Multi-Book Accounting** auto-posts to multiple books (e.g., US GAAP + IFRS). **SuiteBilling** handles subscription revenue recognition with ASC 606/IFRS 15 compliance. **Analytical Accounting** allows 100+ dimensions per entry. Entry validation is real-time: unbalanced entries are rejected before creation.

### SAP S/4HANA: Real-Time Universal Journal
SAP's **Universal Journal (ACDOCA)** combines GL, AP, AR, CO, AA, and AA in a single table. Every transaction is recorded once, in one place, with every dimension available simultaneously. This eliminates reconciliation between sub-ledgers and GL. **HANA in-memory** enables real-time postings with zero lag. **Fiscal Year Variant** supports up to 12 special periods per year. **Parallel Accounting** supports up to 20 ledger versions simultaneously.

### Odoo: Simplicity with Power
Odoo's `account.move` handles all journal entries through a single model. Lines are in `account.move.line`. The system auto-generates entries from invoices, payments, and transfers. **Smart matching** suggests reconciliations based on reference matching. **Analytic distribution** spreads amounts across cost centers. **Journal types** (Sales, Purchase, Bank, Cash, General) control default behavior. Community edition is fully functional; Enterprise adds bank sync and automated matching.

### QuickBooks Online: Streamlined Simplicity
QBO uses a simpler model — Journal Entries exist but are hidden for most users. The default view is the "Chart of Accounts" with running balances. Entries are created through transaction forms (Invoice, Bill, Expense, Transfer). **Bank feeds** import transactions that need categorization. **Recurring transactions** cover both invoices and journal entries. The simplicity is both QBO's strength (easy for non-accountants) and weakness (limited customization for complex needs).

### Sage Intacct: Multi-Entity Power
Sage Intacct's core is the **GL dimension-based architecture** — every transaction carries up to 100 dimensions for segmented reporting. No need to create thousands of COA accounts. **Multi-entity consolidation** is native — not a module. **Cash flow statements** are generated in real-time. **Journal entry approval workflows** enforce segregation of duties. **Period-end close checklist** guides controllers through the close process.

### Xero: Clean and Simple
Xero's ledger is minimal — accounts, journals, and bank feeds. **Journals** are a simple list of entries with debit/credit lines. **Bank reconciliation** is the primary interface — most entries come from bank feeds that get matched to accounts. **Inventory tracking** is add-on. **Multi-currency** is built-in but limited to 200 currencies. The model is deliberately simple: if your accounting is simple, Xero is elegant. If complex, you'll outgrow it.

### Zoho Books: Value-Forward Accounting
Zoho Books uses a familiar COA structure with account types (Bank, Cash, AR, AP, Revenue, Expense, Fixed Asset, Equity, Loan). **Journals** support manual entries with multi-currency. **Recurring journals** handle depreciation, rent, and amortization. **Approval workflows** require manager approval before posting. **Bank reconciliation** includes auto-matching by amount + reference. **Audit log** tracks every change to accounts and entries. Good value at $15-$125/mo but limited depth compared to NetSuite or SAP.

---

## Implementation Roadmap

### Phase 1: Core COA & Journals (2-3 weeks) — P0
1. Define `Account` entity with full COA hierarchy (parent/child, types, subtypes)
2. Define `Journal` entity with type-based default accounts and lock dates
3. Establish COA tree API (full hierarchy with balances)
4. Implement account search by code, type, and classification
5. Seed default COA for common jurisdictions (US GAAP, IFRS, etc.)

### Phase 2: Journal Entries & Balance Enforcement (3-4 weeks) — P0
1. Define `Move` (journal entry) entity with state machine (DRAFT → POSTED → CANCELLED)
2. Define `MoveLine` entity with debit/credit enforcement
3. Implement balance validation (`SUM(debit) == SUM(credit)` on post)
4. Implement entry posting (transitions state to POSTED, sets timestamps)
5. Implement cancellation via reversal entry creation
6. Establish immutability — no UPDATE on POSTED entries

### Phase 3: Fiscal Periods & Analytics (2-3 weeks) — P1
1. Define `FiscalYear` and `FiscalPeriod` entities
2. Implement period locking (prevent entries before lock date)
3. Implement period state machine (DRAFT → OPEN → CLOSED)
4. Define `AnalyticDistribution` support on lines (JSON key-value)
5. Implement account activity history endpoint

### Phase 4: Recurring Entries & Reporting (3-4 weeks) — P1
1. Define `RecurringEntryTemplate` with frequency, intervals, and lines
2. Implement template execution engine (cron-driven entry generation)
3. Implement general ledger report endpoint (entries grouped by account)
4. Implement trial balance report endpoint
5. Implement aged trial balance endpoint

---

## Key Takeaway for Buyers

RERP Accounting's pitch is **open, fast, and self-hosted** with an **OpenAPI-first data model** that no other accounting platform can match. The double-entry ledger is the most fundamental accounting concept — every other module (AP, AR, bank reconciliation, reporting) depends on it. 

The gap with NetSuite/SAP is the entire ledger infrastructure: COA hierarchy, journal entries with balance enforcement, fiscal periods with locking, and audit trails. But RERP's advantage is that once defined, every client gets the complete model automatically via code generation. No vendor release cycle, no feature gating behind enterprise tiers.

**The immediate priority: define the complete COA, journal, and entry models with balance enforcement. Everything else depends on this foundation.**
