# Bank Feeds & Reconciliation

> **Component:** Bank account connections, statement imports, smart matching, auto-reconciliation, and cash position tracking
> **Priority:** P1 — Auto-reconciliation saves hours per week and is the #1 daily accounting workflow
> **Odoo Reference:** account.bank.statement (1,500+ lines), account.bank.statement.line (1,000+ lines), account.journal.reconciliation.widget

---

## The Pitch

**Buyer Question:** *Can I connect my bank accounts automatically, import transactions in real-time, and reconcile them against my invoices and payments without manual data entry?*\

Bank reconciliation is the most time-consuming daily accounting task for most organizations. Manual data entry of bank transactions is error-prone and expensive. This component handles the full bank reconciliation lifecycle: connecting to bank accounts via APIs or file imports, importing statements automatically, smart matching of transactions to open invoices/payments, and maintaining real-time cash position visibility.

---

## What This Component Does

Bank Feeds & Reconciliation is the bridge between your accounting books and your actual bank balance. It handles:

1. **Bank Account Connection** — Connect to bank accounts via API (Plaid, TrueLayer, etc.) or file upload
2. **Statement Import** — Automatic or manual import of bank statements (CSV, OFX, QIF, MT940, CAMT.053)
3. **Smart Matching** — AI-powered auto-matching of bank transactions to invoices, payments, and journal entries
4. **Manual Reconciliation** — Interface for reconciling unmatched transactions
5. **Cash Position Tracking** — Real-time visibility into available bank balances
6. **Reconciliation Rules** — Configurable rules for auto-reconciling recurring transactions

---

## Entity Model

### Bank Account Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `partner_id` | Foreign Key: Partner | Yes | Bank institution |
| `name` | String (128) | Yes | Account name (e.g., "Chase Operating") |
| `currency_id` | Foreign Key: Currency | Yes | Account currency |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `journal_id` | Foreign Key: Journal | No | Related journal |
| `account_number` | String (64) | No | Bank account number |
| `sort_code` | String (16) | No | Bank routing/sort code |
| `iban` | String (34) | No | IBAN |
| `swift_code` | String (11) | No | SWIFT/BIC code |
| `balance_start` | Decimal (15,2) | No | Opening balance |
| `balance_end_real` | Decimal (15,2) | Computed | True bank balance |
| `balance_end` | Decimal (15,2) | Computed | Book balance (before reconciliation) |
| `balance_difference` | Decimal (15,2) | Computed | Difference (should be 0) |
| `active` | Boolean | No | Soft delete |
| `bank_sync_active` | Boolean | No | Is bank sync enabled? |
| `bank_feed_status` | Enum: [NOT_CONNECTED, CONNECTED, SYNCING, SYNCED, ERROR] | No | Connection status |
| `last_sync_date` | DateTime | No | Last successful sync |
| `last_sync_number` | Integer | No | Number of transactions synced |
| `bank_feed_token` | String (512) | No | Auth token for API connection |
| `create_uid` | Foreign Key: User | Computed | Creator |
| `create_date` | DateTime | Computed | Created timestamp |

**Total fields: ~22.**

### Bank Statement Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Statement reference |
| `date` | Date | Yes | Statement date |
| `journal_id` | Foreign Key: Journal | Yes | Bank journal |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `currency_id` | Foreign Key: Currency | Yes | Statement currency |
| `balance_start` | Decimal (15,2) | Yes | Opening balance |
| `balance_end_real` | Decimal (15,2) | Yes | Closing balance from bank |
| `balance_difference` | Decimal (15,2) | Computed | Difference from expected |
| `line_count` | Integer | Computed | Number of statement lines |
| `state` | Enum: [DRAFT, POSTED, CANCELLED] | Yes | Statement state |
| `line_ids` | One2Many: Statement Line | Yes | Transaction lines |
| `create_uid` | Foreign Key: User | Computed | Creator |
| `create_date` | DateTime | Computed | Created timestamp |

**Total fields: ~14.**

### Bank Statement Line Entity

