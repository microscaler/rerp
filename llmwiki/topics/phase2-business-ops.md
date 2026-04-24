# Phase 2: Business Operations Services

> CRM, sales, purchase, and inventory services.

**Status:** unverified

## Services

1. **crm/contacts** — Contact management
2. **sales/orders** — Order processing
3. **purchase/procurement** — Purchase orders and vendor management
4. **inventory/warehouse** — Warehouse and stock management
5. *(plus 10 more in business operations)*

## Dependencies

- crm/contacts → product/catalog (for pricing)
- sales/orders → crm/contacts, inventory/warehouse
- purchase/procurement → inventory/warehouse, product/catalog

## Code Anchors
- `openapi/crm/contacts/openapi.yaml`
- `openapi/sales/orders/openapi.yaml`
- `openapi/purchase/procurement/openapi.yaml`
- `openapi/inventory/warehouse/openapi.yaml`
