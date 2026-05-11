# Multi-Entity Consolidation

> **Component:** Multi-company consolidation — consolidation groups, elimination rules, inter-company transactions, currency translation, and group reporting
> **Priority:** P2 — Only needed for organizations with 2+ subsidiaries or legal entities
> **Odoo Reference:** Multi-company support in all accounts, inter-company journals, consolidation module (Enterprise only)

---

## The Pitch

**Buyer Question:** *Can I consolidate financial results across multiple companies, eliminate inter-company transactions automatically, and report in multiple currencies?*\

When a business operates across multiple legal entities, locations, or subsidiaries, consolidating the results into a single set of financial statements is essential for management, investors, and regulators. This component handles consolidation groups, inter-company transaction matching and elimination, currency translation (current rate, temporal methods), minority interest calculation, and group reporting.

---

## What This Component Does

Multi-Entity Consolidation transforms scattered company results into unified financial statements. It handles:

1. **Consolidation Groups** — Define which companies are part of the consolidated group
2. **Inter-Company Transactions** — Auto-match and eliminate inter-company receivables/payables, sales/purchases
3. **Currency Translation** — Translate subsidiary financials to reporting currency (current rate or temporal method)
4. **Minority Interest** — Calculate and report minority/non-controlling interest in consolidated results
5. **Consolidation Runs** — Batch process for generating consolidated financial statements
6. **Group Reporting** — Reports that roll up across all entities in the group

---

## Entity Model

### Consolidation Group Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Group name |
| `reporting_currency_id` | Foreign Key: Currency | Yes | Group reporting currency |
| `translation_method` | Enum: [CURRENT_RATE, TEMPORAL] | Yes | Translation method |
| `company_ids` | Many2Many: Company | Yes | Members of group |
| `parent_company_id` | Foreign Key: Company | Yes | Parent/consolidating company |
| `equity_method` | Enum: [FULL, PROPORTIONAL, EQUITY] | Yes | Consolidation method |

**Total fields: ~7.**

### Consolidation Run Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `group_id` | Foreign Key: Group | Yes | Parent group |
| `name` | String (128) | Yes | Run name |
| `date_from` | Date | Yes | Period start |
| `date_to` | Date | Yes | Period end |
| `state` | Enum: [DRAFT, RUNNING, COMPLETED, ERROR] | Yes | Run state |
| `translation_adjustment` | Decimal (15,2) | Computed | Currency translation adjustment |
| `elimination_total` | Decimal (15,2) | Computed | Total eliminated amounts |
| `minority_interest` | Decimal (15,2) | Computed | Minority interest amount |

**Total fields: ~9.**

### Inter-Company Transaction Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `source_company_id` | Foreign Key: Company | Yes | Source company |
| `target_company_id` | Foreign Key: Company | Yes | Target company |
| `source_move_id` | Foreign Key: Move | Yes | Source journal entry |
| `target_move_id` | Foreign Key: Move | No | Target journal entry |
| `amount` | Decimal (15,2) | Yes | Transaction amount |
| `currency_id` | Foreign Key: Currency | Yes | Transaction currency |
| `type` | Enum: [SALE_PURCHASE, RECEIVABLE_PAYABLE, LOAN, DIVIDEND, MANAGEMENT_FEE] | Yes | IC transaction type |
| `reconciled` | Boolean | Yes | Matched between entities? |
| `elimination_entry_id` | Foreign Key: Move | No | Elimination journal entry |
| `reporting_currency_amount` | Decimal (15,2) | Computed | Amount in reporting currency |

**Total fields: ~11.**

### Elimination Rule Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `group_id` | Foreign Key: Group | Yes | Parent group |
| `name` | String (128) | Yes | Rule name |
| `account_ids` | Many2Many: Account | Yes | Accounts to eliminate |
| `transaction_type` | Enum | Yes | Type of IC transaction |
| `elimination_account_id` | Foreign Key: Account | Yes | Elimination account |
| `auto_create_entry` | Boolean | Yes | Auto-create elimination entries? |

**Total fields: ~7.**

---

## Required API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/consolidation/groups` | List consolidation groups |
| `POST` | `/consolidation/groups` | Create group |
| `POST` | `/consolidation/runs` | Execute consolidation run |
| `GET` | `/consolidation/runs/{id}/status` | Check run status |
| `GET` | `/consolidation/runs/{id}/results` | Get consolidated results |
| `GET` | `/inter-company/transactions` | List inter-company transactions |
| `POST` | `/inter-company/match` | Match inter-company transactions |
| `POST` | `/inter-company/eliminate` | Generate elimination entries |

---

## Competitive Intelligence

**NetSuite:** OneWorld provides native multi-subsidiary consolidation with automated inter-company elimination. Currency translation with translation adjustments in equity. Minority interest calculation. Automated consolidation journal entries.

**SAP S/4HANA:** Group reporting with real-time consolidation. Inter-company matching and elimination. Currency translation with GAAPEXCEL/IFRS. Minority interest handling. SAP Group Reporting (cloud) for advanced consolidation.

**Odoo:** Enterprise consolidation module for multi-company. Automated inter-company journal entries. Currency translation. Minority interest. Community edition requires manual consolidation.

**Sage Intacct:** Multi-entity consolidation as a core feature. Automated inter-company elimination. Currency translation. Equity method consolidation. Minority interest. Supports hundreds of entities in minutes.

**QuickBooks Online:** No consolidation support. Manual consolidation required. Third-party apps add basic consolidation (Solomon, Consolidator).

**Xero:** No consolidation support. Manual required. Third-party apps for consolidation.

**Zoho Books:** Multi-company support in Zoho suite. No native consolidation. Requires Zoho Analytics or manual processes.

---

## Implementation Roadmap

### Phase 1: Consolidation Group & Inter-Company (3 weeks) — P2
1. Define `ConsolidationGroup` entity with member companies
2. Define `InterCompanyTransaction` entity with source/target
3. Implement inter-company transaction matching
4. Implement elimination entry generation

### Phase 2: Currency Translation & Consolidation (3 weeks) — P2
1. Implement currency translation (current rate method)
2. Implement consolidation run processor
3. Implement minority interest calculation
4. Generate consolidated Balance Sheet and P&L

---

## Key Takeaway for Buyers

Multi-entity consolidation is a must-have for any organization with subsidiaries. A buyer should ask: *Can my system consolidate results across companies, eliminate inter-company transactions, and handle multiple currencies?* RERP's modular architecture naturally supports multi-entity — each company is a separate `company_id` scope. The gap with NetSuite/SAP/Sage Intacct is the depth of automation (real-time inter-company matching, hundreds of entities, equity method). But for organizations with a handful of entities, RERP provides the foundation with complete API control.

**The immediate priority: define ConsolidationGroup and InterCompanyTransaction entities with elimination entry generation.**
