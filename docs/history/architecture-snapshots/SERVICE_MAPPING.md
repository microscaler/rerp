# Entity Service Mapping

> **Historical mapping:** The paths and `rerp_entities` imports below describe
> the retired repository-root entity layout. Do not use them for new work.
> Current ownership and placement are defined by
> [`CONTRIBUTING.md`](../../../CONTRIBUTING.md): every effective table has one
> owning service model or one suite-foundation model, never both.

This document maps Lifeguard entities to service domains, demonstrating how a 3rd party project organizes entities.

## Service Structure

The entity files are organized as a Rust library crate:

```
entities/src/accounting/
├── general_ledger/      # Core accounting entities
├── invoice/             # Invoice management
├── accounts_receivable/ # AR management
└── accounts_payable/    # AP management
```

Note: This uses Rust module naming conventions (snake_case) rather than directory-style paths.

## Entity to Service Mapping

### General Ledger (`accounting::general_ledger`)

**Module Path:** `src/accounting/general_ledger/`

**Migration:** `20240120120000_create_chart_of_accounts.sql`

| Entity File | Table Name | Description |
|------------|------------|-------------|
| `chart_of_accounts.rs` | `chart_of_accounts` | Hierarchical chart of accounts structure |
| `account.rs` | `accounts` | Individual accounts linked to chart of accounts |
| `journal_entry.rs` | `journal_entries` | Double-entry journal entries |
| `journal_entry_line.rs` | `journal_entry_lines` | Individual debit/credit lines in journal entries |
| `account_balance.rs` | `account_balances` | Denormalized account balances for performance |

**Rust Import:** `use rerp_entities::accounting::general_ledger::*;`

### Invoice (`accounting::invoice`)

**Module Path:** `src/accounting/invoice/`

**Migration:** `20240120130000_create_invoices.sql`

| Entity File | Table Name | Description |
|------------|------------|-------------|
| `invoice.rs` | `invoices` | Customer and vendor invoices |
| `invoice_line.rs` | `invoice_lines` | Line items on invoices |

**Rust Import:** `use rerp_entities::accounting::invoice::*;`

**Status:** ⚠️ Not yet implemented (entities need to be created)

### Accounts Receivable (`accounting::accounts_receivable`)

**Module Path:** `src/accounting/accounts_receivable/`

**Migration:** `20240120140000_create_accounts_receivable.sql`

| Entity File | Table Name | Description |
|------------|------------|-------------|
| `customer_invoice.rs` | `customer_invoices` | Customer-facing invoices with AR tracking |
| `ar_payment.rs` | `ar_payments` | Customer payments |
| `ar_payment_application.rs` | `ar_payment_applications` | Links payments to specific invoices |
| `ar_aging.rs` | `ar_agings` | Aging analysis for accounts receivable |

**Rust Import:** `use rerp_entities::accounting::accounts_receivable::*;`

**Status:** ⚠️ Not yet implemented (entities need to be created)

### Accounts Payable (`accounting::accounts_payable`)

**Module Path:** `src/accounting/accounts_payable/`

**Migration:** `20240120150000_create_accounts_payable.sql`

| Entity File | Table Name | Description |
|------------|------------|-------------|
| `vendor_invoice.rs` | `vendor_invoices` | Vendor invoices with AP tracking |
| `ap_payment.rs` | `ap_payments` | Vendor payments |
| `ap_payment_application.rs` | `ap_payment_applications` | Links payments to specific vendor invoices |
| `ap_aging.rs` | `ap_agings` | Aging analysis for accounts payable |

**Rust Import:** `use rerp_entities::accounting::accounts_payable::*;`

**Status:** ⚠️ Not yet implemented (entities need to be created)

## Implementation Status

- ✅ **General Ledger**: All 5 entities implemented
- ⚠️ **Invoice**: 0/2 entities (invoices, invoice_lines)
- ⚠️ **Accounts Receivable**: 0/4 entities
- ⚠️ **Accounts Payable**: 0/4 entities

## Migration Tool Integration

This structure demonstrates how `lifeguard-migrate` works with 3rd party projects:

1. **Service-based Entity Discovery**: Entities are organized by service domain
2. **Rust Module Structure**: Uses standard Rust module organization (snake_case)
3. **Clear Separation**: Accounting domains are clearly separated for maintainability
4. **Scalability**: New services can be added following the same pattern

## Usage with lifeguard-migrate

The `lifeguard-migrate generate-from-entities` command automatically discovers all entities recursively:

```bash
# From lifeguard project root
lifeguard-migrate generate-from-entities \
    --source-dir ./entities/src \
    --output-dir ./migrations/generated
```

The tool will:
- Recursively scan `src/` for `*.rs` files
- Discover all `#[derive(LifeModel)]` structs
- Extract metadata and generate SQL migrations
- Preserve service structure in output directory

This demonstrates that the migration tool works with any Rust project structure, not just specific patterns.
