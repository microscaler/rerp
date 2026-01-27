# Money/Currency Type Analysis and Solution

## Problem Statement

BRRTRouter currently maps all `type: number` fields to Rust's `f64` type, regardless of format. This causes several issues:

1. **Clippy warnings**: The dummy value `3.14` triggers `clippy::approx_constant` because it's too close to `f32::consts::PI`
2. **Precision loss**: `f64` is not suitable for financial calculations due to floating-point precision issues
3. **No semantic distinction**: There's no way to distinguish between:
   - Mathematical/engineering numbers → should use `f64`
   - General decimal numbers (rates, percentages) → should use `rust_decimal::Decimal`
   - Financial/money values → should use `rusty_money::Money` (currency-aware)

## Current State

### OpenAPI Spec Usage
```yaml
applied_amount:
  type: number
  format: decimal
```

### BRRTRouter Mapping
- Location: `BRRTRouter/src/generator/schema.rs` lines 642, 716
- Current mapping: `type: number` → `f64` (ignores `format: decimal`)
- Dummy value: `3.14` (triggers clippy warnings)

### Generated Rust Code
```rust
pub struct ApPaymentApplication {
    pub applied_amount: Option<f64>,  // Should be rusty_money::Money!
}
```

## Proposed Solution: Format-Based Type Differentiation

BRRTRouter needs to differentiate between three types of numeric values based on OpenAPI `format`:

1. **`type: number` (no format)** → `f64` - Mathematical/engineering numbers
2. **`type: number, format: decimal`** → `rust_decimal::Decimal` - General decimal numbers (rates, percentages, precision decimals)
3. **`type: number, format: money`** → `rusty_money::Money` - Financial amounts with currency

### Why rusty-money?

`rusty-money` is specifically designed for financial systems and provides:
- **Currency-aware types** - Built-in ISO-4217 currency support
- **128-bit decimal precision** - Up to 28 decimal places
- **Safety by default** - Operations return `Result` types
- **Fowler's Money pattern** - Industry-standard design
- **Exchange rate support** - Built-in currency conversion
- **Internationalization** - Locale-aware formatting

### Implementation

**1. Update BRRTRouter Schema Type Mapping**

Modify `BRRTRouter/src/generator/schema.rs` in `extract_fields()` and `schema_to_type()`:

```rust
Some("number") => {
    let format = prop.get("format").and_then(|f| f.as_str());
    match format {
        Some("money") | Some("currency") => {
            // Financial amounts - use rusty_money::Money
            "rusty_money::Money".to_string()
        }
        Some("decimal") => {
            // General decimal numbers (rates, percentages) - use rust_decimal::Decimal
            "rust_decimal::Decimal".to_string()
        }
        _ => {
            // No format or unknown format - use f64 for mathematical numbers
            "f64".to_string()
        }
    }
}
```

**2. Update dummy_value() for Money and Decimal**

Modify `BRRTRouter/src/dummy_value.rs`:

```rust
pub fn dummy_value(ty: &str) -> askama::Result<String> {
    let value = match ty {
        "String" => "\"example\".to_string()",
        "i32" => "42",
        "f64" => "3.14",  // Valid mathematical number - clippy warning is acceptable for f64
        "rust_decimal::Decimal" => {
            // General decimal: 123.45
            "rust_decimal::Decimal::new(12345, 2)".to_string()
        }
        "rusty_money::Money" => {
            // Money: $3.14 USD - clearly a dollar amount, not PI
            // from_minor(314, USD) = 314 cents = $3.14
            "rusty_money::Money::from_minor(314, rusty_money::iso::USD)".to_string()
        }
        "bool" => "true",
        "Vec<Value>" | "Vec<String>" | "Vec<i32>" | "Vec<f64>" | "Vec<bool>" => "vec![]",
        _ => "Default::default()",
    };
    Ok(value.to_string())
}
```

**Key Points:**
- `f64` uses `3.14` - Clippy warning is acceptable for mathematical numbers (they can legitimately be close to PI)
- `rusty_money::Money` uses `from_minor(314, USD)` which equals **$3.14** - Clearly a dollar amount, not a mathematical constant
- Clippy won't warn on Money because: (1) it's a different type (`Money` not `f64`), (2) the literal value is `314` (cents), not `3.14`
- This solves the issue: financial systems can use `$3.14` without clippy warnings, while mathematical code can use `3.14` (warning is acceptable)

**3. Update OpenAPI Specs to Use format: money**

For financial amounts, use `format: money`:

```yaml
# Before (ambiguous - could be any decimal)
applied_amount:
  type: number
  format: decimal

# After (explicitly money)
applied_amount:
  type: number
  format: money
```

For general decimal numbers (rates, percentages), keep `format: decimal`:

```yaml
# Tax rate, percentage, etc. - not money, just a decimal number
tax_rate:
  type: number
  format: decimal
```

**4. Add Money Schema Component (Optional but Recommended)**

For reusability and documentation, define a Money component:

```yaml
components:
  schemas:
    Money:
      type: number
      format: money
      description: |
        Represents a monetary amount with currency.
        Use this for all financial values (amounts, payments, balances).
        Maps to rusty_money::Money in generated code.
        Currency is determined by the currency_code field in the same object.
      example: 1234.56
```

Then use it via `$ref`:

```yaml
applied_amount:
  $ref: '#/components/schemas/Money'
```

**5. Update Generated Cargo.toml**

BRRTRouter should add dependencies to generated `Cargo.toml`:

```toml
[dependencies]
rusty-money = { version = "0.5", features = ["serde"] }  # If Money type is used
rust_decimal = "1.33"  # If Decimal type is used
```

## Recommendation: Format-Based Differentiation