Individual bank transactions:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `statement_id` | Foreign Key: Statement | Yes | Parent statement |
| `date` | Date | Yes | Transaction date |
| `name` | String (255) | Yes | Transaction description |
| `ref` | String (255) | No | Transaction reference |
| `amount` | Decimal (15,2) | Yes | Transaction amount (signed) |
| `currency_id` | Foreign Key: Currency | No | Transaction currency |
| `partner_id` | Foreign Key: Partner | No | Counterparty |
| `account_id` | Foreign Key: Account | No | Proposed account |
| `invoice_ids` | Many2Many: Invoice | No | Matched invoices |
| `move_line_ids` | Many2Many: Move Line | No | Matched GL lines |
| `reconciled` | Boolean | Yes | Is this line reconciled? |
| `reconcile_model_id` | Foreign Key: Model | No | Auto-reconciliation model |
| `payment_id` | Foreign Key: Payment | No | Linked payment |
| `move_id` | Foreign Key: Move | No | GL entry (if auto-created) |
| `create_date` | DateTime | Computed | Created timestamp |

**Total fields: ~17.**

### Reconciliation Model Entity

Rules for auto-matching:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Rule name |
| `journal_ids` | Many2Many: Journal | Yes | Applicable journals |
| `account_id` | Foreign Key: Account | Yes | Account to reconcile |
| `reference_match` | String (255) | No | Reference pattern to match |
| `partner_match` | String (255) | No | Partner name pattern |
| `amount_match` | String (64) | No | Amount matching rule |
| `date_range` | String (64) | No | Date range matching |
| `reconcile_type` | Enum: [NORMAL, AUTO, AUTO_INV] | No | Reconciliation type |
| `sequence` | Integer | Yes | Display order |

**Total fields: ~10.**

---

## Entity Relationships

```
account.bank.account
  └── account.bank.statement (statements)
        └── account.bank.statement.line (lines)
              ├── account.invoice (matched invoices)
              ├── account.payment (matched payments)
              ├── account.move.line (matched GL lines)
              └── account.reconciliation.model (matching rules)

account.bank.statement
  └── account.journal (journal_id)     ← Bank journal
```

---

## Required API Endpoints

### Bank Account Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/bank-accounts` | List all bank accounts |
| `GET` | `/bank-accounts/{id}` | Get bank account detail |
| `POST` | `/bank-accounts` | Create bank account |
| `PATCH` | `/bank-accounts/{id}` | Update bank account |
| `POST` | `/bank-accounts/{id}/connect` | Initiate bank API connection |
| `POST` | `/bank-accounts/{id}/sync` | Trigger manual sync |
| `GET` | `/bank-accounts/{id}/status` | Check sync status |

### Statement Import

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/statements/import` | Import bank statement file |
| `POST` | `/statements/auto-import` | Auto-import from bank feed |
| `GET` | `/statements` | List bank statements |
| `GET` | `/statements/{id}` | Get statement detail |
| `POST` | `/statements/{id}/post` | Post statement |

### Reconciliation

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/reconciliation/unreconciled` | List unreconciled transactions |
| `POST` | `/reconciliation/{line_id}/match-invoice/{inv_id}` | Match to invoice |
| `POST` | `/reconciliation/{line_id}/match-payment/{pay_id}` | Match to payment |
| `POST` | `/reconciliation/{line_id}/create-entry` | Create manual entry for unmatched |
| `POST` | `/reconciliation/auto-reconcile` | Run auto-reconciliation |
| `GET` | `/reconciliation/stats` | Reconciliation statistics |

### Cash Position

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/cash/position` | Current cash position across all accounts |
| `GET` | `/cash/forecast` | Cash position forecast (next 30 days) |
| `GET` | `/cash/transaction-history/{account_id}` | Transaction history |

### Reconciliation Rules

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/reconciliation-rules` | List reconciliation rules |
| `POST` | `/reconciliation-rules` | Create rule |
| `PATCH` | `/reconciliation-rules/{id}` | Update rule |

---

## Odoo Technical Patterns to Follow

### Pattern 1: Bank Feeds are Not GL Entries
Odoo's bank statement lines are NOT journal entries. They are a holding layer. Only after reconciliation are GL entries created. This allows manual review before posting.

**Recommendation: RERP should follow this pattern — bank transactions are imported but not immediately posted. Reconciliation creates the actual GL entries.**

### Pattern 2: Smart Matching Uses Multiple Signals
Odoo's auto-matching uses: reference number match, partner name match, amount match, and date range. Confidence scores determine if matching is automatic or flagged for review.

