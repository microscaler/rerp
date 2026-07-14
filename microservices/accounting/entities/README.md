# RERP Accounting Entities Library

RERP's accounting domain entities using Lifeguard ORM.

## Project Structure

This library contains all RERP accounting entities organized by service domain:

```
rerp-entities/
├── Cargo.toml          # Project dependencies (Lifeguard, etc.)
├── build.rs            # Entity registry generation
├── src/
│   ├── lib.rs          # Library root
│   └── accounting/
│       ├── mod.rs      # Accounting module
│       ├── general_ledger/
│       │   ├── mod.rs
│       │   ├── chart_of_accounts.rs
│       │   ├── account.rs
│       │   ├── journal_entry.rs
│       │   ├── journal_entry_line.rs
│       │   └── account_balance.rs
│       ├── invoice/
│       │   ├── mod.rs
│       │   ├── invoice.rs
│       │   └── invoice_line.rs
│       ├── accounts_receivable/
│       │   ├── mod.rs
│       │   ├── customer_invoice.rs
│       │   ├── ar_payment.rs
│       │   ├── ar_payment_application.rs
│       │   └── ar_aging.rs
│       ├── accounts_payable/
│       │   ├── mod.rs
│       │   ├── vendor_invoice.rs
│       │   ├── ap_payment.rs
│       │   ├── ap_payment_application.rs
│       │   └── ap_aging.rs
│       ├── bank_sync/
│       │   ├── mod.rs
│       │   ├── bank.rs
│       │   ├── bank_account.rs
│       │   ├── bank_transaction.rs
│       │   ├── bank_statement.rs
│       │   └── bank_reconciliation.rs
│       ├── asset/
│       │   ├── mod.rs
│       │   ├── asset.rs
│       │   ├── asset_category.rs
│       │   ├── asset_depreciation.rs
│       │   └── asset_transaction.rs
│       ├── budget/
│       │   ├── mod.rs
│       │   ├── budget.rs
│       │   ├── budget_version.rs
│       │   ├── budget_period.rs
│       │   ├── budget_line_item.rs
│       │   └── budget_actual.rs
│       ├── edi/
│       │   ├── mod.rs
│       │   ├── edi_document.rs
│       │   ├── edi_format.rs
│       │   ├── edi_mapping.rs
│       │   └── edi_acknowledgment.rs
│       └── financial_reports/
│           ├── mod.rs
│           ├── financial_report.rs
│           ├── report_template.rs
│           ├── report_schedule.rs
│           └── report_data.rs
└── README.md
```

## Usage

This library is used as a dependency in RERP microservices:

```toml
[dependencies]
rerp_entities = { path = "../entities" }
lifeguard = { path = "../../lifeguard" }
```

Then in your microservice code:

```rust
use rerp_entities::accounting::general_ledger::ChartOfAccount;
use lifeguard::{LifeModelTrait, LifeExecutor};

// Access entity metadata
let entity = ChartOfAccount::Entity::default();
println!("Table: {}", entity.table_name());

// Use in database queries
let accounts = ChartOfAccount::find()
    .filter(Expr::col("is_active").eq(true))
    .all(executor)?;
```

## Entity Organization

Entities are organized by service domain:

- **General Ledger** (`accounting::general_ledger`) - Core accounting entities
  - `ChartOfAccount` - Hierarchical chart of accounts
  - `Account` - Individual accounts
  - `JournalEntry` - Double-entry journal entries
  - `JournalEntryLine` - Journal entry line items
  - `AccountBalance` - Denormalized account balances

- **Invoice** (`accounting::invoice`) - Invoice management
  - `Invoice` - Customer and vendor invoices
  - `InvoiceLine` - Invoice line items

- **Accounts Receivable** (`accounting::accounts_receivable`) - AR management
  - `CustomerInvoice` - Customer invoices
  - `ArPayment` - Customer payments
  - `ArPaymentApplication` - Payment applications
  - `ArAging` - Accounts receivable aging

- **Accounts Payable** (`accounting::accounts_payable`) - AP management
  - `VendorInvoice` - Vendor invoices
  - `ApPayment` - Vendor payments
  - `ApPaymentApplication` - Payment applications
  - `ApAging` - Accounts payable aging

- **Bank Sync** (`accounting::bank_sync`) - Bank synchronization
  - `Bank` - Master bank data
  - `BankAccount` - Bank accounts and credit cards
  - `BankTransaction` - Bank transactions
  - `BankStatement` - Bank statements
  - `BankReconciliation` - Reconciliation records

- **Asset** (`accounting::asset`) - Asset management
  - `Asset` - Fixed assets
  - `AssetCategory` - Asset categories
  - `AssetDepreciation` - Depreciation records
  - `AssetTransaction` - Asset transactions

- **Budget** (`accounting::budget`) - Budget planning
  - `Budget` - Budget definitions
  - `BudgetVersion` - Budget versions
  - `BudgetPeriod` - Budget periods
  - `BudgetLineItem` - Budget line items
  - `BudgetActual` - Actual vs budget comparisons

- **EDI** (`accounting::edi`) - EDI processing
  - `EdiDocument` - EDI documents
  - `EdiFormat` - EDI format definitions
  - `EdiMapping` - Field mappings
  - `EdiAcknowledgment` - EDI acknowledgments

- **Financial Reports** (`accounting::financial_reports`) - Financial reporting
  - `FinancialReport` - Report definitions
  - `ReportTemplate` - Report templates
  - `ReportSchedule` - Report schedules
  - `ReportData` - Report data

## Migration Generation

Generate SQL migrations from entities:

```bash
# From the entities directory
cargo run --bin generate-sql
```

This will:
1. Discover all entities in `src/accounting/`
2. Generate SQL CREATE TABLE statements
3. Write migrations to `../../migrations/generated/accounting/{service}/`

The build script (`build.rs`) automatically generates an entity registry that the migration tool uses.

## Building

This is a library crate that can be built independently:

```bash
cd entities
cargo build --lib
```

Note: The entities use types like `serde_json::Value` and `rust_decimal::Decimal`. Some entities use `#[skip_from_row]` for SQL generation purposes when `FromSql` implementations are not yet available.

## Integration with Microservices

Entities are used in RERP microservices via the `rerp_entities` dependency:

```rust
// In microservice implementation crate
use rerp_entities::accounting::general_ledger::Account;
use lifeguard::{LifeModelTrait, LifeExecutor};
use sea_query::Expr;

pub fn list_accounts(executor: &dyn LifeExecutor) -> Result<Vec<Account::Model>, Error> {
    Account::find()
        .filter(Expr::col("is_active").eq(true))
        .all(executor)
}
```

## Entity Registry

The build script generates an entity registry module that:
- Exposes all entities for SQL generation
- Provides metadata for migration tools
- Enables programmatic entity discovery

The registry is automatically included via `include!()` in `src/lib.rs`.

## See Also

- **RERP Preparation Plan**: `../docs/RERP_PREPARATION_PLAN.md`
- **Accounting PRD**: `../docs/ACCOUNTING_SUITE_ENRICHMENT_PRD.md`
- **Bank Account PRD**: `../docs/BANK_ACCOUNT_IMPROVEMENT_PRD.md`
- **Lifeguard ORM**: `../../lifeguard/`
