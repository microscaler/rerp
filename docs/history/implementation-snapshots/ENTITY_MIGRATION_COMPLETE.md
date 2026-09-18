# Entity Migration Complete

> **Status: HISTORICAL_SNAPSHOT** — records the retired repository-root entity
> migration. Current table/entity ownership is defined by
> [`CONTRIBUTING.md`](../../../CONTRIBUTING.md).

## Summary

All accounting entities have been successfully moved from `lifeguard/examples/entities/` to `rerp/entities/`.

**Date**: 2026-01-23  
**Status**: ✅ Migration Complete

---

## ✅ Migration Details

### Source Location
- **From**: `lifeguard/examples/entities/src/accounting/`
- **Status**: ✅ Removed (filesystem move completed)

### Destination Location
- **To**: `rerp/entities/src/accounting/`
- **Status**: ✅ All entities present

### Entities Moved

All 47 Rust entity files across 9 service domains:

1. **General Ledger** (5 entities)
   - `chart_of_accounts.rs`
   - `account.rs`
   - `journal_entry.rs`
   - `journal_entry_line.rs`
   - `account_balance.rs`

2. **Invoice** (2 entities)
   - `invoice.rs`
   - `invoice_line.rs`

3. **Accounts Receivable** (4 entities)
   - `customer_invoice.rs`
   - `ar_payment.rs`
   - `ar_payment_application.rs`
   - `ar_aging.rs`

4. **Accounts Payable** (4 entities)
   - `vendor_invoice.rs`
   - `ap_payment.rs`
   - `ap_payment_application.rs`
   - `ap_aging.rs`

5. **Bank Sync** (5 entities)
   - `bank.rs` ✅ (newly created for Bank Account PRD)
   - `bank_account.rs` ✅ (updated with bank_id FK and new fields)
   - `bank_transaction.rs`
   - `bank_statement.rs`
   - `bank_reconciliation.rs`

6. **Asset** (4 entities)
   - `asset.rs`
   - `asset_category.rs`
   - `asset_depreciation.rs`
   - `asset_transaction.rs`

7. **Budget** (5 entities)
   - `budget.rs`
   - `budget_version.rs`
   - `budget_period.rs`
   - `budget_line_item.rs`
   - `budget_actual.rs`

8. **EDI** (4 entities)
   - `edi_document.rs`
   - `edi_format.rs`
   - `edi_mapping.rs`
   - `edi_acknowledgment.rs`

9. **Financial Reports** (4 entities)
   - `financial_report.rs`
   - `report_template.rs`
   - `report_schedule.rs`
   - `report_data.rs`

**Total**: 47 entity files + 9 mod.rs files = 56 Rust files

---

## 📝 Files Updated

### RERP Entities
- ✅ `rerp/entities/Cargo.toml` - Updated package name to `rerp-entities`
- ✅ `rerp/entities/src/lib.rs` - Updated to use `rerp_entities`
- ✅ `rerp/entities/README.md` - Updated with RERP-specific documentation

### Lifeguard Examples
- ✅ `lifeguard/examples/entities/src/lib.rs` - Removed accounting module reference
- ✅ `lifeguard/examples/entities/Cargo.toml` - Updated to `example-entities`
- ✅ `lifeguard/examples/entities/README.md` - Updated to note entities moved to RERP

---

## ✅ Verification

### Entity Count
- **RERP**: 47 `.rs` files in `rerp/entities/src/accounting/`
- **Lifeguard**: 0 `.rs` files in `lifeguard/examples/entities/src/accounting/` (removed)

### Service Domains
All 9 service domains present in RERP:
- ✅ `general_ledger/`
- ✅ `invoice/`
- ✅ `accounts_receivable/`
- ✅ `accounts_payable/`
- ✅ `bank_sync/` (includes new Bank entity)
- ✅ `asset/`
- ✅ `budget/`
- ✅ `edi/`
- ✅ `financial_reports/`

---

## 🔧 Usage in Microservices

Entities are now used in RERP microservices via:

```toml
[dependencies]
rerp_entities = { path = "../entities" }
lifeguard = { path = "../../lifeguard" }
```

```rust
use rerp_entities::accounting::general_ledger::Account;
use rerp_entities::accounting::bank_sync::Bank;
use rerp_entities::accounting::bank_sync::BankAccount;
```

---

## 📚 References

- **Historical RERP Preparation Plan**:
  `docs/history/conceptual-bootstrap/RERP_PREPARATION_PLAN.md`
- **Historical implementation snapshot**:
  `docs/history/implementation-snapshots/IMPLEMENTATION_COMPLETE.md`
- **Bank Account PRD**: `docs/BANK_ACCOUNT_IMPROVEMENT_PRD.md`

---

**Status**: ✅ Migration Complete  
**Last Updated**: 2026-01-23