**Recommendation: RERP should implement a scoring system for matches. High-confidence matches auto-reconcile; low-confidence matches are flagged for review.**

### Pattern 3: Reconciliation Widget
Odoo's reconciliation interface shows left side (unreconciled bank lines) and right side (open invoices/payments). User can select items to reconcile. Multi-select allows batch reconciliation.

**Recommendation: RERP should provide a reconciliation API that accepts lists of line IDs and invoice/payment IDs to reconcile together.**

---

## Competitive Positioning

### Where RERP Wins
- **API-native bank feeds** — Bank transactions arrive via API, not email attachments. Structured data from day one.
- **Rust-level matching** — Smart matching across thousands of transactions is instantaneous.
- **Self-hosted, no bank API subscriptions** — No per-bank connection fees. Connect unlimited accounts.

### Where RERP Lags
- **No bank sync entity defined** — Zero fields for bank feeds.
- **No reconciliation engine** — No matching rules or smart matching.
- **No statement import** — No file upload or API integration with banks.

---

## Competitive Intelligence Deep Dive

### Odoo: JAX-Powered Auto-Reconciliation
Odoo uses **JAX (Just Auto-X)** AI for automatic bank reconciliation, matching 80%+ of new bank lines without manual intervention. Matches based on amount, reference, and partner name. **Smart suggestions** learn from manual reconciliations. **Cash management dashboard** shows real-time position across all bank accounts.

### QuickBooks Online: Simple Bank Feeds
QBO imports bank transactions automatically. **Categorization rules** match transactions to accounts. **Bank feed reconciliation** shows side-by-side matching. **Match, suggest, and edit** workflow for unmatched transactions. Simple but effective for basic reconciliation.

### Sage Intacct: Enterprise Bank Rec
Sage Intacct supports **automated bank feeds** from 8,000+ financial institutions. **Bank reconciliation interface** with drag-and-drop matching. **Multi-entity bank management** with centralized cash position. **Cash forecasting** with scenario modeling. **Bank statement parsing** in multiple formats.

### NetSuite: Bank Sync
NetSuite's **Bank Feeds** connect to 6,000+ banks. **Automated download and categorization**. **Bank reconciliation** with auto-suggestions. **Cash management** dashboard with real-time balances. **Multi-currency** with automatic exchange rate application.

### Xero: Automatic Bank Rec
Xero uses **JAX-powered automatic bank reconciliation** for 80%+ match rate. **Smart bank rules** learn from your behavior. **Multi-bank account** support with centralized view. **Cash flow statement** updated in real-time.

### Zoho Books: Bank Feeds
Zoho supports **bank feeds** from Indian banks and international connections. **Auto-matching** by amount and reference. **Transaction rules** for recurring categorization. **Bank reconciliation report** with running balance.

---

## Implementation Roadmap

### Phase 1: Bank Account & Statement Model (2 weeks) — P1
1. Define `BankAccount` entity with sync status tracking
2. Define `BankStatement` entity with opening/closing balances
3. Define `BankStatementLine` entity for individual transactions
4. Implement file-based statement import (CSV, OFX, QIF)
5. Implement basic balance tracking

### Phase 2: Smart Reconciliation (3 weeks) — P1
1. Define `ReconciliationModel` entity with matching rules
2. Implement reference-based auto-matching
3. Implement amount + date range matching
4. Implement partner name pattern matching
5. Build reconciliation endpoint for manual review

### Phase 3: Cash Position & Forecasting (2 weeks) — P2
1. Implement cash position endpoint (sum of all bank balances)
2. Implement 30-day cash forecast (based on incoming/outgoing)
3. Implement transaction history with filters
4. Add bank API integration (Plaid/TrueLayer)

---

## Key Takeaway for Buyers

Bank reconciliation is where theory meets reality. A buyer should ask: *Can I connect my bank accounts and let the system match transactions to my invoices and payments automatically?* RERP's API-first model means bank feeds can integrate with any bank API provider (Plaid, TrueLayer, GoCardless). The gap with Odoo is the JAX-powered AI matching. But for organizations that want full control over reconciliation logic, RERP provides the foundation with complete customization potential.

**The immediate priority: define the BankStatement and BankStatementLine entities with matching rules. This is the bridge between your books and your bank.**
