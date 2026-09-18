# Revenue & Expense Recognition

> **Component:** Deferred revenue/expense management, recognition schedules, ASC 606/IFRS 15 compliance, and cut-off adjustments
> **Priority:** P2 — Essential for SaaS, subscription, and contract-based businesses
> **Odoo Reference:** account.asset (deferred revenue via assets), account.move (reversal entries)

---

## The Pitch

**Buyer Question:** *Can I recognize revenue and expenses over the correct periods, comply with ASC 606/IFRS 15, and generate the required journal entries automatically?*\

Revenue and expense recognition is where accounting principles meet business reality. A customer pays $12,000 upfront for a 12-month subscription — but the revenue belongs to 12 different months. Recognizing it all in month 1 inflates revenue and understates future months. This component handles deferred revenue/expense scheduling, recognition rule automation, cut-off adjustments, and the journal entries that make financial statements accurate under GAAP/IFRS.

---

## What This Component Does

Revenue & Expense Recognition ensures financial statements reflect the correct economics for each period. It handles:

1. **Deferred Revenue** — Recognize revenue ratably over contract periods (not when cash is received)
2. **Deferred Expenses** — Recognize expenses ratably over受益 periods (prepaid expenses)
3. **Recognition Schedules** — Automated schedules based on time, usage, or performance milestones
4. **ASC 606/IFRS 15 Compliance** — Five-step model: identify contract, identify obligations, determine price, allocate, recognize
5. **Cut-Off Adjustments** — Period-end entries for services provided but not yet invoiced (accruals)
6. **Reclassification Entries** — Monthly reclass between deferral and revenue accounts

---

## Entity Model

### Recognition Schedule Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Schedule name |
| `type` | Enum: [DEFERRED_REVENUE, DEFERRED_EXPENSE, AMORTIZATION] | Yes | Recognition type |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `currency_id` | Foreign Key: Currency | Yes | Schedule currency |
| `total_amount` | Decimal (15,2) | Yes | Total amount to recognize |
| `recognized_amount` | Decimal (15,2) | Computed | Amount recognized to date |
| `remaining_amount` | Decimal (15,2) | Computed | Amount remaining |
| `start_date` | Date | Yes | Recognition start |
| `end_date` | Date | Yes | Recognition end |
| `frequency` | Enum: [MONTHLY, QUARTERLY, ANNUALLY, CUSTOM] | Yes | Recognition frequency |
| `method` | Enum: [LINEAR, USAGE_BASED, MILESTONE, PROPORTIONAL] | Yes | Recognition method |
| `state` | Enum: [DRAFT, ACTIVE, COMPLETED, CANCELLED] | Yes | Schedule state |
| `source_invoice_id` | Foreign Key: Invoice | No | Source transaction |
| `source_journal_id` | Foreign Key: Journal | No | Source journal |
| `deferred_account_id` | Foreign Key: Account | Yes | Deferred revenue/expense account |
| `revenue_account_id` | Foreign Key: Account | Yes | Revenue/expense account |
| `company_id` | Foreign Key: Company | Yes | Owner company |

**Total fields: ~17.**

### Recognition Entry Entity

Individual recognition journal entries:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `schedule_id` | Foreign Key: Schedule | Yes | Parent schedule |
| `date` | Date | Yes | Recognition date |
| `period_id` | Foreign Key: Period | Yes | Fiscal period |
| `amount` | Decimal (15,2) | Yes | Amount to recognize |
| `move_id` | Foreign Key: Move | No | Generated journal entry |
| `state` | Enum: [DRAFT, POSTED, CANCELLED] | Yes | Entry state |
| `entry_type` | Enum: [RECOGNITION, RECLASS, CUTOFF, ADJUSTMENT] | Yes | Entry type |
| `narration` | Text | No | Entry description |

**Total fields: ~10.**

### Recognition Rule Entity

Automated recognition rules:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Rule name |
| `account_id` | Foreign Key: Account | Yes | Revenue/expense account |
| `deferred_account_id` | Foreign Key: Account | Yes | Deferred account |
| `method` | Enum: [LINEAR, USAGE_BASED, MILESTONE] | Yes | Recognition method |
| `period` | Enum: [MONTHLY, QUARTERLY, ANNUALLY] | Yes | Recognition period |
| `proration` | Boolean | Yes | Prorate partial months? |
| `auto_create` | Boolean | Yes | Auto-create schedule on invoice? |

**Total fields: ~8.**

---

## Required API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/schedules` | List recognition schedules |
| `POST` | `/schedules` | Create schedule |
| `GET` | `/schedules/{id}/lines` | View schedule lines |
| `POST` | `/schedules/{id}/run` | Run recognition for period |
| `POST` | `/schedules/run-all` | Run all active schedules |
| `GET` | `/schedules/deferred-balance` | Total deferred balance |
| `POST` | `/schedules/adjust` | Manual cut-off adjustment |

---

## Competitive Intelligence

**NetSuite:** Revenue Management module with ASC 606/IFRS 15 automation. Revenue rules, performance obligations, allocation of transaction price. Subscription billing with revenue recognition. Automated journal entries.

**SAP S/4HANA:** Revenue recognition at document level (FICA). Contract accounting for IFRS 15/ASC 606. Integration with SD module. Real-time revenue posting.

**Odoo:** Deferred revenue via asset module. Manual journal entries for recognition. Limited automation — mostly suitable for simple straight-line deferral.

**QuickBooks Online:** Limited revenue recognition. No native ASC 606. Third-party apps add subscription billing and revenue recognition (RevenueWire, Recurring Revenue).

**Sage Intacct:** Revenue recognition with multiple methods. Performance-based recognition. Multi-element arrangements. Automated journal entries. GAAP/IFRS compliant.

**Xero:** No native revenue recognition. Third-party apps (Xero Subscriptions) add recurring billing. Manual journal entries for deferral.

**Zoho Books:** Recurring invoices for revenue. Manual journal entries for deferral. Limited automation.

---

## Implementation Roadmap

### Phase 1: Core Schedule Model (2 weeks) — P2
1. Define `RecognitionSchedule` entity with methods and states
2. Define `RecognitionEntry` entity with journal entry linkage
3. Implement linear recognition calculation
4. Implement schedule execution (period-by-period)

### Phase 2: Automation & Rules (3 weeks) — P2
1. Define `RecognitionRule` entity for automation
2. Implement rule-based schedule creation from invoices
3. Implement cut-off entry generation
4. Add deferred balance reporting

---

## Key Takeaway for Buyers

Revenue recognition is where technical accounting meets business complexity. A buyer should ask: *Can my system recognize revenue correctly over time, handle multi-element contracts, and produce audit-ready entries?* RERP's API-first model means recognition rules are fully programmable and audit trails are complete. The gap with NetSuite/SAP is the depth of ASC 606 automation (performance obligation tracking, contract modification handling). But for organizations that want full control over recognition logic with API access, RERP provides the foundation.

**The immediate priority: define RecognitionSchedule entity with linear recognition. Most businesses start with straight-line deferral.**