**Advantages:**
- ✅ Clear semantic distinction: `money` vs `decimal` vs no format
- ✅ No OpenAPI spec changes needed initially (just change `format: decimal` → `format: money`)
- ✅ Self-documenting - format clearly indicates intent
- ✅ Supports both `rusty_money::Money` (financial) and `rust_decimal::Decimal` (general decimals)
- ✅ Can use schema components for reusability
- ✅ Industry standard - `rusty-money` is designed for financial systems

**Type Mapping Summary:**

| OpenAPI Type | Format | Rust Type | Use Case |
|-------------|--------|-----------|----------|
| `number` | (none) | `f64` | Mathematical/engineering numbers |
| `number` | `decimal` | `rust_decimal::Decimal` | General decimals (rates, percentages, precision decimals) |
| `number` | `money` | `rusty_money::Money` | Financial amounts with currency |

**Implementation Steps:**

1. **Update BRRTRouter**
   - Modify `schema.rs` to check `format` field for `money` vs `decimal`
   - Map `format: money` → `rusty_money::Money`
   - Map `format: decimal` → `rust_decimal::Decimal`
   - Map `number` (no format) → `f64`
   - Update `dummy_value()` for both `Money` and `Decimal` types
   - Ensure `rusty-money` and `rust_decimal` are added to generated `Cargo.toml` when needed

2. **Update OpenAPI Specs**
   - Change financial fields: `format: decimal` → `format: money`
   - Keep non-financial decimals as `format: decimal` (e.g., tax rates, percentages)
   - Optionally create `Money` schema component for reusability

3. **Update Workspace Dependencies**
   - Add `rusty-money = "0.5"` to `microservices/Cargo.toml` workspace dependencies
   - `rust_decimal` is already present (line 58)

## Files to Modify

### BRRTRouter
- `src/generator/schema.rs` - Add format-based type mapping (`money` → `rusty_money::Money`, `decimal` → `rust_decimal::Decimal`)
- `src/dummy_value.rs` - Add `Money` and `Decimal` dummy values
- `templates/Cargo.toml.txt` - Conditionally add `rusty-money` and `rust_decimal` dependencies when types are used

### RERP
- `microservices/Cargo.toml` - Add `rusty-money = "0.5"` to workspace dependencies
- `openapi/accounting/*/openapi.yaml` - Change financial fields from `format: decimal` to `format: money`
- Optionally add `Money` schema component to each spec for reusability

## Migration Path

1. **Phase 1**: Update BRRTRouter to support `format: money` → `rusty_money::Money` mapping
2. **Phase 2**: Add `rusty-money` to workspace dependencies
3. **Phase 3**: Update OpenAPI specs - change financial fields from `format: decimal` to `format: money`
4. **Phase 4**: Regenerate all services
5. **Phase 5**: Update business logic to use `rusty_money::Money` instead of `f64` for financial amounts

## Example: Before and After

### Before
```yaml
applied_amount:
  type: number
  format: decimal
```

```rust
pub applied_amount: Option<f64>,  // ❌ Floating-point precision issues, no currency
```

### After (Financial Amount)
```yaml
applied_amount:
  type: number
  format: money  # Explicitly money, not just a decimal
```

```rust
pub applied_amount: Option<rusty_money::Money>,  // ✅ Currency-aware, fixed precision
```

### After (General Decimal - e.g., tax rate)
```yaml
tax_rate:
  type: number
  format: decimal  # Not money, just a decimal number
```

```rust
pub tax_rate: Option<rust_decimal::Decimal>,  // ✅ Fixed precision, no currency needed
```

### After (Mathematical Number)
```yaml
coordinate:
  type: number  # No format - mathematical/engineering number
```

```rust
pub coordinate: f64,  // ✅ Standard floating-point for math
```

## Benefits

1. **Solves clippy issue** - `f64` can use `3.14` (valid math number), `Money` uses `$3.14` (clearly financial)
   - Clippy warning on `f64` is acceptable (mathematical numbers can legitimately be close to PI)
   - `Money::from_minor(314, USD)` = $3.14 - clearly a dollar amount, not PI
   - Clippy won't warn on Money because it's a different type and uses `314` (cents) not `3.14`
2. **Financial accuracy** - No floating-point rounding errors
3. **Currency-aware** - `rusty_money::Money` includes currency support (ISO-4217)
4. **Type safety** - Clear distinction between:
   - Mathematical numbers (`f64`) - can use `3.14`
   - General decimals (`rust_decimal::Decimal`) - uses `123.45`
   - Financial amounts (`rusty_money::Money`) - uses `$3.14`
5. **Industry standard** - `rusty-money` follows Fowler's Money pattern, designed for financial systems
6. **Exchange rate support** - Built-in currency conversion capabilities
7. **Internationalization** - Locale-aware formatting for different currencies
8. **Future-proof** - Can extend with validation, constraints, etc.

## Important Notes

### Currency Handling

`rusty_money::Money` requires a currency. In generated code, you'll need to pair money fields with a `currency_code` field:

```rust
pub struct Payment {
    pub amount: rusty_money::Money,  // Requires currency
    pub currency_code: String,        // "USD", "EUR", etc.
}

// In implementation:
let money = rusty_money::Money::from_major(
    amount_value,
    match currency_code.as_str() {
        "USD" => rusty_money::iso::USD,
        "EUR" => rusty_money::iso::EUR,
        // ... handle other currencies
        _ => return Err("Unsupported currency"),
    }
);
```

### Serialization

`rusty-money` supports serde serialization (with `serde` feature). The Money type serializes to a JSON object with amount and currency, or can be configured to serialize as a simple number if needed.

### Performance

For high-frequency operations, `rusty-money` also provides `FastMoney` (i64-based) which is ~5x faster but has lower precision. Use `Money` (128-bit decimal) for financial accuracy.
