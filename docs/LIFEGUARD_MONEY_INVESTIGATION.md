# Lifeguard Money Type Support Investigation

## Current State

### Type Support in Lifeguard

**Supported Types (from `type_conversion.rs`):**
- Primitive integers: `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`
- Floating point: `f32`, `f64`
- Boolean: `bool`
- String: `String`
- Binary: `Vec<u8>`
- JSON: `serde_json::Value`
- Option<T> for all above types

**NOT Currently Supported:**
- `rust_decimal::Decimal` - Used in entities but `#[skip_from_row]` is required
- `rusty_money::Money` - Not supported at all

### Current Entity Pattern

Entities using `rust_decimal::Decimal` must use `#[skip_from_row]`:

```rust
#[derive(LifeModel)]
#[table_name = "journal_entry_lines"]
#[skip_from_row] // Skip FromRow generation - types don't implement FromSql yet
pub struct JournalEntryLine {
    #[column_type = "NUMERIC(19, 4)"]
    pub debit_amount: rust_decimal::Decimal,
    // ...
}
```

This means:
- SQL generation works (uses `#[column_type]`)
- FromRow generation is skipped (no database loading)
- Manual FromSql/ToSql implementations would be needed

## Investigation Findings

### 1. Type Conversion System

**Location:** `lifeguard-derive/src/type_conversion.rs`

**Functions:**
- `generate_field_to_value()` - Model → SeaQuery Value
- `generate_option_field_to_value()` - Record Option<T> → Option<Value>
- `generate_option_field_to_value_with_default()` - Model Option<T> → Value
- `generate_value_to_field()` - Value → Model (reserved for future)
- `generate_value_to_option_field()` - Value → Record Option<T>

**Current Limitation:**
- Only handles primitive types and `serde_json::Value`
- Unknown types fall back to `Value::String(None)`
- No support for `rust_decimal::Decimal` or `rusty_money::Money`

### 2. FromRow Generation

**Location:** `lifeguard-derive/src/macros/derive_from_row.rs`

**Current Behavior:**
- Generates `FromRow` implementation using `may_postgres::FromSql`
- Requires types to implement `FromSql` trait
- `#[skip_from_row]` skips generation entirely

**Issue:**
- `rust_decimal::Decimal` implements `FromSql` for NUMERIC types
- But Lifeguard's type conversion doesn't handle it
- `rusty_money::Money` may not implement `FromSql` directly

### 3. SQL Type Inference

**Location:** `lifeguard-derive/src/macros/life_model.rs` → `infer_sql_type_from_rust_type()`

**Current Behavior:**
- Infers SQL types from Rust types for automatic column mapping
- Handles: UUID, NaiveDateTime, NaiveDate, String, Value (JSONB), primitives
- Does NOT handle `rust_decimal::Decimal` or `rusty_money::Money`

**Current Workaround:**
- Entities use `#[column_type = "NUMERIC(19, 4)"]` explicitly
- This works for SQL generation but not for type conversion

## Required Changes

### Phase 1: Add Decimal Support

**1.1 Update Type Conversion (`type_conversion.rs`)**

Add functions to detect and handle `rust_decimal::Decimal`:

```rust
/// Check if a type is `rust_decimal::Decimal`
pub fn is_decimal_type(ty: &Type) -> bool {
    // Check for rust_decimal::Decimal or Decimal (if imported)
}

/// Generate Decimal → Value conversion
// Use Value::String(Some(decimal.to_string())) or Value::Double?
// Need to check SeaQuery Value variants
```

**1.2 Update FromRow Generation**

- Remove `#[skip_from_row]` requirement for Decimal
- Generate proper FromSql calls for Decimal fields
- Test with existing entities

**1.3 Update SQL Type Inference**

- Add Decimal → NUMERIC inference
- Make `#[column_type]` optional for Decimal fields

### Phase 2: Add Money Support

**2.1 Understand Money Type Structure**

```rust
// rusty_money::Money<C: Currency>
// Stores amount as Decimal internally
// Has currency information
```

**Key Questions:**
1. Does Money implement `FromSql`/`ToSql`?
2. How does Money serialize to/from database?
3. Should we store as NUMERIC + VARCHAR(currency_code) or single column?

**2.2 Update Type Conversion**

Add Money type detection and conversion:

```rust
/// Check if a type is `rusty_money::Money`
pub fn is_money_type(ty: &Type) -> bool {
    // Check for rusty_money::Money<...>
}

/// Generate Money → Value conversion
// Need to decide: store as Decimal + currency_code or custom format
```

**2.3 Database Storage Strategy**

**Option A: Store as NUMERIC (amount only)**
- Store amount as NUMERIC(19, 4)
- Keep currency_code as separate VARCHAR(3) column
- Money type constructed from both fields
- Pros: Simple, compatible with existing schema
- Cons: Currency not in Money type directly

