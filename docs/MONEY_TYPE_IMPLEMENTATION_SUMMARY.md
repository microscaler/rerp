# Money Type Implementation Summary

## Implementation Status

### ✅ Completed

1. **Lifeguard Support**
   - ✅ Added Decimal type detection and conversion
   - ✅ Added Money type detection (for future use)
   - ✅ Updated FromRow generation for Decimal
   - ✅ Updated SQL type inference
   - ✅ Removed `#[skip_from_row]` requirement for Decimal

2. **OpenAPI Specifications**
   - ✅ Updated 199 fields from `format: decimal` to `format: money` across accounting suite
   - ✅ Kept `format: decimal` for rates/percentages (tax_rate, discount_percent, etc.)

3. **BRRTRouter Code Generation**
   - ✅ Updated to generate `rust_decimal::Decimal` for `format: money` fields
   - ✅ Updated to generate `rust_decimal::Decimal` for `format: decimal` fields
   - ✅ Updated dummy_value to handle Decimal types

4. **Bootstrap Script**
   - ✅ Added automatic detection of Money/Decimal usage in generated code
   - ✅ Automatically adds `rust_decimal` and `rusty-money` dependencies when needed

### ⚠️ Design Decision: Decimal in API, Money in Entities

**Issue:** `rusty_money::Money<'a, T>` has a lifetime parameter that is incompatible with owned `Deserialize` types needed for API serialization.

**Solution:** 
- **API Layer**: Use `rust_decimal::Decimal` for `format: money` fields
- **Entity Layer**: Use `rusty_money::Money` for financial amounts
- **Business Logic**: Convert between Decimal (API) and Money (entities)

**Rationale:**
- API needs owned types for serde serialization
- Entities can use Money types with proper lifetime management
- Conversion layer provides type safety and currency awareness where needed

## Current Architecture

### API Types (Generated from OpenAPI)
```rust
// Generated from format: money
pub struct ApPayment {
    pub payment_amount: rust_decimal::Decimal,  // API uses Decimal
    pub currency_code: String,
}
```

### Entity Types (Lifeguard)
```rust
// Entities can use Money (with proper lifetime management)
pub struct ApPayment {
    pub payment_amount: rusty_money::Money<rusty_money::iso::Currency>,
    pub currency_code: String,
}
```

### Conversion Pattern
```rust
// In controllers: Convert Decimal → Money
let money = rusty_money::Money::from_minor(
    payment_amount.to_string().parse::<i64>()?,
    currency_from_code(&currency_code)?
);

// In responses: Convert Money → Decimal  
let decimal = money.amount();  // Returns Decimal
```

## Next Steps

1. **Update Entities to Use Money Types**
   - Replace `rust_decimal::Decimal` with `rusty_money::Money` in entity definitions
   - Implement FromRow construction for Money (amount + currency_code)
   - Test database round-trip

2. **Add Conversion Helpers**
   - Create helper functions for Decimal ↔ Money conversion
   - Handle currency code parsing/validation
   - Add error handling for currency mismatches

3. **Update Controllers**
   - Convert API Decimal → Entity Money
   - Convert Entity Money → API Decimal
   - Handle currency code validation

4. **Testing**
   - Test API serialization/deserialization
   - Test entity persistence
   - Test currency conversions
   - Test multi-currency scenarios

## Files Modified

### Lifeguard
- `lifeguard-derive/src/type_conversion.rs` - Added Decimal/Money detection and conversion
- `lifeguard-derive/src/macros/life_model.rs` - Updated FromRow and SQL inference

### BRRTRouter
- `src/generator/schema.rs` - Updated to generate Decimal for money format
- `src/dummy_value.rs` - Updated to handle Decimal types

### RERP
- `openapi/accounting/**/openapi.yaml` - Updated 199 fields to `format: money`
- `tooling/src/rerp_tooling/bootstrap/microservice.py` - Added dependency detection
- `entities/src/accounting/accounts_payable/ap_payment.rs` - Removed `#[skip_from_row]` (test Decimal)

## Usage Notes

### OpenAPI Specs
```yaml
# Financial amounts
payment_amount:
  type: number
  format: money  # → rust_decimal::Decimal in API

# Rates/percentages  
tax_rate:
  type: number
  format: decimal  # → rust_decimal::Decimal in API
```

### Entity Definitions
```rust
// Use Money in entities (after FromRow support is complete)
#[column_type = "NUMERIC(19, 4)"]
pub payment_amount: rusty_money::Money<rusty_money::iso::Currency>,

#[column_type = "VARCHAR(3)"]
pub currency_code: String,  // Required for Money construction
```

### API Types (Generated)
```rust
// API uses Decimal for serialization
pub payment_amount: rust_decimal::Decimal,
pub currency_code: String,
```

## Success Metrics

- ✅ All financial amounts use `format: money` in OpenAPI
- ✅ All rates/percentages use `format: decimal` in OpenAPI  
- ✅ Generated API types use `rust_decimal::Decimal` (serializable)
- ✅ Lifeguard supports Decimal types (no `#[skip_from_row]` needed)
- ⏳ Entities can use Money types (FromRow support pending)
- ⏳ Conversion helpers implemented
- ⏳ All tests passing
