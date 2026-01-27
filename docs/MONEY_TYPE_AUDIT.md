# Money Type Audit and Implementation Plan

## Executive Summary

This document provides a comprehensive audit of RERP's OpenAPI specifications and entity system to identify all financial fields that should use `format: money` instead of `format: decimal` or no format. It also outlines the implementation plan for updating both the OpenAPI specs and the Lifeguard entity system.

## Current State Analysis

### OpenAPI Specifications

**Total OpenAPI Files:** 98 files across multiple suites

**Current Pattern:**
- Financial amounts use `type: number, format: decimal` or `type: number` (no format)
- This generates `rust_decimal::Decimal` or `f64` in Rust code
- Missing currency awareness and proper money type handling

**Target Pattern:**
- Financial amounts should use `type: number, format: money` → `rusty_money::Money`
- Rates/percentages should use `type: number, format: decimal` → `rust_decimal::Decimal`
- Mathematical numbers should use `type: number` (no format) → `f64`

### Entity System (Lifeguard)

**Current State:**
- Entities use `rust_decimal::Decimal` for all financial amounts
- Database columns use `NUMERIC(19, 4)` for amounts
- Currency stored separately as `VARCHAR(3)` (currency_code)
- No currency-aware type system

**Example from `journal_entry_line.rs`:**
```rust
#[column_type = "NUMERIC(19, 4)"]
pub debit_amount: rust_decimal::Decimal,

#[column_type = "NUMERIC(19, 4)"]
pub credit_amount: rust_decimal::Decimal,

#[default_value = "'USD'"]
#[column_type = "VARCHAR(3)"]
pub currency_code: String,
```

**Target State:**
- Entities should use `rusty_money::Money` for financial amounts
- Database columns remain `NUMERIC(19, 4)` (Money stores as Decimal internally)
- Currency integrated into Money type
- Maintain backward compatibility during migration

## Detailed Audit Results

### Summary Statistics

- **Total OpenAPI Files:** 98 files
- **Total Issues Found:** 196 fields requiring updates
- **Affected Services:** 8 accounting services
- **Money Fields:** ~184 fields need `format: money`
- **Decimal Fields:** ~12 fields need `format: decimal` (rates/percentages)

### Accounting Suite - High Priority

**Total Issues in Accounting Suite:** 196 fields

#### accounts-payable (13 money fields, 0 decimal fields)
**Fields requiring `format: money`:**
- `VendorInvoice.original_amount` - Original invoice amount
- `VendorInvoice.outstanding_amount` - Unpaid amount
- `ApPayment.payment_amount` - Payment total
- `ApPayment.applied_amount` - Applied payment amount
- `ApPaymentApplication.applied_amount` - Applied to invoice
- `ApAging.total_outstanding` - Total outstanding across aging buckets
- `CreateVendorInvoiceRequest.early_payment_discount_percent` - ⚠️ Should be `format: decimal` (percentage)
- `UpdateVendorInvoiceRequest.outstanding_amount` - Outstanding amount
- `CreateApPaymentRequest.payment_amount` - Payment amount
- `UpdateApPaymentRequest.payment_amount` - Payment amount
- `CreateApPaymentApplicationRequest.applied_amount` - Applied amount
- `UpdateApPaymentApplicationRequest.applied_amount` - Applied amount