**Option B: Store as JSONB**
- Store Money as JSON: `{"amount": "123.45", "currency": "USD"}`
- Single column for Money
- Pros: Currency and amount together
- Cons: Less efficient, harder to query

**Option C: Custom PostgreSQL Type**
- Create custom Money type in PostgreSQL
- Pros: Type-safe at database level
- Cons: Complex, requires database migration

**Recommended: Option A** (for now)
- Compatible with existing NUMERIC columns
- Currency_code already exists in entities
- Can migrate to Option B later if needed

**2.4 Update FromRow Generation**

- Generate code to construct Money from NUMERIC + currency_code
- Handle Option<Money> fields
- Test round-trip (save → load)

## Implementation Plan

### Step 1: Test Decimal Support (1-2 hours)

1. Create test entity with Decimal field (no `#[skip_from_row]`)
2. Try to generate FromRow
3. Check compilation errors
4. Document what works/doesn't work

### Step 2: Add Decimal to Type Conversion (2-3 hours)

1. Add `is_decimal_type()` function
2. Update `generate_field_to_value()` for Decimal
3. Update `generate_option_field_to_value()` for Decimal
4. Update `generate_value_to_option_field()` for Decimal
5. Test with existing entities

### Step 3: Update FromRow for Decimal (1-2 hours)

1. Remove `#[skip_from_row]` from one test entity
2. Verify FromRow generation works
3. Test database round-trip
4. Update all entities to remove `#[skip_from_row]` for Decimal

### Step 4: Add Money Type Detection (1-2 hours)

1. Research `rusty_money::Money` structure
2. Add `is_money_type()` function
3. Test type detection with various Money<Currency> patterns

### Step 5: Add Money to Type Conversion (3-4 hours)

1. Update `generate_field_to_value()` for Money
   - Extract Decimal from Money
   - Convert to Value
2. Update `generate_option_field_to_value()` for Money
3. Update `generate_value_to_option_field()` for Money
   - Convert Value → Decimal
   - Construct Money from Decimal + currency_code
4. Test conversions

### Step 6: Update FromRow for Money (2-3 hours)

1. Generate code to read NUMERIC + currency_code
2. Construct Money from both fields
3. Handle Option<Money>
4. Test database round-trip

### Step 7: Update SQL Type Inference (1 hour)

1. Add Money → NUMERIC inference
2. Make `#[column_type]` optional for Money
3. Test SQL generation

### Step 8: Update All Entities (2-3 hours)

1. Replace `rust_decimal::Decimal` with `rusty_money::Money` for financial amounts
2. Keep `currency_code` columns
3. Remove `#[skip_from_row]` where appropriate
4. Test all entities compile and work

## Testing Strategy

### Unit Tests

1. **Type Detection Tests**
   - Test `is_decimal_type()` with various Decimal patterns
   - Test `is_money_type()` with various Money<Currency> patterns

2. **Conversion Tests**
   - Test Decimal → Value conversion
   - Test Value → Decimal conversion
   - Test Money → Value conversion
   - Test Value → Money conversion

3. **FromRow Tests**
   - Test Decimal field loading from database
   - Test Money field loading from database
   - Test Option<Decimal> and Option<Money>

### Integration Tests

1. **Database Round-Trip**
   - Save entity with Decimal field → Load → Verify
   - Save entity with Money field → Load → Verify
   - Test with different currencies

2. **Query Tests**
   - Query entities with Decimal/Money filters
   - Test sorting by Decimal/Money fields
   - Test aggregations (SUM, AVG) on Money fields

## Risk Assessment

### Low Risk
- Adding Decimal support (well-understood type)
- Type detection functions (isolated, testable)

### Medium Risk
- Money type conversion (depends on Money internals)
- FromRow generation for Money (complex construction)

### High Risk
- Changing existing entities (may break code)
- Database schema compatibility (if we change storage)

## Success Criteria

1. ✅ Decimal fields work without `#[skip_from_row]`
2. ✅ Money fields work without `#[skip_from_row]`
3. ✅ All existing entities compile and work
4. ✅ Database round-trip works for Decimal and Money
5. ✅ Type conversion handles all Money variants
6. ✅ SQL generation works correctly
7. ✅ No breaking changes to existing code

## Next Steps

1. **Immediate:** Test if Decimal works with current Lifeguard (remove `#[skip_from_row]` from one entity)
2. **If Decimal works:** Add Money support
3. **If Decimal doesn't work:** Fix Decimal support first, then add Money

## References

- Lifeguard derive: `/Users/casibbald/Workspace/microscaler/lifeguard/lifeguard-derive/`
- Type conversion: `lifeguard-derive/src/type_conversion.rs`
- FromRow generation: `lifeguard-derive/src/macros/derive_from_row.rs`
- LifeModel generation: `lifeguard-derive/src/macros/life_model.rs`
- rusty-money docs: https://docs.rs/rusty-money/
- rust_decimal docs: https://docs.rs/rust_decimal/
