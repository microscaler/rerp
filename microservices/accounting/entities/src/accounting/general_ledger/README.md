# Entity Examples for Migration Generation

This directory contains Lifeguard entity definitions that match the SQL migrations in `../../migrations/original/`.

## Purpose

These entities are used to "dog food" (test) Lifeguard's entity-driven migration generation capabilities. They help identify:
1. What features are missing
2. What works correctly
3. What needs enhancement

## Entities

### Implemented
- `chart_of_accounts.rs` - Chart of Accounts (hierarchical structure)
- `account.rs` - Individual accounts
- `journal_entry.rs` - Journal entries (double-entry bookkeeping)

### TODO
- `journal_entry_line.rs` - Journal entry lines (debit/credit)
- `account_balance.rs` - Account balances (denormalized)
- `invoice.rs` - Invoices
- `invoice_line.rs` - Invoice lines
- `customer_invoice.rs` - Customer invoices (AR)
- `vendor_invoice.rs` - Vendor invoices (AP)
- `ar_payment.rs` - AR payments
- `ap_payment.rs` - AP payments
- And more...

## Missing Features Identified

See `../../lifeguard-derive/SEAORM_LIFEGUARD_MAPPING.md` section 12 for complete documentation.

### Critical Blockers (🔴)
1. **Foreign Key Constraints** - `#[foreign_key = "table(column) ON DELETE action"]`
2. **CHECK Constraints** - `#[check = "expression"]`
3. **Composite Unique Constraints** - `#[composite_unique = ["col1", "col2"]]`
4. **Index Definitions** - `#[index = "name(columns) WHERE condition"]`

### Nice-to-Have (🟡)
5. **Table Comments** - `#[table_comment = "..."]`

## Usage

These entities are not meant to be compiled/run directly. They serve as:
1. **Documentation** - Show what we want to support
2. **Test Cases** - Identify gaps in the tooling
3. **Reference** - Guide implementation of missing features

## Next Steps

1. Implement missing features in `lifeguard-derive`
2. Update entities to use new attributes
3. Generate SQL migrations from entities
4. Compare generated SQL with `migrations/original/`
5. Iterate until they match
