# Migration Generation Gaps Identified

> **Status: HISTORICAL_SNAPSHOT**
>
> This analysis described retired General Ledger predecessor entities. The
> current authoritative ledger is the Accounting foundation documented in
> [`microservices/accounting/README.md`](../../../microservices/accounting/README.md).
> Retained for Lifeguard migration-tooling history only.

This document summarizes the gaps identified when building Lifeguard entities to generate SQL migrations.

## Summary

Created entity examples for:
- `chart_of_accounts.rs` - Chart of Accounts (hierarchical, self-referencing)
- `account.rs` - Individual accounts
- `journal_entry.rs` - Journal entries

## Critical Blockers (🔴)

These features **must** be implemented before entity-driven migration generation can work:

### 1. Foreign Key Constraints
**Status:** ❌ Missing  
**Impact:** Cannot generate relationships between tables

**Required For:**
- `chart_of_accounts.parent_id` → `chart_of_accounts(id) ON DELETE SET NULL`
- `accounts.chart_of_account_id` → `chart_of_accounts(id) ON DELETE RESTRICT`
- `journal_entry_lines.journal_entry_id` → `journal_entries(id) ON DELETE CASCADE`
- `journal_entry_lines.account_id` → `accounts(id) ON DELETE RESTRICT`
- And many more...

**Proposed Solution:**
```rust
#[foreign_key = "chart_of_accounts(id) ON DELETE SET NULL"]
pub parent_id: Option<uuid::Uuid>,
```

### 2. CHECK Constraints
**Status:** ❌ Missing  
**Impact:** Cannot generate business logic validation

**Required For:**
- `journal_entries`: `total_debit = total_credit`
- `journal_entry_lines`: `(debit_amount > 0 AND credit_amount = 0) OR (debit_amount = 0 AND credit_amount > 0)`
- `invoice_lines`: `quantity > 0`, `unit_price >= 0`
- And more...

**Proposed Solution:**
```rust
#[check = "total_debit = total_credit"]  // Table-level
// or
#[check = "quantity > 0"]  // Column-level
pub quantity: Decimal,
```

### 3. Composite Unique Constraints
**Status:** ❌ Missing  
**Impact:** Cannot generate multi-column unique constraints

**Required For:**
- `account_balances`: `UNIQUE(account_id, fiscal_period_id, balance_date, currency_code, company_id)`

**Proposed Solution:**
```rust
#[composite_unique = ["account_id", "fiscal_period_id", "balance_date", "currency_code", "company_id"]]
// At table level
```

### 4. Index Definitions
**Status:** ❌ Missing (partial - `#[indexed]` exists for single columns)  
**Impact:** Cannot generate composite indexes, partial indexes, unique indexes

**Required For:**
- Composite indexes: `CREATE INDEX idx_journal_entries_source ON journal_entries(source_type, source_id)`
- Partial indexes: `CREATE INDEX idx_invoices_customer_id ON invoices(customer_id) WHERE customer_id IS NOT NULL`
- Unique indexes: Already handled by `#[unique]` but need composite support

**Proposed Solution:**
```rust
#[index = "idx_journal_entries_source(source_type, source_id)"]  // Composite
#[index = "idx_invoices_customer_id(customer_id) WHERE customer_id IS NOT NULL"]  // Partial
// At table level
```

## Nice-to-Have (🟡)

### 5. Table Comments
**Status:** ❌ Missing  
**Impact:** Missing documentation in generated SQL

**Required For:**
- `COMMENT ON TABLE chart_of_accounts IS 'Hierarchical chart of accounts structure'`

**Proposed Solution:**
```rust
#[table_comment = "Hierarchical chart of accounts structure"]
#[derive(LifeModel)]
#[table_name = "chart_of_accounts"]
pub struct ChartOfAccount { ... }
```

## What Works ✅

The following features are already working:
- ✅ Basic column types (UUID, String, i32, bool, etc.)
- ✅ Primary keys (`#[primary_key]`)
- ✅ Auto-increment (`#[auto_increment]`)
- ✅ Column types (`#[column_type = "VARCHAR(50)"]`)
- ✅ Default values (`#[default_value = "0"]`)
- ✅ Default expressions (`#[default_expr = "CURRENT_TIMESTAMP"]`)
- ✅ Unique constraints (`#[unique]`) - single column only
- ✅ Indexed columns (`#[indexed]`) - single column only
- ✅ Nullable fields (`Option<T>`)
- ✅ JSONB support (`serde_json::Value`)
- ✅ Schema names (`#[schema_name = "..."]`)
- ✅ Column comments (`#[comment = "..."]`)

## Implementation Priority

1. **Foreign Key Constraints** - Highest priority (blocks all relationships)
2. **CHECK Constraints** - High priority (blocks business logic validation)
3. **Composite Unique Constraints** - High priority (blocks multi-column uniques)
4. **Index Definitions** - High priority (blocks composite/partial indexes)
5. **Table Comments** - Low priority (nice-to-have documentation)

## Next Steps

1. Implement missing attributes in `lifeguard-derive/src/attributes.rs`
2. Store metadata in `ColumnDefinition` or new `TableDefinition` struct
3. Update migration generator to read metadata and generate SQL
4. Test with entity examples
5. Compare generated SQL with `migrations/original/`

## Related Documentation

- `../../lifeguard-derive/SEAORM_LIFEGUARD_MAPPING.md` - Section 12: Migration Generation Requirements
- `../../migrations/README.md` - Migration strategy
- `../../migrations/original/` - Reference SQL migrations
