# Lifeguard Money Type Implementation Status

## Completed ✅

### Type Detection
- ✅ `is_decimal_type()` - Detects `rust_decimal::Decimal` (both `rust_decimal::Decimal` and imported `Decimal`)
- ✅ `is_money_type()` - Detects `rusty_money::Money<Currency>` (both fully qualified and imported)

### Type Conversion (Model/Record → SeaQuery Value)
- ✅ `generate_field_to_value()` - Decimal → `Value::String(Some(decimal.to_string()))`
- ✅ `generate_field_to_value()` - Money → `Value::String(Some(money.amount().to_string()))`
- ✅ `generate_option_field_to_value()` - `Option<Decimal>` → `Option<Value::String>`
- ✅ `generate_option_field_to_value()` - `Option<Money>` → `Option<Value::String>`
- ✅ `generate_option_field_to_value_with_default()` - `Option<Decimal>` → `Value::String`
- ✅ `generate_option_field_to_value_with_default()` - `Option<Money>` → `Value::String`

### Value-to-Field Conversion (SeaQuery Value → Model/Record)
- ✅ `generate_value_to_field()` - `Value::String` → Decimal (parsing)
- ✅ `generate_value_to_option_field()` - `Value::String` → `Option<Decimal>` (parsing)

### FromRow Generation
- ✅ Decimal fields - Uses `row.try_get()` directly (Decimal implements FromSql for NUMERIC)
- ⚠️ Money fields - Placeholder using `row.try_get()` (may not work if Money doesn't implement FromSql)

### SQL Type Inference
- ✅ Decimal → `NUMERIC(19, 4)` (automatic inference)
- ✅ Money → `NUMERIC(19, 4)` (automatic inference)

## Pending / TODO

### Money FromRow Construction
**Issue:** Money type requires both:
1. Amount from NUMERIC column (e.g., `debit_amount`)
2. Currency code from VARCHAR column (e.g., `currency_code`)

**Current Status:** Placeholder that tries `row.try_get()` directly, which may fail if Money doesn't implement FromSql.

**Required Solution:**
```rust
// Need to:
// 1. Find currency_code field in same struct
// 2. Read amount: row.try_get::<&str, rust_decimal::Decimal>("debit_amount")?
// 3. Read currency: row.try_get::<&str, String>("currency_code")?
// 4. Construct: rusty_money::Money::from_minor(amount, currency)
```

**Complexity:** Requires:
- Field discovery (find currency_code field in struct)
- Multi-field coordination in FromRow generation
- Currency code parsing/validation

**Options:**
1. **Manual construction in entity** - Keep `#[skip_from_row]` for Money, construct manually
2. **Custom attribute** - `#[currency_field = "currency_code"]` to link Money to currency field
3. **Two-column approach** - Store Money as two separate fields (amount + currency_code), construct in business logic
4. **Full implementation** - Enhance FromRow generation to support multi-field construction

**Recommendation:** Start with Option 1 (manual construction) or Option 3 (two-column approach) for now, implement Option 2 or 4 later if needed.

## Testing Status

### Compilation
- ✅ Lifeguard-derive compiles successfully
- ✅ Type conversion functions compile
- ✅ FromRow generation compiles

### Runtime Testing
- ⏳ Not yet tested with real entities
- ⏳ Decimal FromRow not yet tested
- ⏳ Money FromRow not yet tested

## Next Steps

1. **Test Decimal Support**
   - Remove `#[skip_from_row]` from one entity with Decimal field
   - Verify compilation
   - Test database round-trip

2. **Test Money Support (Basic)**
   - Try using Money type in entity
   - Check if Money implements FromSql
   - If not, implement manual construction or use two-column approach

3. **Enhance Money Support (If Needed)**
   - Implement `#[currency_field]` attribute
   - Or enhance FromRow generation for multi-field construction
   - Or document two-column approach as recommended pattern

## Files Modified

- `/Users/casibbald/Workspace/microscaler/lifeguard/lifeguard-derive/src/type_conversion.rs`
  - Added `is_decimal_type()` and `is_money_type()` functions
  - Updated all conversion functions to handle Decimal and Money

- `/Users/casibbald/Workspace/microscaler/lifeguard/lifeguard-derive/src/macros/life_model.rs`
  - Updated FromRow generation to detect and handle Decimal
  - Added Money placeholder in FromRow
  - Updated SQL type inference for Decimal and Money

## Usage Notes

### Decimal
```rust
#[derive(LifeModel)]
pub struct MyEntity {
    // Decimal works directly - no #[skip_from_row] needed
    #[column_type = "NUMERIC(19, 4)"]  // Optional - auto-inferred
    pub amount: rust_decimal::Decimal,
}
```

### Money (Current - May Need Manual Construction)
```rust
#[derive(LifeModel)]
pub struct MyEntity {
    // Money - may need #[skip_from_row] if Money doesn't implement FromSql
    #[column_type = "NUMERIC(19, 4)"]  // Optional - auto-inferred
    pub amount: rusty_money::Money<rusty_money::iso::Currency>,
    
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,  // Required for Money construction
}
```

### Recommended Pattern for Money (Until Full Support)
```rust
#[derive(LifeModel)]
#[skip_from_row]  // Skip for now, construct manually
pub struct MyEntity {
    #[column_type = "NUMERIC(19, 4)"]
    pub amount: rust_decimal::Decimal,  // Use Decimal for storage
    
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,
    
    // Construct Money in business logic from amount + currency_code
}
```
