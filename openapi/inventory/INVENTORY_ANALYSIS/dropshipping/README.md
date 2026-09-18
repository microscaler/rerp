# Dropshipping & Fulfillment - Competitive Analysis

> **Component:** Dropshipping & Fulfillment
> **Microservice:** `inventory/dropshipping`
> **OpenAPI Spec:** [../dropshipping/openapi.yaml](../../dropshipping/openapi.yaml)
> **Status:** Implemented

---

## Buyer Pitch

> *"I sell products but never touch them — my suppliers ship directly to my customers. I need to track those orders, manage vendor relationships, and keep my customers' experience seamless."*

RERP Inventory's Dropshipping service manages the vendor-order-to-customer-fulfillment pipeline. It handles dropship order creation, vendor order generation, fulfillment tracking, and the financial flow between customer revenue and vendor cost. Essential for businesses that sell without stocking inventory.

---

## Feature Comparison

| Feature | RERP Dropshipping | SAP B1 | NetSuite | MS Dynamics 365 | QuickBooks Ent. | Odoo | Fishbowl | Acctivate | Cin7 |
|---------|------------------|--------|----------|-----------------|-----------------|------|----------|-----------|------|
| Dropship orders | ●●● | ●●○ | ●●● | ●●○ | ●○○ | ●●○ | ●●○ | ●○○ | ●●● |
| Vendor order management | ●●● | ●●○ | ●●● | ●●○ | ●●○ | ●●○ | ●●○ | ●●● | ●●○ |
| Dropship fulfillment tracking | ●●● | ●●○ | ●●● | ●●○ | ●○○ | ●●○ | ●●○ | ●●○ | ●●● |
| Vendor catalog integration | ●○○ | ●●○ | ●●● | ●●○ | ●○○ | ●○ | ●●○ | ●○ | ●●● |
| Multi-vendor dropship | ●●● | ●●○ | ●●● | ●●○ | ●○○ | ●●○ | ●●○ | ●○ | ●●● |
| Automatic PO on sale | ●○○ | ●●○ | ●●● | ●●○ | ●○○ | ●●○ | ●○ | ●○ | ●●○ |
| Vendor portal | ●○○ | ●●○ | ●●○ | ●●○ | ●○○ | ●●○ | ●●○ | ●○○ | ●●○ |
| Blanket PO support | ●○○ | ●●○ | ●●● | ●●○ | ●○○ | ●○ | ●○ | ●●○ | ●○ |
| Dropship return handling | ●○○ | ●●○ | ●●● | ●●○ | ●○○ | ●○ | ●○ | ●○ | ●○ |
| 3PL integration | ●○○ | ●●○ | ●●● | ●●○ | ●○○ | ●○ | ●○ | ●○○ | ●●○ |

**Legend:** ●●● = Full parity, ●●○ = Partial, ●○○ = Planned

---

## RERP vs. Competitors

### RERP Dropshipping Advantages
1. **API-first dropship management** — Every dropship order and vendor order is an HTTP endpoint. No proprietary GUI. Integrate with any e-commerce platform, marketplace, or custom storefront.
2. **Clean vendor-order model** — Vendor orders are first-class resources with full CRUD lifecycle, not an afterthought on purchase orders. Clear separation between customer-facing dropship orders and vendor-side fulfillment.
3. **Multi-vendor support** — Single customer order can span multiple vendors. RERP creates individual vendor orders per vendor automatically.
4. **Independent microservice** — Dropship operations don't compete with warehouse or stock operations for resources. Pure API throughput.
5. **No vendor lock-in** — Unlike competitors tied to QuickBooks (Fishbowl/Acctivate) or Intuit (Acctivate), RERP has no upstream dependency.

### Competitive Gaps
1. **No vendor catalog integration** — Cin7 and NetSuite sync vendor catalogs automatically, so products appear in your store without manual entry. RERP must build this integration layer.
2. **No automatic PO generation** — NetSuite and Odoo can auto-generate vendor purchase orders from confirmed dropship sales. RERP must add this rule engine.
3. **No vendor portal** — NetSuite, SAP, and Odoo provide vendor-facing portals where suppliers confirm orders, upload tracking, and manage fulfillment. RERP must build this.
4. **No blanket PO support** — NetSuite and Acctivate support blanket purchase orders that auto-generate individual POs as sales come in. Important for high-volume dropshippers.
5. **No dropship return handling** — NetSuite and Cin7 have dedicated workflows for dropship returns (vendor receives return, refund issued to customer). RERP must build this.

---

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/inventory/dropship-orders` | List all dropship orders |
| POST | `/api/v1/inventory/dropship-orders` | Create a new dropship order |
| GET | `/api/v1/inventory/dropship-orders/{id}` | Get dropship order details |
| PUT | `/api/v1/inventory/dropship-orders/{id}` | Update dropship order |
| POST | `/api/v1/inventory/dropship-orders/{id}/cancel` | Cancel dropship order |
| GET | `/api/v1/inventory/vendor-orders` | List all vendor orders |
| POST | `/api/v1/inventory/vendor-orders` | Create a new vendor order |
| GET | `/api/v1/inventory/vendor-orders/{id}` | Get vendor order details |
| PUT | `/api/v1/inventory/vendor-orders/{id}` | Update vendor order |

---

## Data Model

The Dropshipping service defines these core schemas:

- **DropshipOrder** — Customer-facing dropship order: ID, customer info, items, vendor assignments, status, customer price, vendor cost
- **VendorOrder** — Supplier-facing order: ID, vendor ID, linked dropship order, items, vendor price, expected delivery date
- **CreateDropshipOrderRequest** / **UpdateDropshipOrderRequest** — Mutation inputs with items, customer info, vendor assignments
- **CreateVendorOrderRequest** / **UpdateVendorOrderRequest** — Mutation inputs with vendor-specific pricing and delivery details

---

## Implementation Notes

- Dropship orders link to one or more vendor orders
- Vendor orders are created when dropship order is confirmed
- Vendor order statuses: pending, confirmed, shipped, delivered, cancelled
- All mutations include standardized error responses (400/401/403/409)
- `x-brrtrouter-impl: true` on all mutations for code generation
- UUID format for all resource IDs
- ISO 8601 date-time for timestamps
