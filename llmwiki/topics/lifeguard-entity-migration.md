# Lifeguard Entity Migration Workflow

> The complete workflow for creating and evolving Lifeguard ORM entities.

**Status:** partially-verified

## CRITICAL RULE: NEVER edit migration files directly

## Workflow

1. Update entity structs (`LifeModel`/`LifeRecord` derive) in `entities/src/`
2. Run migrator to generate new migrations
3. Migrator compares entity definitions against existing migrations and produces additive deltas

## Column Type Inference

| Rust Type | PostgreSQL Type |
|-----------|----------------|
| `Uuid` | UUID |
| `DateTime<Utc>` | TIMESTAMPTZ |
| `NaiveDateTime` | TIMESTAMP |
| `NaiveDate` | DATE |
| `String` | TEXT |
| `serde_json::Value` | JSONB |
| `rust_decimal::Decimal` | NUMERIC(19,4) |
| `i8/i16/i32/u8/u16/u32` | INTEGER |
| `i64/u64` | BIGINT |
| `bool` | BOOLEAN |

## Index Rules

- `#[index = "idx_name(column)"]` and `#[indexed]` must reference **only columns that exist on that struct**
- The derive and migration generator do not validate that index columns exist
- Child entities that link to a base entity via FK do **not** inherit the base entity's columns
- To query by base-entity fields on a child table, use a JOIN

## Common Attributes

```
#[table_name], #[schema_name], #[primary_key], #[column_type]
#[column_name], #[indexed], #[unique], #[nullable], #[auto_increment]
#[default_expr], #[comment], #[foreign_key], #[check], #[soft_delete]
#[skip]/#[ignore], #[has_many]/#[belongs_to]/#[has_one]
#[composite_unique = "..."], #[index = "name(columns)"]
```

## Verification

After changing indices, run:
```bash
cargo run --bin generate-migrations
```
And apply migrations in a test DB to confirm they apply cleanly.

## Code Anchors
- Entity definitions: `entities/src/`
- Migrator: `cargo run --bin generate-migrations`
- Skill: `lifeguard-entity-migration` (loaded automatically)
- DB Pool: `DB_POOL_MAX` env var (default 10 in dev), shared across all 9 services
- Replica routing: disabled (hardcoded empty vec in `parse_replica_config()`)