**Note:** `early_payment_discount_percent` should remain `format: decimal` (it's a percentage, not money)

#### accounts-receivable (18 money fields)
**Fields requiring `format: money`:**
- `CustomerInvoice.outstanding_amount` - Outstanding amount
- `CustomerInvoice.credit_limit` - Credit limit amount
- `CustomerInvoice.credit_used` - Credit used amount
- `CustomerInvoice.last_payment_amount` - Last payment amount
- `CustomerInvoice.write_off_amount` - Write-off amount
- `Payment.payment_amount` - Customer payment amount
- `Payment.applied_amount` - Applied to invoices
- `Payment.unapplied_amount` - Unapplied amount
- `PaymentApplication.applied_amount` - Applied payment amount
- `ArAging.total_outstanding` - Total outstanding
- Plus additional fields in request/response schemas

#### invoice (15 money fields)
**Fields requiring `format: money`:**
- `Invoice.subtotal` - Invoice subtotal
- `Invoice.tax_amount` - Tax amount
- `Invoice.discount_amount` - Discount amount
- `Invoice.total_amount` - Total invoice amount
- `Invoice.paid_amount` - Amount paid
- `Invoice.outstanding_amount` - Outstanding amount
- `InvoiceLine.unit_price` - Line item unit price
- `InvoiceLine.discount_amount` - Line discount
- `InvoiceLine.line_subtotal` - Line subtotal
- `InvoiceLine.tax_amount` - Line tax
- `InvoiceLine.line_total` - Line total
- Plus additional fields in request/response schemas

**Fields requiring `format: decimal`:**
- `InvoiceLine.tax_rate` - Tax rate percentage
- `InvoiceLine.discount_percent` - Discount percentage
- `Invoice.exchange_rate` - Currency exchange rate

#### general-ledger (2 money fields)
**Fields requiring `format: money`:**
- `JournalEntry.total_debit` - Total debits
- `JournalEntry.total_credit` - Total credits

**Note:** Journal entry line amounts and account balances are not in OpenAPI (they're entity-only), but will need entity updates.

#### budget (17 money fields)
**Fields requiring `format: money`:**
- `Budget.total_budget_amount` - Total budget
- `Budget.total_actual_amount` - Total actual
- `Budget.total_variance` - Variance amount
- `BudgetLine.budget_amount` - Budgeted amount
- `BudgetLine.actual_amount` - Actual amount
- `BudgetLine.variance` - Variance amount
- `BudgetVariance.budget_amount` - Variance budget amount
- `BudgetVariance.actual_amount` - Variance actual amount
- `BudgetVariance.variance` - Variance amount
- Plus additional fields in request/response schemas

**Fields requiring `format: decimal`:**
- `BudgetLine.variance_percent` - Variance percentage (should be decimal, not money)

#### asset (10 money fields)
**Fields requiring `format: money`:**
- `Asset.purchase_cost` - Asset purchase cost
- `Asset.current_value` - Current book value
- `Asset.salvage_value` - Residual value
- `Depreciation.depreciation_amount` - Depreciation amount
- `AssetTransaction.transaction_amount` - Transaction amount
- `AssetTransaction.impairment_amount` - Impairment amount
- Plus additional fields in request/response schemas

**Fields requiring `format: decimal`:**
- `Asset.depreciation_rate` - Annual depreciation rate (should be decimal, not money)

#### bank-sync (17 money fields)
**Fields requiring `format: money`:**
- `BankAccount.current_balance` - Current balance
- `BankAccount.reconciled_balance` - Reconciled balance
- `BankTransaction.amount` - Transaction amount
- `BankTransaction.balance_after` - Balance after transaction
- `BankStatement.opening_balance` - Opening balance
- `BankStatement.closing_balance` - Closing balance
- `BankStatement.total_debits` - Total debits
- `BankStatement.total_credits` - Total credits
- `Reconciliation.book_balance` - Book balance
- `Reconciliation.bank_balance` - Bank balance
- Plus additional fields in request/response schemas

### Other Suites - Medium/Low Priority

#### sales (order, quotation, subscription)
- Order totals, line item prices, subscription fees
- Payment amounts, refunds, discounts

#### purchase
- Purchase order amounts, line item costs
- Vendor payment amounts

#### hr/payroll
- Salary, wage, commission amounts
- Deduction amounts, net pay

#### pos
- Transaction amounts, payment amounts
- Product prices, discounts

## Implementation Plan

### Phase 1: OpenAPI Specification Updates

#### Step 1.1: Update Accounting Suite (High Priority)
**Estimated Time:** 2-3 hours

1. **accounts-payable** (`openapi/accounting/accounts-payable/openapi.yaml`)
   - [ ] Update all `applied_amount`, `original_amount`, `outstanding_amount`, `payment_amount` fields
   - [ ] Change from `format: decimal` to `format: money`
   - [ ] Keep `tax_rate`, `discount_percent`, `exchange_rate` as `format: decimal`

2. **accounts-receivable** (`openapi/accounting/accounts-receivable/openapi.yaml`)
   - [ ] Update payment and invoice amount fields
   - [ ] Update aging bucket amount fields
   - [ ] Keep rate/percentage fields as `format: decimal`

3. **invoice** (`openapi/accounting/invoice/openapi.yaml`)
   - [ ] Update invoice amount fields (subtotal, tax_amount, total_amount, etc.)
   - [ ] Update invoice_line amount fields
   - [ ] Keep rate fields as `format: decimal`

4. **general-ledger** (`openapi/accounting/general-ledger/openapi.yaml`)
   - [ ] Update journal entry amount fields
   - [ ] Update account balance fields
   - [ ] Keep exchange_rate as `format: decimal`

5. **budget** (`openapi/accounting/budget/openapi.yaml`)
   - [ ] Update budget amount fields
   - [ ] Keep variance_percent as `format: decimal`

6. **asset** (`openapi/accounting/asset/openapi.yaml`)
   - [ ] Update asset cost and value fields
   - [ ] Update depreciation amount fields
   - [ ] Keep depreciation_rate as `format: decimal`

7. **bank-sync** (`openapi/accounting/bank-sync/openapi.yaml`)
   - [ ] Update transaction and balance fields

#### Step 1.2: Regenerate Microservices
**Estimated Time:** 30 minutes

After updating OpenAPI specs:
```bash
# Regenerate all accounting services
for service in accounts-payable accounts-receivable invoice general-ledger budget asset bank-sync; do
  tooling/.venv/bin/rerp generate accounting/$service
done
```

This will:
- Generate new Rust types with `rusty_money::Money` for money fields
- Update `gen/Cargo.toml` to include `rusty-money` dependency
- Update generated handlers and types

#### Step 1.3: Update Other Suites (Medium Priority)
**Estimated Time:** 4-6 hours

- sales suite (order, quotation, subscription)
- purchase suite
- hr/payroll
- pos
- Other suites with financial fields

### Phase 2: Entity System Updates

#### Step 2.1: Add rusty-money to Entities
**Estimated Time:** 15 minutes

Update `entities/Cargo.toml`:
```toml
[dependencies]
rusty-money = { version = "0.5", features = ["serde"] }
```

#### Step 2.2: Update Entity Definitions
**Estimated Time:** 4-6 hours

**Key Entities Requiring Updates:**
- `journal_entry_line.rs` - debit_amount, credit_amount, base_debit_amount, base_credit_amount
- `journal_entry.rs` - total_debit, total_credit
- `account_balance.rs` - debit_balance, credit_balance
- `invoice.rs` - subtotal, tax_amount, discount_amount, total_amount, paid_amount, outstanding_amount
- `invoice_line.rs` - unit_price, discount_amount, line_subtotal, tax_amount, line_total
- `ap_payment.rs` - payment_amount, applied_amount, unapplied_amount
- `ap_payment_application.rs` - applied_amount
- `ar_payment.rs` - payment_amount, applied_amount, unapplied_amount
- `ar_payment_application.rs` - applied_amount
- `budget.rs` - total_budget_amount, total_actual_amount, total_variance
- `budget_line_item.rs` - budget_amount, actual_amount, variance
- `asset.rs` - purchase_cost, current_value, accumulated_depreciation, salvage_value
- `asset_depreciation.rs` - depreciation_amount
- `asset_transaction.rs` - transaction_amount, impairment_amount
- `bank_transaction.rs` - amount, balance_after
- `bank_account.rs` - current_balance, reconciled_balance
- `bank_statement.rs` - opening_balance, closing_balance, total_debits, total_credits
- `bank_reconciliation.rs` - book_balance, bank_balance

**Migration Pattern:**

**Before:**
```rust
#[column_type = "NUMERIC(19, 4)"]
pub debit_amount: rust_decimal::Decimal,

#[default_value = "'USD'"]
#[column_type = "VARCHAR(3)"]
pub currency_code: String,
```

**After (Option A - Full Migration):**
```rust
#[column_type = "NUMERIC(19, 4)"]  // Keep for database compatibility
pub debit_amount: rusty_money::Money<rusty_money::iso::Currency>,

// Keep currency_code for database queries/filtering (Money stores currency internally)
#[default_value = "'USD'"]
#[column_type = "VARCHAR(3)"]
pub currency_code: String,  // Keep for SQL queries and filtering
```

**After (Option B - Gradual Migration):**
```rust
// New Money field
#[column_type = "NUMERIC(19, 4)"]
pub debit_amount_money: Option<rusty_money::Money<rusty_money::iso::Currency>>,

// Legacy Decimal field (deprecated, will be removed)
#[column_type = "NUMERIC(19, 4)"]
#[deprecated(note = "Use debit_amount_money instead")]
pub debit_amount: rust_decimal::Decimal,

#[default_value = "'USD'"]
#[column_type = "VARCHAR(3)"]
pub currency_code: String,
```

**Recommended Approach:**
- Start with Option A (direct replacement) for new entities
- Use Option B (gradual) for existing entities with data
- Add conversion helpers between Decimal and Money
- Update business logic incrementally

#### Step 2.3: Investigate Lifeguard Compatibility
**Estimated Time:** 2-4 hours

**Investigation Tasks:**
1. Check if Lifeguard's `LifeModel` derive macro supports custom types
2. Test `rusty_money::Money` serialization/deserialization with Lifeguard
3. Verify SQL generation works with Money types
4. Test database persistence (NUMERIC → Money conversion)

**Potential Issues:**
- Lifeguard may need `FromSql`/`ToSql` implementations for Money
- Money type stores as Decimal internally, so NUMERIC should work
- Currency may need separate handling or custom serialization

**If Lifeguard Needs Updates:**
- Add Money type support to Lifeguard derive macro
- Implement FromSql/ToSql traits for Money
- Update SQL generation to handle Money types
- Test thoroughly with existing entities

#### Step 2.4: Database Migration Strategy
**Estimated Time:** 1-2 hours

**Option A: No Schema Changes (Recommended)**
- Keep `NUMERIC(19, 4)` columns as-is
- Money type stores as Decimal internally
- No database migration needed
- Currency stored in Money type, keep `currency_code` column for queries

**Option B: Add Currency Columns (if needed)**
- If Money type requires separate currency storage
- Add migration to ensure currency_code columns exist
- Update entity definitions

### Phase 3: Business Logic Updates

#### Step 3.1: Update Controllers
**Estimated Time:** 6-8 hours

Update all `impl/src/controllers/*.rs` files to:
- Use `rusty_money::Money` instead of `rust_decimal::Decimal` for amounts
- Handle currency conversions properly
- Update calculations to use Money arithmetic

**Example:**
```rust
// Before
let total = invoice.subtotal + invoice.tax_amount;

// After
let total = invoice.subtotal.add(invoice.tax_amount)?;
```

#### Step 3.2: Update Entity Conversions
**Estimated Time:** 2-3 hours

Add conversion methods between:
- OpenAPI types (Money) ↔ Entity types (Money)
- Legacy Decimal ↔ New Money types

### Phase 4: Testing and Validation

#### Step 4.1: Unit Tests
**Estimated Time:** 4-6 hours

- Test Money type serialization/deserialization
- Test currency conversions
- Test Money arithmetic operations
- Test database persistence

#### Step 4.2: Integration Tests
**Estimated Time:** 2-3 hours

- Test API endpoints with Money types
- Test entity persistence
- Test multi-currency scenarios

#### Step 4.3: Regression Testing
**Estimated Time:** 2-3 hours

- Ensure existing functionality still works
- Test backward compatibility
- Verify no data loss during migration

## Risk Assessment

### High Risk Areas

1. **Database Compatibility**
   - Risk: Money type may not serialize correctly to NUMERIC
   - Mitigation: Test thoroughly, keep Decimal as fallback during migration

2. **Currency Handling**
   - Risk: Missing currency_code fields break queries
   - Mitigation: Keep currency_code columns during transition

3. **API Breaking Changes**
   - Risk: Changing types may break API contracts
   - Mitigation: Version API, provide migration guide

4. **Lifeguard Compatibility**
   - Risk: Lifeguard may not support Money type
   - Mitigation: Investigate first, update Lifeguard if needed

### Medium Risk Areas

1. **Business Logic Calculations**
   - Risk: Money arithmetic differs from Decimal
   - Mitigation: Update all calculations, add tests

2. **Multi-Currency Operations**
   - Risk: Currency conversions may fail
   - Mitigation: Test thoroughly, handle errors gracefully

## Success Criteria

1. ✅ All financial amount fields in OpenAPI use `format: money`
2. ✅ All rate/percentage fields use `format: decimal`
3. ✅ All entities use `rusty_money::Money` for financial amounts
4. ✅ All controllers handle Money types correctly
5. ✅ All tests pass
6. ✅ No data loss during migration
7. ✅ API contracts remain compatible (or properly versioned)
8. ✅ Database schema remains compatible

## Entity System Analysis

### Current Entity Structure

**Entities Using `rust_decimal::Decimal` for Financial Amounts:**
- All accounting entities use `Decimal` for amounts
- Database columns use `NUMERIC(19, 4)` for amounts
- Currency stored separately as `VARCHAR(3)` (currency_code)
- Exchange rates use `NUMERIC(19, 6)` (more precision for rates)

**Example Entity Pattern:**
```rust
#[derive(LifeModel)]
pub struct ApPayment {
    // ...
    #[column_type = "NUMERIC(19, 4)"]
    pub payment_amount: rust_decimal::Decimal,
    
    #[column_type = "NUMERIC(19, 4)"]
    pub applied_amount: rust_decimal::Decimal,
    
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,
    
    #[default_value = "1.0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub exchange_rate: rust_decimal::Decimal,
}
```

### Lifeguard Compatibility Considerations

**Key Questions:**
1. Does Lifeguard's `LifeModel` derive support custom types like `rusty_money::Money`?
2. Can Money type serialize/deserialize to/from NUMERIC columns?
3. Does Money type work with Lifeguard's query builder?
4. Are FromSql/ToSql implementations needed?

**Investigation Required:**
- Check Lifeguard source code for type handling
- Test Money type with Lifeguard derive macro
- Verify database round-trip (save → load) works correctly
- Test query filtering and sorting with Money types

**Potential Solutions:**
- If Lifeguard doesn't support Money: Add custom FromSql/ToSql implementations
- If serialization fails: Use custom serde attributes or wrapper types
- If queries fail: Add helper methods or custom query builders

## Timeline Estimate

- **Phase 1 (OpenAPI Updates):** 6-9 hours
- **Phase 2 (Entity Updates):** 7-12 hours
  - Investigation: 2-4 hours
  - Entity updates: 4-6 hours
  - Lifeguard compatibility fixes: 1-2 hours (if needed)
- **Phase 3 (Business Logic):** 8-11 hours
- **Phase 4 (Testing):** 8-12 hours

**Total Estimated Time:** 29-44 hours

**With Lifeguard Updates (if needed):** +4-8 hours

## Next Steps

1. Review this audit document
2. Prioritize which suites to update first
3. Start with Phase 1.1 (accounts-payable) as pilot
4. Test thoroughly before proceeding to other suites
5. Document any issues encountered
6. Update this plan based on findings
