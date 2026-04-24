# Invoice Model

> The base Invoice entity and its child types (CustomerInvoice, VendorInvoice).

**Status:** unverified (needs entity code inspection)

## Base Entity: Invoice

The base `Invoice` entity has these columns:
- `invoice_number` — TEXT
- `due_date` — TIMESTAMPTZ
- `status` — status enum
- `payment_state` — payment state enum
- And standard Lifeguard fields (id, created_at, updated_at, etc.)

## Child Entities

`CustomerInvoice` and `VendorInvoice` link to the base `Invoice` via:
- `invoice_id` — UUID foreign key to `invoices.id`

**CRITICAL:** Child entities do NOT inherit base entity columns.

### Index Rules

- ✅ Index on `customer_id` on `CustomerInvoice` (column exists on struct)
- ✅ Index on `vendor_id` on `VendorInvoice` (column exists on struct)
- ❌ Index on `invoice_number` on `CustomerInvoice` (column exists only on `Invoice`)
- ❌ Index on `due_date`, `status`, `payment_state` on child entities

### Querying

To query by base-entity fields (`invoice_number`, `due_date`, `status`) from a child table:
1. Use a SQL JOIN between child table and `invoices` table
2. Or filter on the base entity and join down to children

## Code Anchors

- Entity definitions: `entities/src/`
- Migration generation: `cargo run --bin generate-migrations` from `entities/`
