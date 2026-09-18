# Lifeguard Entity Conventions

> How Lifeguard ORM entities work in RERP — migration workflow, column types, indices.

**Status:** partially-verified

## Migration Workflow

**CRITICAL: NEVER edit migration files directly.**

1. Update entity structs (`LifeModel`/`LifeRecord` derive in `entities/`)
2. Run migrator to generate new migrations
3. The migrator compares entity definitions against existing migrations and produces additive deltas

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
| `i8`, `i16`, `i32`, `u8`, `u16`, `u32` | INTEGER |
| `i64`, `u64` | BIGINT |
| `bool` | BOOLEAN |

## Index Rules

- `#[index = "idx_name(column)"]` and `#[indexed]` must reference **only columns that exist on that struct**.
- The derive and migration generator do **not** validate that index columns exist.
- Child/specialized entities that link to a base entity via FK do **not** inherit the base entity's columns.
- To query by base-entity fields on a child table, use a JOIN.

## Common Attributes

```rust
#[table_name]
#[schema_name]
#[primary_key]
#[column_type]
#[column_name]
#[indexed]
#[unique]
#[nullable]
#[auto_increment]
#[default_expr]
#[comment]
#[foreign_key]
#[check]
#[soft_delete]
#[skip] / #[ignore]
#[has_many] / #[belongs_to] / #[has_one]
#[composite_unique = "..."]
#[index = "name(columns)"]
#[index = "name(key_parts)"]  // with btree options
```

## DB Pool

- All 9 microservices share a single Postgres pool
- `DB_POOL_MAX` env var (default 10 in dev, 30-300+ in production)
- Replica routing: **disabled** (hardcoded empty vec in `parse_replica_config()`)

## Code Anchors

- Entity definitions: `entities/src/`
- Migrator: `cargo run --bin generate-migrations` (run from `entities/` dir)
- Lifeguard derive: `lifeguard_derive::LifeModel`
