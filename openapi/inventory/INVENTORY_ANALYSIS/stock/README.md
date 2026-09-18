# Stock Items & Availability - Competitive Analysis

> **Component:** Stock Items & Availability
> **Microservice:** `inventory/stock`
> **OpenAPI Spec:** [stock/openapi.yaml](../../stock/openapi.yaml)
> **Status:** Implemented

---

## Buyer Pitch

> *"I need to know what products I carry, where they are, and how much is available — right now, across all my warehouses."*

RERP Inventory's Stock service provides real-time item-level visibility into your entire inventory — SKU, name, category, unit of measure, status, and availability per location. Built as an independent microservice with OpenAPI specs, it exposes clean RESTful endpoints that any client (web, mobile, integration) can consume.

---

## Feature Comparison

| Feature | RERP Stock | SAP Business One | NetSuite | MS Dynamics 365 | QuickBooks Ent. | Odoo | Fishbowl | Acctivate | Cin7 |
|---------|-----------|------------------|----------|-----------------|-----------------|------|----------|-----------|------|
| SKU Management | ●●● | ●●● | ●●● | ●●● | ●●○ | ●●● | ●●● | ●●● | ●●● |
| Multi-location stock | ●●● | ●●○ | ●●● | ●●● | ●○○ | ●●● | ●●● | ●●○ | ●●○ |
| Real-time availability | ●●● | ●●○ | ●●● | ●●● | ●○○ | ●●● | ●●● | ●●○ | ●●○ |
| Reorder point alerts | ●●● | ●●○ | ●●● | ●●● | ●●○ | ●●○ | ●●○ | ●●○ | ●●○ |
| Stock status (active/inactive/discontinued) | ●●● | ●●○ | ●●● | ●●● | ●●○ | ●●● | ●●○ | ●●○ | ●●○ |
| Unit of measure | ●●● | ●●● | ●●● | ●●● | ●●● | ●●● | ●●● | ●●● | ●●● |
| Barcode support | ●○○ | ●●○ | ●●● | ●●○ | ●●○ | ●●● | ●●● | ●●○ | ●●● |
| Batch/Lot tracking | ●○○ | ●●○ | ●●● | ●●○ | ●●○ | ●●○ | ●●○ | ●●○ | ●●○ |
| Serial number tracking | ●○○ | ●●○ | ●●● | ●●○ | ●●○ | ●●○ | ●●○ | ●●○ | ●●○ |

**Legend:** ●●● = Full parity, ●●○ = Partial, ●○○ = Planned

---

## RERP vs. Competitors

### RERP Stock Advantages
1. **API-first, no GUI lock-in** — Every stock operation is an HTTP endpoint. Integrate with any system without middleware. SAP and QuickBooks require add-ons or connectors.
2. **Real-time availability per location** — Single endpoint returns aggregated and per-location stock. Competitors require separate queries for location-level stock.
3. **Status management** — Active/inactive/discontinued status with audit trail. Many competitors treat this as a boolean "active" field.
4. **Reorder intelligence** — Reorder point and quantity configured per item, with availability endpoint surfacing when thresholds are breached.
5. **Independent scaling** — Stock queries don't compete with warehouse or movement operations for resources.

### Competitive Gaps
1. **No barcode scanning** — Fishbowl, Odoo, and Cin7 have native GS1/EAN13 scanner support. RERP plans this as P0.
2. **No batch/lot tracking** — Critical for food, pharma, and regulated industries. All major competitors support it.
3. **No serial number tracking** — Important for electronics, furniture, and high-value items. All major competitors support it.

---

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/inventory/stocks` | List all stock items (paginated, searchable) |
| POST | `/api/v1/inventory/stocks` | Create a new stock item |
| GET | `/api/v1/inventory/stocks/{id}` | Get stock item details |
| PUT | `/api/v1/inventory/stocks/{id}` | Update stock item |
| DELETE | `/api/v1/inventory/stocks/{id}` | Delete stock item |
| GET | `/api/v1/inventory/stocks/{id}/availability` | Get availability across all locations |

---

## Data Model

The Stock service defines these core schemas:

- **Stock** — Core item record: ID, SKU, name, description, category, unit of measure, status, reorder settings
- **StockAvailability** — Aggregated availability: total, reserved, available quantities per location
- **StockLocation** — Per-location breakdown: location ID, name, quantity, reserved quantity
- **CreateStockRequest** / **UpdateStockRequest** — Mutation inputs
- **StockLocationUpdate** — Availability update payload

---

## Implementation Notes

- All endpoints use bearer token authentication
- Paginated list responses include total, page, limit fields
- UUID format for all resource IDs
- ISO 8601 date-time for timestamps
- Error responses follow standard ValidationError/Unauthorized/Forbidden/Conflict/NotFound pattern
- `x-brrtrouter-impl: true` on all mutations for code generation
