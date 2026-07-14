# RERP Accounting Suite

This directory is the ownership boundary for accounting runtime code and
operational assets. RERP hosts multiple suites, so accounting-specific crates,
entities, migrations, SQL contracts and scripts must not be added at the
repository root.

## Layout

- `core/` — deterministic accounting calculations with no transport or database.
- `entities/` — suite-wide Lifeguard persistence models.
- `migrations/` — ordered accounting schema and control migrations.
- `sql/` — vendored database contracts required by accounting, including RLS.
- `scripts/` — accounting database and operational setup.
- `<service>/gen/` — generated BRRTRouter contract layer.
- `<service>/impl/` — user-owned service behavior.
- `<service>/openapi/` — active service contract where a narrow runtime contract exists.

The parent `microservices/Cargo.toml` is the build workspace for these crates
and services. Root-level directories are reserved for genuinely cross-suite
assets.

## Verification

```bash
cd microservices
cargo test -p rerp-accounting-core
cargo check -p rerp-entities --lib
cargo test -p rerp_accounting_invoice
```

Database setup is location-independent:

```bash
./microservices/accounting/scripts/setup-db.sh
```
